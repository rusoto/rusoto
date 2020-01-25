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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddFacetToObjectResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplySchemaRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> into which the schema is copied. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Published schema Amazon Resource Name (ARN) that needs to be copied. For more information, see <a>arns</a>.</p>
    #[serde(rename = "PublishedSchemaArn")]
    pub published_schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachObjectResponse {
    /// <p>The attached <code>ObjectIdentifier</code>, which is the child <code>ObjectIdentifier</code>.</p>
    #[serde(rename = "AttachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachPolicyResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachToIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the object that was attached to the index.</p>
    #[serde(rename = "AttachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAddFacetToObjectResponse {}

/// <p>Represents the output of an <a>AttachObject</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAttachObjectResponse {
    /// <p>The <code>ObjectIdentifier</code> of the object that has been attached.</p>
    #[serde(rename = "attachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

/// <p>Attaches a policy object to a regular object inside a <a>BatchRead</a> operation. For more information, see <a>AttachPolicy</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAttachPolicyResponse {}

/// <p>Attaches the specified object to the specified index inside a <a>BatchRead</a> operation. For more information, see <a>AttachToIndex</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAttachToIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the object that was attached to the index.</p>
    #[serde(rename = "AttachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

/// <p>Attaches a typed link to a specified source and target object inside a <a>BatchRead</a> operation. For more information, see <a>AttachTypedLink</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAttachTypedLinkResponse {
    /// <p>Returns a typed link specifier as output.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_link_specifier: Option<TypedLinkSpecifier>,
}

/// <p>Creates an index object inside of a <a>BatchRead</a> operation. For more information, see <a>CreateIndex</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchCreateIndex {
    /// <p>The batch reference name. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/transaction_support.html">Transaction Support</a> for more information.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchCreateIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the index created by this operation.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

/// <p>Represents the output of a <a>CreateObject</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchCreateObject {
    /// <p>The batch reference name. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/transaction_support.html">Transaction Support</a> for more information.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchCreateObjectResponse {
    /// <p>The ID that is associated with the object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

/// <p>Represents the output of a <a>DeleteObject</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDeleteObject {
    /// <p>The reference that identifies the object.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>DeleteObject</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteObjectResponse {}

/// <p>Detaches the specified object from the specified index inside a <a>BatchRead</a> operation. For more information, see <a>DetachFromIndex</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDetachFromIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the object that was detached from the index.</p>
    #[serde(rename = "DetachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

/// <p>Represents the output of a <a>DetachObject</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDetachObject {
    /// <p>The batch reference name. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/transaction_support.html">Transaction Support</a> for more information.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDetachObjectResponse {
    /// <p>The <code>ObjectIdentifier</code> of the detached object.</p>
    #[serde(rename = "detachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

/// <p>Detaches the specified policy from the specified directory inside a <a>BatchWrite</a> operation. For more information, see <a>DetachPolicy</a> and <a>BatchWriteRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDetachPolicyResponse {}

/// <p>Detaches a typed link from a specified source and target object inside a <a>BatchRead</a> operation. For more information, see <a>DetachTypedLink</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDetachTypedLink {
    /// <p>Used to accept a typed link specifier as input.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

/// <p>Represents the output of a <a>DetachTypedLink</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDetachTypedLinkResponse {}

/// <p>Retrieves attributes that are associated with a typed link inside a <a>BatchRead</a> operation. For more information, see <a>GetLinkAttributes</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetLinkAttributesResponse {
    /// <p>The attributes that are associated with the typed link.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

/// <p>Retrieves attributes within a facet that are associated with an object inside an <a>BatchRead</a> operation. For more information, see <a>GetObjectAttributes</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetObjectAttributesResponse {
    /// <p>The attribute values that are associated with an object.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

/// <p>Retrieves metadata about an object inside a <a>BatchRead</a> operation. For more information, see <a>GetObjectInformation</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetObjectInformation {
    /// <p>A reference to the object.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

/// <p>Represents the output of a <a>GetObjectInformation</a> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchListObjectParents {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchListObjectParentsResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParentLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_links: Option<Vec<ObjectIdentifierAndLinkNameTuple>>,
}

/// <p>Returns policies attached to an object in pagination fashion inside a <a>BatchRead</a> operation. For more information, see <a>ListObjectPolicies</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchLookupPolicyResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Provides list of path to policies. Policies contain <code>PolicyId</code>, <code>ObjectIdentifier</code>, and <code>PolicyType</code>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directory.html#key_concepts_policies">Policies</a>.</p>
    #[serde(rename = "PolicyToPathList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_to_path_list: Option<Vec<PolicyToPath>>,
}

/// <p>The batch read exception structure, which contains the exception type and message.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
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
    /// <p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directorystructure.html">Directory Structure</a>.</p>
    #[serde(rename = "ListObjectParentPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_parent_paths: Option<BatchListObjectParentPaths>,
    #[serde(rename = "ListObjectParents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_parents: Option<BatchListObjectParents>,
    /// <p>Returns policies attached to an object in pagination fashion.</p>
    #[serde(rename = "ListObjectPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_policies: Option<BatchListObjectPolicies>,
    /// <p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    #[serde(rename = "ListOutgoingTypedLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_outgoing_typed_links: Option<BatchListOutgoingTypedLinks>,
    /// <p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>
    #[serde(rename = "ListPolicyAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_policy_attachments: Option<BatchListPolicyAttachments>,
    /// <p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directory.html#key_concepts_policies">Policies</a>.</p>
    #[serde(rename = "LookupPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_policy: Option<BatchLookupPolicy>,
}

/// <p>Represents the output of a <code>BatchRead</code> response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchReadResponse {
    /// <p>A list of all the responses for each batch read.</p>
    #[serde(rename = "Responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<BatchReadOperationResponse>>,
}

/// <p>Represents the output of a <code>BatchRead</code> success response operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
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
    /// <p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directorystructure.html">Directory Structure</a>.</p>
    #[serde(rename = "ListObjectParentPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_parent_paths: Option<BatchListObjectParentPathsResponse>,
    #[serde(rename = "ListObjectParents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_parents: Option<BatchListObjectParentsResponse>,
    /// <p>Returns policies attached to an object in pagination fashion.</p>
    #[serde(rename = "ListObjectPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_policies: Option<BatchListObjectPoliciesResponse>,
    /// <p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    #[serde(rename = "ListOutgoingTypedLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_outgoing_typed_links: Option<BatchListOutgoingTypedLinksResponse>,
    /// <p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>
    #[serde(rename = "ListPolicyAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_policy_attachments: Option<BatchListPolicyAttachmentsResponse>,
    /// <p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directory.html#key_concepts_policies">Policies</a>.</p>
    #[serde(rename = "LookupPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_policy: Option<BatchLookupPolicyResponse>,
}

/// <p>A batch operation to remove a facet from an object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchRemoveFacetFromObjectResponse {}

/// <p>Updates a given typed link’s attributes inside a <a>BatchRead</a> operation. Attributes to be updated must not contribute to the typed link’s identity, as defined by its <code>IdentityAttributeOrder</code>. For more information, see <a>UpdateLinkAttributes</a> and <a>BatchReadRequest$Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateLinkAttributesResponse {}

/// <p>Represents the output of a <code>BatchUpdate</code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateObjectAttributesResponse {
    /// <p>ID that is associated with the object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

/// <p>Represents the output of a <code>BatchWrite</code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>Attaches a typed link to a specified source and target object. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    #[serde(rename = "AttachTypedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_typed_link: Option<BatchAttachTypedLink>,
    /// <p>Creates an index object. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/indexing_search.htm">Indexing and search</a> for more information.</p>
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
    /// <p>Detaches a typed link from a specified source and target object. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Attaches a typed link to a specified source and target object. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    #[serde(rename = "AttachTypedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_typed_link: Option<BatchAttachTypedLinkResponse>,
    /// <p>Creates an index object. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/indexing_search.htm">Indexing and search</a> for more information.</p>
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
    /// <p>Detaches a typed link from a specified source and target object. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchWriteRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>A list of operations that are part of the batch.</p>
    #[serde(rename = "Operations")]
    pub operations: Vec<BatchWriteOperation>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchWriteResponse {
    /// <p>A list of all the responses for each batch write.</p>
    #[serde(rename = "Responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<BatchWriteOperationResponse>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDirectoryRequest {
    /// <p>The name of the <a>Directory</a>. Should be unique per account, per region.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the published schema that will be copied into the data <a>Directory</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFacetRequest {
    /// <p>The attributes that are associated with the <a>Facet</a>.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<FacetAttribute>>,
    /// <p>There are two different styles that you can define on any given facet, <code>Static</code> and <code>Dynamic</code>. For static facets, all attributes must be defined in the schema. For dynamic facets, attributes can be defined during data plane operations.</p>
    #[serde(rename = "FacetStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_style: Option<String>,
    /// <p>The name of the <a>Facet</a>, which is unique for a given schema.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether a given object created from this facet is of type node, leaf node, policy or index.</p> <ul> <li> <p>Node: Can have multiple children but one parent.</p> </li> </ul> <ul> <li> <p>Leaf node: Cannot have children but can have multiple parents.</p> </li> </ul> <ul> <li> <p>Policy: Allows you to store a policy document and policy type. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directory.html#key_concepts_policies">Policies</a>.</p> </li> </ul> <ul> <li> <p>Index: Can be created with the Index API.</p> </li> </ul></p>
    #[serde(rename = "ObjectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    /// <p>The schema ARN in which the new <a>Facet</a> will be created. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the index created by this operation.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateObjectResponse {
    /// <p>The identifier that is associated with the object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSchemaRequest {
    /// <p>The name that is associated with the schema. This is unique to each account and in each region.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSchemaResponse {
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTypedLinkFacetRequest {
    /// <p> <a>Facet</a> structure that is associated with the typed link facet.</p>
    #[serde(rename = "Facet")]
    pub facet: TypedLinkFacet,
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTypedLinkFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDirectoryRequest {
    /// <p>The ARN of the directory to delete.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDirectoryResponse {
    /// <p>The ARN of the deleted directory.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFacetRequest {
    /// <p>The name of the facet to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Facet</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteObjectRequest {
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>A reference that identifies the object.</p>
    #[serde(rename = "ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteObjectResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSchemaRequest {
    /// <p>The Amazon Resource Name (ARN) of the development schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSchemaResponse {
    /// <p>The input ARN that is returned as part of the response. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTypedLinkFacetRequest {
    /// <p>The unique name of the typed link facet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTypedLinkFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetachFromIndexResponse {
    /// <p>The <code>ObjectIdentifier</code> of the object that was detached from the index.</p>
    #[serde(rename = "DetachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetachObjectResponse {
    /// <p>The <code>ObjectIdentifier</code> that was detached from the object.</p>
    #[serde(rename = "DetachedObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetachPolicyResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableDirectoryRequest {
    /// <p>The ARN of the directory to disable.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableDirectoryResponse {
    /// <p>The ARN of the directory that has been disabled.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableDirectoryRequest {
    /// <p>The ARN of the directory to enable.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableDirectoryResponse {
    /// <p>The ARN of the enabled directory.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

/// <p>A structure that contains <code>Name</code>, <code>ARN</code>, <code>Attributes</code>, <code> <a>Rule</a>s</code>, and <code>ObjectTypes</code>. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_whatarefacets.html">Facets</a> for more information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Facet {
    /// <p>There are two different styles that you can define on any given facet, <code>Static</code> and <code>Dynamic</code>. For static facets, all attributes must be defined in the schema. For dynamic facets, attributes can be defined during data plane operations.</p>
    #[serde(rename = "FacetStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_style: Option<String>,
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
    /// <p>A facet attribute consists of either a definition or a reference. This structure contains the attribute definition. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_attributereferences.html">Attribute References</a> for more information.</p>
    #[serde(rename = "AttributeDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definition: Option<FacetAttributeDefinition>,
    /// <p>An attribute reference that is associated with the attribute. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_attributereferences.html">Attribute References</a> for more information.</p>
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

/// <p>A facet attribute definition. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_attributereferences.html">Attribute References</a> for more information.</p>
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
    /// <p>The target attribute name that is associated with the facet reference. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_attributereferences.html">Attribute References</a> for more information.</p>
    #[serde(rename = "TargetAttributeName")]
    pub target_attribute_name: String,
    /// <p>The target facet name that is associated with the facet reference. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_attributereferences.html">Attribute References</a> for more information.</p>
    #[serde(rename = "TargetFacetName")]
    pub target_facet_name: String,
}

/// <p>A structure that contains information used to update an attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAppliedSchemaVersionRequest {
    /// <p>The ARN of the applied schema.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAppliedSchemaVersionResponse {
    /// <p>Current applied schema ARN, including the minor version in use if one was provided.</p>
    #[serde(rename = "AppliedSchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDirectoryRequest {
    /// <p>The ARN of the directory.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDirectoryResponse {
    /// <p>Metadata about the directory.</p>
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFacetRequest {
    /// <p>The name of the facet to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Facet</a>. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFacetResponse {
    /// <p>The <a>Facet</a> structure that is associated with the facet.</p>
    #[serde(rename = "Facet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet: Option<Facet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLinkAttributesRequest {
    /// <p>A list of attribute names whose values will be retrieved.</p>
    #[serde(rename = "AttributeNames")]
    pub attribute_names: Vec<String>,
    /// <p>The consistency level at which to retrieve the attributes on a typed link.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the typed link resides. For more information, see <a>arns</a> or <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLinkAttributesResponse {
    /// <p>The attributes that are associated with the typed link.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetObjectAttributesResponse {
    /// <p>The attributes that are associated with the object.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSchemaAsJsonRequest {
    /// <p>The ARN of the schema to retrieve.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTypedLinkFacetInformationRequest {
    /// <p>The unique name of the typed link facet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTypedLinkFacetInformationResponse {
    /// <p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed links considers the order that the attributes are defined on the typed link facet. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range. Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls. For more information about identity attributes, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    #[serde(rename = "IdentityAttributeOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_attribute_order: Option<Vec<String>>,
}

/// <p>Represents an index and an attached object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The maximum number of objects in a single page to retrieve from the index during a request. For more information, see <a href="http://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Amazon Cloud Directory Limits</a>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListManagedSchemaArnsRequest {
    /// <p>The maximum number of results to retrieve.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The response for ListManagedSchemaArns. When this parameter is used, all minor version ARNs for a major version are listed.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListManagedSchemaArnsResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARNs for all AWS managed schemas.</p>
    #[serde(rename = "SchemaArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListObjectParentsRequest {
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    #[serde(rename = "ConsistencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>When set to True, returns all <a>ListObjectParentsResponse$ParentLinks</a>. There could be multiple links between a parent-child pair.</p>
    #[serde(rename = "IncludeAllLinksToEachParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_links_to_each_parent: Option<bool>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListObjectParentsResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of parent reference and LinkName Tuples.</p>
    #[serde(rename = "ParentLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_links: Option<Vec<ObjectIdentifierAndLinkNameTuple>>,
    /// <p>The parent structure, which is a map with key as the <code>ObjectIdentifier</code> and LinkName as the value.</p>
    #[serde(rename = "Parents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LookupPolicyResponse {
    /// <p>The pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Provides list of path to policies. Policies contain <code>PolicyId</code>, <code>ObjectIdentifier</code>, and <code>PolicyType</code>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directory.html#key_concepts_policies">Policies</a>.</p>
    #[serde(rename = "PolicyToPathList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_to_path_list: Option<Vec<PolicyToPath>>,
}

/// <p>The action to take on the object attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>A pair of ObjectIdentifier and LinkName.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ObjectIdentifierAndLinkNameTuple {
    /// <p>The name of the link between the parent and the child object.</p>
    #[serde(rename = "LinkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    /// <p>The ID that is associated with the object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

/// <p>The reference that identifies an object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectReference {
    /// <p><p>A path selector supports easy selection of an object by the parent/child links leading to it from the directory root. Use the link names from each parent/child link to construct the path. Path selectors start with a slash (/) and link names are separated by slashes. For more information about paths, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_access_objects.html">Access Objects</a>. You can identify an object in one of the following ways:</p> <ul> <li> <p> <i>$ObjectIdentifier</i> - An object identifier is an opaque string provided by Amazon Cloud Directory. When creating objects, the system will provide you with the identifier of the created object. An object’s identifier is immutable and no two objects will ever share the same object identifier</p> </li> <li> <p> <i>/some/path</i> - Identifies the object based on path</p> </li> <li> <p> <i>#SomeBatchReference</i> - Identifies the object in a batch call</p> </li> </ul></p>
    #[serde(rename = "Selector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

/// <p>Returns the path to the <code>ObjectIdentifiers</code> that is associated with the directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Contains the <code>PolicyType</code>, <code>PolicyId</code>, and the <code>ObjectIdentifier</code> to which it is attached. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directory.html#key_concepts_policies">Policies</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PublishSchemaResponse {
    /// <p>The ARN that is associated with the published schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "PublishedSchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutSchemaFromJsonRequest {
    /// <p>The replacement JSON schema.</p>
    #[serde(rename = "Document")]
    pub document: String,
    /// <p>The ARN of the schema to update.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutSchemaFromJsonResponse {
    /// <p>The ARN of the schema to update.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The ARN of the schema that contains the facet with no minor component. See <a>arns</a> and <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_inplaceschemaupgrade.html">In-Place Schema Upgrade</a> for a description of when to provide minor versions.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource. Tagging is only supported for directories.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A list of tag key-value pairs.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_value: Option<bytes::Bytes>,
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

/// <p>A range of attribute values. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_range_filters.html">Range Filters</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource. Tagging is only supported for directories.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Keys of the tag that need to be removed from the resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLinkAttributesRequest {
    /// <p>The attributes update structure.</p>
    #[serde(rename = "AttributeUpdates")]
    pub attribute_updates: Vec<LinkAttributeUpdate>,
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the updated typed link resides. For more information, see <a>arns</a> or <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    #[serde(rename = "TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLinkAttributesResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateObjectAttributesResponse {
    /// <p>The <code>ObjectIdentifier</code> of the updated object.</p>
    #[serde(rename = "ObjectIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSchemaRequest {
    /// <p>The name of the schema.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the development schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSchemaResponse {
    /// <p>The ARN that is associated with the updated schema. For more information, see <a>arns</a>.</p>
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTypedLinkFacetRequest {
    /// <p>Attributes update structure.</p>
    #[serde(rename = "AttributeUpdates")]
    pub attribute_updates: Vec<TypedLinkFacetAttributeUpdate>,
    /// <p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed links considers the order that the attributes are defined on the typed link facet. When providing ranges to a typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range. Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls. For more information about identity attributes, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTypedLinkFacetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl AddFacetToObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddFacetToObjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AddFacetToObjectError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(AddFacetToObjectError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(AddFacetToObjectError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(AddFacetToObjectError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(AddFacetToObjectError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AddFacetToObjectError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddFacetToObjectError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(AddFacetToObjectError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AddFacetToObjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddFacetToObjectError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AddFacetToObjectError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            AddFacetToObjectError::FacetValidation(ref cause) => write!(f, "{}", cause),
            AddFacetToObjectError::InternalService(ref cause) => write!(f, "{}", cause),
            AddFacetToObjectError::InvalidArn(ref cause) => write!(f, "{}", cause),
            AddFacetToObjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AddFacetToObjectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AddFacetToObjectError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddFacetToObjectError {}
/// Errors returned by ApplySchema
#[derive(Debug, PartialEq)]
pub enum ApplySchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that an attempt to make an attachment was invalid. For example, attaching two nodes with a link type that is not applicable to the nodes or attempting to apply a schema to a directory a second time.</p>
    InvalidAttachment(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>Indicates that a schema could not be created due to a naming conflict. Please select a different name and then try again.</p>
    SchemaAlreadyExists(String),
}

impl ApplySchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ApplySchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ApplySchemaError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ApplySchemaError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ApplySchemaError::InvalidArn(err.msg))
                }
                "InvalidAttachmentException" => {
                    return RusotoError::Service(ApplySchemaError::InvalidAttachment(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ApplySchemaError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ApplySchemaError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ApplySchemaError::RetryableConflict(err.msg))
                }
                "SchemaAlreadyExistsException" => {
                    return RusotoError::Service(ApplySchemaError::SchemaAlreadyExists(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ApplySchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplySchemaError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ApplySchemaError::InternalService(ref cause) => write!(f, "{}", cause),
            ApplySchemaError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ApplySchemaError::InvalidAttachment(ref cause) => write!(f, "{}", cause),
            ApplySchemaError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ApplySchemaError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ApplySchemaError::RetryableConflict(ref cause) => write!(f, "{}", cause),
            ApplySchemaError::SchemaAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ApplySchemaError {}
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
    /// <p>Indicates that an attempt to make an attachment was invalid. For example, attaching two nodes with a link type that is not applicable to the nodes or attempting to apply a schema to a directory a second time.</p>
    InvalidAttachment(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl AttachObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachObjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachObjectError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(AttachObjectError::DirectoryNotEnabled(err.msg))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(AttachObjectError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(AttachObjectError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(AttachObjectError::InvalidArn(err.msg))
                }
                "InvalidAttachmentException" => {
                    return RusotoError::Service(AttachObjectError::InvalidAttachment(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AttachObjectError::LimitExceeded(err.msg))
                }
                "LinkNameAlreadyInUseException" => {
                    return RusotoError::Service(AttachObjectError::LinkNameAlreadyInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AttachObjectError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(AttachObjectError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AttachObjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachObjectError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AttachObjectError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            AttachObjectError::FacetValidation(ref cause) => write!(f, "{}", cause),
            AttachObjectError::InternalService(ref cause) => write!(f, "{}", cause),
            AttachObjectError::InvalidArn(ref cause) => write!(f, "{}", cause),
            AttachObjectError::InvalidAttachment(ref cause) => write!(f, "{}", cause),
            AttachObjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AttachObjectError::LinkNameAlreadyInUse(ref cause) => write!(f, "{}", cause),
            AttachObjectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AttachObjectError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachObjectError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation can only operate on policy objects.</p>
    NotPolicy(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl AttachPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachPolicyError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(AttachPolicyError::DirectoryNotEnabled(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(AttachPolicyError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(AttachPolicyError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AttachPolicyError::LimitExceeded(err.msg))
                }
                "NotPolicyException" => {
                    return RusotoError::Service(AttachPolicyError::NotPolicy(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AttachPolicyError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(AttachPolicyError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AttachPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::InternalService(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::NotPolicy(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachPolicyError {}
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
    /// <p>Indicates that an attempt to make an attachment was invalid. For example, attaching two nodes with a link type that is not applicable to the nodes or attempting to apply a schema to a directory a second time.</p>
    InvalidAttachment(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    /// <p>Indicates that the requested operation can only operate on index objects.</p>
    NotIndex(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl AttachToIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachToIndexError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachToIndexError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(AttachToIndexError::DirectoryNotEnabled(err.msg))
                }
                "IndexedAttributeMissingException" => {
                    return RusotoError::Service(AttachToIndexError::IndexedAttributeMissing(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(AttachToIndexError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(AttachToIndexError::InvalidArn(err.msg))
                }
                "InvalidAttachmentException" => {
                    return RusotoError::Service(AttachToIndexError::InvalidAttachment(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AttachToIndexError::LimitExceeded(err.msg))
                }
                "LinkNameAlreadyInUseException" => {
                    return RusotoError::Service(AttachToIndexError::LinkNameAlreadyInUse(err.msg))
                }
                "NotIndexException" => {
                    return RusotoError::Service(AttachToIndexError::NotIndex(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AttachToIndexError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(AttachToIndexError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AttachToIndexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachToIndexError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AttachToIndexError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            AttachToIndexError::IndexedAttributeMissing(ref cause) => write!(f, "{}", cause),
            AttachToIndexError::InternalService(ref cause) => write!(f, "{}", cause),
            AttachToIndexError::InvalidArn(ref cause) => write!(f, "{}", cause),
            AttachToIndexError::InvalidAttachment(ref cause) => write!(f, "{}", cause),
            AttachToIndexError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AttachToIndexError::LinkNameAlreadyInUse(ref cause) => write!(f, "{}", cause),
            AttachToIndexError::NotIndex(ref cause) => write!(f, "{}", cause),
            AttachToIndexError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AttachToIndexError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachToIndexError {}
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
    /// <p>Indicates that an attempt to make an attachment was invalid. For example, attaching two nodes with a link type that is not applicable to the nodes or attempting to apply a schema to a directory a second time.</p>
    InvalidAttachment(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl AttachTypedLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachTypedLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachTypedLinkError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(AttachTypedLinkError::DirectoryNotEnabled(err.msg))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(AttachTypedLinkError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(AttachTypedLinkError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(AttachTypedLinkError::InvalidArn(err.msg))
                }
                "InvalidAttachmentException" => {
                    return RusotoError::Service(AttachTypedLinkError::InvalidAttachment(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AttachTypedLinkError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AttachTypedLinkError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(AttachTypedLinkError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AttachTypedLinkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachTypedLinkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AttachTypedLinkError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            AttachTypedLinkError::FacetValidation(ref cause) => write!(f, "{}", cause),
            AttachTypedLinkError::InternalService(ref cause) => write!(f, "{}", cause),
            AttachTypedLinkError::InvalidArn(ref cause) => write!(f, "{}", cause),
            AttachTypedLinkError::InvalidAttachment(ref cause) => write!(f, "{}", cause),
            AttachTypedLinkError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AttachTypedLinkError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AttachTypedLinkError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachTypedLinkError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl BatchReadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchReadError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(BatchReadError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(BatchReadError::DirectoryNotEnabled(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchReadError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(BatchReadError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(BatchReadError::LimitExceeded(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(BatchReadError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchReadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchReadError::AccessDenied(ref cause) => write!(f, "{}", cause),
            BatchReadError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            BatchReadError::InternalService(ref cause) => write!(f, "{}", cause),
            BatchReadError::InvalidArn(ref cause) => write!(f, "{}", cause),
            BatchReadError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchReadError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchReadError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl BatchWriteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchWriteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(BatchWriteError::AccessDenied(err.msg))
                }
                "BatchWriteException" => {
                    return RusotoError::Service(BatchWriteError::BatchWrite(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(BatchWriteError::DirectoryNotEnabled(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchWriteError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(BatchWriteError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(BatchWriteError::LimitExceeded(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(BatchWriteError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchWriteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchWriteError::AccessDenied(ref cause) => write!(f, "{}", cause),
            BatchWriteError::BatchWrite(ref cause) => write!(f, "{}", cause),
            BatchWriteError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            BatchWriteError::InternalService(ref cause) => write!(f, "{}", cause),
            BatchWriteError::InvalidArn(ref cause) => write!(f, "{}", cause),
            BatchWriteError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchWriteError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchWriteError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl CreateDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDirectoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDirectoryError::AccessDenied(err.msg))
                }
                "DirectoryAlreadyExistsException" => {
                    return RusotoError::Service(CreateDirectoryError::DirectoryAlreadyExists(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateDirectoryError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateDirectoryError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDirectoryError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDirectoryError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(CreateDirectoryError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDirectoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDirectoryError::DirectoryAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateDirectoryError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateDirectoryError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateDirectoryError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDirectoryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateDirectoryError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDirectoryError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl CreateFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFacetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateFacetError::AccessDenied(err.msg))
                }
                "FacetAlreadyExistsException" => {
                    return RusotoError::Service(CreateFacetError::FacetAlreadyExists(err.msg))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(CreateFacetError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateFacetError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateFacetError::InvalidArn(err.msg))
                }
                "InvalidRuleException" => {
                    return RusotoError::Service(CreateFacetError::InvalidRule(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateFacetError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateFacetError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(CreateFacetError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateFacetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFacetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateFacetError::FacetAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateFacetError::FacetValidation(ref cause) => write!(f, "{}", cause),
            CreateFacetError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateFacetError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateFacetError::InvalidRule(ref cause) => write!(f, "{}", cause),
            CreateFacetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateFacetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateFacetError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFacetError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>Indicates that the requested index type is not supported.</p>
    UnsupportedIndexType(String),
}

impl CreateIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIndexError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateIndexError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(CreateIndexError::DirectoryNotEnabled(err.msg))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(CreateIndexError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateIndexError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateIndexError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateIndexError::LimitExceeded(err.msg))
                }
                "LinkNameAlreadyInUseException" => {
                    return RusotoError::Service(CreateIndexError::LinkNameAlreadyInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateIndexError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(CreateIndexError::RetryableConflict(err.msg))
                }
                "UnsupportedIndexTypeException" => {
                    return RusotoError::Service(CreateIndexError::UnsupportedIndexType(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateIndexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIndexError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateIndexError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            CreateIndexError::FacetValidation(ref cause) => write!(f, "{}", cause),
            CreateIndexError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateIndexError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateIndexError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateIndexError::LinkNameAlreadyInUse(ref cause) => write!(f, "{}", cause),
            CreateIndexError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateIndexError::RetryableConflict(ref cause) => write!(f, "{}", cause),
            CreateIndexError::UnsupportedIndexType(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIndexError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>Indicates that the requested index type is not supported.</p>
    UnsupportedIndexType(String),
}

impl CreateObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateObjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateObjectError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(CreateObjectError::DirectoryNotEnabled(err.msg))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(CreateObjectError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateObjectError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateObjectError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateObjectError::LimitExceeded(err.msg))
                }
                "LinkNameAlreadyInUseException" => {
                    return RusotoError::Service(CreateObjectError::LinkNameAlreadyInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateObjectError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(CreateObjectError::RetryableConflict(err.msg))
                }
                "UnsupportedIndexTypeException" => {
                    return RusotoError::Service(CreateObjectError::UnsupportedIndexType(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateObjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateObjectError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateObjectError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            CreateObjectError::FacetValidation(ref cause) => write!(f, "{}", cause),
            CreateObjectError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateObjectError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateObjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateObjectError::LinkNameAlreadyInUse(ref cause) => write!(f, "{}", cause),
            CreateObjectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateObjectError::RetryableConflict(ref cause) => write!(f, "{}", cause),
            CreateObjectError::UnsupportedIndexType(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateObjectError {}
/// Errors returned by CreateSchema
#[derive(Debug, PartialEq)]
pub enum CreateSchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>Indicates that a schema could not be created due to a naming conflict. Please select a different name and then try again.</p>
    SchemaAlreadyExists(String),
}

impl CreateSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateSchemaError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateSchemaError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateSchemaError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateSchemaError::LimitExceeded(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(CreateSchemaError::RetryableConflict(err.msg))
                }
                "SchemaAlreadyExistsException" => {
                    return RusotoError::Service(CreateSchemaError::SchemaAlreadyExists(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSchemaError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateSchemaError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateSchemaError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateSchemaError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSchemaError::RetryableConflict(ref cause) => write!(f, "{}", cause),
            CreateSchemaError::SchemaAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSchemaError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl CreateTypedLinkFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTypedLinkFacetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateTypedLinkFacetError::AccessDenied(err.msg))
                }
                "FacetAlreadyExistsException" => {
                    return RusotoError::Service(CreateTypedLinkFacetError::FacetAlreadyExists(
                        err.msg,
                    ))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(CreateTypedLinkFacetError::FacetValidation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateTypedLinkFacetError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateTypedLinkFacetError::InvalidArn(err.msg))
                }
                "InvalidRuleException" => {
                    return RusotoError::Service(CreateTypedLinkFacetError::InvalidRule(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateTypedLinkFacetError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateTypedLinkFacetError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(CreateTypedLinkFacetError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateTypedLinkFacetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTypedLinkFacetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateTypedLinkFacetError::FacetAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateTypedLinkFacetError::FacetValidation(ref cause) => write!(f, "{}", cause),
            CreateTypedLinkFacetError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateTypedLinkFacetError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateTypedLinkFacetError::InvalidRule(ref cause) => write!(f, "{}", cause),
            CreateTypedLinkFacetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTypedLinkFacetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateTypedLinkFacetError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTypedLinkFacetError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl DeleteDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDirectoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDirectoryError::AccessDenied(err.msg))
                }
                "DirectoryDeletedException" => {
                    return RusotoError::Service(DeleteDirectoryError::DirectoryDeleted(err.msg))
                }
                "DirectoryNotDisabledException" => {
                    return RusotoError::Service(DeleteDirectoryError::DirectoryNotDisabled(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteDirectoryError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DeleteDirectoryError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteDirectoryError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDirectoryError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(DeleteDirectoryError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDirectoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDirectoryError::DirectoryDeleted(ref cause) => write!(f, "{}", cause),
            DeleteDirectoryError::DirectoryNotDisabled(ref cause) => write!(f, "{}", cause),
            DeleteDirectoryError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteDirectoryError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DeleteDirectoryError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteDirectoryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDirectoryError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDirectoryError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl DeleteFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFacetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteFacetError::AccessDenied(err.msg))
                }
                "FacetInUseException" => {
                    return RusotoError::Service(DeleteFacetError::FacetInUse(err.msg))
                }
                "FacetNotFoundException" => {
                    return RusotoError::Service(DeleteFacetError::FacetNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteFacetError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DeleteFacetError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteFacetError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteFacetError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(DeleteFacetError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteFacetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFacetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteFacetError::FacetInUse(ref cause) => write!(f, "{}", cause),
            DeleteFacetError::FacetNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFacetError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteFacetError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DeleteFacetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteFacetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFacetError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFacetError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation cannot be completed because the object has not been detached from the tree.</p>
    ObjectNotDetached(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl DeleteObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteObjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteObjectError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(DeleteObjectError::DirectoryNotEnabled(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteObjectError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DeleteObjectError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteObjectError::LimitExceeded(err.msg))
                }
                "ObjectNotDetachedException" => {
                    return RusotoError::Service(DeleteObjectError::ObjectNotDetached(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteObjectError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(DeleteObjectError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteObjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteObjectError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteObjectError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            DeleteObjectError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteObjectError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DeleteObjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteObjectError::ObjectNotDetached(ref cause) => write!(f, "{}", cause),
            DeleteObjectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteObjectError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteObjectError {}
/// Errors returned by DeleteSchema
#[derive(Debug, PartialEq)]
pub enum DeleteSchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>The object could not be deleted because links still exist. Remove the links and then try the operation again.</p>
    StillContainsLinks(String),
}

impl DeleteSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteSchemaError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteSchemaError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DeleteSchemaError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteSchemaError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteSchemaError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(DeleteSchemaError::RetryableConflict(err.msg))
                }
                "StillContainsLinksException" => {
                    return RusotoError::Service(DeleteSchemaError::StillContainsLinks(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSchemaError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::RetryableConflict(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::StillContainsLinks(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSchemaError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl DeleteTypedLinkFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTypedLinkFacetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteTypedLinkFacetError::AccessDenied(err.msg))
                }
                "FacetNotFoundException" => {
                    return RusotoError::Service(DeleteTypedLinkFacetError::FacetNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteTypedLinkFacetError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DeleteTypedLinkFacetError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteTypedLinkFacetError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTypedLinkFacetError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(DeleteTypedLinkFacetError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTypedLinkFacetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTypedLinkFacetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteTypedLinkFacetError::FacetNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTypedLinkFacetError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteTypedLinkFacetError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DeleteTypedLinkFacetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteTypedLinkFacetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTypedLinkFacetError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTypedLinkFacetError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation can only operate on index objects.</p>
    NotIndex(String),
    /// <p>Indicates that the object is not attached to the index.</p>
    ObjectAlreadyDetached(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl DetachFromIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachFromIndexError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetachFromIndexError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(DetachFromIndexError::DirectoryNotEnabled(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DetachFromIndexError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DetachFromIndexError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DetachFromIndexError::LimitExceeded(err.msg))
                }
                "NotIndexException" => {
                    return RusotoError::Service(DetachFromIndexError::NotIndex(err.msg))
                }
                "ObjectAlreadyDetachedException" => {
                    return RusotoError::Service(DetachFromIndexError::ObjectAlreadyDetached(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DetachFromIndexError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(DetachFromIndexError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetachFromIndexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachFromIndexError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetachFromIndexError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            DetachFromIndexError::InternalService(ref cause) => write!(f, "{}", cause),
            DetachFromIndexError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DetachFromIndexError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DetachFromIndexError::NotIndex(ref cause) => write!(f, "{}", cause),
            DetachFromIndexError::ObjectAlreadyDetached(ref cause) => write!(f, "{}", cause),
            DetachFromIndexError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DetachFromIndexError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachFromIndexError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when any invalid operations are performed on an object that is not a node, such as calling <code>ListObjectChildren</code> for a leaf node object.</p>
    NotNode(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl DetachObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachObjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetachObjectError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(DetachObjectError::DirectoryNotEnabled(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DetachObjectError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DetachObjectError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DetachObjectError::LimitExceeded(err.msg))
                }
                "NotNodeException" => {
                    return RusotoError::Service(DetachObjectError::NotNode(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DetachObjectError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(DetachObjectError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetachObjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachObjectError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetachObjectError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            DetachObjectError::InternalService(ref cause) => write!(f, "{}", cause),
            DetachObjectError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DetachObjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DetachObjectError::NotNode(ref cause) => write!(f, "{}", cause),
            DetachObjectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DetachObjectError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachObjectError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation can only operate on policy objects.</p>
    NotPolicy(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl DetachPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetachPolicyError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(DetachPolicyError::DirectoryNotEnabled(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DetachPolicyError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DetachPolicyError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DetachPolicyError::LimitExceeded(err.msg))
                }
                "NotPolicyException" => {
                    return RusotoError::Service(DetachPolicyError::NotPolicy(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DetachPolicyError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(DetachPolicyError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetachPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::InternalService(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::NotPolicy(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachPolicyError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl DetachTypedLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachTypedLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetachTypedLinkError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(DetachTypedLinkError::DirectoryNotEnabled(err.msg))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(DetachTypedLinkError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DetachTypedLinkError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DetachTypedLinkError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DetachTypedLinkError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DetachTypedLinkError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(DetachTypedLinkError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetachTypedLinkError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachTypedLinkError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetachTypedLinkError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            DetachTypedLinkError::FacetValidation(ref cause) => write!(f, "{}", cause),
            DetachTypedLinkError::InternalService(ref cause) => write!(f, "{}", cause),
            DetachTypedLinkError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DetachTypedLinkError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DetachTypedLinkError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DetachTypedLinkError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachTypedLinkError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl DisableDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableDirectoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisableDirectoryError::AccessDenied(err.msg))
                }
                "DirectoryDeletedException" => {
                    return RusotoError::Service(DisableDirectoryError::DirectoryDeleted(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DisableDirectoryError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DisableDirectoryError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DisableDirectoryError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisableDirectoryError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(DisableDirectoryError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisableDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableDirectoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisableDirectoryError::DirectoryDeleted(ref cause) => write!(f, "{}", cause),
            DisableDirectoryError::InternalService(ref cause) => write!(f, "{}", cause),
            DisableDirectoryError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DisableDirectoryError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DisableDirectoryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DisableDirectoryError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableDirectoryError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl EnableDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableDirectoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(EnableDirectoryError::AccessDenied(err.msg))
                }
                "DirectoryDeletedException" => {
                    return RusotoError::Service(EnableDirectoryError::DirectoryDeleted(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(EnableDirectoryError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(EnableDirectoryError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(EnableDirectoryError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(EnableDirectoryError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(EnableDirectoryError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnableDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableDirectoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            EnableDirectoryError::DirectoryDeleted(ref cause) => write!(f, "{}", cause),
            EnableDirectoryError::InternalService(ref cause) => write!(f, "{}", cause),
            EnableDirectoryError::InvalidArn(ref cause) => write!(f, "{}", cause),
            EnableDirectoryError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            EnableDirectoryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            EnableDirectoryError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableDirectoryError {}
/// Errors returned by GetAppliedSchemaVersion
#[derive(Debug, PartialEq)]
pub enum GetAppliedSchemaVersionError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl GetAppliedSchemaVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppliedSchemaVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetAppliedSchemaVersionError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetAppliedSchemaVersionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetAppliedSchemaVersionError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetAppliedSchemaVersionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetAppliedSchemaVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(GetAppliedSchemaVersionError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAppliedSchemaVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAppliedSchemaVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetAppliedSchemaVersionError::InternalService(ref cause) => write!(f, "{}", cause),
            GetAppliedSchemaVersionError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetAppliedSchemaVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetAppliedSchemaVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetAppliedSchemaVersionError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAppliedSchemaVersionError {}
/// Errors returned by GetDirectory
#[derive(Debug, PartialEq)]
pub enum GetDirectoryError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl GetDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDirectoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDirectoryError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetDirectoryError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetDirectoryError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetDirectoryError::LimitExceeded(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(GetDirectoryError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDirectoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDirectoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDirectoryError::InternalService(ref cause) => write!(f, "{}", cause),
            GetDirectoryError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetDirectoryError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetDirectoryError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDirectoryError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl GetFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFacetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetFacetError::AccessDenied(err.msg))
                }
                "FacetNotFoundException" => {
                    return RusotoError::Service(GetFacetError::FacetNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetFacetError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetFacetError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetFacetError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFacetError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(GetFacetError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFacetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFacetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetFacetError::FacetNotFound(ref cause) => write!(f, "{}", cause),
            GetFacetError::InternalService(ref cause) => write!(f, "{}", cause),
            GetFacetError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetFacetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetFacetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFacetError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFacetError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl GetLinkAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLinkAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLinkAttributesError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(GetLinkAttributesError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(GetLinkAttributesError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetLinkAttributesError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetLinkAttributesError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetLinkAttributesError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLinkAttributesError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(GetLinkAttributesError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLinkAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLinkAttributesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLinkAttributesError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            GetLinkAttributesError::FacetValidation(ref cause) => write!(f, "{}", cause),
            GetLinkAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            GetLinkAttributesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetLinkAttributesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetLinkAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetLinkAttributesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLinkAttributesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl GetObjectAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetObjectAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetObjectAttributesError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(GetObjectAttributesError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(GetObjectAttributesError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetObjectAttributesError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetObjectAttributesError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetObjectAttributesError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetObjectAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(GetObjectAttributesError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetObjectAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetObjectAttributesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetObjectAttributesError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            GetObjectAttributesError::FacetValidation(ref cause) => write!(f, "{}", cause),
            GetObjectAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            GetObjectAttributesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetObjectAttributesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetObjectAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetObjectAttributesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetObjectAttributesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl GetObjectInformationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetObjectInformationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetObjectInformationError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(GetObjectInformationError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetObjectInformationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetObjectInformationError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetObjectInformationError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetObjectInformationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(GetObjectInformationError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetObjectInformationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetObjectInformationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetObjectInformationError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            GetObjectInformationError::InternalService(ref cause) => write!(f, "{}", cause),
            GetObjectInformationError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetObjectInformationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetObjectInformationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetObjectInformationError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetObjectInformationError {}
/// Errors returned by GetSchemaAsJson
#[derive(Debug, PartialEq)]
pub enum GetSchemaAsJsonError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl GetSchemaAsJsonError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSchemaAsJsonError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetSchemaAsJsonError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetSchemaAsJsonError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetSchemaAsJsonError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetSchemaAsJsonError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSchemaAsJsonError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(GetSchemaAsJsonError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSchemaAsJsonError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSchemaAsJsonError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetSchemaAsJsonError::InternalService(ref cause) => write!(f, "{}", cause),
            GetSchemaAsJsonError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetSchemaAsJsonError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetSchemaAsJsonError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetSchemaAsJsonError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSchemaAsJsonError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl GetTypedLinkFacetInformationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetTypedLinkFacetInformationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetTypedLinkFacetInformationError::AccessDenied(
                        err.msg,
                    ))
                }
                "FacetNotFoundException" => {
                    return RusotoError::Service(GetTypedLinkFacetInformationError::FacetNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(
                        GetTypedLinkFacetInformationError::InternalService(err.msg),
                    )
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetTypedLinkFacetInformationError::InvalidArn(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetTypedLinkFacetInformationError::InvalidNextToken(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetTypedLinkFacetInformationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetTypedLinkFacetInformationError::ResourceNotFound(err.msg),
                    )
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(
                        GetTypedLinkFacetInformationError::RetryableConflict(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTypedLinkFacetInformationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTypedLinkFacetInformationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetTypedLinkFacetInformationError::FacetNotFound(ref cause) => write!(f, "{}", cause),
            GetTypedLinkFacetInformationError::InternalService(ref cause) => write!(f, "{}", cause),
            GetTypedLinkFacetInformationError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetTypedLinkFacetInformationError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetTypedLinkFacetInformationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetTypedLinkFacetInformationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetTypedLinkFacetInformationError::RetryableConflict(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetTypedLinkFacetInformationError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListAppliedSchemaArnsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAppliedSchemaArnsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAppliedSchemaArnsError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListAppliedSchemaArnsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListAppliedSchemaArnsError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListAppliedSchemaArnsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListAppliedSchemaArnsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAppliedSchemaArnsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListAppliedSchemaArnsError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAppliedSchemaArnsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAppliedSchemaArnsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListAppliedSchemaArnsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListAppliedSchemaArnsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListAppliedSchemaArnsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListAppliedSchemaArnsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListAppliedSchemaArnsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListAppliedSchemaArnsError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAppliedSchemaArnsError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListAttachedIndicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAttachedIndicesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAttachedIndicesError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(ListAttachedIndicesError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListAttachedIndicesError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListAttachedIndicesError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListAttachedIndicesError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAttachedIndicesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListAttachedIndicesError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAttachedIndicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAttachedIndicesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListAttachedIndicesError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            ListAttachedIndicesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListAttachedIndicesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListAttachedIndicesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListAttachedIndicesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListAttachedIndicesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAttachedIndicesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListDevelopmentSchemaArnsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDevelopmentSchemaArnsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDevelopmentSchemaArnsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListDevelopmentSchemaArnsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListDevelopmentSchemaArnsError::InvalidArn(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDevelopmentSchemaArnsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListDevelopmentSchemaArnsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDevelopmentSchemaArnsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListDevelopmentSchemaArnsError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDevelopmentSchemaArnsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDevelopmentSchemaArnsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListDevelopmentSchemaArnsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListDevelopmentSchemaArnsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListDevelopmentSchemaArnsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListDevelopmentSchemaArnsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListDevelopmentSchemaArnsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDevelopmentSchemaArnsError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDevelopmentSchemaArnsError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListDirectoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDirectoriesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDirectoriesError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListDirectoriesError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListDirectoriesError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDirectoriesError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListDirectoriesError::LimitExceeded(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListDirectoriesError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDirectoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDirectoriesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListDirectoriesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListDirectoriesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListDirectoriesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListDirectoriesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListDirectoriesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDirectoriesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListFacetAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFacetAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListFacetAttributesError::AccessDenied(err.msg))
                }
                "FacetNotFoundException" => {
                    return RusotoError::Service(ListFacetAttributesError::FacetNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListFacetAttributesError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListFacetAttributesError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListFacetAttributesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListFacetAttributesError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFacetAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListFacetAttributesError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListFacetAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFacetAttributesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListFacetAttributesError::FacetNotFound(ref cause) => write!(f, "{}", cause),
            ListFacetAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListFacetAttributesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListFacetAttributesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListFacetAttributesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListFacetAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListFacetAttributesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFacetAttributesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListFacetNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFacetNamesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListFacetNamesError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListFacetNamesError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListFacetNamesError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListFacetNamesError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListFacetNamesError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFacetNamesError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListFacetNamesError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListFacetNamesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFacetNamesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListFacetNamesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListFacetNamesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListFacetNamesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListFacetNamesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListFacetNamesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListFacetNamesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFacetNamesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListIncomingTypedLinksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIncomingTypedLinksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListIncomingTypedLinksError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(ListIncomingTypedLinksError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(ListIncomingTypedLinksError::FacetValidation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListIncomingTypedLinksError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListIncomingTypedLinksError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListIncomingTypedLinksError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListIncomingTypedLinksError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListIncomingTypedLinksError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListIncomingTypedLinksError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListIncomingTypedLinksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIncomingTypedLinksError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListIncomingTypedLinksError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            ListIncomingTypedLinksError::FacetValidation(ref cause) => write!(f, "{}", cause),
            ListIncomingTypedLinksError::InternalService(ref cause) => write!(f, "{}", cause),
            ListIncomingTypedLinksError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListIncomingTypedLinksError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListIncomingTypedLinksError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListIncomingTypedLinksError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListIncomingTypedLinksError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIncomingTypedLinksError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation can only operate on index objects.</p>
    NotIndex(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIndexError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListIndexError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(ListIndexError::DirectoryNotEnabled(err.msg))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(ListIndexError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListIndexError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListIndexError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListIndexError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListIndexError::LimitExceeded(err.msg))
                }
                "NotIndexException" => {
                    return RusotoError::Service(ListIndexError::NotIndex(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListIndexError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListIndexError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListIndexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIndexError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListIndexError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            ListIndexError::FacetValidation(ref cause) => write!(f, "{}", cause),
            ListIndexError::InternalService(ref cause) => write!(f, "{}", cause),
            ListIndexError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListIndexError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListIndexError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListIndexError::NotIndex(ref cause) => write!(f, "{}", cause),
            ListIndexError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListIndexError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIndexError {}
/// Errors returned by ListManagedSchemaArns
#[derive(Debug, PartialEq)]
pub enum ListManagedSchemaArnsError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
}

impl ListManagedSchemaArnsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListManagedSchemaArnsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListManagedSchemaArnsError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListManagedSchemaArnsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListManagedSchemaArnsError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListManagedSchemaArnsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListManagedSchemaArnsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListManagedSchemaArnsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListManagedSchemaArnsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListManagedSchemaArnsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListManagedSchemaArnsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListManagedSchemaArnsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListManagedSchemaArnsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListManagedSchemaArnsError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListObjectAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListObjectAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListObjectAttributesError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(ListObjectAttributesError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(ListObjectAttributesError::FacetValidation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListObjectAttributesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListObjectAttributesError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListObjectAttributesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListObjectAttributesError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListObjectAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListObjectAttributesError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListObjectAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListObjectAttributesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListObjectAttributesError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            ListObjectAttributesError::FacetValidation(ref cause) => write!(f, "{}", cause),
            ListObjectAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListObjectAttributesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListObjectAttributesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListObjectAttributesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListObjectAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListObjectAttributesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListObjectAttributesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when any invalid operations are performed on an object that is not a node, such as calling <code>ListObjectChildren</code> for a leaf node object.</p>
    NotNode(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListObjectChildrenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListObjectChildrenError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListObjectChildrenError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(ListObjectChildrenError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListObjectChildrenError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListObjectChildrenError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListObjectChildrenError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListObjectChildrenError::LimitExceeded(err.msg))
                }
                "NotNodeException" => {
                    return RusotoError::Service(ListObjectChildrenError::NotNode(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListObjectChildrenError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListObjectChildrenError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListObjectChildrenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListObjectChildrenError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListObjectChildrenError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            ListObjectChildrenError::InternalService(ref cause) => write!(f, "{}", cause),
            ListObjectChildrenError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListObjectChildrenError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListObjectChildrenError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListObjectChildrenError::NotNode(ref cause) => write!(f, "{}", cause),
            ListObjectChildrenError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListObjectChildrenError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListObjectChildrenError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListObjectParentPathsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListObjectParentPathsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListObjectParentPathsError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(ListObjectParentPathsError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListObjectParentPathsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListObjectParentPathsError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListObjectParentPathsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListObjectParentPathsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListObjectParentPathsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListObjectParentPathsError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListObjectParentPathsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListObjectParentPathsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListObjectParentPathsError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            ListObjectParentPathsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListObjectParentPathsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListObjectParentPathsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListObjectParentPathsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListObjectParentPathsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListObjectParentPathsError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListObjectParentPathsError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListObjectParentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListObjectParentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListObjectParentsError::AccessDenied(err.msg))
                }
                "CannotListParentOfRootException" => {
                    return RusotoError::Service(ListObjectParentsError::CannotListParentOfRoot(
                        err.msg,
                    ))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(ListObjectParentsError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListObjectParentsError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListObjectParentsError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListObjectParentsError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListObjectParentsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListObjectParentsError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListObjectParentsError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListObjectParentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListObjectParentsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListObjectParentsError::CannotListParentOfRoot(ref cause) => write!(f, "{}", cause),
            ListObjectParentsError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            ListObjectParentsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListObjectParentsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListObjectParentsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListObjectParentsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListObjectParentsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListObjectParentsError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListObjectParentsError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListObjectPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListObjectPoliciesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListObjectPoliciesError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(ListObjectPoliciesError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListObjectPoliciesError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListObjectPoliciesError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListObjectPoliciesError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListObjectPoliciesError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListObjectPoliciesError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListObjectPoliciesError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListObjectPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListObjectPoliciesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListObjectPoliciesError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            ListObjectPoliciesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListObjectPoliciesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListObjectPoliciesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListObjectPoliciesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListObjectPoliciesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListObjectPoliciesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListObjectPoliciesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListOutgoingTypedLinksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOutgoingTypedLinksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListOutgoingTypedLinksError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(ListOutgoingTypedLinksError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(ListOutgoingTypedLinksError::FacetValidation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListOutgoingTypedLinksError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListOutgoingTypedLinksError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListOutgoingTypedLinksError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListOutgoingTypedLinksError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListOutgoingTypedLinksError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListOutgoingTypedLinksError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListOutgoingTypedLinksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOutgoingTypedLinksError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListOutgoingTypedLinksError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            ListOutgoingTypedLinksError::FacetValidation(ref cause) => write!(f, "{}", cause),
            ListOutgoingTypedLinksError::InternalService(ref cause) => write!(f, "{}", cause),
            ListOutgoingTypedLinksError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListOutgoingTypedLinksError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListOutgoingTypedLinksError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListOutgoingTypedLinksError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListOutgoingTypedLinksError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOutgoingTypedLinksError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that the requested operation can only operate on policy objects.</p>
    NotPolicy(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListPolicyAttachmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPolicyAttachmentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListPolicyAttachmentsError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(ListPolicyAttachmentsError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListPolicyAttachmentsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListPolicyAttachmentsError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListPolicyAttachmentsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListPolicyAttachmentsError::LimitExceeded(err.msg))
                }
                "NotPolicyException" => {
                    return RusotoError::Service(ListPolicyAttachmentsError::NotPolicy(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPolicyAttachmentsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListPolicyAttachmentsError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPolicyAttachmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPolicyAttachmentsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListPolicyAttachmentsError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            ListPolicyAttachmentsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListPolicyAttachmentsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListPolicyAttachmentsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListPolicyAttachmentsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListPolicyAttachmentsError::NotPolicy(ref cause) => write!(f, "{}", cause),
            ListPolicyAttachmentsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListPolicyAttachmentsError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPolicyAttachmentsError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListPublishedSchemaArnsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPublishedSchemaArnsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListPublishedSchemaArnsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListPublishedSchemaArnsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListPublishedSchemaArnsError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListPublishedSchemaArnsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListPublishedSchemaArnsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPublishedSchemaArnsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListPublishedSchemaArnsError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPublishedSchemaArnsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPublishedSchemaArnsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListPublishedSchemaArnsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListPublishedSchemaArnsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListPublishedSchemaArnsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListPublishedSchemaArnsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListPublishedSchemaArnsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListPublishedSchemaArnsError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPublishedSchemaArnsError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidArn(err.msg))
                }
                "InvalidTaggingRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidTaggingRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListTagsForResourceError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListTagsForResourceError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidTaggingRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListTypedLinkFacetAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListTypedLinkFacetAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTypedLinkFacetAttributesError::AccessDenied(
                        err.msg,
                    ))
                }
                "FacetNotFoundException" => {
                    return RusotoError::Service(ListTypedLinkFacetAttributesError::FacetNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(
                        ListTypedLinkFacetAttributesError::InternalService(err.msg),
                    )
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListTypedLinkFacetAttributesError::InvalidArn(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        ListTypedLinkFacetAttributesError::InvalidNextToken(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListTypedLinkFacetAttributesError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListTypedLinkFacetAttributesError::ResourceNotFound(err.msg),
                    )
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(
                        ListTypedLinkFacetAttributesError::RetryableConflict(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTypedLinkFacetAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTypedLinkFacetAttributesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetAttributesError::FacetNotFound(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetAttributesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetAttributesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListTypedLinkFacetAttributesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetAttributesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListTypedLinkFacetAttributesError::RetryableConflict(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListTypedLinkFacetAttributesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl ListTypedLinkFacetNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTypedLinkFacetNamesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTypedLinkFacetNamesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListTypedLinkFacetNamesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListTypedLinkFacetNamesError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTypedLinkFacetNamesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListTypedLinkFacetNamesError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTypedLinkFacetNamesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(ListTypedLinkFacetNamesError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTypedLinkFacetNamesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTypedLinkFacetNamesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetNamesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetNamesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetNamesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetNamesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetNamesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTypedLinkFacetNamesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTypedLinkFacetNamesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl LookupPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LookupPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(LookupPolicyError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(LookupPolicyError::DirectoryNotEnabled(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(LookupPolicyError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(LookupPolicyError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(LookupPolicyError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(LookupPolicyError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(LookupPolicyError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(LookupPolicyError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for LookupPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LookupPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            LookupPolicyError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            LookupPolicyError::InternalService(ref cause) => write!(f, "{}", cause),
            LookupPolicyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            LookupPolicyError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            LookupPolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            LookupPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            LookupPolicyError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for LookupPolicyError {}
/// Errors returned by PublishSchema
#[derive(Debug, PartialEq)]
pub enum PublishSchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>Indicates that a schema is already published.</p>
    SchemaAlreadyPublished(String),
}

impl PublishSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PublishSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PublishSchemaError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(PublishSchemaError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(PublishSchemaError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PublishSchemaError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PublishSchemaError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(PublishSchemaError::RetryableConflict(err.msg))
                }
                "SchemaAlreadyPublishedException" => {
                    return RusotoError::Service(PublishSchemaError::SchemaAlreadyPublished(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PublishSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PublishSchemaError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PublishSchemaError::InternalService(ref cause) => write!(f, "{}", cause),
            PublishSchemaError::InvalidArn(ref cause) => write!(f, "{}", cause),
            PublishSchemaError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PublishSchemaError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PublishSchemaError::RetryableConflict(ref cause) => write!(f, "{}", cause),
            PublishSchemaError::SchemaAlreadyPublished(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PublishSchemaError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl PutSchemaFromJsonError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutSchemaFromJsonError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutSchemaFromJsonError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(PutSchemaFromJsonError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(PutSchemaFromJsonError::InvalidArn(err.msg))
                }
                "InvalidRuleException" => {
                    return RusotoError::Service(PutSchemaFromJsonError::InvalidRule(err.msg))
                }
                "InvalidSchemaDocException" => {
                    return RusotoError::Service(PutSchemaFromJsonError::InvalidSchemaDoc(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutSchemaFromJsonError::LimitExceeded(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(PutSchemaFromJsonError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutSchemaFromJsonError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutSchemaFromJsonError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutSchemaFromJsonError::InternalService(ref cause) => write!(f, "{}", cause),
            PutSchemaFromJsonError::InvalidArn(ref cause) => write!(f, "{}", cause),
            PutSchemaFromJsonError::InvalidRule(ref cause) => write!(f, "{}", cause),
            PutSchemaFromJsonError::InvalidSchemaDoc(ref cause) => write!(f, "{}", cause),
            PutSchemaFromJsonError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutSchemaFromJsonError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutSchemaFromJsonError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl RemoveFacetFromObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveFacetFromObjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RemoveFacetFromObjectError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(RemoveFacetFromObjectError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(RemoveFacetFromObjectError::FacetValidation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(RemoveFacetFromObjectError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(RemoveFacetFromObjectError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RemoveFacetFromObjectError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemoveFacetFromObjectError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(RemoveFacetFromObjectError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemoveFacetFromObjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveFacetFromObjectError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RemoveFacetFromObjectError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            RemoveFacetFromObjectError::FacetValidation(ref cause) => write!(f, "{}", cause),
            RemoveFacetFromObjectError::InternalService(ref cause) => write!(f, "{}", cause),
            RemoveFacetFromObjectError::InvalidArn(ref cause) => write!(f, "{}", cause),
            RemoveFacetFromObjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            RemoveFacetFromObjectError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RemoveFacetFromObjectError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveFacetFromObjectError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(TagResourceError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(TagResourceError::InvalidArn(err.msg))
                }
                "InvalidTaggingRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidTaggingRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(TagResourceError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidTaggingRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UntagResourceError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UntagResourceError::InvalidArn(err.msg))
                }
                "InvalidTaggingRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidTaggingRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UntagResourceError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(UntagResourceError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidTaggingRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateFacet
#[derive(Debug, PartialEq)]
pub enum UpdateFacetError {
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl UpdateFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFacetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateFacetError::AccessDenied(err.msg))
                }
                "FacetNotFoundException" => {
                    return RusotoError::Service(UpdateFacetError::FacetNotFound(err.msg))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(UpdateFacetError::FacetValidation(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateFacetError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpdateFacetError::InvalidArn(err.msg))
                }
                "InvalidFacetUpdateException" => {
                    return RusotoError::Service(UpdateFacetError::InvalidFacetUpdate(err.msg))
                }
                "InvalidRuleException" => {
                    return RusotoError::Service(UpdateFacetError::InvalidRule(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateFacetError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateFacetError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(UpdateFacetError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateFacetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFacetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateFacetError::FacetNotFound(ref cause) => write!(f, "{}", cause),
            UpdateFacetError::FacetValidation(ref cause) => write!(f, "{}", cause),
            UpdateFacetError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateFacetError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpdateFacetError::InvalidFacetUpdate(ref cause) => write!(f, "{}", cause),
            UpdateFacetError::InvalidRule(ref cause) => write!(f, "{}", cause),
            UpdateFacetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFacetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateFacetError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFacetError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl UpdateLinkAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateLinkAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateLinkAttributesError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(UpdateLinkAttributesError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(UpdateLinkAttributesError::FacetValidation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateLinkAttributesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpdateLinkAttributesError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateLinkAttributesError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateLinkAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(UpdateLinkAttributesError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateLinkAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLinkAttributesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateLinkAttributesError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            UpdateLinkAttributesError::FacetValidation(ref cause) => write!(f, "{}", cause),
            UpdateLinkAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateLinkAttributesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpdateLinkAttributesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateLinkAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateLinkAttributesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLinkAttributesError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl UpdateObjectAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateObjectAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateObjectAttributesError::AccessDenied(err.msg))
                }
                "DirectoryNotEnabledException" => {
                    return RusotoError::Service(UpdateObjectAttributesError::DirectoryNotEnabled(
                        err.msg,
                    ))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(UpdateObjectAttributesError::FacetValidation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateObjectAttributesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpdateObjectAttributesError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateObjectAttributesError::LimitExceeded(
                        err.msg,
                    ))
                }
                "LinkNameAlreadyInUseException" => {
                    return RusotoError::Service(UpdateObjectAttributesError::LinkNameAlreadyInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateObjectAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(UpdateObjectAttributesError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateObjectAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateObjectAttributesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateObjectAttributesError::DirectoryNotEnabled(ref cause) => write!(f, "{}", cause),
            UpdateObjectAttributesError::FacetValidation(ref cause) => write!(f, "{}", cause),
            UpdateObjectAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateObjectAttributesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpdateObjectAttributesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateObjectAttributesError::LinkNameAlreadyInUse(ref cause) => write!(f, "{}", cause),
            UpdateObjectAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateObjectAttributesError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateObjectAttributesError {}
/// Errors returned by UpdateSchema
#[derive(Debug, PartialEq)]
pub enum UpdateSchemaError {
    /// <p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    /// <p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    /// <p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl UpdateSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateSchemaError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateSchemaError::InternalService(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpdateSchemaError::InvalidArn(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateSchemaError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateSchemaError::ResourceNotFound(err.msg))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(UpdateSchemaError::RetryableConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSchemaError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateSchemaError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateSchemaError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpdateSchemaError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateSchemaError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateSchemaError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSchemaError {}
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
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl UpdateTypedLinkFacetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTypedLinkFacetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateTypedLinkFacetError::AccessDenied(err.msg))
                }
                "FacetNotFoundException" => {
                    return RusotoError::Service(UpdateTypedLinkFacetError::FacetNotFound(err.msg))
                }
                "FacetValidationException" => {
                    return RusotoError::Service(UpdateTypedLinkFacetError::FacetValidation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateTypedLinkFacetError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpdateTypedLinkFacetError::InvalidArn(err.msg))
                }
                "InvalidFacetUpdateException" => {
                    return RusotoError::Service(UpdateTypedLinkFacetError::InvalidFacetUpdate(
                        err.msg,
                    ))
                }
                "InvalidRuleException" => {
                    return RusotoError::Service(UpdateTypedLinkFacetError::InvalidRule(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateTypedLinkFacetError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateTypedLinkFacetError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(UpdateTypedLinkFacetError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateTypedLinkFacetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTypedLinkFacetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateTypedLinkFacetError::FacetNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTypedLinkFacetError::FacetValidation(ref cause) => write!(f, "{}", cause),
            UpdateTypedLinkFacetError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateTypedLinkFacetError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpdateTypedLinkFacetError::InvalidFacetUpdate(ref cause) => write!(f, "{}", cause),
            UpdateTypedLinkFacetError::InvalidRule(ref cause) => write!(f, "{}", cause),
            UpdateTypedLinkFacetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateTypedLinkFacetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTypedLinkFacetError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTypedLinkFacetError {}
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
    /// <p>Indicates that an attempt to make an attachment was invalid. For example, attaching two nodes with a link type that is not applicable to the nodes or attempting to apply a schema to a directory a second time.</p>
    InvalidAttachment(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// <p>Indicates that a schema could not be created due to a naming conflict. Please select a different name and then try again.</p>
    SchemaAlreadyExists(String),
}

impl UpgradeAppliedSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpgradeAppliedSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpgradeAppliedSchemaError::AccessDenied(err.msg))
                }
                "IncompatibleSchemaException" => {
                    return RusotoError::Service(UpgradeAppliedSchemaError::IncompatibleSchema(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpgradeAppliedSchemaError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpgradeAppliedSchemaError::InvalidArn(err.msg))
                }
                "InvalidAttachmentException" => {
                    return RusotoError::Service(UpgradeAppliedSchemaError::InvalidAttachment(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpgradeAppliedSchemaError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(UpgradeAppliedSchemaError::RetryableConflict(
                        err.msg,
                    ))
                }
                "SchemaAlreadyExistsException" => {
                    return RusotoError::Service(UpgradeAppliedSchemaError::SchemaAlreadyExists(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpgradeAppliedSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpgradeAppliedSchemaError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpgradeAppliedSchemaError::IncompatibleSchema(ref cause) => write!(f, "{}", cause),
            UpgradeAppliedSchemaError::InternalService(ref cause) => write!(f, "{}", cause),
            UpgradeAppliedSchemaError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpgradeAppliedSchemaError::InvalidAttachment(ref cause) => write!(f, "{}", cause),
            UpgradeAppliedSchemaError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpgradeAppliedSchemaError::RetryableConflict(ref cause) => write!(f, "{}", cause),
            UpgradeAppliedSchemaError::SchemaAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpgradeAppliedSchemaError {}
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
    /// <p>Indicates that an attempt to make an attachment was invalid. For example, attaching two nodes with a link type that is not applicable to the nodes or attempting to apply a schema to a directory a second time.</p>
    InvalidAttachment(String),
    /// <p>Indicates that limits are exceeded. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
}

impl UpgradePublishedSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpgradePublishedSchemaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpgradePublishedSchemaError::AccessDenied(err.msg))
                }
                "IncompatibleSchemaException" => {
                    return RusotoError::Service(UpgradePublishedSchemaError::IncompatibleSchema(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpgradePublishedSchemaError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpgradePublishedSchemaError::InvalidArn(err.msg))
                }
                "InvalidAttachmentException" => {
                    return RusotoError::Service(UpgradePublishedSchemaError::InvalidAttachment(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpgradePublishedSchemaError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpgradePublishedSchemaError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "RetryableConflictException" => {
                    return RusotoError::Service(UpgradePublishedSchemaError::RetryableConflict(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpgradePublishedSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpgradePublishedSchemaError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpgradePublishedSchemaError::IncompatibleSchema(ref cause) => write!(f, "{}", cause),
            UpgradePublishedSchemaError::InternalService(ref cause) => write!(f, "{}", cause),
            UpgradePublishedSchemaError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpgradePublishedSchemaError::InvalidAttachment(ref cause) => write!(f, "{}", cause),
            UpgradePublishedSchemaError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpgradePublishedSchemaError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpgradePublishedSchemaError::RetryableConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpgradePublishedSchemaError {}
/// Trait representing the capabilities of the Amazon CloudDirectory API. Amazon CloudDirectory clients implement this trait.
#[async_trait]
pub trait CloudDirectory {
    /// <p>Adds a new <a>Facet</a> to an object. An object can have more than one facet applied on it.</p>
    async fn add_facet_to_object(
        &self,
        input: AddFacetToObjectRequest,
    ) -> Result<AddFacetToObjectResponse, RusotoError<AddFacetToObjectError>>;

    /// <p>Copies the input published schema, at the specified version, into the <a>Directory</a> with the same name and version as that of the published schema.</p>
    async fn apply_schema(
        &self,
        input: ApplySchemaRequest,
    ) -> Result<ApplySchemaResponse, RusotoError<ApplySchemaError>>;

    /// <p><p>Attaches an existing object to another object. An object can be accessed in two ways:</p> <ol> <li> <p>Using the path</p> </li> <li> <p>Using <code>ObjectIdentifier</code> </p> </li> </ol></p>
    async fn attach_object(
        &self,
        input: AttachObjectRequest,
    ) -> Result<AttachObjectResponse, RusotoError<AttachObjectError>>;

    /// <p>Attaches a policy object to a regular object. An object can have a limited number of attached policies.</p>
    async fn attach_policy(
        &self,
        input: AttachPolicyRequest,
    ) -> Result<AttachPolicyResponse, RusotoError<AttachPolicyError>>;

    /// <p>Attaches the specified object to the specified index.</p>
    async fn attach_to_index(
        &self,
        input: AttachToIndexRequest,
    ) -> Result<AttachToIndexResponse, RusotoError<AttachToIndexError>>;

    /// <p>Attaches a typed link to a specified source and target object. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn attach_typed_link(
        &self,
        input: AttachTypedLinkRequest,
    ) -> Result<AttachTypedLinkResponse, RusotoError<AttachTypedLinkError>>;

    /// <p>Performs all the read operations in a batch. </p>
    async fn batch_read(
        &self,
        input: BatchReadRequest,
    ) -> Result<BatchReadResponse, RusotoError<BatchReadError>>;

    /// <p>Performs all the write operations in a batch. Either all the operations succeed or none.</p>
    async fn batch_write(
        &self,
        input: BatchWriteRequest,
    ) -> Result<BatchWriteResponse, RusotoError<BatchWriteError>>;

    /// <p>Creates a <a>Directory</a> by copying the published schema into the directory. A directory cannot be created without a schema.</p> <p>You can also quickly create a directory using a managed schema, called the <code>QuickStartSchema</code>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_managed.html">Managed Schema</a> in the <i>Amazon Cloud Directory Developer Guide</i>.</p>
    async fn create_directory(
        &self,
        input: CreateDirectoryRequest,
    ) -> Result<CreateDirectoryResponse, RusotoError<CreateDirectoryError>>;

    /// <p>Creates a new <a>Facet</a> in a schema. Facet creation is allowed only in development or applied schemas.</p>
    async fn create_facet(
        &self,
        input: CreateFacetRequest,
    ) -> Result<CreateFacetResponse, RusotoError<CreateFacetError>>;

    /// <p>Creates an index object. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/indexing_search.html">Indexing and search</a> for more information.</p>
    async fn create_index(
        &self,
        input: CreateIndexRequest,
    ) -> Result<CreateIndexResponse, RusotoError<CreateIndexError>>;

    /// <p>Creates an object in a <a>Directory</a>. Additionally attaches the object to a parent, if a parent reference and <code>LinkName</code> is specified. An object is simply a collection of <a>Facet</a> attributes. You can also use this API call to create a policy object, if the facet from which you create the object is a policy facet. </p>
    async fn create_object(
        &self,
        input: CreateObjectRequest,
    ) -> Result<CreateObjectResponse, RusotoError<CreateObjectError>>;

    /// <p><p>Creates a new schema in a development state. A schema can exist in three phases:</p> <ul> <li> <p> <i>Development:</i> This is a mutable phase of the schema. All new schemas are in the development phase. Once the schema is finalized, it can be published.</p> </li> <li> <p> <i>Published:</i> Published schemas are immutable and have a version associated with them.</p> </li> <li> <p> <i>Applied:</i> Applied schemas are mutable in a way that allows you to add new schema facets. You can also add new, nonrequired attributes to existing schema facets. You can apply only published schemas to directories. </p> </li> </ul></p>
    async fn create_schema(
        &self,
        input: CreateSchemaRequest,
    ) -> Result<CreateSchemaResponse, RusotoError<CreateSchemaError>>;

    /// <p>Creates a <a>TypedLinkFacet</a>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn create_typed_link_facet(
        &self,
        input: CreateTypedLinkFacetRequest,
    ) -> Result<CreateTypedLinkFacetResponse, RusotoError<CreateTypedLinkFacetError>>;

    /// <p>Deletes a directory. Only disabled directories can be deleted. A deleted directory cannot be undone. Exercise extreme caution when deleting directories.</p>
    async fn delete_directory(
        &self,
        input: DeleteDirectoryRequest,
    ) -> Result<DeleteDirectoryResponse, RusotoError<DeleteDirectoryError>>;

    /// <p>Deletes a given <a>Facet</a>. All attributes and <a>Rule</a>s that are associated with the facet will be deleted. Only development schema facets are allowed deletion.</p>
    async fn delete_facet(
        &self,
        input: DeleteFacetRequest,
    ) -> Result<DeleteFacetResponse, RusotoError<DeleteFacetError>>;

    /// <p>Deletes an object and its associated attributes. Only objects with no children and no parents can be deleted. The maximum number of attributes that can be deleted during an object deletion is 30. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Amazon Cloud Directory Limits</a>.</p>
    async fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> Result<DeleteObjectResponse, RusotoError<DeleteObjectError>>;

    /// <p>Deletes a given schema. Schemas in a development and published state can only be deleted. </p>
    async fn delete_schema(
        &self,
        input: DeleteSchemaRequest,
    ) -> Result<DeleteSchemaResponse, RusotoError<DeleteSchemaError>>;

    /// <p>Deletes a <a>TypedLinkFacet</a>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn delete_typed_link_facet(
        &self,
        input: DeleteTypedLinkFacetRequest,
    ) -> Result<DeleteTypedLinkFacetResponse, RusotoError<DeleteTypedLinkFacetError>>;

    /// <p>Detaches the specified object from the specified index.</p>
    async fn detach_from_index(
        &self,
        input: DetachFromIndexRequest,
    ) -> Result<DetachFromIndexResponse, RusotoError<DetachFromIndexError>>;

    /// <p>Detaches a given object from the parent object. The object that is to be detached from the parent is specified by the link name.</p>
    async fn detach_object(
        &self,
        input: DetachObjectRequest,
    ) -> Result<DetachObjectResponse, RusotoError<DetachObjectError>>;

    /// <p>Detaches a policy from an object.</p>
    async fn detach_policy(
        &self,
        input: DetachPolicyRequest,
    ) -> Result<DetachPolicyResponse, RusotoError<DetachPolicyError>>;

    /// <p>Detaches a typed link from a specified source and target object. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn detach_typed_link(
        &self,
        input: DetachTypedLinkRequest,
    ) -> Result<(), RusotoError<DetachTypedLinkError>>;

    /// <p>Disables the specified directory. Disabled directories cannot be read or written to. Only enabled directories can be disabled. Disabled directories may be reenabled.</p>
    async fn disable_directory(
        &self,
        input: DisableDirectoryRequest,
    ) -> Result<DisableDirectoryResponse, RusotoError<DisableDirectoryError>>;

    /// <p>Enables the specified directory. Only disabled directories can be enabled. Once enabled, the directory can then be read and written to.</p>
    async fn enable_directory(
        &self,
        input: EnableDirectoryRequest,
    ) -> Result<EnableDirectoryResponse, RusotoError<EnableDirectoryError>>;

    /// <p>Returns current applied schema version ARN, including the minor version in use.</p>
    async fn get_applied_schema_version(
        &self,
        input: GetAppliedSchemaVersionRequest,
    ) -> Result<GetAppliedSchemaVersionResponse, RusotoError<GetAppliedSchemaVersionError>>;

    /// <p>Retrieves metadata about a directory.</p>
    async fn get_directory(
        &self,
        input: GetDirectoryRequest,
    ) -> Result<GetDirectoryResponse, RusotoError<GetDirectoryError>>;

    /// <p>Gets details of the <a>Facet</a>, such as facet name, attributes, <a>Rule</a>s, or <code>ObjectType</code>. You can call this on all kinds of schema facets -- published, development, or applied.</p>
    async fn get_facet(
        &self,
        input: GetFacetRequest,
    ) -> Result<GetFacetResponse, RusotoError<GetFacetError>>;

    /// <p>Retrieves attributes that are associated with a typed link.</p>
    async fn get_link_attributes(
        &self,
        input: GetLinkAttributesRequest,
    ) -> Result<GetLinkAttributesResponse, RusotoError<GetLinkAttributesError>>;

    /// <p>Retrieves attributes within a facet that are associated with an object.</p>
    async fn get_object_attributes(
        &self,
        input: GetObjectAttributesRequest,
    ) -> Result<GetObjectAttributesResponse, RusotoError<GetObjectAttributesError>>;

    /// <p>Retrieves metadata about an object.</p>
    async fn get_object_information(
        &self,
        input: GetObjectInformationRequest,
    ) -> Result<GetObjectInformationResponse, RusotoError<GetObjectInformationError>>;

    /// <p>Retrieves a JSON representation of the schema. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_jsonformat.html#schemas_json">JSON Schema Format</a> for more information.</p>
    async fn get_schema_as_json(
        &self,
        input: GetSchemaAsJsonRequest,
    ) -> Result<GetSchemaAsJsonResponse, RusotoError<GetSchemaAsJsonError>>;

    /// <p>Returns the identity attribute order for a specific <a>TypedLinkFacet</a>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn get_typed_link_facet_information(
        &self,
        input: GetTypedLinkFacetInformationRequest,
    ) -> Result<GetTypedLinkFacetInformationResponse, RusotoError<GetTypedLinkFacetInformationError>>;

    /// <p>Lists schema major versions applied to a directory. If <code>SchemaArn</code> is provided, lists the minor version.</p>
    async fn list_applied_schema_arns(
        &self,
        input: ListAppliedSchemaArnsRequest,
    ) -> Result<ListAppliedSchemaArnsResponse, RusotoError<ListAppliedSchemaArnsError>>;

    /// <p>Lists indices attached to the specified object.</p>
    async fn list_attached_indices(
        &self,
        input: ListAttachedIndicesRequest,
    ) -> Result<ListAttachedIndicesResponse, RusotoError<ListAttachedIndicesError>>;

    /// <p>Retrieves each Amazon Resource Name (ARN) of schemas in the development state.</p>
    async fn list_development_schema_arns(
        &self,
        input: ListDevelopmentSchemaArnsRequest,
    ) -> Result<ListDevelopmentSchemaArnsResponse, RusotoError<ListDevelopmentSchemaArnsError>>;

    /// <p>Lists directories created within an account.</p>
    async fn list_directories(
        &self,
        input: ListDirectoriesRequest,
    ) -> Result<ListDirectoriesResponse, RusotoError<ListDirectoriesError>>;

    /// <p>Retrieves attributes attached to the facet.</p>
    async fn list_facet_attributes(
        &self,
        input: ListFacetAttributesRequest,
    ) -> Result<ListFacetAttributesResponse, RusotoError<ListFacetAttributesError>>;

    /// <p>Retrieves the names of facets that exist in a schema.</p>
    async fn list_facet_names(
        &self,
        input: ListFacetNamesRequest,
    ) -> Result<ListFacetNamesResponse, RusotoError<ListFacetNamesError>>;

    /// <p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn list_incoming_typed_links(
        &self,
        input: ListIncomingTypedLinksRequest,
    ) -> Result<ListIncomingTypedLinksResponse, RusotoError<ListIncomingTypedLinksError>>;

    /// <p>Lists objects attached to the specified index.</p>
    async fn list_index(
        &self,
        input: ListIndexRequest,
    ) -> Result<ListIndexResponse, RusotoError<ListIndexError>>;

    /// <p>Lists the major version families of each managed schema. If a major version ARN is provided as SchemaArn, the minor version revisions in that family are listed instead.</p>
    async fn list_managed_schema_arns(
        &self,
        input: ListManagedSchemaArnsRequest,
    ) -> Result<ListManagedSchemaArnsResponse, RusotoError<ListManagedSchemaArnsError>>;

    /// <p>Lists all attributes that are associated with an object. </p>
    async fn list_object_attributes(
        &self,
        input: ListObjectAttributesRequest,
    ) -> Result<ListObjectAttributesResponse, RusotoError<ListObjectAttributesError>>;

    /// <p>Returns a paginated list of child objects that are associated with a given object.</p>
    async fn list_object_children(
        &self,
        input: ListObjectChildrenRequest,
    ) -> Result<ListObjectChildrenResponse, RusotoError<ListObjectChildrenError>>;

    /// <p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directorystructure.html">Directory Structure</a>.</p> <p>Use this API to evaluate all parents for an object. The call returns all objects from the root of the directory up to the requested object. The API returns the number of paths based on user-defined <code>MaxResults</code>, in case there are multiple paths to the parent. The order of the paths and nodes returned is consistent among multiple API calls unless the objects are deleted or moved. Paths not leading to the directory root are ignored from the target object.</p>
    async fn list_object_parent_paths(
        &self,
        input: ListObjectParentPathsRequest,
    ) -> Result<ListObjectParentPathsResponse, RusotoError<ListObjectParentPathsError>>;

    /// <p>Lists parent objects that are associated with a given object in pagination fashion.</p>
    async fn list_object_parents(
        &self,
        input: ListObjectParentsRequest,
    ) -> Result<ListObjectParentsResponse, RusotoError<ListObjectParentsError>>;

    /// <p>Returns policies attached to an object in pagination fashion.</p>
    async fn list_object_policies(
        &self,
        input: ListObjectPoliciesRequest,
    ) -> Result<ListObjectPoliciesResponse, RusotoError<ListObjectPoliciesError>>;

    /// <p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn list_outgoing_typed_links(
        &self,
        input: ListOutgoingTypedLinksRequest,
    ) -> Result<ListOutgoingTypedLinksResponse, RusotoError<ListOutgoingTypedLinksError>>;

    /// <p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>
    async fn list_policy_attachments(
        &self,
        input: ListPolicyAttachmentsRequest,
    ) -> Result<ListPolicyAttachmentsResponse, RusotoError<ListPolicyAttachmentsError>>;

    /// <p>Lists the major version families of each published schema. If a major version ARN is provided as <code>SchemaArn</code>, the minor version revisions in that family are listed instead.</p>
    async fn list_published_schema_arns(
        &self,
        input: ListPublishedSchemaArnsRequest,
    ) -> Result<ListPublishedSchemaArnsResponse, RusotoError<ListPublishedSchemaArnsError>>;

    /// <p>Returns tags for a resource. Tagging is currently supported only for directories with a limit of 50 tags per directory. All 50 tags are returned for a given directory with this API call.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Returns a paginated list of all attribute definitions for a particular <a>TypedLinkFacet</a>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn list_typed_link_facet_attributes(
        &self,
        input: ListTypedLinkFacetAttributesRequest,
    ) -> Result<ListTypedLinkFacetAttributesResponse, RusotoError<ListTypedLinkFacetAttributesError>>;

    /// <p>Returns a paginated list of <code>TypedLink</code> facet names for a particular schema. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn list_typed_link_facet_names(
        &self,
        input: ListTypedLinkFacetNamesRequest,
    ) -> Result<ListTypedLinkFacetNamesResponse, RusotoError<ListTypedLinkFacetNamesError>>;

    /// <p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directory.html#key_concepts_policies">Policies</a>.</p>
    async fn lookup_policy(
        &self,
        input: LookupPolicyRequest,
    ) -> Result<LookupPolicyResponse, RusotoError<LookupPolicyError>>;

    /// <p>Publishes a development schema with a major version and a recommended minor version.</p>
    async fn publish_schema(
        &self,
        input: PublishSchemaRequest,
    ) -> Result<PublishSchemaResponse, RusotoError<PublishSchemaError>>;

    /// <p>Allows a schema to be updated using JSON upload. Only available for development schemas. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_jsonformat.html#schemas_json">JSON Schema Format</a> for more information.</p>
    async fn put_schema_from_json(
        &self,
        input: PutSchemaFromJsonRequest,
    ) -> Result<PutSchemaFromJsonResponse, RusotoError<PutSchemaFromJsonError>>;

    /// <p>Removes the specified facet from the specified object.</p>
    async fn remove_facet_from_object(
        &self,
        input: RemoveFacetFromObjectRequest,
    ) -> Result<RemoveFacetFromObjectResponse, RusotoError<RemoveFacetFromObjectError>>;

    /// <p>An API operation for adding tags to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>An API operation for removing tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p><p>Does the following:</p> <ol> <li> <p>Adds new <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Updates existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Deletes existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> </ol></p>
    async fn update_facet(
        &self,
        input: UpdateFacetRequest,
    ) -> Result<UpdateFacetResponse, RusotoError<UpdateFacetError>>;

    /// <p>Updates a given typed link’s attributes. Attributes to be updated must not contribute to the typed link’s identity, as defined by its <code>IdentityAttributeOrder</code>.</p>
    async fn update_link_attributes(
        &self,
        input: UpdateLinkAttributesRequest,
    ) -> Result<UpdateLinkAttributesResponse, RusotoError<UpdateLinkAttributesError>>;

    /// <p>Updates a given object's attributes.</p>
    async fn update_object_attributes(
        &self,
        input: UpdateObjectAttributesRequest,
    ) -> Result<UpdateObjectAttributesResponse, RusotoError<UpdateObjectAttributesError>>;

    /// <p>Updates the schema name with a new name. Only development schema names can be updated.</p>
    async fn update_schema(
        &self,
        input: UpdateSchemaRequest,
    ) -> Result<UpdateSchemaResponse, RusotoError<UpdateSchemaError>>;

    /// <p>Updates a <a>TypedLinkFacet</a>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn update_typed_link_facet(
        &self,
        input: UpdateTypedLinkFacetRequest,
    ) -> Result<UpdateTypedLinkFacetResponse, RusotoError<UpdateTypedLinkFacetError>>;

    /// <p>Upgrades a single directory in-place using the <code>PublishedSchemaArn</code> with schema updates found in <code>MinorVersion</code>. Backwards-compatible minor version upgrades are instantaneously available for readers on all objects in the directory. Note: This is a synchronous API call and upgrades only one schema on a given directory per call. To upgrade multiple directories from one schema, you would need to call this API on each directory.</p>
    async fn upgrade_applied_schema(
        &self,
        input: UpgradeAppliedSchemaRequest,
    ) -> Result<UpgradeAppliedSchemaResponse, RusotoError<UpgradeAppliedSchemaError>>;

    /// <p>Upgrades a published schema under a new minor version revision using the current contents of <code>DevelopmentSchemaArn</code>.</p>
    async fn upgrade_published_schema(
        &self,
        input: UpgradePublishedSchemaRequest,
    ) -> Result<UpgradePublishedSchemaResponse, RusotoError<UpgradePublishedSchemaError>>;
}
/// A client for the Amazon CloudDirectory API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudDirectoryClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CloudDirectoryClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CloudDirectoryClient {
        CloudDirectoryClient { client, region }
    }
}

#[async_trait]
impl CloudDirectory for CloudDirectoryClient {
    /// <p>Adds a new <a>Facet</a> to an object. An object can have more than one facet applied on it.</p>
    async fn add_facet_to_object(
        &self,
        input: AddFacetToObjectRequest,
    ) -> Result<AddFacetToObjectResponse, RusotoError<AddFacetToObjectError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/facets";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AddFacetToObjectResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddFacetToObjectError::from_response(response))
        }
    }

    /// <p>Copies the input published schema, at the specified version, into the <a>Directory</a> with the same name and version as that of the published schema.</p>
    async fn apply_schema(
        &self,
        input: ApplySchemaRequest,
    ) -> Result<ApplySchemaResponse, RusotoError<ApplySchemaError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/apply";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ApplySchemaResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ApplySchemaError::from_response(response))
        }
    }

    /// <p><p>Attaches an existing object to another object. An object can be accessed in two ways:</p> <ol> <li> <p>Using the path</p> </li> <li> <p>Using <code>ObjectIdentifier</code> </p> </li> </ol></p>
    async fn attach_object(
        &self,
        input: AttachObjectRequest,
    ) -> Result<AttachObjectResponse, RusotoError<AttachObjectError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AttachObjectResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AttachObjectError::from_response(response))
        }
    }

    /// <p>Attaches a policy object to a regular object. An object can have a limited number of attached policies.</p>
    async fn attach_policy(
        &self,
        input: AttachPolicyRequest,
    ) -> Result<AttachPolicyResponse, RusotoError<AttachPolicyError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AttachPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AttachPolicyError::from_response(response))
        }
    }

    /// <p>Attaches the specified object to the specified index.</p>
    async fn attach_to_index(
        &self,
        input: AttachToIndexRequest,
    ) -> Result<AttachToIndexResponse, RusotoError<AttachToIndexError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AttachToIndexResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AttachToIndexError::from_response(response))
        }
    }

    /// <p>Attaches a typed link to a specified source and target object. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn attach_typed_link(
        &self,
        input: AttachTypedLinkRequest,
    ) -> Result<AttachTypedLinkResponse, RusotoError<AttachTypedLinkError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AttachTypedLinkResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AttachTypedLinkError::from_response(response))
        }
    }

    /// <p>Performs all the read operations in a batch. </p>
    async fn batch_read(
        &self,
        input: BatchReadRequest,
    ) -> Result<BatchReadResponse, RusotoError<BatchReadError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/batchread";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchReadResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchReadError::from_response(response))
        }
    }

    /// <p>Performs all the write operations in a batch. Either all the operations succeed or none.</p>
    async fn batch_write(
        &self,
        input: BatchWriteRequest,
    ) -> Result<BatchWriteResponse, RusotoError<BatchWriteError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/batchwrite";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchWriteResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchWriteError::from_response(response))
        }
    }

    /// <p>Creates a <a>Directory</a> by copying the published schema into the directory. A directory cannot be created without a schema.</p> <p>You can also quickly create a directory using a managed schema, called the <code>QuickStartSchema</code>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_managed.html">Managed Schema</a> in the <i>Amazon Cloud Directory Developer Guide</i>.</p>
    async fn create_directory(
        &self,
        input: CreateDirectoryRequest,
    ) -> Result<CreateDirectoryResponse, RusotoError<CreateDirectoryError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDirectoryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDirectoryError::from_response(response))
        }
    }

    /// <p>Creates a new <a>Facet</a> in a schema. Facet creation is allowed only in development or applied schemas.</p>
    async fn create_facet(
        &self,
        input: CreateFacetRequest,
    ) -> Result<CreateFacetResponse, RusotoError<CreateFacetError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateFacetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFacetError::from_response(response))
        }
    }

    /// <p>Creates an index object. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/indexing_search.html">Indexing and search</a> for more information.</p>
    async fn create_index(
        &self,
        input: CreateIndexRequest,
    ) -> Result<CreateIndexResponse, RusotoError<CreateIndexError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIndexResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIndexError::from_response(response))
        }
    }

    /// <p>Creates an object in a <a>Directory</a>. Additionally attaches the object to a parent, if a parent reference and <code>LinkName</code> is specified. An object is simply a collection of <a>Facet</a> attributes. You can also use this API call to create a policy object, if the facet from which you create the object is a policy facet. </p>
    async fn create_object(
        &self,
        input: CreateObjectRequest,
    ) -> Result<CreateObjectResponse, RusotoError<CreateObjectError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateObjectResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateObjectError::from_response(response))
        }
    }

    /// <p><p>Creates a new schema in a development state. A schema can exist in three phases:</p> <ul> <li> <p> <i>Development:</i> This is a mutable phase of the schema. All new schemas are in the development phase. Once the schema is finalized, it can be published.</p> </li> <li> <p> <i>Published:</i> Published schemas are immutable and have a version associated with them.</p> </li> <li> <p> <i>Applied:</i> Applied schemas are mutable in a way that allows you to add new schema facets. You can also add new, nonrequired attributes to existing schema facets. You can apply only published schemas to directories. </p> </li> </ul></p>
    async fn create_schema(
        &self,
        input: CreateSchemaRequest,
    ) -> Result<CreateSchemaResponse, RusotoError<CreateSchemaError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSchemaResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSchemaError::from_response(response))
        }
    }

    /// <p>Creates a <a>TypedLinkFacet</a>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn create_typed_link_facet(
        &self,
        input: CreateTypedLinkFacetRequest,
    ) -> Result<CreateTypedLinkFacetResponse, RusotoError<CreateTypedLinkFacetError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateTypedLinkFacetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTypedLinkFacetError::from_response(response))
        }
    }

    /// <p>Deletes a directory. Only disabled directories can be deleted. A deleted directory cannot be undone. Exercise extreme caution when deleting directories.</p>
    async fn delete_directory(
        &self,
        input: DeleteDirectoryRequest,
    ) -> Result<DeleteDirectoryResponse, RusotoError<DeleteDirectoryError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDirectoryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDirectoryError::from_response(response))
        }
    }

    /// <p>Deletes a given <a>Facet</a>. All attributes and <a>Rule</a>s that are associated with the facet will be deleted. Only development schema facets are allowed deletion.</p>
    async fn delete_facet(
        &self,
        input: DeleteFacetRequest,
    ) -> Result<DeleteFacetResponse, RusotoError<DeleteFacetError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteFacetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFacetError::from_response(response))
        }
    }

    /// <p>Deletes an object and its associated attributes. Only objects with no children and no parents can be deleted. The maximum number of attributes that can be deleted during an object deletion is 30. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/limits.html">Amazon Cloud Directory Limits</a>.</p>
    async fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> Result<DeleteObjectResponse, RusotoError<DeleteObjectError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteObjectResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteObjectError::from_response(response))
        }
    }

    /// <p>Deletes a given schema. Schemas in a development and published state can only be deleted. </p>
    async fn delete_schema(
        &self,
        input: DeleteSchemaRequest,
    ) -> Result<DeleteSchemaResponse, RusotoError<DeleteSchemaError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSchemaResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSchemaError::from_response(response))
        }
    }

    /// <p>Deletes a <a>TypedLinkFacet</a>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn delete_typed_link_facet(
        &self,
        input: DeleteTypedLinkFacetRequest,
    ) -> Result<DeleteTypedLinkFacetResponse, RusotoError<DeleteTypedLinkFacetError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteTypedLinkFacetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTypedLinkFacetError::from_response(response))
        }
    }

    /// <p>Detaches the specified object from the specified index.</p>
    async fn detach_from_index(
        &self,
        input: DetachFromIndexRequest,
    ) -> Result<DetachFromIndexResponse, RusotoError<DetachFromIndexError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DetachFromIndexResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DetachFromIndexError::from_response(response))
        }
    }

    /// <p>Detaches a given object from the parent object. The object that is to be detached from the parent is specified by the link name.</p>
    async fn detach_object(
        &self,
        input: DetachObjectRequest,
    ) -> Result<DetachObjectResponse, RusotoError<DetachObjectError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DetachObjectResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DetachObjectError::from_response(response))
        }
    }

    /// <p>Detaches a policy from an object.</p>
    async fn detach_policy(
        &self,
        input: DetachPolicyRequest,
    ) -> Result<DetachPolicyResponse, RusotoError<DetachPolicyError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DetachPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DetachPolicyError::from_response(response))
        }
    }

    /// <p>Detaches a typed link from a specified source and target object. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn detach_typed_link(
        &self,
        input: DetachTypedLinkRequest,
    ) -> Result<(), RusotoError<DetachTypedLinkError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DetachTypedLinkError::from_response(response))
        }
    }

    /// <p>Disables the specified directory. Disabled directories cannot be read or written to. Only enabled directories can be disabled. Disabled directories may be reenabled.</p>
    async fn disable_directory(
        &self,
        input: DisableDirectoryRequest,
    ) -> Result<DisableDirectoryResponse, RusotoError<DisableDirectoryError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/disable";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisableDirectoryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisableDirectoryError::from_response(response))
        }
    }

    /// <p>Enables the specified directory. Only disabled directories can be enabled. Once enabled, the directory can then be read and written to.</p>
    async fn enable_directory(
        &self,
        input: EnableDirectoryRequest,
    ) -> Result<EnableDirectoryResponse, RusotoError<EnableDirectoryError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/enable";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<EnableDirectoryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(EnableDirectoryError::from_response(response))
        }
    }

    /// <p>Returns current applied schema version ARN, including the minor version in use.</p>
    async fn get_applied_schema_version(
        &self,
        input: GetAppliedSchemaVersionRequest,
    ) -> Result<GetAppliedSchemaVersionResponse, RusotoError<GetAppliedSchemaVersionError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/getappliedschema";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAppliedSchemaVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAppliedSchemaVersionError::from_response(response))
        }
    }

    /// <p>Retrieves metadata about a directory.</p>
    async fn get_directory(
        &self,
        input: GetDirectoryRequest,
    ) -> Result<GetDirectoryResponse, RusotoError<GetDirectoryError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/get";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDirectoryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDirectoryError::from_response(response))
        }
    }

    /// <p>Gets details of the <a>Facet</a>, such as facet name, attributes, <a>Rule</a>s, or <code>ObjectType</code>. You can call this on all kinds of schema facets -- published, development, or applied.</p>
    async fn get_facet(
        &self,
        input: GetFacetRequest,
    ) -> Result<GetFacetResponse, RusotoError<GetFacetError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFacetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFacetError::from_response(response))
        }
    }

    /// <p>Retrieves attributes that are associated with a typed link.</p>
    async fn get_link_attributes(
        &self,
        input: GetLinkAttributesRequest,
    ) -> Result<GetLinkAttributesResponse, RusotoError<GetLinkAttributesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/attributes/get";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLinkAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetLinkAttributesError::from_response(response))
        }
    }

    /// <p>Retrieves attributes within a facet that are associated with an object.</p>
    async fn get_object_attributes(
        &self,
        input: GetObjectAttributesRequest,
    ) -> Result<GetObjectAttributesResponse, RusotoError<GetObjectAttributesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/attributes/get";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetObjectAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetObjectAttributesError::from_response(response))
        }
    }

    /// <p>Retrieves metadata about an object.</p>
    async fn get_object_information(
        &self,
        input: GetObjectInformationRequest,
    ) -> Result<GetObjectInformationResponse, RusotoError<GetObjectInformationError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/information";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetObjectInformationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetObjectInformationError::from_response(response))
        }
    }

    /// <p>Retrieves a JSON representation of the schema. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_jsonformat.html#schemas_json">JSON Schema Format</a> for more information.</p>
    async fn get_schema_as_json(
        &self,
        input: GetSchemaAsJsonRequest,
    ) -> Result<GetSchemaAsJsonResponse, RusotoError<GetSchemaAsJsonError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/json";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSchemaAsJsonResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSchemaAsJsonError::from_response(response))
        }
    }

    /// <p>Returns the identity attribute order for a specific <a>TypedLinkFacet</a>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn get_typed_link_facet_information(
        &self,
        input: GetTypedLinkFacetInformationRequest,
    ) -> Result<GetTypedLinkFacetInformationResponse, RusotoError<GetTypedLinkFacetInformationError>>
    {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/get";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetTypedLinkFacetInformationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTypedLinkFacetInformationError::from_response(response))
        }
    }

    /// <p>Lists schema major versions applied to a directory. If <code>SchemaArn</code> is provided, lists the minor version.</p>
    async fn list_applied_schema_arns(
        &self,
        input: ListAppliedSchemaArnsRequest,
    ) -> Result<ListAppliedSchemaArnsResponse, RusotoError<ListAppliedSchemaArnsError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/applied";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAppliedSchemaArnsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAppliedSchemaArnsError::from_response(response))
        }
    }

    /// <p>Lists indices attached to the specified object.</p>
    async fn list_attached_indices(
        &self,
        input: ListAttachedIndicesRequest,
    ) -> Result<ListAttachedIndicesResponse, RusotoError<ListAttachedIndicesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/indices";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAttachedIndicesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAttachedIndicesError::from_response(response))
        }
    }

    /// <p>Retrieves each Amazon Resource Name (ARN) of schemas in the development state.</p>
    async fn list_development_schema_arns(
        &self,
        input: ListDevelopmentSchemaArnsRequest,
    ) -> Result<ListDevelopmentSchemaArnsResponse, RusotoError<ListDevelopmentSchemaArnsError>>
    {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/development";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDevelopmentSchemaArnsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDevelopmentSchemaArnsError::from_response(response))
        }
    }

    /// <p>Lists directories created within an account.</p>
    async fn list_directories(
        &self,
        input: ListDirectoriesRequest,
    ) -> Result<ListDirectoriesResponse, RusotoError<ListDirectoriesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/list";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDirectoriesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDirectoriesError::from_response(response))
        }
    }

    /// <p>Retrieves attributes attached to the facet.</p>
    async fn list_facet_attributes(
        &self,
        input: ListFacetAttributesRequest,
    ) -> Result<ListFacetAttributesResponse, RusotoError<ListFacetAttributesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/attributes";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFacetAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFacetAttributesError::from_response(response))
        }
    }

    /// <p>Retrieves the names of facets that exist in a schema.</p>
    async fn list_facet_names(
        &self,
        input: ListFacetNamesRequest,
    ) -> Result<ListFacetNamesResponse, RusotoError<ListFacetNamesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/list";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFacetNamesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFacetNamesError::from_response(response))
        }
    }

    /// <p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn list_incoming_typed_links(
        &self,
        input: ListIncomingTypedLinksRequest,
    ) -> Result<ListIncomingTypedLinksResponse, RusotoError<ListIncomingTypedLinksError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/incoming";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIncomingTypedLinksResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIncomingTypedLinksError::from_response(response))
        }
    }

    /// <p>Lists objects attached to the specified index.</p>
    async fn list_index(
        &self,
        input: ListIndexRequest,
    ) -> Result<ListIndexResponse, RusotoError<ListIndexError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index/targets";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIndexResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIndexError::from_response(response))
        }
    }

    /// <p>Lists the major version families of each managed schema. If a major version ARN is provided as SchemaArn, the minor version revisions in that family are listed instead.</p>
    async fn list_managed_schema_arns(
        &self,
        input: ListManagedSchemaArnsRequest,
    ) -> Result<ListManagedSchemaArnsResponse, RusotoError<ListManagedSchemaArnsError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/managed";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListManagedSchemaArnsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListManagedSchemaArnsError::from_response(response))
        }
    }

    /// <p>Lists all attributes that are associated with an object. </p>
    async fn list_object_attributes(
        &self,
        input: ListObjectAttributesRequest,
    ) -> Result<ListObjectAttributesResponse, RusotoError<ListObjectAttributesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/attributes";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListObjectAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListObjectAttributesError::from_response(response))
        }
    }

    /// <p>Returns a paginated list of child objects that are associated with a given object.</p>
    async fn list_object_children(
        &self,
        input: ListObjectChildrenRequest,
    ) -> Result<ListObjectChildrenResponse, RusotoError<ListObjectChildrenError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/children";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListObjectChildrenResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListObjectChildrenError::from_response(response))
        }
    }

    /// <p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directorystructure.html">Directory Structure</a>.</p> <p>Use this API to evaluate all parents for an object. The call returns all objects from the root of the directory up to the requested object. The API returns the number of paths based on user-defined <code>MaxResults</code>, in case there are multiple paths to the parent. The order of the paths and nodes returned is consistent among multiple API calls unless the objects are deleted or moved. Paths not leading to the directory root are ignored from the target object.</p>
    async fn list_object_parent_paths(
        &self,
        input: ListObjectParentPathsRequest,
    ) -> Result<ListObjectParentPathsResponse, RusotoError<ListObjectParentPathsError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/parentpaths";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListObjectParentPathsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListObjectParentPathsError::from_response(response))
        }
    }

    /// <p>Lists parent objects that are associated with a given object in pagination fashion.</p>
    async fn list_object_parents(
        &self,
        input: ListObjectParentsRequest,
    ) -> Result<ListObjectParentsResponse, RusotoError<ListObjectParentsError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/parent";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListObjectParentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListObjectParentsError::from_response(response))
        }
    }

    /// <p>Returns policies attached to an object in pagination fashion.</p>
    async fn list_object_policies(
        &self,
        input: ListObjectPoliciesRequest,
    ) -> Result<ListObjectPoliciesResponse, RusotoError<ListObjectPoliciesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/policy";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListObjectPoliciesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListObjectPoliciesError::from_response(response))
        }
    }

    /// <p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn list_outgoing_typed_links(
        &self,
        input: ListOutgoingTypedLinksRequest,
    ) -> Result<ListOutgoingTypedLinksResponse, RusotoError<ListOutgoingTypedLinksError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/outgoing";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListOutgoingTypedLinksResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListOutgoingTypedLinksError::from_response(response))
        }
    }

    /// <p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>
    async fn list_policy_attachments(
        &self,
        input: ListPolicyAttachmentsRequest,
    ) -> Result<ListPolicyAttachmentsResponse, RusotoError<ListPolicyAttachmentsError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/attachment";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPolicyAttachmentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPolicyAttachmentsError::from_response(response))
        }
    }

    /// <p>Lists the major version families of each published schema. If a major version ARN is provided as <code>SchemaArn</code>, the minor version revisions in that family are listed instead.</p>
    async fn list_published_schema_arns(
        &self,
        input: ListPublishedSchemaArnsRequest,
    ) -> Result<ListPublishedSchemaArnsResponse, RusotoError<ListPublishedSchemaArnsError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/published";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPublishedSchemaArnsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPublishedSchemaArnsError::from_response(response))
        }
    }

    /// <p>Returns tags for a resource. Tagging is currently supported only for directories with a limit of 50 tags per directory. All 50 tags are returned for a given directory with this API call.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/tags";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>Returns a paginated list of all attribute definitions for a particular <a>TypedLinkFacet</a>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn list_typed_link_facet_attributes(
        &self,
        input: ListTypedLinkFacetAttributesRequest,
    ) -> Result<ListTypedLinkFacetAttributesResponse, RusotoError<ListTypedLinkFacetAttributesError>>
    {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/attributes";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTypedLinkFacetAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTypedLinkFacetAttributesError::from_response(response))
        }
    }

    /// <p>Returns a paginated list of <code>TypedLink</code> facet names for a particular schema. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn list_typed_link_facet_names(
        &self,
        input: ListTypedLinkFacetNamesRequest,
    ) -> Result<ListTypedLinkFacetNamesResponse, RusotoError<ListTypedLinkFacetNamesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/list";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTypedLinkFacetNamesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTypedLinkFacetNamesError::from_response(response))
        }
    }

    /// <p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directory.html#key_concepts_policies">Policies</a>.</p>
    async fn lookup_policy(
        &self,
        input: LookupPolicyRequest,
    ) -> Result<LookupPolicyResponse, RusotoError<LookupPolicyError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/lookup";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<LookupPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(LookupPolicyError::from_response(response))
        }
    }

    /// <p>Publishes a development schema with a major version and a recommended minor version.</p>
    async fn publish_schema(
        &self,
        input: PublishSchemaRequest,
    ) -> Result<PublishSchemaResponse, RusotoError<PublishSchemaError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/publish";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.development_schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PublishSchemaResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PublishSchemaError::from_response(response))
        }
    }

    /// <p>Allows a schema to be updated using JSON upload. Only available for development schemas. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/schemas_jsonformat.html#schemas_json">JSON Schema Format</a> for more information.</p>
    async fn put_schema_from_json(
        &self,
        input: PutSchemaFromJsonRequest,
    ) -> Result<PutSchemaFromJsonResponse, RusotoError<PutSchemaFromJsonError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/json";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutSchemaFromJsonResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutSchemaFromJsonError::from_response(response))
        }
    }

    /// <p>Removes the specified facet from the specified object.</p>
    async fn remove_facet_from_object(
        &self,
        input: RemoveFacetFromObjectRequest,
    ) -> Result<RemoveFacetFromObjectResponse, RusotoError<RemoveFacetFromObjectError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/facets/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RemoveFacetFromObjectResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveFacetFromObjectError::from_response(response))
        }
    }

    /// <p>An API operation for adding tags to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/tags/add";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p>An API operation for removing tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/tags/remove";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
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

    /// <p><p>Does the following:</p> <ol> <li> <p>Adds new <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Updates existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Deletes existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> </ol></p>
    async fn update_facet(
        &self,
        input: UpdateFacetRequest,
    ) -> Result<UpdateFacetResponse, RusotoError<UpdateFacetError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFacetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFacetError::from_response(response))
        }
    }

    /// <p>Updates a given typed link’s attributes. Attributes to be updated must not contribute to the typed link’s identity, as defined by its <code>IdentityAttributeOrder</code>.</p>
    async fn update_link_attributes(
        &self,
        input: UpdateLinkAttributesRequest,
    ) -> Result<UpdateLinkAttributesResponse, RusotoError<UpdateLinkAttributesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/attributes/update";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateLinkAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateLinkAttributesError::from_response(response))
        }
    }

    /// <p>Updates a given object's attributes.</p>
    async fn update_object_attributes(
        &self,
        input: UpdateObjectAttributesRequest,
    ) -> Result<UpdateObjectAttributesResponse, RusotoError<UpdateObjectAttributesError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/update";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateObjectAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateObjectAttributesError::from_response(response))
        }
    }

    /// <p>Updates the schema name with a new name. Only development schema names can be updated.</p>
    async fn update_schema(
        &self,
        input: UpdateSchemaRequest,
    ) -> Result<UpdateSchemaResponse, RusotoError<UpdateSchemaError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/update";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateSchemaResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSchemaError::from_response(response))
        }
    }

    /// <p>Updates a <a>TypedLinkFacet</a>. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    async fn update_typed_link_facet(
        &self,
        input: UpdateTypedLinkFacetRequest,
    ) -> Result<UpdateTypedLinkFacetResponse, RusotoError<UpdateTypedLinkFacetError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateTypedLinkFacetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTypedLinkFacetError::from_response(response))
        }
    }

    /// <p>Upgrades a single directory in-place using the <code>PublishedSchemaArn</code> with schema updates found in <code>MinorVersion</code>. Backwards-compatible minor version upgrades are instantaneously available for readers on all objects in the directory. Note: This is a synchronous API call and upgrades only one schema on a given directory per call. To upgrade multiple directories from one schema, you would need to call this API on each directory.</p>
    async fn upgrade_applied_schema(
        &self,
        input: UpgradeAppliedSchemaRequest,
    ) -> Result<UpgradeAppliedSchemaResponse, RusotoError<UpgradeAppliedSchemaError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/upgradeapplied";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpgradeAppliedSchemaResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpgradeAppliedSchemaError::from_response(response))
        }
    }

    /// <p>Upgrades a published schema under a new minor version revision using the current contents of <code>DevelopmentSchemaArn</code>.</p>
    async fn upgrade_published_schema(
        &self,
        input: UpgradePublishedSchemaRequest,
    ) -> Result<UpgradePublishedSchemaResponse, RusotoError<UpgradePublishedSchemaError>> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/upgradepublished";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpgradePublishedSchemaResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpgradePublishedSchemaError::from_response(response))
        }
    }
}
