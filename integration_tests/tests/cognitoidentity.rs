#![cfg(feature = "cognito-identity")]

extern crate rusoto_cognito_identity;
extern crate rusoto_iam;
extern crate rusoto_core;
extern crate time;

use rusoto_cognito_identity::{
    SetIdentityPoolRolesInput, 
    CognitoIdentity, 
    CognitoIdentityClient, 
    ListIdentitiesInput, 
    ListIdentityPoolsInput, 
    GetOpenIdTokenForDeveloperIdentityInput, 
    CreateIdentityPoolInput, 
    DeleteIdentityPoolInput, 
    CognitoProvider
};

use rusoto_iam::{
    Iam, 
    IamClient, 
    CreateRoleRequest, 
    PutRolePolicyRequest, 
    DeleteRoleRequest, 
    DeleteRolePolicyRequest
};

use rusoto_core::{Region, RusotoError};
use rusoto_credential::ProvideAwsCredentials;

use time::Time;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::delay_for;
use chrono::offset::Utc;

#[tokio::test]
async fn should_list_identity_pools() {
    let client = CognitoIdentityClient::new(Region::UsEast1);

    let mut request = ListIdentityPoolsInput::default();
    request.max_results = 10;

    client.list_identity_pools(request).await.unwrap();
}

#[tokio::test]
async fn should_handle_validation_errors_gracefully() {
    let client = CognitoIdentityClient::new(Region::UsEast1);

    let mut request = ListIdentitiesInput::default();
    request.max_results = 10;
    request.identity_pool_id = "invalid".to_string();

    match client.list_identities(request).await {
        Err(RusotoError::Validation(msg)) => assert!(msg.contains("identityPoolId")),
        err @ _ => panic!("Expected Validation error - got {:#?}", err),
    };
}

#[tokio::test]
async fn should_work_with_credential_provider() {
    let region = Region::UsEast1;
    let client_cognito = CognitoIdentityClient::new(region.clone());
    let client_iam = IamClient::new(region.clone());
    let tag = Time::now().nanosecond();
    let identity_pool_name = format!("cognito-provider-identity-tst-{}", tag);
    let role_name = format!("cognito-provider-auth-role-tst-{}", tag);
    let policy_name = format!("cognito-provider-auth-policy-tst-{}", tag);
    let developper_provider_name = "test_rusoto";

    // In order to perform this test, we have to create an identity pool, an IAM role, and link them together

    // Creating the identity pool
    let create_identity_input = CreateIdentityPoolInput {
        identity_pool_name: identity_pool_name.clone(),
        developer_provider_name: Some(developper_provider_name.to_string()),
        ..Default::default()
    };
    let identity_pool = client_cognito.create_identity_pool(create_identity_input).await.unwrap();

    // Creating a role for the identity pool
    let policy = format!("{{
        'Version': '2012-10-17',
        'Statement': [
          {{
            'Effect': 'Allow',
            'Principal': {{
              'Federated': 'cognito-identity.amazonaws.com'
            }},
            'Action': 'sts:AssumeRoleWithWebIdentity',
            'Condition': {{
              'StringEquals': {{
                'cognito-identity.amazonaws.com:aud': '{}'
              }}
            }}
          }}
        ]
      }}", identity_pool.identity_pool_id).replace("'", "\"");
      
    let input_create_role = CreateRoleRequest {
        assume_role_policy_document: policy,
        role_name: role_name,
        ..Default::default()
    };
    let role = client_iam.create_role(input_create_role).await.unwrap().role;

    // Adding a policy to the role
    let role_policy = "{
        'Version': '2012-10-17',
        'Statement': [
            {
                'Effect': 'Allow',
                'Action': [
                    'cognito-sync:*',
                    'cognito-identity:*'
                ],
                'Resource': [
                    '*'
                ]
            }
        ]
    }".replace("'", "\"");

    let role_policy_input = PutRolePolicyRequest {
        policy_document: role_policy,
        policy_name: policy_name.clone(),
        role_name: role.role_name.clone(),
        ..Default::default()
    };

    client_iam.put_role_policy(role_policy_input).await.unwrap();
    
    // Assigning the role to the identity pool
    let mut roles = HashMap::new();
    roles.insert("authenticated".to_string(), role.arn.clone());
    roles.insert("unauthenticated".to_string(), role.arn.clone());
    let roles_input = SetIdentityPoolRolesInput {
        identity_pool_id: identity_pool.identity_pool_id.clone(),
        roles: roles,
        ..Default::default()
    };
    client_cognito.set_identity_pool_roles(roles_input).await.unwrap();

    
    // Registering a user whose credentials will be tested
    let login = "login";
    let mut logins = HashMap::new();
    logins.insert(developper_provider_name.to_string(), login.to_string());
    let register_input = GetOpenIdTokenForDeveloperIdentityInput {
        identity_pool_id: identity_pool.identity_pool_id.clone(),
        logins: logins,
        token_duration: Some(3600),
        ..Default::default()
    };
    let response = client_cognito.get_open_id_token_for_developer_identity(register_input).await.unwrap();

    // It can take time for IAM role to be ready for use
    delay_for(Duration::from_secs(10)).await;

    // The test itself
    let provider = CognitoProvider::builder()
         .identity_id(response.identity_id.unwrap())
         .region(region.clone())
         .login("cognito-identity.amazonaws.com".to_string(), response.token.unwrap())
         .build();
    let creds = provider.credentials().await.unwrap();
    assert!(creds.token().is_some());
    assert!(creds.expires_at().is_some());
    assert!(creds.expires_at().unwrap().time() > Utc::now().time());

    // Cleaning time

    // Deleting the identity pool
    let delete_identity_pool_input = DeleteIdentityPoolInput {
        identity_pool_id: identity_pool.identity_pool_id.clone(),
        ..Default::default()
    };
    client_cognito.delete_identity_pool(delete_identity_pool_input).await.unwrap();

    // In order to delete the role, the policy must be deleted first
    let delete_policy_input = DeleteRolePolicyRequest {
        policy_name: policy_name,
        role_name: role.role_name.clone(),
        ..Default::default()
    };
    client_iam.delete_role_policy(delete_policy_input).await.unwrap();

    // Deleting the role
    let delete_role_input = DeleteRoleRequest {
        role_name: role.role_name,
        ..Default::default()
    };
    client_iam.delete_role(delete_role_input).await.unwrap();
}
