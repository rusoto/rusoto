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
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddFacetToObjectRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Attributes on the facet that you are adding to the object.</p>
    #[serde(rename = "ObjectAttributeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_list: Option<Vec<AttributeKeyAndValue>>,
    /// <p>A reference to the object you are adding the specified facet to.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
    /// <p>Identifiers for the facet that you are adding to the object. See <a>SchemaFacet</a> for details.</p>
    #[serde(rename = "SchemaFacet")]
    pub schema_facet: SchemaFacet,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddFacetToObjectResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ApplySchemaRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> into which the schema is copied. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Published schema Amazon Resource Name (ARN) that needs to be copied. For more information, see <a>arns</a>.</p>
    #[serde(rename = "PublishedSchemaArn")]
    pub published_schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ApplySchemaResponse {
    /// <p>The applied schema ARN that is associated with the copied schema in the <a>Directory</a>. You can use this ARN to describe the schema information applied on this directory. For more information, see <a>arns</a>.</p>
    #[serde(rename = "AppliedSchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_schema_arn: Option<String>,
    /// <p>The ARN that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachObjectRequest {
    /// <p>The child object reference to be attached to the object.</p>
    #[serde(rename = "ChildReference")]
    pub child_reference: ObjectReference,
    /// <p>Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where both objects reside. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The link name with which the child object is attached to the parent.</p>
    #[serde(rename = "LinkName")]
    pub link_name: String,
    /// <p>The parent object reference.</p>
    #[serde(rename = "ParentReference")]
    pub parent_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttachObjectResponse {
    /// <p>The attached <code>ObjectIdentifier</code>, which is the child <code>ObjectIdentifier</code>.</p>
    #[serde(rename = "AttachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where both objects reside. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The reference that identifies the object to which the policy will be attached.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
    /// <p>The reference that is associated with the policy object.</p>
    #[serde(rename = "PolicyReference")]
    pub policy_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttachPolicyResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachToIndexRequest {
    /// <p>The Amazon Resource Name (ARN) of the directory where the object and index exist.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>A reference to the index that you are attaching the object to.</p>
    #[serde(rename = "IndexReference")]
    pub index_reference: ObjectReference,
    /// <p>A reference to the object that you are attaching to the index.</p>
    #[serde(rename = "TargetReference")]
    pub target_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttachToIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the object that was attached to the index.</p>
    #[serde(rename = "AttachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachTypedLinkRequest {
    /// <p>A set of attributes that are associated with the typed link.</p>
    #[serde(rename = "Attributes")]
    pub attributes: Vec<AttributeNameAndValue>,
    /// <p>The Amazon Resource Name (ARN) of the directory where you want to attach the typed link.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Identifies the source object that the typed link will attach to.</p>
    #[serde(rename = "SourceObjectReference")]
    pub source_object_reference: ObjectReference,
    /// <p>Identifies the target object that the typed link will attach to.</p>
    #[serde(rename = "TargetObjectReference")]
    pub target_object_reference: ObjectReference,
    /// <p>Identifies the typed link facet that is associated with the typed link.</p>
    #[serde(rename = "TypedLinkFacet")]
    pub typed_link_facet: TypedLinkSchemaAndFacetName,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttachTypedLinkResponse {
    /// <p>Returns a typed link specifier as output.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_link_specifier: Option<TypedLinkSpecifier>,
}

/// <p>A unique identifier for an attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeKey {
    /// <p>The name of the facet that the attribute exists within.</p>
    #[serde(rename = "FacetName")]
    pub facet_name: String,
    /// <p>The name of the attribute.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the schema that contains the facet and attribute.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

/// <p>The combination of an attribute key and an attribute value.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeKeyAndValue {
    /// <p>The key of the attribute.</p>
    #[serde(rename = "Key")]
    pub key: AttributeKey,
    /// <p>The value of the attribute.</p>
    #[serde(rename = "Value")]
    pub value: TypedAttributeValue,
}

/// <p>Identifies the attribute name and value for a typed link.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeNameAndValue {
    /// <p>The attribute name of the typed link.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p>The value for the typed link.</p>
    #[serde(rename = "Value")]
    pub value: TypedAttributeValue,
}

/// <p>Represents the output of a batch add facet to object operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchAddFacetToObject {
    /// <p>The attributes to set on the object.</p>
    #[serde(rename = "ObjectAttributeList")]
    pub object_attribute_list: Vec<AttributeKeyAndValue>,
    /// <p>A reference to the object being mutated.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
    /// <p>Represents the facet being added to the object.</p>
    #[serde(rename = "SchemaFacet")]
    pub schema_facet: SchemaFacet,
}

/// <p>The result of a batch add facet to object operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchAddFacetToObjectResponse {}

/// <p>Represents the output of an <a>AttachObject</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchAttachObject {
    /// <p>The child object reference that is to be attached to the object.</p>
    #[serde(rename = "ChildReference")]
    pub child_reference: ObjectReference,
    /// <p>The name of the link.</p>
    #[serde(rename = "LinkName")]
    pub link_name: String,
    /// <p>The parent object reference.</p>
    #[serde(rename = "ParentReference")]
    pub parent_reference: ObjectReference,
}

/// <p>Represents the output batch <a>AttachObject</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchAttachObjectResponse {
    /// <p>The <code>ObjectIdentifier</code> of the object that has been attached.</p>
    #[serde(rename = "attachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

/// <p>Attaches a policy object to a regular object inside a <a>BatchRead</a> operation. For more information, see <a>AttachPolicy</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchAttachPolicy {
    /// <p>The reference that identifies the object to which the policy will be attached.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
    /// <p>The reference that is associated with the policy object.</p>
    #[serde(rename = "PolicyReference")]
    pub policy_reference: ObjectReference,
}

/// <p>Represents the output of an <a>AttachPolicy</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchAttachPolicyResponse {}

/// <p>Attaches the specified object to the specified index inside a <a>BatchRead</a> operation. For more information, see <a>AttachToIndex</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchAttachToIndex {
    /// <p>A reference to the index that you are attaching the object to.</p>
    #[serde(rename = "IndexReference")]
    pub index_reference: ObjectReference,
    /// <p>A reference to the object that you are attaching to the index.</p>
    #[serde(rename = "TargetReference")]
    pub target_reference: ObjectReference,
}

/// <p>Represents the output of a <a>AttachToIndex</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchAttachToIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the object that was attached to the index.</p>
    #[serde(rename = "AttachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

/// <p>Attaches a typed link to a specified source and target object inside a <a>BatchRead</a> operation. For more information, see <a>AttachTypedLink</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchAttachTypedLink {
    /// <p>A set of attributes that are associated with the typed link.</p>
    #[serde(rename = "Attributes")]
    pub attributes: Vec<AttributeNameAndValue>,
    /// <p>Identifies the source object that the typed link will attach to.</p>
    #[serde(rename = "SourceObjectReference")]
    pub source_object_reference: ObjectReference,
    /// <p>Identifies the target object that the typed link will attach to.</p>
    #[serde(rename = "TargetObjectReference")]
    pub target_object_reference: ObjectReference,
    /// <p>Identifies the typed link facet that is associated with the typed link.</p>
    #[serde(rename = "TypedLinkFacet")]
    pub typed_link_facet: TypedLinkSchemaAndFacetName,
}

/// <p>Represents the output of a <a>AttachTypedLink</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchAttachTypedLinkResponse {
    /// <p>Returns a typed link specifier as output.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_link_specifier: Option<TypedLinkSpecifier>,
}

/// <p>Creates an index object inside of a <a>BatchRead</a> operation. For more information, see <a>CreateIndex</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchCreateIndex {
    /// <p>The batch reference name. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#batches">Batches</a> for more information.</p>
    #[serde(rename = "BatchReferenceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_reference_name: Option<String>,
    /// <p>Indicates whether the attribute that is being indexed has unique values or not.</p>
    #[serde(rename = "IsUnique")]
    pub is_unique: bool,
    /// <p>The name of the link between the parent object and the index object.</p>
    #[serde(rename = "LinkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    /// <p>Specifies the attributes that should be indexed on. Currently only a single attribute is supported.</p>
    #[serde(rename = "OrderedIndexedAttributeList")]
    pub ordered_indexed_attribute_list: Vec<AttributeKey>,
    /// <p>A reference to the parent object that contains the index object.</p>
    #[serde(rename = "ParentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
}

/// <p>Represents the output of a <a>CreateIndex</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchCreateIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the index created by this operation.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

/// <p>Represents the output of a <a>CreateObject</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchCreateObject {
    /// <p>The batch reference name. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#batches">Batches</a> for more information.</p>
    #[serde(rename = "BatchReferenceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_reference_name: Option<String>,
    /// <p>The name of the link.</p>
    #[serde(rename = "LinkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    /// <p>An attribute map, which contains an attribute ARN as the key and attribute value as the map value.</p>
    #[serde(rename = "ObjectAttributeList")]
    pub object_attribute_list: Vec<AttributeKeyAndValue>,
    /// <p>If specified, the parent reference to which this object will be attached.</p>
    #[serde(rename = "ParentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
    /// <p>A list of <code>FacetArns</code> that will be associated with the object. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaFacet")]
    pub schema_facet: Vec<SchemaFacet>,
}

/// <p>Represents the output of a <a>CreateObject</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchCreateObjectResponse {
    /// <p>The ID that is associated with the object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

/// <p>Represents the output of a <a>DeleteObject</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeleteObject {
    /// <p>The reference that identifies the object.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>DeleteObject</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDeleteObjectResponse {}

/// <p>Detaches the specified object from the specified index inside a <a>BatchRead</a> operation. For more information, see <a>DetachFromIndex</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDetachFromIndex {
    /// <p>A reference to the index object.</p>
    #[serde(rename = "IndexReference")]
    pub index_reference: ObjectReference,
    /// <p>A reference to the object being detached from the index.</p>
    #[serde(rename = "TargetReference")]
    pub target_reference: ObjectReference,
}

/// <p>Represents the output of a <a>DetachFromIndex</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetachFromIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the object that was detached from the index.</p>
    #[serde(rename = "DetachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

/// <p>Represents the output of a <a>DetachObject</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDetachObject {
    /// <p>The batch reference name. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#batches">Batches</a> for more information.</p>
    #[serde(rename = "BatchReferenceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_reference_name: Option<String>,
    /// <p>The name of the link.</p>
    #[serde(rename = "LinkName")]
    pub link_name: String,
    /// <p>Parent reference from which the object with the specified link name is detached.</p>
    #[serde(rename = "ParentReference")]
    pub parent_reference: ObjectReference,
}

/// <p>Represents the output of a <a>DetachObject</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetachObjectResponse {
    /// <p>The <code>ObjectIdentifier</code> of the detached object.</p>
    #[serde(rename = "detachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

/// <p>Detaches the specified policy from the specified directory inside a <a>BatchWrite</a> operation. For more information, see <a>DetachPolicy</a> and <a>BatchWriteRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDetachPolicy {
    /// <p>Reference that identifies the object whose policy object will be detached.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
    /// <p>Reference that identifies the policy object.</p>
    #[serde(rename = "PolicyReference")]
    pub policy_reference: ObjectReference,
}

/// <p>Represents the output of a <a>DetachPolicy</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetachPolicyResponse {}

/// <p>Detaches a typed link from a specified source and target object inside a <a>BatchRead</a> operation. For more information, see <a>DetachTypedLink</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDetachTypedLink {
    /// <p>Used to accept a typed link specifier as input.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

/// <p>Represents the output of a <a>DetachTypedLink</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetachTypedLinkResponse {}

/// <p>Retrieves attributes that are associated with a typed link inside a <a>BatchRead</a> operation. For more information, see <a>GetLinkAttributes</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetLinkAttributes {
    /// <p>A list of attribute names whose values will be retrieved.</p>
    #[serde(rename = "AttributeNames")]
    pub attribute_names: Vec<String>,
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

/// <p>Represents the output of a <a>GetLinkAttributes</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetLinkAttributesResponse {
    /// <p>The attributes that are associated with the typed link.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

/// <p>Retrieves attributes within a facet that are associated with an object inside an <a>BatchRead</a> operation. For more information, see <a>GetObjectAttributes</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetObjectAttributes {
    /// <p>List of attribute names whose values will be retrieved.</p>
    #[serde(rename = "AttributeNames")]
    pub attribute_names: Vec<String>,
    /// <p>Reference that identifies the object whose attributes will be retrieved.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
    /// <p>Identifier for the facet whose attributes will be retrieved. See <a>SchemaFacet</a> for details.</p>
    #[serde(rename = "SchemaFacet")]
    pub schema_facet: SchemaFacet,
}

/// <p>Represents the output of a <a>GetObjectAttributes</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetObjectAttributesResponse {
    /// <p>The attribute values that are associated with an object.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

/// <p>Retrieves metadata about an object inside a <a>BatchRead</a> operation. For more information, see <a>GetObjectInformation</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetObjectInformation {
    /// <p>A reference to the object.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>GetObjectInformation</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetObjectInformationResponse {
    /// <p>The <code>ObjectIdentifier</code> of the specified object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    /// <p>The facets attached to the specified object.</p>
    #[serde(rename = "SchemaFacets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_facets: Option<Vec<SchemaFacet>>,
}

/// <p>Lists indices attached to an object inside a <a>BatchRead</a> operation. For more information, see <a>ListAttachedIndices</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchListAttachedIndices {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A reference to the object that has indices attached.</p>
    #[serde(rename = "TargetReference")]
    pub target_reference: ObjectReference,
}

/// <p>Represents the output of a <a>ListAttachedIndices</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchListAttachedIndicesResponse {
    /// <p>The indices attached to the specified object.</p>
    #[serde(rename = "IndexAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object inside a <a>BatchRead</a> operation. For more information, see <a>ListIncomingTypedLinks</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchListIncomingTypedLinks {
    /// <p>Provides range filters for multiple attributes. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range.</p>
    #[serde(rename = "FilterAttributeRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    /// <p>Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls.</p>
    #[serde(rename = "FilterTypedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reference that identifies the object whose attributes will be listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>ListIncomingTypedLinks</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchListIncomingTypedLinksResponse {
    /// <p>Returns one or more typed link specifiers as output.</p>
    #[serde(rename = "LinkSpecifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_specifiers: Option<Vec<TypedLinkSpecifier>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Lists objects attached to the specified index inside a <a>BatchRead</a> operation. For more information, see <a>ListIndex</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchListIndex {
    /// <p>The reference to the index to list.</p>
    #[serde(rename = "IndexReference")]
    pub index_reference: ObjectReference,
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies the ranges of indexed values that you want to query.</p>
    #[serde(rename = "RangesOnIndexedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges_on_indexed_values: Option<Vec<ObjectAttributeRange>>,
}

/// <p>Represents the output of a <a>ListIndex</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchListIndexResponse {
    /// <p>The objects and indexed values attached to the index.</p>
    #[serde(rename = "IndexAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a <a>ListObjectAttributes</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchListObjectAttributes {
    /// <p>Used to filter the list of object attributes that are associated with a certain facet.</p>
    #[serde(rename = "FacetFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_filter: Option<SchemaFacet>,
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Reference of the object whose attributes need to be listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>ListObjectAttributes</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchListObjectAttributesResponse {
    /// <p>The attributes map that is associated with the object. <code>AttributeArn</code> is the key; attribute value is the value.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a <a>ListObjectChildren</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchListObjectChildren {
    /// <p>Maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Reference of the object for which child objects are being listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>ListObjectChildren</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchListObjectChildrenResponse {
    /// <p>The children structure, which is a map with the key as the <code>LinkName</code> and <code>ObjectIdentifier</code> as the value.</p>
    #[serde(rename = "Children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<::std::collections::HashMap<String, String>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects inside a <a>BatchRead</a> operation. For more information, see <a>ListObjectParentPaths</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchListObjectParentPaths {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reference that identifies the object whose attributes will be listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>ListObjectParentPaths</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchListObjectParentPathsResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns the path to the <code>ObjectIdentifiers</code> that are associated with the directory.</p>
    #[serde(rename = "PathToObjectIdentifiersList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_to_object_identifiers_list: Option<Vec<PathToObjectIdentifiers>>,
}

/// <p>Returns policies attached to an object in pagination fashion inside a <a>BatchRead</a> operation. For more information, see <a>ListObjectPolicies</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchListObjectPolicies {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reference that identifies the object whose attributes will be listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>ListObjectPolicies</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchListObjectPoliciesResponse {
    /// <p>A list of policy <code>ObjectIdentifiers</code>, that are attached to the object.</p>
    #[serde(rename = "AttachedPolicyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_policy_ids: Option<Vec<String>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object inside a <a>BatchRead</a> operation. For more information, see <a>ListOutgoingTypedLinks</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchListOutgoingTypedLinks {
    /// <p>Provides range filters for multiple attributes. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range.</p>
    #[serde(rename = "FilterAttributeRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    /// <p>Filters are interpreted in the order of the attributes defined on the typed link facet, not the order they are supplied to any API calls.</p>
    #[serde(rename = "FilterTypedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reference that identifies the object whose attributes will be listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>ListOutgoingTypedLinks</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchListOutgoingTypedLinksResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a typed link specifier as output.</p>
    #[serde(rename = "TypedLinkSpecifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_link_specifiers: Option<Vec<TypedLinkSpecifier>>,
}

/// <p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached inside a <a>BatchRead</a> operation. For more information, see <a>ListPolicyAttachments</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchListPolicyAttachments {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reference that identifies the policy object.</p>
    #[serde(rename = "PolicyReference")]
    pub policy_reference: ObjectReference,
}

/// <p>Represents the output of a <a>ListPolicyAttachments</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchListPolicyAttachmentsResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>ObjectIdentifiers</code> to which the policy is attached.</p>
    #[serde(rename = "ObjectIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifiers: Option<Vec<String>>,
}

/// <p>Lists all policies from the root of the Directory to the object specified inside a <a>BatchRead</a> operation. For more information, see <a>LookupPolicy</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchLookupPolicy {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Reference that identifies the object whose policies will be looked up.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>LookupPolicy</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchLookupPolicyResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Provides list of path to policies. Policies contain <code>PolicyId</code>, <code>ObjectIdentifier</code>, and <code>PolicyType</code>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies">Policies</a>.</p>
    #[serde(rename = "PolicyToPathList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_to_path_list: Option<Vec<PolicyToPath>>,
}

/// <p>The batch read exception structure, which contains the exception type and message.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchReadException {
    /// <p>An exception message that is associated with the failure.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A type of exception, such as <code>InvalidArnException</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the output of a <code>BatchRead</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchReadOperation {
    /// <p>Retrieves attributes that are associated with a typed link.</p>
    #[serde(rename = "GetLinkAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_link_attributes: Option<BatchGetLinkAttributes>,
    /// <p>Retrieves attributes within a facet that are associated with an object.</p>
    #[serde(rename = "GetObjectAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_object_attributes: Option<BatchGetObjectAttributes>,
    /// <p>Retrieves metadata about an object.</p>
    #[serde(rename = "GetObjectInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_object_information: Option<BatchGetObjectInformation>,
    /// <p>Lists indices attached to an object.</p>
    #[serde(rename = "ListAttachedIndices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_attached_indices: Option<BatchListAttachedIndices>,
    /// <p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "ListIncomingTypedLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_incoming_typed_links: Option<BatchListIncomingTypedLinks>,
    /// <p>Lists objects attached to the specified index.</p>
    #[serde(rename = "ListIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_index: Option<BatchListIndex>,
    /// <p>Lists all attributes that are associated with an object.</p>
    #[serde(rename = "ListObjectAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_attributes: Option<BatchListObjectAttributes>,
    /// <p>Returns a paginated list of child objects that are associated with a given object.</p>
    #[serde(rename = "ListObjectChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_children: Option<BatchListObjectChildren>,
    /// <p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#dirstructure">Directory Structure</a>.</p>
    #[serde(rename = "ListObjectParentPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_parent_paths: Option<BatchListObjectParentPaths>,
    /// <p>Returns policies attached to an object in pagination fashion.</p>
    #[serde(rename = "ListObjectPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_policies: Option<BatchListObjectPolicies>,
    /// <p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "ListOutgoingTypedLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_outgoing_typed_links: Option<BatchListOutgoingTypedLinks>,
    /// <p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>
    #[serde(rename = "ListPolicyAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_policy_attachments: Option<BatchListPolicyAttachments>,
    /// <p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies">Policies</a>.</p>
    #[serde(rename = "LookupPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_policy: Option<BatchLookupPolicy>,
}

/// <p>Represents the output of a <code>BatchRead</code> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchReadOperationResponse {
    /// <p>Identifies which operation in a batch has failed.</p>
    #[serde(rename = "ExceptionResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_response: Option<BatchReadException>,
    /// <p>Identifies which operation in a batch has succeeded.</p>
    #[serde(rename = "SuccessfulResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_response: Option<BatchReadSuccessfulResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchReadRequest {
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>A list of operations that are part of the batch.</p>
    #[serde(rename = "Operations")]
    pub operations: Vec<BatchReadOperation>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchReadResponse {
    /// <p>A list of all the responses for each batch read.</p>
    #[serde(rename = "Responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<BatchReadOperationResponse>>,
}

/// <p>Represents the output of a <code>BatchRead</code> success response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchReadSuccessfulResponse {
    /// <p>The list of attributes to retrieve from the typed link.</p>
    #[serde(rename = "GetLinkAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_link_attributes: Option<BatchGetLinkAttributesResponse>,
    /// <p>Retrieves attributes within a facet that are associated with an object.</p>
    #[serde(rename = "GetObjectAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_object_attributes: Option<BatchGetObjectAttributesResponse>,
    /// <p>Retrieves metadata about an object.</p>
    #[serde(rename = "GetObjectInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_object_information: Option<BatchGetObjectInformationResponse>,
    /// <p>Lists indices attached to an object.</p>
    #[serde(rename = "ListAttachedIndices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_attached_indices: Option<BatchListAttachedIndicesResponse>,
    /// <p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "ListIncomingTypedLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_incoming_typed_links: Option<BatchListIncomingTypedLinksResponse>,
    /// <p>Lists objects attached to the specified index.</p>
    #[serde(rename = "ListIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_index: Option<BatchListIndexResponse>,
    /// <p>Lists all attributes that are associated with an object.</p>
    #[serde(rename = "ListObjectAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_attributes: Option<BatchListObjectAttributesResponse>,
    /// <p>Returns a paginated list of child objects that are associated with a given object.</p>
    #[serde(rename = "ListObjectChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_children: Option<BatchListObjectChildrenResponse>,
    /// <p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#dirstructure">Directory Structure</a>.</p>
    #[serde(rename = "ListObjectParentPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_parent_paths: Option<BatchListObjectParentPathsResponse>,
    /// <p>Returns policies attached to an object in pagination fashion.</p>
    #[serde(rename = "ListObjectPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_policies: Option<BatchListObjectPoliciesResponse>,
    /// <p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "ListOutgoingTypedLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_outgoing_typed_links: Option<BatchListOutgoingTypedLinksResponse>,
    /// <p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>
    #[serde(rename = "ListPolicyAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_policy_attachments: Option<BatchListPolicyAttachmentsResponse>,
    /// <p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies">Policies</a>.</p>
    #[serde(rename = "LookupPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_policy: Option<BatchLookupPolicyResponse>,
}

/// <p>A batch operation to remove a facet from an object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchRemoveFacetFromObject {
    /// <p>A reference to the object whose facet will be removed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
    /// <p>The facet to remove from the object.</p>
    #[serde(rename = "SchemaFacet")]
    pub schema_facet: SchemaFacet,
}

/// <p>An empty result that represents success.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchRemoveFacetFromObjectResponse {}

/// <p>Updates a given typed link’s attributes inside a <a>BatchRead</a> operation. Attributes to be updated must not contribute to the typed link’s identity, as defined by its <code>IdentityAttributeOrder</code>. For more information, see <a>UpdateLinkAttributes</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchUpdateLinkAttributes {
    /// <p>The attributes update structure.</p>
    #[serde(rename = "AttributeUpdates")]
    pub attribute_updates: Vec<LinkAttributeUpdate>,
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

/// <p>Represents the output of a <a>UpdateLinkAttributes</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchUpdateLinkAttributesResponse {}

/// <p>Represents the output of a <code>BatchUpdate</code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchUpdateObjectAttributes {
    /// <p>Attributes update structure.</p>
    #[serde(rename = "AttributeUpdates")]
    pub attribute_updates: Vec<ObjectAttributeUpdate>,
    /// <p>Reference that identifies the object.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <code>BatchUpdate</code> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchUpdateObjectAttributesResponse {
    /// <p>ID that is associated with the object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

/// <p>Represents the output of a <code>BatchWrite</code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchWriteOperation {
    /// <p>A batch operation that adds a facet to an object.</p>
    #[serde(rename = "AddFacetToObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_facet_to_object: Option<BatchAddFacetToObject>,
    /// <p>Attaches an object to a <a>Directory</a>.</p>
    #[serde(rename = "AttachObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_object: Option<BatchAttachObject>,
    /// <p>Attaches a policy object to a regular object. An object can have a limited number of attached policies.</p>
    #[serde(rename = "AttachPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_policy: Option<BatchAttachPolicy>,
    /// <p>Attaches the specified object to the specified index.</p>
    #[serde(rename = "AttachToIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_index: Option<BatchAttachToIndex>,
    /// <p>Attaches a typed link to a specified source and target object. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "AttachTypedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_typed_link: Option<BatchAttachTypedLink>,
    /// <p>Creates an index object. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_indexing.html">Indexing</a> for more information.</p>
    #[serde(rename = "CreateIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_index: Option<BatchCreateIndex>,
    /// <p>Creates an object.</p>
    #[serde(rename = "CreateObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_object: Option<BatchCreateObject>,
    /// <p>Deletes an object in a <a>Directory</a>.</p>
    #[serde(rename = "DeleteObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_object: Option<BatchDeleteObject>,
    /// <p>Detaches the specified object from the specified index.</p>
    #[serde(rename = "DetachFromIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_from_index: Option<BatchDetachFromIndex>,
    /// <p>Detaches an object from a <a>Directory</a>.</p>
    #[serde(rename = "DetachObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_object: Option<BatchDetachObject>,
    /// <p>Detaches a policy from a <a>Directory</a>.</p>
    #[serde(rename = "DetachPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_policy: Option<BatchDetachPolicy>,
    /// <p>Detaches a typed link from a specified source and target object. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "DetachTypedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_typed_link: Option<BatchDetachTypedLink>,
    /// <p>A batch operation that removes a facet from an object.</p>
    #[serde(rename = "RemoveFacetFromObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_facet_from_object: Option<BatchRemoveFacetFromObject>,
    /// <p>Updates a given object's attributes.</p>
    #[serde(rename = "UpdateLinkAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_link_attributes: Option<BatchUpdateLinkAttributes>,
    /// <p>Updates a given object's attributes.</p>
    #[serde(rename = "UpdateObjectAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_object_attributes: Option<BatchUpdateObjectAttributes>,
}

/// <p>Represents the output of a <code>BatchWrite</code> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchWriteOperationResponse {
    /// <p>The result of an add facet to object batch operation.</p>
    #[serde(rename = "AddFacetToObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_facet_to_object: Option<BatchAddFacetToObjectResponse>,
    /// <p>Attaches an object to a <a>Directory</a>.</p>
    #[serde(rename = "AttachObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_object: Option<BatchAttachObjectResponse>,
    /// <p>Attaches a policy object to a regular object. An object can have a limited number of attached policies.</p>
    #[serde(rename = "AttachPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_policy: Option<BatchAttachPolicyResponse>,
    /// <p>Attaches the specified object to the specified index.</p>
    #[serde(rename = "AttachToIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_index: Option<BatchAttachToIndexResponse>,
    /// <p>Attaches a typed link to a specified source and target object. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "AttachTypedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_typed_link: Option<BatchAttachTypedLinkResponse>,
    /// <p>Creates an index object. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_indexing.html">Indexing</a> for more information.</p>
    #[serde(rename = "CreateIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_index: Option<BatchCreateIndexResponse>,
    /// <p>Creates an object in a <a>Directory</a>.</p>
    #[serde(rename = "CreateObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_object: Option<BatchCreateObjectResponse>,
    /// <p>Deletes an object in a <a>Directory</a>.</p>
    #[serde(rename = "DeleteObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_object: Option<BatchDeleteObjectResponse>,
    /// <p>Detaches the specified object from the specified index.</p>
    #[serde(rename = "DetachFromIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_from_index: Option<BatchDetachFromIndexResponse>,
    /// <p>Detaches an object from a <a>Directory</a>.</p>
    #[serde(rename = "DetachObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_object: Option<BatchDetachObjectResponse>,
    /// <p>Detaches a policy from a <a>Directory</a>.</p>
    #[serde(rename = "DetachPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_policy: Option<BatchDetachPolicyResponse>,
    /// <p>Detaches a typed link from a specified source and target object. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "DetachTypedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_typed_link: Option<BatchDetachTypedLinkResponse>,
    /// <p>The result of a batch remove facet from object operation.</p>
    #[serde(rename = "RemoveFacetFromObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_facet_from_object: Option<BatchRemoveFacetFromObjectResponse>,
    /// <p>Represents the output of a <code>BatchWrite</code> response operation.</p>
    #[serde(rename = "UpdateLinkAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_link_attributes: Option<BatchUpdateLinkAttributesResponse>,
    /// <p>Updates a given object’s attributes.</p>
    #[serde(rename = "UpdateObjectAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_object_attributes: Option<BatchUpdateObjectAttributesResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchWriteRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>A list of operations that are part of the batch.</p>
    #[serde(rename = "Operations")]
    pub operations: Vec<BatchWriteOperation>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchWriteResponse {
    /// <p>A list of all the responses for each batch write.</p>
    #[serde(rename = "Responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<BatchWriteOperationResponse>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDirectoryRequest {
    /// <p>The name of the <a>Directory</a>. Should be unique per account, per region.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the published schema that will be copied into the data <a>Directory</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDirectoryResponse {
    /// <p>The ARN of the published schema in the <a>Directory</a>. Once a published schema is copied into the directory, it has its own ARN, which is referred to applied schema ARN. For more information, see <a>arns</a>.</p>
    #[serde(rename = "AppliedSchemaArn")]
    pub applied_schema_arn: String,
    /// <p>The ARN that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The name of the <a>Directory</a>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The root object node of the created directory.</p>
    #[serde(rename = "ObjectIdentifier")]
    pub object_identifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFacetRequest {
    /// <p>The attributes that are associated with the <a>Facet</a>.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<FacetAttribute>>,
    /// <p>The name of the <a>Facet</a>, which is unique for a given schema.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether a given object created from this facet is of type node, leaf node, policy or index.</p> <ul> <li> <p>Node: Can have multiple children but one parent.</p> </li> </ul> <ul> <li> <p>Leaf node: Cannot have children but can have multiple parents.</p> </li> </ul> <ul> <li> <p>Policy: Allows you to store a policy document and policy type. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies">Policies</a>.</p> </li> </ul> <ul> <li> <p>Index: Can be created with the Index API.</p> </li> </ul></p>
    #[serde(rename = "ObjectType")]
    pub object_type: String,
    /// <p>The schema ARN in which the new <a>Facet</a> will be created. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIndexRequest {
    /// <p>The ARN of the directory where the index should be created.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Indicates whether the attribute that is being indexed has unique values or not.</p>
    #[serde(rename = "IsUnique")]
    pub is_unique: bool,
    /// <p>The name of the link between the parent object and the index object.</p>
    #[serde(rename = "LinkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    /// <p>Specifies the attributes that should be indexed on. Currently only a single attribute is supported.</p>
    #[serde(rename = "OrderedIndexedAttributeList")]
    pub ordered_indexed_attribute_list: Vec<AttributeKey>,
    /// <p>A reference to the parent object that contains the index object.</p>
    #[serde(rename = "ParentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the index created by this operation.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateObjectRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> in which the object will be created. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The name of link that is used to attach this object to a parent.</p>
    #[serde(rename = "LinkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    /// <p>The attribute map whose attribute ARN contains the key and attribute value as the map value.</p>
    #[serde(rename = "ObjectAttributeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_list: Option<Vec<AttributeKeyAndValue>>,
    /// <p>If specified, the parent reference to which this object will be attached.</p>
    #[serde(rename = "ParentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
    /// <p>A list of schema facets to be associated with the object. Do not provide minor version components. See <a>SchemaFacet</a> for details.</p>
    #[serde(rename = "SchemaFacets")]
    pub schema_facets: Vec<SchemaFacet>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateObjectResponse {
    /// <p>The identifier that is associated with the object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSchemaRequest {
    /// <p>The name that is associated with the schema. This is unique to each account and in each region.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSchemaResponse {
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTypedLinkFacetRequest {
    /// <p> <a>Facet</a> structure that is associated with the typed link facet.</p>
    #[serde(rename = "Facet")]
    pub facet: TypedLinkFacet,
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateTypedLinkFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDirectoryRequest {
    /// <p>The ARN of the directory to delete.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDirectoryResponse {
    /// <p>The ARN of the deleted directory.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFacetRequest {
    /// <p>The name of the facet to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Facet</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteObjectRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>A reference that identifies the object.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteObjectResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSchemaRequest {
    /// <p>The Amazon Resource Name (ARN) of the development schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteSchemaResponse {
    /// <p>The input ARN that is returned as part of the response. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTypedLinkFacetRequest {
    /// <p>The unique name of the typed link facet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteTypedLinkFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachFromIndexRequest {
    /// <p>The Amazon Resource Name (ARN) of the directory the index and object exist in.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>A reference to the index object.</p>
    #[serde(rename = "IndexReference")]
    pub index_reference: ObjectReference,
    /// <p>A reference to the object being detached from the index.</p>
    #[serde(rename = "TargetReference")]
    pub target_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetachFromIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the object that was detached from the index.</p>
    #[serde(rename = "DetachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachObjectRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where objects reside. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The link name associated with the object that needs to be detached.</p>
    #[serde(rename = "LinkName")]
    pub link_name: String,
    /// <p>The parent reference from which the object with the specified link name is detached.</p>
    #[serde(rename = "ParentReference")]
    pub parent_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetachObjectResponse {
    /// <p>The <code>ObjectIdentifier</code> that was detached from the object.</p>
    #[serde(rename = "DetachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where both objects reside. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Reference that identifies the object whose policy object will be detached.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
    /// <p>Reference that identifies the policy object.</p>
    #[serde(rename = "PolicyReference")]
    pub policy_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetachPolicyResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachTypedLinkRequest {
    /// <p>The Amazon Resource Name (ARN) of the directory where you want to detach the typed link.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Used to accept a typed link specifier as input.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

/// <p>Directory structure that includes the directory name and directory ARN.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Directory {
    /// <p>The date and time when the directory was created.</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the directory. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
    /// <p>The name of the directory.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the directory. Can be either <code>Enabled</code>, <code>Disabled</code>, or <code>Deleted</code>.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableDirectoryRequest {
    /// <p>The ARN of the directory to disable.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisableDirectoryResponse {
    /// <p>The ARN of the directory that has been disabled.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableDirectoryRequest {
    /// <p>The ARN of the directory to enable.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnableDirectoryResponse {
    /// <p>The ARN of the enabled directory.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

/// <p>A structure that contains <code>Name</code>, <code>ARN</code>, <code>Attributes</code>, <code> <a>Rule</a>s</code>, and <code>ObjectTypes</code>. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/whatarefacets.html">Facets</a> for more information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Facet {
    /// <p>The name of the <a>Facet</a>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The object type that is associated with the facet. See <a>CreateFacetRequest$ObjectType</a> for more details.</p>
    #[serde(rename = "ObjectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
}

/// <p>An attribute that is associated with the <a>Facet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FacetAttribute {
    /// <p>A facet attribute consists of either a definition or a reference. This structure contains the attribute definition. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#attributereferences">Attribute References</a> for more information.</p>
    #[serde(rename = "AttributeDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definition: Option<FacetAttributeDefinition>,
    /// <p>An attribute reference that is associated with the attribute. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#attributereferences">Attribute References</a> for more information.</p>
    #[serde(rename = "AttributeReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_reference: Option<FacetAttributeReference>,
    /// <p>The name of the facet attribute.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The required behavior of the <code>FacetAttribute</code>.</p>
    #[serde(rename = "RequiredBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_behavior: Option<String>,
}

/// <p>A facet attribute definition. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#attributereferences">Attribute References</a> for more information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FacetAttributeDefinition {
    /// <p>The default value of the attribute (if configured).</p>
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<TypedAttributeValue>,
    /// <p>Whether the attribute is mutable or not.</p>
    #[serde(rename = "IsImmutable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immutable: Option<bool>,
    /// <p>Validation rules attached to the attribute definition.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<::std::collections::HashMap<String, Rule>>,
    /// <p>The type of the attribute.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>The facet attribute reference that specifies the attribute definition that contains the attribute facet name and attribute name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FacetAttributeReference {
    /// <p>The target attribute name that is associated with the facet reference. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#attributereferences">Attribute References</a> for more information.</p>
    #[serde(rename = "TargetAttributeName")]
    pub target_attribute_name: String,
    /// <p>The target facet name that is associated with the facet reference. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#attributereferences">Attribute References</a> for more information.</p>
    #[serde(rename = "TargetFacetName")]
    pub target_facet_name: String,
}

/// <p>A structure that contains information used to update an attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FacetAttributeUpdate {
    /// <p>The action to perform when updating the attribute.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The attribute to update.</p>
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<FacetAttribute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppliedSchemaVersionRequest {
    /// <p>The ARN of the applied schema.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAppliedSchemaVersionResponse {
    /// <p>Current applied schema ARN, including the minor version in use if one was provided.</p>
    #[serde(rename = "AppliedSchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDirectoryRequest {
    /// <p>The ARN of the directory.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDirectoryResponse {
    /// <p>Metadata about the directory.</p>
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFacetRequest {
    /// <p>The name of the facet to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Facet</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetFacetResponse {
    /// <p>The <a>Facet</a> structure that is associated with the facet.</p>
    #[serde(rename = "Facet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet: Option<Facet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLinkAttributesRequest {
    /// <p>A list of attribute names whose values will be retrieved.</p>
    #[serde(rename = "AttributeNames")]
    pub attribute_names: Vec<String>,
    /// <p>The consistency level at which to retrieve the attributes on a typed link.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the typed link resides. For more information, see <a>arns</a> or <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetLinkAttributesResponse {
    /// <p>The attributes that are associated with the typed link.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetObjectAttributesRequest {
    /// <p>List of attribute names whose values will be retrieved.</p>
    #[serde(rename = "AttributeNames")]
    pub attribute_names: Vec<String>,
    /// <p>The consistency level at which to retrieve the attributes on an object.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Reference that identifies the object whose attributes will be retrieved.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
    /// <p>Identifier for the facet whose attributes will be retrieved. See <a>SchemaFacet</a> for details.</p>
    #[serde(rename = "SchemaFacet")]
    pub schema_facet: SchemaFacet,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetObjectAttributesResponse {
    /// <p>The attributes that are associated with the object.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetObjectInformationRequest {
    /// <p>The consistency level at which to retrieve the object information.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The ARN of the directory being retrieved.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>A reference to the object.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetObjectInformationResponse {
    /// <p>The <code>ObjectIdentifier</code> of the specified object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    /// <p>The facets attached to the specified object. Although the response does not include minor version information, the most recently applied minor version of each Facet is in effect. See <a>GetAppliedSchemaVersion</a> for details.</p>
    #[serde(rename = "SchemaFacets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_facets: Option<Vec<SchemaFacet>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSchemaAsJsonRequest {
    /// <p>The ARN of the schema to retrieve.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSchemaAsJsonResponse {
    /// <p>The JSON representation of the schema document.</p>
    #[serde(rename = "Document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// <p>The name of the retrieved schema.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTypedLinkFacetInformationRequest {
    /// <p>The unique name of the typed link facet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetTypedLinkFacetInformationResponse {
    /// <p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed links considers the order that the attributes are defined on the typed link facet. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range. Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls. For more information about identity attributes, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "IdentityAttributeOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_attribute_order: Option<Vec<String>>,
}

/// <p>Represents an index and an attached object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct IndexAttachment {
    /// <p>The indexed attribute values.</p>
    #[serde(rename = "IndexedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_attributes: Option<Vec<AttributeKeyAndValue>>,
    /// <p>In response to <a>ListIndex</a>, the <code>ObjectIdentifier</code> of the object attached to the index. In response to <a>ListAttachedIndices</a>, the <code>ObjectIdentifier</code> of the index attached to the object. This field will always contain the <code>ObjectIdentifier</code> of the object on the opposite side of the attachment specified in the query.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

/// <p>The action to take on a typed link attribute value. Updates are only supported for attributes which don’t contribute to link identity.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LinkAttributeAction {
    /// <p>A type that can be either <code>UPDATE_OR_CREATE</code> or <code>DELETE</code>.</p>
    #[serde(rename = "AttributeActionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_action_type: Option<String>,
    /// <p>The value that you want to update to.</p>
    #[serde(rename = "AttributeUpdateValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_update_value: Option<TypedAttributeValue>,
}

/// <p>Structure that contains attribute update information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LinkAttributeUpdate {
    /// <p>The action to perform as part of the attribute update.</p>
    #[serde(rename = "AttributeAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_action: Option<LinkAttributeAction>,
    /// <p>The key of the attribute being updated.</p>
    #[serde(rename = "AttributeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_key: Option<AttributeKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAppliedSchemaArnsRequest {
    /// <p>The ARN of the directory you are listing.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The response for <code>ListAppliedSchemaArns</code> when this parameter is used will list all minor version ARNs for a major version.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAppliedSchemaArnsResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARNs of schemas that are applied to the directory.</p>
    #[serde(rename = "SchemaArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAttachedIndicesRequest {
    /// <p>The consistency level to use for this operation.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The ARN of the directory.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A reference to the object that has indices attached.</p>
    #[serde(rename = "TargetReference")]
    pub target_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAttachedIndicesResponse {
    /// <p>The indices attached to the specified object.</p>
    #[serde(rename = "IndexAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDevelopmentSchemaArnsRequest {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDevelopmentSchemaArnsResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARNs of retrieved development schemas.</p>
    #[serde(rename = "SchemaArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDirectoriesRequest {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The state of the directories in the list. Can be either Enabled, Disabled, or Deleted.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDirectoriesResponse {
    /// <p>Lists all directories that are associated with your account in pagination fashion.</p>
    #[serde(rename = "Directories")]
    pub directories: Vec<Directory>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFacetAttributesRequest {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the facet whose attributes will be retrieved.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the schema where the facet resides.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListFacetAttributesResponse {
    /// <p>The attributes attached to the facet.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<FacetAttribute>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFacetNamesRequest {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) to retrieve facet names from.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListFacetNamesResponse {
    /// <p>The names of facets that exist within the schema.</p>
    #[serde(rename = "FacetNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_names: Option<Vec<String>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListIncomingTypedLinksRequest {
    /// <p>The consistency level to execute the request at.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the directory where you want to list the typed links.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Provides range filters for multiple attributes. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range.</p>
    #[serde(rename = "FilterAttributeRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    /// <p>Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls.</p>
    #[serde(rename = "FilterTypedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Reference that identifies the object whose attributes will be listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListIncomingTypedLinksResponse {
    /// <p>Returns one or more typed link specifiers as output.</p>
    #[serde(rename = "LinkSpecifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_specifiers: Option<Vec<TypedLinkSpecifier>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListIndexRequest {
    /// <p>The consistency level to execute the request at.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The ARN of the directory that the index exists in.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The reference to the index to list.</p>
    #[serde(rename = "IndexReference")]
    pub index_reference: ObjectReference,
    /// <p>The maximum number of objects in a single page to retrieve from the index during a request. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html#limits_cd">AWS Directory Service Limits</a>.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies the ranges of indexed values that you want to query.</p>
    #[serde(rename = "RangesOnIndexedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges_on_indexed_values: Option<Vec<ObjectAttributeRange>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListIndexResponse {
    /// <p>The objects and indexed values attached to the index.</p>
    #[serde(rename = "IndexAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListObjectAttributesRequest {
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Used to filter the list of object attributes that are associated with a certain facet.</p>
    #[serde(rename = "FacetFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_filter: Option<SchemaFacet>,
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reference that identifies the object whose attributes will be listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListObjectAttributesResponse {
    /// <p>Attributes map that is associated with the object. <code>AttributeArn</code> is the key, and attribute value is the value.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListObjectChildrenRequest {
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reference that identifies the object for which child objects are being listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListObjectChildrenResponse {
    /// <p>Children structure, which is a map with key as the <code>LinkName</code> and <code>ObjectIdentifier</code> as the value.</p>
    #[serde(rename = "Children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<::std::collections::HashMap<String, String>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListObjectParentPathsRequest {
    /// <p>The ARN of the directory to which the parent path applies.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reference that identifies the object whose parent paths are listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListObjectParentPathsResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns the path to the <code>ObjectIdentifiers</code> that are associated with the directory.</p>
    #[serde(rename = "PathToObjectIdentifiersList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_to_object_identifiers_list: Option<Vec<PathToObjectIdentifiers>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListObjectParentsRequest {
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reference that identifies the object for which parent objects are being listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListObjectParentsResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The parent structure, which is a map with key as the <code>ObjectIdentifier</code> and LinkName as the value.</p>
    #[serde(rename = "Parents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListObjectPoliciesRequest {
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where objects reside. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Reference that identifies the object for which policies will be listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListObjectPoliciesResponse {
    /// <p>A list of policy <code>ObjectIdentifiers</code>, that are attached to the object.</p>
    #[serde(rename = "AttachedPolicyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_policy_ids: Option<Vec<String>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOutgoingTypedLinksRequest {
    /// <p>The consistency level to execute the request at.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the directory where you want to list the typed links.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Provides range filters for multiple attributes. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range.</p>
    #[serde(rename = "FilterAttributeRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    /// <p>Filters are interpreted in the order of the attributes defined on the typed link facet, not the order they are supplied to any API calls.</p>
    #[serde(rename = "FilterTypedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A reference that identifies the object whose attributes will be listed.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListOutgoingTypedLinksResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a typed link specifier as output.</p>
    #[serde(rename = "TypedLinkSpecifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_link_specifiers: Option<Vec<TypedLinkSpecifier>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPolicyAttachmentsRequest {
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where objects reside. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reference that identifies the policy object.</p>
    #[serde(rename = "PolicyReference")]
    pub policy_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListPolicyAttachmentsResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>ObjectIdentifiers</code> to which the policy is attached.</p>
    #[serde(rename = "ObjectIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifiers: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPublishedSchemaArnsRequest {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The response for <code>ListPublishedSchemaArns</code> when this parameter is used will list all minor version ARNs for a major version.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListPublishedSchemaArnsResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARNs of published schemas.</p>
    #[serde(rename = "SchemaArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The <code>MaxResults</code> parameter sets the maximum number of results returned in a single page. This is for future use and is not supported currently.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token. This is for future use. Currently pagination is not supported for tagging.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource. Tagging is only supported for directories.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsForResourceResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tag key value pairs that are associated with the response.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTypedLinkFacetAttributesRequest {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The unique name of the typed link facet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTypedLinkFacetAttributesResponse {
    /// <p>An ordered set of attributes associate with the typed link.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<TypedLinkAttributeDefinition>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTypedLinkFacetNamesRequest {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTypedLinkFacetNamesResponse {
    /// <p>The names of typed link facets that exist within the schema.</p>
    #[serde(rename = "FacetNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_names: Option<Vec<String>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LookupPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Reference that identifies the object whose policies will be looked up.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LookupPolicyResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Provides list of path to policies. Policies contain <code>PolicyId</code>, <code>ObjectIdentifier</code>, and <code>PolicyType</code>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies">Policies</a>.</p>
    #[serde(rename = "PolicyToPathList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_to_path_list: Option<Vec<PolicyToPath>>,
}

/// <p>The action to take on the object attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ObjectAttributeAction {
    /// <p>A type that can be either <code>Update</code> or <code>Delete</code>.</p>
    #[serde(rename = "ObjectAttributeActionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_action_type: Option<String>,
    /// <p>The value that you want to update to.</p>
    #[serde(rename = "ObjectAttributeUpdateValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_update_value: Option<TypedAttributeValue>,
}

/// <p>A range of attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ObjectAttributeRange {
    /// <p>The key of the attribute that the attribute range covers.</p>
    #[serde(rename = "AttributeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_key: Option<AttributeKey>,
    /// <p>The range of attribute values being selected.</p>
    #[serde(rename = "Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<TypedAttributeValueRange>,
}

/// <p>Structure that contains attribute update information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ObjectAttributeUpdate {
    /// <p>The action to perform as part of the attribute update.</p>
    #[serde(rename = "ObjectAttributeAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_action: Option<ObjectAttributeAction>,
    /// <p>The key of the attribute being updated.</p>
    #[serde(rename = "ObjectAttributeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_key: Option<AttributeKey>,
}

/// <p>The reference that identifies an object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectReference {
    /// <p><p>A path selector supports easy selection of an object by the parent/child links leading to it from the directory root. Use the link names from each parent/child link to construct the path. Path selectors start with a slash (/) and link names are separated by slashes. For more information about paths, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#accessingobjects">Accessing Objects</a>. You can identify an object in one of the following ways:</p> <ul> <li> <p> <i>$ObjectIdentifier</i> - An object identifier is an opaque string provided by Amazon Cloud Directory. When creating objects, the system will provide you with the identifier of the created object. An object’s identifier is immutable and no two objects will ever share the same object identifier</p> </li> <li> <p> <i>/some/path</i> - Identifies the object based on path</p> </li> <li> <p> <i>#SomeBatchReference</i> - Identifies the object in a batch call</p> </li> </ul></p>
    #[serde(rename = "Selector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

/// <p>Returns the path to the <code>ObjectIdentifiers</code> that is associated with the directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PathToObjectIdentifiers {
    /// <p>Lists <code>ObjectIdentifiers</code> starting from directory root to the object in the request.</p>
    #[serde(rename = "ObjectIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifiers: Option<Vec<String>>,
    /// <p>The path that is used to identify the object starting from directory root.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>Contains the <code>PolicyType</code>, <code>PolicyId</code>, and the <code>ObjectIdentifier</code> to which it is attached. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies">Policies</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PolicyAttachment {
    /// <p>The <code>ObjectIdentifier</code> that is associated with <code>PolicyAttachment</code>.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    /// <p>The ID of <code>PolicyAttachment</code>.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// <p>The type of policy that can be associated with <code>PolicyAttachment</code>.</p>
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

/// <p>Used when a regular object exists in a <a>Directory</a> and you want to find all of the policies that are associated with that object and the parent to that object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PolicyToPath {
    /// <p>The path that is referenced from the root.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>List of policy objects.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicyAttachment>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PublishSchemaRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the development schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DevelopmentSchemaArn")]
    pub development_schema_arn: String,
    /// <p>The minor version under which the schema will be published. This parameter is recommended. Schemas have both a major and minor version associated with them.</p>
    #[serde(rename = "MinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    /// <p>The new name under which the schema will be published. If this is not provided, the development schema is considered.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The major version under which the schema will be published. Schemas have both a major and minor version associated with them.</p>
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PublishSchemaResponse {
    /// <p>The ARN that is associated with the published schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "PublishedSchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutSchemaFromJsonRequest {
    /// <p>The replacement JSON schema.</p>
    #[serde(rename = "Document")]
    pub document: String,
    /// <p>The ARN of the schema to update.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutSchemaFromJsonResponse {
    /// <p>The ARN of the schema to update.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveFacetFromObjectRequest {
    /// <p>The ARN of the directory in which the object resides.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>A reference to the object to remove the facet from.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
    /// <p>The facet to remove. See <a>SchemaFacet</a> for details.</p>
    #[serde(rename = "SchemaFacet")]
    pub schema_facet: SchemaFacet,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RemoveFacetFromObjectResponse {}

/// <p>Contains an Amazon Resource Name (ARN) and parameters that are associated with the rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    /// <p>The minimum and maximum parameters that are associated with the rule.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of attribute validation rule.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A facet.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaFacet {
    /// <p>The name of the facet.</p>
    #[serde(rename = "FacetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_name: Option<String>,
    /// <p>The ARN of the schema that contains the facet with no minor component. See <a>arns</a> and <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/inplaceschemaupgrade.html">In-Place Schema Upgrade</a> for a description of when to provide minor versions.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

/// <p>The tag structure that contains a tag key and value.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key that is associated with the tag.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value that is associated with the tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource. Tagging is only supported for directories.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A list of tag key-value pairs.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TagResourceResponse {}

/// <p>Represents the data for a typed attribute. You can set one, and only one, of the elements. Each attribute in an item is a name-value pair. Attributes have a single value.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedAttributeValue {
    /// <p>A binary data value.</p>
    #[serde(rename = "BinaryValue")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub binary_value: Option<Vec<u8>>,
    /// <p>A Boolean data value.</p>
    #[serde(rename = "BooleanValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    /// <p>A date and time value.</p>
    #[serde(rename = "DatetimeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_value: Option<f64>,
    /// <p>A number data value.</p>
    #[serde(rename = "NumberValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_value: Option<String>,
    /// <p>A string data value.</p>
    #[serde(rename = "StringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

/// <p>A range of attribute values. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#rangefilters">Range Filters</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TypedAttributeValueRange {
    /// <p>The inclusive or exclusive range end.</p>
    #[serde(rename = "EndMode")]
    pub end_mode: String,
    /// <p>The attribute value to terminate the range at.</p>
    #[serde(rename = "EndValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value: Option<TypedAttributeValue>,
    /// <p>The inclusive or exclusive range start.</p>
    #[serde(rename = "StartMode")]
    pub start_mode: String,
    /// <p>The value to start the range at.</p>
    #[serde(rename = "StartValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_value: Option<TypedAttributeValue>,
}

/// <p>A typed link attribute definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedLinkAttributeDefinition {
    /// <p>The default value of the attribute (if configured).</p>
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<TypedAttributeValue>,
    /// <p>Whether the attribute is mutable or not.</p>
    #[serde(rename = "IsImmutable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immutable: Option<bool>,
    /// <p>The unique name of the typed link attribute.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The required behavior of the <code>TypedLinkAttributeDefinition</code>.</p>
    #[serde(rename = "RequiredBehavior")]
    pub required_behavior: String,
    /// <p>Validation rules that are attached to the attribute definition.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<::std::collections::HashMap<String, Rule>>,
    /// <p>The type of the attribute.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Identifies the range of attributes that are used by a specified filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TypedLinkAttributeRange {
    /// <p>The unique name of the typed link attribute.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The range of attribute values that are being selected.</p>
    #[serde(rename = "Range")]
    pub range: TypedAttributeValueRange,
}

/// <p>Defines the typed links structure and its attributes. To create a typed link facet, use the <a>CreateTypedLinkFacet</a> API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TypedLinkFacet {
    /// <p>A set of key-value pairs associated with the typed link. Typed link attributes are used when you have data values that are related to the link itself, and not to one of the two objects being linked. Identity attributes also serve to distinguish the link from others of the same type between the same objects.</p>
    #[serde(rename = "Attributes")]
    pub attributes: Vec<TypedLinkAttributeDefinition>,
    /// <p>The set of attributes that distinguish links made from this facet from each other, in the order of significance. Listing typed links can filter on the values of these attributes. See <a>ListOutgoingTypedLinks</a> and <a>ListIncomingTypedLinks</a> for details.</p>
    #[serde(rename = "IdentityAttributeOrder")]
    pub identity_attribute_order: Vec<String>,
    /// <p>The unique name of the typed link facet.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>A typed link facet attribute update.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TypedLinkFacetAttributeUpdate {
    /// <p>The action to perform when updating the attribute.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>The attribute to update.</p>
    #[serde(rename = "Attribute")]
    pub attribute: TypedLinkAttributeDefinition,
}

/// <p>Identifies the schema Amazon Resource Name (ARN) and facet name for the typed link.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedLinkSchemaAndFacetName {
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
    /// <p>The unique name of the typed link facet.</p>
    #[serde(rename = "TypedLinkName")]
    pub typed_link_name: String,
}

/// <p>Contains all the information that is used to uniquely identify a typed link. The parameters discussed in this topic are used to uniquely specify the typed link being operated on. The <a>AttachTypedLink</a> API returns a typed link specifier while the <a>DetachTypedLink</a> API accepts one as input. Similarly, the <a>ListIncomingTypedLinks</a> and <a>ListOutgoingTypedLinks</a> API operations provide typed link specifiers as output. You can also construct a typed link specifier from scratch.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedLinkSpecifier {
    /// <p>Identifies the attribute value to update.</p>
    #[serde(rename = "IdentityAttributeValues")]
    pub identity_attribute_values: Vec<AttributeNameAndValue>,
    /// <p>Identifies the source object that the typed link will attach to.</p>
    #[serde(rename = "SourceObjectReference")]
    pub source_object_reference: ObjectReference,
    /// <p>Identifies the target object that the typed link will attach to.</p>
    #[serde(rename = "TargetObjectReference")]
    pub target_object_reference: ObjectReference,
    /// <p>Identifies the typed link facet that is associated with the typed link.</p>
    #[serde(rename = "TypedLinkFacet")]
    pub typed_link_facet: TypedLinkSchemaAndFacetName,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource. Tagging is only supported for directories.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Keys of the tag that need to be removed from the resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFacetRequest {
    /// <p>List of attributes that need to be updated in a given schema <a>Facet</a>. Each attribute is followed by <code>AttributeAction</code>, which specifies the type of update operation to perform. </p>
    #[serde(rename = "AttributeUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_updates: Option<Vec<FacetAttributeUpdate>>,
    /// <p>The name of the facet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The object type that is associated with the facet. See <a>CreateFacetRequest$ObjectType</a> for more details.</p>
    #[serde(rename = "ObjectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Facet</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateLinkAttributesRequest {
    /// <p>The attributes update structure.</p>
    #[serde(rename = "AttributeUpdates")]
    pub attribute_updates: Vec<LinkAttributeUpdate>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the updated typed link resides. For more information, see <a>arns</a> or <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateLinkAttributesResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateObjectAttributesRequest {
    /// <p>The attributes update structure.</p>
    #[serde(rename = "AttributeUpdates")]
    pub attribute_updates: Vec<ObjectAttributeUpdate>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>The reference that identifies the object.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateObjectAttributesResponse {
    /// <p>The <code>ObjectIdentifier</code> of the updated object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSchemaRequest {
    /// <p>The name of the schema.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the development schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateSchemaResponse {
    /// <p>The ARN that is associated with the updated schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTypedLinkFacetRequest {
    /// <p>Attributes update structure.</p>
    #[serde(rename = "AttributeUpdates")]
    pub attribute_updates: Vec<TypedLinkFacetAttributeUpdate>,
    /// <p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed links considers the order that the attributes are defined on the typed link facet. When providing ranges to a typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range. Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls. For more information about identity attributes, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    #[serde(rename = "IdentityAttributeOrder")]
    pub identity_attribute_order: Vec<String>,
    /// <p>The unique name of the typed link facet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateTypedLinkFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpgradeAppliedSchemaRequest {
    /// <p>The ARN for the directory to which the upgraded schema will be applied.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Used for testing whether the major version schemas are backward compatible or not. If schema compatibility fails, an exception would be thrown else the call would succeed but no changes will be saved. This parameter is optional.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>The revision of the published schema to upgrade the directory to.</p>
    #[serde(rename = "PublishedSchemaArn")]
    pub published_schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpgradeAppliedSchemaResponse {
    /// <p>The ARN of the directory that is returned as part of the response.</p>
    #[serde(rename = "DirectoryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
    /// <p>The ARN of the upgraded schema that is returned as part of the response.</p>
    #[serde(rename = "UpgradedSchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgraded_schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpgradePublishedSchemaRequest {
    /// <p>The ARN of the development schema with the changes used for the upgrade.</p>
    #[serde(rename = "DevelopmentSchemaArn")]
    pub development_schema_arn: String,
    /// <p>Used for testing whether the Development schema provided is backwards compatible, or not, with the publish schema provided by the user to be upgraded. If schema compatibility fails, an exception would be thrown else the call would succeed. This parameter is optional and defaults to false.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>Identifies the minor version of the published schema that will be created. This parameter is NOT optional.</p>
    #[serde(rename = "MinorVersion")]
    pub minor_version: String,
    /// <p>The ARN of the published schema to be upgraded.</p>
    #[serde(rename = "PublishedSchemaArn")]
    pub published_schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpgradePublishedSchemaResponse {
    /// <p>The ARN of the upgraded schema that is returned as part of the response.</p>
    #[serde(rename = "UpgradedSchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgraded_schema_arn: Option<String>,
}

/// Errors returned by AddFacetToObject
#[derive(Debug, PartialEq)]
pub enum AddFacetToObjectError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AddFacetToObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> AddFacetToObjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return AddFacetToObjectError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return AddFacetToObjectError::DirectoryNotEnabled(String::from(error_message))
                }
                "FacetValidationException" => {
                    return AddFacetToObjectError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return AddFacetToObjectError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return AddFacetToObjectError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return AddFacetToObjectError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return AddFacetToObjectError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return AddFacetToObjectError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return AddFacetToObjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AddFacetToObjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AddFacetToObjectError {
    fn from(err: serde_json::error::Error) -> AddFacetToObjectError {
        AddFacetToObjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AddFacetToObjectError {
    fn from(err: CredentialsError) -> AddFacetToObjectError {
        AddFacetToObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddFacetToObjectError {
    fn from(err: HttpDispatchError) -> AddFacetToObjectError {
        AddFacetToObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddFacetToObjectError {
    fn from(err: io::Error) -> AddFacetToObjectError {
        AddFacetToObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddFacetToObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddFacetToObjectError {
    fn description(&self) -> &str {
        match *self {
            AddFacetToObjectError::AccessDenied(ref cause) => cause,
            AddFacetToObjectError::DirectoryNotEnabled(ref cause) => cause,
            AddFacetToObjectError::FacetValidation(ref cause) => cause,
            AddFacetToObjectError::InternalService(ref cause) => cause,
            AddFacetToObjectError::InvalidArn(ref cause) => cause,
            AddFacetToObjectError::LimitExceeded(ref cause) => cause,
            AddFacetToObjectError::ResourceNotFound(ref cause) => cause,
            AddFacetToObjectError::RetryableConflict(ref cause) => cause,
            AddFacetToObjectError::Validation(ref cause) => cause,
            AddFacetToObjectError::Credentials(ref err) => err.description(),
            AddFacetToObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddFacetToObjectError::ParseError(ref cause) => cause,
            AddFacetToObjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ApplySchema
#[derive(Debug, PartialEq)]
pub enum ApplySchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that an attempt to attach an object with the same link name or to apply a schema with the same name has occurred. Rename the link or the schema and then try again.</p>
    InvalidAttachment(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ApplySchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> ApplySchemaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ApplySchemaError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ApplySchemaError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ApplySchemaError::InvalidArn(String::from(error_message))
                }
                "InvalidAttachmentException" => {
                    return ApplySchemaError::InvalidAttachment(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ApplySchemaError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ApplySchemaError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ApplySchemaError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ApplySchemaError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ApplySchemaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ApplySchemaError {
    fn from(err: serde_json::error::Error) -> ApplySchemaError {
        ApplySchemaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ApplySchemaError {
    fn from(err: CredentialsError) -> ApplySchemaError {
        ApplySchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ApplySchemaError {
    fn from(err: HttpDispatchError) -> ApplySchemaError {
        ApplySchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for ApplySchemaError {
    fn from(err: io::Error) -> ApplySchemaError {
        ApplySchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ApplySchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ApplySchemaError {
    fn description(&self) -> &str {
        match *self {
            ApplySchemaError::AccessDenied(ref cause) => cause,
            ApplySchemaError::InternalService(ref cause) => cause,
            ApplySchemaError::InvalidArn(ref cause) => cause,
            ApplySchemaError::InvalidAttachment(ref cause) => cause,
            ApplySchemaError::LimitExceeded(ref cause) => cause,
            ApplySchemaError::ResourceNotFound(ref cause) => cause,
            ApplySchemaError::RetryableConflict(ref cause) => cause,
            ApplySchemaError::Validation(ref cause) => cause,
            ApplySchemaError::Credentials(ref err) => err.description(),
            ApplySchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ApplySchemaError::ParseError(ref cause) => cause,
            ApplySchemaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AttachObject
#[derive(Debug, PartialEq)]
pub enum AttachObjectError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that an attempt to attach an object with the same link name or to apply a schema with the same name has occurred. Rename the link or the schema and then try again.</p>
    InvalidAttachment(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AttachObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> AttachObjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return AttachObjectError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return AttachObjectError::DirectoryNotEnabled(String::from(error_message))
                }
                "FacetValidationException" => {
                    return AttachObjectError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return AttachObjectError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return AttachObjectError::InvalidArn(String::from(error_message))
                }
                "InvalidAttachmentException" => {
                    return AttachObjectError::InvalidAttachment(String::from(error_message))
                }
                "LimitExceededException" => {
                    return AttachObjectError::LimitExceeded(String::from(error_message))
                }
                "LinkNameAlreadyInUseException" => {
                    return AttachObjectError::LinkNameAlreadyInUse(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return AttachObjectError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return AttachObjectError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return AttachObjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AttachObjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AttachObjectError {
    fn from(err: serde_json::error::Error) -> AttachObjectError {
        AttachObjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachObjectError {
    fn from(err: CredentialsError) -> AttachObjectError {
        AttachObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachObjectError {
    fn from(err: HttpDispatchError) -> AttachObjectError {
        AttachObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachObjectError {
    fn from(err: io::Error) -> AttachObjectError {
        AttachObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachObjectError {
    fn description(&self) -> &str {
        match *self {
            AttachObjectError::AccessDenied(ref cause) => cause,
            AttachObjectError::DirectoryNotEnabled(ref cause) => cause,
            AttachObjectError::FacetValidation(ref cause) => cause,
            AttachObjectError::InternalService(ref cause) => cause,
            AttachObjectError::InvalidArn(ref cause) => cause,
            AttachObjectError::InvalidAttachment(ref cause) => cause,
            AttachObjectError::LimitExceeded(ref cause) => cause,
            AttachObjectError::LinkNameAlreadyInUse(ref cause) => cause,
            AttachObjectError::ResourceNotFound(ref cause) => cause,
            AttachObjectError::RetryableConflict(ref cause) => cause,
            AttachObjectError::Validation(ref cause) => cause,
            AttachObjectError::Credentials(ref err) => err.description(),
            AttachObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachObjectError::ParseError(ref cause) => cause,
            AttachObjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AttachPolicy
#[derive(Debug, PartialEq)]
pub enum AttachPolicyError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation can only operate on policy objects.</p>
    NotPolicy(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AttachPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> AttachPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return AttachPolicyError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return AttachPolicyError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return AttachPolicyError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return AttachPolicyError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return AttachPolicyError::LimitExceeded(String::from(error_message))
                }
                "NotPolicyException" => {
                    return AttachPolicyError::NotPolicy(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return AttachPolicyError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return AttachPolicyError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return AttachPolicyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AttachPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AttachPolicyError {
    fn from(err: serde_json::error::Error) -> AttachPolicyError {
        AttachPolicyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachPolicyError {
    fn from(err: CredentialsError) -> AttachPolicyError {
        AttachPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachPolicyError {
    fn from(err: HttpDispatchError) -> AttachPolicyError {
        AttachPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachPolicyError {
    fn from(err: io::Error) -> AttachPolicyError {
        AttachPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachPolicyError {
    fn description(&self) -> &str {
        match *self {
            AttachPolicyError::AccessDenied(ref cause) => cause,
            AttachPolicyError::DirectoryNotEnabled(ref cause) => cause,
            AttachPolicyError::InternalService(ref cause) => cause,
            AttachPolicyError::InvalidArn(ref cause) => cause,
            AttachPolicyError::LimitExceeded(ref cause) => cause,
            AttachPolicyError::NotPolicy(ref cause) => cause,
            AttachPolicyError::ResourceNotFound(ref cause) => cause,
            AttachPolicyError::RetryableConflict(ref cause) => cause,
            AttachPolicyError::Validation(ref cause) => cause,
            AttachPolicyError::Credentials(ref err) => err.description(),
            AttachPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachPolicyError::ParseError(ref cause) => cause,
            AttachPolicyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AttachToIndex
#[derive(Debug, PartialEq)]
pub enum AttachToIndexError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>An object has been attempted to be attached to an object that does not have the appropriate attribute value.</p>
    IndexedAttributeMissing(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that an attempt to attach an object with the same link name or to apply a schema with the same name has occurred. Rename the link or the schema and then try again.</p>
    InvalidAttachment(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    /// <p>Indicates that the requested operation can only operate on index objects.</p>
    NotIndex(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AttachToIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> AttachToIndexError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return AttachToIndexError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return AttachToIndexError::DirectoryNotEnabled(String::from(error_message))
                }
                "IndexedAttributeMissingException" => {
                    return AttachToIndexError::IndexedAttributeMissing(String::from(error_message))
                }
                "InternalServiceException" => {
                    return AttachToIndexError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return AttachToIndexError::InvalidArn(String::from(error_message))
                }
                "InvalidAttachmentException" => {
                    return AttachToIndexError::InvalidAttachment(String::from(error_message))
                }
                "LimitExceededException" => {
                    return AttachToIndexError::LimitExceeded(String::from(error_message))
                }
                "LinkNameAlreadyInUseException" => {
                    return AttachToIndexError::LinkNameAlreadyInUse(String::from(error_message))
                }
                "NotIndexException" => {
                    return AttachToIndexError::NotIndex(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return AttachToIndexError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return AttachToIndexError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return AttachToIndexError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AttachToIndexError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AttachToIndexError {
    fn from(err: serde_json::error::Error) -> AttachToIndexError {
        AttachToIndexError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachToIndexError {
    fn from(err: CredentialsError) -> AttachToIndexError {
        AttachToIndexError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachToIndexError {
    fn from(err: HttpDispatchError) -> AttachToIndexError {
        AttachToIndexError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachToIndexError {
    fn from(err: io::Error) -> AttachToIndexError {
        AttachToIndexError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachToIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachToIndexError {
    fn description(&self) -> &str {
        match *self {
            AttachToIndexError::AccessDenied(ref cause) => cause,
            AttachToIndexError::DirectoryNotEnabled(ref cause) => cause,
            AttachToIndexError::IndexedAttributeMissing(ref cause) => cause,
            AttachToIndexError::InternalService(ref cause) => cause,
            AttachToIndexError::InvalidArn(ref cause) => cause,
            AttachToIndexError::InvalidAttachment(ref cause) => cause,
            AttachToIndexError::LimitExceeded(ref cause) => cause,
            AttachToIndexError::LinkNameAlreadyInUse(ref cause) => cause,
            AttachToIndexError::NotIndex(ref cause) => cause,
            AttachToIndexError::ResourceNotFound(ref cause) => cause,
            AttachToIndexError::RetryableConflict(ref cause) => cause,
            AttachToIndexError::Validation(ref cause) => cause,
            AttachToIndexError::Credentials(ref err) => err.description(),
            AttachToIndexError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachToIndexError::ParseError(ref cause) => cause,
            AttachToIndexError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AttachTypedLink
#[derive(Debug, PartialEq)]
pub enum AttachTypedLinkError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that an attempt to attach an object with the same link name or to apply a schema with the same name has occurred. Rename the link or the schema and then try again.</p>
    InvalidAttachment(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AttachTypedLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> AttachTypedLinkError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return AttachTypedLinkError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return AttachTypedLinkError::DirectoryNotEnabled(String::from(error_message))
                }
                "FacetValidationException" => {
                    return AttachTypedLinkError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return AttachTypedLinkError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return AttachTypedLinkError::InvalidArn(String::from(error_message))
                }
                "InvalidAttachmentException" => {
                    return AttachTypedLinkError::InvalidAttachment(String::from(error_message))
                }
                "LimitExceededException" => {
                    return AttachTypedLinkError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return AttachTypedLinkError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return AttachTypedLinkError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return AttachTypedLinkError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AttachTypedLinkError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AttachTypedLinkError {
    fn from(err: serde_json::error::Error) -> AttachTypedLinkError {
        AttachTypedLinkError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachTypedLinkError {
    fn from(err: CredentialsError) -> AttachTypedLinkError {
        AttachTypedLinkError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachTypedLinkError {
    fn from(err: HttpDispatchError) -> AttachTypedLinkError {
        AttachTypedLinkError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachTypedLinkError {
    fn from(err: io::Error) -> AttachTypedLinkError {
        AttachTypedLinkError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachTypedLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachTypedLinkError {
    fn description(&self) -> &str {
        match *self {
            AttachTypedLinkError::AccessDenied(ref cause) => cause,
            AttachTypedLinkError::DirectoryNotEnabled(ref cause) => cause,
            AttachTypedLinkError::FacetValidation(ref cause) => cause,
            AttachTypedLinkError::InternalService(ref cause) => cause,
            AttachTypedLinkError::InvalidArn(ref cause) => cause,
            AttachTypedLinkError::InvalidAttachment(ref cause) => cause,
            AttachTypedLinkError::LimitExceeded(ref cause) => cause,
            AttachTypedLinkError::ResourceNotFound(ref cause) => cause,
            AttachTypedLinkError::RetryableConflict(ref cause) => cause,
            AttachTypedLinkError::Validation(ref cause) => cause,
            AttachTypedLinkError::Credentials(ref err) => err.description(),
            AttachTypedLinkError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachTypedLinkError::ParseError(ref cause) => cause,
            AttachTypedLinkError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchRead
#[derive(Debug, PartialEq)]
pub enum BatchReadError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl BatchReadError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchReadError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return BatchReadError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return BatchReadError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return BatchReadError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return BatchReadError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return BatchReadError::LimitExceeded(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return BatchReadError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return BatchReadError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchReadError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchReadError {
    fn from(err: serde_json::error::Error) -> BatchReadError {
        BatchReadError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchReadError {
    fn from(err: CredentialsError) -> BatchReadError {
        BatchReadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchReadError {
    fn from(err: HttpDispatchError) -> BatchReadError {
        BatchReadError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchReadError {
    fn from(err: io::Error) -> BatchReadError {
        BatchReadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchReadError {
    fn description(&self) -> &str {
        match *self {
            BatchReadError::AccessDenied(ref cause) => cause,
            BatchReadError::DirectoryNotEnabled(ref cause) => cause,
            BatchReadError::InternalService(ref cause) => cause,
            BatchReadError::InvalidArn(ref cause) => cause,
            BatchReadError::LimitExceeded(ref cause) => cause,
            BatchReadError::RetryableConflict(ref cause) => cause,
            BatchReadError::Validation(ref cause) => cause,
            BatchReadError::Credentials(ref err) => err.description(),
            BatchReadError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchReadError::ParseError(ref cause) => cause,
            BatchReadError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchWrite
#[derive(Debug, PartialEq)]
pub enum BatchWriteError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>A <code>BatchWrite</code> exception has occurred.</p>
    BatchWrite(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl BatchWriteError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchWriteError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return BatchWriteError::AccessDenied(String::from(error_message))
                }
                "BatchWriteException" => {
                    return BatchWriteError::BatchWrite(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return BatchWriteError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return BatchWriteError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return BatchWriteError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return BatchWriteError::LimitExceeded(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return BatchWriteError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return BatchWriteError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchWriteError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchWriteError {
    fn from(err: serde_json::error::Error) -> BatchWriteError {
        BatchWriteError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchWriteError {
    fn from(err: CredentialsError) -> BatchWriteError {
        BatchWriteError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchWriteError {
    fn from(err: HttpDispatchError) -> BatchWriteError {
        BatchWriteError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchWriteError {
    fn from(err: io::Error) -> BatchWriteError {
        BatchWriteError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchWriteError {
    fn description(&self) -> &str {
        match *self {
            BatchWriteError::AccessDenied(ref cause) => cause,
            BatchWriteError::BatchWrite(ref cause) => cause,
            BatchWriteError::DirectoryNotEnabled(ref cause) => cause,
            BatchWriteError::InternalService(ref cause) => cause,
            BatchWriteError::InvalidArn(ref cause) => cause,
            BatchWriteError::LimitExceeded(ref cause) => cause,
            BatchWriteError::RetryableConflict(ref cause) => cause,
            BatchWriteError::Validation(ref cause) => cause,
            BatchWriteError::Credentials(ref err) => err.description(),
            BatchWriteError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchWriteError::ParseError(ref cause) => cause,
            BatchWriteError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDirectory
#[derive(Debug, PartialEq)]
pub enum CreateDirectoryError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates that a <a>Directory</a> could not be created due to a naming conflict. Choose a different name and try again.</p>
    DirectoryAlreadyExists(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDirectoryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return CreateDirectoryError::AccessDenied(String::from(error_message))
                }
                "DirectoryAlreadyExistsException" => {
                    return CreateDirectoryError::DirectoryAlreadyExists(String::from(error_message))
                }
                "InternalServiceException" => {
                    return CreateDirectoryError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return CreateDirectoryError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateDirectoryError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return CreateDirectoryError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return CreateDirectoryError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateDirectoryError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateDirectoryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDirectoryError {
    fn from(err: serde_json::error::Error) -> CreateDirectoryError {
        CreateDirectoryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDirectoryError {
    fn from(err: CredentialsError) -> CreateDirectoryError {
        CreateDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDirectoryError {
    fn from(err: HttpDispatchError) -> CreateDirectoryError {
        CreateDirectoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDirectoryError {
    fn from(err: io::Error) -> CreateDirectoryError {
        CreateDirectoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDirectoryError {
    fn description(&self) -> &str {
        match *self {
            CreateDirectoryError::AccessDenied(ref cause) => cause,
            CreateDirectoryError::DirectoryAlreadyExists(ref cause) => cause,
            CreateDirectoryError::InternalService(ref cause) => cause,
            CreateDirectoryError::InvalidArn(ref cause) => cause,
            CreateDirectoryError::LimitExceeded(ref cause) => cause,
            CreateDirectoryError::ResourceNotFound(ref cause) => cause,
            CreateDirectoryError::RetryableConflict(ref cause) => cause,
            CreateDirectoryError::Validation(ref cause) => cause,
            CreateDirectoryError::Credentials(ref err) => err.description(),
            CreateDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDirectoryError::ParseError(ref cause) => cause,
            CreateDirectoryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateFacet
#[derive(Debug, PartialEq)]
pub enum CreateFacetError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>A facet with the same name already exists.</p>
    FacetAlreadyExists(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Occurs when any of the rule parameter keys or values are invalid.</p>
    InvalidRule(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateFacetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return CreateFacetError::AccessDenied(String::from(error_message))
                }
                "FacetAlreadyExistsException" => {
                    return CreateFacetError::FacetAlreadyExists(String::from(error_message))
                }
                "FacetValidationException" => {
                    return CreateFacetError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return CreateFacetError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return CreateFacetError::InvalidArn(String::from(error_message))
                }
                "InvalidRuleException" => {
                    return CreateFacetError::InvalidRule(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateFacetError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return CreateFacetError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return CreateFacetError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateFacetError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateFacetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateFacetError {
    fn from(err: serde_json::error::Error) -> CreateFacetError {
        CreateFacetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateFacetError {
    fn from(err: CredentialsError) -> CreateFacetError {
        CreateFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateFacetError {
    fn from(err: HttpDispatchError) -> CreateFacetError {
        CreateFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateFacetError {
    fn from(err: io::Error) -> CreateFacetError {
        CreateFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFacetError {
    fn description(&self) -> &str {
        match *self {
            CreateFacetError::AccessDenied(ref cause) => cause,
            CreateFacetError::FacetAlreadyExists(ref cause) => cause,
            CreateFacetError::FacetValidation(ref cause) => cause,
            CreateFacetError::InternalService(ref cause) => cause,
            CreateFacetError::InvalidArn(ref cause) => cause,
            CreateFacetError::InvalidRule(ref cause) => cause,
            CreateFacetError::LimitExceeded(ref cause) => cause,
            CreateFacetError::ResourceNotFound(ref cause) => cause,
            CreateFacetError::RetryableConflict(ref cause) => cause,
            CreateFacetError::Validation(ref cause) => cause,
            CreateFacetError::Credentials(ref err) => err.description(),
            CreateFacetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateFacetError::ParseError(ref cause) => cause,
            CreateFacetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateIndex
#[derive(Debug, PartialEq)]
pub enum CreateIndexError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>Indicates that the requested index type is not supported.</p>
    UnsupportedIndexType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateIndexError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return CreateIndexError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return CreateIndexError::DirectoryNotEnabled(String::from(error_message))
                }
                "FacetValidationException" => {
                    return CreateIndexError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return CreateIndexError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return CreateIndexError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateIndexError::LimitExceeded(String::from(error_message))
                }
                "LinkNameAlreadyInUseException" => {
                    return CreateIndexError::LinkNameAlreadyInUse(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return CreateIndexError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return CreateIndexError::RetryableConflict(String::from(error_message))
                }
                "UnsupportedIndexTypeException" => {
                    return CreateIndexError::UnsupportedIndexType(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateIndexError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateIndexError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateIndexError {
    fn from(err: serde_json::error::Error) -> CreateIndexError {
        CreateIndexError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateIndexError {
    fn from(err: CredentialsError) -> CreateIndexError {
        CreateIndexError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateIndexError {
    fn from(err: HttpDispatchError) -> CreateIndexError {
        CreateIndexError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateIndexError {
    fn from(err: io::Error) -> CreateIndexError {
        CreateIndexError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIndexError {
    fn description(&self) -> &str {
        match *self {
            CreateIndexError::AccessDenied(ref cause) => cause,
            CreateIndexError::DirectoryNotEnabled(ref cause) => cause,
            CreateIndexError::FacetValidation(ref cause) => cause,
            CreateIndexError::InternalService(ref cause) => cause,
            CreateIndexError::InvalidArn(ref cause) => cause,
            CreateIndexError::LimitExceeded(ref cause) => cause,
            CreateIndexError::LinkNameAlreadyInUse(ref cause) => cause,
            CreateIndexError::ResourceNotFound(ref cause) => cause,
            CreateIndexError::RetryableConflict(ref cause) => cause,
            CreateIndexError::UnsupportedIndexType(ref cause) => cause,
            CreateIndexError::Validation(ref cause) => cause,
            CreateIndexError::Credentials(ref err) => err.description(),
            CreateIndexError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateIndexError::ParseError(ref cause) => cause,
            CreateIndexError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateObject
#[derive(Debug, PartialEq)]
pub enum CreateObjectError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>Indicates that the requested index type is not supported.</p>
    UnsupportedIndexType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateObjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return CreateObjectError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return CreateObjectError::DirectoryNotEnabled(String::from(error_message))
                }
                "FacetValidationException" => {
                    return CreateObjectError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return CreateObjectError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return CreateObjectError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateObjectError::LimitExceeded(String::from(error_message))
                }
                "LinkNameAlreadyInUseException" => {
                    return CreateObjectError::LinkNameAlreadyInUse(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return CreateObjectError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return CreateObjectError::RetryableConflict(String::from(error_message))
                }
                "UnsupportedIndexTypeException" => {
                    return CreateObjectError::UnsupportedIndexType(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateObjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateObjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateObjectError {
    fn from(err: serde_json::error::Error) -> CreateObjectError {
        CreateObjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateObjectError {
    fn from(err: CredentialsError) -> CreateObjectError {
        CreateObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateObjectError {
    fn from(err: HttpDispatchError) -> CreateObjectError {
        CreateObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateObjectError {
    fn from(err: io::Error) -> CreateObjectError {
        CreateObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateObjectError {
    fn description(&self) -> &str {
        match *self {
            CreateObjectError::AccessDenied(ref cause) => cause,
            CreateObjectError::DirectoryNotEnabled(ref cause) => cause,
            CreateObjectError::FacetValidation(ref cause) => cause,
            CreateObjectError::InternalService(ref cause) => cause,
            CreateObjectError::InvalidArn(ref cause) => cause,
            CreateObjectError::LimitExceeded(ref cause) => cause,
            CreateObjectError::LinkNameAlreadyInUse(ref cause) => cause,
            CreateObjectError::ResourceNotFound(ref cause) => cause,
            CreateObjectError::RetryableConflict(ref cause) => cause,
            CreateObjectError::UnsupportedIndexType(ref cause) => cause,
            CreateObjectError::Validation(ref cause) => cause,
            CreateObjectError::Credentials(ref err) => err.description(),
            CreateObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateObjectError::ParseError(ref cause) => cause,
            CreateObjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateSchema
#[derive(Debug, PartialEq)]
pub enum CreateSchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>Indicates that a schema could not be created due to a naming conflict. Please select a different name and then try again.</p>
    SchemaAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateSchemaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return CreateSchemaError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return CreateSchemaError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return CreateSchemaError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateSchemaError::LimitExceeded(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return CreateSchemaError::RetryableConflict(String::from(error_message))
                }
                "SchemaAlreadyExistsException" => {
                    return CreateSchemaError::SchemaAlreadyExists(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateSchemaError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateSchemaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateSchemaError {
    fn from(err: serde_json::error::Error) -> CreateSchemaError {
        CreateSchemaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSchemaError {
    fn from(err: CredentialsError) -> CreateSchemaError {
        CreateSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSchemaError {
    fn from(err: HttpDispatchError) -> CreateSchemaError {
        CreateSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSchemaError {
    fn from(err: io::Error) -> CreateSchemaError {
        CreateSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSchemaError {
    fn description(&self) -> &str {
        match *self {
            CreateSchemaError::AccessDenied(ref cause) => cause,
            CreateSchemaError::InternalService(ref cause) => cause,
            CreateSchemaError::InvalidArn(ref cause) => cause,
            CreateSchemaError::LimitExceeded(ref cause) => cause,
            CreateSchemaError::RetryableConflict(ref cause) => cause,
            CreateSchemaError::SchemaAlreadyExists(ref cause) => cause,
            CreateSchemaError::Validation(ref cause) => cause,
            CreateSchemaError::Credentials(ref err) => err.description(),
            CreateSchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSchemaError::ParseError(ref cause) => cause,
            CreateSchemaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateTypedLinkFacet
#[derive(Debug, PartialEq)]
pub enum CreateTypedLinkFacetError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>A facet with the same name already exists.</p>
    FacetAlreadyExists(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Occurs when any of the rule parameter keys or values are invalid.</p>
    InvalidRule(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateTypedLinkFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateTypedLinkFacetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return CreateTypedLinkFacetError::AccessDenied(String::from(error_message))
                }
                "FacetAlreadyExistsException" => {
                    return CreateTypedLinkFacetError::FacetAlreadyExists(String::from(
                        error_message,
                    ))
                }
                "FacetValidationException" => {
                    return CreateTypedLinkFacetError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return CreateTypedLinkFacetError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return CreateTypedLinkFacetError::InvalidArn(String::from(error_message))
                }
                "InvalidRuleException" => {
                    return CreateTypedLinkFacetError::InvalidRule(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateTypedLinkFacetError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return CreateTypedLinkFacetError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return CreateTypedLinkFacetError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateTypedLinkFacetError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateTypedLinkFacetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateTypedLinkFacetError {
    fn from(err: serde_json::error::Error) -> CreateTypedLinkFacetError {
        CreateTypedLinkFacetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTypedLinkFacetError {
    fn from(err: CredentialsError) -> CreateTypedLinkFacetError {
        CreateTypedLinkFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTypedLinkFacetError {
    fn from(err: HttpDispatchError) -> CreateTypedLinkFacetError {
        CreateTypedLinkFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTypedLinkFacetError {
    fn from(err: io::Error) -> CreateTypedLinkFacetError {
        CreateTypedLinkFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTypedLinkFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTypedLinkFacetError {
    fn description(&self) -> &str {
        match *self {
            CreateTypedLinkFacetError::AccessDenied(ref cause) => cause,
            CreateTypedLinkFacetError::FacetAlreadyExists(ref cause) => cause,
            CreateTypedLinkFacetError::FacetValidation(ref cause) => cause,
            CreateTypedLinkFacetError::InternalService(ref cause) => cause,
            CreateTypedLinkFacetError::InvalidArn(ref cause) => cause,
            CreateTypedLinkFacetError::InvalidRule(ref cause) => cause,
            CreateTypedLinkFacetError::LimitExceeded(ref cause) => cause,
            CreateTypedLinkFacetError::ResourceNotFound(ref cause) => cause,
            CreateTypedLinkFacetError::RetryableConflict(ref cause) => cause,
            CreateTypedLinkFacetError::Validation(ref cause) => cause,
            CreateTypedLinkFacetError::Credentials(ref err) => err.description(),
            CreateTypedLinkFacetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateTypedLinkFacetError::ParseError(ref cause) => cause,
            CreateTypedLinkFacetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDirectory
#[derive(Debug, PartialEq)]
pub enum DeleteDirectoryError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>A directory that has been deleted and to which access has been attempted. Note: The requested resource will eventually cease to exist.</p>
    DirectoryDeleted(String),
    /// <p>An operation can only operate on a disabled directory.</p>
    DirectoryNotDisabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDirectoryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DeleteDirectoryError::AccessDenied(String::from(error_message))
                }
                "DirectoryDeletedException" => {
                    return DeleteDirectoryError::DirectoryDeleted(String::from(error_message))
                }
                "DirectoryNotDisabledException" => {
                    return DeleteDirectoryError::DirectoryNotDisabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DeleteDirectoryError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return DeleteDirectoryError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteDirectoryError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DeleteDirectoryError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return DeleteDirectoryError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteDirectoryError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteDirectoryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDirectoryError {
    fn from(err: serde_json::error::Error) -> DeleteDirectoryError {
        DeleteDirectoryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDirectoryError {
    fn from(err: CredentialsError) -> DeleteDirectoryError {
        DeleteDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDirectoryError {
    fn from(err: HttpDispatchError) -> DeleteDirectoryError {
        DeleteDirectoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDirectoryError {
    fn from(err: io::Error) -> DeleteDirectoryError {
        DeleteDirectoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDirectoryError {
    fn description(&self) -> &str {
        match *self {
            DeleteDirectoryError::AccessDenied(ref cause) => cause,
            DeleteDirectoryError::DirectoryDeleted(ref cause) => cause,
            DeleteDirectoryError::DirectoryNotDisabled(ref cause) => cause,
            DeleteDirectoryError::InternalService(ref cause) => cause,
            DeleteDirectoryError::InvalidArn(ref cause) => cause,
            DeleteDirectoryError::LimitExceeded(ref cause) => cause,
            DeleteDirectoryError::ResourceNotFound(ref cause) => cause,
            DeleteDirectoryError::RetryableConflict(ref cause) => cause,
            DeleteDirectoryError::Validation(ref cause) => cause,
            DeleteDirectoryError::Credentials(ref err) => err.description(),
            DeleteDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDirectoryError::ParseError(ref cause) => cause,
            DeleteDirectoryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteFacet
#[derive(Debug, PartialEq)]
pub enum DeleteFacetError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Occurs when deleting a facet that contains an attribute that is a target to an attribute reference in a different facet.</p>
    FacetInUse(String),
    /// <p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteFacetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DeleteFacetError::AccessDenied(String::from(error_message))
                }
                "FacetInUseException" => {
                    return DeleteFacetError::FacetInUse(String::from(error_message))
                }
                "FacetNotFoundException" => {
                    return DeleteFacetError::FacetNotFound(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DeleteFacetError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return DeleteFacetError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteFacetError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DeleteFacetError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return DeleteFacetError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteFacetError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteFacetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteFacetError {
    fn from(err: serde_json::error::Error) -> DeleteFacetError {
        DeleteFacetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFacetError {
    fn from(err: CredentialsError) -> DeleteFacetError {
        DeleteFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFacetError {
    fn from(err: HttpDispatchError) -> DeleteFacetError {
        DeleteFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFacetError {
    fn from(err: io::Error) -> DeleteFacetError {
        DeleteFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFacetError {
    fn description(&self) -> &str {
        match *self {
            DeleteFacetError::AccessDenied(ref cause) => cause,
            DeleteFacetError::FacetInUse(ref cause) => cause,
            DeleteFacetError::FacetNotFound(ref cause) => cause,
            DeleteFacetError::InternalService(ref cause) => cause,
            DeleteFacetError::InvalidArn(ref cause) => cause,
            DeleteFacetError::LimitExceeded(ref cause) => cause,
            DeleteFacetError::ResourceNotFound(ref cause) => cause,
            DeleteFacetError::RetryableConflict(ref cause) => cause,
            DeleteFacetError::Validation(ref cause) => cause,
            DeleteFacetError::Credentials(ref err) => err.description(),
            DeleteFacetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFacetError::ParseError(ref cause) => cause,
            DeleteFacetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteObject
#[derive(Debug, PartialEq)]
pub enum DeleteObjectError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation cannot be completed because the object has not been detached from the tree.</p>
    ObjectNotDetached(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteObjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DeleteObjectError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return DeleteObjectError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DeleteObjectError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return DeleteObjectError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteObjectError::LimitExceeded(String::from(error_message))
                }
                "ObjectNotDetachedException" => {
                    return DeleteObjectError::ObjectNotDetached(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DeleteObjectError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return DeleteObjectError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteObjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteObjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteObjectError {
    fn from(err: serde_json::error::Error) -> DeleteObjectError {
        DeleteObjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteObjectError {
    fn from(err: CredentialsError) -> DeleteObjectError {
        DeleteObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteObjectError {
    fn from(err: HttpDispatchError) -> DeleteObjectError {
        DeleteObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteObjectError {
    fn from(err: io::Error) -> DeleteObjectError {
        DeleteObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectError {
    fn description(&self) -> &str {
        match *self {
            DeleteObjectError::AccessDenied(ref cause) => cause,
            DeleteObjectError::DirectoryNotEnabled(ref cause) => cause,
            DeleteObjectError::InternalService(ref cause) => cause,
            DeleteObjectError::InvalidArn(ref cause) => cause,
            DeleteObjectError::LimitExceeded(ref cause) => cause,
            DeleteObjectError::ObjectNotDetached(ref cause) => cause,
            DeleteObjectError::ResourceNotFound(ref cause) => cause,
            DeleteObjectError::RetryableConflict(ref cause) => cause,
            DeleteObjectError::Validation(ref cause) => cause,
            DeleteObjectError::Credentials(ref err) => err.description(),
            DeleteObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteObjectError::ParseError(ref cause) => cause,
            DeleteObjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteSchema
#[derive(Debug, PartialEq)]
pub enum DeleteSchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>The object could not be deleted because links still exist. Remove the links and then try the operation again.</p>
    StillContainsLinks(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteSchemaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DeleteSchemaError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DeleteSchemaError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return DeleteSchemaError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteSchemaError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DeleteSchemaError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return DeleteSchemaError::RetryableConflict(String::from(error_message))
                }
                "StillContainsLinksException" => {
                    return DeleteSchemaError::StillContainsLinks(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteSchemaError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteSchemaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteSchemaError {
    fn from(err: serde_json::error::Error) -> DeleteSchemaError {
        DeleteSchemaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSchemaError {
    fn from(err: CredentialsError) -> DeleteSchemaError {
        DeleteSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSchemaError {
    fn from(err: HttpDispatchError) -> DeleteSchemaError {
        DeleteSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSchemaError {
    fn from(err: io::Error) -> DeleteSchemaError {
        DeleteSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSchemaError {
    fn description(&self) -> &str {
        match *self {
            DeleteSchemaError::AccessDenied(ref cause) => cause,
            DeleteSchemaError::InternalService(ref cause) => cause,
            DeleteSchemaError::InvalidArn(ref cause) => cause,
            DeleteSchemaError::LimitExceeded(ref cause) => cause,
            DeleteSchemaError::ResourceNotFound(ref cause) => cause,
            DeleteSchemaError::RetryableConflict(ref cause) => cause,
            DeleteSchemaError::StillContainsLinks(ref cause) => cause,
            DeleteSchemaError::Validation(ref cause) => cause,
            DeleteSchemaError::Credentials(ref err) => err.description(),
            DeleteSchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSchemaError::ParseError(ref cause) => cause,
            DeleteSchemaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteTypedLinkFacet
#[derive(Debug, PartialEq)]
pub enum DeleteTypedLinkFacetError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteTypedLinkFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteTypedLinkFacetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DeleteTypedLinkFacetError::AccessDenied(String::from(error_message))
                }
                "FacetNotFoundException" => {
                    return DeleteTypedLinkFacetError::FacetNotFound(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DeleteTypedLinkFacetError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return DeleteTypedLinkFacetError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteTypedLinkFacetError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DeleteTypedLinkFacetError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return DeleteTypedLinkFacetError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteTypedLinkFacetError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteTypedLinkFacetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteTypedLinkFacetError {
    fn from(err: serde_json::error::Error) -> DeleteTypedLinkFacetError {
        DeleteTypedLinkFacetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTypedLinkFacetError {
    fn from(err: CredentialsError) -> DeleteTypedLinkFacetError {
        DeleteTypedLinkFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTypedLinkFacetError {
    fn from(err: HttpDispatchError) -> DeleteTypedLinkFacetError {
        DeleteTypedLinkFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTypedLinkFacetError {
    fn from(err: io::Error) -> DeleteTypedLinkFacetError {
        DeleteTypedLinkFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTypedLinkFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTypedLinkFacetError {
    fn description(&self) -> &str {
        match *self {
            DeleteTypedLinkFacetError::AccessDenied(ref cause) => cause,
            DeleteTypedLinkFacetError::FacetNotFound(ref cause) => cause,
            DeleteTypedLinkFacetError::InternalService(ref cause) => cause,
            DeleteTypedLinkFacetError::InvalidArn(ref cause) => cause,
            DeleteTypedLinkFacetError::LimitExceeded(ref cause) => cause,
            DeleteTypedLinkFacetError::ResourceNotFound(ref cause) => cause,
            DeleteTypedLinkFacetError::RetryableConflict(ref cause) => cause,
            DeleteTypedLinkFacetError::Validation(ref cause) => cause,
            DeleteTypedLinkFacetError::Credentials(ref err) => err.description(),
            DeleteTypedLinkFacetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteTypedLinkFacetError::ParseError(ref cause) => cause,
            DeleteTypedLinkFacetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetachFromIndex
#[derive(Debug, PartialEq)]
pub enum DetachFromIndexError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation can only operate on index objects.</p>
    NotIndex(String),
    /// <p>Indicates that the object is not attached to the index.</p>
    ObjectAlreadyDetached(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DetachFromIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> DetachFromIndexError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DetachFromIndexError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return DetachFromIndexError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DetachFromIndexError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return DetachFromIndexError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DetachFromIndexError::LimitExceeded(String::from(error_message))
                }
                "NotIndexException" => {
                    return DetachFromIndexError::NotIndex(String::from(error_message))
                }
                "ObjectAlreadyDetachedException" => {
                    return DetachFromIndexError::ObjectAlreadyDetached(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DetachFromIndexError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return DetachFromIndexError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return DetachFromIndexError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetachFromIndexError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetachFromIndexError {
    fn from(err: serde_json::error::Error) -> DetachFromIndexError {
        DetachFromIndexError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachFromIndexError {
    fn from(err: CredentialsError) -> DetachFromIndexError {
        DetachFromIndexError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachFromIndexError {
    fn from(err: HttpDispatchError) -> DetachFromIndexError {
        DetachFromIndexError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachFromIndexError {
    fn from(err: io::Error) -> DetachFromIndexError {
        DetachFromIndexError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachFromIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachFromIndexError {
    fn description(&self) -> &str {
        match *self {
            DetachFromIndexError::AccessDenied(ref cause) => cause,
            DetachFromIndexError::DirectoryNotEnabled(ref cause) => cause,
            DetachFromIndexError::InternalService(ref cause) => cause,
            DetachFromIndexError::InvalidArn(ref cause) => cause,
            DetachFromIndexError::LimitExceeded(ref cause) => cause,
            DetachFromIndexError::NotIndex(ref cause) => cause,
            DetachFromIndexError::ObjectAlreadyDetached(ref cause) => cause,
            DetachFromIndexError::ResourceNotFound(ref cause) => cause,
            DetachFromIndexError::RetryableConflict(ref cause) => cause,
            DetachFromIndexError::Validation(ref cause) => cause,
            DetachFromIndexError::Credentials(ref err) => err.description(),
            DetachFromIndexError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachFromIndexError::ParseError(ref cause) => cause,
            DetachFromIndexError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetachObject
#[derive(Debug, PartialEq)]
pub enum DetachObjectError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when any invalid operations are performed on an object that is not a node, such as calling <code>ListObjectChildren</code> for a leaf node object.</p>
    NotNode(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DetachObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> DetachObjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DetachObjectError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return DetachObjectError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DetachObjectError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return DetachObjectError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DetachObjectError::LimitExceeded(String::from(error_message))
                }
                "NotNodeException" => {
                    return DetachObjectError::NotNode(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DetachObjectError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return DetachObjectError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return DetachObjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetachObjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetachObjectError {
    fn from(err: serde_json::error::Error) -> DetachObjectError {
        DetachObjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachObjectError {
    fn from(err: CredentialsError) -> DetachObjectError {
        DetachObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachObjectError {
    fn from(err: HttpDispatchError) -> DetachObjectError {
        DetachObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachObjectError {
    fn from(err: io::Error) -> DetachObjectError {
        DetachObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachObjectError {
    fn description(&self) -> &str {
        match *self {
            DetachObjectError::AccessDenied(ref cause) => cause,
            DetachObjectError::DirectoryNotEnabled(ref cause) => cause,
            DetachObjectError::InternalService(ref cause) => cause,
            DetachObjectError::InvalidArn(ref cause) => cause,
            DetachObjectError::LimitExceeded(ref cause) => cause,
            DetachObjectError::NotNode(ref cause) => cause,
            DetachObjectError::ResourceNotFound(ref cause) => cause,
            DetachObjectError::RetryableConflict(ref cause) => cause,
            DetachObjectError::Validation(ref cause) => cause,
            DetachObjectError::Credentials(ref err) => err.description(),
            DetachObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachObjectError::ParseError(ref cause) => cause,
            DetachObjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetachPolicy
#[derive(Debug, PartialEq)]
pub enum DetachPolicyError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation can only operate on policy objects.</p>
    NotPolicy(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DetachPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> DetachPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DetachPolicyError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return DetachPolicyError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DetachPolicyError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return DetachPolicyError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DetachPolicyError::LimitExceeded(String::from(error_message))
                }
                "NotPolicyException" => {
                    return DetachPolicyError::NotPolicy(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DetachPolicyError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return DetachPolicyError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return DetachPolicyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetachPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetachPolicyError {
    fn from(err: serde_json::error::Error) -> DetachPolicyError {
        DetachPolicyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachPolicyError {
    fn from(err: CredentialsError) -> DetachPolicyError {
        DetachPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachPolicyError {
    fn from(err: HttpDispatchError) -> DetachPolicyError {
        DetachPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachPolicyError {
    fn from(err: io::Error) -> DetachPolicyError {
        DetachPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachPolicyError {
    fn description(&self) -> &str {
        match *self {
            DetachPolicyError::AccessDenied(ref cause) => cause,
            DetachPolicyError::DirectoryNotEnabled(ref cause) => cause,
            DetachPolicyError::InternalService(ref cause) => cause,
            DetachPolicyError::InvalidArn(ref cause) => cause,
            DetachPolicyError::LimitExceeded(ref cause) => cause,
            DetachPolicyError::NotPolicy(ref cause) => cause,
            DetachPolicyError::ResourceNotFound(ref cause) => cause,
            DetachPolicyError::RetryableConflict(ref cause) => cause,
            DetachPolicyError::Validation(ref cause) => cause,
            DetachPolicyError::Credentials(ref err) => err.description(),
            DetachPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachPolicyError::ParseError(ref cause) => cause,
            DetachPolicyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetachTypedLink
#[derive(Debug, PartialEq)]
pub enum DetachTypedLinkError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DetachTypedLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> DetachTypedLinkError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DetachTypedLinkError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return DetachTypedLinkError::DirectoryNotEnabled(String::from(error_message))
                }
                "FacetValidationException" => {
                    return DetachTypedLinkError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DetachTypedLinkError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return DetachTypedLinkError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DetachTypedLinkError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DetachTypedLinkError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return DetachTypedLinkError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return DetachTypedLinkError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetachTypedLinkError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetachTypedLinkError {
    fn from(err: serde_json::error::Error) -> DetachTypedLinkError {
        DetachTypedLinkError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachTypedLinkError {
    fn from(err: CredentialsError) -> DetachTypedLinkError {
        DetachTypedLinkError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachTypedLinkError {
    fn from(err: HttpDispatchError) -> DetachTypedLinkError {
        DetachTypedLinkError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachTypedLinkError {
    fn from(err: io::Error) -> DetachTypedLinkError {
        DetachTypedLinkError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachTypedLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachTypedLinkError {
    fn description(&self) -> &str {
        match *self {
            DetachTypedLinkError::AccessDenied(ref cause) => cause,
            DetachTypedLinkError::DirectoryNotEnabled(ref cause) => cause,
            DetachTypedLinkError::FacetValidation(ref cause) => cause,
            DetachTypedLinkError::InternalService(ref cause) => cause,
            DetachTypedLinkError::InvalidArn(ref cause) => cause,
            DetachTypedLinkError::LimitExceeded(ref cause) => cause,
            DetachTypedLinkError::ResourceNotFound(ref cause) => cause,
            DetachTypedLinkError::RetryableConflict(ref cause) => cause,
            DetachTypedLinkError::Validation(ref cause) => cause,
            DetachTypedLinkError::Credentials(ref err) => err.description(),
            DetachTypedLinkError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachTypedLinkError::ParseError(ref cause) => cause,
            DetachTypedLinkError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisableDirectory
#[derive(Debug, PartialEq)]
pub enum DisableDirectoryError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>A directory that has been deleted and to which access has been attempted. Note: The requested resource will eventually cease to exist.</p>
    DirectoryDeleted(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisableDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> DisableDirectoryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DisableDirectoryError::AccessDenied(String::from(error_message))
                }
                "DirectoryDeletedException" => {
                    return DisableDirectoryError::DirectoryDeleted(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DisableDirectoryError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return DisableDirectoryError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DisableDirectoryError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DisableDirectoryError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return DisableDirectoryError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return DisableDirectoryError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DisableDirectoryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisableDirectoryError {
    fn from(err: serde_json::error::Error) -> DisableDirectoryError {
        DisableDirectoryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableDirectoryError {
    fn from(err: CredentialsError) -> DisableDirectoryError {
        DisableDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableDirectoryError {
    fn from(err: HttpDispatchError) -> DisableDirectoryError {
        DisableDirectoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableDirectoryError {
    fn from(err: io::Error) -> DisableDirectoryError {
        DisableDirectoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableDirectoryError {
    fn description(&self) -> &str {
        match *self {
            DisableDirectoryError::AccessDenied(ref cause) => cause,
            DisableDirectoryError::DirectoryDeleted(ref cause) => cause,
            DisableDirectoryError::InternalService(ref cause) => cause,
            DisableDirectoryError::InvalidArn(ref cause) => cause,
            DisableDirectoryError::LimitExceeded(ref cause) => cause,
            DisableDirectoryError::ResourceNotFound(ref cause) => cause,
            DisableDirectoryError::RetryableConflict(ref cause) => cause,
            DisableDirectoryError::Validation(ref cause) => cause,
            DisableDirectoryError::Credentials(ref err) => err.description(),
            DisableDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableDirectoryError::ParseError(ref cause) => cause,
            DisableDirectoryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by EnableDirectory
#[derive(Debug, PartialEq)]
pub enum EnableDirectoryError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>A directory that has been deleted and to which access has been attempted. Note: The requested resource will eventually cease to exist.</p>
    DirectoryDeleted(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl EnableDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> EnableDirectoryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return EnableDirectoryError::AccessDenied(String::from(error_message))
                }
                "DirectoryDeletedException" => {
                    return EnableDirectoryError::DirectoryDeleted(String::from(error_message))
                }
                "InternalServiceException" => {
                    return EnableDirectoryError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return EnableDirectoryError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return EnableDirectoryError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return EnableDirectoryError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return EnableDirectoryError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return EnableDirectoryError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return EnableDirectoryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for EnableDirectoryError {
    fn from(err: serde_json::error::Error) -> EnableDirectoryError {
        EnableDirectoryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableDirectoryError {
    fn from(err: CredentialsError) -> EnableDirectoryError {
        EnableDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableDirectoryError {
    fn from(err: HttpDispatchError) -> EnableDirectoryError {
        EnableDirectoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableDirectoryError {
    fn from(err: io::Error) -> EnableDirectoryError {
        EnableDirectoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableDirectoryError {
    fn description(&self) -> &str {
        match *self {
            EnableDirectoryError::AccessDenied(ref cause) => cause,
            EnableDirectoryError::DirectoryDeleted(ref cause) => cause,
            EnableDirectoryError::InternalService(ref cause) => cause,
            EnableDirectoryError::InvalidArn(ref cause) => cause,
            EnableDirectoryError::LimitExceeded(ref cause) => cause,
            EnableDirectoryError::ResourceNotFound(ref cause) => cause,
            EnableDirectoryError::RetryableConflict(ref cause) => cause,
            EnableDirectoryError::Validation(ref cause) => cause,
            EnableDirectoryError::Credentials(ref err) => err.description(),
            EnableDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnableDirectoryError::ParseError(ref cause) => cause,
            EnableDirectoryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetAppliedSchemaVersion
#[derive(Debug, PartialEq)]
pub enum GetAppliedSchemaVersionError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetAppliedSchemaVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> GetAppliedSchemaVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetAppliedSchemaVersionError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return GetAppliedSchemaVersionError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidArnException" => {
                    return GetAppliedSchemaVersionError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetAppliedSchemaVersionError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return GetAppliedSchemaVersionError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "RetryableConflictException" => {
                    return GetAppliedSchemaVersionError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetAppliedSchemaVersionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetAppliedSchemaVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAppliedSchemaVersionError {
    fn from(err: serde_json::error::Error) -> GetAppliedSchemaVersionError {
        GetAppliedSchemaVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAppliedSchemaVersionError {
    fn from(err: CredentialsError) -> GetAppliedSchemaVersionError {
        GetAppliedSchemaVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAppliedSchemaVersionError {
    fn from(err: HttpDispatchError) -> GetAppliedSchemaVersionError {
        GetAppliedSchemaVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAppliedSchemaVersionError {
    fn from(err: io::Error) -> GetAppliedSchemaVersionError {
        GetAppliedSchemaVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAppliedSchemaVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppliedSchemaVersionError {
    fn description(&self) -> &str {
        match *self {
            GetAppliedSchemaVersionError::AccessDenied(ref cause) => cause,
            GetAppliedSchemaVersionError::InternalService(ref cause) => cause,
            GetAppliedSchemaVersionError::InvalidArn(ref cause) => cause,
            GetAppliedSchemaVersionError::LimitExceeded(ref cause) => cause,
            GetAppliedSchemaVersionError::ResourceNotFound(ref cause) => cause,
            GetAppliedSchemaVersionError::RetryableConflict(ref cause) => cause,
            GetAppliedSchemaVersionError::Validation(ref cause) => cause,
            GetAppliedSchemaVersionError::Credentials(ref err) => err.description(),
            GetAppliedSchemaVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAppliedSchemaVersionError::ParseError(ref cause) => cause,
            GetAppliedSchemaVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetDirectory
#[derive(Debug, PartialEq)]
pub enum GetDirectoryError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> GetDirectoryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetDirectoryError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return GetDirectoryError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return GetDirectoryError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetDirectoryError::LimitExceeded(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return GetDirectoryError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return GetDirectoryError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetDirectoryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDirectoryError {
    fn from(err: serde_json::error::Error) -> GetDirectoryError {
        GetDirectoryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDirectoryError {
    fn from(err: CredentialsError) -> GetDirectoryError {
        GetDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDirectoryError {
    fn from(err: HttpDispatchError) -> GetDirectoryError {
        GetDirectoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDirectoryError {
    fn from(err: io::Error) -> GetDirectoryError {
        GetDirectoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDirectoryError {
    fn description(&self) -> &str {
        match *self {
            GetDirectoryError::AccessDenied(ref cause) => cause,
            GetDirectoryError::InternalService(ref cause) => cause,
            GetDirectoryError::InvalidArn(ref cause) => cause,
            GetDirectoryError::LimitExceeded(ref cause) => cause,
            GetDirectoryError::RetryableConflict(ref cause) => cause,
            GetDirectoryError::Validation(ref cause) => cause,
            GetDirectoryError::Credentials(ref err) => err.description(),
            GetDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDirectoryError::ParseError(ref cause) => cause,
            GetDirectoryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetFacet
#[derive(Debug, PartialEq)]
pub enum GetFacetError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> GetFacetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetFacetError::AccessDenied(String::from(error_message))
                }
                "FacetNotFoundException" => {
                    return GetFacetError::FacetNotFound(String::from(error_message))
                }
                "InternalServiceException" => {
                    return GetFacetError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return GetFacetError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetFacetError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return GetFacetError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return GetFacetError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return GetFacetError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetFacetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetFacetError {
    fn from(err: serde_json::error::Error) -> GetFacetError {
        GetFacetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFacetError {
    fn from(err: CredentialsError) -> GetFacetError {
        GetFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFacetError {
    fn from(err: HttpDispatchError) -> GetFacetError {
        GetFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFacetError {
    fn from(err: io::Error) -> GetFacetError {
        GetFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFacetError {
    fn description(&self) -> &str {
        match *self {
            GetFacetError::AccessDenied(ref cause) => cause,
            GetFacetError::FacetNotFound(ref cause) => cause,
            GetFacetError::InternalService(ref cause) => cause,
            GetFacetError::InvalidArn(ref cause) => cause,
            GetFacetError::LimitExceeded(ref cause) => cause,
            GetFacetError::ResourceNotFound(ref cause) => cause,
            GetFacetError::RetryableConflict(ref cause) => cause,
            GetFacetError::Validation(ref cause) => cause,
            GetFacetError::Credentials(ref err) => err.description(),
            GetFacetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetFacetError::ParseError(ref cause) => cause,
            GetFacetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetLinkAttributes
#[derive(Debug, PartialEq)]
pub enum GetLinkAttributesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetLinkAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> GetLinkAttributesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetLinkAttributesError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return GetLinkAttributesError::DirectoryNotEnabled(String::from(error_message))
                }
                "FacetValidationException" => {
                    return GetLinkAttributesError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return GetLinkAttributesError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return GetLinkAttributesError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetLinkAttributesError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return GetLinkAttributesError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return GetLinkAttributesError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return GetLinkAttributesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetLinkAttributesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetLinkAttributesError {
    fn from(err: serde_json::error::Error) -> GetLinkAttributesError {
        GetLinkAttributesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLinkAttributesError {
    fn from(err: CredentialsError) -> GetLinkAttributesError {
        GetLinkAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLinkAttributesError {
    fn from(err: HttpDispatchError) -> GetLinkAttributesError {
        GetLinkAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLinkAttributesError {
    fn from(err: io::Error) -> GetLinkAttributesError {
        GetLinkAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLinkAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLinkAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetLinkAttributesError::AccessDenied(ref cause) => cause,
            GetLinkAttributesError::DirectoryNotEnabled(ref cause) => cause,
            GetLinkAttributesError::FacetValidation(ref cause) => cause,
            GetLinkAttributesError::InternalService(ref cause) => cause,
            GetLinkAttributesError::InvalidArn(ref cause) => cause,
            GetLinkAttributesError::LimitExceeded(ref cause) => cause,
            GetLinkAttributesError::ResourceNotFound(ref cause) => cause,
            GetLinkAttributesError::RetryableConflict(ref cause) => cause,
            GetLinkAttributesError::Validation(ref cause) => cause,
            GetLinkAttributesError::Credentials(ref err) => err.description(),
            GetLinkAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLinkAttributesError::ParseError(ref cause) => cause,
            GetLinkAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetObjectAttributes
#[derive(Debug, PartialEq)]
pub enum GetObjectAttributesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetObjectAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> GetObjectAttributesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetObjectAttributesError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return GetObjectAttributesError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "FacetValidationException" => {
                    return GetObjectAttributesError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return GetObjectAttributesError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return GetObjectAttributesError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetObjectAttributesError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return GetObjectAttributesError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return GetObjectAttributesError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return GetObjectAttributesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetObjectAttributesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetObjectAttributesError {
    fn from(err: serde_json::error::Error) -> GetObjectAttributesError {
        GetObjectAttributesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetObjectAttributesError {
    fn from(err: CredentialsError) -> GetObjectAttributesError {
        GetObjectAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectAttributesError {
    fn from(err: HttpDispatchError) -> GetObjectAttributesError {
        GetObjectAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectAttributesError {
    fn from(err: io::Error) -> GetObjectAttributesError {
        GetObjectAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetObjectAttributesError::AccessDenied(ref cause) => cause,
            GetObjectAttributesError::DirectoryNotEnabled(ref cause) => cause,
            GetObjectAttributesError::FacetValidation(ref cause) => cause,
            GetObjectAttributesError::InternalService(ref cause) => cause,
            GetObjectAttributesError::InvalidArn(ref cause) => cause,
            GetObjectAttributesError::LimitExceeded(ref cause) => cause,
            GetObjectAttributesError::ResourceNotFound(ref cause) => cause,
            GetObjectAttributesError::RetryableConflict(ref cause) => cause,
            GetObjectAttributesError::Validation(ref cause) => cause,
            GetObjectAttributesError::Credentials(ref err) => err.description(),
            GetObjectAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetObjectAttributesError::ParseError(ref cause) => cause,
            GetObjectAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetObjectInformation
#[derive(Debug, PartialEq)]
pub enum GetObjectInformationError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetObjectInformationError {
    pub fn from_response(res: BufferedHttpResponse) -> GetObjectInformationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetObjectInformationError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return GetObjectInformationError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "InternalServiceException" => {
                    return GetObjectInformationError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return GetObjectInformationError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetObjectInformationError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return GetObjectInformationError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return GetObjectInformationError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return GetObjectInformationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetObjectInformationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetObjectInformationError {
    fn from(err: serde_json::error::Error) -> GetObjectInformationError {
        GetObjectInformationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetObjectInformationError {
    fn from(err: CredentialsError) -> GetObjectInformationError {
        GetObjectInformationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectInformationError {
    fn from(err: HttpDispatchError) -> GetObjectInformationError {
        GetObjectInformationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectInformationError {
    fn from(err: io::Error) -> GetObjectInformationError {
        GetObjectInformationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectInformationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectInformationError {
    fn description(&self) -> &str {
        match *self {
            GetObjectInformationError::AccessDenied(ref cause) => cause,
            GetObjectInformationError::DirectoryNotEnabled(ref cause) => cause,
            GetObjectInformationError::InternalService(ref cause) => cause,
            GetObjectInformationError::InvalidArn(ref cause) => cause,
            GetObjectInformationError::LimitExceeded(ref cause) => cause,
            GetObjectInformationError::ResourceNotFound(ref cause) => cause,
            GetObjectInformationError::RetryableConflict(ref cause) => cause,
            GetObjectInformationError::Validation(ref cause) => cause,
            GetObjectInformationError::Credentials(ref err) => err.description(),
            GetObjectInformationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetObjectInformationError::ParseError(ref cause) => cause,
            GetObjectInformationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSchemaAsJson
#[derive(Debug, PartialEq)]
pub enum GetSchemaAsJsonError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSchemaAsJsonError {
    pub fn from_response(res: BufferedHttpResponse) -> GetSchemaAsJsonError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetSchemaAsJsonError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return GetSchemaAsJsonError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return GetSchemaAsJsonError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetSchemaAsJsonError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return GetSchemaAsJsonError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return GetSchemaAsJsonError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSchemaAsJsonError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSchemaAsJsonError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSchemaAsJsonError {
    fn from(err: serde_json::error::Error) -> GetSchemaAsJsonError {
        GetSchemaAsJsonError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSchemaAsJsonError {
    fn from(err: CredentialsError) -> GetSchemaAsJsonError {
        GetSchemaAsJsonError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSchemaAsJsonError {
    fn from(err: HttpDispatchError) -> GetSchemaAsJsonError {
        GetSchemaAsJsonError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSchemaAsJsonError {
    fn from(err: io::Error) -> GetSchemaAsJsonError {
        GetSchemaAsJsonError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSchemaAsJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSchemaAsJsonError {
    fn description(&self) -> &str {
        match *self {
            GetSchemaAsJsonError::AccessDenied(ref cause) => cause,
            GetSchemaAsJsonError::InternalService(ref cause) => cause,
            GetSchemaAsJsonError::InvalidArn(ref cause) => cause,
            GetSchemaAsJsonError::LimitExceeded(ref cause) => cause,
            GetSchemaAsJsonError::ResourceNotFound(ref cause) => cause,
            GetSchemaAsJsonError::RetryableConflict(ref cause) => cause,
            GetSchemaAsJsonError::Validation(ref cause) => cause,
            GetSchemaAsJsonError::Credentials(ref err) => err.description(),
            GetSchemaAsJsonError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSchemaAsJsonError::ParseError(ref cause) => cause,
            GetSchemaAsJsonError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetTypedLinkFacetInformation
#[derive(Debug, PartialEq)]
pub enum GetTypedLinkFacetInformationError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetTypedLinkFacetInformationError {
    pub fn from_response(res: BufferedHttpResponse) -> GetTypedLinkFacetInformationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetTypedLinkFacetInformationError::AccessDenied(String::from(
                        error_message,
                    ))
                }
                "FacetNotFoundException" => {
                    return GetTypedLinkFacetInformationError::FacetNotFound(String::from(
                        error_message,
                    ))
                }
                "InternalServiceException" => {
                    return GetTypedLinkFacetInformationError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidArnException" => {
                    return GetTypedLinkFacetInformationError::InvalidArn(String::from(
                        error_message,
                    ))
                }
                "InvalidNextTokenException" => {
                    return GetTypedLinkFacetInformationError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return GetTypedLinkFacetInformationError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return GetTypedLinkFacetInformationError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "RetryableConflictException" => {
                    return GetTypedLinkFacetInformationError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetTypedLinkFacetInformationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetTypedLinkFacetInformationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetTypedLinkFacetInformationError {
    fn from(err: serde_json::error::Error) -> GetTypedLinkFacetInformationError {
        GetTypedLinkFacetInformationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTypedLinkFacetInformationError {
    fn from(err: CredentialsError) -> GetTypedLinkFacetInformationError {
        GetTypedLinkFacetInformationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTypedLinkFacetInformationError {
    fn from(err: HttpDispatchError) -> GetTypedLinkFacetInformationError {
        GetTypedLinkFacetInformationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTypedLinkFacetInformationError {
    fn from(err: io::Error) -> GetTypedLinkFacetInformationError {
        GetTypedLinkFacetInformationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTypedLinkFacetInformationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTypedLinkFacetInformationError {
    fn description(&self) -> &str {
        match *self {
            GetTypedLinkFacetInformationError::AccessDenied(ref cause) => cause,
            GetTypedLinkFacetInformationError::FacetNotFound(ref cause) => cause,
            GetTypedLinkFacetInformationError::InternalService(ref cause) => cause,
            GetTypedLinkFacetInformationError::InvalidArn(ref cause) => cause,
            GetTypedLinkFacetInformationError::InvalidNextToken(ref cause) => cause,
            GetTypedLinkFacetInformationError::LimitExceeded(ref cause) => cause,
            GetTypedLinkFacetInformationError::ResourceNotFound(ref cause) => cause,
            GetTypedLinkFacetInformationError::RetryableConflict(ref cause) => cause,
            GetTypedLinkFacetInformationError::Validation(ref cause) => cause,
            GetTypedLinkFacetInformationError::Credentials(ref err) => err.description(),
            GetTypedLinkFacetInformationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetTypedLinkFacetInformationError::ParseError(ref cause) => cause,
            GetTypedLinkFacetInformationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListAppliedSchemaArns
#[derive(Debug, PartialEq)]
pub enum ListAppliedSchemaArnsError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListAppliedSchemaArnsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListAppliedSchemaArnsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListAppliedSchemaArnsError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListAppliedSchemaArnsError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListAppliedSchemaArnsError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListAppliedSchemaArnsError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListAppliedSchemaArnsError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListAppliedSchemaArnsError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListAppliedSchemaArnsError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListAppliedSchemaArnsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListAppliedSchemaArnsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListAppliedSchemaArnsError {
    fn from(err: serde_json::error::Error) -> ListAppliedSchemaArnsError {
        ListAppliedSchemaArnsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAppliedSchemaArnsError {
    fn from(err: CredentialsError) -> ListAppliedSchemaArnsError {
        ListAppliedSchemaArnsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAppliedSchemaArnsError {
    fn from(err: HttpDispatchError) -> ListAppliedSchemaArnsError {
        ListAppliedSchemaArnsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAppliedSchemaArnsError {
    fn from(err: io::Error) -> ListAppliedSchemaArnsError {
        ListAppliedSchemaArnsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAppliedSchemaArnsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAppliedSchemaArnsError {
    fn description(&self) -> &str {
        match *self {
            ListAppliedSchemaArnsError::AccessDenied(ref cause) => cause,
            ListAppliedSchemaArnsError::InternalService(ref cause) => cause,
            ListAppliedSchemaArnsError::InvalidArn(ref cause) => cause,
            ListAppliedSchemaArnsError::InvalidNextToken(ref cause) => cause,
            ListAppliedSchemaArnsError::LimitExceeded(ref cause) => cause,
            ListAppliedSchemaArnsError::ResourceNotFound(ref cause) => cause,
            ListAppliedSchemaArnsError::RetryableConflict(ref cause) => cause,
            ListAppliedSchemaArnsError::Validation(ref cause) => cause,
            ListAppliedSchemaArnsError::Credentials(ref err) => err.description(),
            ListAppliedSchemaArnsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAppliedSchemaArnsError::ParseError(ref cause) => cause,
            ListAppliedSchemaArnsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListAttachedIndices
#[derive(Debug, PartialEq)]
pub enum ListAttachedIndicesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListAttachedIndicesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListAttachedIndicesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListAttachedIndicesError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return ListAttachedIndicesError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "InternalServiceException" => {
                    return ListAttachedIndicesError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListAttachedIndicesError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListAttachedIndicesError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListAttachedIndicesError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListAttachedIndicesError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ListAttachedIndicesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListAttachedIndicesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListAttachedIndicesError {
    fn from(err: serde_json::error::Error) -> ListAttachedIndicesError {
        ListAttachedIndicesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAttachedIndicesError {
    fn from(err: CredentialsError) -> ListAttachedIndicesError {
        ListAttachedIndicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAttachedIndicesError {
    fn from(err: HttpDispatchError) -> ListAttachedIndicesError {
        ListAttachedIndicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAttachedIndicesError {
    fn from(err: io::Error) -> ListAttachedIndicesError {
        ListAttachedIndicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAttachedIndicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAttachedIndicesError {
    fn description(&self) -> &str {
        match *self {
            ListAttachedIndicesError::AccessDenied(ref cause) => cause,
            ListAttachedIndicesError::DirectoryNotEnabled(ref cause) => cause,
            ListAttachedIndicesError::InternalService(ref cause) => cause,
            ListAttachedIndicesError::InvalidArn(ref cause) => cause,
            ListAttachedIndicesError::LimitExceeded(ref cause) => cause,
            ListAttachedIndicesError::ResourceNotFound(ref cause) => cause,
            ListAttachedIndicesError::RetryableConflict(ref cause) => cause,
            ListAttachedIndicesError::Validation(ref cause) => cause,
            ListAttachedIndicesError::Credentials(ref err) => err.description(),
            ListAttachedIndicesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAttachedIndicesError::ParseError(ref cause) => cause,
            ListAttachedIndicesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDevelopmentSchemaArns
#[derive(Debug, PartialEq)]
pub enum ListDevelopmentSchemaArnsError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListDevelopmentSchemaArnsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDevelopmentSchemaArnsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListDevelopmentSchemaArnsError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListDevelopmentSchemaArnsError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidArnException" => {
                    return ListDevelopmentSchemaArnsError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListDevelopmentSchemaArnsError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return ListDevelopmentSchemaArnsError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return ListDevelopmentSchemaArnsError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "RetryableConflictException" => {
                    return ListDevelopmentSchemaArnsError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListDevelopmentSchemaArnsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListDevelopmentSchemaArnsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDevelopmentSchemaArnsError {
    fn from(err: serde_json::error::Error) -> ListDevelopmentSchemaArnsError {
        ListDevelopmentSchemaArnsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDevelopmentSchemaArnsError {
    fn from(err: CredentialsError) -> ListDevelopmentSchemaArnsError {
        ListDevelopmentSchemaArnsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDevelopmentSchemaArnsError {
    fn from(err: HttpDispatchError) -> ListDevelopmentSchemaArnsError {
        ListDevelopmentSchemaArnsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDevelopmentSchemaArnsError {
    fn from(err: io::Error) -> ListDevelopmentSchemaArnsError {
        ListDevelopmentSchemaArnsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDevelopmentSchemaArnsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDevelopmentSchemaArnsError {
    fn description(&self) -> &str {
        match *self {
            ListDevelopmentSchemaArnsError::AccessDenied(ref cause) => cause,
            ListDevelopmentSchemaArnsError::InternalService(ref cause) => cause,
            ListDevelopmentSchemaArnsError::InvalidArn(ref cause) => cause,
            ListDevelopmentSchemaArnsError::InvalidNextToken(ref cause) => cause,
            ListDevelopmentSchemaArnsError::LimitExceeded(ref cause) => cause,
            ListDevelopmentSchemaArnsError::ResourceNotFound(ref cause) => cause,
            ListDevelopmentSchemaArnsError::RetryableConflict(ref cause) => cause,
            ListDevelopmentSchemaArnsError::Validation(ref cause) => cause,
            ListDevelopmentSchemaArnsError::Credentials(ref err) => err.description(),
            ListDevelopmentSchemaArnsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDevelopmentSchemaArnsError::ParseError(ref cause) => cause,
            ListDevelopmentSchemaArnsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDirectories
#[derive(Debug, PartialEq)]
pub enum ListDirectoriesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListDirectoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDirectoriesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListDirectoriesError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListDirectoriesError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListDirectoriesError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListDirectoriesError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListDirectoriesError::LimitExceeded(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListDirectoriesError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ListDirectoriesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListDirectoriesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDirectoriesError {
    fn from(err: serde_json::error::Error) -> ListDirectoriesError {
        ListDirectoriesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDirectoriesError {
    fn from(err: CredentialsError) -> ListDirectoriesError {
        ListDirectoriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDirectoriesError {
    fn from(err: HttpDispatchError) -> ListDirectoriesError {
        ListDirectoriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDirectoriesError {
    fn from(err: io::Error) -> ListDirectoriesError {
        ListDirectoriesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDirectoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDirectoriesError {
    fn description(&self) -> &str {
        match *self {
            ListDirectoriesError::AccessDenied(ref cause) => cause,
            ListDirectoriesError::InternalService(ref cause) => cause,
            ListDirectoriesError::InvalidArn(ref cause) => cause,
            ListDirectoriesError::InvalidNextToken(ref cause) => cause,
            ListDirectoriesError::LimitExceeded(ref cause) => cause,
            ListDirectoriesError::RetryableConflict(ref cause) => cause,
            ListDirectoriesError::Validation(ref cause) => cause,
            ListDirectoriesError::Credentials(ref err) => err.description(),
            ListDirectoriesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDirectoriesError::ParseError(ref cause) => cause,
            ListDirectoriesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListFacetAttributes
#[derive(Debug, PartialEq)]
pub enum ListFacetAttributesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListFacetAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListFacetAttributesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListFacetAttributesError::AccessDenied(String::from(error_message))
                }
                "FacetNotFoundException" => {
                    return ListFacetAttributesError::FacetNotFound(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListFacetAttributesError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListFacetAttributesError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListFacetAttributesError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListFacetAttributesError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListFacetAttributesError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListFacetAttributesError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ListFacetAttributesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListFacetAttributesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListFacetAttributesError {
    fn from(err: serde_json::error::Error) -> ListFacetAttributesError {
        ListFacetAttributesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFacetAttributesError {
    fn from(err: CredentialsError) -> ListFacetAttributesError {
        ListFacetAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFacetAttributesError {
    fn from(err: HttpDispatchError) -> ListFacetAttributesError {
        ListFacetAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFacetAttributesError {
    fn from(err: io::Error) -> ListFacetAttributesError {
        ListFacetAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFacetAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFacetAttributesError {
    fn description(&self) -> &str {
        match *self {
            ListFacetAttributesError::AccessDenied(ref cause) => cause,
            ListFacetAttributesError::FacetNotFound(ref cause) => cause,
            ListFacetAttributesError::InternalService(ref cause) => cause,
            ListFacetAttributesError::InvalidArn(ref cause) => cause,
            ListFacetAttributesError::InvalidNextToken(ref cause) => cause,
            ListFacetAttributesError::LimitExceeded(ref cause) => cause,
            ListFacetAttributesError::ResourceNotFound(ref cause) => cause,
            ListFacetAttributesError::RetryableConflict(ref cause) => cause,
            ListFacetAttributesError::Validation(ref cause) => cause,
            ListFacetAttributesError::Credentials(ref err) => err.description(),
            ListFacetAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListFacetAttributesError::ParseError(ref cause) => cause,
            ListFacetAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListFacetNames
#[derive(Debug, PartialEq)]
pub enum ListFacetNamesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListFacetNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListFacetNamesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListFacetNamesError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListFacetNamesError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListFacetNamesError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListFacetNamesError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListFacetNamesError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListFacetNamesError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListFacetNamesError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ListFacetNamesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListFacetNamesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListFacetNamesError {
    fn from(err: serde_json::error::Error) -> ListFacetNamesError {
        ListFacetNamesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFacetNamesError {
    fn from(err: CredentialsError) -> ListFacetNamesError {
        ListFacetNamesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFacetNamesError {
    fn from(err: HttpDispatchError) -> ListFacetNamesError {
        ListFacetNamesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFacetNamesError {
    fn from(err: io::Error) -> ListFacetNamesError {
        ListFacetNamesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFacetNamesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFacetNamesError {
    fn description(&self) -> &str {
        match *self {
            ListFacetNamesError::AccessDenied(ref cause) => cause,
            ListFacetNamesError::InternalService(ref cause) => cause,
            ListFacetNamesError::InvalidArn(ref cause) => cause,
            ListFacetNamesError::InvalidNextToken(ref cause) => cause,
            ListFacetNamesError::LimitExceeded(ref cause) => cause,
            ListFacetNamesError::ResourceNotFound(ref cause) => cause,
            ListFacetNamesError::RetryableConflict(ref cause) => cause,
            ListFacetNamesError::Validation(ref cause) => cause,
            ListFacetNamesError::Credentials(ref err) => err.description(),
            ListFacetNamesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFacetNamesError::ParseError(ref cause) => cause,
            ListFacetNamesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListIncomingTypedLinks
#[derive(Debug, PartialEq)]
pub enum ListIncomingTypedLinksError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListIncomingTypedLinksError {
    pub fn from_response(res: BufferedHttpResponse) -> ListIncomingTypedLinksError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListIncomingTypedLinksError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return ListIncomingTypedLinksError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "FacetValidationException" => {
                    return ListIncomingTypedLinksError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListIncomingTypedLinksError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListIncomingTypedLinksError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListIncomingTypedLinksError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return ListIncomingTypedLinksError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListIncomingTypedLinksError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "RetryableConflictException" => {
                    return ListIncomingTypedLinksError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListIncomingTypedLinksError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListIncomingTypedLinksError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListIncomingTypedLinksError {
    fn from(err: serde_json::error::Error) -> ListIncomingTypedLinksError {
        ListIncomingTypedLinksError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListIncomingTypedLinksError {
    fn from(err: CredentialsError) -> ListIncomingTypedLinksError {
        ListIncomingTypedLinksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIncomingTypedLinksError {
    fn from(err: HttpDispatchError) -> ListIncomingTypedLinksError {
        ListIncomingTypedLinksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListIncomingTypedLinksError {
    fn from(err: io::Error) -> ListIncomingTypedLinksError {
        ListIncomingTypedLinksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListIncomingTypedLinksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIncomingTypedLinksError {
    fn description(&self) -> &str {
        match *self {
            ListIncomingTypedLinksError::AccessDenied(ref cause) => cause,
            ListIncomingTypedLinksError::DirectoryNotEnabled(ref cause) => cause,
            ListIncomingTypedLinksError::FacetValidation(ref cause) => cause,
            ListIncomingTypedLinksError::InternalService(ref cause) => cause,
            ListIncomingTypedLinksError::InvalidArn(ref cause) => cause,
            ListIncomingTypedLinksError::InvalidNextToken(ref cause) => cause,
            ListIncomingTypedLinksError::LimitExceeded(ref cause) => cause,
            ListIncomingTypedLinksError::ResourceNotFound(ref cause) => cause,
            ListIncomingTypedLinksError::RetryableConflict(ref cause) => cause,
            ListIncomingTypedLinksError::Validation(ref cause) => cause,
            ListIncomingTypedLinksError::Credentials(ref err) => err.description(),
            ListIncomingTypedLinksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListIncomingTypedLinksError::ParseError(ref cause) => cause,
            ListIncomingTypedLinksError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListIndex
#[derive(Debug, PartialEq)]
pub enum ListIndexError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation can only operate on index objects.</p>
    NotIndex(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> ListIndexError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListIndexError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return ListIndexError::DirectoryNotEnabled(String::from(error_message))
                }
                "FacetValidationException" => {
                    return ListIndexError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListIndexError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListIndexError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListIndexError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListIndexError::LimitExceeded(String::from(error_message))
                }
                "NotIndexException" => return ListIndexError::NotIndex(String::from(error_message)),
                "ResourceNotFoundException" => {
                    return ListIndexError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListIndexError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ListIndexError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListIndexError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListIndexError {
    fn from(err: serde_json::error::Error) -> ListIndexError {
        ListIndexError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListIndexError {
    fn from(err: CredentialsError) -> ListIndexError {
        ListIndexError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIndexError {
    fn from(err: HttpDispatchError) -> ListIndexError {
        ListIndexError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListIndexError {
    fn from(err: io::Error) -> ListIndexError {
        ListIndexError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIndexError {
    fn description(&self) -> &str {
        match *self {
            ListIndexError::AccessDenied(ref cause) => cause,
            ListIndexError::DirectoryNotEnabled(ref cause) => cause,
            ListIndexError::FacetValidation(ref cause) => cause,
            ListIndexError::InternalService(ref cause) => cause,
            ListIndexError::InvalidArn(ref cause) => cause,
            ListIndexError::InvalidNextToken(ref cause) => cause,
            ListIndexError::LimitExceeded(ref cause) => cause,
            ListIndexError::NotIndex(ref cause) => cause,
            ListIndexError::ResourceNotFound(ref cause) => cause,
            ListIndexError::RetryableConflict(ref cause) => cause,
            ListIndexError::Validation(ref cause) => cause,
            ListIndexError::Credentials(ref err) => err.description(),
            ListIndexError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListIndexError::ParseError(ref cause) => cause,
            ListIndexError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListObjectAttributes
#[derive(Debug, PartialEq)]
pub enum ListObjectAttributesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListObjectAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListObjectAttributesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListObjectAttributesError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return ListObjectAttributesError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "FacetValidationException" => {
                    return ListObjectAttributesError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListObjectAttributesError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListObjectAttributesError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListObjectAttributesError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListObjectAttributesError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListObjectAttributesError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListObjectAttributesError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ListObjectAttributesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListObjectAttributesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListObjectAttributesError {
    fn from(err: serde_json::error::Error) -> ListObjectAttributesError {
        ListObjectAttributesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListObjectAttributesError {
    fn from(err: CredentialsError) -> ListObjectAttributesError {
        ListObjectAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectAttributesError {
    fn from(err: HttpDispatchError) -> ListObjectAttributesError {
        ListObjectAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectAttributesError {
    fn from(err: io::Error) -> ListObjectAttributesError {
        ListObjectAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectAttributesError {
    fn description(&self) -> &str {
        match *self {
            ListObjectAttributesError::AccessDenied(ref cause) => cause,
            ListObjectAttributesError::DirectoryNotEnabled(ref cause) => cause,
            ListObjectAttributesError::FacetValidation(ref cause) => cause,
            ListObjectAttributesError::InternalService(ref cause) => cause,
            ListObjectAttributesError::InvalidArn(ref cause) => cause,
            ListObjectAttributesError::InvalidNextToken(ref cause) => cause,
            ListObjectAttributesError::LimitExceeded(ref cause) => cause,
            ListObjectAttributesError::ResourceNotFound(ref cause) => cause,
            ListObjectAttributesError::RetryableConflict(ref cause) => cause,
            ListObjectAttributesError::Validation(ref cause) => cause,
            ListObjectAttributesError::Credentials(ref err) => err.description(),
            ListObjectAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectAttributesError::ParseError(ref cause) => cause,
            ListObjectAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListObjectChildren
#[derive(Debug, PartialEq)]
pub enum ListObjectChildrenError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when any invalid operations are performed on an object that is not a node, such as calling <code>ListObjectChildren</code> for a leaf node object.</p>
    NotNode(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListObjectChildrenError {
    pub fn from_response(res: BufferedHttpResponse) -> ListObjectChildrenError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListObjectChildrenError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return ListObjectChildrenError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListObjectChildrenError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListObjectChildrenError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListObjectChildrenError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListObjectChildrenError::LimitExceeded(String::from(error_message))
                }
                "NotNodeException" => {
                    return ListObjectChildrenError::NotNode(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListObjectChildrenError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListObjectChildrenError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ListObjectChildrenError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListObjectChildrenError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListObjectChildrenError {
    fn from(err: serde_json::error::Error) -> ListObjectChildrenError {
        ListObjectChildrenError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListObjectChildrenError {
    fn from(err: CredentialsError) -> ListObjectChildrenError {
        ListObjectChildrenError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectChildrenError {
    fn from(err: HttpDispatchError) -> ListObjectChildrenError {
        ListObjectChildrenError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectChildrenError {
    fn from(err: io::Error) -> ListObjectChildrenError {
        ListObjectChildrenError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectChildrenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectChildrenError {
    fn description(&self) -> &str {
        match *self {
            ListObjectChildrenError::AccessDenied(ref cause) => cause,
            ListObjectChildrenError::DirectoryNotEnabled(ref cause) => cause,
            ListObjectChildrenError::InternalService(ref cause) => cause,
            ListObjectChildrenError::InvalidArn(ref cause) => cause,
            ListObjectChildrenError::InvalidNextToken(ref cause) => cause,
            ListObjectChildrenError::LimitExceeded(ref cause) => cause,
            ListObjectChildrenError::NotNode(ref cause) => cause,
            ListObjectChildrenError::ResourceNotFound(ref cause) => cause,
            ListObjectChildrenError::RetryableConflict(ref cause) => cause,
            ListObjectChildrenError::Validation(ref cause) => cause,
            ListObjectChildrenError::Credentials(ref err) => err.description(),
            ListObjectChildrenError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectChildrenError::ParseError(ref cause) => cause,
            ListObjectChildrenError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListObjectParentPaths
#[derive(Debug, PartialEq)]
pub enum ListObjectParentPathsError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListObjectParentPathsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListObjectParentPathsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListObjectParentPathsError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return ListObjectParentPathsError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "InternalServiceException" => {
                    return ListObjectParentPathsError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListObjectParentPathsError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListObjectParentPathsError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListObjectParentPathsError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListObjectParentPathsError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListObjectParentPathsError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListObjectParentPathsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListObjectParentPathsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListObjectParentPathsError {
    fn from(err: serde_json::error::Error) -> ListObjectParentPathsError {
        ListObjectParentPathsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListObjectParentPathsError {
    fn from(err: CredentialsError) -> ListObjectParentPathsError {
        ListObjectParentPathsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectParentPathsError {
    fn from(err: HttpDispatchError) -> ListObjectParentPathsError {
        ListObjectParentPathsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectParentPathsError {
    fn from(err: io::Error) -> ListObjectParentPathsError {
        ListObjectParentPathsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectParentPathsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectParentPathsError {
    fn description(&self) -> &str {
        match *self {
            ListObjectParentPathsError::AccessDenied(ref cause) => cause,
            ListObjectParentPathsError::DirectoryNotEnabled(ref cause) => cause,
            ListObjectParentPathsError::InternalService(ref cause) => cause,
            ListObjectParentPathsError::InvalidArn(ref cause) => cause,
            ListObjectParentPathsError::InvalidNextToken(ref cause) => cause,
            ListObjectParentPathsError::LimitExceeded(ref cause) => cause,
            ListObjectParentPathsError::ResourceNotFound(ref cause) => cause,
            ListObjectParentPathsError::RetryableConflict(ref cause) => cause,
            ListObjectParentPathsError::Validation(ref cause) => cause,
            ListObjectParentPathsError::Credentials(ref err) => err.description(),
            ListObjectParentPathsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectParentPathsError::ParseError(ref cause) => cause,
            ListObjectParentPathsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListObjectParents
#[derive(Debug, PartialEq)]
pub enum ListObjectParentsError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Cannot list the parents of a <a>Directory</a> root.</p>
    CannotListParentOfRoot(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListObjectParentsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListObjectParentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListObjectParentsError::AccessDenied(String::from(error_message))
                }
                "CannotListParentOfRootException" => {
                    return ListObjectParentsError::CannotListParentOfRoot(String::from(
                        error_message,
                    ))
                }
                "DirectoryNotEnabledException" => {
                    return ListObjectParentsError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListObjectParentsError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListObjectParentsError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListObjectParentsError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListObjectParentsError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListObjectParentsError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListObjectParentsError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ListObjectParentsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListObjectParentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListObjectParentsError {
    fn from(err: serde_json::error::Error) -> ListObjectParentsError {
        ListObjectParentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListObjectParentsError {
    fn from(err: CredentialsError) -> ListObjectParentsError {
        ListObjectParentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectParentsError {
    fn from(err: HttpDispatchError) -> ListObjectParentsError {
        ListObjectParentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectParentsError {
    fn from(err: io::Error) -> ListObjectParentsError {
        ListObjectParentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectParentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectParentsError {
    fn description(&self) -> &str {
        match *self {
            ListObjectParentsError::AccessDenied(ref cause) => cause,
            ListObjectParentsError::CannotListParentOfRoot(ref cause) => cause,
            ListObjectParentsError::DirectoryNotEnabled(ref cause) => cause,
            ListObjectParentsError::InternalService(ref cause) => cause,
            ListObjectParentsError::InvalidArn(ref cause) => cause,
            ListObjectParentsError::InvalidNextToken(ref cause) => cause,
            ListObjectParentsError::LimitExceeded(ref cause) => cause,
            ListObjectParentsError::ResourceNotFound(ref cause) => cause,
            ListObjectParentsError::RetryableConflict(ref cause) => cause,
            ListObjectParentsError::Validation(ref cause) => cause,
            ListObjectParentsError::Credentials(ref err) => err.description(),
            ListObjectParentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectParentsError::ParseError(ref cause) => cause,
            ListObjectParentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListObjectPolicies
#[derive(Debug, PartialEq)]
pub enum ListObjectPoliciesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListObjectPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListObjectPoliciesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListObjectPoliciesError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return ListObjectPoliciesError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListObjectPoliciesError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListObjectPoliciesError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListObjectPoliciesError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListObjectPoliciesError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListObjectPoliciesError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListObjectPoliciesError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ListObjectPoliciesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListObjectPoliciesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListObjectPoliciesError {
    fn from(err: serde_json::error::Error) -> ListObjectPoliciesError {
        ListObjectPoliciesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListObjectPoliciesError {
    fn from(err: CredentialsError) -> ListObjectPoliciesError {
        ListObjectPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectPoliciesError {
    fn from(err: HttpDispatchError) -> ListObjectPoliciesError {
        ListObjectPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectPoliciesError {
    fn from(err: io::Error) -> ListObjectPoliciesError {
        ListObjectPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListObjectPoliciesError::AccessDenied(ref cause) => cause,
            ListObjectPoliciesError::DirectoryNotEnabled(ref cause) => cause,
            ListObjectPoliciesError::InternalService(ref cause) => cause,
            ListObjectPoliciesError::InvalidArn(ref cause) => cause,
            ListObjectPoliciesError::InvalidNextToken(ref cause) => cause,
            ListObjectPoliciesError::LimitExceeded(ref cause) => cause,
            ListObjectPoliciesError::ResourceNotFound(ref cause) => cause,
            ListObjectPoliciesError::RetryableConflict(ref cause) => cause,
            ListObjectPoliciesError::Validation(ref cause) => cause,
            ListObjectPoliciesError::Credentials(ref err) => err.description(),
            ListObjectPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectPoliciesError::ParseError(ref cause) => cause,
            ListObjectPoliciesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListOutgoingTypedLinks
#[derive(Debug, PartialEq)]
pub enum ListOutgoingTypedLinksError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListOutgoingTypedLinksError {
    pub fn from_response(res: BufferedHttpResponse) -> ListOutgoingTypedLinksError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListOutgoingTypedLinksError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return ListOutgoingTypedLinksError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "FacetValidationException" => {
                    return ListOutgoingTypedLinksError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListOutgoingTypedLinksError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListOutgoingTypedLinksError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListOutgoingTypedLinksError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return ListOutgoingTypedLinksError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListOutgoingTypedLinksError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "RetryableConflictException" => {
                    return ListOutgoingTypedLinksError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListOutgoingTypedLinksError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListOutgoingTypedLinksError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListOutgoingTypedLinksError {
    fn from(err: serde_json::error::Error) -> ListOutgoingTypedLinksError {
        ListOutgoingTypedLinksError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOutgoingTypedLinksError {
    fn from(err: CredentialsError) -> ListOutgoingTypedLinksError {
        ListOutgoingTypedLinksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOutgoingTypedLinksError {
    fn from(err: HttpDispatchError) -> ListOutgoingTypedLinksError {
        ListOutgoingTypedLinksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOutgoingTypedLinksError {
    fn from(err: io::Error) -> ListOutgoingTypedLinksError {
        ListOutgoingTypedLinksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOutgoingTypedLinksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOutgoingTypedLinksError {
    fn description(&self) -> &str {
        match *self {
            ListOutgoingTypedLinksError::AccessDenied(ref cause) => cause,
            ListOutgoingTypedLinksError::DirectoryNotEnabled(ref cause) => cause,
            ListOutgoingTypedLinksError::FacetValidation(ref cause) => cause,
            ListOutgoingTypedLinksError::InternalService(ref cause) => cause,
            ListOutgoingTypedLinksError::InvalidArn(ref cause) => cause,
            ListOutgoingTypedLinksError::InvalidNextToken(ref cause) => cause,
            ListOutgoingTypedLinksError::LimitExceeded(ref cause) => cause,
            ListOutgoingTypedLinksError::ResourceNotFound(ref cause) => cause,
            ListOutgoingTypedLinksError::RetryableConflict(ref cause) => cause,
            ListOutgoingTypedLinksError::Validation(ref cause) => cause,
            ListOutgoingTypedLinksError::Credentials(ref err) => err.description(),
            ListOutgoingTypedLinksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOutgoingTypedLinksError::ParseError(ref cause) => cause,
            ListOutgoingTypedLinksError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListPolicyAttachments
#[derive(Debug, PartialEq)]
pub enum ListPolicyAttachmentsError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation can only operate on policy objects.</p>
    NotPolicy(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListPolicyAttachmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListPolicyAttachmentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListPolicyAttachmentsError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return ListPolicyAttachmentsError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "InternalServiceException" => {
                    return ListPolicyAttachmentsError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListPolicyAttachmentsError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListPolicyAttachmentsError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListPolicyAttachmentsError::LimitExceeded(String::from(error_message))
                }
                "NotPolicyException" => {
                    return ListPolicyAttachmentsError::NotPolicy(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListPolicyAttachmentsError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListPolicyAttachmentsError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListPolicyAttachmentsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListPolicyAttachmentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListPolicyAttachmentsError {
    fn from(err: serde_json::error::Error) -> ListPolicyAttachmentsError {
        ListPolicyAttachmentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPolicyAttachmentsError {
    fn from(err: CredentialsError) -> ListPolicyAttachmentsError {
        ListPolicyAttachmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPolicyAttachmentsError {
    fn from(err: HttpDispatchError) -> ListPolicyAttachmentsError {
        ListPolicyAttachmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPolicyAttachmentsError {
    fn from(err: io::Error) -> ListPolicyAttachmentsError {
        ListPolicyAttachmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPolicyAttachmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPolicyAttachmentsError {
    fn description(&self) -> &str {
        match *self {
            ListPolicyAttachmentsError::AccessDenied(ref cause) => cause,
            ListPolicyAttachmentsError::DirectoryNotEnabled(ref cause) => cause,
            ListPolicyAttachmentsError::InternalService(ref cause) => cause,
            ListPolicyAttachmentsError::InvalidArn(ref cause) => cause,
            ListPolicyAttachmentsError::InvalidNextToken(ref cause) => cause,
            ListPolicyAttachmentsError::LimitExceeded(ref cause) => cause,
            ListPolicyAttachmentsError::NotPolicy(ref cause) => cause,
            ListPolicyAttachmentsError::ResourceNotFound(ref cause) => cause,
            ListPolicyAttachmentsError::RetryableConflict(ref cause) => cause,
            ListPolicyAttachmentsError::Validation(ref cause) => cause,
            ListPolicyAttachmentsError::Credentials(ref err) => err.description(),
            ListPolicyAttachmentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPolicyAttachmentsError::ParseError(ref cause) => cause,
            ListPolicyAttachmentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListPublishedSchemaArns
#[derive(Debug, PartialEq)]
pub enum ListPublishedSchemaArnsError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListPublishedSchemaArnsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListPublishedSchemaArnsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListPublishedSchemaArnsError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListPublishedSchemaArnsError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidArnException" => {
                    return ListPublishedSchemaArnsError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListPublishedSchemaArnsError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return ListPublishedSchemaArnsError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListPublishedSchemaArnsError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "RetryableConflictException" => {
                    return ListPublishedSchemaArnsError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListPublishedSchemaArnsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListPublishedSchemaArnsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListPublishedSchemaArnsError {
    fn from(err: serde_json::error::Error) -> ListPublishedSchemaArnsError {
        ListPublishedSchemaArnsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPublishedSchemaArnsError {
    fn from(err: CredentialsError) -> ListPublishedSchemaArnsError {
        ListPublishedSchemaArnsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPublishedSchemaArnsError {
    fn from(err: HttpDispatchError) -> ListPublishedSchemaArnsError {
        ListPublishedSchemaArnsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPublishedSchemaArnsError {
    fn from(err: io::Error) -> ListPublishedSchemaArnsError {
        ListPublishedSchemaArnsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPublishedSchemaArnsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPublishedSchemaArnsError {
    fn description(&self) -> &str {
        match *self {
            ListPublishedSchemaArnsError::AccessDenied(ref cause) => cause,
            ListPublishedSchemaArnsError::InternalService(ref cause) => cause,
            ListPublishedSchemaArnsError::InvalidArn(ref cause) => cause,
            ListPublishedSchemaArnsError::InvalidNextToken(ref cause) => cause,
            ListPublishedSchemaArnsError::LimitExceeded(ref cause) => cause,
            ListPublishedSchemaArnsError::ResourceNotFound(ref cause) => cause,
            ListPublishedSchemaArnsError::RetryableConflict(ref cause) => cause,
            ListPublishedSchemaArnsError::Validation(ref cause) => cause,
            ListPublishedSchemaArnsError::Credentials(ref err) => err.description(),
            ListPublishedSchemaArnsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPublishedSchemaArnsError::ParseError(ref cause) => cause,
            ListPublishedSchemaArnsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Can occur for multiple reasons such as when you tag a resource that doesn’t exist or if you specify a higher number of tags for a resource than the allowed limit. Allowed limit is 50 tags per resource.</p>
    InvalidTaggingRequest(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTagsForResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListTagsForResourceError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListTagsForResourceError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return ListTagsForResourceError::InvalidArn(String::from(error_message))
                }
                "InvalidTaggingRequestException" => {
                    return ListTagsForResourceError::InvalidTaggingRequest(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return ListTagsForResourceError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListTagsForResourceError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return ListTagsForResourceError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return ListTagsForResourceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTagsForResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTagsForResourceError {
    fn from(err: serde_json::error::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourceError {
    fn from(err: io::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::AccessDenied(ref cause) => cause,
            ListTagsForResourceError::InternalService(ref cause) => cause,
            ListTagsForResourceError::InvalidArn(ref cause) => cause,
            ListTagsForResourceError::InvalidTaggingRequest(ref cause) => cause,
            ListTagsForResourceError::LimitExceeded(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
            ListTagsForResourceError::RetryableConflict(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::ParseError(ref cause) => cause,
            ListTagsForResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTypedLinkFacetAttributes
#[derive(Debug, PartialEq)]
pub enum ListTypedLinkFacetAttributesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListTypedLinkFacetAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTypedLinkFacetAttributesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListTypedLinkFacetAttributesError::AccessDenied(String::from(
                        error_message,
                    ))
                }
                "FacetNotFoundException" => {
                    return ListTypedLinkFacetAttributesError::FacetNotFound(String::from(
                        error_message,
                    ))
                }
                "InternalServiceException" => {
                    return ListTypedLinkFacetAttributesError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidArnException" => {
                    return ListTypedLinkFacetAttributesError::InvalidArn(String::from(
                        error_message,
                    ))
                }
                "InvalidNextTokenException" => {
                    return ListTypedLinkFacetAttributesError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return ListTypedLinkFacetAttributesError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return ListTypedLinkFacetAttributesError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "RetryableConflictException" => {
                    return ListTypedLinkFacetAttributesError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListTypedLinkFacetAttributesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTypedLinkFacetAttributesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTypedLinkFacetAttributesError {
    fn from(err: serde_json::error::Error) -> ListTypedLinkFacetAttributesError {
        ListTypedLinkFacetAttributesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTypedLinkFacetAttributesError {
    fn from(err: CredentialsError) -> ListTypedLinkFacetAttributesError {
        ListTypedLinkFacetAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTypedLinkFacetAttributesError {
    fn from(err: HttpDispatchError) -> ListTypedLinkFacetAttributesError {
        ListTypedLinkFacetAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTypedLinkFacetAttributesError {
    fn from(err: io::Error) -> ListTypedLinkFacetAttributesError {
        ListTypedLinkFacetAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTypedLinkFacetAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTypedLinkFacetAttributesError {
    fn description(&self) -> &str {
        match *self {
            ListTypedLinkFacetAttributesError::AccessDenied(ref cause) => cause,
            ListTypedLinkFacetAttributesError::FacetNotFound(ref cause) => cause,
            ListTypedLinkFacetAttributesError::InternalService(ref cause) => cause,
            ListTypedLinkFacetAttributesError::InvalidArn(ref cause) => cause,
            ListTypedLinkFacetAttributesError::InvalidNextToken(ref cause) => cause,
            ListTypedLinkFacetAttributesError::LimitExceeded(ref cause) => cause,
            ListTypedLinkFacetAttributesError::ResourceNotFound(ref cause) => cause,
            ListTypedLinkFacetAttributesError::RetryableConflict(ref cause) => cause,
            ListTypedLinkFacetAttributesError::Validation(ref cause) => cause,
            ListTypedLinkFacetAttributesError::Credentials(ref err) => err.description(),
            ListTypedLinkFacetAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTypedLinkFacetAttributesError::ParseError(ref cause) => cause,
            ListTypedLinkFacetAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTypedLinkFacetNames
#[derive(Debug, PartialEq)]
pub enum ListTypedLinkFacetNamesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListTypedLinkFacetNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTypedLinkFacetNamesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListTypedLinkFacetNamesError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return ListTypedLinkFacetNamesError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidArnException" => {
                    return ListTypedLinkFacetNamesError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return ListTypedLinkFacetNamesError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return ListTypedLinkFacetNamesError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListTypedLinkFacetNamesError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "RetryableConflictException" => {
                    return ListTypedLinkFacetNamesError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListTypedLinkFacetNamesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTypedLinkFacetNamesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTypedLinkFacetNamesError {
    fn from(err: serde_json::error::Error) -> ListTypedLinkFacetNamesError {
        ListTypedLinkFacetNamesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTypedLinkFacetNamesError {
    fn from(err: CredentialsError) -> ListTypedLinkFacetNamesError {
        ListTypedLinkFacetNamesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTypedLinkFacetNamesError {
    fn from(err: HttpDispatchError) -> ListTypedLinkFacetNamesError {
        ListTypedLinkFacetNamesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTypedLinkFacetNamesError {
    fn from(err: io::Error) -> ListTypedLinkFacetNamesError {
        ListTypedLinkFacetNamesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTypedLinkFacetNamesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTypedLinkFacetNamesError {
    fn description(&self) -> &str {
        match *self {
            ListTypedLinkFacetNamesError::AccessDenied(ref cause) => cause,
            ListTypedLinkFacetNamesError::InternalService(ref cause) => cause,
            ListTypedLinkFacetNamesError::InvalidArn(ref cause) => cause,
            ListTypedLinkFacetNamesError::InvalidNextToken(ref cause) => cause,
            ListTypedLinkFacetNamesError::LimitExceeded(ref cause) => cause,
            ListTypedLinkFacetNamesError::ResourceNotFound(ref cause) => cause,
            ListTypedLinkFacetNamesError::RetryableConflict(ref cause) => cause,
            ListTypedLinkFacetNamesError::Validation(ref cause) => cause,
            ListTypedLinkFacetNamesError::Credentials(ref err) => err.description(),
            ListTypedLinkFacetNamesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTypedLinkFacetNamesError::ParseError(ref cause) => cause,
            ListTypedLinkFacetNamesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by LookupPolicy
#[derive(Debug, PartialEq)]
pub enum LookupPolicyError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl LookupPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> LookupPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return LookupPolicyError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return LookupPolicyError::DirectoryNotEnabled(String::from(error_message))
                }
                "InternalServiceException" => {
                    return LookupPolicyError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return LookupPolicyError::InvalidArn(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return LookupPolicyError::InvalidNextToken(String::from(error_message))
                }
                "LimitExceededException" => {
                    return LookupPolicyError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return LookupPolicyError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return LookupPolicyError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return LookupPolicyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return LookupPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for LookupPolicyError {
    fn from(err: serde_json::error::Error) -> LookupPolicyError {
        LookupPolicyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for LookupPolicyError {
    fn from(err: CredentialsError) -> LookupPolicyError {
        LookupPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for LookupPolicyError {
    fn from(err: HttpDispatchError) -> LookupPolicyError {
        LookupPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for LookupPolicyError {
    fn from(err: io::Error) -> LookupPolicyError {
        LookupPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for LookupPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for LookupPolicyError {
    fn description(&self) -> &str {
        match *self {
            LookupPolicyError::AccessDenied(ref cause) => cause,
            LookupPolicyError::DirectoryNotEnabled(ref cause) => cause,
            LookupPolicyError::InternalService(ref cause) => cause,
            LookupPolicyError::InvalidArn(ref cause) => cause,
            LookupPolicyError::InvalidNextToken(ref cause) => cause,
            LookupPolicyError::LimitExceeded(ref cause) => cause,
            LookupPolicyError::ResourceNotFound(ref cause) => cause,
            LookupPolicyError::RetryableConflict(ref cause) => cause,
            LookupPolicyError::Validation(ref cause) => cause,
            LookupPolicyError::Credentials(ref err) => err.description(),
            LookupPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            LookupPolicyError::ParseError(ref cause) => cause,
            LookupPolicyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PublishSchema
#[derive(Debug, PartialEq)]
pub enum PublishSchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>Indicates that a schema is already published.</p>
    SchemaAlreadyPublished(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PublishSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> PublishSchemaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return PublishSchemaError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return PublishSchemaError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return PublishSchemaError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return PublishSchemaError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return PublishSchemaError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return PublishSchemaError::RetryableConflict(String::from(error_message))
                }
                "SchemaAlreadyPublishedException" => {
                    return PublishSchemaError::SchemaAlreadyPublished(String::from(error_message))
                }
                "ValidationException" => {
                    return PublishSchemaError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PublishSchemaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PublishSchemaError {
    fn from(err: serde_json::error::Error) -> PublishSchemaError {
        PublishSchemaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PublishSchemaError {
    fn from(err: CredentialsError) -> PublishSchemaError {
        PublishSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PublishSchemaError {
    fn from(err: HttpDispatchError) -> PublishSchemaError {
        PublishSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for PublishSchemaError {
    fn from(err: io::Error) -> PublishSchemaError {
        PublishSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PublishSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PublishSchemaError {
    fn description(&self) -> &str {
        match *self {
            PublishSchemaError::AccessDenied(ref cause) => cause,
            PublishSchemaError::InternalService(ref cause) => cause,
            PublishSchemaError::InvalidArn(ref cause) => cause,
            PublishSchemaError::LimitExceeded(ref cause) => cause,
            PublishSchemaError::ResourceNotFound(ref cause) => cause,
            PublishSchemaError::RetryableConflict(ref cause) => cause,
            PublishSchemaError::SchemaAlreadyPublished(ref cause) => cause,
            PublishSchemaError::Validation(ref cause) => cause,
            PublishSchemaError::Credentials(ref err) => err.description(),
            PublishSchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PublishSchemaError::ParseError(ref cause) => cause,
            PublishSchemaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutSchemaFromJson
#[derive(Debug, PartialEq)]
pub enum PutSchemaFromJsonError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Occurs when any of the rule parameter keys or values are invalid.</p>
    InvalidRule(String),
    /// <p>Indicates that the provided <code>SchemaDoc</code> value is not valid.</p>
    InvalidSchemaDoc(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutSchemaFromJsonError {
    pub fn from_response(res: BufferedHttpResponse) -> PutSchemaFromJsonError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return PutSchemaFromJsonError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return PutSchemaFromJsonError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return PutSchemaFromJsonError::InvalidArn(String::from(error_message))
                }
                "InvalidRuleException" => {
                    return PutSchemaFromJsonError::InvalidRule(String::from(error_message))
                }
                "InvalidSchemaDocException" => {
                    return PutSchemaFromJsonError::InvalidSchemaDoc(String::from(error_message))
                }
                "LimitExceededException" => {
                    return PutSchemaFromJsonError::LimitExceeded(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return PutSchemaFromJsonError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return PutSchemaFromJsonError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutSchemaFromJsonError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutSchemaFromJsonError {
    fn from(err: serde_json::error::Error) -> PutSchemaFromJsonError {
        PutSchemaFromJsonError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutSchemaFromJsonError {
    fn from(err: CredentialsError) -> PutSchemaFromJsonError {
        PutSchemaFromJsonError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutSchemaFromJsonError {
    fn from(err: HttpDispatchError) -> PutSchemaFromJsonError {
        PutSchemaFromJsonError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutSchemaFromJsonError {
    fn from(err: io::Error) -> PutSchemaFromJsonError {
        PutSchemaFromJsonError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutSchemaFromJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutSchemaFromJsonError {
    fn description(&self) -> &str {
        match *self {
            PutSchemaFromJsonError::AccessDenied(ref cause) => cause,
            PutSchemaFromJsonError::InternalService(ref cause) => cause,
            PutSchemaFromJsonError::InvalidArn(ref cause) => cause,
            PutSchemaFromJsonError::InvalidRule(ref cause) => cause,
            PutSchemaFromJsonError::InvalidSchemaDoc(ref cause) => cause,
            PutSchemaFromJsonError::LimitExceeded(ref cause) => cause,
            PutSchemaFromJsonError::RetryableConflict(ref cause) => cause,
            PutSchemaFromJsonError::Validation(ref cause) => cause,
            PutSchemaFromJsonError::Credentials(ref err) => err.description(),
            PutSchemaFromJsonError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutSchemaFromJsonError::ParseError(ref cause) => cause,
            PutSchemaFromJsonError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RemoveFacetFromObject
#[derive(Debug, PartialEq)]
pub enum RemoveFacetFromObjectError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RemoveFacetFromObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RemoveFacetFromObjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return RemoveFacetFromObjectError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return RemoveFacetFromObjectError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "FacetValidationException" => {
                    return RemoveFacetFromObjectError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return RemoveFacetFromObjectError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return RemoveFacetFromObjectError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return RemoveFacetFromObjectError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return RemoveFacetFromObjectError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return RemoveFacetFromObjectError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return RemoveFacetFromObjectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RemoveFacetFromObjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RemoveFacetFromObjectError {
    fn from(err: serde_json::error::Error) -> RemoveFacetFromObjectError {
        RemoveFacetFromObjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveFacetFromObjectError {
    fn from(err: CredentialsError) -> RemoveFacetFromObjectError {
        RemoveFacetFromObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveFacetFromObjectError {
    fn from(err: HttpDispatchError) -> RemoveFacetFromObjectError {
        RemoveFacetFromObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveFacetFromObjectError {
    fn from(err: io::Error) -> RemoveFacetFromObjectError {
        RemoveFacetFromObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveFacetFromObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveFacetFromObjectError {
    fn description(&self) -> &str {
        match *self {
            RemoveFacetFromObjectError::AccessDenied(ref cause) => cause,
            RemoveFacetFromObjectError::DirectoryNotEnabled(ref cause) => cause,
            RemoveFacetFromObjectError::FacetValidation(ref cause) => cause,
            RemoveFacetFromObjectError::InternalService(ref cause) => cause,
            RemoveFacetFromObjectError::InvalidArn(ref cause) => cause,
            RemoveFacetFromObjectError::LimitExceeded(ref cause) => cause,
            RemoveFacetFromObjectError::ResourceNotFound(ref cause) => cause,
            RemoveFacetFromObjectError::RetryableConflict(ref cause) => cause,
            RemoveFacetFromObjectError::Validation(ref cause) => cause,
            RemoveFacetFromObjectError::Credentials(ref err) => err.description(),
            RemoveFacetFromObjectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveFacetFromObjectError::ParseError(ref cause) => cause,
            RemoveFacetFromObjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Can occur for multiple reasons such as when you tag a resource that doesn’t exist or if you specify a higher number of tags for a resource than the allowed limit. Allowed limit is 50 tags per resource.</p>
    InvalidTaggingRequest(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> TagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return TagResourceError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return TagResourceError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return TagResourceError::InvalidArn(String::from(error_message))
                }
                "InvalidTaggingRequestException" => {
                    return TagResourceError::InvalidTaggingRequest(String::from(error_message))
                }
                "LimitExceededException" => {
                    return TagResourceError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return TagResourceError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return TagResourceError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return TagResourceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return TagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::AccessDenied(ref cause) => cause,
            TagResourceError::InternalService(ref cause) => cause,
            TagResourceError::InvalidArn(ref cause) => cause,
            TagResourceError::InvalidTaggingRequest(ref cause) => cause,
            TagResourceError::LimitExceeded(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::RetryableConflict(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::ParseError(ref cause) => cause,
            TagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Can occur for multiple reasons such as when you tag a resource that doesn’t exist or if you specify a higher number of tags for a resource than the allowed limit. Allowed limit is 50 tags per resource.</p>
    InvalidTaggingRequest(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> UntagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return UntagResourceError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return UntagResourceError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return UntagResourceError::InvalidArn(String::from(error_message))
                }
                "InvalidTaggingRequestException" => {
                    return UntagResourceError::InvalidTaggingRequest(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UntagResourceError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UntagResourceError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return UntagResourceError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return UntagResourceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UntagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::AccessDenied(ref cause) => cause,
            UntagResourceError::InternalService(ref cause) => cause,
            UntagResourceError::InvalidArn(ref cause) => cause,
            UntagResourceError::InvalidTaggingRequest(ref cause) => cause,
            UntagResourceError::LimitExceeded(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::RetryableConflict(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::ParseError(ref cause) => cause,
            UntagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateFacet
#[derive(Debug, PartialEq)]
pub enum UpdateFacetError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>An attempt to modify a <a>Facet</a> resulted in an invalid schema exception.</p>
    InvalidFacetUpdate(String),
    /// <p>Occurs when any of the rule parameter keys or values are invalid.</p>
    InvalidRule(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateFacetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return UpdateFacetError::AccessDenied(String::from(error_message))
                }
                "FacetNotFoundException" => {
                    return UpdateFacetError::FacetNotFound(String::from(error_message))
                }
                "InternalServiceException" => {
                    return UpdateFacetError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return UpdateFacetError::InvalidArn(String::from(error_message))
                }
                "InvalidFacetUpdateException" => {
                    return UpdateFacetError::InvalidFacetUpdate(String::from(error_message))
                }
                "InvalidRuleException" => {
                    return UpdateFacetError::InvalidRule(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UpdateFacetError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UpdateFacetError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return UpdateFacetError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateFacetError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateFacetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateFacetError {
    fn from(err: serde_json::error::Error) -> UpdateFacetError {
        UpdateFacetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFacetError {
    fn from(err: CredentialsError) -> UpdateFacetError {
        UpdateFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFacetError {
    fn from(err: HttpDispatchError) -> UpdateFacetError {
        UpdateFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFacetError {
    fn from(err: io::Error) -> UpdateFacetError {
        UpdateFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFacetError {
    fn description(&self) -> &str {
        match *self {
            UpdateFacetError::AccessDenied(ref cause) => cause,
            UpdateFacetError::FacetNotFound(ref cause) => cause,
            UpdateFacetError::InternalService(ref cause) => cause,
            UpdateFacetError::InvalidArn(ref cause) => cause,
            UpdateFacetError::InvalidFacetUpdate(ref cause) => cause,
            UpdateFacetError::InvalidRule(ref cause) => cause,
            UpdateFacetError::LimitExceeded(ref cause) => cause,
            UpdateFacetError::ResourceNotFound(ref cause) => cause,
            UpdateFacetError::RetryableConflict(ref cause) => cause,
            UpdateFacetError::Validation(ref cause) => cause,
            UpdateFacetError::Credentials(ref err) => err.description(),
            UpdateFacetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateFacetError::ParseError(ref cause) => cause,
            UpdateFacetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateLinkAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateLinkAttributesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateLinkAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateLinkAttributesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return UpdateLinkAttributesError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return UpdateLinkAttributesError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "FacetValidationException" => {
                    return UpdateLinkAttributesError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return UpdateLinkAttributesError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return UpdateLinkAttributesError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UpdateLinkAttributesError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UpdateLinkAttributesError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return UpdateLinkAttributesError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateLinkAttributesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateLinkAttributesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateLinkAttributesError {
    fn from(err: serde_json::error::Error) -> UpdateLinkAttributesError {
        UpdateLinkAttributesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateLinkAttributesError {
    fn from(err: CredentialsError) -> UpdateLinkAttributesError {
        UpdateLinkAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateLinkAttributesError {
    fn from(err: HttpDispatchError) -> UpdateLinkAttributesError {
        UpdateLinkAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateLinkAttributesError {
    fn from(err: io::Error) -> UpdateLinkAttributesError {
        UpdateLinkAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateLinkAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLinkAttributesError {
    fn description(&self) -> &str {
        match *self {
            UpdateLinkAttributesError::AccessDenied(ref cause) => cause,
            UpdateLinkAttributesError::DirectoryNotEnabled(ref cause) => cause,
            UpdateLinkAttributesError::FacetValidation(ref cause) => cause,
            UpdateLinkAttributesError::InternalService(ref cause) => cause,
            UpdateLinkAttributesError::InvalidArn(ref cause) => cause,
            UpdateLinkAttributesError::LimitExceeded(ref cause) => cause,
            UpdateLinkAttributesError::ResourceNotFound(ref cause) => cause,
            UpdateLinkAttributesError::RetryableConflict(ref cause) => cause,
            UpdateLinkAttributesError::Validation(ref cause) => cause,
            UpdateLinkAttributesError::Credentials(ref err) => err.description(),
            UpdateLinkAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateLinkAttributesError::ParseError(ref cause) => cause,
            UpdateLinkAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateObjectAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateObjectAttributesError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Operations are only permitted on enabled directories.</p>
    DirectoryNotEnabled(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateObjectAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateObjectAttributesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return UpdateObjectAttributesError::AccessDenied(String::from(error_message))
                }
                "DirectoryNotEnabledException" => {
                    return UpdateObjectAttributesError::DirectoryNotEnabled(String::from(
                        error_message,
                    ))
                }
                "FacetValidationException" => {
                    return UpdateObjectAttributesError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return UpdateObjectAttributesError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return UpdateObjectAttributesError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UpdateObjectAttributesError::LimitExceeded(String::from(error_message))
                }
                "LinkNameAlreadyInUseException" => {
                    return UpdateObjectAttributesError::LinkNameAlreadyInUse(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return UpdateObjectAttributesError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "RetryableConflictException" => {
                    return UpdateObjectAttributesError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateObjectAttributesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateObjectAttributesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateObjectAttributesError {
    fn from(err: serde_json::error::Error) -> UpdateObjectAttributesError {
        UpdateObjectAttributesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateObjectAttributesError {
    fn from(err: CredentialsError) -> UpdateObjectAttributesError {
        UpdateObjectAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateObjectAttributesError {
    fn from(err: HttpDispatchError) -> UpdateObjectAttributesError {
        UpdateObjectAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateObjectAttributesError {
    fn from(err: io::Error) -> UpdateObjectAttributesError {
        UpdateObjectAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateObjectAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateObjectAttributesError {
    fn description(&self) -> &str {
        match *self {
            UpdateObjectAttributesError::AccessDenied(ref cause) => cause,
            UpdateObjectAttributesError::DirectoryNotEnabled(ref cause) => cause,
            UpdateObjectAttributesError::FacetValidation(ref cause) => cause,
            UpdateObjectAttributesError::InternalService(ref cause) => cause,
            UpdateObjectAttributesError::InvalidArn(ref cause) => cause,
            UpdateObjectAttributesError::LimitExceeded(ref cause) => cause,
            UpdateObjectAttributesError::LinkNameAlreadyInUse(ref cause) => cause,
            UpdateObjectAttributesError::ResourceNotFound(ref cause) => cause,
            UpdateObjectAttributesError::RetryableConflict(ref cause) => cause,
            UpdateObjectAttributesError::Validation(ref cause) => cause,
            UpdateObjectAttributesError::Credentials(ref err) => err.description(),
            UpdateObjectAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateObjectAttributesError::ParseError(ref cause) => cause,
            UpdateObjectAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateSchema
#[derive(Debug, PartialEq)]
pub enum UpdateSchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateSchemaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return UpdateSchemaError::AccessDenied(String::from(error_message))
                }
                "InternalServiceException" => {
                    return UpdateSchemaError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return UpdateSchemaError::InvalidArn(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UpdateSchemaError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UpdateSchemaError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return UpdateSchemaError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateSchemaError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateSchemaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateSchemaError {
    fn from(err: serde_json::error::Error) -> UpdateSchemaError {
        UpdateSchemaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSchemaError {
    fn from(err: CredentialsError) -> UpdateSchemaError {
        UpdateSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSchemaError {
    fn from(err: HttpDispatchError) -> UpdateSchemaError {
        UpdateSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSchemaError {
    fn from(err: io::Error) -> UpdateSchemaError {
        UpdateSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSchemaError {
    fn description(&self) -> &str {
        match *self {
            UpdateSchemaError::AccessDenied(ref cause) => cause,
            UpdateSchemaError::InternalService(ref cause) => cause,
            UpdateSchemaError::InvalidArn(ref cause) => cause,
            UpdateSchemaError::LimitExceeded(ref cause) => cause,
            UpdateSchemaError::ResourceNotFound(ref cause) => cause,
            UpdateSchemaError::RetryableConflict(ref cause) => cause,
            UpdateSchemaError::Validation(ref cause) => cause,
            UpdateSchemaError::Credentials(ref err) => err.description(),
            UpdateSchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSchemaError::ParseError(ref cause) => cause,
            UpdateSchemaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateTypedLinkFacet
#[derive(Debug, PartialEq)]
pub enum UpdateTypedLinkFacetError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    /// <p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>An attempt to modify a <a>Facet</a> resulted in an invalid schema exception.</p>
    InvalidFacetUpdate(String),
    /// <p>Occurs when any of the rule parameter keys or values are invalid.</p>
    InvalidRule(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateTypedLinkFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateTypedLinkFacetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return UpdateTypedLinkFacetError::AccessDenied(String::from(error_message))
                }
                "FacetNotFoundException" => {
                    return UpdateTypedLinkFacetError::FacetNotFound(String::from(error_message))
                }
                "FacetValidationException" => {
                    return UpdateTypedLinkFacetError::FacetValidation(String::from(error_message))
                }
                "InternalServiceException" => {
                    return UpdateTypedLinkFacetError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return UpdateTypedLinkFacetError::InvalidArn(String::from(error_message))
                }
                "InvalidFacetUpdateException" => {
                    return UpdateTypedLinkFacetError::InvalidFacetUpdate(String::from(
                        error_message,
                    ))
                }
                "InvalidRuleException" => {
                    return UpdateTypedLinkFacetError::InvalidRule(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UpdateTypedLinkFacetError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UpdateTypedLinkFacetError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return UpdateTypedLinkFacetError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateTypedLinkFacetError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateTypedLinkFacetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateTypedLinkFacetError {
    fn from(err: serde_json::error::Error) -> UpdateTypedLinkFacetError {
        UpdateTypedLinkFacetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateTypedLinkFacetError {
    fn from(err: CredentialsError) -> UpdateTypedLinkFacetError {
        UpdateTypedLinkFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTypedLinkFacetError {
    fn from(err: HttpDispatchError) -> UpdateTypedLinkFacetError {
        UpdateTypedLinkFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTypedLinkFacetError {
    fn from(err: io::Error) -> UpdateTypedLinkFacetError {
        UpdateTypedLinkFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTypedLinkFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTypedLinkFacetError {
    fn description(&self) -> &str {
        match *self {
            UpdateTypedLinkFacetError::AccessDenied(ref cause) => cause,
            UpdateTypedLinkFacetError::FacetNotFound(ref cause) => cause,
            UpdateTypedLinkFacetError::FacetValidation(ref cause) => cause,
            UpdateTypedLinkFacetError::InternalService(ref cause) => cause,
            UpdateTypedLinkFacetError::InvalidArn(ref cause) => cause,
            UpdateTypedLinkFacetError::InvalidFacetUpdate(ref cause) => cause,
            UpdateTypedLinkFacetError::InvalidRule(ref cause) => cause,
            UpdateTypedLinkFacetError::LimitExceeded(ref cause) => cause,
            UpdateTypedLinkFacetError::ResourceNotFound(ref cause) => cause,
            UpdateTypedLinkFacetError::RetryableConflict(ref cause) => cause,
            UpdateTypedLinkFacetError::Validation(ref cause) => cause,
            UpdateTypedLinkFacetError::Credentials(ref err) => err.description(),
            UpdateTypedLinkFacetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateTypedLinkFacetError::ParseError(ref cause) => cause,
            UpdateTypedLinkFacetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpgradeAppliedSchema
#[derive(Debug, PartialEq)]
pub enum UpgradeAppliedSchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a failure occurred while performing a check for backward compatibility between the specified schema and the schema that is currently applied to the directory.</p>
    IncompatibleSchema(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that an attempt to attach an object with the same link name or to apply a schema with the same name has occurred. Rename the link or the schema and then try again.</p>
    InvalidAttachment(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpgradeAppliedSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> UpgradeAppliedSchemaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return UpgradeAppliedSchemaError::AccessDenied(String::from(error_message))
                }
                "IncompatibleSchemaException" => {
                    return UpgradeAppliedSchemaError::IncompatibleSchema(String::from(
                        error_message,
                    ))
                }
                "InternalServiceException" => {
                    return UpgradeAppliedSchemaError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return UpgradeAppliedSchemaError::InvalidArn(String::from(error_message))
                }
                "InvalidAttachmentException" => {
                    return UpgradeAppliedSchemaError::InvalidAttachment(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UpgradeAppliedSchemaError::ResourceNotFound(String::from(error_message))
                }
                "RetryableConflictException" => {
                    return UpgradeAppliedSchemaError::RetryableConflict(String::from(error_message))
                }
                "ValidationException" => {
                    return UpgradeAppliedSchemaError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpgradeAppliedSchemaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpgradeAppliedSchemaError {
    fn from(err: serde_json::error::Error) -> UpgradeAppliedSchemaError {
        UpgradeAppliedSchemaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpgradeAppliedSchemaError {
    fn from(err: CredentialsError) -> UpgradeAppliedSchemaError {
        UpgradeAppliedSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpgradeAppliedSchemaError {
    fn from(err: HttpDispatchError) -> UpgradeAppliedSchemaError {
        UpgradeAppliedSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpgradeAppliedSchemaError {
    fn from(err: io::Error) -> UpgradeAppliedSchemaError {
        UpgradeAppliedSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpgradeAppliedSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpgradeAppliedSchemaError {
    fn description(&self) -> &str {
        match *self {
            UpgradeAppliedSchemaError::AccessDenied(ref cause) => cause,
            UpgradeAppliedSchemaError::IncompatibleSchema(ref cause) => cause,
            UpgradeAppliedSchemaError::InternalService(ref cause) => cause,
            UpgradeAppliedSchemaError::InvalidArn(ref cause) => cause,
            UpgradeAppliedSchemaError::InvalidAttachment(ref cause) => cause,
            UpgradeAppliedSchemaError::ResourceNotFound(ref cause) => cause,
            UpgradeAppliedSchemaError::RetryableConflict(ref cause) => cause,
            UpgradeAppliedSchemaError::Validation(ref cause) => cause,
            UpgradeAppliedSchemaError::Credentials(ref err) => err.description(),
            UpgradeAppliedSchemaError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpgradeAppliedSchemaError::ParseError(ref cause) => cause,
            UpgradeAppliedSchemaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpgradePublishedSchema
#[derive(Debug, PartialEq)]
pub enum UpgradePublishedSchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a failure occurred while performing a check for backward compatibility between the specified schema and the schema that is currently applied to the directory.</p>
    IncompatibleSchema(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that an attempt to attach an object with the same link name or to apply a schema with the same name has occurred. Rename the link or the schema and then try again.</p>
    InvalidAttachment(String),
    /// <p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpgradePublishedSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> UpgradePublishedSchemaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return UpgradePublishedSchemaError::AccessDenied(String::from(error_message))
                }
                "IncompatibleSchemaException" => {
                    return UpgradePublishedSchemaError::IncompatibleSchema(String::from(
                        error_message,
                    ))
                }
                "InternalServiceException" => {
                    return UpgradePublishedSchemaError::InternalService(String::from(error_message))
                }
                "InvalidArnException" => {
                    return UpgradePublishedSchemaError::InvalidArn(String::from(error_message))
                }
                "InvalidAttachmentException" => {
                    return UpgradePublishedSchemaError::InvalidAttachment(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return UpgradePublishedSchemaError::LimitExceeded(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UpgradePublishedSchemaError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "RetryableConflictException" => {
                    return UpgradePublishedSchemaError::RetryableConflict(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpgradePublishedSchemaError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpgradePublishedSchemaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpgradePublishedSchemaError {
    fn from(err: serde_json::error::Error) -> UpgradePublishedSchemaError {
        UpgradePublishedSchemaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpgradePublishedSchemaError {
    fn from(err: CredentialsError) -> UpgradePublishedSchemaError {
        UpgradePublishedSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpgradePublishedSchemaError {
    fn from(err: HttpDispatchError) -> UpgradePublishedSchemaError {
        UpgradePublishedSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpgradePublishedSchemaError {
    fn from(err: io::Error) -> UpgradePublishedSchemaError {
        UpgradePublishedSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpgradePublishedSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpgradePublishedSchemaError {
    fn description(&self) -> &str {
        match *self {
            UpgradePublishedSchemaError::AccessDenied(ref cause) => cause,
            UpgradePublishedSchemaError::IncompatibleSchema(ref cause) => cause,
            UpgradePublishedSchemaError::InternalService(ref cause) => cause,
            UpgradePublishedSchemaError::InvalidArn(ref cause) => cause,
            UpgradePublishedSchemaError::InvalidAttachment(ref cause) => cause,
            UpgradePublishedSchemaError::LimitExceeded(ref cause) => cause,
            UpgradePublishedSchemaError::ResourceNotFound(ref cause) => cause,
            UpgradePublishedSchemaError::RetryableConflict(ref cause) => cause,
            UpgradePublishedSchemaError::Validation(ref cause) => cause,
            UpgradePublishedSchemaError::Credentials(ref err) => err.description(),
            UpgradePublishedSchemaError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpgradePublishedSchemaError::ParseError(ref cause) => cause,
            UpgradePublishedSchemaError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon CloudDirectory API. Amazon CloudDirectory clients implement this trait.
pub trait CloudDirectory {
    /// <p>Adds a new <a>Facet</a> to an object. An object can have more than one facet applied on it.</p>
    fn add_facet_to_object(
        &self,
        input: AddFacetToObjectRequest,
    ) -> RusotoFuture<AddFacetToObjectResponse, AddFacetToObjectError>;

    /// <p>Copies the input published schema, at the specified version, into the <a>Directory</a> with the same name and version as that of the published schema.</p>
    fn apply_schema(
        &self,
        input: ApplySchemaRequest,
    ) -> RusotoFuture<ApplySchemaResponse, ApplySchemaError>;

    /// <p><p>Attaches an existing object to another object. An object can be accessed in two ways:</p> <ol> <li> <p>Using the path</p> </li> <li> <p>Using <code>ObjectIdentifier</code> </p> </li> </ol></p>
    fn attach_object(
        &self,
        input: AttachObjectRequest,
    ) -> RusotoFuture<AttachObjectResponse, AttachObjectError>;

    /// <p>Attaches a policy object to a regular object. An object can have a limited number of attached policies.</p>
    fn attach_policy(
        &self,
        input: AttachPolicyRequest,
    ) -> RusotoFuture<AttachPolicyResponse, AttachPolicyError>;

    /// <p>Attaches the specified object to the specified index.</p>
    fn attach_to_index(
        &self,
        input: AttachToIndexRequest,
    ) -> RusotoFuture<AttachToIndexResponse, AttachToIndexError>;

    /// <p>Attaches a typed link to a specified source and target object. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn attach_typed_link(
        &self,
        input: AttachTypedLinkRequest,
    ) -> RusotoFuture<AttachTypedLinkResponse, AttachTypedLinkError>;

    /// <p>Performs all the read operations in a batch. </p>
    fn batch_read(
        &self,
        input: BatchReadRequest,
    ) -> RusotoFuture<BatchReadResponse, BatchReadError>;

    /// <p>Performs all the write operations in a batch. Either all the operations succeed or none.</p>
    fn batch_write(
        &self,
        input: BatchWriteRequest,
    ) -> RusotoFuture<BatchWriteResponse, BatchWriteError>;

    /// <p>Creates a <a>Directory</a> by copying the published schema into the directory. A directory cannot be created without a schema.</p>
    fn create_directory(
        &self,
        input: CreateDirectoryRequest,
    ) -> RusotoFuture<CreateDirectoryResponse, CreateDirectoryError>;

    /// <p>Creates a new <a>Facet</a> in a schema. Facet creation is allowed only in development or applied schemas.</p>
    fn create_facet(
        &self,
        input: CreateFacetRequest,
    ) -> RusotoFuture<CreateFacetResponse, CreateFacetError>;

    /// <p>Creates an index object. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_indexing.html">Indexing</a> for more information.</p>
    fn create_index(
        &self,
        input: CreateIndexRequest,
    ) -> RusotoFuture<CreateIndexResponse, CreateIndexError>;

    /// <p>Creates an object in a <a>Directory</a>. Additionally attaches the object to a parent, if a parent reference and <code>LinkName</code> is specified. An object is simply a collection of <a>Facet</a> attributes. You can also use this API call to create a policy object, if the facet from which you create the object is a policy facet. </p>
    fn create_object(
        &self,
        input: CreateObjectRequest,
    ) -> RusotoFuture<CreateObjectResponse, CreateObjectError>;

    /// <p><p>Creates a new schema in a development state. A schema can exist in three phases:</p> <ul> <li> <p> <i>Development:</i> This is a mutable phase of the schema. All new schemas are in the development phase. Once the schema is finalized, it can be published.</p> </li> <li> <p> <i>Published:</i> Published schemas are immutable and have a version associated with them.</p> </li> <li> <p> <i>Applied:</i> Applied schemas are mutable in a way that allows you to add new schema facets. You can also add new, nonrequired attributes to existing schema facets. You can apply only published schemas to directories. </p> </li> </ul></p>
    fn create_schema(
        &self,
        input: CreateSchemaRequest,
    ) -> RusotoFuture<CreateSchemaResponse, CreateSchemaError>;

    /// <p>Creates a <a>TypedLinkFacet</a>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn create_typed_link_facet(
        &self,
        input: CreateTypedLinkFacetRequest,
    ) -> RusotoFuture<CreateTypedLinkFacetResponse, CreateTypedLinkFacetError>;

    /// <p>Deletes a directory. Only disabled directories can be deleted. A deleted directory cannot be undone. Exercise extreme caution when deleting directories.</p>
    fn delete_directory(
        &self,
        input: DeleteDirectoryRequest,
    ) -> RusotoFuture<DeleteDirectoryResponse, DeleteDirectoryError>;

    /// <p>Deletes a given <a>Facet</a>. All attributes and <a>Rule</a>s that are associated with the facet will be deleted. Only development schema facets are allowed deletion.</p>
    fn delete_facet(
        &self,
        input: DeleteFacetRequest,
    ) -> RusotoFuture<DeleteFacetResponse, DeleteFacetError>;

    /// <p>Deletes an object and its associated attributes. Only objects with no children and no parents can be deleted.</p>
    fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> RusotoFuture<DeleteObjectResponse, DeleteObjectError>;

    /// <p>Deletes a given schema. Schemas in a development and published state can only be deleted. </p>
    fn delete_schema(
        &self,
        input: DeleteSchemaRequest,
    ) -> RusotoFuture<DeleteSchemaResponse, DeleteSchemaError>;

    /// <p>Deletes a <a>TypedLinkFacet</a>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn delete_typed_link_facet(
        &self,
        input: DeleteTypedLinkFacetRequest,
    ) -> RusotoFuture<DeleteTypedLinkFacetResponse, DeleteTypedLinkFacetError>;

    /// <p>Detaches the specified object from the specified index.</p>
    fn detach_from_index(
        &self,
        input: DetachFromIndexRequest,
    ) -> RusotoFuture<DetachFromIndexResponse, DetachFromIndexError>;

    /// <p>Detaches a given object from the parent object. The object that is to be detached from the parent is specified by the link name.</p>
    fn detach_object(
        &self,
        input: DetachObjectRequest,
    ) -> RusotoFuture<DetachObjectResponse, DetachObjectError>;

    /// <p>Detaches a policy from an object.</p>
    fn detach_policy(
        &self,
        input: DetachPolicyRequest,
    ) -> RusotoFuture<DetachPolicyResponse, DetachPolicyError>;

    /// <p>Detaches a typed link from a specified source and target object. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn detach_typed_link(
        &self,
        input: DetachTypedLinkRequest,
    ) -> RusotoFuture<(), DetachTypedLinkError>;

    /// <p>Disables the specified directory. Disabled directories cannot be read or written to. Only enabled directories can be disabled. Disabled directories may be reenabled.</p>
    fn disable_directory(
        &self,
        input: DisableDirectoryRequest,
    ) -> RusotoFuture<DisableDirectoryResponse, DisableDirectoryError>;

    /// <p>Enables the specified directory. Only disabled directories can be enabled. Once enabled, the directory can then be read and written to.</p>
    fn enable_directory(
        &self,
        input: EnableDirectoryRequest,
    ) -> RusotoFuture<EnableDirectoryResponse, EnableDirectoryError>;

    /// <p>Returns current applied schema version ARN, including the minor version in use.</p>
    fn get_applied_schema_version(
        &self,
        input: GetAppliedSchemaVersionRequest,
    ) -> RusotoFuture<GetAppliedSchemaVersionResponse, GetAppliedSchemaVersionError>;

    /// <p>Retrieves metadata about a directory.</p>
    fn get_directory(
        &self,
        input: GetDirectoryRequest,
    ) -> RusotoFuture<GetDirectoryResponse, GetDirectoryError>;

    /// <p>Gets details of the <a>Facet</a>, such as facet name, attributes, <a>Rule</a>s, or <code>ObjectType</code>. You can call this on all kinds of schema facets -- published, development, or applied.</p>
    fn get_facet(&self, input: GetFacetRequest) -> RusotoFuture<GetFacetResponse, GetFacetError>;

    /// <p>Retrieves attributes that are associated with a typed link.</p>
    fn get_link_attributes(
        &self,
        input: GetLinkAttributesRequest,
    ) -> RusotoFuture<GetLinkAttributesResponse, GetLinkAttributesError>;

    /// <p>Retrieves attributes within a facet that are associated with an object.</p>
    fn get_object_attributes(
        &self,
        input: GetObjectAttributesRequest,
    ) -> RusotoFuture<GetObjectAttributesResponse, GetObjectAttributesError>;

    /// <p>Retrieves metadata about an object.</p>
    fn get_object_information(
        &self,
        input: GetObjectInformationRequest,
    ) -> RusotoFuture<GetObjectInformationResponse, GetObjectInformationError>;

    /// <p>Retrieves a JSON representation of the schema. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_schemas.html#jsonformat">JSON Schema Format</a> for more information.</p>
    fn get_schema_as_json(
        &self,
        input: GetSchemaAsJsonRequest,
    ) -> RusotoFuture<GetSchemaAsJsonResponse, GetSchemaAsJsonError>;

    /// <p>Returns the identity attribute order for a specific <a>TypedLinkFacet</a>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn get_typed_link_facet_information(
        &self,
        input: GetTypedLinkFacetInformationRequest,
    ) -> RusotoFuture<GetTypedLinkFacetInformationResponse, GetTypedLinkFacetInformationError>;

    /// <p>Lists schema major versions applied to a directory. If <code>SchemaArn</code> is provided, lists the minor version.</p>
    fn list_applied_schema_arns(
        &self,
        input: ListAppliedSchemaArnsRequest,
    ) -> RusotoFuture<ListAppliedSchemaArnsResponse, ListAppliedSchemaArnsError>;

    /// <p>Lists indices attached to the specified object.</p>
    fn list_attached_indices(
        &self,
        input: ListAttachedIndicesRequest,
    ) -> RusotoFuture<ListAttachedIndicesResponse, ListAttachedIndicesError>;

    /// <p>Retrieves each Amazon Resource Name (ARN) of schemas in the development state.</p>
    fn list_development_schema_arns(
        &self,
        input: ListDevelopmentSchemaArnsRequest,
    ) -> RusotoFuture<ListDevelopmentSchemaArnsResponse, ListDevelopmentSchemaArnsError>;

    /// <p>Lists directories created within an account.</p>
    fn list_directories(
        &self,
        input: ListDirectoriesRequest,
    ) -> RusotoFuture<ListDirectoriesResponse, ListDirectoriesError>;

    /// <p>Retrieves attributes attached to the facet.</p>
    fn list_facet_attributes(
        &self,
        input: ListFacetAttributesRequest,
    ) -> RusotoFuture<ListFacetAttributesResponse, ListFacetAttributesError>;

    /// <p>Retrieves the names of facets that exist in a schema.</p>
    fn list_facet_names(
        &self,
        input: ListFacetNamesRequest,
    ) -> RusotoFuture<ListFacetNamesResponse, ListFacetNamesError>;

    /// <p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn list_incoming_typed_links(
        &self,
        input: ListIncomingTypedLinksRequest,
    ) -> RusotoFuture<ListIncomingTypedLinksResponse, ListIncomingTypedLinksError>;

    /// <p>Lists objects attached to the specified index.</p>
    fn list_index(
        &self,
        input: ListIndexRequest,
    ) -> RusotoFuture<ListIndexResponse, ListIndexError>;

    /// <p>Lists all attributes that are associated with an object. </p>
    fn list_object_attributes(
        &self,
        input: ListObjectAttributesRequest,
    ) -> RusotoFuture<ListObjectAttributesResponse, ListObjectAttributesError>;

    /// <p>Returns a paginated list of child objects that are associated with a given object.</p>
    fn list_object_children(
        &self,
        input: ListObjectChildrenRequest,
    ) -> RusotoFuture<ListObjectChildrenResponse, ListObjectChildrenError>;

    /// <p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#dirstructure">Directory Structure</a>.</p> <p>Use this API to evaluate all parents for an object. The call returns all objects from the root of the directory up to the requested object. The API returns the number of paths based on user-defined <code>MaxResults</code>, in case there are multiple paths to the parent. The order of the paths and nodes returned is consistent among multiple API calls unless the objects are deleted or moved. Paths not leading to the directory root are ignored from the target object.</p>
    fn list_object_parent_paths(
        &self,
        input: ListObjectParentPathsRequest,
    ) -> RusotoFuture<ListObjectParentPathsResponse, ListObjectParentPathsError>;

    /// <p>Lists parent objects that are associated with a given object in pagination fashion.</p>
    fn list_object_parents(
        &self,
        input: ListObjectParentsRequest,
    ) -> RusotoFuture<ListObjectParentsResponse, ListObjectParentsError>;

    /// <p>Returns policies attached to an object in pagination fashion.</p>
    fn list_object_policies(
        &self,
        input: ListObjectPoliciesRequest,
    ) -> RusotoFuture<ListObjectPoliciesResponse, ListObjectPoliciesError>;

    /// <p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn list_outgoing_typed_links(
        &self,
        input: ListOutgoingTypedLinksRequest,
    ) -> RusotoFuture<ListOutgoingTypedLinksResponse, ListOutgoingTypedLinksError>;

    /// <p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>
    fn list_policy_attachments(
        &self,
        input: ListPolicyAttachmentsRequest,
    ) -> RusotoFuture<ListPolicyAttachmentsResponse, ListPolicyAttachmentsError>;

    /// <p>Lists the major version families of each published schema. If a major version ARN is provided as <code>SchemaArn</code>, the minor version revisions in that family are listed instead.</p>
    fn list_published_schema_arns(
        &self,
        input: ListPublishedSchemaArnsRequest,
    ) -> RusotoFuture<ListPublishedSchemaArnsResponse, ListPublishedSchemaArnsError>;

    /// <p>Returns tags for a resource. Tagging is currently supported only for directories with a limit of 50 tags per directory. All 50 tags are returned for a given directory with this API call.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Returns a paginated list of all attribute definitions for a particular <a>TypedLinkFacet</a>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn list_typed_link_facet_attributes(
        &self,
        input: ListTypedLinkFacetAttributesRequest,
    ) -> RusotoFuture<ListTypedLinkFacetAttributesResponse, ListTypedLinkFacetAttributesError>;

    /// <p>Returns a paginated list of <code>TypedLink</code> facet names for a particular schema. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn list_typed_link_facet_names(
        &self,
        input: ListTypedLinkFacetNamesRequest,
    ) -> RusotoFuture<ListTypedLinkFacetNamesResponse, ListTypedLinkFacetNamesError>;

    /// <p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies">Policies</a>.</p>
    fn lookup_policy(
        &self,
        input: LookupPolicyRequest,
    ) -> RusotoFuture<LookupPolicyResponse, LookupPolicyError>;

    /// <p>Publishes a development schema with a major version and a recommended minor version.</p>
    fn publish_schema(
        &self,
        input: PublishSchemaRequest,
    ) -> RusotoFuture<PublishSchemaResponse, PublishSchemaError>;

    /// <p>Allows a schema to be updated using JSON upload. Only available for development schemas. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_schemas.html#jsonformat">JSON Schema Format</a> for more information.</p>
    fn put_schema_from_json(
        &self,
        input: PutSchemaFromJsonRequest,
    ) -> RusotoFuture<PutSchemaFromJsonResponse, PutSchemaFromJsonError>;

    /// <p>Removes the specified facet from the specified object.</p>
    fn remove_facet_from_object(
        &self,
        input: RemoveFacetFromObjectRequest,
    ) -> RusotoFuture<RemoveFacetFromObjectResponse, RemoveFacetFromObjectError>;

    /// <p>An API operation for adding tags to a resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>An API operation for removing tags from a resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p><p>Does the following:</p> <ol> <li> <p>Adds new <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Updates existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Deletes existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> </ol></p>
    fn update_facet(
        &self,
        input: UpdateFacetRequest,
    ) -> RusotoFuture<UpdateFacetResponse, UpdateFacetError>;

    /// <p>Updates a given typed link’s attributes. Attributes to be updated must not contribute to the typed link’s identity, as defined by its <code>IdentityAttributeOrder</code>.</p>
    fn update_link_attributes(
        &self,
        input: UpdateLinkAttributesRequest,
    ) -> RusotoFuture<UpdateLinkAttributesResponse, UpdateLinkAttributesError>;

    /// <p>Updates a given object's attributes.</p>
    fn update_object_attributes(
        &self,
        input: UpdateObjectAttributesRequest,
    ) -> RusotoFuture<UpdateObjectAttributesResponse, UpdateObjectAttributesError>;

    /// <p>Updates the schema name with a new name. Only development schema names can be updated.</p>
    fn update_schema(
        &self,
        input: UpdateSchemaRequest,
    ) -> RusotoFuture<UpdateSchemaResponse, UpdateSchemaError>;

    /// <p>Updates a <a>TypedLinkFacet</a>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn update_typed_link_facet(
        &self,
        input: UpdateTypedLinkFacetRequest,
    ) -> RusotoFuture<UpdateTypedLinkFacetResponse, UpdateTypedLinkFacetError>;

    /// <p>Upgrades a single directory in-place using the <code>PublishedSchemaArn</code> with schema updates found in <code>MinorVersion</code>. Backwards-compatible minor version upgrades are instantaneously available for readers on all objects in the directory. Note: This is a synchronous API call and upgrades only one schema on a given directory per call. To upgrade multiple directories from one schema, you would need to call this API on each directory.</p>
    fn upgrade_applied_schema(
        &self,
        input: UpgradeAppliedSchemaRequest,
    ) -> RusotoFuture<UpgradeAppliedSchemaResponse, UpgradeAppliedSchemaError>;

    /// <p>Upgrades a published schema under a new minor version revision using the current contents of <code>DevelopmentSchemaArn</code>.</p>
    fn upgrade_published_schema(
        &self,
        input: UpgradePublishedSchemaRequest,
    ) -> RusotoFuture<UpgradePublishedSchemaResponse, UpgradePublishedSchemaError>;
}
/// A client for the Amazon CloudDirectory API.
pub struct CloudDirectoryClient {
    client: Client,
    region: region::Region,
}

impl CloudDirectoryClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudDirectoryClient {
        CloudDirectoryClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudDirectoryClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CloudDirectoryClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CloudDirectory for CloudDirectoryClient {
    /// <p>Adds a new <a>Facet</a> to an object. An object can have more than one facet applied on it.</p>
    fn add_facet_to_object(
        &self,
        input: AddFacetToObjectRequest,
    ) -> RusotoFuture<AddFacetToObjectResponse, AddFacetToObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/facets";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AddFacetToObjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AddFacetToObjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Copies the input published schema, at the specified version, into the <a>Directory</a> with the same name and version as that of the published schema.</p>
    fn apply_schema(
        &self,
        input: ApplySchemaRequest,
    ) -> RusotoFuture<ApplySchemaResponse, ApplySchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/apply";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ApplySchemaResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ApplySchemaError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Attaches an existing object to another object. An object can be accessed in two ways:</p> <ol> <li> <p>Using the path</p> </li> <li> <p>Using <code>ObjectIdentifier</code> </p> </li> </ol></p>
    fn attach_object(
        &self,
        input: AttachObjectRequest,
    ) -> RusotoFuture<AttachObjectResponse, AttachObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AttachObjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AttachObjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Attaches a policy object to a regular object. An object can have a limited number of attached policies.</p>
    fn attach_policy(
        &self,
        input: AttachPolicyRequest,
    ) -> RusotoFuture<AttachPolicyResponse, AttachPolicyError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AttachPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AttachPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Attaches the specified object to the specified index.</p>
    fn attach_to_index(
        &self,
        input: AttachToIndexRequest,
    ) -> RusotoFuture<AttachToIndexResponse, AttachToIndexError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AttachToIndexResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AttachToIndexError::from_response(response))),
                )
            }
        })
    }

    /// <p>Attaches a typed link to a specified source and target object. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn attach_typed_link(
        &self,
        input: AttachTypedLinkRequest,
    ) -> RusotoFuture<AttachTypedLinkResponse, AttachTypedLinkError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AttachTypedLinkResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AttachTypedLinkError::from_response(response))),
                )
            }
        })
    }

    /// <p>Performs all the read operations in a batch. </p>
    fn batch_read(
        &self,
        input: BatchReadRequest,
    ) -> RusotoFuture<BatchReadResponse, BatchReadError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/batchread";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<BatchReadResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchReadError::from_response(response))),
                )
            }
        })
    }

    /// <p>Performs all the write operations in a batch. Either all the operations succeed or none.</p>
    fn batch_write(
        &self,
        input: BatchWriteRequest,
    ) -> RusotoFuture<BatchWriteResponse, BatchWriteError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/batchwrite";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<BatchWriteResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchWriteError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <a>Directory</a> by copying the published schema into the directory. A directory cannot be created without a schema.</p>
    fn create_directory(
        &self,
        input: CreateDirectoryRequest,
    ) -> RusotoFuture<CreateDirectoryResponse, CreateDirectoryError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateDirectoryResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDirectoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new <a>Facet</a> in a schema. Facet creation is allowed only in development or applied schemas.</p>
    fn create_facet(
        &self,
        input: CreateFacetRequest,
    ) -> RusotoFuture<CreateFacetResponse, CreateFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateFacetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateFacetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an index object. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_indexing.html">Indexing</a> for more information.</p>
    fn create_index(
        &self,
        input: CreateIndexRequest,
    ) -> RusotoFuture<CreateIndexResponse, CreateIndexError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateIndexResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateIndexError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an object in a <a>Directory</a>. Additionally attaches the object to a parent, if a parent reference and <code>LinkName</code> is specified. An object is simply a collection of <a>Facet</a> attributes. You can also use this API call to create a policy object, if the facet from which you create the object is a policy facet. </p>
    fn create_object(
        &self,
        input: CreateObjectRequest,
    ) -> RusotoFuture<CreateObjectResponse, CreateObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateObjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateObjectError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates a new schema in a development state. A schema can exist in three phases:</p> <ul> <li> <p> <i>Development:</i> This is a mutable phase of the schema. All new schemas are in the development phase. Once the schema is finalized, it can be published.</p> </li> <li> <p> <i>Published:</i> Published schemas are immutable and have a version associated with them.</p> </li> <li> <p> <i>Applied:</i> Applied schemas are mutable in a way that allows you to add new schema facets. You can also add new, nonrequired attributes to existing schema facets. You can apply only published schemas to directories. </p> </li> </ul></p>
    fn create_schema(
        &self,
        input: CreateSchemaRequest,
    ) -> RusotoFuture<CreateSchemaResponse, CreateSchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateSchemaResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateSchemaError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <a>TypedLinkFacet</a>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn create_typed_link_facet(
        &self,
        input: CreateTypedLinkFacetRequest,
    ) -> RusotoFuture<CreateTypedLinkFacetResponse, CreateTypedLinkFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateTypedLinkFacetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateTypedLinkFacetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a directory. Only disabled directories can be deleted. A deleted directory cannot be undone. Exercise extreme caution when deleting directories.</p>
    fn delete_directory(
        &self,
        input: DeleteDirectoryRequest,
    ) -> RusotoFuture<DeleteDirectoryResponse, DeleteDirectoryError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteDirectoryResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDirectoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a given <a>Facet</a>. All attributes and <a>Rule</a>s that are associated with the facet will be deleted. Only development schema facets are allowed deletion.</p>
    fn delete_facet(
        &self,
        input: DeleteFacetRequest,
    ) -> RusotoFuture<DeleteFacetResponse, DeleteFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteFacetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteFacetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an object and its associated attributes. Only objects with no children and no parents can be deleted.</p>
    fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> RusotoFuture<DeleteObjectResponse, DeleteObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteObjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteObjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a given schema. Schemas in a development and published state can only be deleted. </p>
    fn delete_schema(
        &self,
        input: DeleteSchemaRequest,
    ) -> RusotoFuture<DeleteSchemaResponse, DeleteSchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteSchemaResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteSchemaError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a <a>TypedLinkFacet</a>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn delete_typed_link_facet(
        &self,
        input: DeleteTypedLinkFacetRequest,
    ) -> RusotoFuture<DeleteTypedLinkFacetResponse, DeleteTypedLinkFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteTypedLinkFacetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteTypedLinkFacetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Detaches the specified object from the specified index.</p>
    fn detach_from_index(
        &self,
        input: DetachFromIndexRequest,
    ) -> RusotoFuture<DetachFromIndexResponse, DetachFromIndexError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DetachFromIndexResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetachFromIndexError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detaches a given object from the parent object. The object that is to be detached from the parent is specified by the link name.</p>
    fn detach_object(
        &self,
        input: DetachObjectRequest,
    ) -> RusotoFuture<DetachObjectResponse, DetachObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DetachObjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetachObjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detaches a policy from an object.</p>
    fn detach_policy(
        &self,
        input: DetachPolicyRequest,
    ) -> RusotoFuture<DetachPolicyResponse, DetachPolicyError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DetachPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetachPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detaches a typed link from a specified source and target object. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn detach_typed_link(
        &self,
        input: DetachTypedLinkRequest,
    ) -> RusotoFuture<(), DetachTypedLinkError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetachTypedLinkError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disables the specified directory. Disabled directories cannot be read or written to. Only enabled directories can be disabled. Disabled directories may be reenabled.</p>
    fn disable_directory(
        &self,
        input: DisableDirectoryRequest,
    ) -> RusotoFuture<DisableDirectoryResponse, DisableDirectoryError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/disable";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DisableDirectoryResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DisableDirectoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Enables the specified directory. Only disabled directories can be enabled. Once enabled, the directory can then be read and written to.</p>
    fn enable_directory(
        &self,
        input: EnableDirectoryRequest,
    ) -> RusotoFuture<EnableDirectoryResponse, EnableDirectoryError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/enable";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<EnableDirectoryResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(EnableDirectoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns current applied schema version ARN, including the minor version in use.</p>
    fn get_applied_schema_version(
        &self,
        input: GetAppliedSchemaVersionRequest,
    ) -> RusotoFuture<GetAppliedSchemaVersionResponse, GetAppliedSchemaVersionError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/getappliedschema";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetAppliedSchemaVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAppliedSchemaVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves metadata about a directory.</p>
    fn get_directory(
        &self,
        input: GetDirectoryRequest,
    ) -> RusotoFuture<GetDirectoryResponse, GetDirectoryError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/get";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetDirectoryResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDirectoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets details of the <a>Facet</a>, such as facet name, attributes, <a>Rule</a>s, or <code>ObjectType</code>. You can call this on all kinds of schema facets -- published, development, or applied.</p>
    fn get_facet(&self, input: GetFacetRequest) -> RusotoFuture<GetFacetResponse, GetFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetFacetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFacetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves attributes that are associated with a typed link.</p>
    fn get_link_attributes(
        &self,
        input: GetLinkAttributesRequest,
    ) -> RusotoFuture<GetLinkAttributesResponse, GetLinkAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/attributes/get";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetLinkAttributesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetLinkAttributesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves attributes within a facet that are associated with an object.</p>
    fn get_object_attributes(
        &self,
        input: GetObjectAttributesRequest,
    ) -> RusotoFuture<GetObjectAttributesResponse, GetObjectAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/attributes/get";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetObjectAttributesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetObjectAttributesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves metadata about an object.</p>
    fn get_object_information(
        &self,
        input: GetObjectInformationRequest,
    ) -> RusotoFuture<GetObjectInformationResponse, GetObjectInformationError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/information";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetObjectInformationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetObjectInformationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves a JSON representation of the schema. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_schemas.html#jsonformat">JSON Schema Format</a> for more information.</p>
    fn get_schema_as_json(
        &self,
        input: GetSchemaAsJsonRequest,
    ) -> RusotoFuture<GetSchemaAsJsonResponse, GetSchemaAsJsonError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/json";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetSchemaAsJsonResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSchemaAsJsonError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the identity attribute order for a specific <a>TypedLinkFacet</a>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn get_typed_link_facet_information(
        &self,
        input: GetTypedLinkFacetInformationRequest,
    ) -> RusotoFuture<GetTypedLinkFacetInformationResponse, GetTypedLinkFacetInformationError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/get";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetTypedLinkFacetInformationResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetTypedLinkFacetInformationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists schema major versions applied to a directory. If <code>SchemaArn</code> is provided, lists the minor version.</p>
    fn list_applied_schema_arns(
        &self,
        input: ListAppliedSchemaArnsRequest,
    ) -> RusotoFuture<ListAppliedSchemaArnsResponse, ListAppliedSchemaArnsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/applied";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListAppliedSchemaArnsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListAppliedSchemaArnsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists indices attached to the specified object.</p>
    fn list_attached_indices(
        &self,
        input: ListAttachedIndicesRequest,
    ) -> RusotoFuture<ListAttachedIndicesResponse, ListAttachedIndicesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/indices";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListAttachedIndicesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListAttachedIndicesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves each Amazon Resource Name (ARN) of schemas in the development state.</p>
    fn list_development_schema_arns(
        &self,
        input: ListDevelopmentSchemaArnsRequest,
    ) -> RusotoFuture<ListDevelopmentSchemaArnsResponse, ListDevelopmentSchemaArnsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/development";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListDevelopmentSchemaArnsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDevelopmentSchemaArnsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists directories created within an account.</p>
    fn list_directories(
        &self,
        input: ListDirectoriesRequest,
    ) -> RusotoFuture<ListDirectoriesResponse, ListDirectoriesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/list";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListDirectoriesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDirectoriesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves attributes attached to the facet.</p>
    fn list_facet_attributes(
        &self,
        input: ListFacetAttributesRequest,
    ) -> RusotoFuture<ListFacetAttributesResponse, ListFacetAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/attributes";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListFacetAttributesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListFacetAttributesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves the names of facets that exist in a schema.</p>
    fn list_facet_names(
        &self,
        input: ListFacetNamesRequest,
    ) -> RusotoFuture<ListFacetNamesResponse, ListFacetNamesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/list";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListFacetNamesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListFacetNamesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn list_incoming_typed_links(
        &self,
        input: ListIncomingTypedLinksRequest,
    ) -> RusotoFuture<ListIncomingTypedLinksResponse, ListIncomingTypedLinksError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/incoming";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListIncomingTypedLinksResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListIncomingTypedLinksError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists objects attached to the specified index.</p>
    fn list_index(
        &self,
        input: ListIndexRequest,
    ) -> RusotoFuture<ListIndexResponse, ListIndexError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index/targets";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListIndexResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListIndexError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all attributes that are associated with an object. </p>
    fn list_object_attributes(
        &self,
        input: ListObjectAttributesRequest,
    ) -> RusotoFuture<ListObjectAttributesResponse, ListObjectAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/attributes";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListObjectAttributesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListObjectAttributesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a paginated list of child objects that are associated with a given object.</p>
    fn list_object_children(
        &self,
        input: ListObjectChildrenRequest,
    ) -> RusotoFuture<ListObjectChildrenResponse, ListObjectChildrenError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/children";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListObjectChildrenResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListObjectChildrenError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#dirstructure">Directory Structure</a>.</p> <p>Use this API to evaluate all parents for an object. The call returns all objects from the root of the directory up to the requested object. The API returns the number of paths based on user-defined <code>MaxResults</code>, in case there are multiple paths to the parent. The order of the paths and nodes returned is consistent among multiple API calls unless the objects are deleted or moved. Paths not leading to the directory root are ignored from the target object.</p>
    fn list_object_parent_paths(
        &self,
        input: ListObjectParentPathsRequest,
    ) -> RusotoFuture<ListObjectParentPathsResponse, ListObjectParentPathsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/parentpaths";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListObjectParentPathsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListObjectParentPathsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists parent objects that are associated with a given object in pagination fashion.</p>
    fn list_object_parents(
        &self,
        input: ListObjectParentsRequest,
    ) -> RusotoFuture<ListObjectParentsResponse, ListObjectParentsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/parent";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListObjectParentsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListObjectParentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns policies attached to an object in pagination fashion.</p>
    fn list_object_policies(
        &self,
        input: ListObjectPoliciesRequest,
    ) -> RusotoFuture<ListObjectPoliciesResponse, ListObjectPoliciesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/policy";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListObjectPoliciesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListObjectPoliciesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn list_outgoing_typed_links(
        &self,
        input: ListOutgoingTypedLinksRequest,
    ) -> RusotoFuture<ListOutgoingTypedLinksResponse, ListOutgoingTypedLinksError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/outgoing";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListOutgoingTypedLinksResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListOutgoingTypedLinksError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>
    fn list_policy_attachments(
        &self,
        input: ListPolicyAttachmentsRequest,
    ) -> RusotoFuture<ListPolicyAttachmentsResponse, ListPolicyAttachmentsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/attachment";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListPolicyAttachmentsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListPolicyAttachmentsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the major version families of each published schema. If a major version ARN is provided as <code>SchemaArn</code>, the minor version revisions in that family are listed instead.</p>
    fn list_published_schema_arns(
        &self,
        input: ListPublishedSchemaArnsRequest,
    ) -> RusotoFuture<ListPublishedSchemaArnsResponse, ListPublishedSchemaArnsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/published";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListPublishedSchemaArnsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListPublishedSchemaArnsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns tags for a resource. Tagging is currently supported only for directories with a limit of 50 tags per directory. All 50 tags are returned for a given directory with this API call.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/tags";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListTagsForResourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a paginated list of all attribute definitions for a particular <a>TypedLinkFacet</a>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn list_typed_link_facet_attributes(
        &self,
        input: ListTypedLinkFacetAttributesRequest,
    ) -> RusotoFuture<ListTypedLinkFacetAttributesResponse, ListTypedLinkFacetAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/attributes";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListTypedLinkFacetAttributesResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListTypedLinkFacetAttributesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a paginated list of <code>TypedLink</code> facet names for a particular schema. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn list_typed_link_facet_names(
        &self,
        input: ListTypedLinkFacetNamesRequest,
    ) -> RusotoFuture<ListTypedLinkFacetNamesResponse, ListTypedLinkFacetNamesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/list";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListTypedLinkFacetNamesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListTypedLinkFacetNamesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies">Policies</a>.</p>
    fn lookup_policy(
        &self,
        input: LookupPolicyRequest,
    ) -> RusotoFuture<LookupPolicyResponse, LookupPolicyError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/lookup";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<LookupPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(LookupPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Publishes a development schema with a major version and a recommended minor version.</p>
    fn publish_schema(
        &self,
        input: PublishSchemaRequest,
    ) -> RusotoFuture<PublishSchemaResponse, PublishSchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/publish";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.development_schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<PublishSchemaResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PublishSchemaError::from_response(response))),
                )
            }
        })
    }

    /// <p>Allows a schema to be updated using JSON upload. Only available for development schemas. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_schemas.html#jsonformat">JSON Schema Format</a> for more information.</p>
    fn put_schema_from_json(
        &self,
        input: PutSchemaFromJsonRequest,
    ) -> RusotoFuture<PutSchemaFromJsonResponse, PutSchemaFromJsonError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/json";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<PutSchemaFromJsonResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutSchemaFromJsonError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes the specified facet from the specified object.</p>
    fn remove_facet_from_object(
        &self,
        input: RemoveFacetFromObjectRequest,
    ) -> RusotoFuture<RemoveFacetFromObjectResponse, RemoveFacetFromObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/facets/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<RemoveFacetFromObjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RemoveFacetFromObjectError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>An API operation for adding tags to a resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/tags/add";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<TagResourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>An API operation for removing tags from a resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/tags/remove";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UntagResourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Does the following:</p> <ol> <li> <p>Adds new <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Updates existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Deletes existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> </ol></p>
    fn update_facet(
        &self,
        input: UpdateFacetRequest,
    ) -> RusotoFuture<UpdateFacetResponse, UpdateFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateFacetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateFacetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a given typed link’s attributes. Attributes to be updated must not contribute to the typed link’s identity, as defined by its <code>IdentityAttributeOrder</code>.</p>
    fn update_link_attributes(
        &self,
        input: UpdateLinkAttributesRequest,
    ) -> RusotoFuture<UpdateLinkAttributesResponse, UpdateLinkAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/attributes/update";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateLinkAttributesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateLinkAttributesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates a given object's attributes.</p>
    fn update_object_attributes(
        &self,
        input: UpdateObjectAttributesRequest,
    ) -> RusotoFuture<UpdateObjectAttributesResponse, UpdateObjectAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/update";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateObjectAttributesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateObjectAttributesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the schema name with a new name. Only development schema names can be updated.</p>
    fn update_schema(
        &self,
        input: UpdateSchemaRequest,
    ) -> RusotoFuture<UpdateSchemaResponse, UpdateSchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/update";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateSchemaResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateSchemaError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a <a>TypedLinkFacet</a>. For more information, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink">Typed link</a>.</p>
    fn update_typed_link_facet(
        &self,
        input: UpdateTypedLinkFacetRequest,
    ) -> RusotoFuture<UpdateTypedLinkFacetResponse, UpdateTypedLinkFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateTypedLinkFacetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateTypedLinkFacetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Upgrades a single directory in-place using the <code>PublishedSchemaArn</code> with schema updates found in <code>MinorVersion</code>. Backwards-compatible minor version upgrades are instantaneously available for readers on all objects in the directory. Note: This is a synchronous API call and upgrades only one schema on a given directory per call. To upgrade multiple directories from one schema, you would need to call this API on each directory.</p>
    fn upgrade_applied_schema(
        &self,
        input: UpgradeAppliedSchemaRequest,
    ) -> RusotoFuture<UpgradeAppliedSchemaResponse, UpgradeAppliedSchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/upgradeapplied";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpgradeAppliedSchemaResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpgradeAppliedSchemaError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Upgrades a published schema under a new minor version revision using the current contents of <code>DevelopmentSchemaArn</code>.</p>
    fn upgrade_published_schema(
        &self,
        input: UpgradePublishedSchemaRequest,
    ) -> RusotoFuture<UpgradePublishedSchemaResponse, UpgradePublishedSchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/upgradepublished";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpgradePublishedSchemaResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpgradePublishedSchemaError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
