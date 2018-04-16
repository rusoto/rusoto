#![cfg(feature = "codedeploy-commands")]

extern crate rusoto_core;
extern crate rusoto_codedeploy;
extern crate rusoto_codedeploy_commands;
extern crate rusoto_iam;
extern crate rusoto_sts;

use rusoto_iam::{Iam, IamClient};
use rusoto_sts::{Sts, StsClient};
use rusoto_codedeploy::{CodeDeploy, CodeDeployClient, ListApplicationsInput};
use rusoto_core::{ProvideAwsCredentials, Region};
use rusoto_codedeploy_commands::{CodeDeployCommands, CodeDeployCommandsClient, PollHostCommandInput, PollHostCommandError};

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
/// - An IAM role, used to simulate an on-premise instance.
/// - A CodeDeploy on-premise instance registration.
/// - A CodeDeploy application and deployment group.
///
/// All of the resources are deleted if the test succeeds. If the test fails, some resources
/// may remain in the account. The resources are created with deterministic names, so if the
/// test is subsequently run successfully, it will detect the existing resources and clean
/// them up.
#[test]
fn successful_deployment_to_host() {
    let region = Region::UsEast1;
    let iam = IamClient::simple(region);
    let sts = StsClient::simple(region);
    let cd = CodeDeployClient::simple(region);
    let cdc = CodeDeployCommandsClient::simple(region);

    // TODO: Create on-premise IAM role.
    // TODO: Assume role to get on-premise session credentials.
    // TODO: Create CodeDeploy application.
    // TODO: Create CodeDeploy deployment group.
    // TODO: Register and tag the on-premise instance.
    // TODO: Start a CodeDeploy deployment.
    // TODO: Process each command sequentially, returning success for each.
    // TODO: Wait for deployment to complete. Verify it completed successfully.
    // TODO: Unregister/untag the on-premise instance.
    // TODO: Delete CodeDeploy deployment group.
    // TODO: Delete CodeDeploy application.
    // TODO: Delete on-premise IAM role.
}
