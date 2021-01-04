// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl IotThingsGraphClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "iotthingsgraph", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateEntityToThingRequest {
    /// <p>The ID of the device to be associated with the thing.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:device:DEVICENAME</code> </p>
    #[serde(rename = "entityId")]
    pub entity_id: String,
    /// <p>The version of the user's namespace. Defaults to the latest version of the user's namespace.</p>
    #[serde(rename = "namespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_version: Option<i64>,
    /// <p>The name of the thing to which the entity is to be associated.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateEntityToThingResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFlowTemplateRequest {
    /// <p>The namespace version in which the workflow is to be created.</p> <p>If no value is specified, the latest version is used by default.</p>
    #[serde(rename = "compatibleNamespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_namespace_version: Option<i64>,
    /// <p>The workflow <code>DefinitionDocument</code>.</p>
    #[serde(rename = "definition")]
    pub definition: DefinitionDocument,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFlowTemplateResponse {
    /// <p>The summary object that describes the created workflow.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<FlowTemplateSummary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSystemInstanceRequest {
    #[serde(rename = "definition")]
    pub definition: DefinitionDocument,
    /// <p>The ARN of the IAM role that AWS IoT Things Graph will assume when it executes the flow. This role must have read and write access to AWS Lambda and AWS IoT and any other AWS services that the flow uses when it executes. This value is required if the value of the <code>target</code> parameter is <code>CLOUD</code>.</p>
    #[serde(rename = "flowActionsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_actions_role_arn: Option<String>,
    /// <p>The name of the Greengrass group where the system instance will be deployed. This value is required if the value of the <code>target</code> parameter is <code>GREENGRASS</code>.</p>
    #[serde(rename = "greengrassGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greengrass_group_name: Option<String>,
    #[serde(rename = "metricsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_configuration: Option<MetricsConfiguration>,
    /// <p>The name of the Amazon Simple Storage Service bucket that will be used to store and deploy the system instance's resource file. This value is required if the value of the <code>target</code> parameter is <code>GREENGRASS</code>.</p>
    #[serde(rename = "s3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket_name: Option<String>,
    /// <p>Metadata, consisting of key-value pairs, that can be used to categorize your system instances.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The target type of the deployment. Valid values are <code>GREENGRASS</code> and <code>CLOUD</code>.</p>
    #[serde(rename = "target")]
    pub target: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSystemInstanceResponse {
    /// <p>The summary object that describes the new system instance.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SystemInstanceSummary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSystemTemplateRequest {
    /// <p>The namespace version in which the system is to be created.</p> <p>If no value is specified, the latest version is used by default.</p>
    #[serde(rename = "compatibleNamespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_namespace_version: Option<i64>,
    /// <p>The <code>DefinitionDocument</code> used to create the system.</p>
    #[serde(rename = "definition")]
    pub definition: DefinitionDocument,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSystemTemplateResponse {
    /// <p>The summary object that describes the created system.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SystemTemplateSummary>,
}

/// <p>A document that defines an entity. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DefinitionDocument {
    /// <p>The language used to define the entity. <code>GRAPHQL</code> is the only valid value.</p>
    #[serde(rename = "language")]
    pub language: String,
    /// <p>The GraphQL text that defines the entity.</p>
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFlowTemplateRequest {
    /// <p>The ID of the workflow to be deleted.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:workflow:WORKFLOWNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFlowTemplateResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNamespaceRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNamespaceResponse {
    /// <p>The ARN of the namespace to be deleted.</p>
    #[serde(rename = "namespaceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_arn: Option<String>,
    /// <p>The name of the namespace to be deleted.</p>
    #[serde(rename = "namespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSystemInstanceRequest {
    /// <p>The ID of the system instance to be deleted.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSystemInstanceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSystemTemplateRequest {
    /// <p>The ID of the system to be deleted.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:system:SYSTEMNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSystemTemplateResponse {}

/// <p>An object that contains the ID and revision number of a workflow or system that is part of a deployment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DependencyRevision {
    /// <p>The ID of the workflow or system.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The revision number of the workflow or system.</p>
    #[serde(rename = "revisionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_number: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeploySystemInstanceRequest {
    /// <p>The ID of the system instance. This value is returned by the <code>CreateSystemInstance</code> action.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:deployment:DEPLOYMENTNAME</code> </p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeploySystemInstanceResponse {
    /// <p>The ID of the Greengrass deployment used to deploy the system instance.</p>
    #[serde(rename = "greengrassDeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greengrass_deployment_id: Option<String>,
    /// <p>An object that contains summary information about a system instance that was deployed. </p>
    #[serde(rename = "summary")]
    pub summary: SystemInstanceSummary,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeprecateFlowTemplateRequest {
    /// <p>The ID of the workflow to be deleted.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:workflow:WORKFLOWNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeprecateFlowTemplateResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeprecateSystemTemplateRequest {
    /// <p>The ID of the system to delete.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:system:SYSTEMNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeprecateSystemTemplateResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeNamespaceRequest {
    /// <p>The name of the user's namespace. Set this to <code>aws</code> to get the public namespace.</p>
    #[serde(rename = "namespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeNamespaceResponse {
    /// <p>The ARN of the namespace.</p>
    #[serde(rename = "namespaceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_arn: Option<String>,
    /// <p>The name of the namespace.</p>
    #[serde(rename = "namespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// <p>The version of the user's namespace to describe.</p>
    #[serde(rename = "namespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_version: Option<i64>,
    /// <p>The name of the public namespace that the latest namespace version is tracking.</p>
    #[serde(rename = "trackingNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_namespace_name: Option<String>,
    /// <p>The version of the public namespace that the latest version is tracking.</p>
    #[serde(rename = "trackingNamespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_namespace_version: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DissociateEntityFromThingRequest {
    /// <p>The entity type from which to disassociate the thing.</p>
    #[serde(rename = "entityType")]
    pub entity_type: String,
    /// <p>The name of the thing to disassociate.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DissociateEntityFromThingResponse {}

/// <p>Describes the properties of an entity.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EntityDescription {
    /// <p>The entity ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time at which the entity was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The definition document of the entity.</p>
    #[serde(rename = "definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DefinitionDocument>,
    /// <p>The entity ID.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The entity type.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>An object that filters an entity search. Multiple filters function as OR criteria in the search. For example a search that includes a <code>NAMESPACE</code> and a <code>REFERENCED_ENTITY_ID</code> filter searches for entities in the specified namespace that use the entity specified by the value of <code>REFERENCED_ENTITY_ID</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EntityFilter {
    /// <p>The name of the entity search filter field. <code>REFERENCED_ENTITY_ID</code> filters on entities that are used by the entity in the result set. For example, you can filter on the ID of a property that is used in a state.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of string values for the search filter field. Multiple values function as AND criteria in the search.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// <p>An object that contains information about a flow event.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FlowExecutionMessage {
    /// <p>The type of flow event .</p>
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// <p>The unique identifier of the message.</p>
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// <p>A string containing information about the flow event.</p>
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    /// <p>The date and time when the message was last updated.</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

/// <p>An object that contains summary information about a flow execution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FlowExecutionSummary {
    /// <p>The date and time when the flow execution summary was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The ID of the flow execution.</p>
    #[serde(rename = "flowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_execution_id: Option<String>,
    /// <p>The ID of the flow.</p>
    #[serde(rename = "flowTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_template_id: Option<String>,
    /// <p>The current status of the flow execution.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The ID of the system instance that contains the flow.</p>
    #[serde(rename = "systemInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instance_id: Option<String>,
    /// <p>The date and time when the flow execution summary was last updated.</p>
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>An object that contains a workflow's definition and summary information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FlowTemplateDescription {
    /// <p>A workflow's definition document.</p>
    #[serde(rename = "definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DefinitionDocument>,
    /// <p>An object that contains summary information about a workflow.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<FlowTemplateSummary>,
    /// <p>The version of the user's namespace against which the workflow was validated. Use this value in your system instance.</p>
    #[serde(rename = "validatedNamespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated_namespace_version: Option<i64>,
}

/// <p>An object that filters a workflow search.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FlowTemplateFilter {
    /// <p>The name of the search filter field.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>An array of string values for the search filter field. Multiple values function as AND criteria in the search.</p>
    #[serde(rename = "value")]
    pub value: Vec<String>,
}

/// <p>An object that contains summary information about a workflow.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FlowTemplateSummary {
    /// <p>The ARN of the workflow.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the workflow was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The ID of the workflow.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The revision number of the workflow.</p>
    #[serde(rename = "revisionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_number: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEntitiesRequest {
    /// <p>An array of entity IDs.</p> <p>The IDs should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:device:DEVICENAME</code> </p>
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    /// <p>The version of the user's namespace. Defaults to the latest version of the user's namespace.</p>
    #[serde(rename = "namespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_version: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEntitiesResponse {
    /// <p>An array of descriptions for the specified entities.</p>
    #[serde(rename = "descriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<EntityDescription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFlowTemplateRequest {
    /// <p>The ID of the workflow.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:workflow:WORKFLOWNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The number of the workflow revision to retrieve.</p>
    #[serde(rename = "revisionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_number: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFlowTemplateResponse {
    /// <p>The object that describes the specified workflow.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<FlowTemplateDescription>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFlowTemplateRevisionsRequest {
    /// <p>The ID of the workflow.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:workflow:WORKFLOWNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFlowTemplateRevisionsResponse {
    /// <p>The string to specify as <code>nextToken</code> when you request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that provide summary data about each revision.</p>
    #[serde(rename = "summaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<FlowTemplateSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetNamespaceDeletionStatusRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetNamespaceDeletionStatusResponse {
    /// <p>An error code returned by the namespace deletion task.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>An error code returned by the namespace deletion task.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ARN of the namespace that is being deleted.</p>
    #[serde(rename = "namespaceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_arn: Option<String>,
    /// <p>The name of the namespace that is being deleted.</p>
    #[serde(rename = "namespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// <p>The status of the deletion request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSystemInstanceRequest {
    /// <p>The ID of the system deployment instance. This value is returned by <code>CreateSystemInstance</code>.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:deployment:DEPLOYMENTNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSystemInstanceResponse {
    /// <p>An object that describes the system instance.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SystemInstanceDescription>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSystemTemplateRequest {
    /// <p>The ID of the system to get. This ID must be in the user's namespace.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:system:SYSTEMNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The number that specifies the revision of the system to get.</p>
    #[serde(rename = "revisionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_number: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSystemTemplateResponse {
    /// <p>An object that contains summary data about the system.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SystemTemplateDescription>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSystemTemplateRevisionsRequest {
    /// <p>The ID of the system template.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:system:SYSTEMNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSystemTemplateRevisionsResponse {
    /// <p>The string to specify as <code>nextToken</code> when you request the next page of results. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that contain summary data about the system template revisions.</p>
    #[serde(rename = "summaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<SystemTemplateSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUploadStatusRequest {
    /// <p>The ID of the upload. This value is returned by the <code>UploadEntityDefinitions</code> action.</p>
    #[serde(rename = "uploadId")]
    pub upload_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUploadStatusResponse {
    /// <p>The date at which the upload was created.</p>
    #[serde(rename = "createdDate")]
    pub created_date: f64,
    /// <p>The reason for an upload failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<Vec<String>>,
    /// <p>The ARN of the upload.</p>
    #[serde(rename = "namespaceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_arn: Option<String>,
    /// <p>The name of the upload's namespace.</p>
    #[serde(rename = "namespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// <p>The version of the user's namespace. Defaults to the latest version of the user's namespace.</p>
    #[serde(rename = "namespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_version: Option<i64>,
    /// <p>The ID of the upload.</p>
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    /// <p>The status of the upload. The initial status is <code>IN_PROGRESS</code>. The response show all validation failures if the upload fails.</p>
    #[serde(rename = "uploadStatus")]
    pub upload_status: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFlowExecutionMessagesRequest {
    /// <p>The ID of the flow execution.</p>
    #[serde(rename = "flowExecutionId")]
    pub flow_execution_id: String,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFlowExecutionMessagesResponse {
    /// <p>A list of objects that contain information about events in the specified flow execution.</p>
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<FlowExecutionMessage>>,
    /// <p>The string to specify as <code>nextToken</code> when you request the next page of results. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The maximum number of tags to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource whose tags are to be returned.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The token that specifies the next page of results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of tags returned by the <code>ListTagsForResource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>An object that specifies whether cloud metrics are collected in a deployment and, if so, what role is used to collect metrics.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MetricsConfiguration {
    /// <p>A Boolean that specifies whether cloud metrics are collected.</p>
    #[serde(rename = "cloudMetricEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_metric_enabled: Option<bool>,
    /// <p>The ARN of the role that is used to collect cloud metrics.</p>
    #[serde(rename = "metricRuleRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_rule_role_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchEntitiesRequest {
    /// <p>The entity types for which to search.</p>
    #[serde(rename = "entityTypes")]
    pub entity_types: Vec<String>,
    /// <p>Optional filter to apply to the search. Valid filters are <code>NAME</code> <code>NAMESPACE</code>, <code>SEMANTIC_TYPE_PATH</code> and <code>REFERENCED_ENTITY_ID</code>. <code>REFERENCED_ENTITY_ID</code> filters on entities that are used by the entity in the result set. For example, you can filter on the ID of a property that is used in a state.</p> <p>Multiple filters function as OR criteria in the query. Multiple values passed inside the filter function as AND criteria.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<EntityFilter>>,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The version of the user's namespace. Defaults to the latest version of the user's namespace.</p>
    #[serde(rename = "namespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_version: Option<i64>,
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchEntitiesResponse {
    /// <p>An array of descriptions for each entity returned in the search result.</p>
    #[serde(rename = "descriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<EntityDescription>>,
    /// <p>The string to specify as <code>nextToken</code> when you request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchFlowExecutionsRequest {
    /// <p>The date and time of the latest flow execution to return.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The ID of a flow execution.</p>
    #[serde(rename = "flowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_execution_id: Option<String>,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The date and time of the earliest flow execution to return.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The ID of the system instance that contains the flow.</p>
    #[serde(rename = "systemInstanceId")]
    pub system_instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchFlowExecutionsResponse {
    /// <p>The string to specify as <code>nextToken</code> when you request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that contain summary information about each workflow execution in the result set.</p>
    #[serde(rename = "summaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<FlowExecutionSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchFlowTemplatesRequest {
    /// <p>An array of objects that limit the result set. The only valid filter is <code>DEVICE_MODEL_ID</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<FlowTemplateFilter>>,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchFlowTemplatesResponse {
    /// <p>The string to specify as <code>nextToken</code> when you request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that contain summary information about each workflow in the result set.</p>
    #[serde(rename = "summaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<FlowTemplateSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchSystemInstancesRequest {
    /// <p>Optional filter to apply to the search. Valid filters are <code>SYSTEM_TEMPLATE_ID</code>, <code>STATUS</code>, and <code>GREENGRASS_GROUP_NAME</code>.</p> <p>Multiple filters function as OR criteria in the query. Multiple values passed inside the filter function as AND criteria.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SystemInstanceFilter>>,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchSystemInstancesResponse {
    /// <p>The string to specify as <code>nextToken</code> when you request the next page of results. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that contain summary data abour the system instances in the result set.</p>
    #[serde(rename = "summaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<SystemInstanceSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchSystemTemplatesRequest {
    /// <p>An array of filters that limit the result set. The only valid filter is <code>FLOW_TEMPLATE_ID</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SystemTemplateFilter>>,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchSystemTemplatesResponse {
    /// <p>The string to specify as <code>nextToken</code> when you request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that contain summary information about each system deployment in the result set.</p>
    #[serde(rename = "summaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<SystemTemplateSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchThingsRequest {
    /// <p>The ID of the entity to which the things are associated.</p> <p>The IDs should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:device:DEVICENAME</code> </p>
    #[serde(rename = "entityId")]
    pub entity_id: String,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The version of the user's namespace. Defaults to the latest version of the user's namespace.</p>
    #[serde(rename = "namespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_version: Option<i64>,
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchThingsResponse {
    /// <p>The string to specify as <code>nextToken</code> when you request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of things in the result set.</p>
    #[serde(rename = "things")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub things: Option<Vec<Thing>>,
}

/// <p>An object that contains a system instance definition and summary information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SystemInstanceDescription {
    #[serde(rename = "definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DefinitionDocument>,
    /// <p>The AWS Identity and Access Management (IAM) role that AWS IoT Things Graph assumes during flow execution in a cloud deployment. This role must have read and write permissionss to AWS Lambda and AWS IoT and to any other AWS services that the flow uses.</p>
    #[serde(rename = "flowActionsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_actions_role_arn: Option<String>,
    #[serde(rename = "metricsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_configuration: Option<MetricsConfiguration>,
    /// <p>The Amazon Simple Storage Service bucket where information about a system instance is stored.</p>
    #[serde(rename = "s3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket_name: Option<String>,
    /// <p>An object that contains summary information about a system instance.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SystemInstanceSummary>,
    /// <p>A list of objects that contain all of the IDs and revision numbers of workflows and systems that are used in a system instance.</p>
    #[serde(rename = "validatedDependencyRevisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated_dependency_revisions: Option<Vec<DependencyRevision>>,
    /// <p>The version of the user's namespace against which the system instance was validated.</p>
    #[serde(rename = "validatedNamespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated_namespace_version: Option<i64>,
}

/// <p>An object that filters a system instance search. Multiple filters function as OR criteria in the search. For example a search that includes a GREENGRASS_GROUP_NAME and a STATUS filter searches for system instances in the specified Greengrass group that have the specified status.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SystemInstanceFilter {
    /// <p>The name of the search filter field.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of string values for the search filter field. Multiple values function as AND criteria in the search. </p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// <p>An object that contains summary information about a system instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SystemInstanceSummary {
    /// <p>The ARN of the system instance.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the system instance was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The ID of the Greengrass group where the system instance is deployed.</p>
    #[serde(rename = "greengrassGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greengrass_group_id: Option<String>,
    /// <p>The ID of the Greengrass group where the system instance is deployed.</p>
    #[serde(rename = "greengrassGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greengrass_group_name: Option<String>,
    /// <p>The version of the Greengrass group where the system instance is deployed.</p>
    #[serde(rename = "greengrassGroupVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greengrass_group_version_id: Option<String>,
    /// <p>The ID of the system instance.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The status of the system instance.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The target of the system instance.</p>
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p> The date and time when the system instance was last updated.</p>
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>An object that contains a system's definition document and summary information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SystemTemplateDescription {
    /// <p>The definition document of a system.</p>
    #[serde(rename = "definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DefinitionDocument>,
    /// <p>An object that contains summary information about a system.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SystemTemplateSummary>,
    /// <p>The namespace version against which the system was validated. Use this value in your system instance.</p>
    #[serde(rename = "validatedNamespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated_namespace_version: Option<i64>,
}

/// <p>An object that filters a system search.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SystemTemplateFilter {
    /// <p>The name of the system search filter field.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>An array of string values for the search filter field. Multiple values function as AND criteria in the search.</p>
    #[serde(rename = "value")]
    pub value: Vec<String>,
}

/// <p>An object that contains information about a system.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SystemTemplateSummary {
    /// <p>The ARN of the system.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the system was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The ID of the system.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The revision number of the system.</p>
    #[serde(rename = "revisionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_number: Option<i64>,
}

/// <p>Metadata assigned to an AWS IoT Things Graph resource consisting of a key-value pair.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The required name of the tag. The string value can be from 1 to 128 Unicode characters in length.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The optional value of the tag. The string value can be from 1 to 256 Unicode characters in length.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource whose tags are returned.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A list of tags to add to the resource.&gt;</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>An AWS IoT thing.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Thing {
    /// <p>The ARN of the thing.</p>
    #[serde(rename = "thingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UndeploySystemInstanceRequest {
    /// <p>The ID of the system instance to remove from its target.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UndeploySystemInstanceResponse {
    /// <p>An object that contains summary information about the system instance that was removed from its target.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SystemInstanceSummary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource whose tags are to be removed.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A list of tag key names to remove from the resource. You don't specify the value. Both the key and its associated value are removed. </p> <p>This parameter to the API requires a JSON text string argument. For information on how to format a JSON parameter for the various command line tool environments, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-parameters.html#cli-using-param-json">Using JSON for Parameters</a> in the <i>AWS CLI User Guide</i>. </p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFlowTemplateRequest {
    /// <p>The version of the user's namespace.</p> <p>If no value is specified, the latest version is used by default. Use the <code>GetFlowTemplateRevisions</code> if you want to find earlier revisions of the flow to update.</p>
    #[serde(rename = "compatibleNamespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_namespace_version: Option<i64>,
    /// <p>The <code>DefinitionDocument</code> that contains the updated workflow definition.</p>
    #[serde(rename = "definition")]
    pub definition: DefinitionDocument,
    /// <p>The ID of the workflow to be updated.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:workflow:WORKFLOWNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFlowTemplateResponse {
    /// <p>An object containing summary information about the updated workflow.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<FlowTemplateSummary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSystemTemplateRequest {
    /// <p>The version of the user's namespace. Defaults to the latest version of the user's namespace.</p> <p>If no value is specified, the latest version is used by default.</p>
    #[serde(rename = "compatibleNamespaceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_namespace_version: Option<i64>,
    /// <p>The <code>DefinitionDocument</code> that contains the updated system definition.</p>
    #[serde(rename = "definition")]
    pub definition: DefinitionDocument,
    /// <p>The ID of the system to be updated.</p> <p>The ID should be in the following format.</p> <p> <code>urn:tdm:REGION/ACCOUNT ID/default:system:SYSTEMNAME</code> </p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSystemTemplateResponse {
    /// <p>An object containing summary information about the updated system.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SystemTemplateSummary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UploadEntityDefinitionsRequest {
    /// <p>A Boolean that specifies whether to deprecate all entities in the latest version before uploading the new <code>DefinitionDocument</code>. If set to <code>true</code>, the upload will create a new namespace version.</p>
    #[serde(rename = "deprecateExistingEntities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecate_existing_entities: Option<bool>,
    /// <p>The <code>DefinitionDocument</code> that defines the updated entities.</p>
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DefinitionDocument>,
    /// <p>A Boolean that specifies whether to synchronize with the latest version of the public namespace. If set to <code>true</code>, the upload will create a new namespace version.</p>
    #[serde(rename = "syncWithPublicNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_with_public_namespace: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UploadEntityDefinitionsResponse {
    /// <p>The ID that specifies the upload action. You can use this to track the status of the upload.</p>
    #[serde(rename = "uploadId")]
    pub upload_id: String,
}

/// Errors returned by AssociateEntityToThing
#[derive(Debug, PartialEq)]
pub enum AssociateEntityToThingError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl AssociateEntityToThingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateEntityToThingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(AssociateEntityToThingError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AssociateEntityToThingError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateEntityToThingError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateEntityToThingError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateEntityToThingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateEntityToThingError::InternalFailure(ref cause) => write!(f, "{}", cause),
            AssociateEntityToThingError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AssociateEntityToThingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateEntityToThingError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateEntityToThingError {}
/// Errors returned by CreateFlowTemplate
#[derive(Debug, PartialEq)]
pub enum CreateFlowTemplateError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    LimitExceeded(String),
    /// <p><p/></p>
    ResourceAlreadyExists(String),
    /// <p><p/></p>
    Throttling(String),
}

impl CreateFlowTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFlowTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateFlowTemplateError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateFlowTemplateError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateFlowTemplateError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateFlowTemplateError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateFlowTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFlowTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFlowTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateFlowTemplateError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateFlowTemplateError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateFlowTemplateError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateFlowTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFlowTemplateError {}
/// Errors returned by CreateSystemInstance
#[derive(Debug, PartialEq)]
pub enum CreateSystemInstanceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    LimitExceeded(String),
    /// <p><p/></p>
    ResourceAlreadyExists(String),
    /// <p><p/></p>
    Throttling(String),
}

impl CreateSystemInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSystemInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateSystemInstanceError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateSystemInstanceError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateSystemInstanceError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateSystemInstanceError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateSystemInstanceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSystemInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSystemInstanceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateSystemInstanceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateSystemInstanceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSystemInstanceError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateSystemInstanceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSystemInstanceError {}
/// Errors returned by CreateSystemTemplate
#[derive(Debug, PartialEq)]
pub enum CreateSystemTemplateError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceAlreadyExists(String),
    /// <p><p/></p>
    Throttling(String),
}

impl CreateSystemTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSystemTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateSystemTemplateError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateSystemTemplateError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateSystemTemplateError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateSystemTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSystemTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSystemTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateSystemTemplateError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateSystemTemplateError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateSystemTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSystemTemplateError {}
/// Errors returned by DeleteFlowTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteFlowTemplateError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DeleteFlowTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFlowTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteFlowTemplateError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteFlowTemplateError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteFlowTemplateError::ResourceInUse(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteFlowTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFlowTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFlowTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteFlowTemplateError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteFlowTemplateError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteFlowTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFlowTemplateError {}
/// Errors returned by DeleteNamespace
#[derive(Debug, PartialEq)]
pub enum DeleteNamespaceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DeleteNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNamespaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteNamespaceError::InternalFailure(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteNamespaceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteNamespaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNamespaceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteNamespaceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNamespaceError {}
/// Errors returned by DeleteSystemInstance
#[derive(Debug, PartialEq)]
pub enum DeleteSystemInstanceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DeleteSystemInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSystemInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteSystemInstanceError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteSystemInstanceError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteSystemInstanceError::ResourceInUse(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteSystemInstanceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSystemInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSystemInstanceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteSystemInstanceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteSystemInstanceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteSystemInstanceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSystemInstanceError {}
/// Errors returned by DeleteSystemTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteSystemTemplateError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DeleteSystemTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSystemTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteSystemTemplateError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteSystemTemplateError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteSystemTemplateError::ResourceInUse(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteSystemTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSystemTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSystemTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteSystemTemplateError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteSystemTemplateError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteSystemTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSystemTemplateError {}
/// Errors returned by DeploySystemInstance
#[derive(Debug, PartialEq)]
pub enum DeploySystemInstanceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DeploySystemInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeploySystemInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeploySystemInstanceError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeploySystemInstanceError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeploySystemInstanceError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeploySystemInstanceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeploySystemInstanceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeploySystemInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeploySystemInstanceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeploySystemInstanceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeploySystemInstanceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeploySystemInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeploySystemInstanceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeploySystemInstanceError {}
/// Errors returned by DeprecateFlowTemplate
#[derive(Debug, PartialEq)]
pub enum DeprecateFlowTemplateError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DeprecateFlowTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeprecateFlowTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeprecateFlowTemplateError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeprecateFlowTemplateError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeprecateFlowTemplateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeprecateFlowTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeprecateFlowTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeprecateFlowTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeprecateFlowTemplateError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeprecateFlowTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeprecateFlowTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeprecateFlowTemplateError {}
/// Errors returned by DeprecateSystemTemplate
#[derive(Debug, PartialEq)]
pub enum DeprecateSystemTemplateError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DeprecateSystemTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeprecateSystemTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeprecateSystemTemplateError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeprecateSystemTemplateError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeprecateSystemTemplateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeprecateSystemTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeprecateSystemTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeprecateSystemTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeprecateSystemTemplateError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeprecateSystemTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeprecateSystemTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeprecateSystemTemplateError {}
/// Errors returned by DescribeNamespace
#[derive(Debug, PartialEq)]
pub enum DescribeNamespaceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DescribeNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeNamespaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeNamespaceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeNamespaceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeNamespaceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeNamespaceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeNamespaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeNamespaceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeNamespaceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeNamespaceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeNamespaceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeNamespaceError {}
/// Errors returned by DissociateEntityFromThing
#[derive(Debug, PartialEq)]
pub enum DissociateEntityFromThingError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DissociateEntityFromThingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DissociateEntityFromThingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DissociateEntityFromThingError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DissociateEntityFromThingError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DissociateEntityFromThingError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DissociateEntityFromThingError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DissociateEntityFromThingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DissociateEntityFromThingError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DissociateEntityFromThingError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DissociateEntityFromThingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DissociateEntityFromThingError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DissociateEntityFromThingError {}
/// Errors returned by GetEntities
#[derive(Debug, PartialEq)]
pub enum GetEntitiesError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl GetEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEntitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetEntitiesError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetEntitiesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetEntitiesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetEntitiesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEntitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEntitiesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetEntitiesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetEntitiesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetEntitiesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEntitiesError {}
/// Errors returned by GetFlowTemplate
#[derive(Debug, PartialEq)]
pub enum GetFlowTemplateError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl GetFlowTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFlowTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetFlowTemplateError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetFlowTemplateError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFlowTemplateError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetFlowTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFlowTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFlowTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetFlowTemplateError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetFlowTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFlowTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFlowTemplateError {}
/// Errors returned by GetFlowTemplateRevisions
#[derive(Debug, PartialEq)]
pub enum GetFlowTemplateRevisionsError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl GetFlowTemplateRevisionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFlowTemplateRevisionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetFlowTemplateRevisionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetFlowTemplateRevisionsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFlowTemplateRevisionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetFlowTemplateRevisionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFlowTemplateRevisionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFlowTemplateRevisionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetFlowTemplateRevisionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetFlowTemplateRevisionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFlowTemplateRevisionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFlowTemplateRevisionsError {}
/// Errors returned by GetNamespaceDeletionStatus
#[derive(Debug, PartialEq)]
pub enum GetNamespaceDeletionStatusError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    Throttling(String),
}

impl GetNamespaceDeletionStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetNamespaceDeletionStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetNamespaceDeletionStatusError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetNamespaceDeletionStatusError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetNamespaceDeletionStatusError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetNamespaceDeletionStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetNamespaceDeletionStatusError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetNamespaceDeletionStatusError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetNamespaceDeletionStatusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetNamespaceDeletionStatusError {}
/// Errors returned by GetSystemInstance
#[derive(Debug, PartialEq)]
pub enum GetSystemInstanceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl GetSystemInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSystemInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetSystemInstanceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetSystemInstanceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSystemInstanceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetSystemInstanceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSystemInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSystemInstanceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetSystemInstanceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetSystemInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetSystemInstanceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSystemInstanceError {}
/// Errors returned by GetSystemTemplate
#[derive(Debug, PartialEq)]
pub enum GetSystemTemplateError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl GetSystemTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSystemTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetSystemTemplateError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetSystemTemplateError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSystemTemplateError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetSystemTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSystemTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSystemTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetSystemTemplateError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetSystemTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetSystemTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSystemTemplateError {}
/// Errors returned by GetSystemTemplateRevisions
#[derive(Debug, PartialEq)]
pub enum GetSystemTemplateRevisionsError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl GetSystemTemplateRevisionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetSystemTemplateRevisionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetSystemTemplateRevisionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetSystemTemplateRevisionsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSystemTemplateRevisionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetSystemTemplateRevisionsError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSystemTemplateRevisionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSystemTemplateRevisionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetSystemTemplateRevisionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetSystemTemplateRevisionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetSystemTemplateRevisionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSystemTemplateRevisionsError {}
/// Errors returned by GetUploadStatus
#[derive(Debug, PartialEq)]
pub enum GetUploadStatusError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl GetUploadStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUploadStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetUploadStatusError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetUploadStatusError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetUploadStatusError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetUploadStatusError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUploadStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUploadStatusError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetUploadStatusError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetUploadStatusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetUploadStatusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUploadStatusError {}
/// Errors returned by ListFlowExecutionMessages
#[derive(Debug, PartialEq)]
pub enum ListFlowExecutionMessagesError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl ListFlowExecutionMessagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFlowExecutionMessagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListFlowExecutionMessagesError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListFlowExecutionMessagesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFlowExecutionMessagesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListFlowExecutionMessagesError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFlowExecutionMessagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFlowExecutionMessagesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListFlowExecutionMessagesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListFlowExecutionMessagesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListFlowExecutionMessagesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFlowExecutionMessagesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceAlreadyExists(String),
    /// <p><p/></p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTagsForResourceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by SearchEntities
#[derive(Debug, PartialEq)]
pub enum SearchEntitiesError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    Throttling(String),
}

impl SearchEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchEntitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(SearchEntitiesError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SearchEntitiesError::InvalidRequest(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchEntitiesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchEntitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchEntitiesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            SearchEntitiesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SearchEntitiesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchEntitiesError {}
/// Errors returned by SearchFlowExecutions
#[derive(Debug, PartialEq)]
pub enum SearchFlowExecutionsError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl SearchFlowExecutionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchFlowExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(SearchFlowExecutionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SearchFlowExecutionsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SearchFlowExecutionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchFlowExecutionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchFlowExecutionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchFlowExecutionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            SearchFlowExecutionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SearchFlowExecutionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SearchFlowExecutionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchFlowExecutionsError {}
/// Errors returned by SearchFlowTemplates
#[derive(Debug, PartialEq)]
pub enum SearchFlowTemplatesError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    Throttling(String),
}

impl SearchFlowTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchFlowTemplatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(SearchFlowTemplatesError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SearchFlowTemplatesError::InvalidRequest(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchFlowTemplatesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchFlowTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchFlowTemplatesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            SearchFlowTemplatesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SearchFlowTemplatesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchFlowTemplatesError {}
/// Errors returned by SearchSystemInstances
#[derive(Debug, PartialEq)]
pub enum SearchSystemInstancesError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    Throttling(String),
}

impl SearchSystemInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchSystemInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(SearchSystemInstancesError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SearchSystemInstancesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchSystemInstancesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchSystemInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchSystemInstancesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            SearchSystemInstancesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SearchSystemInstancesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchSystemInstancesError {}
/// Errors returned by SearchSystemTemplates
#[derive(Debug, PartialEq)]
pub enum SearchSystemTemplatesError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    Throttling(String),
}

impl SearchSystemTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchSystemTemplatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(SearchSystemTemplatesError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SearchSystemTemplatesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchSystemTemplatesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchSystemTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchSystemTemplatesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            SearchSystemTemplatesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SearchSystemTemplatesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchSystemTemplatesError {}
/// Errors returned by SearchThings
#[derive(Debug, PartialEq)]
pub enum SearchThingsError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl SearchThingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchThingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(SearchThingsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SearchThingsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SearchThingsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchThingsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchThingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchThingsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            SearchThingsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SearchThingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SearchThingsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchThingsError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceAlreadyExists(String),
    /// <p><p/></p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(TagResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(TagResourceError::ResourceAlreadyExists(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(TagResourceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UndeploySystemInstance
#[derive(Debug, PartialEq)]
pub enum UndeploySystemInstanceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl UndeploySystemInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UndeploySystemInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UndeploySystemInstanceError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UndeploySystemInstanceError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UndeploySystemInstanceError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UndeploySystemInstanceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UndeploySystemInstanceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UndeploySystemInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UndeploySystemInstanceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UndeploySystemInstanceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UndeploySystemInstanceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UndeploySystemInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UndeploySystemInstanceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UndeploySystemInstanceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceAlreadyExists(String),
    /// <p><p/></p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UntagResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(UntagResourceError::ResourceAlreadyExists(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UntagResourceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateFlowTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateFlowTemplateError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl UpdateFlowTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFlowTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateFlowTemplateError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateFlowTemplateError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateFlowTemplateError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateFlowTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFlowTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFlowTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateFlowTemplateError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateFlowTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateFlowTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFlowTemplateError {}
/// Errors returned by UpdateSystemTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateSystemTemplateError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl UpdateSystemTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSystemTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateSystemTemplateError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateSystemTemplateError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateSystemTemplateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateSystemTemplateError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSystemTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSystemTemplateError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateSystemTemplateError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateSystemTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateSystemTemplateError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSystemTemplateError {}
/// Errors returned by UploadEntityDefinitions
#[derive(Debug, PartialEq)]
pub enum UploadEntityDefinitionsError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    Throttling(String),
}

impl UploadEntityDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UploadEntityDefinitionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UploadEntityDefinitionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UploadEntityDefinitionsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UploadEntityDefinitionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UploadEntityDefinitionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UploadEntityDefinitionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UploadEntityDefinitionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UploadEntityDefinitionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UploadEntityDefinitionsError {}
/// Trait representing the capabilities of the AWS IoT Things Graph API. AWS IoT Things Graph clients implement this trait.
#[async_trait]
pub trait IotThingsGraph {
    /// <p>Associates a device with a concrete thing that is in the user's registry.</p> <p>A thing can be associated with only one device at a time. If you associate a thing with a new device id, its previous association will be removed.</p>
    async fn associate_entity_to_thing(
        &self,
        input: AssociateEntityToThingRequest,
    ) -> Result<AssociateEntityToThingResponse, RusotoError<AssociateEntityToThingError>>;

    /// <p>Creates a workflow template. Workflows can be created only in the user's namespace. (The public namespace contains only entities.) The workflow can contain only entities in the specified namespace. The workflow is validated against the entities in the latest version of the user's namespace unless another namespace version is specified in the request.</p>
    async fn create_flow_template(
        &self,
        input: CreateFlowTemplateRequest,
    ) -> Result<CreateFlowTemplateResponse, RusotoError<CreateFlowTemplateError>>;

    /// <p>Creates a system instance. </p> <p>This action validates the system instance, prepares the deployment-related resources. For Greengrass deployments, it updates the Greengrass group that is specified by the <code>greengrassGroupName</code> parameter. It also adds a file to the S3 bucket specified by the <code>s3BucketName</code> parameter. You need to call <code>DeploySystemInstance</code> after running this action.</p> <p>For Greengrass deployments, since this action modifies and adds resources to a Greengrass group and an S3 bucket on the caller's behalf, the calling identity must have write permissions to both the specified Greengrass group and S3 bucket. Otherwise, the call will fail with an authorization error.</p> <p>For cloud deployments, this action requires a <code>flowActionsRoleArn</code> value. This is an IAM role that has permissions to access AWS services, such as AWS Lambda and AWS IoT, that the flow uses when it executes.</p> <p>If the definition document doesn't specify a version of the user's namespace, the latest version will be used by default.</p>
    async fn create_system_instance(
        &self,
        input: CreateSystemInstanceRequest,
    ) -> Result<CreateSystemInstanceResponse, RusotoError<CreateSystemInstanceError>>;

    /// <p>Creates a system. The system is validated against the entities in the latest version of the user's namespace unless another namespace version is specified in the request.</p>
    async fn create_system_template(
        &self,
        input: CreateSystemTemplateRequest,
    ) -> Result<CreateSystemTemplateResponse, RusotoError<CreateSystemTemplateError>>;

    /// <p>Deletes a workflow. Any new system or deployment that contains this workflow will fail to update or deploy. Existing deployments that contain the workflow will continue to run (since they use a snapshot of the workflow taken at the time of deployment).</p>
    async fn delete_flow_template(
        &self,
        input: DeleteFlowTemplateRequest,
    ) -> Result<DeleteFlowTemplateResponse, RusotoError<DeleteFlowTemplateError>>;

    /// <p>Deletes the specified namespace. This action deletes all of the entities in the namespace. Delete the systems and flows that use entities in the namespace before performing this action.</p>
    async fn delete_namespace(
        &self,
    ) -> Result<DeleteNamespaceResponse, RusotoError<DeleteNamespaceError>>;

    /// <p>Deletes a system instance. Only system instances that have never been deployed, or that have been undeployed can be deleted.</p> <p>Users can create a new system instance that has the same ID as a deleted system instance.</p>
    async fn delete_system_instance(
        &self,
        input: DeleteSystemInstanceRequest,
    ) -> Result<DeleteSystemInstanceResponse, RusotoError<DeleteSystemInstanceError>>;

    /// <p>Deletes a system. New deployments can't contain the system after its deletion. Existing deployments that contain the system will continue to work because they use a snapshot of the system that is taken when it is deployed.</p>
    async fn delete_system_template(
        &self,
        input: DeleteSystemTemplateRequest,
    ) -> Result<DeleteSystemTemplateResponse, RusotoError<DeleteSystemTemplateError>>;

    /// <p> <b>Greengrass and Cloud Deployments</b> </p> <p>Deploys the system instance to the target specified in <code>CreateSystemInstance</code>. </p> <p> <b>Greengrass Deployments</b> </p> <p>If the system or any workflows and entities have been updated before this action is called, then the deployment will create a new Amazon Simple Storage Service resource file and then deploy it.</p> <p>Since this action creates a Greengrass deployment on the caller's behalf, the calling identity must have write permissions to the specified Greengrass group. Otherwise, the call will fail with an authorization error.</p> <p>For information about the artifacts that get added to your Greengrass core device when you use this API, see <a href="https://docs.aws.amazon.com/thingsgraph/latest/ug/iot-tg-greengrass.html">AWS IoT Things Graph and AWS IoT Greengrass</a>.</p>
    async fn deploy_system_instance(
        &self,
        input: DeploySystemInstanceRequest,
    ) -> Result<DeploySystemInstanceResponse, RusotoError<DeploySystemInstanceError>>;

    /// <p>Deprecates the specified workflow. This action marks the workflow for deletion. Deprecated flows can't be deployed, but existing deployments will continue to run.</p>
    async fn deprecate_flow_template(
        &self,
        input: DeprecateFlowTemplateRequest,
    ) -> Result<DeprecateFlowTemplateResponse, RusotoError<DeprecateFlowTemplateError>>;

    /// <p>Deprecates the specified system.</p>
    async fn deprecate_system_template(
        &self,
        input: DeprecateSystemTemplateRequest,
    ) -> Result<DeprecateSystemTemplateResponse, RusotoError<DeprecateSystemTemplateError>>;

    /// <p>Gets the latest version of the user's namespace and the public version that it is tracking.</p>
    async fn describe_namespace(
        &self,
        input: DescribeNamespaceRequest,
    ) -> Result<DescribeNamespaceResponse, RusotoError<DescribeNamespaceError>>;

    /// <p>Dissociates a device entity from a concrete thing. The action takes only the type of the entity that you need to dissociate because only one entity of a particular type can be associated with a thing.</p>
    async fn dissociate_entity_from_thing(
        &self,
        input: DissociateEntityFromThingRequest,
    ) -> Result<DissociateEntityFromThingResponse, RusotoError<DissociateEntityFromThingError>>;

    /// <p>Gets definitions of the specified entities. Uses the latest version of the user's namespace by default. This API returns the following TDM entities.</p> <ul> <li> <p>Properties</p> </li> <li> <p>States</p> </li> <li> <p>Events</p> </li> <li> <p>Actions</p> </li> <li> <p>Capabilities</p> </li> <li> <p>Mappings</p> </li> <li> <p>Devices</p> </li> <li> <p>Device Models</p> </li> <li> <p>Services</p> </li> </ul> <p>This action doesn't return definitions for systems, flows, and deployments.</p>
    async fn get_entities(
        &self,
        input: GetEntitiesRequest,
    ) -> Result<GetEntitiesResponse, RusotoError<GetEntitiesError>>;

    /// <p>Gets the latest version of the <code>DefinitionDocument</code> and <code>FlowTemplateSummary</code> for the specified workflow.</p>
    async fn get_flow_template(
        &self,
        input: GetFlowTemplateRequest,
    ) -> Result<GetFlowTemplateResponse, RusotoError<GetFlowTemplateError>>;

    /// <p>Gets revisions of the specified workflow. Only the last 100 revisions are stored. If the workflow has been deprecated, this action will return revisions that occurred before the deprecation. This action won't work for workflows that have been deleted.</p>
    async fn get_flow_template_revisions(
        &self,
        input: GetFlowTemplateRevisionsRequest,
    ) -> Result<GetFlowTemplateRevisionsResponse, RusotoError<GetFlowTemplateRevisionsError>>;

    /// <p>Gets the status of a namespace deletion task.</p>
    async fn get_namespace_deletion_status(
        &self,
    ) -> Result<GetNamespaceDeletionStatusResponse, RusotoError<GetNamespaceDeletionStatusError>>;

    /// <p>Gets a system instance.</p>
    async fn get_system_instance(
        &self,
        input: GetSystemInstanceRequest,
    ) -> Result<GetSystemInstanceResponse, RusotoError<GetSystemInstanceError>>;

    /// <p>Gets a system.</p>
    async fn get_system_template(
        &self,
        input: GetSystemTemplateRequest,
    ) -> Result<GetSystemTemplateResponse, RusotoError<GetSystemTemplateError>>;

    /// <p>Gets revisions made to the specified system template. Only the previous 100 revisions are stored. If the system has been deprecated, this action will return the revisions that occurred before its deprecation. This action won't work with systems that have been deleted.</p>
    async fn get_system_template_revisions(
        &self,
        input: GetSystemTemplateRevisionsRequest,
    ) -> Result<GetSystemTemplateRevisionsResponse, RusotoError<GetSystemTemplateRevisionsError>>;

    /// <p>Gets the status of the specified upload.</p>
    async fn get_upload_status(
        &self,
        input: GetUploadStatusRequest,
    ) -> Result<GetUploadStatusResponse, RusotoError<GetUploadStatusError>>;

    /// <p>Returns a list of objects that contain information about events in a flow execution.</p>
    async fn list_flow_execution_messages(
        &self,
        input: ListFlowExecutionMessagesRequest,
    ) -> Result<ListFlowExecutionMessagesResponse, RusotoError<ListFlowExecutionMessagesError>>;

    /// <p>Lists all tags on an AWS IoT Things Graph resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Searches for entities of the specified type. You can search for entities in your namespace and the public namespace that you're tracking.</p>
    async fn search_entities(
        &self,
        input: SearchEntitiesRequest,
    ) -> Result<SearchEntitiesResponse, RusotoError<SearchEntitiesError>>;

    /// <p>Searches for AWS IoT Things Graph workflow execution instances.</p>
    async fn search_flow_executions(
        &self,
        input: SearchFlowExecutionsRequest,
    ) -> Result<SearchFlowExecutionsResponse, RusotoError<SearchFlowExecutionsError>>;

    /// <p>Searches for summary information about workflows.</p>
    async fn search_flow_templates(
        &self,
        input: SearchFlowTemplatesRequest,
    ) -> Result<SearchFlowTemplatesResponse, RusotoError<SearchFlowTemplatesError>>;

    /// <p>Searches for system instances in the user's account.</p>
    async fn search_system_instances(
        &self,
        input: SearchSystemInstancesRequest,
    ) -> Result<SearchSystemInstancesResponse, RusotoError<SearchSystemInstancesError>>;

    /// <p>Searches for summary information about systems in the user's account. You can filter by the ID of a workflow to return only systems that use the specified workflow.</p>
    async fn search_system_templates(
        &self,
        input: SearchSystemTemplatesRequest,
    ) -> Result<SearchSystemTemplatesResponse, RusotoError<SearchSystemTemplatesError>>;

    /// <p>Searches for things associated with the specified entity. You can search by both device and device model.</p> <p>For example, if two different devices, camera1 and camera2, implement the camera device model, the user can associate thing1 to camera1 and thing2 to camera2. <code>SearchThings(camera2)</code> will return only thing2, but <code>SearchThings(camera)</code> will return both thing1 and thing2.</p> <p>This action searches for exact matches and doesn't perform partial text matching.</p>
    async fn search_things(
        &self,
        input: SearchThingsRequest,
    ) -> Result<SearchThingsResponse, RusotoError<SearchThingsError>>;

    /// <p>Creates a tag for the specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes a system instance from its target (Cloud or Greengrass).</p>
    async fn undeploy_system_instance(
        &self,
        input: UndeploySystemInstanceRequest,
    ) -> Result<UndeploySystemInstanceResponse, RusotoError<UndeploySystemInstanceError>>;

    /// <p>Removes a tag from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the specified workflow. All deployed systems and system instances that use the workflow will see the changes in the flow when it is redeployed. If you don't want this behavior, copy the workflow (creating a new workflow with a different ID), and update the copy. The workflow can contain only entities in the specified namespace. </p>
    async fn update_flow_template(
        &self,
        input: UpdateFlowTemplateRequest,
    ) -> Result<UpdateFlowTemplateResponse, RusotoError<UpdateFlowTemplateError>>;

    /// <p>Updates the specified system. You don't need to run this action after updating a workflow. Any deployment that uses the system will see the changes in the system when it is redeployed.</p>
    async fn update_system_template(
        &self,
        input: UpdateSystemTemplateRequest,
    ) -> Result<UpdateSystemTemplateResponse, RusotoError<UpdateSystemTemplateError>>;

    /// <p>Asynchronously uploads one or more entity definitions to the user's namespace. The <code>document</code> parameter is required if <code>syncWithPublicNamespace</code> and <code>deleteExistingEntites</code> are false. If the <code>syncWithPublicNamespace</code> parameter is set to <code>true</code>, the user's namespace will synchronize with the latest version of the public namespace. If <code>deprecateExistingEntities</code> is set to true, all entities in the latest version will be deleted before the new <code>DefinitionDocument</code> is uploaded.</p> <p>When a user uploads entity definitions for the first time, the service creates a new namespace for the user. The new namespace tracks the public namespace. Currently users can have only one namespace. The namespace version increments whenever a user uploads entity definitions that are backwards-incompatible and whenever a user sets the <code>syncWithPublicNamespace</code> parameter or the <code>deprecateExistingEntities</code> parameter to <code>true</code>.</p> <p>The IDs for all of the entities should be in URN format. Each entity must be in the user's namespace. Users can't create entities in the public namespace, but entity definitions can refer to entities in the public namespace.</p> <p>Valid entities are <code>Device</code>, <code>DeviceModel</code>, <code>Service</code>, <code>Capability</code>, <code>State</code>, <code>Action</code>, <code>Event</code>, <code>Property</code>, <code>Mapping</code>, <code>Enum</code>. </p>
    async fn upload_entity_definitions(
        &self,
        input: UploadEntityDefinitionsRequest,
    ) -> Result<UploadEntityDefinitionsResponse, RusotoError<UploadEntityDefinitionsError>>;
}
/// A client for the AWS IoT Things Graph API.
#[derive(Clone)]
pub struct IotThingsGraphClient {
    client: Client,
    region: region::Region,
}

impl IotThingsGraphClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> IotThingsGraphClient {
        IotThingsGraphClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> IotThingsGraphClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        IotThingsGraphClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> IotThingsGraphClient {
        IotThingsGraphClient { client, region }
    }
}

#[async_trait]
impl IotThingsGraph for IotThingsGraphClient {
    /// <p>Associates a device with a concrete thing that is in the user's registry.</p> <p>A thing can be associated with only one device at a time. If you associate a thing with a new device id, its previous association will be removed.</p>
    async fn associate_entity_to_thing(
        &self,
        input: AssociateEntityToThingRequest,
    ) -> Result<AssociateEntityToThingResponse, RusotoError<AssociateEntityToThingError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.AssociateEntityToThing",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateEntityToThingError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateEntityToThingResponse, _>()
    }

    /// <p>Creates a workflow template. Workflows can be created only in the user's namespace. (The public namespace contains only entities.) The workflow can contain only entities in the specified namespace. The workflow is validated against the entities in the latest version of the user's namespace unless another namespace version is specified in the request.</p>
    async fn create_flow_template(
        &self,
        input: CreateFlowTemplateRequest,
    ) -> Result<CreateFlowTemplateResponse, RusotoError<CreateFlowTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.CreateFlowTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateFlowTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateFlowTemplateResponse, _>()
    }

    /// <p>Creates a system instance. </p> <p>This action validates the system instance, prepares the deployment-related resources. For Greengrass deployments, it updates the Greengrass group that is specified by the <code>greengrassGroupName</code> parameter. It also adds a file to the S3 bucket specified by the <code>s3BucketName</code> parameter. You need to call <code>DeploySystemInstance</code> after running this action.</p> <p>For Greengrass deployments, since this action modifies and adds resources to a Greengrass group and an S3 bucket on the caller's behalf, the calling identity must have write permissions to both the specified Greengrass group and S3 bucket. Otherwise, the call will fail with an authorization error.</p> <p>For cloud deployments, this action requires a <code>flowActionsRoleArn</code> value. This is an IAM role that has permissions to access AWS services, such as AWS Lambda and AWS IoT, that the flow uses when it executes.</p> <p>If the definition document doesn't specify a version of the user's namespace, the latest version will be used by default.</p>
    async fn create_system_instance(
        &self,
        input: CreateSystemInstanceRequest,
    ) -> Result<CreateSystemInstanceResponse, RusotoError<CreateSystemInstanceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.CreateSystemInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateSystemInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateSystemInstanceResponse, _>()
    }

    /// <p>Creates a system. The system is validated against the entities in the latest version of the user's namespace unless another namespace version is specified in the request.</p>
    async fn create_system_template(
        &self,
        input: CreateSystemTemplateRequest,
    ) -> Result<CreateSystemTemplateResponse, RusotoError<CreateSystemTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.CreateSystemTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateSystemTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateSystemTemplateResponse, _>()
    }

    /// <p>Deletes a workflow. Any new system or deployment that contains this workflow will fail to update or deploy. Existing deployments that contain the workflow will continue to run (since they use a snapshot of the workflow taken at the time of deployment).</p>
    async fn delete_flow_template(
        &self,
        input: DeleteFlowTemplateRequest,
    ) -> Result<DeleteFlowTemplateResponse, RusotoError<DeleteFlowTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.DeleteFlowTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteFlowTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteFlowTemplateResponse, _>()
    }

    /// <p>Deletes the specified namespace. This action deletes all of the entities in the namespace. Delete the systems and flows that use entities in the namespace before performing this action.</p>
    async fn delete_namespace(
        &self,
    ) -> Result<DeleteNamespaceResponse, RusotoError<DeleteNamespaceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.DeleteNamespace",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DeleteNamespaceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteNamespaceResponse, _>()
    }

    /// <p>Deletes a system instance. Only system instances that have never been deployed, or that have been undeployed can be deleted.</p> <p>Users can create a new system instance that has the same ID as a deleted system instance.</p>
    async fn delete_system_instance(
        &self,
        input: DeleteSystemInstanceRequest,
    ) -> Result<DeleteSystemInstanceResponse, RusotoError<DeleteSystemInstanceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.DeleteSystemInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteSystemInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteSystemInstanceResponse, _>()
    }

    /// <p>Deletes a system. New deployments can't contain the system after its deletion. Existing deployments that contain the system will continue to work because they use a snapshot of the system that is taken when it is deployed.</p>
    async fn delete_system_template(
        &self,
        input: DeleteSystemTemplateRequest,
    ) -> Result<DeleteSystemTemplateResponse, RusotoError<DeleteSystemTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.DeleteSystemTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteSystemTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteSystemTemplateResponse, _>()
    }

    /// <p> <b>Greengrass and Cloud Deployments</b> </p> <p>Deploys the system instance to the target specified in <code>CreateSystemInstance</code>. </p> <p> <b>Greengrass Deployments</b> </p> <p>If the system or any workflows and entities have been updated before this action is called, then the deployment will create a new Amazon Simple Storage Service resource file and then deploy it.</p> <p>Since this action creates a Greengrass deployment on the caller's behalf, the calling identity must have write permissions to the specified Greengrass group. Otherwise, the call will fail with an authorization error.</p> <p>For information about the artifacts that get added to your Greengrass core device when you use this API, see <a href="https://docs.aws.amazon.com/thingsgraph/latest/ug/iot-tg-greengrass.html">AWS IoT Things Graph and AWS IoT Greengrass</a>.</p>
    async fn deploy_system_instance(
        &self,
        input: DeploySystemInstanceRequest,
    ) -> Result<DeploySystemInstanceResponse, RusotoError<DeploySystemInstanceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.DeploySystemInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeploySystemInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeploySystemInstanceResponse, _>()
    }

    /// <p>Deprecates the specified workflow. This action marks the workflow for deletion. Deprecated flows can't be deployed, but existing deployments will continue to run.</p>
    async fn deprecate_flow_template(
        &self,
        input: DeprecateFlowTemplateRequest,
    ) -> Result<DeprecateFlowTemplateResponse, RusotoError<DeprecateFlowTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.DeprecateFlowTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeprecateFlowTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeprecateFlowTemplateResponse, _>()
    }

    /// <p>Deprecates the specified system.</p>
    async fn deprecate_system_template(
        &self,
        input: DeprecateSystemTemplateRequest,
    ) -> Result<DeprecateSystemTemplateResponse, RusotoError<DeprecateSystemTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.DeprecateSystemTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeprecateSystemTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeprecateSystemTemplateResponse, _>()
    }

    /// <p>Gets the latest version of the user's namespace and the public version that it is tracking.</p>
    async fn describe_namespace(
        &self,
        input: DescribeNamespaceRequest,
    ) -> Result<DescribeNamespaceResponse, RusotoError<DescribeNamespaceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.DescribeNamespace",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeNamespaceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeNamespaceResponse, _>()
    }

    /// <p>Dissociates a device entity from a concrete thing. The action takes only the type of the entity that you need to dissociate because only one entity of a particular type can be associated with a thing.</p>
    async fn dissociate_entity_from_thing(
        &self,
        input: DissociateEntityFromThingRequest,
    ) -> Result<DissociateEntityFromThingResponse, RusotoError<DissociateEntityFromThingError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.DissociateEntityFromThing",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DissociateEntityFromThingError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DissociateEntityFromThingResponse, _>()
    }

    /// <p>Gets definitions of the specified entities. Uses the latest version of the user's namespace by default. This API returns the following TDM entities.</p> <ul> <li> <p>Properties</p> </li> <li> <p>States</p> </li> <li> <p>Events</p> </li> <li> <p>Actions</p> </li> <li> <p>Capabilities</p> </li> <li> <p>Mappings</p> </li> <li> <p>Devices</p> </li> <li> <p>Device Models</p> </li> <li> <p>Services</p> </li> </ul> <p>This action doesn't return definitions for systems, flows, and deployments.</p>
    async fn get_entities(
        &self,
        input: GetEntitiesRequest,
    ) -> Result<GetEntitiesResponse, RusotoError<GetEntitiesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "IotThingsGraphFrontEndService.GetEntities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetEntitiesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetEntitiesResponse, _>()
    }

    /// <p>Gets the latest version of the <code>DefinitionDocument</code> and <code>FlowTemplateSummary</code> for the specified workflow.</p>
    async fn get_flow_template(
        &self,
        input: GetFlowTemplateRequest,
    ) -> Result<GetFlowTemplateResponse, RusotoError<GetFlowTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.GetFlowTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetFlowTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetFlowTemplateResponse, _>()
    }

    /// <p>Gets revisions of the specified workflow. Only the last 100 revisions are stored. If the workflow has been deprecated, this action will return revisions that occurred before the deprecation. This action won't work for workflows that have been deleted.</p>
    async fn get_flow_template_revisions(
        &self,
        input: GetFlowTemplateRevisionsRequest,
    ) -> Result<GetFlowTemplateRevisionsResponse, RusotoError<GetFlowTemplateRevisionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.GetFlowTemplateRevisions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetFlowTemplateRevisionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetFlowTemplateRevisionsResponse, _>()
    }

    /// <p>Gets the status of a namespace deletion task.</p>
    async fn get_namespace_deletion_status(
        &self,
    ) -> Result<GetNamespaceDeletionStatusResponse, RusotoError<GetNamespaceDeletionStatusError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.GetNamespaceDeletionStatus",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetNamespaceDeletionStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetNamespaceDeletionStatusResponse, _>()
    }

    /// <p>Gets a system instance.</p>
    async fn get_system_instance(
        &self,
        input: GetSystemInstanceRequest,
    ) -> Result<GetSystemInstanceResponse, RusotoError<GetSystemInstanceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.GetSystemInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSystemInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetSystemInstanceResponse, _>()
    }

    /// <p>Gets a system.</p>
    async fn get_system_template(
        &self,
        input: GetSystemTemplateRequest,
    ) -> Result<GetSystemTemplateResponse, RusotoError<GetSystemTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.GetSystemTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSystemTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetSystemTemplateResponse, _>()
    }

    /// <p>Gets revisions made to the specified system template. Only the previous 100 revisions are stored. If the system has been deprecated, this action will return the revisions that occurred before its deprecation. This action won't work with systems that have been deleted.</p>
    async fn get_system_template_revisions(
        &self,
        input: GetSystemTemplateRevisionsRequest,
    ) -> Result<GetSystemTemplateRevisionsResponse, RusotoError<GetSystemTemplateRevisionsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.GetSystemTemplateRevisions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSystemTemplateRevisionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetSystemTemplateRevisionsResponse, _>()
    }

    /// <p>Gets the status of the specified upload.</p>
    async fn get_upload_status(
        &self,
        input: GetUploadStatusRequest,
    ) -> Result<GetUploadStatusResponse, RusotoError<GetUploadStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.GetUploadStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetUploadStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetUploadStatusResponse, _>()
    }

    /// <p>Returns a list of objects that contain information about events in a flow execution.</p>
    async fn list_flow_execution_messages(
        &self,
        input: ListFlowExecutionMessagesRequest,
    ) -> Result<ListFlowExecutionMessagesResponse, RusotoError<ListFlowExecutionMessagesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.ListFlowExecutionMessages",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListFlowExecutionMessagesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListFlowExecutionMessagesResponse, _>()
    }

    /// <p>Lists all tags on an AWS IoT Things Graph resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Searches for entities of the specified type. You can search for entities in your namespace and the public namespace that you're tracking.</p>
    async fn search_entities(
        &self,
        input: SearchEntitiesRequest,
    ) -> Result<SearchEntitiesResponse, RusotoError<SearchEntitiesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.SearchEntities",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchEntitiesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<SearchEntitiesResponse, _>()
    }

    /// <p>Searches for AWS IoT Things Graph workflow execution instances.</p>
    async fn search_flow_executions(
        &self,
        input: SearchFlowExecutionsRequest,
    ) -> Result<SearchFlowExecutionsResponse, RusotoError<SearchFlowExecutionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.SearchFlowExecutions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchFlowExecutionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SearchFlowExecutionsResponse, _>()
    }

    /// <p>Searches for summary information about workflows.</p>
    async fn search_flow_templates(
        &self,
        input: SearchFlowTemplatesRequest,
    ) -> Result<SearchFlowTemplatesResponse, RusotoError<SearchFlowTemplatesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.SearchFlowTemplates",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchFlowTemplatesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<SearchFlowTemplatesResponse, _>()
    }

    /// <p>Searches for system instances in the user's account.</p>
    async fn search_system_instances(
        &self,
        input: SearchSystemInstancesRequest,
    ) -> Result<SearchSystemInstancesResponse, RusotoError<SearchSystemInstancesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.SearchSystemInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchSystemInstancesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SearchSystemInstancesResponse, _>()
    }

    /// <p>Searches for summary information about systems in the user's account. You can filter by the ID of a workflow to return only systems that use the specified workflow.</p>
    async fn search_system_templates(
        &self,
        input: SearchSystemTemplatesRequest,
    ) -> Result<SearchSystemTemplatesResponse, RusotoError<SearchSystemTemplatesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.SearchSystemTemplates",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchSystemTemplatesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SearchSystemTemplatesResponse, _>()
    }

    /// <p>Searches for things associated with the specified entity. You can search by both device and device model.</p> <p>For example, if two different devices, camera1 and camera2, implement the camera device model, the user can associate thing1 to camera1 and thing2 to camera2. <code>SearchThings(camera2)</code> will return only thing2, but <code>SearchThings(camera)</code> will return both thing1 and thing2.</p> <p>This action searches for exact matches and doesn't perform partial text matching.</p>
    async fn search_things(
        &self,
        input: SearchThingsRequest,
    ) -> Result<SearchThingsResponse, RusotoError<SearchThingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "IotThingsGraphFrontEndService.SearchThings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchThingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<SearchThingsResponse, _>()
    }

    /// <p>Creates a tag for the specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "IotThingsGraphFrontEndService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes a system instance from its target (Cloud or Greengrass).</p>
    async fn undeploy_system_instance(
        &self,
        input: UndeploySystemInstanceRequest,
    ) -> Result<UndeploySystemInstanceResponse, RusotoError<UndeploySystemInstanceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.UndeploySystemInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UndeploySystemInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UndeploySystemInstanceResponse, _>()
    }

    /// <p>Removes a tag from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.UntagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Updates the specified workflow. All deployed systems and system instances that use the workflow will see the changes in the flow when it is redeployed. If you don't want this behavior, copy the workflow (creating a new workflow with a different ID), and update the copy. The workflow can contain only entities in the specified namespace. </p>
    async fn update_flow_template(
        &self,
        input: UpdateFlowTemplateRequest,
    ) -> Result<UpdateFlowTemplateResponse, RusotoError<UpdateFlowTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.UpdateFlowTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateFlowTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateFlowTemplateResponse, _>()
    }

    /// <p>Updates the specified system. You don't need to run this action after updating a workflow. Any deployment that uses the system will see the changes in the system when it is redeployed.</p>
    async fn update_system_template(
        &self,
        input: UpdateSystemTemplateRequest,
    ) -> Result<UpdateSystemTemplateResponse, RusotoError<UpdateSystemTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.UpdateSystemTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateSystemTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateSystemTemplateResponse, _>()
    }

    /// <p>Asynchronously uploads one or more entity definitions to the user's namespace. The <code>document</code> parameter is required if <code>syncWithPublicNamespace</code> and <code>deleteExistingEntites</code> are false. If the <code>syncWithPublicNamespace</code> parameter is set to <code>true</code>, the user's namespace will synchronize with the latest version of the public namespace. If <code>deprecateExistingEntities</code> is set to true, all entities in the latest version will be deleted before the new <code>DefinitionDocument</code> is uploaded.</p> <p>When a user uploads entity definitions for the first time, the service creates a new namespace for the user. The new namespace tracks the public namespace. Currently users can have only one namespace. The namespace version increments whenever a user uploads entity definitions that are backwards-incompatible and whenever a user sets the <code>syncWithPublicNamespace</code> parameter or the <code>deprecateExistingEntities</code> parameter to <code>true</code>.</p> <p>The IDs for all of the entities should be in URN format. Each entity must be in the user's namespace. Users can't create entities in the public namespace, but entity definitions can refer to entities in the public namespace.</p> <p>Valid entities are <code>Device</code>, <code>DeviceModel</code>, <code>Service</code>, <code>Capability</code>, <code>State</code>, <code>Action</code>, <code>Event</code>, <code>Property</code>, <code>Mapping</code>, <code>Enum</code>. </p>
    async fn upload_entity_definitions(
        &self,
        input: UploadEntityDefinitionsRequest,
    ) -> Result<UploadEntityDefinitionsResponse, RusotoError<UploadEntityDefinitionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "IotThingsGraphFrontEndService.UploadEntityDefinitions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UploadEntityDefinitionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UploadEntityDefinitionsResponse, _>()
    }
}
