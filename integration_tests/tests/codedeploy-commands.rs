#![cfg(feature = "codedeploy-commands")]

extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_codedeploy;
extern crate rusoto_codedeploy_commands;
extern crate rusoto_iam;
extern crate rusoto_sts;
extern crate uuid;

use rusoto_iam::{Iam, IamClient};
use rusoto_sts::{Sts, StsClient};
use rusoto_codedeploy::{CodeDeploy, CodeDeployClient};
use rusoto_core::{ProvideAwsCredentials, Region};
use rusoto_codedeploy_commands::{CodeDeployCommands, CodeDeployCommandsClient};
use std::fmt::Write;

/// The domain component for the current ARN partition. Currently this test will only succeed in 
/// the 'aws' partition - it will fail in 'aws-cn' and 'aws-us-gov'.
const ARN_PARTITION_DOMAIN: &'static str = "amazonaws.com";

const COMMAND_NAMES: &'static [&'static str; 7] = &[
    "ApplicationStop",
    "DownloadBundle",
    "BeforeInstall",
    "Install",
    "AfterInstall",
    "ApplicationStart",
    "ValidateService"
];

/// This test performs a full CodeDeploy deployment to a single on-premise instance. The
/// "instance" is actually simulated by this test by calling the CodeDeploy commands service
/// to process commands using the same public APIs that the CodeDeploy agent uses. The instance
/// is simulated by using CodeDeploy's ability to register an on-premise instance using IAM
/// session ARNs, as described [here](https://docs.aws.amazon.com/codedeploy/latest/userguide/register-on-premises-instance-iam-session-arn.html).
///
/// # Resources
/// 
/// This tests creates the following resources in the AWS account against which the integration
/// tests run:
///
/// - An IAM role used to simulate an on-premise instance (using an embedded policy).
/// - An IAM role for CodeDeploy's service role (uses the "AWSCodeDeployRole" managed policy).
/// - A CodeDeploy on-premise instance registration.
/// - A CodeDeploy application and deployment group.
///
/// All of the resources are deleted if the test succeeds. If the test fails, some resources
/// may remain in the account. The resources are created with deterministic names, so if the
/// test is subsequently run successfully, it will detect the existing resources and clean
/// them up.
#[test]
fn successful_deployment_to_host() {
    const ON_PREMISE_INSTANCE_ROLE_NAME: &'static str = "rusoto-cd-commands-test-succ-dep";
    const ON_PREMISE_INSTANCE_SESSION_NAME_BASE: &'static str = "simulated-instance-";
    const ON_PREMISE_INSTANCE_POLICY_NAME: &'static str = "integration-test-policy";
    const CODEDEPLOY_SERVICE_ROLE_NAME: &'static str = "rusot-cd-commands-test-svc-role";
    const APPLICATION_NAME: &'static str = "rusoto-cd-commands-test-succ-app";
    const DEPLOYMENT_GROUP_NAME: &'static str = "rusoto-cd-commands-test-succ-depgroup";
    const ON_PREMISE_INSTANCE_NAME_BASE: &'static str = "simulated-instance-";
    const ON_PREMISE_TAG_KEY: &'static str = "rusoto-cd-commands-test-run";
    const ON_PREMISE_TAG_VALUE_BASE: &'static str = "succ-dep-";

    // CodeDeploy forces instance name and session name to be different due to how it
    // has a mandatory cooldown period after deregistering an on-premise instance where
    // the name is unusable.
    let run_uuid = uuid::Uuid::new_v4();
    let mut on_premise_instance_session_name = ON_PREMISE_INSTANCE_SESSION_NAME_BASE.to_owned();
    write!(on_premise_instance_session_name, "{}", run_uuid).unwrap();
    let mut on_premise_instance_name = ON_PREMISE_INSTANCE_NAME_BASE.to_owned();
    write!(on_premise_instance_name, "{}", run_uuid).unwrap();
    let mut on_premise_tag_value = ON_PREMISE_TAG_VALUE_BASE.to_owned();
    write!(on_premise_tag_value, "{}", run_uuid).unwrap();

    let region = Region::UsEast1;
    let iam = IamClient::simple(region.clone());
    let sts = StsClient::simple(region.clone());
    let cd = CodeDeployClient::simple(region.clone());

    // Get identity information for integration test credentials.
    println!("get_integration_test_identity");
    let test_identity = get_integration_test_identity(&sts);

    // Create on-premise IAM role, with permissions to call "codedeploy-commands:*".
    println!("create_on_premise_iam_role");
    let on_premise_role = create_on_premise_iam_role(&iam, ON_PREMISE_INSTANCE_ROLE_NAME, &test_identity, ON_PREMISE_INSTANCE_POLICY_NAME);

    // Create CodeDeploy service role with AWSCodeDeployRole managed policy.
    println!("create_codedeploy_service_iam_role");
    let (codedeploy_service_role, codedeploy_policy_arn) = create_codedeploy_service_iam_role(&iam, CODEDEPLOY_SERVICE_ROLE_NAME);

    // Assume role to get on-premise session credentials.
    println!("assume_on_premise_session_credentials");
    let (on_premise_credentials, on_premise_session_arn) = assume_on_premise_session_credentials(&sts, &on_premise_role.arn, &on_premise_instance_session_name);

    // Create CodeDeploy application and deployment group.
    println!("create_codedeploy_state");
    let (application_id, deployment_group_id) = create_codedeploy_state(&cd, APPLICATION_NAME, DEPLOYMENT_GROUP_NAME, &codedeploy_service_role.arn, ON_PREMISE_TAG_KEY, &on_premise_tag_value);

    // Register and tag the on-premise instance.
    println!("register_and_tag_on_premise_instance");
    register_and_tag_on_premise_instance(&cd, &on_premise_instance_name, &on_premise_session_arn, ON_PREMISE_TAG_KEY, &on_premise_tag_value);

    // Start a CodeDeploy deployment.
    println!("start_deployment_to_on_premise_instance");
    let deployment_id = start_deployment_to_on_premise_instance(&cd, APPLICATION_NAME, DEPLOYMENT_GROUP_NAME);

    // Process each command sequentially, returning success for each.
    println!("process_all_deployment_commands");
    process_all_deployment_commands(region.clone(), on_premise_credentials, &on_premise_session_arn);

    // Wait for deployment to complete successfully.
    wait_for_successful_deployment_completion(&cd, &deployment_id);

    // Unregister/untag the on-premise instance.
    untag_and_deregister_on_premise_instance(&cd, &on_premise_instance_name, ON_PREMISE_TAG_KEY, &on_premise_tag_value);

    // Delete Codedeploy deployment group and application.
    delete_codedeploy_state(&cd, APPLICATION_NAME, DEPLOYMENT_GROUP_NAME);

    // Delete CodeDeploy service role.
    delete_codedeploy_service_iam_role(&iam, CODEDEPLOY_SERVICE_ROLE_NAME, &codedeploy_policy_arn);

    // Delete on-premise IAM role.
    delete_on_premise_iam_role(&iam, ON_PREMISE_INSTANCE_ROLE_NAME, ON_PREMISE_INSTANCE_POLICY_NAME);
}

struct IntegrationTestIdentity {
    account_id: String,
    arn: String,
    user_id: String
}

fn get_integration_test_identity<Sts: rusoto_sts::Sts>(sts: &Sts) -> IntegrationTestIdentity {
    let response = sts.get_caller_identity(rusoto_sts::GetCallerIdentityRequest {}).sync()
        .expect("Failed to invoke STS::GetCallerIdentity for the integration test identity");
    IntegrationTestIdentity {
        account_id: response.account.expect("STS::GetCallerIdentity did not return an AWS account id for the integration test identity"),
        arn: response.arn.expect("STS::GetCallerIdentity did not return an ARN for the integration test identity"),
        user_id: response.user_id.expect("STS::GetCallerIdentity did not return a user id for the integration test identity")
    }
}

/// Creates an IAM role to represent the on-premise instance, and return the ARN of the created
/// role. If a role with the same name already exists then this function will assume that the
/// role was previously created with the correct permissions and will return the existing role's
/// ARN instead of creating a new one.
fn create_on_premise_iam_role<Iam: rusoto_iam::Iam>(
        iam: &Iam,
        role_name: &str,
        test_identity: &IntegrationTestIdentity,
        policy_name: &str) -> rusoto_iam::Role {
    let create_request = rusoto_iam::CreateRoleRequest {
        assume_role_policy_document: format!(r#"{{ "Version":"2012-10-17", "Statement": [ {{ "Effect":"Allow", "Principal": {{ "AWS": "{account_id}" }}, "Action":"sts:AssumeRole", "Condition": {{ "StringEquals": {{ "aws:userid":"{user_id}" }} }} }} ] }}"#,
            account_id = test_identity.account_id,
            user_id = test_identity.user_id)
            .to_owned(),
        description: Some("Represents an on-premise CodeDeploy instance for Rusoto codedeploy-commands integration test".to_owned()),
        max_session_duration: None,
        path: None,
        role_name: role_name.to_owned()
    };
    let role = match iam.create_role(create_request).sync() {
        Ok(response) => {
            response.role
        },
        Err(err) => {
            match err {
                // If the role already exists, then we assume it was left over from a previous unsuccessful test run.
                rusoto_iam::CreateRoleError::EntityAlreadyExists(msg) => {
                    let response = iam.get_role(rusoto_iam::GetRoleRequest { role_name: role_name.to_owned() }).sync()
                        .expect("Failed to invoke IAM::GetRole to retrieve information about an existing on-premise instance role");
                    response.role
                },
                other => { panic!("Failed to invoke IAM::CreateRole to create on-premise instance role: {}", other) }
            }
        }
    };

    // Add inline permissions to the role to allow us to call CodeDeployCommands service APIs to process commands.
    let put_policy_request = rusoto_iam::PutRolePolicyRequest {
        policy_document: r#"{ "Version":"2012-10-17", "Statement": { "Effect":"Allow", "Action":"codedeploy-commands:*", "Resource":"*" } }"#.to_owned(),
        policy_name: policy_name.to_owned(),
        role_name: role_name.to_owned()
    };
    iam.put_role_policy(put_policy_request).sync()
        .expect("Failed to invoke IAM::PutPolicy to allow the on-premise IAM role permissions to call codedeploy-commands:* operations");

    role
}

/// Deletes the on-premise IAM role.
fn delete_on_premise_iam_role<Iam: rusoto_iam::Iam>(iam: &Iam, role_name: &str, policy_name: &str) {
    iam.delete_role_policy(rusoto_iam::DeleteRolePolicyRequest { role_name: role_name.to_owned(), policy_name: policy_name.to_owned() }).sync()
        .expect("Failed to invoke IAM::DeleteRolePolicy to delete on-premise instance role's embedded policy");
    iam.delete_role(rusoto_iam::DeleteRoleRequest { role_name: role_name.to_owned() }).sync()
        .expect("Failed to invoke IAM::DeleteRole to delete on-premise instance role");
}

/// Assumes an on premise session role and returns the assumed temporary credentials, which can be used
/// to call the CodeDeploy commands service.
fn assume_on_premise_session_credentials<Sts: rusoto_sts::Sts>(sts: &Sts, role_arn: &str, role_session_name: &str) -> (rusoto_credential::StaticProvider, String) {
    let mut i = 0;
    let response = loop {
        ::std::thread::sleep(::std::time::Duration::from_millis(1000));
        let request = rusoto_sts::AssumeRoleRequest {
            duration_seconds: None,
            external_id: None,
            policy: Some(r#"{ "Version":"2012-10-17", "Statement": [ { "Action":"codedeploy-commands:*", "Effect":"Allow", "Resource":"*" } ] }"#.to_owned()),
            role_arn: role_arn.to_owned(),
            role_session_name: role_session_name.to_owned(),
            serial_number: None,
            token_code: None
        };
        match sts.assume_role(request).sync() {
            Ok(response) => break response,
            Err(err) => match err {
                // Unfortunately not having permission to assume is surfaced as an Unknown error.
                rusoto_sts::AssumeRoleError::Unknown(msg) => {
                    if !msg.contains("Not authorized to perform sts:AssumeRole") && !msg.contains("not authorized to perform: sts:AssumeRole") {
                        panic!("Failed to invoke STS::AssumeRole to assume on-premise instance role: {:?}", msg);
                    }
                    // Otherwise do nothing and allow the loop to retry the authorization error.
                },
                other => { panic!(); }
            }
        }
        i += 1;
        if i >= 30 {
            panic!("Unable to assume the on-premise IAM role to get session credentials to call CodeDeploy with even after waiting 30 seconds");
        }
    };
    let session_arn = response.assumed_role_user.expect("STS::AssumeRole did not return information about the assumed role user").arn;
    let credentials = response.credentials.expect("STS::AssumeRole did not return temporary session credentials");
    (rusoto_credential::StaticProvider::new(
        credentials.access_key_id,
        credentials.secret_access_key,
        Some(credentials.session_token),
        None
    ), session_arn)
}

/// Creates the CodeDeploy application and deployment group that will be used to deploy to the simulated
/// on-premise instance. If either the application or deployment group already exists then this function 
/// will assume that the objects were previously created with the correct configuration and will not attempt
/// to recreate them.
fn create_codedeploy_state<CodeDeploy: rusoto_codedeploy::CodeDeploy>(
        cd: &CodeDeploy,
        application_name: &str,
        deployment_group_name: &str,
        service_role_arn: &str,
        tag_key: &str,
        tag_value: &str) -> (String, String) {
    let application_request = rusoto_codedeploy::CreateApplicationInput {
        application_name: application_name.to_owned(),
        compute_platform: Some("Server".to_owned())
    };
    let application_id = match cd.create_application(application_request).sync() {
        Ok(output) => output.application_id.expect("CodeDeploy::CreateApplication succeeded but did not return an application id"),
        Err(err) => match err {
            rusoto_codedeploy::CreateApplicationError::ApplicationAlreadyExists(_) => {
                let response = cd.get_application(rusoto_codedeploy::GetApplicationInput { application_name: application_name.to_owned() }).sync()
                    .expect("Failed to invoke CodeDeploy::GetApplication to retrieve information about an existing application");
                response
                    .application.expect("CodeDeploy::GetApplication succeeded but did not return any application information")
                    .application_id.expect("CodeDeploy::GetApplication succeeded but did not return the application id")
            },
            other => { panic!("Failed to invoke CodeDeploy::CreateApplication while setting up CodeDeploy state: {}", other); }
        }
    };

    let deployment_group_request = rusoto_codedeploy::CreateDeploymentGroupInput {
        alarm_configuration: Some(rusoto_codedeploy::AlarmConfiguration {
            alarms: Some(vec![]),
            enabled: Some(false),
            ignore_poll_alarm_failure: Some(true)
        }),
        application_name: application_name.to_owned(),
        auto_rollback_configuration: Some(rusoto_codedeploy::AutoRollbackConfiguration {
            enabled: Some(false),
            events: Some(vec![])
        }),
        auto_scaling_groups:None,
        blue_green_deployment_configuration: None,
        deployment_config_name: Some("CodeDeployDefault.OneAtATime".to_owned()),
        deployment_group_name: deployment_group_name.to_owned(),
        deployment_style: Some(rusoto_codedeploy::DeploymentStyle {
            deployment_option: Some("WITHOUT_TRAFFIC_CONTROL".to_string()),
            deployment_type: Some("IN_PLACE".to_string())
        }),
        ec_2_tag_filters: None,
        ec_2_tag_set: None,
        load_balancer_info: None,
        on_premises_instance_tag_filters: Some(vec![rusoto_codedeploy::TagFilter {
            key: Some(tag_key.to_owned()),
            type_: Some("KEY_AND_VALUE".to_owned()),
            value: Some(tag_value.to_owned())
        }]),
        on_premises_tag_set: None,
        service_role_arn: service_role_arn.to_string(),
        trigger_configurations: None
    };
    let deployment_group_id = create_deployment_group(cd, &deployment_group_request, application_name, deployment_group_name);

    (application_id, deployment_group_id)
}

// HACK: CodeDeploy attempts to assume the service role provided when CreateDeploymentGroup is called,
// to verify that the role's trust policy is set up correctly. Unfortunately IAM is eventually consistent,
// meaning that CodeDeploy may be unable to assume the role for a short while after the role has been created.
// In this code we continue retrying the CreateDeploymentGroup for up to 30 seconds, waiting one second
// between attempts.
fn create_deployment_group<CodeDeploy: rusoto_codedeploy::CodeDeploy>(
        cd: &CodeDeploy,
        request: &rusoto_codedeploy::CreateDeploymentGroupInput,
        application_name: &str,
        deployment_group_name: &str) -> String {
    let mut i = 0;
    loop {
        ::std::thread::sleep(::std::time::Duration::from_millis(1000));
        match cd.create_deployment_group(request.clone()).sync() {
            Ok(output) => return output.deployment_group_id.expect("CodeDeploy::CreateDeploymentGroup succeeded but did not return a deployment group id"),
            Err(err) => match err {
                rusoto_codedeploy::CreateDeploymentGroupError::DeploymentGroupAlreadyExists(_) => {
                    let request = rusoto_codedeploy::GetDeploymentGroupInput { 
                        application_name: application_name.to_owned(),
                        deployment_group_name: deployment_group_name.to_owned()
                    };
                    let response = cd.get_deployment_group(request).sync()
                        .expect("Failed to invoke CodeDeploy::GetDeploymentGroup to retrieve information about an existing deployment group");
                    return response
                        .deployment_group_info.expect("CodeDeploy::GetDeploymentGroup succeeded but did not return deployment group info")
                        .deployment_group_id.expect("CodeDeploy::GetDeploymentGroup succeeded but did not return the deployment group's id")
                },
                rusoto_codedeploy::CreateDeploymentGroupError::InvalidRole(_) => {
                    // No-op
                },
                other => { panic!("Failed to invoke CodeDeploy::CreateDeploymentGroup while setting up CodeDeploy state: {:?}", other); }
            }
        }
        i += 1;
        if i >= 30 {
            panic!("CodeDeploy was unable to assume the CodeDeploy service role when calling CreateDeploymentGroup even after waiting 30 seconds");
        }
    }
}

fn delete_codedeploy_state<CodeDeploy: rusoto_codedeploy::CodeDeploy>(
        cd: &CodeDeploy,
        application_name: &str,
        deployment_group_name: &str) {
    let deployment_group_request = rusoto_codedeploy::DeleteDeploymentGroupInput {
        application_name: application_name.to_owned(),
        deployment_group_name: deployment_group_name.to_owned()
    };
    cd.delete_deployment_group(deployment_group_request).sync()
        .expect("Failed to invoke CodeDeploy::DeleteDeploymentGroup to delete deployment group");

    let application_request = rusoto_codedeploy::DeleteApplicationInput {
        application_name: application_name.to_owned()
    };
    cd.delete_application(application_request).sync()
        .expect("Failed to invoke CodeDeploy::CreateApplication to delete application");
}

/// Creates the CodeDeploy service IAM role, which allows the central CodeDeploy service to call AWS services
/// on our behalf, and return the ARN of the created role. If a role with the same name already exists then
/// this function will assume that the role was previously created with the correct permissions and will return
/// the existing role's ARN instead of creating a new one.
fn create_codedeploy_service_iam_role<Iam: rusoto_iam::Iam>(iam: &Iam, role_name: &str) -> (rusoto_iam::Role, String) {
    // Call IAM::ListPolicies to find the AWSCodeDeployRole (failing if it cannot be found).
    let codedeploy_policy_arn = find_codedeploy_managed_policy_arn(iam);

    let create_request = rusoto_iam::CreateRoleRequest {
        // Allow CodeDeploy service principal to assume role.
        assume_role_policy_document: format!(r#"{{ "Version":"2012-10-17", "Statement": [ {{ "Effect":"Allow", "Principal": {{ "Service":"codedeploy.{arn_partition_domain}" }}, "Action": ["sts:AssumeRole"] }} ] }}"#, arn_partition_domain = ARN_PARTITION_DOMAIN).to_owned(),
        description: Some("CodeDeploy service role for Rusoto codedeploy-commands integration test".to_owned()),
        max_session_duration: None,
        path: None,
        role_name: role_name.to_owned()
    };
    let role = match iam.create_role(create_request).sync() {
        Ok(response) => {
            response.role
        },
        Err(err) => {
            match err {
                // If the role already exists, then we assume it was left over from a previous unsuccessful test run.
                rusoto_iam::CreateRoleError::EntityAlreadyExists(msg) => {
                    let response = iam.get_role(rusoto_iam::GetRoleRequest { role_name: role_name.to_owned() }).sync()
                        .expect("Failed to invoke IAM::GetRole to retrieve information about an existing CodeDeploy service role");
                    response.role
                },
                other => { panic!("Failed to invoke IAM::CreateRole to create CodeDeploy service role: {}", other) }
            }
        }
    };

    // Attach the managed policy to the role.
    let attach_request = rusoto_iam::AttachRolePolicyRequest {
        policy_arn: codedeploy_policy_arn.clone(),
        role_name: role_name.to_owned()
    };
    iam.attach_role_policy(attach_request).sync()
        .expect("Failed to invoke IAM::AttachRolePolicy to attach AWSCodeDeployRole managed policy to CodeDeploy service role");

    (role, codedeploy_policy_arn)
}

fn find_codedeploy_managed_policy_arn<Iam: rusoto_iam::Iam>(iam: &Iam) -> String {
    let mut is_truncated = true;
    let mut marker = None;
    while is_truncated {
        let list_request = rusoto_iam::ListPoliciesRequest {
            marker,
            max_items: Some(100),
            only_attached: None,
            path_prefix: None,
            scope: Some("AWS".to_owned())
        };
        let list_response = iam.list_policies(list_request).sync()
            .expect("Failed to invoke IAM::ListPolicies while searching for AWSCodeDeployRole managed policy");
        is_truncated = list_response.is_truncated.unwrap_or(false);
        marker = list_response.marker;
        if let Some(policies) = list_response.policies {
            for policy in policies {
                if policy.policy_name.map_or(false, |policy_name| policy_name == "AWSCodeDeployRole") {
                    return policy.arn.expect("Found AWSCodeDeployRole from IAM::ListPolicies but it does not have an ARN");
                }
            }
        }
    }

    panic!("Did not find the AWSCodeDeployRole managed policy among all the managed policies available to the integration test account");
}

/// Deletes an IAM role.
fn delete_codedeploy_service_iam_role<Iam: rusoto_iam::Iam>(iam: &Iam, role_name: &str, codedeploy_policy_arn: &str) {
    iam.detach_role_policy(rusoto_iam::DetachRolePolicyRequest { role_name: role_name.to_owned(), policy_arn: codedeploy_policy_arn.to_owned() }).sync()
        .expect("Failed to invoke IAM::DetachRolePolicy while deleting the CodeDeploy service role");
    iam.delete_role(rusoto_iam::DeleteRoleRequest { role_name: role_name.to_owned() }).sync()
        .expect("Failed to invoke IAM::DeleteRole to CodeDeploy service role");
}

fn register_and_tag_on_premise_instance<CodeDeploy: rusoto_codedeploy::CodeDeploy>(
        cd: &CodeDeploy,
        instance_name: &str,
        session_arn: &str,
        tag_key: &str,
        tag_value: &str) {
    let register_request = rusoto_codedeploy::RegisterOnPremisesInstanceInput {
        iam_session_arn: Some(session_arn.to_owned()),
        iam_user_arn: None,
        instance_name: instance_name.to_owned()
    };
    cd.register_on_premises_instance(register_request).sync()
        .expect("Failed to invoke CodeDeploy::RegisterOnPremiseInstance to register simulated on-premise instance");

    let tags_request = rusoto_codedeploy::AddTagsToOnPremisesInstancesInput {
        instance_names: vec![instance_name.to_owned()],
        tags: vec![rusoto_codedeploy::Tag {
            key: Some(tag_key.to_owned()),
            value: Some(tag_value.to_owned())
        }]
    };
    cd.add_tags_to_on_premises_instances(tags_request).sync()
        .expect("Failed to invoke CodeDeploy::AddTagsToOnPremiseInstances to add tags to simulated on-premise instance");
}

fn untag_and_deregister_on_premise_instance<CodeDeploy: rusoto_codedeploy::CodeDeploy>(
        cd: &CodeDeploy,
        instance_name: &str,
        tag_key: &str,
        tag_value: &str) {
    let tags_request = rusoto_codedeploy::RemoveTagsFromOnPremisesInstancesInput {
        instance_names: vec![instance_name.to_owned()],
        tags: vec![rusoto_codedeploy::Tag {
            key: Some(tag_key.to_owned()),
            value: Some(tag_value.to_owned())
        }]
    };
    cd.remove_tags_from_on_premises_instances(tags_request).sync()
        .expect("Failed to invoke CodeDeploy::RemoveTagsFromOnPremisesInstances to untag simulated on-premise instance");

    let deregister_request = rusoto_codedeploy::DeregisterOnPremisesInstanceInput {
        instance_name: instance_name.to_owned()
    };
    cd.deregister_on_premises_instance(deregister_request).sync()
        .expect("Failed to invoke CodeDeploy::DeregisterOnPremisesInstance to deregister the simulated on-premise instance");
}

fn start_deployment_to_on_premise_instance<CodeDeploy: rusoto_codedeploy::CodeDeploy>(
        cd: &CodeDeploy,
        application_name: &str,
        deployment_group_name: &str) -> String {
    let create_request = rusoto_codedeploy::CreateDeploymentInput {
        application_name: application_name.to_owned(),
        auto_rollback_configuration: None,
        deployment_config_name: None,
        deployment_group_name: Some(deployment_group_name.to_owned()),
        description: Some("Rusoto codedeploy-commands integration test".to_owned()),
        file_exists_behavior: None,
        ignore_application_stop_failures: None,
        revision: Some(rusoto_codedeploy::RevisionLocation {
            revision_type: Some("S3".to_owned()),
            s_3_location: Some(rusoto_codedeploy::S3Location {
                bucket: Some("fake-bucket".to_owned()),
                bundle_type: Some("tgz".to_owned()),
                e_tag: None,
                key: Some("fake-key".to_owned()),
                version: None
            }),
            git_hub_location: None,
            string: None
        }),
        target_instances: None,
        update_outdated_instances_only: None
    };
    let response = cd.create_deployment(create_request).sync()
        .expect("Failed to invoke CodeDeploy::CreateDeployment to begin the deployment to the simulated on-premise instance");
    response.deployment_id.expect("CodeDeploy::CreateDeployment succeeded but did not return a deployment id")
}

fn process_all_deployment_commands<Credentials: rusoto_core::ProvideAwsCredentials + 'static> (
        region: Region,
        credentials: Credentials,
        session_arn: &str) {
    let cdc = rusoto_codedeploy_commands::CodeDeployCommandsClient::new(
        rusoto_core::reactor::RequestDispatcher::default(),
        credentials,
        region
    );

    for ref command_name in COMMAND_NAMES {
        process_single_instance_command(&cdc, true, command_name, session_arn);
    }
}

fn process_single_instance_command<CodeDeployCommands: rusoto_codedeploy_commands::CodeDeployCommands>(
        cdc: &CodeDeployCommands,
        succeed: bool,
        expected_command_name: &str,
        session_arn: &str) {
    // Poll.
    let poll_request = rusoto_codedeploy_commands::PollHostCommandInput {
        host_identifier: session_arn.to_owned()
    };
    let host_command = cdc.poll_host_command(poll_request).sync()
        .expect("Failed to invoke CodeDeployCommands::PollHostCommand to retrieve the command to be processed")
        .host_command.expect("CodeDeployCommands::PollHostCommand did not return a host command when one was expected to be available");
    println!("Got command {:?}", host_command);

    // Extract fields from the host command.
    let host_command_identifier = host_command.host_command_identifier.expect("No host command identifier received for polled command");
    let command_name = host_command.command_name.expect("No command name received for polled command");
    let deployment_execution_id = host_command.deployment_execution_id.expect("No deployment execution id received for polled command");
    let host_identifier = host_command.host_identifier.expect("No host identifier received for polled command");

    // Make sure we get the command we expect.
    assert_eq!(command_name, expected_command_name, "Received unexpected command name - has CodeDeploy modified the set or order of commands it uses?");

    // Acknowledge.
    let ack_request = rusoto_codedeploy_commands::PutHostCommandAcknowledgementInput {
        host_command_identifier: host_command_identifier.clone(),
        diagnostics: None
    };
    cdc.put_host_command_acknowledgement(ack_request).sync()
        .expect("Failed to invoke CodeDeployCommands::PutHostCommandAcknowledgement to acknowledge the polled host command");

    // Get deployment specification.
    let spec_request = rusoto_codedeploy_commands::GetDeploymentSpecificationInput {
        deployment_execution_id: deployment_execution_id.clone(),
        host_identifier: host_identifier.clone()
    };
    cdc.get_deployment_specification(spec_request).sync()
        .expect("Failed to invoke CodeDeployCommands::GetDeploymentSpecification to retrieve instructions for the polled host command");

    // Complete.
    let complete_request = rusoto_codedeploy_commands::PutHostCommandCompleteInput {
        command_status: if succeed { "Succeeded".to_owned() } else { "Failed".to_owned() },
        diagnostics: None,
        host_command_identifier: host_command_identifier.clone()
    };
    cdc.put_host_command_complete(complete_request).sync()
        .expect("Failed to invoke CodeDeployCommands::PutHostCommandComplete to finish processing the polled host command");
}

fn wait_for_successful_deployment_completion<CodeDeploy: rusoto_codedeploy::CodeDeploy>(
        cd: &CodeDeploy,
        deployment_id: &str) {
    let mut i = 0;
    loop {
        ::std::thread::sleep(::std::time::Duration::from_millis(1000));
        let request = rusoto_codedeploy::GetDeploymentInput {
            deployment_id: deployment_id.to_owned()
        };
        match cd.get_deployment(request).sync() {
            Ok(output) => {
                let info = output.deployment_info.expect("CodeDeploy::GetDeployment succeeded but returned no deployment info");
                let status = info.status.expect("CodeDeploy::GetDeployment succeeded but returned deployment info without a status");
                if status == "Succeeded" {
                    return;
                }
                if status == "Failed" || status == "Stopped" {
                    panic!("Deployment entered unexpected {} state", status);
                }
            },
            other => { panic!("Failed to invoke CodeDeploy::GetDeployment while waiting for successful deployment completion: {:?}", other); }
        }
        i += 1;
        if i >= 30 {
            panic!("The deployment failed to transition to the Succeeded state even after waiting 30 seconds");
        }
    }
}