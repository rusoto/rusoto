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

use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use futures::prelude::*;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::pin::Pin;
/// <p>Contains details about the analyzed resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnalyzedResource {
    /// <p>The actions that an external principal is granted permission to use by the policy that generated the finding.</p>
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// <p>The time at which the resource was analyzed.</p>
    #[serde(rename = "analyzedAt")]
    pub analyzed_at: f64,
    /// <p>The time at which the finding was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>An error message.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>Indicates whether the policy that generated the finding grants public access to the resource.</p>
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    /// <p>The ARN of the resource that was analyzed.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The type of the resource that was analyzed.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// <p>Indicates how the access that generated the finding is granted.</p>
    #[serde(rename = "sharedVia")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_via: Option<Vec<String>>,
    /// <p>The current status of the finding generated from the analyzed resource.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The time at which the finding was updated.</p>
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}

/// <p>Contains the ARN of the analyzed resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnalyzedResourceSummary {
    /// <p>The ARN of the analyzed resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The type of resource that was analyzed.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>Contains information about the analyzer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnalyzerSummary {
    /// <p>The ARN of the analyzer.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>A timestamp for the time at which the analyzer was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The resource that was most recently analyzed by the analyzer.</p>
    #[serde(rename = "lastResourceAnalyzed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resource_analyzed: Option<String>,
    /// <p>The time at which the most recently analyzed resource was analyzed.</p>
    #[serde(rename = "lastResourceAnalyzedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resource_analyzed_at: Option<f64>,
    /// <p>The name of the analyzer.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The tags added to the analyzer.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of analyzer, which corresponds to the zone of trust chosen for the analyzer.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Contains information about an archive rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ArchiveRuleSummary {
    /// <p>The time at which the archive rule was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>A filter used to define the archive rule.</p>
    #[serde(rename = "filter")]
    pub filter: ::std::collections::HashMap<String, Criterion>,
    /// <p>The name of the archive rule.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    /// <p>The time at which the archive rule was last updated.</p>
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}

/// <p>Creates an analyzer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAnalyzerRequest {
    /// <p>The name of the analyzer to create.</p>
    #[serde(rename = "analyzerName")]
    pub analyzer_name: String,
    /// <p>Specifies the archive rules to add for the analyzer. Archive rules automatically archive findings that meet the criteria you define for the rule.</p>
    #[serde(rename = "archiveRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_rules: Option<Vec<InlineArchiveRule>>,
    /// <p>A client token.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The tags to apply to the analyzer.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of analyzer to create. Only ACCOUNT analyzers are supported. You can create only one analyzer per account per Region.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>The response to the request to create an analyzer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAnalyzerResponse {
    /// <p>The ARN of the analyzer that was created by the request.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>Creates an archive rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateArchiveRuleRequest {
    /// <p>The name of the created analyzer.</p>
    #[serde(rename = "analyzerName")]
    pub analyzer_name: String,
    /// <p>A client token.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The criteria for the rule.</p>
    #[serde(rename = "filter")]
    pub filter: ::std::collections::HashMap<String, Criterion>,
    /// <p>The name of the rule to create.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
}

/// <p>The criteria to use in the filter that defines the archive rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Criterion {
    /// <p>A "contains" operator to match for the filter used to create the rule.</p>
    #[serde(rename = "contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<Vec<String>>,
    /// <p>An "equals" operator to match for the filter used to create the rule.</p>
    #[serde(rename = "eq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    /// <p>An "exists" operator to match for the filter used to create the rule. </p>
    #[serde(rename = "exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    /// <p>A "not equals" operator to match for the filter used to create the rule.</p>
    #[serde(rename = "neq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neq: Option<Vec<String>>,
}

/// <p>Deletes an analyzer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAnalyzerRequest {
    /// <p>The name of the analyzer to delete.</p>
    #[serde(rename = "analyzerName")]
    pub analyzer_name: String,
    /// <p>A client token.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

/// <p>Deletes an archive rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteArchiveRuleRequest {
    /// <p>The name of the analyzer that associated with the archive rule to delete.</p>
    #[serde(rename = "analyzerName")]
    pub analyzer_name: String,
    /// <p>A client token.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the rule to delete.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
}

/// <p>Contains information about a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Finding {
    /// <p>The action in the analyzed policy statement that an external principal has permission to use.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<String>>,
    /// <p>The time at which the resource was analyzed.</p>
    #[serde(rename = "analyzedAt")]
    pub analyzed_at: f64,
    /// <p>The condition in the analyzed policy statement that resulted in a finding.</p>
    #[serde(rename = "condition")]
    pub condition: ::std::collections::HashMap<String, String>,
    /// <p>The time at which the finding was generated.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>An error.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The ID of the finding.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>Indicates whether the policy that generated the finding allows public access to the resource.</p>
    #[serde(rename = "isPublic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// <p>The external principal that access to a resource within the zone of trust.</p>
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<::std::collections::HashMap<String, String>>,
    /// <p>The resource that an external principal has access to.</p>
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// <p>The type of the resource reported in the finding.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// <p>The current status of the finding.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>The time at which the finding was updated.</p>
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}

/// <p>Contains information about a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FindingSummary {
    /// <p>The action in the analyzed policy statement that an external principal has permission to use.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<String>>,
    /// <p>The time at which the resource-based policy that generated the finding was analyzed.</p>
    #[serde(rename = "analyzedAt")]
    pub analyzed_at: f64,
    /// <p>The condition in the analyzed policy statement that resulted in a finding.</p>
    #[serde(rename = "condition")]
    pub condition: ::std::collections::HashMap<String, String>,
    /// <p>The time at which the finding was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The error that resulted in an Error finding.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The ID of the finding.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>Indicates whether the finding reports a resource that has a policy that allows public access.</p>
    #[serde(rename = "isPublic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// <p>The external principal that has access to a resource within the zone of trust.</p>
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<::std::collections::HashMap<String, String>>,
    /// <p>The resource that the external principal has access to.</p>
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// <p>The type of the resource that the external principal has access to.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// <p>The status of the finding.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>The time at which the finding was most recently updated.</p>
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}

/// <p>Retrieves an analyzed resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAnalyzedResourceRequest {
    /// <p>The ARN of the analyzer to retrieve information from.</p>
    #[serde(rename = "analyzerArn")]
    pub analyzer_arn: String,
    /// <p>The ARN of the resource to retrieve information about.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAnalyzedResourceResponse {
    /// <p>An <code>AnalyedResource</code> object that contains information that Access Analyzer found when it analyzed the resource.</p>
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<AnalyzedResource>,
}

/// <p>Retrieves an analyzer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAnalyzerRequest {
    /// <p>The name of the analyzer retrieved.</p>
    #[serde(rename = "analyzerName")]
    pub analyzer_name: String,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAnalyzerResponse {
    /// <p>An <code>AnalyzerSummary</code> object that contains information about the analyzer.</p>
    #[serde(rename = "analyzer")]
    pub analyzer: AnalyzerSummary,
}

/// <p>Retrieves an archive rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetArchiveRuleRequest {
    /// <p>The name of the analyzer to retrieve rules from.</p>
    #[serde(rename = "analyzerName")]
    pub analyzer_name: String,
    /// <p>The name of the rule to retrieve.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetArchiveRuleResponse {
    #[serde(rename = "archiveRule")]
    pub archive_rule: ArchiveRuleSummary,
}

/// <p>Retrieves a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFindingRequest {
    /// <p>The ARN of the analyzer that generated the finding.</p>
    #[serde(rename = "analyzerArn")]
    pub analyzer_arn: String,
    /// <p>The ID of the finding to retrieve.</p>
    #[serde(rename = "id")]
    pub id: String,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFindingResponse {
    /// <p>A <code>finding</code> object that contains finding details.</p>
    #[serde(rename = "finding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<Finding>,
}

/// <p>An criterion statement in an archive rule. Each archive rule may have multiple criteria.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InlineArchiveRule {
    /// <p>The condition and values for a criterion.</p>
    #[serde(rename = "filter")]
    pub filter: ::std::collections::HashMap<String, Criterion>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
}

/// <p>Retrieves a list of resources that have been analyzed.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAnalyzedResourcesRequest {
    /// <p>The ARN of the analyzer to retrieve a list of analyzed resources from.</p>
    #[serde(rename = "analyzerArn")]
    pub analyzer_arn: String,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used for pagination of results returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAnalyzedResourcesResponse {
    /// <p>A list of resources that were analyzed.</p>
    #[serde(rename = "analyzedResources")]
    pub analyzed_resources: Vec<AnalyzedResourceSummary>,
    /// <p>A token used for pagination of results returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Retrieves a list of analyzers.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAnalyzersRequest {
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used for pagination of results returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of analyzer.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAnalyzersResponse {
    /// <p>The analyzers retrieved.</p>
    #[serde(rename = "analyzers")]
    pub analyzers: Vec<AnalyzerSummary>,
    /// <p>A token used for pagination of results returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Retrieves a list of archive rules created for the specified analyzer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListArchiveRulesRequest {
    /// <p>The name of the analyzer to retrieve rules from.</p>
    #[serde(rename = "analyzerName")]
    pub analyzer_name: String,
    /// <p>The maximum number of results to return in the request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used for pagination of results returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListArchiveRulesResponse {
    /// <p>A list of archive rules created for the specified analyzer.</p>
    #[serde(rename = "archiveRules")]
    pub archive_rules: Vec<ArchiveRuleSummary>,
    /// <p>A token used for pagination of results returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Retrieves a list of findings generated by the specified analyzer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFindingsRequest {
    /// <p>The ARN of the analyzer to retrieve findings from.</p>
    #[serde(rename = "analyzerArn")]
    pub analyzer_arn: String,
    /// <p>A filter to match for the findings to return.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<::std::collections::HashMap<String, Criterion>>,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used for pagination of results returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sort order for the findings returned.</p>
    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortCriteria>,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFindingsResponse {
    /// <p>A list of findings retrieved from the analyzer that match the filter criteria specified, if any.</p>
    #[serde(rename = "findings")]
    pub findings: Vec<FindingSummary>,
    /// <p>A token used for pagination of results returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Retrieves a list of tags applied to the specified resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the resource to retrieve tags from.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags that are applied to the specified resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The criteria used to sort.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SortCriteria {
    /// <p>The name of the attribute to sort on.</p>
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The sort order, ascending or descending.</p>
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

/// <p>Starts a scan of the policies applied to the specified resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartResourceScanRequest {
    /// <p>The ARN of the analyzer to use to scan the policies applied to the specified resource.</p>
    #[serde(rename = "analyzerArn")]
    pub analyzer_arn: String,
    /// <p>The ARN of the resource to scan.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p>Adds a tag to the specified resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource to add the tag to.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Removes a tag from the specified resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource to remove the tag from.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The key for the tag to add.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>The response to the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>Updates the specified archive rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateArchiveRuleRequest {
    /// <p>The name of the analyzer to update the archive rules for.</p>
    #[serde(rename = "analyzerName")]
    pub analyzer_name: String,
    /// <p>A client token.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>A filter to match for the rules to update. Only rules that match the filter are updated.</p>
    #[serde(rename = "filter")]
    pub filter: ::std::collections::HashMap<String, Criterion>,
    /// <p>The name of the rule to update.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
}

/// <p>Updates findings with the new values provided in the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFindingsRequest {
    /// <p>The ARN of the analyzer that generated the findings to update.</p>
    #[serde(rename = "analyzerArn")]
    pub analyzer_arn: String,
    /// <p>A client token.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The IDs of the findings to update.</p>
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>The ARN of the resource identified in the finding.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The state represents the action to take to update the finding Status. Use <code>ARCHIVE</code> to change an Active finding to an Archived finding. Use <code>ACTIVE</code> to change an Archived finding to an Active finding.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Contains information about a validation exception.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ValidationExceptionField {
    /// <p>A message about the validation exception.</p>
    pub message: String,
    /// <p>The name of the validation exception.</p>
    pub name: String,
}

/// Errors returned by CreateAnalyzer
#[derive(Debug, PartialEq)]
pub enum CreateAnalyzerError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>A conflict exception error.</p>
    Conflict(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>Service quote met error.</p>
    ServiceQuotaExceeded(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl CreateAnalyzerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAnalyzerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateAnalyzerError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateAnalyzerError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateAnalyzerError::InternalServer(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateAnalyzerError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateAnalyzerError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAnalyzerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAnalyzerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateAnalyzerError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateAnalyzerError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateAnalyzerError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateAnalyzerError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAnalyzerError {}
/// Errors returned by CreateArchiveRule
#[derive(Debug, PartialEq)]
pub enum CreateArchiveRuleError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>A conflict exception error.</p>
    Conflict(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Service quote met error.</p>
    ServiceQuotaExceeded(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl CreateArchiveRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateArchiveRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateArchiveRuleError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateArchiveRuleError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateArchiveRuleError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateArchiveRuleError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateArchiveRuleError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateArchiveRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateArchiveRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateArchiveRuleError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateArchiveRuleError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateArchiveRuleError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateArchiveRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateArchiveRuleError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateArchiveRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateArchiveRuleError {}
/// Errors returned by DeleteAnalyzer
#[derive(Debug, PartialEq)]
pub enum DeleteAnalyzerError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl DeleteAnalyzerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAnalyzerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteAnalyzerError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteAnalyzerError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteAnalyzerError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteAnalyzerError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAnalyzerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAnalyzerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteAnalyzerError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteAnalyzerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAnalyzerError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAnalyzerError {}
/// Errors returned by DeleteArchiveRule
#[derive(Debug, PartialEq)]
pub enum DeleteArchiveRuleError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl DeleteArchiveRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteArchiveRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteArchiveRuleError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteArchiveRuleError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteArchiveRuleError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteArchiveRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteArchiveRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteArchiveRuleError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteArchiveRuleError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteArchiveRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteArchiveRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteArchiveRuleError {}
/// Errors returned by GetAnalyzedResource
#[derive(Debug, PartialEq)]
pub enum GetAnalyzedResourceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl GetAnalyzedResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAnalyzedResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetAnalyzedResourceError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetAnalyzedResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetAnalyzedResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetAnalyzedResourceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAnalyzedResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAnalyzedResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetAnalyzedResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetAnalyzedResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetAnalyzedResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAnalyzedResourceError {}
/// Errors returned by GetAnalyzer
#[derive(Debug, PartialEq)]
pub enum GetAnalyzerError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl GetAnalyzerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAnalyzerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetAnalyzerError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetAnalyzerError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetAnalyzerError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetAnalyzerError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAnalyzerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAnalyzerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetAnalyzerError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetAnalyzerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetAnalyzerError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAnalyzerError {}
/// Errors returned by GetArchiveRule
#[derive(Debug, PartialEq)]
pub enum GetArchiveRuleError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl GetArchiveRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetArchiveRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetArchiveRuleError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetArchiveRuleError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetArchiveRuleError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetArchiveRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetArchiveRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetArchiveRuleError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetArchiveRuleError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetArchiveRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetArchiveRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetArchiveRuleError {}
/// Errors returned by GetFinding
#[derive(Debug, PartialEq)]
pub enum GetFindingError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl GetFindingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFindingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetFindingError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetFindingError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFindingError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetFindingError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFindingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFindingError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetFindingError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetFindingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFindingError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFindingError {}
/// Errors returned by ListAnalyzedResources
#[derive(Debug, PartialEq)]
pub enum ListAnalyzedResourcesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl ListAnalyzedResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAnalyzedResourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAnalyzedResourcesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListAnalyzedResourcesError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAnalyzedResourcesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListAnalyzedResourcesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAnalyzedResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAnalyzedResourcesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListAnalyzedResourcesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListAnalyzedResourcesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListAnalyzedResourcesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAnalyzedResourcesError {}
/// Errors returned by ListAnalyzers
#[derive(Debug, PartialEq)]
pub enum ListAnalyzersError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl ListAnalyzersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAnalyzersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAnalyzersError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListAnalyzersError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListAnalyzersError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAnalyzersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAnalyzersError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListAnalyzersError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListAnalyzersError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAnalyzersError {}
/// Errors returned by ListArchiveRules
#[derive(Debug, PartialEq)]
pub enum ListArchiveRulesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl ListArchiveRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListArchiveRulesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListArchiveRulesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListArchiveRulesError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListArchiveRulesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListArchiveRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListArchiveRulesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListArchiveRulesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListArchiveRulesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListArchiveRulesError {}
/// Errors returned by ListFindings
#[derive(Debug, PartialEq)]
pub enum ListFindingsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl ListFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListFindingsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListFindingsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFindingsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListFindingsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFindingsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListFindingsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListFindingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListFindingsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFindingsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
            ListTagsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by StartResourceScan
#[derive(Debug, PartialEq)]
pub enum StartResourceScanError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl StartResourceScanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartResourceScanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartResourceScanError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(StartResourceScanError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartResourceScanError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartResourceScanError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartResourceScanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartResourceScanError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartResourceScanError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartResourceScanError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartResourceScanError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartResourceScanError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateArchiveRule
#[derive(Debug, PartialEq)]
pub enum UpdateArchiveRuleError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl UpdateArchiveRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateArchiveRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateArchiveRuleError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateArchiveRuleError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateArchiveRuleError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateArchiveRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateArchiveRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateArchiveRuleError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateArchiveRuleError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateArchiveRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateArchiveRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateArchiveRuleError {}
/// Errors returned by UpdateFindings
#[derive(Debug, PartialEq)]
pub enum UpdateFindingsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    InternalServer(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Throttling limit exceeded error.</p>
    Throttling(String),
}

impl UpdateFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateFindingsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateFindingsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateFindingsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateFindingsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFindingsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFindingsError {}
/// Trait representing the capabilities of the Access Analyzer API. Access Analyzer clients implement this trait.
pub trait AccessAnalyzer {
    /// <p>Creates an analyzer for your account.</p>
    fn create_analyzer(
        &self,
        input: CreateAnalyzerRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateAnalyzerResponse, RusotoError<CreateAnalyzerError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Creates an archive rule for the specified analyzer. Archive rules automatically archive findings that meet the criteria you define when you create the rule.</p>
    fn create_archive_rule(
        &self,
        input: CreateArchiveRuleRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<CreateArchiveRuleError>>> + Send + 'static>,
    >;

    /// <p>Deletes the specified analyzer. When you delete an analyzer, Access Analyzer is disabled for the account in the current or specific Region. All findings that were generated by the analyzer are deleted. You cannot undo this action.</p>
    fn delete_analyzer(
        &self,
        input: DeleteAnalyzerRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<DeleteAnalyzerError>>> + Send + 'static>>;

    /// <p>Deletes the specified archive rule.</p>
    fn delete_archive_rule(
        &self,
        input: DeleteArchiveRuleRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<DeleteArchiveRuleError>>> + Send + 'static>,
    >;

    /// <p>Retrieves information about a resource that was analyzed.</p>
    fn get_analyzed_resource(
        &self,
        input: GetAnalyzedResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetAnalyzedResourceResponse,
                        RusotoError<GetAnalyzedResourceError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves information about the specified analyzer.</p>
    fn get_analyzer(
        &self,
        input: GetAnalyzerRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<GetAnalyzerResponse, RusotoError<GetAnalyzerError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves information about an archive rule.</p>
    fn get_archive_rule(
        &self,
        input: GetArchiveRuleRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<GetArchiveRuleResponse, RusotoError<GetArchiveRuleError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves information about the specified finding.</p>
    fn get_finding(
        &self,
        input: GetFindingRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<GetFindingResponse, RusotoError<GetFindingError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves a list of resources of the specified type that have been analyzed by the specified analyzer..</p>
    fn list_analyzed_resources(
        &self,
        input: ListAnalyzedResourcesRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListAnalyzedResourcesResponse,
                        RusotoError<ListAnalyzedResourcesError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves a list of analyzers.</p>
    fn list_analyzers(
        &self,
        input: ListAnalyzersRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListAnalyzersResponse, RusotoError<ListAnalyzersError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves a list of archive rules created for the specified analyzer.</p>
    fn list_archive_rules(
        &self,
        input: ListArchiveRulesRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<ListArchiveRulesResponse, RusotoError<ListArchiveRulesError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves a list of findings generated by the specified analyzer.</p>
    fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListFindingsResponse, RusotoError<ListFindingsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves a list of tags applied to the specified resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListTagsForResourceResponse,
                        RusotoError<ListTagsForResourceError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Immediately starts a scan of the policies applied to the specified resource.</p>
    fn start_resource_scan(
        &self,
        input: StartResourceScanRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<StartResourceScanError>>> + Send + 'static>,
    >;

    /// <p>Adds a tag to the specified resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<TagResourceResponse, RusotoError<TagResourceError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Removes a tag from the specified resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UntagResourceResponse, RusotoError<UntagResourceError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Updates the criteria and values for the specified archive rule.</p>
    fn update_archive_rule(
        &self,
        input: UpdateArchiveRuleRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<UpdateArchiveRuleError>>> + Send + 'static>,
    >;

    /// <p>Updates the status for the specified findings.</p>
    fn update_findings(
        &self,
        input: UpdateFindingsRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<UpdateFindingsError>>> + Send + 'static>>;
}
/// A client for the Access Analyzer API.
#[derive(Clone)]
pub struct AccessAnalyzerClient {
    client: Client,
    region: region::Region,
}

impl AccessAnalyzerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AccessAnalyzerClient {
        AccessAnalyzerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AccessAnalyzerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AccessAnalyzerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AccessAnalyzerClient {
        AccessAnalyzerClient { client, region }
    }
}

impl AccessAnalyzer for AccessAnalyzerClient {
    /// <p>Creates an analyzer for your account.</p>
    fn create_analyzer(
        &self,
        input: CreateAnalyzerRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateAnalyzerResponse, RusotoError<CreateAnalyzerError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/analyzer";

        let mut request = SignedRequest::new("PUT", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateAnalyzerResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateAnalyzerError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates an archive rule for the specified analyzer. Archive rules automatically archive findings that meet the criteria you define when you create the rule.</p>
    fn create_archive_rule(
        &self,
        input: CreateArchiveRuleRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<CreateArchiveRuleError>>> + Send + 'static>,
    > {
        let request_uri = format!(
            "/analyzer/{analyzer_name}/archive-rule",
            analyzer_name = input.analyzer_name
        );

        let mut request = SignedRequest::new("PUT", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateArchiveRuleError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes the specified analyzer. When you delete an analyzer, Access Analyzer is disabled for the account in the current or specific Region. All findings that were generated by the analyzer are deleted. You cannot undo this action.</p>
    fn delete_analyzer(
        &self,
        input: DeleteAnalyzerRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<DeleteAnalyzerError>>> + Send + 'static>>
    {
        let request_uri = format!(
            "/analyzer/{analyzer_name}",
            analyzer_name = input.analyzer_name
        );

        let mut request =
            SignedRequest::new("DELETE", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.client_token {
            params.put("clientToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteAnalyzerError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes the specified archive rule.</p>
    fn delete_archive_rule(
        &self,
        input: DeleteArchiveRuleRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<DeleteArchiveRuleError>>> + Send + 'static>,
    > {
        let request_uri = format!(
            "/analyzer/{analyzer_name}/archive-rule/{rule_name}",
            analyzer_name = input.analyzer_name,
            rule_name = input.rule_name
        );

        let mut request =
            SignedRequest::new("DELETE", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.client_token {
            params.put("clientToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteArchiveRuleError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves information about a resource that was analyzed.</p>
    fn get_analyzed_resource(
        &self,
        input: GetAnalyzedResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetAnalyzedResourceResponse,
                        RusotoError<GetAnalyzedResourceError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/analyzed-resource";

        let mut request = SignedRequest::new("GET", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("analyzerArn", &input.analyzer_arn);
        params.put("resourceArn", &input.resource_arn);
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetAnalyzedResourceResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(GetAnalyzedResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves information about the specified analyzer.</p>
    fn get_analyzer(
        &self,
        input: GetAnalyzerRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<GetAnalyzerResponse, RusotoError<GetAnalyzerError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/analyzer/{analyzer_name}",
            analyzer_name = input.analyzer_name
        );

        let mut request = SignedRequest::new("GET", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetAnalyzerResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(GetAnalyzerError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves information about an archive rule.</p>
    fn get_archive_rule(
        &self,
        input: GetArchiveRuleRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<GetArchiveRuleResponse, RusotoError<GetArchiveRuleError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/analyzer/{analyzer_name}/archive-rule/{rule_name}",
            analyzer_name = input.analyzer_name,
            rule_name = input.rule_name
        );

        let mut request = SignedRequest::new("GET", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetArchiveRuleResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(GetArchiveRuleError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves information about the specified finding.</p>
    fn get_finding(
        &self,
        input: GetFindingRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<GetFindingResponse, RusotoError<GetFindingError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/finding/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("analyzerArn", &input.analyzer_arn);
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetFindingResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(GetFindingError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves a list of resources of the specified type that have been analyzed by the specified analyzer..</p>
    fn list_analyzed_resources(
        &self,
        input: ListAnalyzedResourcesRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListAnalyzedResourcesResponse,
                        RusotoError<ListAnalyzedResourcesError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/analyzed-resource";

        let mut request = SignedRequest::new("POST", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListAnalyzedResourcesResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListAnalyzedResourcesError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves a list of analyzers.</p>
    fn list_analyzers(
        &self,
        input: ListAnalyzersRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListAnalyzersResponse, RusotoError<ListAnalyzersError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/analyzer";

        let mut request = SignedRequest::new("GET", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.type_ {
            params.put("type", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListAnalyzersResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListAnalyzersError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves a list of archive rules created for the specified analyzer.</p>
    fn list_archive_rules(
        &self,
        input: ListArchiveRulesRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<ListArchiveRulesResponse, RusotoError<ListArchiveRulesError>>,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/analyzer/{analyzer_name}/archive-rule",
            analyzer_name = input.analyzer_name
        );

        let mut request = SignedRequest::new("GET", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListArchiveRulesResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListArchiveRulesError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves a list of findings generated by the specified analyzer.</p>
    fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListFindingsResponse, RusotoError<ListFindingsError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/finding";

        let mut request = SignedRequest::new("POST", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListFindingsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListFindingsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves a list of tags applied to the specified resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListTagsForResourceResponse,
                        RusotoError<ListTagsForResourceError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListTagsForResourceResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListTagsForResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Immediately starts a scan of the policies applied to the specified resource.</p>
    fn start_resource_scan(
        &self,
        input: StartResourceScanRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<StartResourceScanError>>> + Send + 'static>,
    > {
        let request_uri = "/resource/scan";

        let mut request = SignedRequest::new("POST", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(StartResourceScanError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Adds a tag to the specified resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<TagResourceResponse, RusotoError<TagResourceError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<TagResourceResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(TagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes a tag from the specified resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UntagResourceResponse, RusotoError<UntagResourceError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("DELETE", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UntagResourceResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UntagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates the criteria and values for the specified archive rule.</p>
    fn update_archive_rule(
        &self,
        input: UpdateArchiveRuleRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<UpdateArchiveRuleError>>> + Send + 'static>,
    > {
        let request_uri = format!(
            "/analyzer/{analyzer_name}/archive-rule/{rule_name}",
            analyzer_name = input.analyzer_name,
            rule_name = input.rule_name
        );

        let mut request = SignedRequest::new("PUT", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateArchiveRuleError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates the status for the specified findings.</p>
    fn update_findings(
        &self,
        input: UpdateFindingsRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<UpdateFindingsError>>> + Send + 'static>>
    {
        let request_uri = "/finding";

        let mut request = SignedRequest::new("PUT", "access-analyzer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 200 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateFindingsError::from_response(response))
            }
        }
        .boxed()
    }
}
