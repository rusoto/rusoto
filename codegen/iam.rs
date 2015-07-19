use std::collections::HashMap;
use std::str;
/// The request was rejected because the provided password did not meet the
/// requirements imposed by the account password policy.
#[derive(Debug, Default)]
pub struct PasswordPolicyViolationException {
	pub message: passwordPolicyViolationMessage,
}

/// Parse PasswordPolicyViolationException from XML
struct PasswordPolicyViolationExceptionParser;
impl PasswordPolicyViolationExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PasswordPolicyViolationException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PasswordPolicyViolationException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(passwordPolicyViolationMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PasswordPolicyViolationException contents to a SignedRequest
struct PasswordPolicyViolationExceptionWriter;
impl PasswordPolicyViolationExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PasswordPolicyViolationException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		passwordPolicyViolationMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains the response to a successful ListAttachedRolePolicies request.
#[derive(Debug, Default)]
pub struct ListAttachedRolePoliciesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: markerType,
	/// A list of the attached policies.
	pub attached_policies: attachedPoliciesListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: booleanType,
}

/// Parse ListAttachedRolePoliciesResponse from XML
struct ListAttachedRolePoliciesResponseParser;
impl ListAttachedRolePoliciesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListAttachedRolePoliciesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListAttachedRolePoliciesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "AttachedPolicy" {
				obj.attached_policies = try!(attachedPoliciesListTypeParser::parse_xml("AttachedPolicy", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = try!(booleanTypeParser::parse_xml("IsTruncated", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListAttachedRolePoliciesResponse contents to a SignedRequest
struct ListAttachedRolePoliciesResponseWriter;
impl ListAttachedRolePoliciesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListAttachedRolePoliciesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		attachedPoliciesListTypeWriter::write_params(params, &(prefix.to_string() + "AttachedPolicy"), &obj.attached_policies);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
	}
}
/// Contains the response to a successful UploadSigningCertificate request.
#[derive(Debug, Default)]
pub struct UploadSigningCertificateResponse {
	/// Information about the certificate.
	pub certificate: SigningCertificate,
}

/// Parse UploadSigningCertificateResponse from XML
struct UploadSigningCertificateResponseParser;
impl UploadSigningCertificateResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UploadSigningCertificateResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UploadSigningCertificateResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Certificate" {
				obj.certificate = try!(SigningCertificateParser::parse_xml("Certificate", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UploadSigningCertificateResponse contents to a SignedRequest
struct UploadSigningCertificateResponseWriter;
impl UploadSigningCertificateResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UploadSigningCertificateResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		SigningCertificateWriter::write_params(params, &(prefix.to_string() + "Certificate"), &obj.certificate);
	}
}
#[derive(Debug, Default)]
pub struct ListInstanceProfilesRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// The path prefix for filtering the results. For example, the prefix
	/// `/application_abc/component_xyz/` gets all instance profiles whose path starts
	/// with `/application_abc/component_xyz/`.
	/// This parameter is optional. If it is not included, it defaults to a slash (/),
	/// listing all instance profiles.
	pub path_prefix: pathPrefixType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListInstanceProfilesRequest from XML
struct ListInstanceProfilesRequestParser;
impl ListInstanceProfilesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListInstanceProfilesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListInstanceProfilesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "PathPrefix" {
				obj.path_prefix = try!(pathPrefixTypeParser::parse_xml("PathPrefix", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListInstanceProfilesRequest contents to a SignedRequest
struct ListInstanceProfilesRequestWriter;
impl ListInstanceProfilesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListInstanceProfilesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		pathPrefixTypeWriter::write_params(params, &(prefix.to_string() + "PathPrefix"), &obj.path_prefix);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
pub type duplicateSSHPublicKeyMessage = String;
/// Parse duplicateSSHPublicKeyMessage from XML
struct duplicateSSHPublicKeyMessageParser;
impl duplicateSSHPublicKeyMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<duplicateSSHPublicKeyMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write duplicateSSHPublicKeyMessage contents to a SignedRequest
struct duplicateSSHPublicKeyMessageWriter;
impl duplicateSSHPublicKeyMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &duplicateSSHPublicKeyMessage) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful ListGroupPolicies request.
#[derive(Debug, Default)]
pub struct ListGroupPoliciesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A list of policy names.
	pub policy_names: policyNameListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
}

/// Parse ListGroupPoliciesResponse from XML
struct ListGroupPoliciesResponseParser;
impl ListGroupPoliciesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListGroupPoliciesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListGroupPoliciesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "policyNameType" {
				obj.policy_names = try!(policyNameListTypeParser::parse_xml("policyNameType", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListGroupPoliciesResponse contents to a SignedRequest
struct ListGroupPoliciesResponseWriter;
impl ListGroupPoliciesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListGroupPoliciesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		policyNameListTypeWriter::write_params(params, &(prefix.to_string() + "policyNameType"), &obj.policy_names);
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
	}
}
pub type groupNameListType = Vec<groupNameType>;
/// Parse groupNameListType from XML
struct groupNameListTypeParser;
impl groupNameListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<groupNameListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "groupNameType" {
			obj.push(try!(groupNameTypeParser::parse_xml("groupNameType", stack)));
		}
		Ok(obj)
	}
}
/// Write groupNameListType contents to a SignedRequest
struct groupNameListTypeWriter;
impl groupNameListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &groupNameListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			groupNameTypeWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// The request was rejected because the certificate is invalid.
#[derive(Debug, Default)]
pub struct InvalidCertificateException {
	pub message: invalidCertificateMessage,
}

/// Parse InvalidCertificateException from XML
struct InvalidCertificateExceptionParser;
impl InvalidCertificateExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidCertificateException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidCertificateException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(invalidCertificateMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidCertificateException contents to a SignedRequest
struct InvalidCertificateExceptionWriter;
impl InvalidCertificateExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidCertificateException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		invalidCertificateMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains the response to a successful ListSigningCertificates request.
#[derive(Debug, Default)]
pub struct ListSigningCertificatesResponse {
	/// A list of the user's signing certificate information.
	pub certificates: certificateListType,
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
}

/// Parse ListSigningCertificatesResponse from XML
struct ListSigningCertificatesResponseParser;
impl ListSigningCertificatesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSigningCertificatesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListSigningCertificatesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SigningCertificate" {
				obj.certificates = try!(certificateListTypeParser::parse_xml("SigningCertificate", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListSigningCertificatesResponse contents to a SignedRequest
struct ListSigningCertificatesResponseWriter;
impl ListSigningCertificatesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListSigningCertificatesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		certificateListTypeWriter::write_params(params, &(prefix.to_string() + "SigningCertificate"), &obj.certificates);
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct ListAccessKeysRequest {
	/// The name of the user.
	pub user_name: existingUserNameType,
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListAccessKeysRequest from XML
struct ListAccessKeysRequestParser;
impl ListAccessKeysRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListAccessKeysRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListAccessKeysRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListAccessKeysRequest contents to a SignedRequest
struct ListAccessKeysRequestWriter;
impl ListAccessKeysRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListAccessKeysRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
/// Contains information about an IAM group, including all of the group's
/// policies.
/// This data type is used as a response element in the
/// GetAccountAuthorizationDetails action.
#[derive(Debug, Default)]
pub struct GroupDetail {
	/// A list of the inline policies embedded in the group.
	pub group_policy_list: policyDetailListType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the group was created.
	pub create_date: dateType,
	/// The friendly name that identifies the group.
	pub group_name: groupNameType,
	/// The path to the group. For more information about paths, see [IAM Identifiers]
	/// (http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in
	/// the _Using IAM_ guide.
	pub path: pathType,
	/// A list of the managed policies attached to the group.
	pub attached_managed_policies: attachedPoliciesListType,
	/// The stable and unique string identifying the group. For more information about
	/// IDs, see [IAM Identifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Usi
	/// ng_Identifiers.html) in the _Using IAM_ guide.
	pub group_id: idType,
	pub arn: arnType,
}

/// Parse GroupDetail from XML
struct GroupDetailParser;
impl GroupDetailParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GroupDetail, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GroupDetail::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyDetail" {
				obj.group_policy_list = try!(policyDetailListTypeParser::parse_xml("PolicyDetail", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "Path" {
				obj.path = try!(pathTypeParser::parse_xml("Path", stack));
				continue;
			}
			if current_name == "AttachedPolicy" {
				obj.attached_managed_policies = try!(attachedPoliciesListTypeParser::parse_xml("AttachedPolicy", stack));
				continue;
			}
			if current_name == "GroupId" {
				obj.group_id = try!(idTypeParser::parse_xml("GroupId", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GroupDetail contents to a SignedRequest
struct GroupDetailWriter;
impl GroupDetailWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GroupDetail) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		policyDetailListTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDetail"), &obj.group_policy_list);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), &obj.path);
		attachedPoliciesListTypeWriter::write_params(params, &(prefix.to_string() + "AttachedPolicy"), &obj.attached_managed_policies);
		idTypeWriter::write_params(params, &(prefix.to_string() + "GroupId"), &obj.group_id);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
	}
}
pub type authenticationCodeType = String;
/// Parse authenticationCodeType from XML
struct authenticationCodeTypeParser;
impl authenticationCodeTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<authenticationCodeType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write authenticationCodeType contents to a SignedRequest
struct authenticationCodeTypeWriter;
impl authenticationCodeTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &authenticationCodeType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct GetSSHPublicKeyRequest {
	/// The name of the IAM user associated with the SSH public key.
	pub user_name: userNameType,
	/// The unique identifier for the SSH public key.
	pub ssh_public_key_id: publicKeyIdType,
	/// Specifies the public key encoding format to use in the response. To retrieve
	/// the public key in ssh-rsa format, use `SSH`. To retrieve the public key in PEM
	/// format, use `PEM`.
	pub encoding: encodingType,
}

/// Parse GetSSHPublicKeyRequest from XML
struct GetSSHPublicKeyRequestParser;
impl GetSSHPublicKeyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetSSHPublicKeyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetSSHPublicKeyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "SSHPublicKeyId" {
				obj.ssh_public_key_id = try!(publicKeyIdTypeParser::parse_xml("SSHPublicKeyId", stack));
				continue;
			}
			if current_name == "Encoding" {
				obj.encoding = try!(encodingTypeParser::parse_xml("Encoding", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetSSHPublicKeyRequest contents to a SignedRequest
struct GetSSHPublicKeyRequestWriter;
impl GetSSHPublicKeyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetSSHPublicKeyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		publicKeyIdTypeWriter::write_params(params, &(prefix.to_string() + "SSHPublicKeyId"), &obj.ssh_public_key_id);
		encodingTypeWriter::write_params(params, &(prefix.to_string() + "Encoding"), &obj.encoding);
	}
}
#[derive(Debug, Default)]
pub struct UpdateSAMLProviderRequest {
	/// The Amazon Resource Name (ARN) of the SAML provider to update.
	pub saml_provider_arn: arnType,
	/// An XML document generated by an identity provider (IdP) that supports SAML
	/// 2.0. The document includes the issuer's name, expiration information, and keys
	/// that can be used to validate the SAML authentication response (assertions)
	/// that are received from the IdP. You must generate the metadata document using
	/// the identity management software that is used as your organization's IdP.
	pub saml_metadata_document: SAMLMetadataDocumentType,
}

/// Parse UpdateSAMLProviderRequest from XML
struct UpdateSAMLProviderRequestParser;
impl UpdateSAMLProviderRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateSAMLProviderRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateSAMLProviderRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SAMLProviderArn" {
				obj.saml_provider_arn = try!(arnTypeParser::parse_xml("SAMLProviderArn", stack));
				continue;
			}
			if current_name == "SAMLMetadataDocument" {
				obj.saml_metadata_document = try!(SAMLMetadataDocumentTypeParser::parse_xml("SAMLMetadataDocument", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateSAMLProviderRequest contents to a SignedRequest
struct UpdateSAMLProviderRequestWriter;
impl UpdateSAMLProviderRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateSAMLProviderRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "SAMLProviderArn"), &obj.saml_provider_arn);
		SAMLMetadataDocumentTypeWriter::write_params(params, &(prefix.to_string() + "SAMLMetadataDocument"), &obj.saml_metadata_document);
	}
}
pub type minimumPasswordLengthType = i32;
/// Parse minimumPasswordLengthType from XML
struct minimumPasswordLengthTypeParser;
impl minimumPasswordLengthTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<minimumPasswordLengthType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write minimumPasswordLengthType contents to a SignedRequest
struct minimumPasswordLengthTypeWriter;
impl minimumPasswordLengthTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &minimumPasswordLengthType) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct DeleteUserRequest {
	/// The name of the user to delete.
	pub user_name: existingUserNameType,
}

/// Parse DeleteUserRequest from XML
struct DeleteUserRequestParser;
impl DeleteUserRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteUserRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteUserRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteUserRequest contents to a SignedRequest
struct DeleteUserRequestWriter;
impl DeleteUserRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteUserRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
	}
}
#[derive(Debug, Default)]
pub struct DetachGroupPolicyRequest {
	/// The name (friendly name, not ARN) of the group to detach the policy from.
	pub group_name: groupNameType,
	pub policy_arn: arnType,
}

/// Parse DetachGroupPolicyRequest from XML
struct DetachGroupPolicyRequestParser;
impl DetachGroupPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DetachGroupPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DetachGroupPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DetachGroupPolicyRequest contents to a SignedRequest
struct DetachGroupPolicyRequestWriter;
impl DetachGroupPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DetachGroupPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
/// Contains information about an IAM role.
/// This data type is used as a response element in the following actions:
///   * CreateRole
///   * GetRole
///   * ListRoles
#[derive(Debug, Default)]
pub struct Role {
	/// The policy that grants an entity permission to assume the role.
	pub assume_role_policy_document: Option<policyDocumentType>,
	/// The stable and unique string identifying the role. For more information about
	/// IDs, see [IAM Identifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Usi
	/// ng_Identifiers.html) in the _Using IAM_ guide.
	pub role_id: idType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the role was created.
	pub create_date: dateType,
	/// The friendly name that identifies the role.
	pub role_name: roleNameType,
	/// The path to the role. For more information about paths, see [IAM Identifiers](
	/// http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the
	/// _Using IAM_ guide.
	pub path: pathType,
	/// The Amazon Resource Name (ARN) specifying the role. For more information about
	/// ARNs and how to use them in policies, see [IAM Identifiers](http://docs.aws.am
	/// azon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the _Using IAM_
	/// guide.
	pub arn: arnType,
}

/// Parse Role from XML
struct RoleParser;
impl RoleParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Role, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Role::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AssumeRolePolicyDocument" {
				obj.assume_role_policy_document = Some(try!(policyDocumentTypeParser::parse_xml("AssumeRolePolicyDocument", stack)));
				continue;
			}
			if current_name == "RoleId" {
				obj.role_id = try!(idTypeParser::parse_xml("RoleId", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "Path" {
				obj.path = try!(pathTypeParser::parse_xml("Path", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Role contents to a SignedRequest
struct RoleWriter;
impl RoleWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Role) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.assume_role_policy_document {
			policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "AssumeRolePolicyDocument"), obj);
		}
		idTypeWriter::write_params(params, &(prefix.to_string() + "RoleId"), &obj.role_id);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), &obj.path);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
	}
}
/// Contains the response to a successful ListSAMLProviders request.
#[derive(Debug, Default)]
pub struct ListSAMLProvidersResponse {
	/// The list of SAML providers for this account.
	pub saml_provider_list: SAMLProviderListType,
}

/// Parse ListSAMLProvidersResponse from XML
struct ListSAMLProvidersResponseParser;
impl ListSAMLProvidersResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSAMLProvidersResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListSAMLProvidersResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SAMLProviderListEntry" {
				obj.saml_provider_list = try!(SAMLProviderListTypeParser::parse_xml("SAMLProviderListEntry", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListSAMLProvidersResponse contents to a SignedRequest
struct ListSAMLProvidersResponseWriter;
impl ListSAMLProvidersResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListSAMLProvidersResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		SAMLProviderListTypeWriter::write_params(params, &(prefix.to_string() + "SAMLProviderListEntry"), &obj.saml_provider_list);
	}
}
/// Contains the response to a successful ListUserPolicies request.
#[derive(Debug, Default)]
pub struct ListUserPoliciesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A list of policy names.
	pub policy_names: policyNameListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
}

/// Parse ListUserPoliciesResponse from XML
struct ListUserPoliciesResponseParser;
impl ListUserPoliciesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListUserPoliciesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListUserPoliciesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "policyNameType" {
				obj.policy_names = try!(policyNameListTypeParser::parse_xml("policyNameType", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListUserPoliciesResponse contents to a SignedRequest
struct ListUserPoliciesResponseWriter;
impl ListUserPoliciesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListUserPoliciesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		policyNameListTypeWriter::write_params(params, &(prefix.to_string() + "policyNameType"), &obj.policy_names);
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct UploadSSHPublicKeyRequest {
	/// The name of the IAM user to associate the SSH public key with.
	pub user_name: userNameType,
	/// The SSH public key. The public key must be encoded in ssh-rsa format or PEM
	/// format.
	pub ssh_public_key_body: publicKeyMaterialType,
}

/// Parse UploadSSHPublicKeyRequest from XML
struct UploadSSHPublicKeyRequestParser;
impl UploadSSHPublicKeyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UploadSSHPublicKeyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UploadSSHPublicKeyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "SSHPublicKeyBody" {
				obj.ssh_public_key_body = try!(publicKeyMaterialTypeParser::parse_xml("SSHPublicKeyBody", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UploadSSHPublicKeyRequest contents to a SignedRequest
struct UploadSSHPublicKeyRequestWriter;
impl UploadSSHPublicKeyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UploadSSHPublicKeyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		publicKeyMaterialTypeWriter::write_params(params, &(prefix.to_string() + "SSHPublicKeyBody"), &obj.ssh_public_key_body);
	}
}
/// Contains the response to a successful GetGroupPolicy request.
#[derive(Debug, Default)]
pub struct GetGroupPolicyResponse {
	/// The group the policy is associated with.
	pub group_name: groupNameType,
	/// The policy document.
	pub policy_document: policyDocumentType,
	/// The name of the policy.
	pub policy_name: policyNameType,
}

/// Parse GetGroupPolicyResponse from XML
struct GetGroupPolicyResponseParser;
impl GetGroupPolicyResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetGroupPolicyResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetGroupPolicyResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "PolicyDocument" {
				obj.policy_document = try!(policyDocumentTypeParser::parse_xml("PolicyDocument", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetGroupPolicyResponse contents to a SignedRequest
struct GetGroupPolicyResponseWriter;
impl GetGroupPolicyResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetGroupPolicyResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDocument"), &obj.policy_document);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
	}
}
#[derive(Debug, Default)]
pub struct GetServerCertificateRequest {
	/// The name of the server certificate you want to retrieve information about.
	pub server_certificate_name: serverCertificateNameType,
}

/// Parse GetServerCertificateRequest from XML
struct GetServerCertificateRequestParser;
impl GetServerCertificateRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetServerCertificateRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetServerCertificateRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ServerCertificateName" {
				obj.server_certificate_name = try!(serverCertificateNameTypeParser::parse_xml("ServerCertificateName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetServerCertificateRequest contents to a SignedRequest
struct GetServerCertificateRequestWriter;
impl GetServerCertificateRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetServerCertificateRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		serverCertificateNameTypeWriter::write_params(params, &(prefix.to_string() + "ServerCertificateName"), &obj.server_certificate_name);
	}
}
/// Contains the response to a successful ListGroups request.
#[derive(Debug, Default)]
pub struct ListGroupsResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
	/// A list of groups.
	pub groups: groupListType,
}

/// Parse ListGroupsResponse from XML
struct ListGroupsResponseParser;
impl ListGroupsResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListGroupsResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListGroupsResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			if current_name == "Group" {
				obj.groups = try!(groupListTypeParser::parse_xml("Group", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListGroupsResponse contents to a SignedRequest
struct ListGroupsResponseWriter;
impl ListGroupsResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListGroupsResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
		groupListTypeWriter::write_params(params, &(prefix.to_string() + "Group"), &obj.groups);
	}
}
/// Contains information about a server certificate without its certificate body,
/// certificate chain, and private key.
/// This data type is used as a response element in the UploadServerCertificate
/// and ListServerCertificates actions.
#[derive(Debug, Default)]
pub struct ServerCertificateMetadata {
	/// The stable and unique string identifying the server certificate. For more
	/// information about IDs, see [IAM Identifiers](http://docs.aws.amazon.com/IAM/la
	/// test/UserGuide/Using_Identifiers.html) in the _Using IAM_ guide.
	pub server_certificate_id: idType,
	/// The name that identifies the server certificate.
	pub server_certificate_name: serverCertificateNameType,
	/// The date on which the certificate is set to expire.
	pub expiration: Option<dateType>,
	/// The path to the server certificate. For more information about paths, see [IAM
	/// Identifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers
	/// .html) in the _Using IAM_ guide.
	pub path: pathType,
	/// The Amazon Resource Name (ARN) specifying the server certificate. For more
	/// information about ARNs and how to use them in policies, see [IAM Identifiers](
	/// http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the
	/// _Using IAM_ guide.
	pub arn: arnType,
	/// The date when the server certificate was uploaded.
	pub upload_date: Option<dateType>,
}

/// Parse ServerCertificateMetadata from XML
struct ServerCertificateMetadataParser;
impl ServerCertificateMetadataParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ServerCertificateMetadata, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ServerCertificateMetadata::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ServerCertificateId" {
				obj.server_certificate_id = try!(idTypeParser::parse_xml("ServerCertificateId", stack));
				continue;
			}
			if current_name == "ServerCertificateName" {
				obj.server_certificate_name = try!(serverCertificateNameTypeParser::parse_xml("ServerCertificateName", stack));
				continue;
			}
			if current_name == "Expiration" {
				obj.expiration = Some(try!(dateTypeParser::parse_xml("Expiration", stack)));
				continue;
			}
			if current_name == "Path" {
				obj.path = try!(pathTypeParser::parse_xml("Path", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			if current_name == "UploadDate" {
				obj.upload_date = Some(try!(dateTypeParser::parse_xml("UploadDate", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ServerCertificateMetadata contents to a SignedRequest
struct ServerCertificateMetadataWriter;
impl ServerCertificateMetadataWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ServerCertificateMetadata) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		idTypeWriter::write_params(params, &(prefix.to_string() + "ServerCertificateId"), &obj.server_certificate_id);
		serverCertificateNameTypeWriter::write_params(params, &(prefix.to_string() + "ServerCertificateName"), &obj.server_certificate_name);
		if let Some(ref obj) = obj.expiration {
			dateTypeWriter::write_params(params, &(prefix.to_string() + "Expiration"), obj);
		}
		pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), &obj.path);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
		if let Some(ref obj) = obj.upload_date {
			dateTypeWriter::write_params(params, &(prefix.to_string() + "UploadDate"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct CreateOpenIDConnectProviderRequest {
	/// The URL of the identity provider. The URL must begin with "https://" and
	/// should correspond to the `iss` claim in the provider's OpenID Connect ID
	/// tokens. Per the OIDC standard, path components are allowed but query
	/// parameters are not. Typically the URL consists of only a host name, like
	/// "https://server.example.org" or "https://example.com".
	/// You cannot register the same provider multiple times in a single AWS account.
	/// If you try to submit a URL that has already been used for an OpenID Connect
	/// provider in the AWS account, you will get an error.
	pub url: OpenIDConnectProviderUrlType,
	/// A list of server certificate thumbprints for the OpenID Connect (OIDC)
	/// identity provider's server certificate(s). Typically this list includes only
	/// one entry. However, IAM lets you have up to five thumbprints for an OIDC
	/// provider. This lets you maintain multiple thumbprints if the identity provider
	/// is rotating certificates.
	/// The server certificate thumbprint is the hex-encoded SHA-1 hash value of the
	/// X.509 certificate used by the domain where the OpenID Connect provider makes
	/// its keys available. It is always a 40-character string.
	/// You must provide at least one thumbprint when creating an IAM OIDC provider.
	/// For example, if the OIDC provider is `server.example.com` and the provider
	/// stores its keys at "https://keys.server.example.com/openid-connect", the
	/// thumbprint string would be the hex-encoded SHA-1 hash value of the certificate
	/// used by https://keys.server.example.com.
	/// For more information about obtaining the OIDC provider's thumbprint, see
	/// [Obtaining the Thumbprint for an OpenID Connect
	/// Provider](http://docs.aws.amazon.com/IAM/latest/UserGuide/identity-providers-
	/// oidc-obtain-thumbprint.html) in the _Using IAM_ guide.
	pub thumbprint_list: thumbprintListType,
	/// A list of client IDs (also known as audiences). When a mobile or web app
	/// registers with an OpenID Connect provider, they establish a value that
	/// identifies the application. (This is the value that's sent as the `client_id`
	/// parameter on OAuth requests.)
	/// You can register multiple client IDs with the same provider. For example, you
	/// might have multiple applications that use the same OIDC provider. You cannot
	/// register more than 100 client IDs with a single IAM OIDC provider.
	/// There is no defined format for a client ID. The
	/// `CreateOpenIDConnectProviderRequest` action accepts client IDs up to 255
	/// characters long.
	pub client_id_list: Option<clientIDListType>,
}

/// Parse CreateOpenIDConnectProviderRequest from XML
struct CreateOpenIDConnectProviderRequestParser;
impl CreateOpenIDConnectProviderRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateOpenIDConnectProviderRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateOpenIDConnectProviderRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Url" {
				obj.url = try!(OpenIDConnectProviderUrlTypeParser::parse_xml("Url", stack));
				continue;
			}
			if current_name == "thumbprintType" {
				obj.thumbprint_list = try!(thumbprintListTypeParser::parse_xml("thumbprintType", stack));
				continue;
			}
			if current_name == "clientIDType" {
				obj.client_id_list = Some(try!(clientIDListTypeParser::parse_xml("clientIDType", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateOpenIDConnectProviderRequest contents to a SignedRequest
struct CreateOpenIDConnectProviderRequestWriter;
impl CreateOpenIDConnectProviderRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateOpenIDConnectProviderRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		OpenIDConnectProviderUrlTypeWriter::write_params(params, &(prefix.to_string() + "Url"), &obj.url);
		thumbprintListTypeWriter::write_params(params, &(prefix.to_string() + "thumbprintType"), &obj.thumbprint_list);
		if let Some(ref obj) = obj.client_id_list {
			clientIDListTypeWriter::write_params(params, &(prefix.to_string() + "clientIDType"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct DeleteServerCertificateRequest {
	/// The name of the server certificate you want to delete.
	pub server_certificate_name: serverCertificateNameType,
}

/// Parse DeleteServerCertificateRequest from XML
struct DeleteServerCertificateRequestParser;
impl DeleteServerCertificateRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteServerCertificateRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteServerCertificateRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ServerCertificateName" {
				obj.server_certificate_name = try!(serverCertificateNameTypeParser::parse_xml("ServerCertificateName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteServerCertificateRequest contents to a SignedRequest
struct DeleteServerCertificateRequestWriter;
impl DeleteServerCertificateRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteServerCertificateRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		serverCertificateNameTypeWriter::write_params(params, &(prefix.to_string() + "ServerCertificateName"), &obj.server_certificate_name);
	}
}
/// Contains information about a user that a managed policy is attached to.
/// This data type is used as a response element in the ListEntitiesForPolicy
/// action.
/// For more information about managed policies, refer to [Managed Policies and
/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
/// managed-vs-inline.html) in the _Using IAM_ guide.
#[derive(Debug, Default)]
pub struct PolicyUser {
	/// The name (friendly name, not ARN) identifying the user.
	pub user_name: userNameType,
}

/// Parse PolicyUser from XML
struct PolicyUserParser;
impl PolicyUserParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyUser, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PolicyUser::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PolicyUser contents to a SignedRequest
struct PolicyUserWriter;
impl PolicyUserWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyUser) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
	}
}
/// Contains the response to a successful GetLoginProfile request.
#[derive(Debug, Default)]
pub struct GetLoginProfileResponse {
	/// The user name and password create date for the user.
	pub login_profile: LoginProfile,
}

/// Parse GetLoginProfileResponse from XML
struct GetLoginProfileResponseParser;
impl GetLoginProfileResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetLoginProfileResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetLoginProfileResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "LoginProfile" {
				obj.login_profile = try!(LoginProfileParser::parse_xml("LoginProfile", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetLoginProfileResponse contents to a SignedRequest
struct GetLoginProfileResponseWriter;
impl GetLoginProfileResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetLoginProfileResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		LoginProfileWriter::write_params(params, &(prefix.to_string() + "LoginProfile"), &obj.login_profile);
	}
}
#[derive(Debug, Default)]
pub struct GetRoleRequest {
	/// The name of the role to get information about.
	pub role_name: roleNameType,
}

/// Parse GetRoleRequest from XML
struct GetRoleRequestParser;
impl GetRoleRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetRoleRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetRoleRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetRoleRequest contents to a SignedRequest
struct GetRoleRequestWriter;
impl GetRoleRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetRoleRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
	}
}
#[derive(Debug, Default)]
pub struct UpdateAccountPasswordPolicyRequest {
	/// Allows all IAM users in your account to use the AWS Management Console to
	/// change their own passwords. For more information, see [Letting IAM Users
	/// Change Their Own Passwords](http://docs.aws.amazon.com/IAM/latest/UserGuide/Ho
	/// wToPwdIAMUser.html) in the _Using IAM_ guide.
	/// Default value: false
	pub allow_users_to_change_password: booleanType,
	/// Specifies whether IAM user passwords must contain at least one lowercase
	/// character from the ISO basic Latin alphabet (a to z).
	/// Default value: false
	pub require_lowercase_characters: booleanType,
	/// Specifies whether IAM user passwords must contain at least one uppercase
	/// character from the ISO basic Latin alphabet (A to Z).
	/// Default value: false
	pub require_uppercase_characters: booleanType,
	/// The minimum number of characters allowed in an IAM user password.
	/// Default value: 6
	pub minimum_password_length: minimumPasswordLengthType,
	/// Specifies whether IAM user passwords must contain at least one numeric
	/// character (0 to 9).
	/// Default value: false
	pub require_numbers: booleanType,
	/// Specifies the number of previous passwords that IAM users are prevented from
	/// reusing. The default value of 0 means IAM users are not prevented from reusing
	/// previous passwords.
	/// Default value: 0
	pub password_reuse_prevention: passwordReusePreventionType,
	/// Prevents IAM users from setting a new password after their password has
	/// expired.
	/// Default value: false
	pub hard_expiry: booleanObjectType,
	/// Specifies whether IAM user passwords must contain at least one of the
	/// following non-alphanumeric characters:
	/// ! @ # $ % ^ &amp;amp; * ( ) _ + - = [ ] { } | '
	/// Default value: false
	pub require_symbols: booleanType,
	/// The number of days that an IAM user password is valid. The default value of 0
	/// means IAM user passwords never expire.
	/// Default value: 0
	pub max_password_age: maxPasswordAgeType,
}

/// Parse UpdateAccountPasswordPolicyRequest from XML
struct UpdateAccountPasswordPolicyRequestParser;
impl UpdateAccountPasswordPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateAccountPasswordPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateAccountPasswordPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AllowUsersToChangePassword" {
				obj.allow_users_to_change_password = try!(booleanTypeParser::parse_xml("AllowUsersToChangePassword", stack));
				continue;
			}
			if current_name == "RequireLowercaseCharacters" {
				obj.require_lowercase_characters = try!(booleanTypeParser::parse_xml("RequireLowercaseCharacters", stack));
				continue;
			}
			if current_name == "RequireUppercaseCharacters" {
				obj.require_uppercase_characters = try!(booleanTypeParser::parse_xml("RequireUppercaseCharacters", stack));
				continue;
			}
			if current_name == "MinimumPasswordLength" {
				obj.minimum_password_length = try!(minimumPasswordLengthTypeParser::parse_xml("MinimumPasswordLength", stack));
				continue;
			}
			if current_name == "RequireNumbers" {
				obj.require_numbers = try!(booleanTypeParser::parse_xml("RequireNumbers", stack));
				continue;
			}
			if current_name == "PasswordReusePrevention" {
				obj.password_reuse_prevention = try!(passwordReusePreventionTypeParser::parse_xml("PasswordReusePrevention", stack));
				continue;
			}
			if current_name == "HardExpiry" {
				obj.hard_expiry = try!(booleanObjectTypeParser::parse_xml("HardExpiry", stack));
				continue;
			}
			if current_name == "RequireSymbols" {
				obj.require_symbols = try!(booleanTypeParser::parse_xml("RequireSymbols", stack));
				continue;
			}
			if current_name == "MaxPasswordAge" {
				obj.max_password_age = try!(maxPasswordAgeTypeParser::parse_xml("MaxPasswordAge", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateAccountPasswordPolicyRequest contents to a SignedRequest
struct UpdateAccountPasswordPolicyRequestWriter;
impl UpdateAccountPasswordPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateAccountPasswordPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "AllowUsersToChangePassword"), &obj.allow_users_to_change_password);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "RequireLowercaseCharacters"), &obj.require_lowercase_characters);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "RequireUppercaseCharacters"), &obj.require_uppercase_characters);
		minimumPasswordLengthTypeWriter::write_params(params, &(prefix.to_string() + "MinimumPasswordLength"), &obj.minimum_password_length);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "RequireNumbers"), &obj.require_numbers);
		passwordReusePreventionTypeWriter::write_params(params, &(prefix.to_string() + "PasswordReusePrevention"), &obj.password_reuse_prevention);
		booleanObjectTypeWriter::write_params(params, &(prefix.to_string() + "HardExpiry"), &obj.hard_expiry);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "RequireSymbols"), &obj.require_symbols);
		maxPasswordAgeTypeWriter::write_params(params, &(prefix.to_string() + "MaxPasswordAge"), &obj.max_password_age);
	}
}
pub type summaryKeyType = String;
/// Parse summaryKeyType from XML
struct summaryKeyTypeParser;
impl summaryKeyTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<summaryKeyType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write summaryKeyType contents to a SignedRequest
struct summaryKeyTypeWriter;
impl summaryKeyTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &summaryKeyType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct ListAttachedGroupPoliciesRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// The name (friendly name, not ARN) of the group to list attached policies for.
	pub group_name: groupNameType,
	/// The path prefix for filtering the results. This parameter is optional. If it
	/// is not included, it defaults to a slash (/), listing all policies.
	pub path_prefix: Option<policyPathType>,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
}

/// Parse ListAttachedGroupPoliciesRequest from XML
struct ListAttachedGroupPoliciesRequestParser;
impl ListAttachedGroupPoliciesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListAttachedGroupPoliciesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListAttachedGroupPoliciesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "PathPrefix" {
				obj.path_prefix = Some(try!(policyPathTypeParser::parse_xml("PathPrefix", stack)));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListAttachedGroupPoliciesRequest contents to a SignedRequest
struct ListAttachedGroupPoliciesRequestWriter;
impl ListAttachedGroupPoliciesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListAttachedGroupPoliciesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		if let Some(ref obj) = obj.path_prefix {
			policyPathTypeWriter::write_params(params, &(prefix.to_string() + "PathPrefix"), obj);
		}
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct ListGroupsForUserRequest {
	/// The name of the user to list groups for.
	pub user_name: existingUserNameType,
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
}

/// Parse ListGroupsForUserRequest from XML
struct ListGroupsForUserRequestParser;
impl ListGroupsForUserRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListGroupsForUserRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListGroupsForUserRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListGroupsForUserRequest contents to a SignedRequest
struct ListGroupsForUserRequestWriter;
impl ListGroupsForUserRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListGroupsForUserRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
	}
}
pub type ReportContentType = Vec<u8>;
/// Parse ReportContentType from XML
struct ReportContentTypeParser;
impl ReportContentTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ReportContentType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack)).into_bytes();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ReportContentType contents to a SignedRequest
struct ReportContentTypeWriter;
impl ReportContentTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ReportContentType) {
		params.put(name, str::from_utf8(&obj).unwrap());
	}
}
#[derive(Debug, Default)]
pub struct RemoveClientIDFromOpenIDConnectProviderRequest {
	/// The Amazon Resource Name (ARN) of the IAM OpenID Connect (OIDC) provider to
	/// remove the client ID from. You can get a list of OIDC provider ARNs by using
	/// the ListOpenIDConnectProviders action.
	pub open_id_connect_provider_arn: arnType,
	/// The client ID (also known as audience) to remove from the IAM OpenID Connect
	/// provider. For more information about client IDs, see
	/// CreateOpenIDConnectProvider.
	pub client_id: clientIDType,
}

/// Parse RemoveClientIDFromOpenIDConnectProviderRequest from XML
struct RemoveClientIDFromOpenIDConnectProviderRequestParser;
impl RemoveClientIDFromOpenIDConnectProviderRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<RemoveClientIDFromOpenIDConnectProviderRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = RemoveClientIDFromOpenIDConnectProviderRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "OpenIDConnectProviderArn" {
				obj.open_id_connect_provider_arn = try!(arnTypeParser::parse_xml("OpenIDConnectProviderArn", stack));
				continue;
			}
			if current_name == "ClientID" {
				obj.client_id = try!(clientIDTypeParser::parse_xml("ClientID", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write RemoveClientIDFromOpenIDConnectProviderRequest contents to a SignedRequest
struct RemoveClientIDFromOpenIDConnectProviderRequestWriter;
impl RemoveClientIDFromOpenIDConnectProviderRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &RemoveClientIDFromOpenIDConnectProviderRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "OpenIDConnectProviderArn"), &obj.open_id_connect_provider_arn);
		clientIDTypeWriter::write_params(params, &(prefix.to_string() + "ClientID"), &obj.client_id);
	}
}
#[derive(Debug, Default)]
pub struct DeleteVirtualMFADeviceRequest {
	/// The serial number that uniquely identifies the MFA device. For virtual MFA
	/// devices, the serial number is the same as the ARN.
	pub serial_number: serialNumberType,
}

/// Parse DeleteVirtualMFADeviceRequest from XML
struct DeleteVirtualMFADeviceRequestParser;
impl DeleteVirtualMFADeviceRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteVirtualMFADeviceRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteVirtualMFADeviceRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SerialNumber" {
				obj.serial_number = try!(serialNumberTypeParser::parse_xml("SerialNumber", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteVirtualMFADeviceRequest contents to a SignedRequest
struct DeleteVirtualMFADeviceRequestWriter;
impl DeleteVirtualMFADeviceRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteVirtualMFADeviceRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		serialNumberTypeWriter::write_params(params, &(prefix.to_string() + "SerialNumber"), &obj.serial_number);
	}
}
/// Contains a list of IAM OpenID Connect providers.
pub type OpenIDConnectProviderListType = Vec<OpenIDConnectProviderListEntry>;
/// Parse OpenIDConnectProviderListType from XML
struct OpenIDConnectProviderListTypeParser;
impl OpenIDConnectProviderListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<OpenIDConnectProviderListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "OpenIDConnectProviderListEntry" {
			obj.push(try!(OpenIDConnectProviderListEntryParser::parse_xml("OpenIDConnectProviderListEntry", stack)));
		}
		Ok(obj)
	}
}
/// Write OpenIDConnectProviderListType contents to a SignedRequest
struct OpenIDConnectProviderListTypeWriter;
impl OpenIDConnectProviderListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &OpenIDConnectProviderListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			OpenIDConnectProviderListEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type SAMLMetadataDocumentType = String;
/// Parse SAMLMetadataDocumentType from XML
struct SAMLMetadataDocumentTypeParser;
impl SAMLMetadataDocumentTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SAMLMetadataDocumentType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SAMLMetadataDocumentType contents to a SignedRequest
struct SAMLMetadataDocumentTypeWriter;
impl SAMLMetadataDocumentTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SAMLMetadataDocumentType) {
		params.put(name, obj);
	}
}
pub type policyPathType = String;
/// Parse policyPathType from XML
struct policyPathTypeParser;
impl policyPathTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<policyPathType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write policyPathType contents to a SignedRequest
struct policyPathTypeWriter;
impl policyPathTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &policyPathType) {
		params.put(name, obj);
	}
}
pub type credentialReportNotReadyExceptionMessage = String;
/// Parse credentialReportNotReadyExceptionMessage from XML
struct credentialReportNotReadyExceptionMessageParser;
impl credentialReportNotReadyExceptionMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<credentialReportNotReadyExceptionMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write credentialReportNotReadyExceptionMessage contents to a SignedRequest
struct credentialReportNotReadyExceptionMessageWriter;
impl credentialReportNotReadyExceptionMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &credentialReportNotReadyExceptionMessage) {
		params.put(name, obj);
	}
}
pub type policyDescriptionType = String;
/// Parse policyDescriptionType from XML
struct policyDescriptionTypeParser;
impl policyDescriptionTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<policyDescriptionType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write policyDescriptionType contents to a SignedRequest
struct policyDescriptionTypeWriter;
impl policyDescriptionTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &policyDescriptionType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct CreateGroupRequest {
	/// The path to the group. For more information about paths, see [IAM Identifiers]
	/// (http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in
	/// the _Using IAM_ guide.
	/// This parameter is optional. If it is not included, it defaults to a slash (/).
	pub path: Option<pathType>,
	/// The name of the group to create. Do not include the path in this value.
	pub group_name: groupNameType,
}

/// Parse CreateGroupRequest from XML
struct CreateGroupRequestParser;
impl CreateGroupRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateGroupRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateGroupRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Path" {
				obj.path = Some(try!(pathTypeParser::parse_xml("Path", stack)));
				continue;
			}
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateGroupRequest contents to a SignedRequest
struct CreateGroupRequestWriter;
impl CreateGroupRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateGroupRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.path {
			pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), obj);
		}
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
	}
}
pub type booleanType = bool;
/// Parse booleanType from XML
struct booleanTypeParser;
impl booleanTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<booleanType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write booleanType contents to a SignedRequest
struct booleanTypeWriter;
impl booleanTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &booleanType) {
		params.put(name, &obj.to_string());
	}
}
/// Contains the response to a successful ListUsers request.
#[derive(Debug, Default)]
pub struct ListUsersResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A list of users.
	pub users: userListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
}

/// Parse ListUsersResponse from XML
struct ListUsersResponseParser;
impl ListUsersResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListUsersResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListUsersResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "User" {
				obj.users = try!(userListTypeParser::parse_xml("User", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListUsersResponse contents to a SignedRequest
struct ListUsersResponseWriter;
impl ListUsersResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListUsersResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		userListTypeWriter::write_params(params, &(prefix.to_string() + "User"), &obj.users);
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
	}
}
/// Contains the Amazon Resource Name (ARN) for an IAM OpenID Connect provider.
#[derive(Debug, Default)]
pub struct OpenIDConnectProviderListEntry {
	pub arn: arnType,
}

/// Parse OpenIDConnectProviderListEntry from XML
struct OpenIDConnectProviderListEntryParser;
impl OpenIDConnectProviderListEntryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<OpenIDConnectProviderListEntry, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = OpenIDConnectProviderListEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write OpenIDConnectProviderListEntry contents to a SignedRequest
struct OpenIDConnectProviderListEntryWriter;
impl OpenIDConnectProviderListEntryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &OpenIDConnectProviderListEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
	}
}
#[derive(Debug, Default)]
pub struct ResyncMFADeviceRequest {
	/// The name of the user whose MFA device you want to resynchronize.
	pub user_name: existingUserNameType,
	/// An authentication code emitted by the device.
	pub authentication_code1: authenticationCodeType,
	/// Serial number that uniquely identifies the MFA device.
	pub serial_number: serialNumberType,
	/// A subsequent authentication code emitted by the device.
	pub authentication_code2: authenticationCodeType,
}

/// Parse ResyncMFADeviceRequest from XML
struct ResyncMFADeviceRequestParser;
impl ResyncMFADeviceRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ResyncMFADeviceRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ResyncMFADeviceRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "AuthenticationCode1" {
				obj.authentication_code1 = try!(authenticationCodeTypeParser::parse_xml("AuthenticationCode1", stack));
				continue;
			}
			if current_name == "SerialNumber" {
				obj.serial_number = try!(serialNumberTypeParser::parse_xml("SerialNumber", stack));
				continue;
			}
			if current_name == "AuthenticationCode2" {
				obj.authentication_code2 = try!(authenticationCodeTypeParser::parse_xml("AuthenticationCode2", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ResyncMFADeviceRequest contents to a SignedRequest
struct ResyncMFADeviceRequestWriter;
impl ResyncMFADeviceRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ResyncMFADeviceRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		authenticationCodeTypeWriter::write_params(params, &(prefix.to_string() + "AuthenticationCode1"), &obj.authentication_code1);
		serialNumberTypeWriter::write_params(params, &(prefix.to_string() + "SerialNumber"), &obj.serial_number);
		authenticationCodeTypeWriter::write_params(params, &(prefix.to_string() + "AuthenticationCode2"), &obj.authentication_code2);
	}
}
#[derive(Debug, Default)]
pub struct GetAccountAuthorizationDetailsRequest {
	/// A list of entity types (user, group, role, local managed policy, or AWS
	/// managed policy) for filtering the results.
	pub filter: entityListType,
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse GetAccountAuthorizationDetailsRequest from XML
struct GetAccountAuthorizationDetailsRequestParser;
impl GetAccountAuthorizationDetailsRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetAccountAuthorizationDetailsRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetAccountAuthorizationDetailsRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "EntityType" {
				obj.filter = try!(entityListTypeParser::parse_xml("EntityType", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetAccountAuthorizationDetailsRequest contents to a SignedRequest
struct GetAccountAuthorizationDetailsRequestWriter;
impl GetAccountAuthorizationDetailsRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetAccountAuthorizationDetailsRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		entityListTypeWriter::write_params(params, &(prefix.to_string() + "EntityType"), &obj.filter);
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
/// The request processing has failed because of an unknown error, exception or
/// failure.
#[derive(Debug, Default)]
pub struct ServiceFailureException {
	pub message: serviceFailureExceptionMessage,
}

/// Parse ServiceFailureException from XML
struct ServiceFailureExceptionParser;
impl ServiceFailureExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ServiceFailureException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ServiceFailureException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(serviceFailureExceptionMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ServiceFailureException contents to a SignedRequest
struct ServiceFailureExceptionWriter;
impl ServiceFailureExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ServiceFailureException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		serviceFailureExceptionMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains the response to a successful CreateOpenIDConnectProvider request.
#[derive(Debug, Default)]
pub struct CreateOpenIDConnectProviderResponse {
	/// The Amazon Resource Name (ARN) of the IAM OpenID Connect provider that was
	/// created. For more information, see OpenIDConnectProviderListEntry.
	pub open_id_connect_provider_arn: arnType,
}

/// Parse CreateOpenIDConnectProviderResponse from XML
struct CreateOpenIDConnectProviderResponseParser;
impl CreateOpenIDConnectProviderResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateOpenIDConnectProviderResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateOpenIDConnectProviderResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "OpenIDConnectProviderArn" {
				obj.open_id_connect_provider_arn = try!(arnTypeParser::parse_xml("OpenIDConnectProviderArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateOpenIDConnectProviderResponse contents to a SignedRequest
struct CreateOpenIDConnectProviderResponseWriter;
impl CreateOpenIDConnectProviderResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateOpenIDConnectProviderResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "OpenIDConnectProviderArn"), &obj.open_id_connect_provider_arn);
	}
}
#[derive(Debug, Default)]
pub struct DetachUserPolicyRequest {
	/// The name (friendly name, not ARN) of the user to detach the policy from.
	pub user_name: userNameType,
	pub policy_arn: arnType,
}

/// Parse DetachUserPolicyRequest from XML
struct DetachUserPolicyRequestParser;
impl DetachUserPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DetachUserPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DetachUserPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DetachUserPolicyRequest contents to a SignedRequest
struct DetachUserPolicyRequestWriter;
impl DetachUserPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DetachUserPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
/// Contains the response to a successful GetInstanceProfile request.
#[derive(Debug, Default)]
pub struct GetInstanceProfileResponse {
	/// Information about the instance profile.
	pub instance_profile: InstanceProfile,
}

/// Parse GetInstanceProfileResponse from XML
struct GetInstanceProfileResponseParser;
impl GetInstanceProfileResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetInstanceProfileResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetInstanceProfileResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "InstanceProfile" {
				obj.instance_profile = try!(InstanceProfileParser::parse_xml("InstanceProfile", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetInstanceProfileResponse contents to a SignedRequest
struct GetInstanceProfileResponseWriter;
impl GetInstanceProfileResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetInstanceProfileResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		InstanceProfileWriter::write_params(params, &(prefix.to_string() + "InstanceProfile"), &obj.instance_profile);
	}
}
#[derive(Debug, Default)]
pub struct PutRolePolicyRequest {
	/// The name of the role to associate the policy with.
	pub role_name: roleNameType,
	/// The policy document.
	pub policy_document: policyDocumentType,
	/// The name of the policy document.
	pub policy_name: policyNameType,
}

/// Parse PutRolePolicyRequest from XML
struct PutRolePolicyRequestParser;
impl PutRolePolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PutRolePolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PutRolePolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "PolicyDocument" {
				obj.policy_document = try!(policyDocumentTypeParser::parse_xml("PolicyDocument", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PutRolePolicyRequest contents to a SignedRequest
struct PutRolePolicyRequestWriter;
impl PutRolePolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PutRolePolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDocument"), &obj.policy_document);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
	}
}
pub type summaryValueType = i32;
/// Parse summaryValueType from XML
struct summaryValueTypeParser;
impl summaryValueTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<summaryValueType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write summaryValueType contents to a SignedRequest
struct summaryValueTypeWriter;
impl summaryValueTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &summaryValueType) {
		params.put(name, &obj.to_string());
	}
}
/// Contains information about a version of a managed policy.
/// This data type is used as a response element in the CreatePolicyVersion,
/// GetPolicyVersion, ListPolicyVersions, and GetAccountAuthorizationDetails
/// actions.
/// For more information about managed policies, refer to [Managed Policies and
/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
/// managed-vs-inline.html) in the _Using IAM_ guide.
#[derive(Debug, Default)]
pub struct PolicyVersion {
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the policy version was created.
	pub create_date: dateType,
	/// The identifier for the policy version.
	/// Policy version identifiers always begin with `v` (always lowercase). When a
	/// policy is created, the first policy version is `v1`.
	pub version_id: policyVersionIdType,
	/// The policy document.
	/// The policy document is returned in the response to the GetPolicyVersion and
	/// GetAccountAuthorizationDetails operations. It is not returned in the response
	/// to the CreatePolicyVersion or ListPolicyVersions operations.
	pub document: policyDocumentType,
	/// Specifies whether the policy version is set as the policy's default version.
	pub is_default_version: booleanType,
}

/// Parse PolicyVersion from XML
struct PolicyVersionParser;
impl PolicyVersionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyVersion, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PolicyVersion::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "VersionId" {
				obj.version_id = try!(policyVersionIdTypeParser::parse_xml("VersionId", stack));
				continue;
			}
			if current_name == "Document" {
				obj.document = try!(policyDocumentTypeParser::parse_xml("Document", stack));
				continue;
			}
			if current_name == "IsDefaultVersion" {
				obj.is_default_version = try!(booleanTypeParser::parse_xml("IsDefaultVersion", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PolicyVersion contents to a SignedRequest
struct PolicyVersionWriter;
impl PolicyVersionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyVersion) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		policyVersionIdTypeWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "Document"), &obj.document);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsDefaultVersion"), &obj.is_default_version);
	}
}
pub type malformedPolicyDocumentMessage = String;
/// Parse malformedPolicyDocumentMessage from XML
struct malformedPolicyDocumentMessageParser;
impl malformedPolicyDocumentMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<malformedPolicyDocumentMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write malformedPolicyDocumentMessage contents to a SignedRequest
struct malformedPolicyDocumentMessageWriter;
impl malformedPolicyDocumentMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &malformedPolicyDocumentMessage) {
		params.put(name, obj);
	}
}
pub type booleanObjectType = bool;
/// Parse booleanObjectType from XML
struct booleanObjectTypeParser;
impl booleanObjectTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<booleanObjectType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write booleanObjectType contents to a SignedRequest
struct booleanObjectTypeWriter;
impl booleanObjectTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &booleanObjectType) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct ListPolicyVersionsRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
	pub policy_arn: arnType,
}

/// Parse ListPolicyVersionsRequest from XML
struct ListPolicyVersionsRequestParser;
impl ListPolicyVersionsRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListPolicyVersionsRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListPolicyVersionsRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListPolicyVersionsRequest contents to a SignedRequest
struct ListPolicyVersionsRequestWriter;
impl ListPolicyVersionsRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListPolicyVersionsRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
#[derive(Debug, Default)]
pub struct UpdateOpenIDConnectProviderThumbprintRequest {
	/// A list of certificate thumbprints that are associated with the specified IAM
	/// OpenID Connect provider. For more information, see
	/// CreateOpenIDConnectProvider.
	pub thumbprint_list: thumbprintListType,
	/// The Amazon Resource Name (ARN) of the IAM OpenID Connect (OIDC) provider to
	/// update the thumbprint for. You can get a list of OIDC provider ARNs by using
	/// the ListOpenIDConnectProviders action.
	pub open_id_connect_provider_arn: arnType,
}

/// Parse UpdateOpenIDConnectProviderThumbprintRequest from XML
struct UpdateOpenIDConnectProviderThumbprintRequestParser;
impl UpdateOpenIDConnectProviderThumbprintRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateOpenIDConnectProviderThumbprintRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateOpenIDConnectProviderThumbprintRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "thumbprintType" {
				obj.thumbprint_list = try!(thumbprintListTypeParser::parse_xml("thumbprintType", stack));
				continue;
			}
			if current_name == "OpenIDConnectProviderArn" {
				obj.open_id_connect_provider_arn = try!(arnTypeParser::parse_xml("OpenIDConnectProviderArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateOpenIDConnectProviderThumbprintRequest contents to a SignedRequest
struct UpdateOpenIDConnectProviderThumbprintRequestWriter;
impl UpdateOpenIDConnectProviderThumbprintRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateOpenIDConnectProviderThumbprintRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		thumbprintListTypeWriter::write_params(params, &(prefix.to_string() + "thumbprintType"), &obj.thumbprint_list);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "OpenIDConnectProviderArn"), &obj.open_id_connect_provider_arn);
	}
}
pub type maxPasswordAgeType = i32;
/// Parse maxPasswordAgeType from XML
struct maxPasswordAgeTypeParser;
impl maxPasswordAgeTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<maxPasswordAgeType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write maxPasswordAgeType contents to a SignedRequest
struct maxPasswordAgeTypeWriter;
impl maxPasswordAgeTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &maxPasswordAgeType) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct ListUserPoliciesRequest {
	/// The name of the user to list policies for.
	pub user_name: existingUserNameType,
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
}

/// Parse ListUserPoliciesRequest from XML
struct ListUserPoliciesRequestParser;
impl ListUserPoliciesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListUserPoliciesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListUserPoliciesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListUserPoliciesRequest contents to a SignedRequest
struct ListUserPoliciesRequestWriter;
impl ListUserPoliciesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListUserPoliciesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct GetGroupPolicyRequest {
	/// The name of the group the policy is associated with.
	pub group_name: groupNameType,
	/// The name of the policy document to get.
	pub policy_name: policyNameType,
}

/// Parse GetGroupPolicyRequest from XML
struct GetGroupPolicyRequestParser;
impl GetGroupPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetGroupPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetGroupPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetGroupPolicyRequest contents to a SignedRequest
struct GetGroupPolicyRequestWriter;
impl GetGroupPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetGroupPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
	}
}
/// The request was rejected because it referenced an entity that does not exist.
/// The error message describes the entity.
#[derive(Debug, Default)]
pub struct NoSuchEntityException {
	pub message: noSuchEntityMessage,
}

/// Parse NoSuchEntityException from XML
struct NoSuchEntityExceptionParser;
impl NoSuchEntityExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<NoSuchEntityException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = NoSuchEntityException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(noSuchEntityMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write NoSuchEntityException contents to a SignedRequest
struct NoSuchEntityExceptionWriter;
impl NoSuchEntityExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &NoSuchEntityException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		noSuchEntityMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// The request was rejected because the authentication code was not recognized.
/// The error message describes the specific error.
#[derive(Debug, Default)]
pub struct InvalidAuthenticationCodeException {
	pub message: invalidAuthenticationCodeMessage,
}

/// Parse InvalidAuthenticationCodeException from XML
struct InvalidAuthenticationCodeExceptionParser;
impl InvalidAuthenticationCodeExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidAuthenticationCodeException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidAuthenticationCodeException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(invalidAuthenticationCodeMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidAuthenticationCodeException contents to a SignedRequest
struct InvalidAuthenticationCodeExceptionWriter;
impl InvalidAuthenticationCodeExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidAuthenticationCodeException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		invalidAuthenticationCodeMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type summaryMapType = HashMap<summaryKeyType,summaryValueType>;
/// Parse summaryMapType from XML
struct summaryMapTypeParser;
impl summaryMapTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<summaryMapType, XmlParseError> {
		let mut obj = HashMap::new();
		while try!(peek_at_name(stack)) == tag_name {
			try!(start_element(tag_name, stack));
			let key = try!(summaryKeyTypeParser::parse_xml("summaryKeyType", stack));
			let value = try!(summaryValueTypeParser::parse_xml("summaryValueType", stack));
			obj.insert(key, value);
			try!(end_element(tag_name, stack));
		}
		Ok(obj)
	}
}
/// Write summaryMapType contents to a SignedRequest
struct summaryMapTypeWriter;
impl summaryMapTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &summaryMapType) {
		let mut index = 1;
		for (key,value) in obj {
			let prefix = &format!("{}.{}", name, index);
			summaryKeyTypeWriter::write_params(params, &format!("{}.{}", prefix, "summaryKeyType"), &key);
			summaryValueTypeWriter::write_params(params, &format!("{}.{}", prefix, "summaryValueType"), &value);
			index += 1;
		}
	}
}
pub type malformedCertificateMessage = String;
/// Parse malformedCertificateMessage from XML
struct malformedCertificateMessageParser;
impl malformedCertificateMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<malformedCertificateMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write malformedCertificateMessage contents to a SignedRequest
struct malformedCertificateMessageWriter;
impl malformedCertificateMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &malformedCertificateMessage) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct GetGroupRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// The name of the group.
	pub group_name: groupNameType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
}

/// Parse GetGroupRequest from XML
struct GetGroupRequestParser;
impl GetGroupRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetGroupRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetGroupRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetGroupRequest contents to a SignedRequest
struct GetGroupRequestWriter;
impl GetGroupRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetGroupRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
	}
}
pub type publicKeyIdType = String;
/// Parse publicKeyIdType from XML
struct publicKeyIdTypeParser;
impl publicKeyIdTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<publicKeyIdType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write publicKeyIdType contents to a SignedRequest
struct publicKeyIdTypeWriter;
impl publicKeyIdTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &publicKeyIdType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct ListRolesRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// The path prefix for filtering the results. For example, the prefix
	/// `/application_abc/component_xyz/` gets all roles whose path starts with
	/// `/application_abc/component_xyz/`.
	/// This parameter is optional. If it is not included, it defaults to a slash (/),
	/// listing all roles.
	pub path_prefix: pathPrefixType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListRolesRequest from XML
struct ListRolesRequestParser;
impl ListRolesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListRolesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListRolesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "PathPrefix" {
				obj.path_prefix = try!(pathPrefixTypeParser::parse_xml("PathPrefix", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListRolesRequest contents to a SignedRequest
struct ListRolesRequestWriter;
impl ListRolesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListRolesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		pathPrefixTypeWriter::write_params(params, &(prefix.to_string() + "PathPrefix"), &obj.path_prefix);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
#[derive(Debug, Default)]
pub struct ListSSHPublicKeysRequest {
	/// The name of the IAM user to list SSH public keys for. If none is specified,
	/// the UserName field is determined implicitly based on the AWS access key used
	/// to sign the request.
	pub user_name: userNameType,
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListSSHPublicKeysRequest from XML
struct ListSSHPublicKeysRequestParser;
impl ListSSHPublicKeysRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSSHPublicKeysRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListSSHPublicKeysRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListSSHPublicKeysRequest contents to a SignedRequest
struct ListSSHPublicKeysRequestWriter;
impl ListSSHPublicKeysRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListSSHPublicKeysRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
#[derive(Debug, Default)]
pub struct DeleteRolePolicyRequest {
	/// The name (friendly name, not ARN) identifying the role that the policy is
	/// embedded in.
	pub role_name: roleNameType,
	/// The name identifying the policy document to delete.
	pub policy_name: policyNameType,
}

/// Parse DeleteRolePolicyRequest from XML
struct DeleteRolePolicyRequestParser;
impl DeleteRolePolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteRolePolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteRolePolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteRolePolicyRequest contents to a SignedRequest
struct DeleteRolePolicyRequestWriter;
impl DeleteRolePolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteRolePolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
	}
}
pub type userDetailListType = Vec<UserDetail>;
/// Parse userDetailListType from XML
struct userDetailListTypeParser;
impl userDetailListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<userDetailListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "UserDetail" {
			obj.push(try!(UserDetailParser::parse_xml("UserDetail", stack)));
		}
		Ok(obj)
	}
}
/// Write userDetailListType contents to a SignedRequest
struct userDetailListTypeWriter;
impl userDetailListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &userDetailListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			UserDetailWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct ListVirtualMFADevicesRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// The status (unassigned or assigned) of the devices to list. If you do not
	/// specify an `AssignmentStatus`, the action defaults to `Any` which lists both
	/// assigned and unassigned virtual MFA devices.
	pub assignment_status: assignmentStatusType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListVirtualMFADevicesRequest from XML
struct ListVirtualMFADevicesRequestParser;
impl ListVirtualMFADevicesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListVirtualMFADevicesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListVirtualMFADevicesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "AssignmentStatus" {
				obj.assignment_status = try!(assignmentStatusTypeParser::parse_xml("AssignmentStatus", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListVirtualMFADevicesRequest contents to a SignedRequest
struct ListVirtualMFADevicesRequestWriter;
impl ListVirtualMFADevicesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListVirtualMFADevicesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		assignmentStatusTypeWriter::write_params(params, &(prefix.to_string() + "AssignmentStatus"), &obj.assignment_status);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
#[derive(Debug, Default)]
pub struct ListSAMLProvidersRequest;

/// Parse ListSAMLProvidersRequest from XML
struct ListSAMLProvidersRequestParser;
impl ListSAMLProvidersRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSAMLProvidersRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListSAMLProvidersRequest::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListSAMLProvidersRequest contents to a SignedRequest
struct ListSAMLProvidersRequestWriter;
impl ListSAMLProvidersRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListSAMLProvidersRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
#[derive(Debug, Default)]
pub struct UploadServerCertificateRequest {
	/// The path for the server certificate. For more information about paths, see
	/// [IAM Identifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identi
	/// fiers.html) in the _Using IAM_ guide.
	/// This parameter is optional. If it is not included, it defaults to a slash (/).
	/// If you are uploading a server certificate specifically for use with Amazon
	/// CloudFront distributions, you must specify a path using the `--path` option.
	/// The path must begin with `/cloudfront` and must include a trailing slash (for
	/// example, `/cloudfront/test/`).
	pub path: Option<pathType>,
	/// The contents of the public key certificate in PEM-encoded format.
	pub certificate_body: certificateBodyType,
	/// The contents of the private key in PEM-encoded format.
	pub private_key: privateKeyType,
	/// The name for the server certificate. Do not include the path in this value.
	/// The name of the certificate cannot contain any spaces.
	pub server_certificate_name: serverCertificateNameType,
	/// The contents of the certificate chain. This is typically a concatenation of
	/// the PEM-encoded public key certificates of the chain.
	pub certificate_chain: Option<certificateChainType>,
}

/// Parse UploadServerCertificateRequest from XML
struct UploadServerCertificateRequestParser;
impl UploadServerCertificateRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UploadServerCertificateRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UploadServerCertificateRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Path" {
				obj.path = Some(try!(pathTypeParser::parse_xml("Path", stack)));
				continue;
			}
			if current_name == "CertificateBody" {
				obj.certificate_body = try!(certificateBodyTypeParser::parse_xml("CertificateBody", stack));
				continue;
			}
			if current_name == "PrivateKey" {
				obj.private_key = try!(privateKeyTypeParser::parse_xml("PrivateKey", stack));
				continue;
			}
			if current_name == "ServerCertificateName" {
				obj.server_certificate_name = try!(serverCertificateNameTypeParser::parse_xml("ServerCertificateName", stack));
				continue;
			}
			if current_name == "CertificateChain" {
				obj.certificate_chain = Some(try!(certificateChainTypeParser::parse_xml("CertificateChain", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UploadServerCertificateRequest contents to a SignedRequest
struct UploadServerCertificateRequestWriter;
impl UploadServerCertificateRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UploadServerCertificateRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.path {
			pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), obj);
		}
		certificateBodyTypeWriter::write_params(params, &(prefix.to_string() + "CertificateBody"), &obj.certificate_body);
		privateKeyTypeWriter::write_params(params, &(prefix.to_string() + "PrivateKey"), &obj.private_key);
		serverCertificateNameTypeWriter::write_params(params, &(prefix.to_string() + "ServerCertificateName"), &obj.server_certificate_name);
		if let Some(ref obj) = obj.certificate_chain {
			certificateChainTypeWriter::write_params(params, &(prefix.to_string() + "CertificateChain"), obj);
		}
	}
}
/// Contains the response to a successful CreateAccessKey request.
#[derive(Debug, Default)]
pub struct CreateAccessKeyResponse {
	/// Information about the access key.
	pub access_key: AccessKey,
}

/// Parse CreateAccessKeyResponse from XML
struct CreateAccessKeyResponseParser;
impl CreateAccessKeyResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateAccessKeyResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateAccessKeyResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AccessKey" {
				obj.access_key = try!(AccessKeyParser::parse_xml("AccessKey", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateAccessKeyResponse contents to a SignedRequest
struct CreateAccessKeyResponseWriter;
impl CreateAccessKeyResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateAccessKeyResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AccessKeyWriter::write_params(params, &(prefix.to_string() + "AccessKey"), &obj.access_key);
	}
}
/// Contains the user name and password create date for a user.
/// This data type is used as a response element in the CreateLoginProfile and
/// GetLoginProfile actions.
#[derive(Debug, Default)]
pub struct LoginProfile {
	/// The name of the user, which can be used for signing in to the AWS Management
	/// Console.
	pub user_name: userNameType,
	/// The date when the password for the user was created.
	pub create_date: dateType,
	/// Specifies whether the user is required to set a new password on next sign-in.
	pub password_reset_required: Option<booleanType>,
}

/// Parse LoginProfile from XML
struct LoginProfileParser;
impl LoginProfileParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LoginProfile, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = LoginProfile::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "PasswordResetRequired" {
				obj.password_reset_required = Some(try!(booleanTypeParser::parse_xml("PasswordResetRequired", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LoginProfile contents to a SignedRequest
struct LoginProfileWriter;
impl LoginProfileWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LoginProfile) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		if let Some(ref obj) = obj.password_reset_required {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "PasswordResetRequired"), obj);
		}
	}
}
/// Contains a list of access key metadata.
/// This data type is used as a response element in the ListAccessKeys action.
pub type accessKeyMetadataListType = Vec<AccessKeyMetadata>;
/// Parse accessKeyMetadataListType from XML
struct accessKeyMetadataListTypeParser;
impl accessKeyMetadataListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<accessKeyMetadataListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AccessKeyMetadata" {
			obj.push(try!(AccessKeyMetadataParser::parse_xml("AccessKeyMetadata", stack)));
		}
		Ok(obj)
	}
}
/// Write accessKeyMetadataListType contents to a SignedRequest
struct accessKeyMetadataListTypeWriter;
impl accessKeyMetadataListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &accessKeyMetadataListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AccessKeyMetadataWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct CreateAccessKeyRequest {
	/// The user name that the new key will belong to.
	pub user_name: existingUserNameType,
}

/// Parse CreateAccessKeyRequest from XML
struct CreateAccessKeyRequestParser;
impl CreateAccessKeyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateAccessKeyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateAccessKeyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateAccessKeyRequest contents to a SignedRequest
struct CreateAccessKeyRequestWriter;
impl CreateAccessKeyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateAccessKeyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
	}
}
/// Contains information about an instance profile.
/// This data type is used as a response element in the following actions:
///   * CreateInstanceProfile
///   * GetInstanceProfile
///   * ListInstanceProfiles
///   * ListInstanceProfilesForRole
#[derive(Debug, Default)]
pub struct InstanceProfile {
	/// The stable and unique string identifying the instance profile. For more
	/// information about IDs, see [IAM Identifiers](http://docs.aws.amazon.com/IAM/la
	/// test/UserGuide/Using_Identifiers.html) in the _Using IAM_ guide.
	pub instance_profile_id: idType,
	/// The role associated with the instance profile.
	pub roles: roleListType,
	/// The date when the instance profile was created.
	pub create_date: dateType,
	/// The name identifying the instance profile.
	pub instance_profile_name: instanceProfileNameType,
	/// The path to the instance profile. For more information about paths, see [IAM I
	/// dentifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.
	/// html) in the _Using IAM_ guide.
	pub path: pathType,
	/// The Amazon Resource Name (ARN) specifying the instance profile. For more
	/// information about ARNs and how to use them in policies, see [IAM Identifiers](
	/// http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the
	/// _Using IAM_ guide.
	pub arn: arnType,
}

/// Parse InstanceProfile from XML
struct InstanceProfileParser;
impl InstanceProfileParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InstanceProfile, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InstanceProfile::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "InstanceProfileId" {
				obj.instance_profile_id = try!(idTypeParser::parse_xml("InstanceProfileId", stack));
				continue;
			}
			if current_name == "Role" {
				obj.roles = try!(roleListTypeParser::parse_xml("Role", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "InstanceProfileName" {
				obj.instance_profile_name = try!(instanceProfileNameTypeParser::parse_xml("InstanceProfileName", stack));
				continue;
			}
			if current_name == "Path" {
				obj.path = try!(pathTypeParser::parse_xml("Path", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InstanceProfile contents to a SignedRequest
struct InstanceProfileWriter;
impl InstanceProfileWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InstanceProfile) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		idTypeWriter::write_params(params, &(prefix.to_string() + "InstanceProfileId"), &obj.instance_profile_id);
		roleListTypeWriter::write_params(params, &(prefix.to_string() + "Role"), &obj.roles);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		instanceProfileNameTypeWriter::write_params(params, &(prefix.to_string() + "InstanceProfileName"), &obj.instance_profile_name);
		pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), &obj.path);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
	}
}
/// Contains information about a role that a managed policy is attached to.
/// This data type is used as a response element in the ListEntitiesForPolicy
/// action.
/// For more information about managed policies, refer to [Managed Policies and
/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
/// managed-vs-inline.html) in the _Using IAM_ guide.
#[derive(Debug, Default)]
pub struct PolicyRole {
	/// The name (friendly name, not ARN) identifying the role.
	pub role_name: roleNameType,
}

/// Parse PolicyRole from XML
struct PolicyRoleParser;
impl PolicyRoleParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyRole, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PolicyRole::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PolicyRole contents to a SignedRequest
struct PolicyRoleWriter;
impl PolicyRoleWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyRole) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
	}
}
#[derive(Debug, Default)]
pub struct UpdateLoginProfileRequest {
	/// The name of the user whose password you want to update.
	pub user_name: userNameType,
	/// Require the specified user to set a new password on next sign-in.
	pub password_reset_required: Option<booleanObjectType>,
	/// The new password for the specified user.
	pub password: Option<passwordType>,
}

/// Parse UpdateLoginProfileRequest from XML
struct UpdateLoginProfileRequestParser;
impl UpdateLoginProfileRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateLoginProfileRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateLoginProfileRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "PasswordResetRequired" {
				obj.password_reset_required = Some(try!(booleanObjectTypeParser::parse_xml("PasswordResetRequired", stack)));
				continue;
			}
			if current_name == "Password" {
				obj.password = Some(try!(passwordTypeParser::parse_xml("Password", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateLoginProfileRequest contents to a SignedRequest
struct UpdateLoginProfileRequestWriter;
impl UpdateLoginProfileRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateLoginProfileRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		if let Some(ref obj) = obj.password_reset_required {
			booleanObjectTypeWriter::write_params(params, &(prefix.to_string() + "PasswordResetRequired"), obj);
		}
		if let Some(ref obj) = obj.password {
			passwordTypeWriter::write_params(params, &(prefix.to_string() + "Password"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct CreateInstanceProfileRequest {
	/// The path to the instance profile. For more information about paths, see [IAM I
	/// dentifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.
	/// html) in the _Using IAM_ guide.
	/// This parameter is optional. If it is not included, it defaults to a slash (/).
	pub path: Option<pathType>,
	/// The name of the instance profile to create.
	pub instance_profile_name: instanceProfileNameType,
}

/// Parse CreateInstanceProfileRequest from XML
struct CreateInstanceProfileRequestParser;
impl CreateInstanceProfileRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateInstanceProfileRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateInstanceProfileRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Path" {
				obj.path = Some(try!(pathTypeParser::parse_xml("Path", stack)));
				continue;
			}
			if current_name == "InstanceProfileName" {
				obj.instance_profile_name = try!(instanceProfileNameTypeParser::parse_xml("InstanceProfileName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateInstanceProfileRequest contents to a SignedRequest
struct CreateInstanceProfileRequestWriter;
impl CreateInstanceProfileRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateInstanceProfileRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.path {
			pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), obj);
		}
		instanceProfileNameTypeWriter::write_params(params, &(prefix.to_string() + "InstanceProfileName"), &obj.instance_profile_name);
	}
}
/// Contains the response to a successful ListGroupsForUser request.
#[derive(Debug, Default)]
pub struct ListGroupsForUserResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
	/// A list of groups.
	pub groups: groupListType,
}

/// Parse ListGroupsForUserResponse from XML
struct ListGroupsForUserResponseParser;
impl ListGroupsForUserResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListGroupsForUserResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListGroupsForUserResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			if current_name == "Group" {
				obj.groups = try!(groupListTypeParser::parse_xml("Group", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListGroupsForUserResponse contents to a SignedRequest
struct ListGroupsForUserResponseWriter;
impl ListGroupsForUserResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListGroupsForUserResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
		groupListTypeWriter::write_params(params, &(prefix.to_string() + "Group"), &obj.groups);
	}
}
/// The request was rejected because an invalid or out-of-range value was supplied
/// for an input parameter.
#[derive(Debug, Default)]
pub struct InvalidInputException {
	pub message: invalidInputMessage,
}

/// Parse InvalidInputException from XML
struct InvalidInputExceptionParser;
impl InvalidInputExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidInputException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidInputException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(invalidInputMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidInputException contents to a SignedRequest
struct InvalidInputExceptionWriter;
impl InvalidInputExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidInputException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		invalidInputMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains the response to a successful CreatePolicyVersion request.
#[derive(Debug, Default)]
pub struct CreatePolicyVersionResponse {
	/// Information about the policy version.
	pub policy_version: PolicyVersion,
}

/// Parse CreatePolicyVersionResponse from XML
struct CreatePolicyVersionResponseParser;
impl CreatePolicyVersionResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreatePolicyVersionResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreatePolicyVersionResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyVersion" {
				obj.policy_version = try!(PolicyVersionParser::parse_xml("PolicyVersion", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreatePolicyVersionResponse contents to a SignedRequest
struct CreatePolicyVersionResponseWriter;
impl CreatePolicyVersionResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreatePolicyVersionResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		PolicyVersionWriter::write_params(params, &(prefix.to_string() + "PolicyVersion"), &obj.policy_version);
	}
}
pub type serverCertificateNameType = String;
/// Parse serverCertificateNameType from XML
struct serverCertificateNameTypeParser;
impl serverCertificateNameTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<serverCertificateNameType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write serverCertificateNameType contents to a SignedRequest
struct serverCertificateNameTypeWriter;
impl serverCertificateNameTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &serverCertificateNameType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct PutUserPolicyRequest {
	/// The name of the user to associate the policy with.
	pub user_name: existingUserNameType,
	/// The name of the policy document.
	pub policy_name: policyNameType,
	/// The policy document.
	pub policy_document: policyDocumentType,
}

/// Parse PutUserPolicyRequest from XML
struct PutUserPolicyRequestParser;
impl PutUserPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PutUserPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PutUserPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "PolicyDocument" {
				obj.policy_document = try!(policyDocumentTypeParser::parse_xml("PolicyDocument", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PutUserPolicyRequest contents to a SignedRequest
struct PutUserPolicyRequestWriter;
impl PutUserPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PutUserPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDocument"), &obj.policy_document);
	}
}
/// Contains the response to a successful GenerateCredentialReport request.
#[derive(Debug, Default)]
pub struct GenerateCredentialReportResponse {
	/// Information about the state of the credential report.
	pub state: ReportStateType,
	/// Information about the credential report.
	pub description: ReportStateDescriptionType,
}

/// Parse GenerateCredentialReportResponse from XML
struct GenerateCredentialReportResponseParser;
impl GenerateCredentialReportResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GenerateCredentialReportResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GenerateCredentialReportResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "State" {
				obj.state = try!(ReportStateTypeParser::parse_xml("State", stack));
				continue;
			}
			if current_name == "Description" {
				obj.description = try!(ReportStateDescriptionTypeParser::parse_xml("Description", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GenerateCredentialReportResponse contents to a SignedRequest
struct GenerateCredentialReportResponseWriter;
impl GenerateCredentialReportResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GenerateCredentialReportResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ReportStateTypeWriter::write_params(params, &(prefix.to_string() + "State"), &obj.state);
		ReportStateDescriptionTypeWriter::write_params(params, &(prefix.to_string() + "Description"), &obj.description);
	}
}
/// The request was rejected because it attempted to create resources beyond the
/// current AWS account limits. The error message describes the limit exceeded.
#[derive(Debug, Default)]
pub struct LimitExceededException {
	pub message: limitExceededMessage,
}

/// Parse LimitExceededException from XML
struct LimitExceededExceptionParser;
impl LimitExceededExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LimitExceededException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = LimitExceededException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(limitExceededMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LimitExceededException contents to a SignedRequest
struct LimitExceededExceptionWriter;
impl LimitExceededExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LimitExceededException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		limitExceededMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type keyPairMismatchMessage = String;
/// Parse keyPairMismatchMessage from XML
struct keyPairMismatchMessageParser;
impl keyPairMismatchMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<keyPairMismatchMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write keyPairMismatchMessage contents to a SignedRequest
struct keyPairMismatchMessageWriter;
impl keyPairMismatchMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &keyPairMismatchMessage) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct DeleteAccessKeyRequest {
	/// The name of the user whose key you want to delete.
	pub user_name: Option<existingUserNameType>,
	/// The access key ID for the access key ID and secret access key you want to
	/// delete.
	pub access_key_id: accessKeyIdType,
}

/// Parse DeleteAccessKeyRequest from XML
struct DeleteAccessKeyRequestParser;
impl DeleteAccessKeyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteAccessKeyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteAccessKeyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = Some(try!(existingUserNameTypeParser::parse_xml("UserName", stack)));
				continue;
			}
			if current_name == "AccessKeyId" {
				obj.access_key_id = try!(accessKeyIdTypeParser::parse_xml("AccessKeyId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteAccessKeyRequest contents to a SignedRequest
struct DeleteAccessKeyRequestWriter;
impl DeleteAccessKeyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteAccessKeyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.user_name {
			existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), obj);
		}
		accessKeyIdTypeWriter::write_params(params, &(prefix.to_string() + "AccessKeyId"), &obj.access_key_id);
	}
}
#[derive(Debug, Default)]
pub struct DeleteSSHPublicKeyRequest {
	/// The name of the IAM user associated with the SSH public key.
	pub user_name: userNameType,
	/// The unique identifier for the SSH public key.
	pub ssh_public_key_id: publicKeyIdType,
}

/// Parse DeleteSSHPublicKeyRequest from XML
struct DeleteSSHPublicKeyRequestParser;
impl DeleteSSHPublicKeyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteSSHPublicKeyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteSSHPublicKeyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "SSHPublicKeyId" {
				obj.ssh_public_key_id = try!(publicKeyIdTypeParser::parse_xml("SSHPublicKeyId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteSSHPublicKeyRequest contents to a SignedRequest
struct DeleteSSHPublicKeyRequestWriter;
impl DeleteSSHPublicKeyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteSSHPublicKeyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		publicKeyIdTypeWriter::write_params(params, &(prefix.to_string() + "SSHPublicKeyId"), &obj.ssh_public_key_id);
	}
}
pub type PolicyUserListType = Vec<PolicyUser>;
/// Parse PolicyUserListType from XML
struct PolicyUserListTypeParser;
impl PolicyUserListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyUserListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "PolicyUser" {
			obj.push(try!(PolicyUserParser::parse_xml("PolicyUser", stack)));
		}
		Ok(obj)
	}
}
/// Write PolicyUserListType contents to a SignedRequest
struct PolicyUserListTypeWriter;
impl PolicyUserListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyUserListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			PolicyUserWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type policyDocumentVersionListType = Vec<PolicyVersion>;
/// Parse policyDocumentVersionListType from XML
struct policyDocumentVersionListTypeParser;
impl policyDocumentVersionListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<policyDocumentVersionListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "PolicyVersion" {
			obj.push(try!(PolicyVersionParser::parse_xml("PolicyVersion", stack)));
		}
		Ok(obj)
	}
}
/// Write policyDocumentVersionListType contents to a SignedRequest
struct policyDocumentVersionListTypeWriter;
impl policyDocumentVersionListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &policyDocumentVersionListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			PolicyVersionWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Contains the response to a successful ListServerCertificates request.
#[derive(Debug, Default)]
pub struct ListServerCertificatesResponse {
	/// A list of server certificates.
	pub server_certificate_metadata_list: serverCertificateMetadataListType,
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
}

/// Parse ListServerCertificatesResponse from XML
struct ListServerCertificatesResponseParser;
impl ListServerCertificatesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListServerCertificatesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListServerCertificatesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ServerCertificateMetadata" {
				obj.server_certificate_metadata_list = try!(serverCertificateMetadataListTypeParser::parse_xml("ServerCertificateMetadata", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListServerCertificatesResponse contents to a SignedRequest
struct ListServerCertificatesResponseWriter;
impl ListServerCertificatesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListServerCertificatesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		serverCertificateMetadataListTypeWriter::write_params(params, &(prefix.to_string() + "ServerCertificateMetadata"), &obj.server_certificate_metadata_list);
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
	}
}
pub type limitExceededMessage = String;
/// Parse limitExceededMessage from XML
struct limitExceededMessageParser;
impl limitExceededMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<limitExceededMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write limitExceededMessage contents to a SignedRequest
struct limitExceededMessageWriter;
impl limitExceededMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &limitExceededMessage) {
		params.put(name, obj);
	}
}
/// Contains information about an X.509 signing certificate.
/// This data type is used as a response element in the UploadSigningCertificate
/// and ListSigningCertificates actions.
#[derive(Debug, Default)]
pub struct SigningCertificate {
	/// The name of the user the signing certificate is associated with.
	pub user_name: userNameType,
	/// The status of the signing certificate. `Active` means the key is valid for API
	/// calls, while `Inactive` means it is not.
	pub status: statusType,
	/// The contents of the signing certificate.
	pub certificate_body: certificateBodyType,
	/// The ID for the signing certificate.
	pub certificate_id: certificateIdType,
	/// The date when the signing certificate was uploaded.
	pub upload_date: Option<dateType>,
}

/// Parse SigningCertificate from XML
struct SigningCertificateParser;
impl SigningCertificateParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SigningCertificate, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SigningCertificate::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Status" {
				obj.status = try!(statusTypeParser::parse_xml("Status", stack));
				continue;
			}
			if current_name == "CertificateBody" {
				obj.certificate_body = try!(certificateBodyTypeParser::parse_xml("CertificateBody", stack));
				continue;
			}
			if current_name == "CertificateId" {
				obj.certificate_id = try!(certificateIdTypeParser::parse_xml("CertificateId", stack));
				continue;
			}
			if current_name == "UploadDate" {
				obj.upload_date = Some(try!(dateTypeParser::parse_xml("UploadDate", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SigningCertificate contents to a SignedRequest
struct SigningCertificateWriter;
impl SigningCertificateWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SigningCertificate) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		statusTypeWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		certificateBodyTypeWriter::write_params(params, &(prefix.to_string() + "CertificateBody"), &obj.certificate_body);
		certificateIdTypeWriter::write_params(params, &(prefix.to_string() + "CertificateId"), &obj.certificate_id);
		if let Some(ref obj) = obj.upload_date {
			dateTypeWriter::write_params(params, &(prefix.to_string() + "UploadDate"), obj);
		}
	}
}
/// The request was rejected because the public key is malformed or otherwise
/// invalid.
#[derive(Debug, Default)]
pub struct InvalidPublicKeyException {
	pub message: invalidPublicKeyMessage,
}

/// Parse InvalidPublicKeyException from XML
struct InvalidPublicKeyExceptionParser;
impl InvalidPublicKeyExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidPublicKeyException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidPublicKeyException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(invalidPublicKeyMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidPublicKeyException contents to a SignedRequest
struct InvalidPublicKeyExceptionWriter;
impl InvalidPublicKeyExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidPublicKeyException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		invalidPublicKeyMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
#[derive(Debug, Default)]
pub struct UpdateSigningCertificateRequest {
	/// The name of the user the signing certificate belongs to.
	pub user_name: Option<existingUserNameType>,
	/// The status you want to assign to the certificate. `Active` means the
	/// certificate can be used for API calls to AWS, while `Inactive` means the
	/// certificate cannot be used.
	pub status: statusType,
	/// The ID of the signing certificate you want to update.
	pub certificate_id: certificateIdType,
}

/// Parse UpdateSigningCertificateRequest from XML
struct UpdateSigningCertificateRequestParser;
impl UpdateSigningCertificateRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateSigningCertificateRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateSigningCertificateRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = Some(try!(existingUserNameTypeParser::parse_xml("UserName", stack)));
				continue;
			}
			if current_name == "Status" {
				obj.status = try!(statusTypeParser::parse_xml("Status", stack));
				continue;
			}
			if current_name == "CertificateId" {
				obj.certificate_id = try!(certificateIdTypeParser::parse_xml("CertificateId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateSigningCertificateRequest contents to a SignedRequest
struct UpdateSigningCertificateRequestWriter;
impl UpdateSigningCertificateRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateSigningCertificateRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.user_name {
			existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), obj);
		}
		statusTypeWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		certificateIdTypeWriter::write_params(params, &(prefix.to_string() + "CertificateId"), &obj.certificate_id);
	}
}
pub type invalidUserTypeMessage = String;
/// Parse invalidUserTypeMessage from XML
struct invalidUserTypeMessageParser;
impl invalidUserTypeMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<invalidUserTypeMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write invalidUserTypeMessage contents to a SignedRequest
struct invalidUserTypeMessageWriter;
impl invalidUserTypeMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &invalidUserTypeMessage) {
		params.put(name, obj);
	}
}
/// Contains information about the last time an AWS access key was used.
/// This data type is used as a response element in the GetAccessKeyLastUsed
/// action.
#[derive(Debug, Default)]
pub struct AccessKeyLastUsed {
	/// The AWS region where this access key was most recently used. This field is
	/// null when:
	///   * The user does not have an access key.
	///   * An access key exists but has never been used, at least not since IAM started tracking this information on April 22nd, 2015.
	///   * There is no sign-in data associated with the user
	/// For more information about AWS regions, see [Regions and
	/// Endpoints](http://docs.aws.amazon.com/general/latest/gr/rande.html) in the
	/// Amazon Web Services General Reference.
	pub region: stringType,
	/// The name of the AWS service with which this access key was most recently used.
	/// This field is null when:
	///   * The user does not have an access key.
	///   * An access key exists but has never been used, at least not since IAM started tracking this information on April 22nd, 2015.
	///   * There is no sign-in data associated with the user
	pub service_name: stringType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the access key was most recently
	/// used. This field is null when:
	///   * The user does not have an access key.
	///   * An access key exists but has never been used, at least not since IAM started tracking this information on April 22nd, 2015.
	///   * There is no sign-in data associated with the user
	pub last_used_date: dateType,
}

/// Parse AccessKeyLastUsed from XML
struct AccessKeyLastUsedParser;
impl AccessKeyLastUsedParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AccessKeyLastUsed, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AccessKeyLastUsed::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Region" {
				obj.region = try!(stringTypeParser::parse_xml("Region", stack));
				continue;
			}
			if current_name == "ServiceName" {
				obj.service_name = try!(stringTypeParser::parse_xml("ServiceName", stack));
				continue;
			}
			if current_name == "LastUsedDate" {
				obj.last_used_date = try!(dateTypeParser::parse_xml("LastUsedDate", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AccessKeyLastUsed contents to a SignedRequest
struct AccessKeyLastUsedWriter;
impl AccessKeyLastUsedWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AccessKeyLastUsed) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringTypeWriter::write_params(params, &(prefix.to_string() + "Region"), &obj.region);
		stringTypeWriter::write_params(params, &(prefix.to_string() + "ServiceName"), &obj.service_name);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "LastUsedDate"), &obj.last_used_date);
	}
}
/// Contains the response to a successful CreateUser request.
#[derive(Debug, Default)]
pub struct CreateUserResponse {
	/// Information about the user.
	pub user: User,
}

/// Parse CreateUserResponse from XML
struct CreateUserResponseParser;
impl CreateUserResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateUserResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateUserResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "User" {
				obj.user = try!(UserParser::parse_xml("User", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateUserResponse contents to a SignedRequest
struct CreateUserResponseWriter;
impl CreateUserResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateUserResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		UserWriter::write_params(params, &(prefix.to_string() + "User"), &obj.user);
	}
}
#[derive(Debug, Default)]
pub struct CreatePolicyVersionRequest {
	/// Specifies whether to set this version as the policy's default version.
	/// When this parameter is `true`, the new policy version becomes the operative
	/// version; that is, the version that is in effect for the IAM users, groups, and
	/// roles that the policy is attached to.
	/// For more information about managed policy versions, see [Versioning for
	/// Managed Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-versions.html) in the _Using IAM_ guide.
	pub set_as_default: Option<booleanType>,
	/// The policy document.
	pub policy_document: policyDocumentType,
	pub policy_arn: arnType,
}

/// Parse CreatePolicyVersionRequest from XML
struct CreatePolicyVersionRequestParser;
impl CreatePolicyVersionRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreatePolicyVersionRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreatePolicyVersionRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SetAsDefault" {
				obj.set_as_default = Some(try!(booleanTypeParser::parse_xml("SetAsDefault", stack)));
				continue;
			}
			if current_name == "PolicyDocument" {
				obj.policy_document = try!(policyDocumentTypeParser::parse_xml("PolicyDocument", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreatePolicyVersionRequest contents to a SignedRequest
struct CreatePolicyVersionRequestWriter;
impl CreatePolicyVersionRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreatePolicyVersionRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.set_as_default {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "SetAsDefault"), obj);
		}
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDocument"), &obj.policy_document);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
#[derive(Debug, Default)]
pub struct DeactivateMFADeviceRequest {
	/// The name of the user whose MFA device you want to deactivate.
	pub user_name: existingUserNameType,
	/// The serial number that uniquely identifies the MFA device. For virtual MFA
	/// devices, the serial number is the device ARN.
	pub serial_number: serialNumberType,
}

/// Parse DeactivateMFADeviceRequest from XML
struct DeactivateMFADeviceRequestParser;
impl DeactivateMFADeviceRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeactivateMFADeviceRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeactivateMFADeviceRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "SerialNumber" {
				obj.serial_number = try!(serialNumberTypeParser::parse_xml("SerialNumber", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeactivateMFADeviceRequest contents to a SignedRequest
struct DeactivateMFADeviceRequestWriter;
impl DeactivateMFADeviceRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeactivateMFADeviceRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		serialNumberTypeWriter::write_params(params, &(prefix.to_string() + "SerialNumber"), &obj.serial_number);
	}
}
pub type stringType = String;
/// Parse stringType from XML
struct stringTypeParser;
impl stringTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<stringType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write stringType contents to a SignedRequest
struct stringTypeWriter;
impl stringTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &stringType) {
		params.put(name, obj);
	}
}
/// Contains information about a managed policy, including the policy's ARN,
/// versions, and the number of principal entities (users, groups, and roles) that
/// the policy is attached to.
/// This data type is used as a response element in the
/// GetAccountAuthorizationDetails action.
/// For more information about managed policies, see [Managed Policies and Inline
/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
/// inline.html) in the _Using IAM_ guide.
#[derive(Debug, Default)]
pub struct ManagedPolicyDetail {
	/// The friendly name (not ARN) identifying the policy.
	pub policy_name: policyNameType,
	/// A friendly description of the policy.
	pub description: policyDescriptionType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the policy was created.
	pub create_date: dateType,
	/// The number of principal entities (users, groups, and roles) that the policy is
	/// attached to.
	pub attachment_count: attachmentCountType,
	/// Specifies whether the policy can be attached to an IAM user, group, or role.
	pub is_attachable: booleanType,
	/// The stable and unique string identifying the policy.
	/// For more information about IDs, see [IAM Identifiers](http://docs.aws.amazon.c
	/// om/IAM/latest/UserGuide/Using_Identifiers.html) in the _Using IAM_ guide.
	pub policy_id: idType,
	/// The identifier for the version of the policy that is set as the default
	/// (operative) version.
	/// For more information about policy versions, see [Versioning for Managed
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-
	/// versions.html) in the _Using IAM_ guide.
	pub default_version_id: policyVersionIdType,
	/// A list containing information about the versions of the policy.
	pub policy_version_list: policyDocumentVersionListType,
	/// The path to the policy.
	/// For more information about paths, see [IAM Identifiers](http://docs.aws.amazon
	/// .com/IAM/latest/UserGuide/Using_Identifiers.html) in the _Using IAM_ guide.
	pub path: policyPathType,
	pub arn: arnType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the policy was last updated.
	/// When a policy has only one version, this field contains the date and time when
	/// the policy was created. When a policy has more than one version, this field
	/// contains the date and time when the most recent policy version was created.
	pub update_date: dateType,
}

/// Parse ManagedPolicyDetail from XML
struct ManagedPolicyDetailParser;
impl ManagedPolicyDetailParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ManagedPolicyDetail, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ManagedPolicyDetail::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "Description" {
				obj.description = try!(policyDescriptionTypeParser::parse_xml("Description", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "AttachmentCount" {
				obj.attachment_count = try!(attachmentCountTypeParser::parse_xml("AttachmentCount", stack));
				continue;
			}
			if current_name == "IsAttachable" {
				obj.is_attachable = try!(booleanTypeParser::parse_xml("IsAttachable", stack));
				continue;
			}
			if current_name == "PolicyId" {
				obj.policy_id = try!(idTypeParser::parse_xml("PolicyId", stack));
				continue;
			}
			if current_name == "DefaultVersionId" {
				obj.default_version_id = try!(policyVersionIdTypeParser::parse_xml("DefaultVersionId", stack));
				continue;
			}
			if current_name == "PolicyVersion" {
				obj.policy_version_list = try!(policyDocumentVersionListTypeParser::parse_xml("PolicyVersion", stack));
				continue;
			}
			if current_name == "Path" {
				obj.path = try!(policyPathTypeParser::parse_xml("Path", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			if current_name == "UpdateDate" {
				obj.update_date = try!(dateTypeParser::parse_xml("UpdateDate", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ManagedPolicyDetail contents to a SignedRequest
struct ManagedPolicyDetailWriter;
impl ManagedPolicyDetailWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ManagedPolicyDetail) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		policyDescriptionTypeWriter::write_params(params, &(prefix.to_string() + "Description"), &obj.description);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		attachmentCountTypeWriter::write_params(params, &(prefix.to_string() + "AttachmentCount"), &obj.attachment_count);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsAttachable"), &obj.is_attachable);
		idTypeWriter::write_params(params, &(prefix.to_string() + "PolicyId"), &obj.policy_id);
		policyVersionIdTypeWriter::write_params(params, &(prefix.to_string() + "DefaultVersionId"), &obj.default_version_id);
		policyDocumentVersionListTypeWriter::write_params(params, &(prefix.to_string() + "PolicyVersion"), &obj.policy_version_list);
		policyPathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), &obj.path);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "UpdateDate"), &obj.update_date);
	}
}
#[derive(Debug, Default)]
pub struct DeleteGroupRequest {
	/// The name of the group to delete.
	pub group_name: groupNameType,
}

/// Parse DeleteGroupRequest from XML
struct DeleteGroupRequestParser;
impl DeleteGroupRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteGroupRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteGroupRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteGroupRequest contents to a SignedRequest
struct DeleteGroupRequestWriter;
impl DeleteGroupRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteGroupRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
	}
}
/// Contains the response to a successful ListAccountAliases request.
#[derive(Debug, Default)]
pub struct ListAccountAliasesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A list of aliases associated with the account.
	pub account_aliases: accountAliasListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
}

/// Parse ListAccountAliasesResponse from XML
struct ListAccountAliasesResponseParser;
impl ListAccountAliasesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListAccountAliasesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListAccountAliasesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "accountAliasType" {
				obj.account_aliases = try!(accountAliasListTypeParser::parse_xml("accountAliasType", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListAccountAliasesResponse contents to a SignedRequest
struct ListAccountAliasesResponseWriter;
impl ListAccountAliasesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListAccountAliasesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		accountAliasListTypeWriter::write_params(params, &(prefix.to_string() + "accountAliasType"), &obj.account_aliases);
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
	}
}
/// Contains information about an IAM user, including all the user's policies and
/// all the IAM groups the user is in.
/// This data type is used as a response element in the
/// GetAccountAuthorizationDetails action.
#[derive(Debug, Default)]
pub struct UserDetail {
	/// The friendly name identifying the user.
	pub user_name: userNameType,
	/// A list of IAM groups that the user is in.
	pub group_list: groupNameListType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the user was created.
	pub create_date: dateType,
	/// The stable and unique string identifying the user. For more information about
	/// IDs, see [IAM Identifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Usi
	/// ng_Identifiers.html) in the _Using IAM_ guide.
	pub user_id: idType,
	/// A list of the inline policies embedded in the user.
	pub user_policy_list: policyDetailListType,
	/// The path to the user. For more information about paths, see [IAM Identifiers](
	/// http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the
	/// _Using IAM_ guide.
	pub path: pathType,
	/// A list of the managed policies attached to the user.
	pub attached_managed_policies: attachedPoliciesListType,
	pub arn: arnType,
}

/// Parse UserDetail from XML
struct UserDetailParser;
impl UserDetailParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UserDetail, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UserDetail::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "groupNameType" {
				obj.group_list = try!(groupNameListTypeParser::parse_xml("groupNameType", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "UserId" {
				obj.user_id = try!(idTypeParser::parse_xml("UserId", stack));
				continue;
			}
			if current_name == "PolicyDetail" {
				obj.user_policy_list = try!(policyDetailListTypeParser::parse_xml("PolicyDetail", stack));
				continue;
			}
			if current_name == "Path" {
				obj.path = try!(pathTypeParser::parse_xml("Path", stack));
				continue;
			}
			if current_name == "AttachedPolicy" {
				obj.attached_managed_policies = try!(attachedPoliciesListTypeParser::parse_xml("AttachedPolicy", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UserDetail contents to a SignedRequest
struct UserDetailWriter;
impl UserDetailWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UserDetail) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		groupNameListTypeWriter::write_params(params, &(prefix.to_string() + "groupNameType"), &obj.group_list);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		idTypeWriter::write_params(params, &(prefix.to_string() + "UserId"), &obj.user_id);
		policyDetailListTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDetail"), &obj.user_policy_list);
		pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), &obj.path);
		attachedPoliciesListTypeWriter::write_params(params, &(prefix.to_string() + "AttachedPolicy"), &obj.attached_managed_policies);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
	}
}
/// Contains a thumbprint for an identity provider's server certificate.
/// The identity provider's server certificate thumbprint is the hex-encoded SHA-1
/// hash value of the self-signed X.509 certificate used by the domain where the
/// OpenID Connect provider makes its keys available. It is always a 40-character
/// string.
pub type thumbprintType = String;
/// Parse thumbprintType from XML
struct thumbprintTypeParser;
impl thumbprintTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<thumbprintType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write thumbprintType contents to a SignedRequest
struct thumbprintTypeWriter;
impl thumbprintTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &thumbprintType) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful GetUser request.
#[derive(Debug, Default)]
pub struct GetUserResponse {
	/// Information about the user.
	pub user: User,
}

/// Parse GetUserResponse from XML
struct GetUserResponseParser;
impl GetUserResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetUserResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetUserResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "User" {
				obj.user = try!(UserParser::parse_xml("User", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetUserResponse contents to a SignedRequest
struct GetUserResponseWriter;
impl GetUserResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetUserResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		UserWriter::write_params(params, &(prefix.to_string() + "User"), &obj.user);
	}
}
pub type virtualMFADeviceListType = Vec<VirtualMFADevice>;
/// Parse virtualMFADeviceListType from XML
struct virtualMFADeviceListTypeParser;
impl virtualMFADeviceListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<virtualMFADeviceListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "VirtualMFADevice" {
			obj.push(try!(VirtualMFADeviceParser::parse_xml("VirtualMFADevice", stack)));
		}
		Ok(obj)
	}
}
/// Write virtualMFADeviceListType contents to a SignedRequest
struct virtualMFADeviceListTypeWriter;
impl virtualMFADeviceListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &virtualMFADeviceListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			VirtualMFADeviceWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type entityTemporarilyUnmodifiableMessage = String;
/// Parse entityTemporarilyUnmodifiableMessage from XML
struct entityTemporarilyUnmodifiableMessageParser;
impl entityTemporarilyUnmodifiableMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<entityTemporarilyUnmodifiableMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write entityTemporarilyUnmodifiableMessage contents to a SignedRequest
struct entityTemporarilyUnmodifiableMessageWriter;
impl entityTemporarilyUnmodifiableMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &entityTemporarilyUnmodifiableMessage) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful CreatePolicy request.
#[derive(Debug, Default)]
pub struct CreatePolicyResponse {
	/// Information about the policy.
	pub policy: Policy,
}

/// Parse CreatePolicyResponse from XML
struct CreatePolicyResponseParser;
impl CreatePolicyResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreatePolicyResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreatePolicyResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Policy" {
				obj.policy = try!(PolicyParser::parse_xml("Policy", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreatePolicyResponse contents to a SignedRequest
struct CreatePolicyResponseWriter;
impl CreatePolicyResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreatePolicyResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		PolicyWriter::write_params(params, &(prefix.to_string() + "Policy"), &obj.policy);
	}
}
pub type publicKeyFingerprintType = String;
/// Parse publicKeyFingerprintType from XML
struct publicKeyFingerprintTypeParser;
impl publicKeyFingerprintTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<publicKeyFingerprintType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write publicKeyFingerprintType contents to a SignedRequest
struct publicKeyFingerprintTypeWriter;
impl publicKeyFingerprintTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &publicKeyFingerprintType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct DetachRolePolicyRequest {
	/// The name (friendly name, not ARN) of the role to detach the policy from.
	pub role_name: roleNameType,
	pub policy_arn: arnType,
}

/// Parse DetachRolePolicyRequest from XML
struct DetachRolePolicyRequestParser;
impl DetachRolePolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DetachRolePolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DetachRolePolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DetachRolePolicyRequest contents to a SignedRequest
struct DetachRolePolicyRequestWriter;
impl DetachRolePolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DetachRolePolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
pub type duplicateCertificateMessage = String;
/// Parse duplicateCertificateMessage from XML
struct duplicateCertificateMessageParser;
impl duplicateCertificateMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<duplicateCertificateMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write duplicateCertificateMessage contents to a SignedRequest
struct duplicateCertificateMessageWriter;
impl duplicateCertificateMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &duplicateCertificateMessage) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct GetAccessKeyLastUsedRequest {
	/// The identifier of an access key.
	pub access_key_id: accessKeyIdType,
}

/// Parse GetAccessKeyLastUsedRequest from XML
struct GetAccessKeyLastUsedRequestParser;
impl GetAccessKeyLastUsedRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetAccessKeyLastUsedRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetAccessKeyLastUsedRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AccessKeyId" {
				obj.access_key_id = try!(accessKeyIdTypeParser::parse_xml("AccessKeyId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetAccessKeyLastUsedRequest contents to a SignedRequest
struct GetAccessKeyLastUsedRequestWriter;
impl GetAccessKeyLastUsedRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetAccessKeyLastUsedRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		accessKeyIdTypeWriter::write_params(params, &(prefix.to_string() + "AccessKeyId"), &obj.access_key_id);
	}
}
pub type ReportStateType = String;
/// Parse ReportStateType from XML
struct ReportStateTypeParser;
impl ReportStateTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ReportStateType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ReportStateType contents to a SignedRequest
struct ReportStateTypeWriter;
impl ReportStateTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ReportStateType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct UpdateSSHPublicKeyRequest {
	/// The name of the IAM user associated with the SSH public key.
	pub user_name: userNameType,
	/// The status to assign to the SSH public key. `Active` means the key can be used
	/// for authentication with an AWS CodeCommit repository. `Inactive` means the key
	/// cannot be used.
	pub status: statusType,
	/// The unique identifier for the SSH public key.
	pub ssh_public_key_id: publicKeyIdType,
}

/// Parse UpdateSSHPublicKeyRequest from XML
struct UpdateSSHPublicKeyRequestParser;
impl UpdateSSHPublicKeyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateSSHPublicKeyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateSSHPublicKeyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Status" {
				obj.status = try!(statusTypeParser::parse_xml("Status", stack));
				continue;
			}
			if current_name == "SSHPublicKeyId" {
				obj.ssh_public_key_id = try!(publicKeyIdTypeParser::parse_xml("SSHPublicKeyId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateSSHPublicKeyRequest contents to a SignedRequest
struct UpdateSSHPublicKeyRequestWriter;
impl UpdateSSHPublicKeyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateSSHPublicKeyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		statusTypeWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		publicKeyIdTypeWriter::write_params(params, &(prefix.to_string() + "SSHPublicKeyId"), &obj.ssh_public_key_id);
	}
}
/// Contains a list of policy names.
/// This data type is used as a response element in the ListPolicies action.
pub type policyNameListType = Vec<policyNameType>;
/// Parse policyNameListType from XML
struct policyNameListTypeParser;
impl policyNameListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<policyNameListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "policyNameType" {
			obj.push(try!(policyNameTypeParser::parse_xml("policyNameType", stack)));
		}
		Ok(obj)
	}
}
/// Write policyNameListType contents to a SignedRequest
struct policyNameListTypeWriter;
impl policyNameListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &policyNameListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			policyNameTypeWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type accountAliasListType = Vec<accountAliasType>;
/// Parse accountAliasListType from XML
struct accountAliasListTypeParser;
impl accountAliasListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<accountAliasListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "accountAliasType" {
			obj.push(try!(accountAliasTypeParser::parse_xml("accountAliasType", stack)));
		}
		Ok(obj)
	}
}
/// Write accountAliasListType contents to a SignedRequest
struct accountAliasListTypeWriter;
impl accountAliasListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &accountAliasListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			accountAliasTypeWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type userNameType = String;
/// Parse userNameType from XML
struct userNameTypeParser;
impl userNameTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<userNameType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write userNameType contents to a SignedRequest
struct userNameTypeWriter;
impl userNameTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &userNameType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct DeletePolicyRequest {
	pub policy_arn: arnType,
}

/// Parse DeletePolicyRequest from XML
struct DeletePolicyRequestParser;
impl DeletePolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeletePolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeletePolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeletePolicyRequest contents to a SignedRequest
struct DeletePolicyRequestWriter;
impl DeletePolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeletePolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
/// Contains information about an IAM policy, including the policy document.
/// This data type is used as a response element in the
/// GetAccountAuthorizationDetails action.
#[derive(Debug, Default)]
pub struct PolicyDetail {
	/// The name of the policy.
	pub policy_name: policyNameType,
	/// The policy document.
	pub policy_document: policyDocumentType,
}

/// Parse PolicyDetail from XML
struct PolicyDetailParser;
impl PolicyDetailParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyDetail, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PolicyDetail::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "PolicyDocument" {
				obj.policy_document = try!(policyDocumentTypeParser::parse_xml("PolicyDocument", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PolicyDetail contents to a SignedRequest
struct PolicyDetailWriter;
impl PolicyDetailWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyDetail) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDocument"), &obj.policy_document);
	}
}
/// Contains the response to a successful GetAccountPasswordPolicy request.
#[derive(Debug, Default)]
pub struct GetAccountPasswordPolicyResponse {
	pub password_policy: PasswordPolicy,
}

/// Parse GetAccountPasswordPolicyResponse from XML
struct GetAccountPasswordPolicyResponseParser;
impl GetAccountPasswordPolicyResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetAccountPasswordPolicyResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetAccountPasswordPolicyResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PasswordPolicy" {
				obj.password_policy = try!(PasswordPolicyParser::parse_xml("PasswordPolicy", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetAccountPasswordPolicyResponse contents to a SignedRequest
struct GetAccountPasswordPolicyResponseWriter;
impl GetAccountPasswordPolicyResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetAccountPasswordPolicyResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		PasswordPolicyWriter::write_params(params, &(prefix.to_string() + "PasswordPolicy"), &obj.password_policy);
	}
}
#[derive(Debug, Default)]
pub struct DeleteUserPolicyRequest {
	/// The name (friendly name, not ARN) identifying the user that the policy is
	/// embedded in.
	pub user_name: existingUserNameType,
	/// The name identifying the policy document to delete.
	pub policy_name: policyNameType,
}

/// Parse DeleteUserPolicyRequest from XML
struct DeleteUserPolicyRequestParser;
impl DeleteUserPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteUserPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteUserPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteUserPolicyRequest contents to a SignedRequest
struct DeleteUserPolicyRequestWriter;
impl DeleteUserPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteUserPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
	}
}
#[derive(Debug, Default)]
pub struct RemoveRoleFromInstanceProfileRequest {
	/// The name of the role to remove.
	pub role_name: roleNameType,
	/// The name of the instance profile to update.
	pub instance_profile_name: instanceProfileNameType,
}

/// Parse RemoveRoleFromInstanceProfileRequest from XML
struct RemoveRoleFromInstanceProfileRequestParser;
impl RemoveRoleFromInstanceProfileRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<RemoveRoleFromInstanceProfileRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = RemoveRoleFromInstanceProfileRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "InstanceProfileName" {
				obj.instance_profile_name = try!(instanceProfileNameTypeParser::parse_xml("InstanceProfileName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write RemoveRoleFromInstanceProfileRequest contents to a SignedRequest
struct RemoveRoleFromInstanceProfileRequestWriter;
impl RemoveRoleFromInstanceProfileRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &RemoveRoleFromInstanceProfileRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		instanceProfileNameTypeWriter::write_params(params, &(prefix.to_string() + "InstanceProfileName"), &obj.instance_profile_name);
	}
}
/// Contains the response to a successful GetServerCertificate request.
#[derive(Debug, Default)]
pub struct GetServerCertificateResponse {
	/// Information about the server certificate.
	pub server_certificate: ServerCertificate,
}

/// Parse GetServerCertificateResponse from XML
struct GetServerCertificateResponseParser;
impl GetServerCertificateResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetServerCertificateResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetServerCertificateResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ServerCertificate" {
				obj.server_certificate = try!(ServerCertificateParser::parse_xml("ServerCertificate", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetServerCertificateResponse contents to a SignedRequest
struct GetServerCertificateResponseWriter;
impl GetServerCertificateResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetServerCertificateResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ServerCertificateWriter::write_params(params, &(prefix.to_string() + "ServerCertificate"), &obj.server_certificate);
	}
}
#[derive(Debug, Default)]
pub struct GetOpenIDConnectProviderRequest {
	/// The Amazon Resource Name (ARN) of the IAM OpenID Connect (OIDC) provider to
	/// get information for. You can get a list of OIDC provider ARNs by using the
	/// ListOpenIDConnectProviders action.
	pub open_id_connect_provider_arn: arnType,
}

/// Parse GetOpenIDConnectProviderRequest from XML
struct GetOpenIDConnectProviderRequestParser;
impl GetOpenIDConnectProviderRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetOpenIDConnectProviderRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetOpenIDConnectProviderRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "OpenIDConnectProviderArn" {
				obj.open_id_connect_provider_arn = try!(arnTypeParser::parse_xml("OpenIDConnectProviderArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetOpenIDConnectProviderRequest contents to a SignedRequest
struct GetOpenIDConnectProviderRequestWriter;
impl GetOpenIDConnectProviderRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetOpenIDConnectProviderRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "OpenIDConnectProviderArn"), &obj.open_id_connect_provider_arn);
	}
}
#[derive(Debug, Default)]
pub struct UpdateAssumeRolePolicyRequest {
	/// The name of the role to update.
	pub role_name: roleNameType,
	/// The policy that grants an entity permission to assume the role.
	pub policy_document: policyDocumentType,
}

/// Parse UpdateAssumeRolePolicyRequest from XML
struct UpdateAssumeRolePolicyRequestParser;
impl UpdateAssumeRolePolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateAssumeRolePolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateAssumeRolePolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "PolicyDocument" {
				obj.policy_document = try!(policyDocumentTypeParser::parse_xml("PolicyDocument", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateAssumeRolePolicyRequest contents to a SignedRequest
struct UpdateAssumeRolePolicyRequestWriter;
impl UpdateAssumeRolePolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateAssumeRolePolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDocument"), &obj.policy_document);
	}
}
/// Contains the response to a successful UploadSSHPublicKey request.
#[derive(Debug, Default)]
pub struct UploadSSHPublicKeyResponse {
	/// Contains information about the SSH public key.
	pub ssh_public_key: SSHPublicKey,
}

/// Parse UploadSSHPublicKeyResponse from XML
struct UploadSSHPublicKeyResponseParser;
impl UploadSSHPublicKeyResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UploadSSHPublicKeyResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UploadSSHPublicKeyResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SSHPublicKey" {
				obj.ssh_public_key = try!(SSHPublicKeyParser::parse_xml("SSHPublicKey", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UploadSSHPublicKeyResponse contents to a SignedRequest
struct UploadSSHPublicKeyResponseWriter;
impl UploadSSHPublicKeyResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UploadSSHPublicKeyResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		SSHPublicKeyWriter::write_params(params, &(prefix.to_string() + "SSHPublicKey"), &obj.ssh_public_key);
	}
}
#[derive(Debug, Default)]
pub struct DeleteRoleRequest {
	/// The name of the role to delete.
	pub role_name: roleNameType,
}

/// Parse DeleteRoleRequest from XML
struct DeleteRoleRequestParser;
impl DeleteRoleRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteRoleRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteRoleRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteRoleRequest contents to a SignedRequest
struct DeleteRoleRequestWriter;
impl DeleteRoleRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteRoleRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
	}
}
pub type invalidInputMessage = String;
/// Parse invalidInputMessage from XML
struct invalidInputMessageParser;
impl invalidInputMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<invalidInputMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write invalidInputMessage contents to a SignedRequest
struct invalidInputMessageWriter;
impl invalidInputMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &invalidInputMessage) {
		params.put(name, obj);
	}
}
pub type serverCertificateMetadataListType = Vec<ServerCertificateMetadata>;
/// Parse serverCertificateMetadataListType from XML
struct serverCertificateMetadataListTypeParser;
impl serverCertificateMetadataListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<serverCertificateMetadataListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ServerCertificateMetadata" {
			obj.push(try!(ServerCertificateMetadataParser::parse_xml("ServerCertificateMetadata", stack)));
		}
		Ok(obj)
	}
}
/// Write serverCertificateMetadataListType contents to a SignedRequest
struct serverCertificateMetadataListTypeWriter;
impl serverCertificateMetadataListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &serverCertificateMetadataListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ServerCertificateMetadataWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct GetUserPolicyRequest {
	/// The name of the user who the policy is associated with.
	pub user_name: existingUserNameType,
	/// The name of the policy document to get.
	pub policy_name: policyNameType,
}

/// Parse GetUserPolicyRequest from XML
struct GetUserPolicyRequestParser;
impl GetUserPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetUserPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetUserPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetUserPolicyRequest contents to a SignedRequest
struct GetUserPolicyRequestWriter;
impl GetUserPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetUserPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
	}
}
pub type clientIDListType = Vec<clientIDType>;
/// Parse clientIDListType from XML
struct clientIDListTypeParser;
impl clientIDListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<clientIDListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "clientIDType" {
			obj.push(try!(clientIDTypeParser::parse_xml("clientIDType", stack)));
		}
		Ok(obj)
	}
}
/// Write clientIDListType contents to a SignedRequest
struct clientIDListTypeWriter;
impl clientIDListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &clientIDListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			clientIDTypeWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct GetPolicyVersionRequest {
	/// Identifies the policy version to retrieve.
	pub version_id: policyVersionIdType,
	pub policy_arn: arnType,
}

/// Parse GetPolicyVersionRequest from XML
struct GetPolicyVersionRequestParser;
impl GetPolicyVersionRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetPolicyVersionRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetPolicyVersionRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "VersionId" {
				obj.version_id = try!(policyVersionIdTypeParser::parse_xml("VersionId", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetPolicyVersionRequest contents to a SignedRequest
struct GetPolicyVersionRequestWriter;
impl GetPolicyVersionRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetPolicyVersionRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		policyVersionIdTypeWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
/// Contains the response to a successful CreateSAMLProvider request.
#[derive(Debug, Default)]
pub struct CreateSAMLProviderResponse {
	/// The Amazon Resource Name (ARN) of the SAML provider.
	pub saml_provider_arn: arnType,
}

/// Parse CreateSAMLProviderResponse from XML
struct CreateSAMLProviderResponseParser;
impl CreateSAMLProviderResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateSAMLProviderResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateSAMLProviderResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SAMLProviderArn" {
				obj.saml_provider_arn = try!(arnTypeParser::parse_xml("SAMLProviderArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateSAMLProviderResponse contents to a SignedRequest
struct CreateSAMLProviderResponseWriter;
impl CreateSAMLProviderResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateSAMLProviderResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "SAMLProviderArn"), &obj.saml_provider_arn);
	}
}
/// Contains the response to a successful ListOpenIDConnectProviders request.
#[derive(Debug, Default)]
pub struct ListOpenIDConnectProvidersResponse {
	/// The list of IAM OpenID Connect providers in the AWS account.
	pub open_id_connect_provider_list: OpenIDConnectProviderListType,
}

/// Parse ListOpenIDConnectProvidersResponse from XML
struct ListOpenIDConnectProvidersResponseParser;
impl ListOpenIDConnectProvidersResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListOpenIDConnectProvidersResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListOpenIDConnectProvidersResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "OpenIDConnectProviderListEntry" {
				obj.open_id_connect_provider_list = try!(OpenIDConnectProviderListTypeParser::parse_xml("OpenIDConnectProviderListEntry", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListOpenIDConnectProvidersResponse contents to a SignedRequest
struct ListOpenIDConnectProvidersResponseWriter;
impl ListOpenIDConnectProvidersResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListOpenIDConnectProvidersResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		OpenIDConnectProviderListTypeWriter::write_params(params, &(prefix.to_string() + "OpenIDConnectProviderListEntry"), &obj.open_id_connect_provider_list);
	}
}
#[derive(Debug, Default)]
pub struct AddClientIDToOpenIDConnectProviderRequest {
	/// The Amazon Resource Name (ARN) of the IAM OpenID Connect (OIDC) provider to
	/// add the client ID to. You can get a list of OIDC provider ARNs by using the
	/// ListOpenIDConnectProviders action.
	pub open_id_connect_provider_arn: arnType,
	/// The client ID (also known as audience) to add to the IAM OpenID Connect
	/// provider.
	pub client_id: clientIDType,
}

/// Parse AddClientIDToOpenIDConnectProviderRequest from XML
struct AddClientIDToOpenIDConnectProviderRequestParser;
impl AddClientIDToOpenIDConnectProviderRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AddClientIDToOpenIDConnectProviderRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AddClientIDToOpenIDConnectProviderRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "OpenIDConnectProviderArn" {
				obj.open_id_connect_provider_arn = try!(arnTypeParser::parse_xml("OpenIDConnectProviderArn", stack));
				continue;
			}
			if current_name == "ClientID" {
				obj.client_id = try!(clientIDTypeParser::parse_xml("ClientID", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AddClientIDToOpenIDConnectProviderRequest contents to a SignedRequest
struct AddClientIDToOpenIDConnectProviderRequestWriter;
impl AddClientIDToOpenIDConnectProviderRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AddClientIDToOpenIDConnectProviderRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "OpenIDConnectProviderArn"), &obj.open_id_connect_provider_arn);
		clientIDTypeWriter::write_params(params, &(prefix.to_string() + "ClientID"), &obj.client_id);
	}
}
/// Contains the response to a successful GetRolePolicy request.
#[derive(Debug, Default)]
pub struct GetRolePolicyResponse {
	/// The role the policy is associated with.
	pub role_name: roleNameType,
	/// The policy document.
	pub policy_document: policyDocumentType,
	/// The name of the policy.
	pub policy_name: policyNameType,
}

/// Parse GetRolePolicyResponse from XML
struct GetRolePolicyResponseParser;
impl GetRolePolicyResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetRolePolicyResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetRolePolicyResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "PolicyDocument" {
				obj.policy_document = try!(policyDocumentTypeParser::parse_xml("PolicyDocument", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetRolePolicyResponse contents to a SignedRequest
struct GetRolePolicyResponseWriter;
impl GetRolePolicyResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetRolePolicyResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDocument"), &obj.policy_document);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
	}
}
pub type unrecognizedPublicKeyEncodingMessage = String;
/// Parse unrecognizedPublicKeyEncodingMessage from XML
struct unrecognizedPublicKeyEncodingMessageParser;
impl unrecognizedPublicKeyEncodingMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<unrecognizedPublicKeyEncodingMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write unrecognizedPublicKeyEncodingMessage contents to a SignedRequest
struct unrecognizedPublicKeyEncodingMessageWriter;
impl unrecognizedPublicKeyEncodingMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &unrecognizedPublicKeyEncodingMessage) {
		params.put(name, obj);
	}
}
/// The request was rejected because the public key encoding format is unsupported
/// or unrecognized.
#[derive(Debug, Default)]
pub struct UnrecognizedPublicKeyEncodingException {
	pub message: unrecognizedPublicKeyEncodingMessage,
}

/// Parse UnrecognizedPublicKeyEncodingException from XML
struct UnrecognizedPublicKeyEncodingExceptionParser;
impl UnrecognizedPublicKeyEncodingExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UnrecognizedPublicKeyEncodingException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UnrecognizedPublicKeyEncodingException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(unrecognizedPublicKeyEncodingMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UnrecognizedPublicKeyEncodingException contents to a SignedRequest
struct UnrecognizedPublicKeyEncodingExceptionWriter;
impl UnrecognizedPublicKeyEncodingExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UnrecognizedPublicKeyEncodingException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		unrecognizedPublicKeyEncodingMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
#[derive(Debug, Default)]
pub struct RemoveUserFromGroupRequest {
	/// The name of the user to remove.
	pub user_name: existingUserNameType,
	/// The name of the group to update.
	pub group_name: groupNameType,
}

/// Parse RemoveUserFromGroupRequest from XML
struct RemoveUserFromGroupRequestParser;
impl RemoveUserFromGroupRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<RemoveUserFromGroupRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = RemoveUserFromGroupRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write RemoveUserFromGroupRequest contents to a SignedRequest
struct RemoveUserFromGroupRequestWriter;
impl RemoveUserFromGroupRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &RemoveUserFromGroupRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
	}
}
/// The request was rejected because the public key certificate and the private
/// key do not match.
#[derive(Debug, Default)]
pub struct KeyPairMismatchException {
	pub message: keyPairMismatchMessage,
}

/// Parse KeyPairMismatchException from XML
struct KeyPairMismatchExceptionParser;
impl KeyPairMismatchExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<KeyPairMismatchException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = KeyPairMismatchException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(keyPairMismatchMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write KeyPairMismatchException contents to a SignedRequest
struct KeyPairMismatchExceptionWriter;
impl KeyPairMismatchExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &KeyPairMismatchException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		keyPairMismatchMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
#[derive(Debug, Default)]
pub struct GetRolePolicyRequest {
	/// The name of the role associated with the policy.
	pub role_name: roleNameType,
	/// The name of the policy document to get.
	pub policy_name: policyNameType,
}

/// Parse GetRolePolicyRequest from XML
struct GetRolePolicyRequestParser;
impl GetRolePolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetRolePolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetRolePolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetRolePolicyRequest contents to a SignedRequest
struct GetRolePolicyRequestWriter;
impl GetRolePolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetRolePolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
	}
}
pub type PolicyGroupListType = Vec<PolicyGroup>;
/// Parse PolicyGroupListType from XML
struct PolicyGroupListTypeParser;
impl PolicyGroupListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyGroupListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "PolicyGroup" {
			obj.push(try!(PolicyGroupParser::parse_xml("PolicyGroup", stack)));
		}
		Ok(obj)
	}
}
/// Write PolicyGroupListType contents to a SignedRequest
struct PolicyGroupListTypeWriter;
impl PolicyGroupListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyGroupListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			PolicyGroupWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type instanceProfileNameType = String;
/// Parse instanceProfileNameType from XML
struct instanceProfileNameTypeParser;
impl instanceProfileNameTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<instanceProfileNameType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write instanceProfileNameType contents to a SignedRequest
struct instanceProfileNameTypeWriter;
impl instanceProfileNameTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &instanceProfileNameType) {
		params.put(name, obj);
	}
}
pub type credentialReportExpiredExceptionMessage = String;
/// Parse credentialReportExpiredExceptionMessage from XML
struct credentialReportExpiredExceptionMessageParser;
impl credentialReportExpiredExceptionMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<credentialReportExpiredExceptionMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write credentialReportExpiredExceptionMessage contents to a SignedRequest
struct credentialReportExpiredExceptionMessageWriter;
impl credentialReportExpiredExceptionMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &credentialReportExpiredExceptionMessage) {
		params.put(name, obj);
	}
}
/// Contains a list of IAM roles.
/// This data type is used as a response element in the ListRoles action.
pub type roleListType = Vec<Role>;
/// Parse roleListType from XML
struct roleListTypeParser;
impl roleListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<roleListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Role" {
			obj.push(try!(RoleParser::parse_xml("Role", stack)));
		}
		Ok(obj)
	}
}
/// Write roleListType contents to a SignedRequest
struct roleListTypeWriter;
impl roleListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &roleListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			RoleWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type groupNameType = String;
/// Parse groupNameType from XML
struct groupNameTypeParser;
impl groupNameTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<groupNameType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write groupNameType contents to a SignedRequest
struct groupNameTypeWriter;
impl groupNameTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &groupNameType) {
		params.put(name, obj);
	}
}
/// The request was rejected because the credential report does not exist. To
/// generate a credential report, use GenerateCredentialReport.
#[derive(Debug, Default)]
pub struct CredentialReportNotPresentException {
	pub message: credentialReportNotPresentExceptionMessage,
}

/// Parse CredentialReportNotPresentException from XML
struct CredentialReportNotPresentExceptionParser;
impl CredentialReportNotPresentExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CredentialReportNotPresentException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CredentialReportNotPresentException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(credentialReportNotPresentExceptionMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CredentialReportNotPresentException contents to a SignedRequest
struct CredentialReportNotPresentExceptionWriter;
impl CredentialReportNotPresentExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CredentialReportNotPresentException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		credentialReportNotPresentExceptionMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains the list of SAML providers for this account.
#[derive(Debug, Default)]
pub struct SAMLProviderListEntry {
	/// The date and time when the SAML provider was created.
	pub create_date: dateType,
	/// The expiration date and time for the SAML provider.
	pub valid_until: dateType,
	/// The Amazon Resource Name (ARN) of the SAML provider.
	pub arn: arnType,
}

/// Parse SAMLProviderListEntry from XML
struct SAMLProviderListEntryParser;
impl SAMLProviderListEntryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SAMLProviderListEntry, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SAMLProviderListEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "ValidUntil" {
				obj.valid_until = try!(dateTypeParser::parse_xml("ValidUntil", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SAMLProviderListEntry contents to a SignedRequest
struct SAMLProviderListEntryWriter;
impl SAMLProviderListEntryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SAMLProviderListEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "ValidUntil"), &obj.valid_until);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
	}
}
pub type entityAlreadyExistsMessage = String;
/// Parse entityAlreadyExistsMessage from XML
struct entityAlreadyExistsMessageParser;
impl entityAlreadyExistsMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<entityAlreadyExistsMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write entityAlreadyExistsMessage contents to a SignedRequest
struct entityAlreadyExistsMessageWriter;
impl entityAlreadyExistsMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &entityAlreadyExistsMessage) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct ListAttachedUserPoliciesRequest {
	/// The name (friendly name, not ARN) of the user to list attached policies for.
	pub user_name: userNameType,
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// The path prefix for filtering the results. This parameter is optional. If it
	/// is not included, it defaults to a slash (/), listing all policies.
	pub path_prefix: Option<policyPathType>,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
}

/// Parse ListAttachedUserPoliciesRequest from XML
struct ListAttachedUserPoliciesRequestParser;
impl ListAttachedUserPoliciesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListAttachedUserPoliciesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListAttachedUserPoliciesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "PathPrefix" {
				obj.path_prefix = Some(try!(policyPathTypeParser::parse_xml("PathPrefix", stack)));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListAttachedUserPoliciesRequest contents to a SignedRequest
struct ListAttachedUserPoliciesRequestWriter;
impl ListAttachedUserPoliciesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListAttachedUserPoliciesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.path_prefix {
			policyPathTypeWriter::write_params(params, &(prefix.to_string() + "PathPrefix"), obj);
		}
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
	}
}
pub type markerType = String;
/// Parse markerType from XML
struct markerTypeParser;
impl markerTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<markerType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write markerType contents to a SignedRequest
struct markerTypeWriter;
impl markerTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &markerType) {
		params.put(name, obj);
	}
}
/// Contains information about an IAM user entity.
/// This data type is used as a response element in the following actions:
///   * CreateUser
///   * GetUser
///   * ListUsers
#[derive(Debug, Default)]
pub struct User {
	/// The friendly name identifying the user.
	pub user_name: userNameType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the user's password was last
	/// used to sign in to an AWS website. For a list of AWS websites that capture a
	/// user's last sign-in time, see the [Credential
	/// Reports](http://docs.aws.amazon.com/IAM/latest/UserGuide/credential-
	/// reports.html) topic in the _Using IAM_ guide. If a password is used more than
	/// once in a five-minute span, only the first use is returned in this field. This
	/// field is null (not present) when:
	///   * The user does not have a password
	///   * The password exists but has never been used (at least not since IAM started tracking this information on October 20th, 2014
	///   * there is no sign-in data associated with the user
	/// This value is returned only in the GetUser and ListUsers actions.
	pub password_last_used: Option<dateType>,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the user was created.
	pub create_date: dateType,
	/// The stable and unique string identifying the user. For more information about
	/// IDs, see [IAM Identifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Usi
	/// ng_Identifiers.html) in the _Using IAM_ guide.
	pub user_id: idType,
	/// The path to the user. For more information about paths, see [IAM Identifiers](
	/// http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the
	/// _Using IAM_ guide.
	pub path: pathType,
	/// The Amazon Resource Name (ARN) that identifies the user. For more information
	/// about ARNs and how to use ARNs in policies, see [IAM Identifiers](http://docs.
	/// aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the _Using IAM_
	/// guide.
	pub arn: arnType,
}

/// Parse User from XML
struct UserParser;
impl UserParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<User, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = User::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "PasswordLastUsed" {
				obj.password_last_used = Some(try!(dateTypeParser::parse_xml("PasswordLastUsed", stack)));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "UserId" {
				obj.user_id = try!(idTypeParser::parse_xml("UserId", stack));
				continue;
			}
			if current_name == "Path" {
				obj.path = try!(pathTypeParser::parse_xml("Path", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write User contents to a SignedRequest
struct UserWriter;
impl UserWriter {
	fn write_params(params: &mut Params, name: &str, obj: &User) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		if let Some(ref obj) = obj.password_last_used {
			dateTypeWriter::write_params(params, &(prefix.to_string() + "PasswordLastUsed"), obj);
		}
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		idTypeWriter::write_params(params, &(prefix.to_string() + "UserId"), &obj.user_id);
		pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), &obj.path);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
	}
}
#[derive(Debug, Default)]
pub struct ListServerCertificatesRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// The path prefix for filtering the results. For example: `/company/servercerts`
	/// would get all server certificates for which the path starts with
	/// `/company/servercerts`.
	/// This parameter is optional. If it is not included, it defaults to a slash (/),
	/// listing all server certificates.
	pub path_prefix: pathPrefixType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListServerCertificatesRequest from XML
struct ListServerCertificatesRequestParser;
impl ListServerCertificatesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListServerCertificatesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListServerCertificatesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "PathPrefix" {
				obj.path_prefix = try!(pathPrefixTypeParser::parse_xml("PathPrefix", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListServerCertificatesRequest contents to a SignedRequest
struct ListServerCertificatesRequestWriter;
impl ListServerCertificatesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListServerCertificatesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		pathPrefixTypeWriter::write_params(params, &(prefix.to_string() + "PathPrefix"), &obj.path_prefix);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
pub type clientIDType = String;
/// Parse clientIDType from XML
struct clientIDTypeParser;
impl clientIDTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<clientIDType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write clientIDType contents to a SignedRequest
struct clientIDTypeWriter;
impl clientIDTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &clientIDType) {
		params.put(name, obj);
	}
}
pub type accountAliasType = String;
/// Parse accountAliasType from XML
struct accountAliasTypeParser;
impl accountAliasTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<accountAliasType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write accountAliasType contents to a SignedRequest
struct accountAliasTypeWriter;
impl accountAliasTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &accountAliasType) {
		params.put(name, obj);
	}
}
/// Contains information about an attached policy.
/// An attached policy is a managed policy that has been attached to a user,
/// group, or role. This data type is used as a response element in the
/// ListAttachedGroupPolicies, ListAttachedRolePolicies, ListAttachedUserPolicies,
/// and GetAccountAuthorizationDetails actions.
/// For more information about managed policies, refer to [Managed Policies and
/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
/// managed-vs-inline.html) in the _Using IAM_ guide.
#[derive(Debug, Default)]
pub struct AttachedPolicy {
	/// The friendly name of the attached policy.
	pub policy_name: policyNameType,
	pub policy_arn: arnType,
}

/// Parse AttachedPolicy from XML
struct AttachedPolicyParser;
impl AttachedPolicyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AttachedPolicy, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AttachedPolicy::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AttachedPolicy contents to a SignedRequest
struct AttachedPolicyWriter;
impl AttachedPolicyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AttachedPolicy) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
pub type virtualMFADeviceName = String;
/// Parse virtualMFADeviceName from XML
struct virtualMFADeviceNameParser;
impl virtualMFADeviceNameParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<virtualMFADeviceName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write virtualMFADeviceName contents to a SignedRequest
struct virtualMFADeviceNameWriter;
impl virtualMFADeviceNameWriter {
	fn write_params(params: &mut Params, name: &str, obj: &virtualMFADeviceName) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct UpdateAccessKeyRequest {
	/// The name of the user whose key you want to update.
	pub user_name: Option<existingUserNameType>,
	/// The status you want to assign to the secret access key. `Active` means the key
	/// can be used for API calls to AWS, while `Inactive` means the key cannot be
	/// used.
	pub status: statusType,
	/// The access key ID of the secret access key you want to update.
	pub access_key_id: accessKeyIdType,
}

/// Parse UpdateAccessKeyRequest from XML
struct UpdateAccessKeyRequestParser;
impl UpdateAccessKeyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateAccessKeyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateAccessKeyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = Some(try!(existingUserNameTypeParser::parse_xml("UserName", stack)));
				continue;
			}
			if current_name == "Status" {
				obj.status = try!(statusTypeParser::parse_xml("Status", stack));
				continue;
			}
			if current_name == "AccessKeyId" {
				obj.access_key_id = try!(accessKeyIdTypeParser::parse_xml("AccessKeyId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateAccessKeyRequest contents to a SignedRequest
struct UpdateAccessKeyRequestWriter;
impl UpdateAccessKeyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateAccessKeyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.user_name {
			existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), obj);
		}
		statusTypeWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		accessKeyIdTypeWriter::write_params(params, &(prefix.to_string() + "AccessKeyId"), &obj.access_key_id);
	}
}
#[derive(Debug, Default)]
pub struct UpdateUserRequest {
	/// Name of the user to update. If you're changing the name of the user, this is
	/// the original user name.
	pub user_name: existingUserNameType,
	/// New path for the user. Include this parameter only if you're changing the
	/// user's path.
	pub new_path: Option<pathType>,
	/// New name for the user. Include this parameter only if you're changing the
	/// user's name.
	pub new_user_name: Option<userNameType>,
}

/// Parse UpdateUserRequest from XML
struct UpdateUserRequestParser;
impl UpdateUserRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateUserRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateUserRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "NewPath" {
				obj.new_path = Some(try!(pathTypeParser::parse_xml("NewPath", stack)));
				continue;
			}
			if current_name == "NewUserName" {
				obj.new_user_name = Some(try!(userNameTypeParser::parse_xml("NewUserName", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateUserRequest contents to a SignedRequest
struct UpdateUserRequestWriter;
impl UpdateUserRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateUserRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		if let Some(ref obj) = obj.new_path {
			pathTypeWriter::write_params(params, &(prefix.to_string() + "NewPath"), obj);
		}
		if let Some(ref obj) = obj.new_user_name {
			userNameTypeWriter::write_params(params, &(prefix.to_string() + "NewUserName"), obj);
		}
	}
}
/// The request was rejected because it attempted to delete a resource that has
/// attached subordinate entities. The error message describes these entities.
#[derive(Debug, Default)]
pub struct DeleteConflictException {
	pub message: deleteConflictMessage,
}

/// Parse DeleteConflictException from XML
struct DeleteConflictExceptionParser;
impl DeleteConflictExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteConflictException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteConflictException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(deleteConflictMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteConflictException contents to a SignedRequest
struct DeleteConflictExceptionWriter;
impl DeleteConflictExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteConflictException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		deleteConflictMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains information about an IAM group entity.
/// This data type is used as a response element in the following actions:
///   * CreateGroup
///   * GetGroup
///   * ListGroups
#[derive(Debug, Default)]
pub struct Group {
	/// The path to the group. For more information about paths, see [IAM Identifiers]
	/// (http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in
	/// the _Using IAM_ guide.
	pub path: pathType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the group was created.
	pub create_date: dateType,
	/// The stable and unique string identifying the group. For more information about
	/// IDs, see [IAM Identifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Usi
	/// ng_Identifiers.html) in the _Using IAM_ guide.
	pub group_id: idType,
	/// The Amazon Resource Name (ARN) specifying the group. For more information
	/// about ARNs and how to use them in policies, see [IAM Identifiers](http://docs.
	/// aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the _Using IAM_
	/// guide.
	pub arn: arnType,
	/// The friendly name that identifies the group.
	pub group_name: groupNameType,
}

/// Parse Group from XML
struct GroupParser;
impl GroupParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Group, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Group::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Path" {
				obj.path = try!(pathTypeParser::parse_xml("Path", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "GroupId" {
				obj.group_id = try!(idTypeParser::parse_xml("GroupId", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Group contents to a SignedRequest
struct GroupWriter;
impl GroupWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Group) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), &obj.path);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		idTypeWriter::write_params(params, &(prefix.to_string() + "GroupId"), &obj.group_id);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
	}
}
#[derive(Debug, Default)]
pub struct CreateSAMLProviderRequest {
	/// An XML document generated by an identity provider (IdP) that supports SAML
	/// 2.0. The document includes the issuer's name, expiration information, and keys
	/// that can be used to validate the SAML authentication response (assertions)
	/// that are received from the IdP. You must generate the metadata document using
	/// the identity management software that is used as your organization's IdP.
	/// For more information, see [Creating Temporary Security Credentials for SAML
	/// Federation](http://docs.aws.amazon.com/STS/latest/UsingSTS/CreatingSAML.html)
	/// in the _Using Temporary Security Credentials_ guide.
	pub saml_metadata_document: SAMLMetadataDocumentType,
	/// The name of the provider to create.
	pub name: SAMLProviderNameType,
}

/// Parse CreateSAMLProviderRequest from XML
struct CreateSAMLProviderRequestParser;
impl CreateSAMLProviderRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateSAMLProviderRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateSAMLProviderRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SAMLMetadataDocument" {
				obj.saml_metadata_document = try!(SAMLMetadataDocumentTypeParser::parse_xml("SAMLMetadataDocument", stack));
				continue;
			}
			if current_name == "Name" {
				obj.name = try!(SAMLProviderNameTypeParser::parse_xml("Name", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateSAMLProviderRequest contents to a SignedRequest
struct CreateSAMLProviderRequestWriter;
impl CreateSAMLProviderRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateSAMLProviderRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		SAMLMetadataDocumentTypeWriter::write_params(params, &(prefix.to_string() + "SAMLMetadataDocument"), &obj.saml_metadata_document);
		SAMLProviderNameTypeWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
	}
}
pub type statusType = String;
/// Parse statusType from XML
struct statusTypeParser;
impl statusTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<statusType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write statusType contents to a SignedRequest
struct statusTypeWriter;
impl statusTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &statusType) {
		params.put(name, obj);
	}
}
/// Contains information about an SSH public key.
/// This data type is used as a response element in the GetSSHPublicKey and
/// UploadSSHPublicKey actions.
#[derive(Debug, Default)]
pub struct SSHPublicKey {
	/// The name of the IAM user associated with the SSH public key.
	pub user_name: userNameType,
	/// The status of the SSH public key. `Active` means the key can be used for
	/// authentication with an AWS CodeCommit repository. `Inactive` means the key
	/// cannot be used.
	pub status: statusType,
	/// The SSH public key.
	pub ssh_public_key_body: publicKeyMaterialType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the SSH public key was uploaded.
	pub upload_date: Option<dateType>,
	/// The MD5 message digest of the SSH public key.
	pub fingerprint: publicKeyFingerprintType,
	/// The unique identifier for the SSH public key.
	pub ssh_public_key_id: publicKeyIdType,
}

/// Parse SSHPublicKey from XML
struct SSHPublicKeyParser;
impl SSHPublicKeyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SSHPublicKey, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SSHPublicKey::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Status" {
				obj.status = try!(statusTypeParser::parse_xml("Status", stack));
				continue;
			}
			if current_name == "SSHPublicKeyBody" {
				obj.ssh_public_key_body = try!(publicKeyMaterialTypeParser::parse_xml("SSHPublicKeyBody", stack));
				continue;
			}
			if current_name == "UploadDate" {
				obj.upload_date = Some(try!(dateTypeParser::parse_xml("UploadDate", stack)));
				continue;
			}
			if current_name == "Fingerprint" {
				obj.fingerprint = try!(publicKeyFingerprintTypeParser::parse_xml("Fingerprint", stack));
				continue;
			}
			if current_name == "SSHPublicKeyId" {
				obj.ssh_public_key_id = try!(publicKeyIdTypeParser::parse_xml("SSHPublicKeyId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SSHPublicKey contents to a SignedRequest
struct SSHPublicKeyWriter;
impl SSHPublicKeyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SSHPublicKey) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		statusTypeWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		publicKeyMaterialTypeWriter::write_params(params, &(prefix.to_string() + "SSHPublicKeyBody"), &obj.ssh_public_key_body);
		if let Some(ref obj) = obj.upload_date {
			dateTypeWriter::write_params(params, &(prefix.to_string() + "UploadDate"), obj);
		}
		publicKeyFingerprintTypeWriter::write_params(params, &(prefix.to_string() + "Fingerprint"), &obj.fingerprint);
		publicKeyIdTypeWriter::write_params(params, &(prefix.to_string() + "SSHPublicKeyId"), &obj.ssh_public_key_id);
	}
}
pub type entityListType = Vec<EntityType>;
/// Parse entityListType from XML
struct entityListTypeParser;
impl entityListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<entityListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "EntityType" {
			obj.push(try!(EntityTypeParser::parse_xml("EntityType", stack)));
		}
		Ok(obj)
	}
}
/// Write entityListType contents to a SignedRequest
struct entityListTypeWriter;
impl entityListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &entityListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			EntityTypeWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct EnableMFADeviceRequest {
	/// The name of the user for whom you want to enable the MFA device.
	pub user_name: existingUserNameType,
	/// An authentication code emitted by the device.
	pub authentication_code1: authenticationCodeType,
	/// The serial number that uniquely identifies the MFA device. For virtual MFA
	/// devices, the serial number is the device ARN.
	pub serial_number: serialNumberType,
	/// A subsequent authentication code emitted by the device.
	pub authentication_code2: authenticationCodeType,
}

/// Parse EnableMFADeviceRequest from XML
struct EnableMFADeviceRequestParser;
impl EnableMFADeviceRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EnableMFADeviceRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EnableMFADeviceRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "AuthenticationCode1" {
				obj.authentication_code1 = try!(authenticationCodeTypeParser::parse_xml("AuthenticationCode1", stack));
				continue;
			}
			if current_name == "SerialNumber" {
				obj.serial_number = try!(serialNumberTypeParser::parse_xml("SerialNumber", stack));
				continue;
			}
			if current_name == "AuthenticationCode2" {
				obj.authentication_code2 = try!(authenticationCodeTypeParser::parse_xml("AuthenticationCode2", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EnableMFADeviceRequest contents to a SignedRequest
struct EnableMFADeviceRequestWriter;
impl EnableMFADeviceRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EnableMFADeviceRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		authenticationCodeTypeWriter::write_params(params, &(prefix.to_string() + "AuthenticationCode1"), &obj.authentication_code1);
		serialNumberTypeWriter::write_params(params, &(prefix.to_string() + "SerialNumber"), &obj.serial_number);
		authenticationCodeTypeWriter::write_params(params, &(prefix.to_string() + "AuthenticationCode2"), &obj.authentication_code2);
	}
}
/// Contains the response to a successful GetAccessKeyLastUsed request. It is also
/// returned as a member of the AccessKeyMetaData structure returned by the
/// ListAccessKeys action.
#[derive(Debug, Default)]
pub struct GetAccessKeyLastUsedResponse {
	/// The name of the AWS IAM user that owns this access key.
	pub user_name: existingUserNameType,
	/// Contains information about the last time the access key was used.
	pub access_key_last_used: AccessKeyLastUsed,
}

/// Parse GetAccessKeyLastUsedResponse from XML
struct GetAccessKeyLastUsedResponseParser;
impl GetAccessKeyLastUsedResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetAccessKeyLastUsedResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetAccessKeyLastUsedResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "AccessKeyLastUsed" {
				obj.access_key_last_used = try!(AccessKeyLastUsedParser::parse_xml("AccessKeyLastUsed", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetAccessKeyLastUsedResponse contents to a SignedRequest
struct GetAccessKeyLastUsedResponseWriter;
impl GetAccessKeyLastUsedResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetAccessKeyLastUsedResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		AccessKeyLastUsedWriter::write_params(params, &(prefix.to_string() + "AccessKeyLastUsed"), &obj.access_key_last_used);
	}
}
/// Contains the response to a successful ListSSHPublicKeys request.
#[derive(Debug, Default)]
pub struct ListSSHPublicKeysResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: markerType,
	/// A list of SSH public keys.
	pub ssh_public_keys: SSHPublicKeyListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: booleanType,
}

/// Parse ListSSHPublicKeysResponse from XML
struct ListSSHPublicKeysResponseParser;
impl ListSSHPublicKeysResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSSHPublicKeysResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListSSHPublicKeysResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "SSHPublicKeyMetadata" {
				obj.ssh_public_keys = try!(SSHPublicKeyListTypeParser::parse_xml("SSHPublicKeyMetadata", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = try!(booleanTypeParser::parse_xml("IsTruncated", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListSSHPublicKeysResponse contents to a SignedRequest
struct ListSSHPublicKeysResponseWriter;
impl ListSSHPublicKeysResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListSSHPublicKeysResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		SSHPublicKeyListTypeWriter::write_params(params, &(prefix.to_string() + "SSHPublicKeyMetadata"), &obj.ssh_public_keys);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
	}
}
#[derive(Debug, Default)]
pub struct CreateVirtualMFADeviceRequest {
	/// The path for the virtual MFA device. For more information about paths, see
	/// [IAM Identifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identi
	/// fiers.html) in the _Using IAM_ guide.
	/// This parameter is optional. If it is not included, it defaults to a slash (/).
	pub path: Option<pathType>,
	/// The name of the virtual MFA device. Use with path to uniquely identify a
	/// virtual MFA device.
	pub virtual_mfa_device_name: virtualMFADeviceName,
}

/// Parse CreateVirtualMFADeviceRequest from XML
struct CreateVirtualMFADeviceRequestParser;
impl CreateVirtualMFADeviceRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateVirtualMFADeviceRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateVirtualMFADeviceRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Path" {
				obj.path = Some(try!(pathTypeParser::parse_xml("Path", stack)));
				continue;
			}
			if current_name == "VirtualMFADeviceName" {
				obj.virtual_mfa_device_name = try!(virtualMFADeviceNameParser::parse_xml("VirtualMFADeviceName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateVirtualMFADeviceRequest contents to a SignedRequest
struct CreateVirtualMFADeviceRequestWriter;
impl CreateVirtualMFADeviceRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateVirtualMFADeviceRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.path {
			pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), obj);
		}
		virtualMFADeviceNameWriter::write_params(params, &(prefix.to_string() + "VirtualMFADeviceName"), &obj.virtual_mfa_device_name);
	}
}
#[derive(Debug, Default)]
pub struct AttachRolePolicyRequest {
	/// The name (friendly name, not ARN) of the role to attach the policy to.
	pub role_name: roleNameType,
	pub policy_arn: arnType,
}

/// Parse AttachRolePolicyRequest from XML
struct AttachRolePolicyRequestParser;
impl AttachRolePolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AttachRolePolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AttachRolePolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AttachRolePolicyRequest contents to a SignedRequest
struct AttachRolePolicyRequestWriter;
impl AttachRolePolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AttachRolePolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
#[derive(Debug, Default)]
pub struct AttachUserPolicyRequest {
	/// The name (friendly name, not ARN) of the user to attach the policy to.
	pub user_name: userNameType,
	pub policy_arn: arnType,
}

/// Parse AttachUserPolicyRequest from XML
struct AttachUserPolicyRequestParser;
impl AttachUserPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AttachUserPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AttachUserPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AttachUserPolicyRequest contents to a SignedRequest
struct AttachUserPolicyRequestWriter;
impl AttachUserPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AttachUserPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
/// Contains the response to a successful ListAttachedGroupPolicies request.
#[derive(Debug, Default)]
pub struct ListAttachedGroupPoliciesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: markerType,
	/// A list of the attached policies.
	pub attached_policies: attachedPoliciesListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: booleanType,
}

/// Parse ListAttachedGroupPoliciesResponse from XML
struct ListAttachedGroupPoliciesResponseParser;
impl ListAttachedGroupPoliciesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListAttachedGroupPoliciesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListAttachedGroupPoliciesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "AttachedPolicy" {
				obj.attached_policies = try!(attachedPoliciesListTypeParser::parse_xml("AttachedPolicy", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = try!(booleanTypeParser::parse_xml("IsTruncated", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListAttachedGroupPoliciesResponse contents to a SignedRequest
struct ListAttachedGroupPoliciesResponseWriter;
impl ListAttachedGroupPoliciesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListAttachedGroupPoliciesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		attachedPoliciesListTypeWriter::write_params(params, &(prefix.to_string() + "AttachedPolicy"), &obj.attached_policies);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
	}
}
/// Contains the response to a successful ListAttachedUserPolicies request.
#[derive(Debug, Default)]
pub struct ListAttachedUserPoliciesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: markerType,
	/// A list of the attached policies.
	pub attached_policies: attachedPoliciesListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: booleanType,
}

/// Parse ListAttachedUserPoliciesResponse from XML
struct ListAttachedUserPoliciesResponseParser;
impl ListAttachedUserPoliciesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListAttachedUserPoliciesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListAttachedUserPoliciesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "AttachedPolicy" {
				obj.attached_policies = try!(attachedPoliciesListTypeParser::parse_xml("AttachedPolicy", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = try!(booleanTypeParser::parse_xml("IsTruncated", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListAttachedUserPoliciesResponse contents to a SignedRequest
struct ListAttachedUserPoliciesResponseWriter;
impl ListAttachedUserPoliciesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListAttachedUserPoliciesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		attachedPoliciesListTypeWriter::write_params(params, &(prefix.to_string() + "AttachedPolicy"), &obj.attached_policies);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
	}
}
/// Contains a list of signing certificates.
/// This data type is used as a response element in the ListSigningCertificates
/// action.
pub type certificateListType = Vec<SigningCertificate>;
/// Parse certificateListType from XML
struct certificateListTypeParser;
impl certificateListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<certificateListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "SigningCertificate" {
			obj.push(try!(SigningCertificateParser::parse_xml("SigningCertificate", stack)));
		}
		Ok(obj)
	}
}
/// Write certificateListType contents to a SignedRequest
struct certificateListTypeWriter;
impl certificateListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &certificateListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			SigningCertificateWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct ListMFADevicesRequest {
	/// The name of the user whose MFA devices you want to list.
	pub user_name: existingUserNameType,
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListMFADevicesRequest from XML
struct ListMFADevicesRequestParser;
impl ListMFADevicesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListMFADevicesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListMFADevicesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListMFADevicesRequest contents to a SignedRequest
struct ListMFADevicesRequestWriter;
impl ListMFADevicesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListMFADevicesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
/// The request was rejected because the same certificate is associated with an
/// IAM user in the account.
#[derive(Debug, Default)]
pub struct DuplicateCertificateException {
	pub message: duplicateCertificateMessage,
}

/// Parse DuplicateCertificateException from XML
struct DuplicateCertificateExceptionParser;
impl DuplicateCertificateExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DuplicateCertificateException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DuplicateCertificateException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(duplicateCertificateMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DuplicateCertificateException contents to a SignedRequest
struct DuplicateCertificateExceptionWriter;
impl DuplicateCertificateExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DuplicateCertificateException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		duplicateCertificateMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains a list of instance profiles.
pub type instanceProfileListType = Vec<InstanceProfile>;
/// Parse instanceProfileListType from XML
struct instanceProfileListTypeParser;
impl instanceProfileListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<instanceProfileListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "InstanceProfile" {
			obj.push(try!(InstanceProfileParser::parse_xml("InstanceProfile", stack)));
		}
		Ok(obj)
	}
}
/// Write instanceProfileListType contents to a SignedRequest
struct instanceProfileListTypeWriter;
impl instanceProfileListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &instanceProfileListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			InstanceProfileWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Contains information about an MFA device.
/// This data type is used as a response element in the ListMFADevices action.
#[derive(Debug, Default)]
pub struct MFADevice {
	/// The user with whom the MFA device is associated.
	pub user_name: userNameType,
	/// The serial number that uniquely identifies the MFA device. For virtual MFA
	/// devices, the serial number is the device ARN.
	pub serial_number: serialNumberType,
	/// The date when the MFA device was enabled for the user.
	pub enable_date: dateType,
}

/// Parse MFADevice from XML
struct MFADeviceParser;
impl MFADeviceParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MFADevice, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = MFADevice::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "SerialNumber" {
				obj.serial_number = try!(serialNumberTypeParser::parse_xml("SerialNumber", stack));
				continue;
			}
			if current_name == "EnableDate" {
				obj.enable_date = try!(dateTypeParser::parse_xml("EnableDate", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MFADevice contents to a SignedRequest
struct MFADeviceWriter;
impl MFADeviceWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MFADevice) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		serialNumberTypeWriter::write_params(params, &(prefix.to_string() + "SerialNumber"), &obj.serial_number);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "EnableDate"), &obj.enable_date);
	}
}
#[derive(Debug, Default)]
pub struct ListSigningCertificatesRequest {
	/// The name of the user.
	pub user_name: existingUserNameType,
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListSigningCertificatesRequest from XML
struct ListSigningCertificatesRequestParser;
impl ListSigningCertificatesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSigningCertificatesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListSigningCertificatesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListSigningCertificatesRequest contents to a SignedRequest
struct ListSigningCertificatesRequestWriter;
impl ListSigningCertificatesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListSigningCertificatesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
pub type policyVersionIdType = String;
/// Parse policyVersionIdType from XML
struct policyVersionIdTypeParser;
impl policyVersionIdTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<policyVersionIdType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write policyVersionIdType contents to a SignedRequest
struct policyVersionIdTypeWriter;
impl policyVersionIdTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &policyVersionIdType) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful GetPolicyVersion request.
#[derive(Debug, Default)]
pub struct GetPolicyVersionResponse {
	/// Information about the policy version.
	/// For more information about managed policy versions, see [Versioning for
	/// Managed Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-versions.html) in the _Using IAM_ guide.
	pub policy_version: PolicyVersion,
}

/// Parse GetPolicyVersionResponse from XML
struct GetPolicyVersionResponseParser;
impl GetPolicyVersionResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetPolicyVersionResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetPolicyVersionResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyVersion" {
				obj.policy_version = try!(PolicyVersionParser::parse_xml("PolicyVersion", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetPolicyVersionResponse contents to a SignedRequest
struct GetPolicyVersionResponseWriter;
impl GetPolicyVersionResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetPolicyVersionResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		PolicyVersionWriter::write_params(params, &(prefix.to_string() + "PolicyVersion"), &obj.policy_version);
	}
}
pub type deleteConflictMessage = String;
/// Parse deleteConflictMessage from XML
struct deleteConflictMessageParser;
impl deleteConflictMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<deleteConflictMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write deleteConflictMessage contents to a SignedRequest
struct deleteConflictMessageWriter;
impl deleteConflictMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &deleteConflictMessage) {
		params.put(name, obj);
	}
}
pub type existingUserNameType = String;
/// Parse existingUserNameType from XML
struct existingUserNameTypeParser;
impl existingUserNameTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<existingUserNameType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write existingUserNameType contents to a SignedRequest
struct existingUserNameTypeWriter;
impl existingUserNameTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &existingUserNameType) {
		params.put(name, obj);
	}
}
pub type dateType = String;
/// Parse dateType from XML
struct dateTypeParser;
impl dateTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<dateType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write dateType contents to a SignedRequest
struct dateTypeWriter;
impl dateTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &dateType) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful GetGroup request.
#[derive(Debug, Default)]
pub struct GetGroupResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// Information about the group.
	pub group: Group,
	/// A list of users in the group.
	pub users: userListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
}

/// Parse GetGroupResponse from XML
struct GetGroupResponseParser;
impl GetGroupResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetGroupResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetGroupResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "Group" {
				obj.group = try!(GroupParser::parse_xml("Group", stack));
				continue;
			}
			if current_name == "User" {
				obj.users = try!(userListTypeParser::parse_xml("User", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetGroupResponse contents to a SignedRequest
struct GetGroupResponseWriter;
impl GetGroupResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetGroupResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		GroupWriter::write_params(params, &(prefix.to_string() + "Group"), &obj.group);
		userListTypeWriter::write_params(params, &(prefix.to_string() + "User"), &obj.users);
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct GetUserRequest {
	/// The name of the user to get information about.
	/// This parameter is optional. If it is not included, it defaults to the user
	/// making the request.
	pub user_name: existingUserNameType,
}

/// Parse GetUserRequest from XML
struct GetUserRequestParser;
impl GetUserRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetUserRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetUserRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetUserRequest contents to a SignedRequest
struct GetUserRequestWriter;
impl GetUserRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetUserRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
	}
}
/// The request was rejected because the policy document was malformed. The error
/// message describes the specific error.
#[derive(Debug, Default)]
pub struct MalformedPolicyDocumentException {
	pub message: malformedPolicyDocumentMessage,
}

/// Parse MalformedPolicyDocumentException from XML
struct MalformedPolicyDocumentExceptionParser;
impl MalformedPolicyDocumentExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MalformedPolicyDocumentException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = MalformedPolicyDocumentException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(malformedPolicyDocumentMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MalformedPolicyDocumentException contents to a SignedRequest
struct MalformedPolicyDocumentExceptionWriter;
impl MalformedPolicyDocumentExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MalformedPolicyDocumentException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		malformedPolicyDocumentMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
#[derive(Debug, Default)]
pub struct GetInstanceProfileRequest {
	/// The name of the instance profile to get information about.
	pub instance_profile_name: instanceProfileNameType,
}

/// Parse GetInstanceProfileRequest from XML
struct GetInstanceProfileRequestParser;
impl GetInstanceProfileRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetInstanceProfileRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetInstanceProfileRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "InstanceProfileName" {
				obj.instance_profile_name = try!(instanceProfileNameTypeParser::parse_xml("InstanceProfileName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetInstanceProfileRequest contents to a SignedRequest
struct GetInstanceProfileRequestWriter;
impl GetInstanceProfileRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetInstanceProfileRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		instanceProfileNameTypeWriter::write_params(params, &(prefix.to_string() + "InstanceProfileName"), &obj.instance_profile_name);
	}
}
/// Contains the response to a successful ListRolePolicies request.
#[derive(Debug, Default)]
pub struct ListRolePoliciesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A list of policy names.
	pub policy_names: policyNameListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
}

/// Parse ListRolePoliciesResponse from XML
struct ListRolePoliciesResponseParser;
impl ListRolePoliciesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListRolePoliciesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListRolePoliciesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "policyNameType" {
				obj.policy_names = try!(policyNameListTypeParser::parse_xml("policyNameType", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListRolePoliciesResponse contents to a SignedRequest
struct ListRolePoliciesResponseWriter;
impl ListRolePoliciesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListRolePoliciesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		policyNameListTypeWriter::write_params(params, &(prefix.to_string() + "policyNameType"), &obj.policy_names);
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct AttachGroupPolicyRequest {
	/// The name (friendly name, not ARN) of the group to attach the policy to.
	pub group_name: groupNameType,
	pub policy_arn: arnType,
}

/// Parse AttachGroupPolicyRequest from XML
struct AttachGroupPolicyRequestParser;
impl AttachGroupPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AttachGroupPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AttachGroupPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AttachGroupPolicyRequest contents to a SignedRequest
struct AttachGroupPolicyRequestWriter;
impl AttachGroupPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AttachGroupPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
pub type accessKeyIdType = String;
/// Parse accessKeyIdType from XML
struct accessKeyIdTypeParser;
impl accessKeyIdTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<accessKeyIdType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write accessKeyIdType contents to a SignedRequest
struct accessKeyIdTypeWriter;
impl accessKeyIdTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &accessKeyIdType) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful ListPolicies request.
#[derive(Debug, Default)]
pub struct ListPoliciesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: markerType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: booleanType,
	/// A list of policies.
	pub policies: policyListType,
}

/// Parse ListPoliciesResponse from XML
struct ListPoliciesResponseParser;
impl ListPoliciesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListPoliciesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListPoliciesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = try!(booleanTypeParser::parse_xml("IsTruncated", stack));
				continue;
			}
			if current_name == "Policy" {
				obj.policies = try!(policyListTypeParser::parse_xml("Policy", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListPoliciesResponse contents to a SignedRequest
struct ListPoliciesResponseWriter;
impl ListPoliciesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListPoliciesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
		policyListTypeWriter::write_params(params, &(prefix.to_string() + "Policy"), &obj.policies);
	}
}
pub type ManagedPolicyDetailListType = Vec<ManagedPolicyDetail>;
/// Parse ManagedPolicyDetailListType from XML
struct ManagedPolicyDetailListTypeParser;
impl ManagedPolicyDetailListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ManagedPolicyDetailListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ManagedPolicyDetail" {
			obj.push(try!(ManagedPolicyDetailParser::parse_xml("ManagedPolicyDetail", stack)));
		}
		Ok(obj)
	}
}
/// Write ManagedPolicyDetailListType contents to a SignedRequest
struct ManagedPolicyDetailListTypeWriter;
impl ManagedPolicyDetailListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ManagedPolicyDetailListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ManagedPolicyDetailWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type idType = String;
/// Parse idType from XML
struct idTypeParser;
impl idTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<idType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write idType contents to a SignedRequest
struct idTypeWriter;
impl idTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &idType) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful CreateInstanceProfile request.
#[derive(Debug, Default)]
pub struct CreateInstanceProfileResponse {
	/// Information about the instance profile.
	pub instance_profile: InstanceProfile,
}

/// Parse CreateInstanceProfileResponse from XML
struct CreateInstanceProfileResponseParser;
impl CreateInstanceProfileResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateInstanceProfileResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateInstanceProfileResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "InstanceProfile" {
				obj.instance_profile = try!(InstanceProfileParser::parse_xml("InstanceProfile", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateInstanceProfileResponse contents to a SignedRequest
struct CreateInstanceProfileResponseWriter;
impl CreateInstanceProfileResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateInstanceProfileResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		InstanceProfileWriter::write_params(params, &(prefix.to_string() + "InstanceProfile"), &obj.instance_profile);
	}
}
#[derive(Debug, Default)]
pub struct ListGroupPoliciesRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// The name of the group to list policies for.
	pub group_name: groupNameType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
}

/// Parse ListGroupPoliciesRequest from XML
struct ListGroupPoliciesRequestParser;
impl ListGroupPoliciesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListGroupPoliciesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListGroupPoliciesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListGroupPoliciesRequest contents to a SignedRequest
struct ListGroupPoliciesRequestWriter;
impl ListGroupPoliciesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListGroupPoliciesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
	}
}
/// Contains the response to a successful GetAccountSummary request.
#[derive(Debug, Default)]
pub struct GetAccountSummaryResponse {
	/// A set of key value pairs containing information about IAM entity usage and IAM
	/// quotas.
	/// `SummaryMap` contains the following keys:
	///   * **AccessKeysPerUserQuota**
	/// The maximum number of active access keys allowed for each IAM user.
	///   * **AccountAccessKeysPresent**
	/// This value is 1 if the AWS account (root) has an access key, otherwise it is
	/// 0.
	///   * **AccountMFAEnabled**
	/// This value is 1 if the AWS account (root) has an MFA device assigned,
	/// otherwise it is 0.
	///   * **AccountSigningCertificatesPresent**
	/// This value is 1 if the AWS account (root) has a signing certificate, otherwise
	/// it is 0.
	///   * **AssumeRolePolicySizeQuota**
	/// The maximum allowed size for assume role policy documents (trust policies), in
	/// non-whitespace characters.
	///   * **AttachedPoliciesPerGroupQuota**
	/// The maximum number of managed policies that can be attached to an IAM group.
	///   * **AttachedPoliciesPerRoleQuota**
	/// The maximum number of managed policies that can be attached to an IAM role.
	///   * **AttachedPoliciesPerUserQuota**
	/// The maximum number of managed policies that can be attached to an IAM user.
	///   * **GroupPolicySizeQuota**
	/// The maximum allowed size for the aggregate of all inline policies embedded in
	/// an IAM group, in non-whitespace characters.
	///   * **Groups**
	/// The number of IAM groups in the AWS account.
	///   * **GroupsPerUserQuota**
	/// The maximum number of IAM groups each IAM user can belong to.
	///   * **GroupsQuota**
	/// The maximum number of IAM groups allowed in the AWS account.
	///   * **InstanceProfiles**
	/// The number of instance profiles in the AWS account.
	///   * **InstanceProfilesQuota**
	/// The maximum number of instance profiles allowed in the AWS account.
	///   * **MFADevices**
	/// The number of MFA devices in the AWS account, including those assigned and
	/// unassigned.
	///   * **MFADevicesInUse**
	/// The number of MFA devices that have been assigned to an IAM user or to the AWS
	/// account (root).
	///   * **Policies**
	/// The number of customer managed policies in the AWS account.
	///   * **PoliciesQuota**
	/// The maximum number of customer managed policies allowed in the AWS account.
	///   * **PolicySizeQuota**
	/// The maximum allowed size of a customer managed policy, in non-whitespace
	/// characters.
	///   * **PolicyVersionsInUse**
	/// The number of managed policies that are attached to IAM users, groups, or
	/// roles in the AWS account.
	///   * **PolicyVersionsInUseQuota**
	/// The maximum number of managed policies that can be attached to IAM users,
	/// groups, or roles in the AWS account.
	///   * **Providers**
	/// The number of identity providers in the AWS account.
	///   * **RolePolicySizeQuota**
	/// The maximum allowed size for the aggregate of all inline policies (access
	/// policies, not the trust policy) embedded in an IAM role, in non-whitespace
	/// characters.
	///   * **Roles**
	/// The number of IAM roles in the AWS account.
	///   * **RolesQuota**
	/// The maximum number of IAM roles allowed in the AWS account.
	///   * **ServerCertificates**
	/// The number of server certificates in the AWS account.
	///   * **ServerCertificatesQuota**
	/// The maximum number of server certificates allowed in the AWS account.
	///   * **SigningCertificatesPerUserQuota**
	/// The maximum number of X.509 signing certificates allowed for each IAM user.
	///   * **UserPolicySizeQuota**
	/// The maximum allowed size for the aggregate of all inline policies embedded in
	/// an IAM user, in non-whitespace characters.
	///   * **Users**
	/// The number of IAM users in the AWS account.
	///   * **UsersQuota**
	/// The maximum number of IAM users allowed in the AWS account.
	///   * **VersionsPerPolicyQuota**
	/// The maximum number of policy versions allowed for each managed policy.
	pub summary_map: summaryMapType,
}

/// Parse GetAccountSummaryResponse from XML
struct GetAccountSummaryResponseParser;
impl GetAccountSummaryResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetAccountSummaryResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetAccountSummaryResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SummaryMap" {
				obj.summary_map = try!(summaryMapTypeParser::parse_xml("SummaryMap", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetAccountSummaryResponse contents to a SignedRequest
struct GetAccountSummaryResponseWriter;
impl GetAccountSummaryResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetAccountSummaryResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		summaryMapTypeWriter::write_params(params, &(prefix.to_string() + "SummaryMap"), &obj.summary_map);
	}
}
pub type invalidCertificateMessage = String;
/// Parse invalidCertificateMessage from XML
struct invalidCertificateMessageParser;
impl invalidCertificateMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<invalidCertificateMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write invalidCertificateMessage contents to a SignedRequest
struct invalidCertificateMessageWriter;
impl invalidCertificateMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &invalidCertificateMessage) {
		params.put(name, obj);
	}
}
pub type certificateBodyType = String;
/// Parse certificateBodyType from XML
struct certificateBodyTypeParser;
impl certificateBodyTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<certificateBodyType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write certificateBodyType contents to a SignedRequest
struct certificateBodyTypeWriter;
impl certificateBodyTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &certificateBodyType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct UploadSigningCertificateRequest {
	/// The name of the user the signing certificate is for.
	pub user_name: Option<existingUserNameType>,
	/// The contents of the signing certificate.
	pub certificate_body: certificateBodyType,
}

/// Parse UploadSigningCertificateRequest from XML
struct UploadSigningCertificateRequestParser;
impl UploadSigningCertificateRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UploadSigningCertificateRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UploadSigningCertificateRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = Some(try!(existingUserNameTypeParser::parse_xml("UserName", stack)));
				continue;
			}
			if current_name == "CertificateBody" {
				obj.certificate_body = try!(certificateBodyTypeParser::parse_xml("CertificateBody", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UploadSigningCertificateRequest contents to a SignedRequest
struct UploadSigningCertificateRequestWriter;
impl UploadSigningCertificateRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UploadSigningCertificateRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.user_name {
			existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), obj);
		}
		certificateBodyTypeWriter::write_params(params, &(prefix.to_string() + "CertificateBody"), &obj.certificate_body);
	}
}
pub type policyScopeType = String;
/// Parse policyScopeType from XML
struct policyScopeTypeParser;
impl policyScopeTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<policyScopeType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write policyScopeType contents to a SignedRequest
struct policyScopeTypeWriter;
impl policyScopeTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &policyScopeType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct ListRolePoliciesRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// The name of the role to list policies for.
	pub role_name: roleNameType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
}

/// Parse ListRolePoliciesRequest from XML
struct ListRolePoliciesRequestParser;
impl ListRolePoliciesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListRolePoliciesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListRolePoliciesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListRolePoliciesRequest contents to a SignedRequest
struct ListRolePoliciesRequestWriter;
impl ListRolePoliciesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListRolePoliciesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
	}
}
pub type attachedPoliciesListType = Vec<AttachedPolicy>;
/// Parse attachedPoliciesListType from XML
struct attachedPoliciesListTypeParser;
impl attachedPoliciesListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<attachedPoliciesListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AttachedPolicy" {
			obj.push(try!(AttachedPolicyParser::parse_xml("AttachedPolicy", stack)));
		}
		Ok(obj)
	}
}
/// Write attachedPoliciesListType contents to a SignedRequest
struct attachedPoliciesListTypeWriter;
impl attachedPoliciesListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &attachedPoliciesListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AttachedPolicyWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct CreateUserRequest {
	/// The name of the user to create.
	pub user_name: userNameType,
	/// The path for the user name. For more information about paths, see [IAM Identif
	/// iers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html)
	/// in the _Using IAM_ guide.
	/// This parameter is optional. If it is not included, it defaults to a slash (/).
	pub path: Option<pathType>,
}

/// Parse CreateUserRequest from XML
struct CreateUserRequestParser;
impl CreateUserRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateUserRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateUserRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Path" {
				obj.path = Some(try!(pathTypeParser::parse_xml("Path", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateUserRequest contents to a SignedRequest
struct CreateUserRequestWriter;
impl CreateUserRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateUserRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		if let Some(ref obj) = obj.path {
			pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), obj);
		}
	}
}
pub type serialNumberType = String;
/// Parse serialNumberType from XML
struct serialNumberTypeParser;
impl serialNumberTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<serialNumberType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write serialNumberType contents to a SignedRequest
struct serialNumberTypeWriter;
impl serialNumberTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &serialNumberType) {
		params.put(name, obj);
	}
}
pub type passwordPolicyViolationMessage = String;
/// Parse passwordPolicyViolationMessage from XML
struct passwordPolicyViolationMessageParser;
impl passwordPolicyViolationMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<passwordPolicyViolationMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write passwordPolicyViolationMessage contents to a SignedRequest
struct passwordPolicyViolationMessageWriter;
impl passwordPolicyViolationMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &passwordPolicyViolationMessage) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct ListEntitiesForPolicyRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// The entity type to use for filtering the results.
	/// For example, when `EntityFilter` is `Role`, only the roles that are attached
	/// to the specified policy are returned. This parameter is optional. If it is not
	/// included, all attached entities (users, groups, and roles) are returned.
	pub entity_filter: Option<EntityType>,
	/// The path prefix for filtering the results. This parameter is optional. If it
	/// is not included, it defaults to a slash (/), listing all entities.
	pub path_prefix: Option<pathType>,
	pub policy_arn: arnType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
}

/// Parse ListEntitiesForPolicyRequest from XML
struct ListEntitiesForPolicyRequestParser;
impl ListEntitiesForPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListEntitiesForPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListEntitiesForPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "EntityFilter" {
				obj.entity_filter = Some(try!(EntityTypeParser::parse_xml("EntityFilter", stack)));
				continue;
			}
			if current_name == "PathPrefix" {
				obj.path_prefix = Some(try!(pathTypeParser::parse_xml("PathPrefix", stack)));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListEntitiesForPolicyRequest contents to a SignedRequest
struct ListEntitiesForPolicyRequestWriter;
impl ListEntitiesForPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListEntitiesForPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.entity_filter {
			EntityTypeWriter::write_params(params, &(prefix.to_string() + "EntityFilter"), obj);
		}
		if let Some(ref obj) = obj.path_prefix {
			pathTypeWriter::write_params(params, &(prefix.to_string() + "PathPrefix"), obj);
		}
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
	}
}
/// The request was rejected because the credential report is still being
/// generated.
#[derive(Debug, Default)]
pub struct CredentialReportNotReadyException {
	pub message: credentialReportNotReadyExceptionMessage,
}

/// Parse CredentialReportNotReadyException from XML
struct CredentialReportNotReadyExceptionParser;
impl CredentialReportNotReadyExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CredentialReportNotReadyException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CredentialReportNotReadyException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(credentialReportNotReadyExceptionMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CredentialReportNotReadyException contents to a SignedRequest
struct CredentialReportNotReadyExceptionWriter;
impl CredentialReportNotReadyExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CredentialReportNotReadyException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		credentialReportNotReadyExceptionMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains information about a group that a managed policy is attached to.
/// This data type is used as a response element in the ListEntitiesForPolicy
/// action.
/// For more information about managed policies, refer to [Managed Policies and
/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
/// managed-vs-inline.html) in the _Using IAM_ guide.
#[derive(Debug, Default)]
pub struct PolicyGroup {
	/// The name (friendly name, not ARN) identifying the group.
	pub group_name: groupNameType,
}

/// Parse PolicyGroup from XML
struct PolicyGroupParser;
impl PolicyGroupParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyGroup, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PolicyGroup::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PolicyGroup contents to a SignedRequest
struct PolicyGroupWriter;
impl PolicyGroupWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyGroup) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
	}
}
pub type invalidPublicKeyMessage = String;
/// Parse invalidPublicKeyMessage from XML
struct invalidPublicKeyMessageParser;
impl invalidPublicKeyMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<invalidPublicKeyMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write invalidPublicKeyMessage contents to a SignedRequest
struct invalidPublicKeyMessageWriter;
impl invalidPublicKeyMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &invalidPublicKeyMessage) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct ListPoliciesRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// The scope to use for filtering the results.
	/// To list only AWS managed policies, set `Scope` to `AWS`. To list only the
	/// customer managed policies in your AWS account, set `Scope` to `Local`.
	/// This parameter is optional. If it is not included, or if it is set to `All`,
	/// all policies are returned.
	pub scope: policyScopeType,
	/// A flag to filter the results to only the attached policies.
	/// When `OnlyAttached` is `true`, the returned list contains only the policies
	/// that are attached to a user, group, or role. When `OnlyAttached` is `false`,
	/// or when the parameter is not included, all policies are returned.
	pub only_attached: booleanType,
	/// The path prefix for filtering the results. This parameter is optional. If it
	/// is not included, it defaults to a slash (/), listing all policies.
	pub path_prefix: policyPathType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListPoliciesRequest from XML
struct ListPoliciesRequestParser;
impl ListPoliciesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListPoliciesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListPoliciesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "Scope" {
				obj.scope = try!(policyScopeTypeParser::parse_xml("Scope", stack));
				continue;
			}
			if current_name == "OnlyAttached" {
				obj.only_attached = try!(booleanTypeParser::parse_xml("OnlyAttached", stack));
				continue;
			}
			if current_name == "PathPrefix" {
				obj.path_prefix = try!(policyPathTypeParser::parse_xml("PathPrefix", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListPoliciesRequest contents to a SignedRequest
struct ListPoliciesRequestWriter;
impl ListPoliciesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListPoliciesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		policyScopeTypeWriter::write_params(params, &(prefix.to_string() + "Scope"), &obj.scope);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "OnlyAttached"), &obj.only_attached);
		policyPathTypeWriter::write_params(params, &(prefix.to_string() + "PathPrefix"), &obj.path_prefix);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
pub type serviceFailureExceptionMessage = String;
/// Parse serviceFailureExceptionMessage from XML
struct serviceFailureExceptionMessageParser;
impl serviceFailureExceptionMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<serviceFailureExceptionMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write serviceFailureExceptionMessage contents to a SignedRequest
struct serviceFailureExceptionMessageWriter;
impl serviceFailureExceptionMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &serviceFailureExceptionMessage) {
		params.put(name, obj);
	}
}
/// Contains information about a virtual MFA device.
#[derive(Debug, Default)]
pub struct VirtualMFADevice {
	/// The Base32 seed defined as specified in
	/// [RFC3548](http://www.ietf.org/rfc/rfc3548.txt). The `Base32StringSeed` is
	/// Base64-encoded.
	pub base32_string_seed: Option<BootstrapDatum>,
	/// The serial number associated with `VirtualMFADevice`.
	pub serial_number: serialNumberType,
	/// The date and time on which the virtual MFA device was enabled.
	pub enable_date: Option<dateType>,
	pub user: Option<User>,
	/// A QR code PNG image that encodes
	/// `otpauth://totp/$virtualMFADeviceName@$AccountName?secret=$Base32String` where
	/// `$virtualMFADeviceName` is one of the create call arguments, `AccountName` is
	/// the user name if set (otherwise, the account ID otherwise), and `Base32String`
	/// is the seed in Base32 format. The `Base32String` value is Base64-encoded.
	pub qr_code_png: Option<BootstrapDatum>,
}

/// Parse VirtualMFADevice from XML
struct VirtualMFADeviceParser;
impl VirtualMFADeviceParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<VirtualMFADevice, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = VirtualMFADevice::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Base32StringSeed" {
				obj.base32_string_seed = Some(try!(BootstrapDatumParser::parse_xml("Base32StringSeed", stack)));
				continue;
			}
			if current_name == "SerialNumber" {
				obj.serial_number = try!(serialNumberTypeParser::parse_xml("SerialNumber", stack));
				continue;
			}
			if current_name == "EnableDate" {
				obj.enable_date = Some(try!(dateTypeParser::parse_xml("EnableDate", stack)));
				continue;
			}
			if current_name == "User" {
				obj.user = Some(try!(UserParser::parse_xml("User", stack)));
				continue;
			}
			if current_name == "QRCodePNG" {
				obj.qr_code_png = Some(try!(BootstrapDatumParser::parse_xml("QRCodePNG", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write VirtualMFADevice contents to a SignedRequest
struct VirtualMFADeviceWriter;
impl VirtualMFADeviceWriter {
	fn write_params(params: &mut Params, name: &str, obj: &VirtualMFADevice) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.base32_string_seed {
			BootstrapDatumWriter::write_params(params, &(prefix.to_string() + "Base32StringSeed"), obj);
		}
		serialNumberTypeWriter::write_params(params, &(prefix.to_string() + "SerialNumber"), &obj.serial_number);
		if let Some(ref obj) = obj.enable_date {
			dateTypeWriter::write_params(params, &(prefix.to_string() + "EnableDate"), obj);
		}
		if let Some(ref obj) = obj.user {
			UserWriter::write_params(params, &(prefix.to_string() + "User"), obj);
		}
		if let Some(ref obj) = obj.qr_code_png {
			BootstrapDatumWriter::write_params(params, &(prefix.to_string() + "QRCodePNG"), obj);
		}
	}
}
/// Contains the response to a successful GetOpenIDConnectProvider request.
#[derive(Debug, Default)]
pub struct GetOpenIDConnectProviderResponse {
	/// The URL that the IAM OpenID Connect provider is associated with. For more
	/// information, see CreateOpenIDConnectProvider.
	pub url: OpenIDConnectProviderUrlType,
	/// The date and time when the IAM OpenID Connect provider entity was created in
	/// the AWS account.
	pub create_date: dateType,
	/// A list of certificate thumbprints that are associated with the specified IAM
	/// OpenID Connect provider. For more information, see
	/// CreateOpenIDConnectProvider.
	pub thumbprint_list: thumbprintListType,
	/// A list of client IDs (also known as audiences) that are associated with the
	/// specified IAM OpenID Connect provider. For more information, see
	/// CreateOpenIDConnectProvider.
	pub client_id_list: clientIDListType,
}

/// Parse GetOpenIDConnectProviderResponse from XML
struct GetOpenIDConnectProviderResponseParser;
impl GetOpenIDConnectProviderResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetOpenIDConnectProviderResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetOpenIDConnectProviderResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Url" {
				obj.url = try!(OpenIDConnectProviderUrlTypeParser::parse_xml("Url", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "thumbprintType" {
				obj.thumbprint_list = try!(thumbprintListTypeParser::parse_xml("thumbprintType", stack));
				continue;
			}
			if current_name == "clientIDType" {
				obj.client_id_list = try!(clientIDListTypeParser::parse_xml("clientIDType", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetOpenIDConnectProviderResponse contents to a SignedRequest
struct GetOpenIDConnectProviderResponseWriter;
impl GetOpenIDConnectProviderResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetOpenIDConnectProviderResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		OpenIDConnectProviderUrlTypeWriter::write_params(params, &(prefix.to_string() + "Url"), &obj.url);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		thumbprintListTypeWriter::write_params(params, &(prefix.to_string() + "thumbprintType"), &obj.thumbprint_list);
		clientIDListTypeWriter::write_params(params, &(prefix.to_string() + "clientIDType"), &obj.client_id_list);
	}
}
pub type policyDocumentType = String;
/// Parse policyDocumentType from XML
struct policyDocumentTypeParser;
impl policyDocumentTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<policyDocumentType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write policyDocumentType contents to a SignedRequest
struct policyDocumentTypeWriter;
impl policyDocumentTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &policyDocumentType) {
		params.put(name, obj);
	}
}
/// Contains a list of IAM groups.
/// This data type is used as a response element in the ListGroups action.
pub type groupListType = Vec<Group>;
/// Parse groupListType from XML
struct groupListTypeParser;
impl groupListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<groupListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Group" {
			obj.push(try!(GroupParser::parse_xml("Group", stack)));
		}
		Ok(obj)
	}
}
/// Write groupListType contents to a SignedRequest
struct groupListTypeWriter;
impl groupListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &groupListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			GroupWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type SAMLProviderNameType = String;
/// Parse SAMLProviderNameType from XML
struct SAMLProviderNameTypeParser;
impl SAMLProviderNameTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SAMLProviderNameType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SAMLProviderNameType contents to a SignedRequest
struct SAMLProviderNameTypeWriter;
impl SAMLProviderNameTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SAMLProviderNameType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct CreateAccountAliasRequest {
	/// The account alias to create.
	pub account_alias: accountAliasType,
}

/// Parse CreateAccountAliasRequest from XML
struct CreateAccountAliasRequestParser;
impl CreateAccountAliasRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateAccountAliasRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateAccountAliasRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AccountAlias" {
				obj.account_alias = try!(accountAliasTypeParser::parse_xml("AccountAlias", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateAccountAliasRequest contents to a SignedRequest
struct CreateAccountAliasRequestWriter;
impl CreateAccountAliasRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateAccountAliasRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		accountAliasTypeWriter::write_params(params, &(prefix.to_string() + "AccountAlias"), &obj.account_alias);
	}
}
pub type roleNameType = String;
/// Parse roleNameType from XML
struct roleNameTypeParser;
impl roleNameTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<roleNameType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write roleNameType contents to a SignedRequest
struct roleNameTypeWriter;
impl roleNameTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &roleNameType) {
		params.put(name, obj);
	}
}
/// Contains information about an IAM role, including all of the role's policies.
/// This data type is used as a response element in the
/// GetAccountAuthorizationDetails action.
#[derive(Debug, Default)]
pub struct RoleDetail {
	/// The trust policy that grants permission to assume the role.
	pub assume_role_policy_document: policyDocumentType,
	/// The stable and unique string identifying the role. For more information about
	/// IDs, see [IAM Identifiers](http://docs.aws.amazon.com/IAM/latest/UserGuide/Usi
	/// ng_Identifiers.html) in the _Using IAM_ guide.
	pub role_id: idType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the role was created.
	pub create_date: dateType,
	pub instance_profile_list: instanceProfileListType,
	/// The friendly name that identifies the role.
	pub role_name: roleNameType,
	/// The path to the role. For more information about paths, see [IAM Identifiers](
	/// http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the
	/// _Using IAM_ guide.
	pub path: pathType,
	/// A list of managed policies attached to the role. These policies are the role's
	/// access (permissions) policies.
	pub attached_managed_policies: attachedPoliciesListType,
	/// A list of inline policies embedded in the role. These policies are the role's
	/// access (permissions) policies.
	pub role_policy_list: policyDetailListType,
	pub arn: arnType,
}

/// Parse RoleDetail from XML
struct RoleDetailParser;
impl RoleDetailParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<RoleDetail, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = RoleDetail::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AssumeRolePolicyDocument" {
				obj.assume_role_policy_document = try!(policyDocumentTypeParser::parse_xml("AssumeRolePolicyDocument", stack));
				continue;
			}
			if current_name == "RoleId" {
				obj.role_id = try!(idTypeParser::parse_xml("RoleId", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "InstanceProfile" {
				obj.instance_profile_list = try!(instanceProfileListTypeParser::parse_xml("InstanceProfile", stack));
				continue;
			}
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "Path" {
				obj.path = try!(pathTypeParser::parse_xml("Path", stack));
				continue;
			}
			if current_name == "AttachedPolicy" {
				obj.attached_managed_policies = try!(attachedPoliciesListTypeParser::parse_xml("AttachedPolicy", stack));
				continue;
			}
			if current_name == "PolicyDetail" {
				obj.role_policy_list = try!(policyDetailListTypeParser::parse_xml("PolicyDetail", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write RoleDetail contents to a SignedRequest
struct RoleDetailWriter;
impl RoleDetailWriter {
	fn write_params(params: &mut Params, name: &str, obj: &RoleDetail) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "AssumeRolePolicyDocument"), &obj.assume_role_policy_document);
		idTypeWriter::write_params(params, &(prefix.to_string() + "RoleId"), &obj.role_id);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		instanceProfileListTypeWriter::write_params(params, &(prefix.to_string() + "InstanceProfile"), &obj.instance_profile_list);
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), &obj.path);
		attachedPoliciesListTypeWriter::write_params(params, &(prefix.to_string() + "AttachedPolicy"), &obj.attached_managed_policies);
		policyDetailListTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDetail"), &obj.role_policy_list);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
	}
}
#[derive(Debug, Default)]
pub struct CreatePolicyRequest {
	/// The name of the policy document.
	pub policy_name: policyNameType,
	/// The policy document.
	pub policy_document: policyDocumentType,
	/// A friendly description of the policy.
	/// Typically used to store information about the permissions defined in the
	/// policy. For example, "Grants access to production DynamoDB tables."
	/// The policy description is immutable. After a value is assigned, it cannot be
	/// changed.
	pub description: Option<policyDescriptionType>,
	/// The path for the policy.
	/// For more information about paths, see [IAM Identifiers](http://docs.aws.amazon
	/// .com/IAM/latest/UserGuide/Using_Identifiers.html) in the _Using IAM_ guide.
	/// This parameter is optional. If it is not included, it defaults to a slash (/).
	pub path: Option<policyPathType>,
}

/// Parse CreatePolicyRequest from XML
struct CreatePolicyRequestParser;
impl CreatePolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreatePolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreatePolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "PolicyDocument" {
				obj.policy_document = try!(policyDocumentTypeParser::parse_xml("PolicyDocument", stack));
				continue;
			}
			if current_name == "Description" {
				obj.description = Some(try!(policyDescriptionTypeParser::parse_xml("Description", stack)));
				continue;
			}
			if current_name == "Path" {
				obj.path = Some(try!(policyPathTypeParser::parse_xml("Path", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreatePolicyRequest contents to a SignedRequest
struct CreatePolicyRequestWriter;
impl CreatePolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreatePolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDocument"), &obj.policy_document);
		if let Some(ref obj) = obj.description {
			policyDescriptionTypeWriter::write_params(params, &(prefix.to_string() + "Description"), obj);
		}
		if let Some(ref obj) = obj.path {
			policyPathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), obj);
		}
	}
}
pub type accessKeySecretType = String;
/// Parse accessKeySecretType from XML
struct accessKeySecretTypeParser;
impl accessKeySecretTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<accessKeySecretType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write accessKeySecretType contents to a SignedRequest
struct accessKeySecretTypeWriter;
impl accessKeySecretTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &accessKeySecretType) {
		params.put(name, obj);
	}
}
/// Contains information about an AWS access key, without its secret key.
/// This data type is used as a response element in the ListAccessKeys action.
#[derive(Debug, Default)]
pub struct AccessKeyMetadata {
	/// The name of the IAM user that the key is associated with.
	pub user_name: userNameType,
	/// The status of the access key. `Active` means the key is valid for API calls;
	/// `Inactive` means it is not.
	pub status: statusType,
	/// The date when the access key was created.
	pub create_date: dateType,
	/// The ID for this access key.
	pub access_key_id: accessKeyIdType,
}

/// Parse AccessKeyMetadata from XML
struct AccessKeyMetadataParser;
impl AccessKeyMetadataParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AccessKeyMetadata, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AccessKeyMetadata::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Status" {
				obj.status = try!(statusTypeParser::parse_xml("Status", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "AccessKeyId" {
				obj.access_key_id = try!(accessKeyIdTypeParser::parse_xml("AccessKeyId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AccessKeyMetadata contents to a SignedRequest
struct AccessKeyMetadataWriter;
impl AccessKeyMetadataWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AccessKeyMetadata) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		statusTypeWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		accessKeyIdTypeWriter::write_params(params, &(prefix.to_string() + "AccessKeyId"), &obj.access_key_id);
	}
}
pub type publicKeyMaterialType = String;
/// Parse publicKeyMaterialType from XML
struct publicKeyMaterialTypeParser;
impl publicKeyMaterialTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<publicKeyMaterialType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write publicKeyMaterialType contents to a SignedRequest
struct publicKeyMaterialTypeWriter;
impl publicKeyMaterialTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &publicKeyMaterialType) {
		params.put(name, obj);
	}
}
pub type passwordType = String;
/// Parse passwordType from XML
struct passwordTypeParser;
impl passwordTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<passwordType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write passwordType contents to a SignedRequest
struct passwordTypeWriter;
impl passwordTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &passwordType) {
		params.put(name, obj);
	}
}
/// The request was rejected because it referenced an entity that is temporarily
/// unmodifiable, such as a user name that was deleted and then recreated. The
/// error indicates that the request is likely to succeed if you try again after
/// waiting several minutes. The error message describes the entity.
#[derive(Debug, Default)]
pub struct EntityTemporarilyUnmodifiableException {
	pub message: entityTemporarilyUnmodifiableMessage,
}

/// Parse EntityTemporarilyUnmodifiableException from XML
struct EntityTemporarilyUnmodifiableExceptionParser;
impl EntityTemporarilyUnmodifiableExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EntityTemporarilyUnmodifiableException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EntityTemporarilyUnmodifiableException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(entityTemporarilyUnmodifiableMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EntityTemporarilyUnmodifiableException contents to a SignedRequest
struct EntityTemporarilyUnmodifiableExceptionWriter;
impl EntityTemporarilyUnmodifiableExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EntityTemporarilyUnmodifiableException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		entityTemporarilyUnmodifiableMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
#[derive(Debug, Default)]
pub struct DeletePolicyVersionRequest {
	/// The policy version to delete.
	/// For more information about managed policy versions, see [Versioning for
	/// Managed Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-versions.html) in the _Using IAM_ guide.
	pub version_id: policyVersionIdType,
	pub policy_arn: arnType,
}

/// Parse DeletePolicyVersionRequest from XML
struct DeletePolicyVersionRequestParser;
impl DeletePolicyVersionRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeletePolicyVersionRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeletePolicyVersionRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "VersionId" {
				obj.version_id = try!(policyVersionIdTypeParser::parse_xml("VersionId", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeletePolicyVersionRequest contents to a SignedRequest
struct DeletePolicyVersionRequestWriter;
impl DeletePolicyVersionRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeletePolicyVersionRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		policyVersionIdTypeWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
pub type invalidAuthenticationCodeMessage = String;
/// Parse invalidAuthenticationCodeMessage from XML
struct invalidAuthenticationCodeMessageParser;
impl invalidAuthenticationCodeMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<invalidAuthenticationCodeMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write invalidAuthenticationCodeMessage contents to a SignedRequest
struct invalidAuthenticationCodeMessageWriter;
impl invalidAuthenticationCodeMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &invalidAuthenticationCodeMessage) {
		params.put(name, obj);
	}
}
/// The request was rejected because the certificate was malformed or expired. The
/// error message describes the specific error.
#[derive(Debug, Default)]
pub struct MalformedCertificateException {
	pub message: malformedCertificateMessage,
}

/// Parse MalformedCertificateException from XML
struct MalformedCertificateExceptionParser;
impl MalformedCertificateExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MalformedCertificateException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = MalformedCertificateException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(malformedCertificateMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MalformedCertificateException contents to a SignedRequest
struct MalformedCertificateExceptionWriter;
impl MalformedCertificateExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MalformedCertificateException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		malformedCertificateMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
#[derive(Debug, Default)]
pub struct UpdateGroupRequest {
	/// New path for the group. Only include this if changing the group's path.
	pub new_path: Option<pathType>,
	/// Name of the group to update. If you're changing the name of the group, this is
	/// the original name.
	pub group_name: groupNameType,
	/// New name for the group. Only include this if changing the group's name.
	pub new_group_name: Option<groupNameType>,
}

/// Parse UpdateGroupRequest from XML
struct UpdateGroupRequestParser;
impl UpdateGroupRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateGroupRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateGroupRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NewPath" {
				obj.new_path = Some(try!(pathTypeParser::parse_xml("NewPath", stack)));
				continue;
			}
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "NewGroupName" {
				obj.new_group_name = Some(try!(groupNameTypeParser::parse_xml("NewGroupName", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateGroupRequest contents to a SignedRequest
struct UpdateGroupRequestWriter;
impl UpdateGroupRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateGroupRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.new_path {
			pathTypeWriter::write_params(params, &(prefix.to_string() + "NewPath"), obj);
		}
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		if let Some(ref obj) = obj.new_group_name {
			groupNameTypeWriter::write_params(params, &(prefix.to_string() + "NewGroupName"), obj);
		}
	}
}
/// The Amazon Resource Name (ARN). ARNs are unique identifiers for AWS resources.
/// For more information about ARNs, go to [Amazon Resource Names (ARNs) and AWS
/// Service Namespaces](http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-
/// namespaces.html) in the _AWS General Reference_.
pub type arnType = String;
/// Parse arnType from XML
struct arnTypeParser;
impl arnTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<arnType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write arnType contents to a SignedRequest
struct arnTypeWriter;
impl arnTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &arnType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct GetLoginProfileRequest {
	/// The name of the user whose login profile you want to retrieve.
	pub user_name: userNameType,
}

/// Parse GetLoginProfileRequest from XML
struct GetLoginProfileRequestParser;
impl GetLoginProfileRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetLoginProfileRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetLoginProfileRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetLoginProfileRequest contents to a SignedRequest
struct GetLoginProfileRequestWriter;
impl GetLoginProfileRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetLoginProfileRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
	}
}
#[derive(Debug, Default)]
pub struct DeleteAccountAliasRequest {
	/// The name of the account alias to delete.
	pub account_alias: accountAliasType,
}

/// Parse DeleteAccountAliasRequest from XML
struct DeleteAccountAliasRequestParser;
impl DeleteAccountAliasRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteAccountAliasRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteAccountAliasRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AccountAlias" {
				obj.account_alias = try!(accountAliasTypeParser::parse_xml("AccountAlias", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteAccountAliasRequest contents to a SignedRequest
struct DeleteAccountAliasRequestWriter;
impl DeleteAccountAliasRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteAccountAliasRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		accountAliasTypeWriter::write_params(params, &(prefix.to_string() + "AccountAlias"), &obj.account_alias);
	}
}
pub type roleDetailListType = Vec<RoleDetail>;
/// Parse roleDetailListType from XML
struct roleDetailListTypeParser;
impl roleDetailListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<roleDetailListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "RoleDetail" {
			obj.push(try!(RoleDetailParser::parse_xml("RoleDetail", stack)));
		}
		Ok(obj)
	}
}
/// Write roleDetailListType contents to a SignedRequest
struct roleDetailListTypeWriter;
impl roleDetailListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &roleDetailListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			RoleDetailWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Contains the response to a successful ListInstanceProfiles request.
#[derive(Debug, Default)]
pub struct ListInstanceProfilesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
	/// A list of instance profiles.
	pub instance_profiles: instanceProfileListType,
}

/// Parse ListInstanceProfilesResponse from XML
struct ListInstanceProfilesResponseParser;
impl ListInstanceProfilesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListInstanceProfilesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListInstanceProfilesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			if current_name == "InstanceProfile" {
				obj.instance_profiles = try!(instanceProfileListTypeParser::parse_xml("InstanceProfile", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListInstanceProfilesResponse contents to a SignedRequest
struct ListInstanceProfilesResponseWriter;
impl ListInstanceProfilesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListInstanceProfilesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
		instanceProfileListTypeWriter::write_params(params, &(prefix.to_string() + "InstanceProfile"), &obj.instance_profiles);
	}
}
/// The request was rejected because the most recent credential report has
/// expired. To generate a new credential report, use GenerateCredentialReport.
/// For more information about credential report expiration, see [Getting
/// Credential Reports](http://docs.aws.amazon.com/IAM/latest/UserGuide
/// /credential-reports.html) in the _Using IAM_ guide.
#[derive(Debug, Default)]
pub struct CredentialReportExpiredException {
	pub message: credentialReportExpiredExceptionMessage,
}

/// Parse CredentialReportExpiredException from XML
struct CredentialReportExpiredExceptionParser;
impl CredentialReportExpiredExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CredentialReportExpiredException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CredentialReportExpiredException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(credentialReportExpiredExceptionMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CredentialReportExpiredException contents to a SignedRequest
struct CredentialReportExpiredExceptionWriter;
impl CredentialReportExpiredExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CredentialReportExpiredException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		credentialReportExpiredExceptionMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type privateKeyType = String;
/// Parse privateKeyType from XML
struct privateKeyTypeParser;
impl privateKeyTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<privateKeyType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write privateKeyType contents to a SignedRequest
struct privateKeyTypeWriter;
impl privateKeyTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &privateKeyType) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful ListVirtualMFADevices request.
#[derive(Debug, Default)]
pub struct ListVirtualMFADevicesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
	/// The list of virtual MFA devices in the current account that match the
	/// `AssignmentStatus` value that was passed in the request.
	pub virtual_mfa_devices: virtualMFADeviceListType,
}

/// Parse ListVirtualMFADevicesResponse from XML
struct ListVirtualMFADevicesResponseParser;
impl ListVirtualMFADevicesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListVirtualMFADevicesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListVirtualMFADevicesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			if current_name == "VirtualMFADevice" {
				obj.virtual_mfa_devices = try!(virtualMFADeviceListTypeParser::parse_xml("VirtualMFADevice", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListVirtualMFADevicesResponse contents to a SignedRequest
struct ListVirtualMFADevicesResponseWriter;
impl ListVirtualMFADevicesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListVirtualMFADevicesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
		virtualMFADeviceListTypeWriter::write_params(params, &(prefix.to_string() + "VirtualMFADevice"), &obj.virtual_mfa_devices);
	}
}
/// Contains the response to a successful CreateRole request.
#[derive(Debug, Default)]
pub struct CreateRoleResponse {
	/// Information about the role.
	pub role: Role,
}

/// Parse CreateRoleResponse from XML
struct CreateRoleResponseParser;
impl CreateRoleResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateRoleResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateRoleResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Role" {
				obj.role = try!(RoleParser::parse_xml("Role", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateRoleResponse contents to a SignedRequest
struct CreateRoleResponseWriter;
impl CreateRoleResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateRoleResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		RoleWriter::write_params(params, &(prefix.to_string() + "Role"), &obj.role);
	}
}
#[derive(Debug, Default)]
pub struct UpdateServerCertificateRequest {
	/// The new path for the server certificate. Include this only if you are updating
	/// the server certificate's path.
	pub new_path: Option<pathType>,
	/// The new name for the server certificate. Include this only if you are updating
	/// the server certificate's name. The name of the certificate cannot contain any
	/// spaces.
	pub new_server_certificate_name: Option<serverCertificateNameType>,
	/// The name of the server certificate that you want to update.
	pub server_certificate_name: serverCertificateNameType,
}

/// Parse UpdateServerCertificateRequest from XML
struct UpdateServerCertificateRequestParser;
impl UpdateServerCertificateRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateServerCertificateRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateServerCertificateRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NewPath" {
				obj.new_path = Some(try!(pathTypeParser::parse_xml("NewPath", stack)));
				continue;
			}
			if current_name == "NewServerCertificateName" {
				obj.new_server_certificate_name = Some(try!(serverCertificateNameTypeParser::parse_xml("NewServerCertificateName", stack)));
				continue;
			}
			if current_name == "ServerCertificateName" {
				obj.server_certificate_name = try!(serverCertificateNameTypeParser::parse_xml("ServerCertificateName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateServerCertificateRequest contents to a SignedRequest
struct UpdateServerCertificateRequestWriter;
impl UpdateServerCertificateRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateServerCertificateRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.new_path {
			pathTypeWriter::write_params(params, &(prefix.to_string() + "NewPath"), obj);
		}
		if let Some(ref obj) = obj.new_server_certificate_name {
			serverCertificateNameTypeWriter::write_params(params, &(prefix.to_string() + "NewServerCertificateName"), obj);
		}
		serverCertificateNameTypeWriter::write_params(params, &(prefix.to_string() + "ServerCertificateName"), &obj.server_certificate_name);
	}
}
/// Contains the response to a successful GetSAMLProvider request.
#[derive(Debug, Default)]
pub struct GetSAMLProviderResponse {
	/// The date and time when the SAML provider was created.
	pub create_date: dateType,
	/// The XML metadata document that includes information about an identity
	/// provider.
	pub saml_metadata_document: SAMLMetadataDocumentType,
	/// The expiration date and time for the SAML provider.
	pub valid_until: dateType,
}

/// Parse GetSAMLProviderResponse from XML
struct GetSAMLProviderResponseParser;
impl GetSAMLProviderResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetSAMLProviderResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetSAMLProviderResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "SAMLMetadataDocument" {
				obj.saml_metadata_document = try!(SAMLMetadataDocumentTypeParser::parse_xml("SAMLMetadataDocument", stack));
				continue;
			}
			if current_name == "ValidUntil" {
				obj.valid_until = try!(dateTypeParser::parse_xml("ValidUntil", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetSAMLProviderResponse contents to a SignedRequest
struct GetSAMLProviderResponseWriter;
impl GetSAMLProviderResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetSAMLProviderResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		SAMLMetadataDocumentTypeWriter::write_params(params, &(prefix.to_string() + "SAMLMetadataDocument"), &obj.saml_metadata_document);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "ValidUntil"), &obj.valid_until);
	}
}
pub type certificateChainType = String;
/// Parse certificateChainType from XML
struct certificateChainTypeParser;
impl certificateChainTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<certificateChainType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write certificateChainType contents to a SignedRequest
struct certificateChainTypeWriter;
impl certificateChainTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &certificateChainType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct ListGroupsRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// The path prefix for filtering the results. For example, the prefix
	/// `/division_abc/subdivision_xyz/` gets all groups whose path starts with
	/// `/division_abc/subdivision_xyz/`.
	/// This parameter is optional. If it is not included, it defaults to a slash (/),
	/// listing all groups.
	pub path_prefix: pathPrefixType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListGroupsRequest from XML
struct ListGroupsRequestParser;
impl ListGroupsRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListGroupsRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListGroupsRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "PathPrefix" {
				obj.path_prefix = try!(pathPrefixTypeParser::parse_xml("PathPrefix", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListGroupsRequest contents to a SignedRequest
struct ListGroupsRequestWriter;
impl ListGroupsRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListGroupsRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		pathPrefixTypeWriter::write_params(params, &(prefix.to_string() + "PathPrefix"), &obj.path_prefix);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
#[derive(Debug, Default)]
pub struct ListOpenIDConnectProvidersRequest;

/// Parse ListOpenIDConnectProvidersRequest from XML
struct ListOpenIDConnectProvidersRequestParser;
impl ListOpenIDConnectProvidersRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListOpenIDConnectProvidersRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListOpenIDConnectProvidersRequest::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListOpenIDConnectProvidersRequest contents to a SignedRequest
struct ListOpenIDConnectProvidersRequestWriter;
impl ListOpenIDConnectProvidersRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListOpenIDConnectProvidersRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type assignmentStatusType = String;
/// Parse assignmentStatusType from XML
struct assignmentStatusTypeParser;
impl assignmentStatusTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<assignmentStatusType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write assignmentStatusType contents to a SignedRequest
struct assignmentStatusTypeWriter;
impl assignmentStatusTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &assignmentStatusType) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful ListPolicyVersions request.
#[derive(Debug, Default)]
pub struct ListPolicyVersionsResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: markerType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: booleanType,
	/// A list of policy versions.
	/// For more information about managed policy versions, see [Versioning for
	/// Managed Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-versions.html) in the _Using IAM_ guide.
	pub versions: policyDocumentVersionListType,
}

/// Parse ListPolicyVersionsResponse from XML
struct ListPolicyVersionsResponseParser;
impl ListPolicyVersionsResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListPolicyVersionsResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListPolicyVersionsResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = try!(booleanTypeParser::parse_xml("IsTruncated", stack));
				continue;
			}
			if current_name == "PolicyVersion" {
				obj.versions = try!(policyDocumentVersionListTypeParser::parse_xml("PolicyVersion", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListPolicyVersionsResponse contents to a SignedRequest
struct ListPolicyVersionsResponseWriter;
impl ListPolicyVersionsResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListPolicyVersionsResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
		policyDocumentVersionListTypeWriter::write_params(params, &(prefix.to_string() + "PolicyVersion"), &obj.versions);
	}
}
#[derive(Debug, Default)]
pub struct ListAccountAliasesRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListAccountAliasesRequest from XML
struct ListAccountAliasesRequestParser;
impl ListAccountAliasesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListAccountAliasesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListAccountAliasesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListAccountAliasesRequest contents to a SignedRequest
struct ListAccountAliasesRequestWriter;
impl ListAccountAliasesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListAccountAliasesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
/// Contains information about a managed policy.
/// This data type is used as a response element in the CreatePolicy, GetPolicy,
/// and ListPolicies actions.
/// For more information about managed policies, refer to [Managed Policies and
/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
/// managed-vs-inline.html) in the _Using IAM_ guide.
#[derive(Debug, Default)]
pub struct Policy {
	/// The friendly name (not ARN) identifying the policy.
	pub policy_name: policyNameType,
	/// A friendly description of the policy.
	/// This element is included in the response to the GetPolicy operation. It is not
	/// included in the response to the ListPolicies operation.
	pub description: policyDescriptionType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the policy was created.
	pub create_date: dateType,
	/// The number of entities (users, groups, and roles) that the policy is attached
	/// to.
	pub attachment_count: attachmentCountType,
	/// Specifies whether the policy can be attached to an IAM user, group, or role.
	pub is_attachable: booleanType,
	/// The stable and unique string identifying the policy.
	/// For more information about IDs, see [IAM Identifiers](http://docs.aws.amazon.c
	/// om/IAM/latest/UserGuide/Using_Identifiers.html) in the _Using IAM_ guide.
	pub policy_id: idType,
	/// The identifier for the version of the policy that is set as the default
	/// version.
	pub default_version_id: policyVersionIdType,
	/// The path to the policy.
	/// For more information about paths, see [IAM Identifiers](http://docs.aws.amazon
	/// .com/IAM/latest/UserGuide/Using_Identifiers.html) in the _Using IAM_ guide.
	pub path: policyPathType,
	pub arn: arnType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the policy was last updated.
	/// When a policy has only one version, this field contains the date and time when
	/// the policy was created. When a policy has more than one version, this field
	/// contains the date and time when the most recent policy version was created.
	pub update_date: dateType,
}

/// Parse Policy from XML
struct PolicyParser;
impl PolicyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Policy, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Policy::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "Description" {
				obj.description = try!(policyDescriptionTypeParser::parse_xml("Description", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = try!(dateTypeParser::parse_xml("CreateDate", stack));
				continue;
			}
			if current_name == "AttachmentCount" {
				obj.attachment_count = try!(attachmentCountTypeParser::parse_xml("AttachmentCount", stack));
				continue;
			}
			if current_name == "IsAttachable" {
				obj.is_attachable = try!(booleanTypeParser::parse_xml("IsAttachable", stack));
				continue;
			}
			if current_name == "PolicyId" {
				obj.policy_id = try!(idTypeParser::parse_xml("PolicyId", stack));
				continue;
			}
			if current_name == "DefaultVersionId" {
				obj.default_version_id = try!(policyVersionIdTypeParser::parse_xml("DefaultVersionId", stack));
				continue;
			}
			if current_name == "Path" {
				obj.path = try!(policyPathTypeParser::parse_xml("Path", stack));
				continue;
			}
			if current_name == "Arn" {
				obj.arn = try!(arnTypeParser::parse_xml("Arn", stack));
				continue;
			}
			if current_name == "UpdateDate" {
				obj.update_date = try!(dateTypeParser::parse_xml("UpdateDate", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Policy contents to a SignedRequest
struct PolicyWriter;
impl PolicyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Policy) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		policyDescriptionTypeWriter::write_params(params, &(prefix.to_string() + "Description"), &obj.description);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), &obj.create_date);
		attachmentCountTypeWriter::write_params(params, &(prefix.to_string() + "AttachmentCount"), &obj.attachment_count);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsAttachable"), &obj.is_attachable);
		idTypeWriter::write_params(params, &(prefix.to_string() + "PolicyId"), &obj.policy_id);
		policyVersionIdTypeWriter::write_params(params, &(prefix.to_string() + "DefaultVersionId"), &obj.default_version_id);
		policyPathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), &obj.path);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "Arn"), &obj.arn);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "UpdateDate"), &obj.update_date);
	}
}
#[derive(Debug, Default)]
pub struct GetPolicyRequest {
	pub policy_arn: arnType,
}

/// Parse GetPolicyRequest from XML
struct GetPolicyRequestParser;
impl GetPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetPolicyRequest contents to a SignedRequest
struct GetPolicyRequestWriter;
impl GetPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
/// Contains the response to a successful GetCredentialReport request.
#[derive(Debug, Default)]
pub struct GetCredentialReportResponse {
	/// Contains the credential report. The report is Base64-encoded.
	pub content: ReportContentType,
	/// The date and time when the credential report was created, in [ISO 8601 date-
	/// time format](http://www.iso.org/iso/iso8601).
	pub generated_time: dateType,
	/// The format (MIME type) of the credential report.
	pub report_format: ReportFormatType,
}

/// Parse GetCredentialReportResponse from XML
struct GetCredentialReportResponseParser;
impl GetCredentialReportResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetCredentialReportResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetCredentialReportResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Content" {
				obj.content = try!(ReportContentTypeParser::parse_xml("Content", stack));
				continue;
			}
			if current_name == "GeneratedTime" {
				obj.generated_time = try!(dateTypeParser::parse_xml("GeneratedTime", stack));
				continue;
			}
			if current_name == "ReportFormat" {
				obj.report_format = try!(ReportFormatTypeParser::parse_xml("ReportFormat", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetCredentialReportResponse contents to a SignedRequest
struct GetCredentialReportResponseWriter;
impl GetCredentialReportResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetCredentialReportResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ReportContentTypeWriter::write_params(params, &(prefix.to_string() + "Content"), &obj.content);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "GeneratedTime"), &obj.generated_time);
		ReportFormatTypeWriter::write_params(params, &(prefix.to_string() + "ReportFormat"), &obj.report_format);
	}
}
pub type pathPrefixType = String;
/// Parse pathPrefixType from XML
struct pathPrefixTypeParser;
impl pathPrefixTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<pathPrefixType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write pathPrefixType contents to a SignedRequest
struct pathPrefixTypeWriter;
impl pathPrefixTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &pathPrefixType) {
		params.put(name, obj);
	}
}
pub type policyDetailListType = Vec<PolicyDetail>;
/// Parse policyDetailListType from XML
struct policyDetailListTypeParser;
impl policyDetailListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<policyDetailListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "PolicyDetail" {
			obj.push(try!(PolicyDetailParser::parse_xml("PolicyDetail", stack)));
		}
		Ok(obj)
	}
}
/// Write policyDetailListType contents to a SignedRequest
struct policyDetailListTypeWriter;
impl policyDetailListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &policyDetailListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			PolicyDetailWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct AddUserToGroupRequest {
	/// The name of the user to add.
	pub user_name: existingUserNameType,
	/// The name of the group to update.
	pub group_name: groupNameType,
}

/// Parse AddUserToGroupRequest from XML
struct AddUserToGroupRequestParser;
impl AddUserToGroupRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AddUserToGroupRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AddUserToGroupRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AddUserToGroupRequest contents to a SignedRequest
struct AddUserToGroupRequestWriter;
impl AddUserToGroupRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AddUserToGroupRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
	}
}
/// Contains a URL that specifies the endpoint for an OpenID Connect provider.
pub type OpenIDConnectProviderUrlType = String;
/// Parse OpenIDConnectProviderUrlType from XML
struct OpenIDConnectProviderUrlTypeParser;
impl OpenIDConnectProviderUrlTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<OpenIDConnectProviderUrlType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write OpenIDConnectProviderUrlType contents to a SignedRequest
struct OpenIDConnectProviderUrlTypeWriter;
impl OpenIDConnectProviderUrlTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &OpenIDConnectProviderUrlType) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful UploadServerCertificate request.
#[derive(Debug, Default)]
pub struct UploadServerCertificateResponse {
	/// The meta information of the uploaded server certificate without its
	/// certificate body, certificate chain, and private key.
	pub server_certificate_metadata: ServerCertificateMetadata,
}

/// Parse UploadServerCertificateResponse from XML
struct UploadServerCertificateResponseParser;
impl UploadServerCertificateResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UploadServerCertificateResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UploadServerCertificateResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ServerCertificateMetadata" {
				obj.server_certificate_metadata = try!(ServerCertificateMetadataParser::parse_xml("ServerCertificateMetadata", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UploadServerCertificateResponse contents to a SignedRequest
struct UploadServerCertificateResponseWriter;
impl UploadServerCertificateResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UploadServerCertificateResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ServerCertificateMetadataWriter::write_params(params, &(prefix.to_string() + "ServerCertificateMetadata"), &obj.server_certificate_metadata);
	}
}
pub type groupDetailListType = Vec<GroupDetail>;
/// Parse groupDetailListType from XML
struct groupDetailListTypeParser;
impl groupDetailListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<groupDetailListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "GroupDetail" {
			obj.push(try!(GroupDetailParser::parse_xml("GroupDetail", stack)));
		}
		Ok(obj)
	}
}
/// Write groupDetailListType contents to a SignedRequest
struct groupDetailListTypeWriter;
impl groupDetailListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &groupDetailListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			GroupDetailWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Contains the response to a successful GetUserPolicy request.
#[derive(Debug, Default)]
pub struct GetUserPolicyResponse {
	/// The user the policy is associated with.
	pub user_name: existingUserNameType,
	/// The name of the policy.
	pub policy_name: policyNameType,
	/// The policy document.
	pub policy_document: policyDocumentType,
}

/// Parse GetUserPolicyResponse from XML
struct GetUserPolicyResponseParser;
impl GetUserPolicyResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetUserPolicyResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetUserPolicyResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(existingUserNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "PolicyDocument" {
				obj.policy_document = try!(policyDocumentTypeParser::parse_xml("PolicyDocument", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetUserPolicyResponse contents to a SignedRequest
struct GetUserPolicyResponseWriter;
impl GetUserPolicyResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetUserPolicyResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDocument"), &obj.policy_document);
	}
}
#[derive(Debug, Default)]
pub struct CreateLoginProfileRequest {
	/// The name of the user to create a password for.
	pub user_name: userNameType,
	/// Specifies whether the user is required to set a new password on next sign-in.
	pub password_reset_required: Option<booleanType>,
	/// The new password for the user.
	pub password: passwordType,
}

/// Parse CreateLoginProfileRequest from XML
struct CreateLoginProfileRequestParser;
impl CreateLoginProfileRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateLoginProfileRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateLoginProfileRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "PasswordResetRequired" {
				obj.password_reset_required = Some(try!(booleanTypeParser::parse_xml("PasswordResetRequired", stack)));
				continue;
			}
			if current_name == "Password" {
				obj.password = try!(passwordTypeParser::parse_xml("Password", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateLoginProfileRequest contents to a SignedRequest
struct CreateLoginProfileRequestWriter;
impl CreateLoginProfileRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateLoginProfileRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		if let Some(ref obj) = obj.password_reset_required {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "PasswordResetRequired"), obj);
		}
		passwordTypeWriter::write_params(params, &(prefix.to_string() + "Password"), &obj.password);
	}
}
#[derive(Debug, Default)]
pub struct AddRoleToInstanceProfileRequest {
	/// The name of the role to add.
	pub role_name: roleNameType,
	/// The name of the instance profile to update.
	pub instance_profile_name: instanceProfileNameType,
}

/// Parse AddRoleToInstanceProfileRequest from XML
struct AddRoleToInstanceProfileRequestParser;
impl AddRoleToInstanceProfileRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AddRoleToInstanceProfileRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AddRoleToInstanceProfileRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "InstanceProfileName" {
				obj.instance_profile_name = try!(instanceProfileNameTypeParser::parse_xml("InstanceProfileName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AddRoleToInstanceProfileRequest contents to a SignedRequest
struct AddRoleToInstanceProfileRequestWriter;
impl AddRoleToInstanceProfileRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AddRoleToInstanceProfileRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		instanceProfileNameTypeWriter::write_params(params, &(prefix.to_string() + "InstanceProfileName"), &obj.instance_profile_name);
	}
}
pub type policyNameType = String;
/// Parse policyNameType from XML
struct policyNameTypeParser;
impl policyNameTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<policyNameType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write policyNameType contents to a SignedRequest
struct policyNameTypeWriter;
impl policyNameTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &policyNameType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct DeleteGroupPolicyRequest {
	/// The name (friendly name, not ARN) identifying the group that the policy is
	/// embedded in.
	pub group_name: groupNameType,
	/// The name identifying the policy document to delete.
	pub policy_name: policyNameType,
}

/// Parse DeleteGroupPolicyRequest from XML
struct DeleteGroupPolicyRequestParser;
impl DeleteGroupPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteGroupPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteGroupPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteGroupPolicyRequest contents to a SignedRequest
struct DeleteGroupPolicyRequestWriter;
impl DeleteGroupPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteGroupPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
	}
}
pub type passwordReusePreventionType = i32;
/// Parse passwordReusePreventionType from XML
struct passwordReusePreventionTypeParser;
impl passwordReusePreventionTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<passwordReusePreventionType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write passwordReusePreventionType contents to a SignedRequest
struct passwordReusePreventionTypeWriter;
impl passwordReusePreventionTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &passwordReusePreventionType) {
		params.put(name, &obj.to_string());
	}
}
pub type EntityType = String;
/// Parse EntityType from XML
struct EntityTypeParser;
impl EntityTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EntityType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EntityType contents to a SignedRequest
struct EntityTypeWriter;
impl EntityTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EntityType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct ListAttachedRolePoliciesRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// The name (friendly name, not ARN) of the role to list attached policies for.
	pub role_name: roleNameType,
	/// The path prefix for filtering the results. This parameter is optional. If it
	/// is not included, it defaults to a slash (/), listing all policies.
	pub path_prefix: Option<policyPathType>,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
}

/// Parse ListAttachedRolePoliciesRequest from XML
struct ListAttachedRolePoliciesRequestParser;
impl ListAttachedRolePoliciesRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListAttachedRolePoliciesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListAttachedRolePoliciesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "PathPrefix" {
				obj.path_prefix = Some(try!(policyPathTypeParser::parse_xml("PathPrefix", stack)));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListAttachedRolePoliciesRequest contents to a SignedRequest
struct ListAttachedRolePoliciesRequestWriter;
impl ListAttachedRolePoliciesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListAttachedRolePoliciesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		if let Some(ref obj) = obj.path_prefix {
			policyPathTypeWriter::write_params(params, &(prefix.to_string() + "PathPrefix"), obj);
		}
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
	}
}
/// Contains the response to a successful ListRoles request.
#[derive(Debug, Default)]
pub struct ListRolesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
	/// A list of roles.
	pub roles: roleListType,
}

/// Parse ListRolesResponse from XML
struct ListRolesResponseParser;
impl ListRolesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListRolesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListRolesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			if current_name == "Role" {
				obj.roles = try!(roleListTypeParser::parse_xml("Role", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListRolesResponse contents to a SignedRequest
struct ListRolesResponseWriter;
impl ListRolesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListRolesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
		roleListTypeWriter::write_params(params, &(prefix.to_string() + "Role"), &obj.roles);
	}
}
#[derive(Debug, Default)]
pub struct GetSAMLProviderRequest {
	/// The Amazon Resource Name (ARN) of the SAML provider to get information about.
	pub saml_provider_arn: arnType,
}

/// Parse GetSAMLProviderRequest from XML
struct GetSAMLProviderRequestParser;
impl GetSAMLProviderRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetSAMLProviderRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetSAMLProviderRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SAMLProviderArn" {
				obj.saml_provider_arn = try!(arnTypeParser::parse_xml("SAMLProviderArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetSAMLProviderRequest contents to a SignedRequest
struct GetSAMLProviderRequestWriter;
impl GetSAMLProviderRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetSAMLProviderRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "SAMLProviderArn"), &obj.saml_provider_arn);
	}
}
#[derive(Debug, Default)]
pub struct DeleteInstanceProfileRequest {
	/// The name of the instance profile to delete.
	pub instance_profile_name: instanceProfileNameType,
}

/// Parse DeleteInstanceProfileRequest from XML
struct DeleteInstanceProfileRequestParser;
impl DeleteInstanceProfileRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteInstanceProfileRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteInstanceProfileRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "InstanceProfileName" {
				obj.instance_profile_name = try!(instanceProfileNameTypeParser::parse_xml("InstanceProfileName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteInstanceProfileRequest contents to a SignedRequest
struct DeleteInstanceProfileRequestWriter;
impl DeleteInstanceProfileRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteInstanceProfileRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		instanceProfileNameTypeWriter::write_params(params, &(prefix.to_string() + "InstanceProfileName"), &obj.instance_profile_name);
	}
}
/// The request was rejected because the type of user for the transaction was
/// incorrect.
#[derive(Debug, Default)]
pub struct InvalidUserTypeException {
	pub message: invalidUserTypeMessage,
}

/// Parse InvalidUserTypeException from XML
struct InvalidUserTypeExceptionParser;
impl InvalidUserTypeExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidUserTypeException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidUserTypeException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(invalidUserTypeMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidUserTypeException contents to a SignedRequest
struct InvalidUserTypeExceptionWriter;
impl InvalidUserTypeExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidUserTypeException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		invalidUserTypeMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains the response to a successful GetRole request.
#[derive(Debug, Default)]
pub struct GetRoleResponse {
	/// Information about the role.
	pub role: Role,
}

/// Parse GetRoleResponse from XML
struct GetRoleResponseParser;
impl GetRoleResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetRoleResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetRoleResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Role" {
				obj.role = try!(RoleParser::parse_xml("Role", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetRoleResponse contents to a SignedRequest
struct GetRoleResponseWriter;
impl GetRoleResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetRoleResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		RoleWriter::write_params(params, &(prefix.to_string() + "Role"), &obj.role);
	}
}
#[derive(Debug, Default)]
pub struct CreateRoleRequest {
	/// The path to the role. For more information about paths, see [IAM Identifiers](
	/// http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the
	/// _Using IAM_ guide.
	/// This parameter is optional. If it is not included, it defaults to a slash (/).
	pub path: Option<pathType>,
	/// The policy that grants an entity permission to assume the role.
	pub assume_role_policy_document: policyDocumentType,
	/// The name of the role to create.
	pub role_name: roleNameType,
}

/// Parse CreateRoleRequest from XML
struct CreateRoleRequestParser;
impl CreateRoleRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateRoleRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateRoleRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Path" {
				obj.path = Some(try!(pathTypeParser::parse_xml("Path", stack)));
				continue;
			}
			if current_name == "AssumeRolePolicyDocument" {
				obj.assume_role_policy_document = try!(policyDocumentTypeParser::parse_xml("AssumeRolePolicyDocument", stack));
				continue;
			}
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateRoleRequest contents to a SignedRequest
struct CreateRoleRequestWriter;
impl CreateRoleRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateRoleRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.path {
			pathTypeWriter::write_params(params, &(prefix.to_string() + "Path"), obj);
		}
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "AssumeRolePolicyDocument"), &obj.assume_role_policy_document);
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
	}
}
#[derive(Debug, Default)]
pub struct PutGroupPolicyRequest {
	/// The name of the group to associate the policy with.
	pub group_name: groupNameType,
	/// The policy document.
	pub policy_document: policyDocumentType,
	/// The name of the policy document.
	pub policy_name: policyNameType,
}

/// Parse PutGroupPolicyRequest from XML
struct PutGroupPolicyRequestParser;
impl PutGroupPolicyRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PutGroupPolicyRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PutGroupPolicyRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "GroupName" {
				obj.group_name = try!(groupNameTypeParser::parse_xml("GroupName", stack));
				continue;
			}
			if current_name == "PolicyDocument" {
				obj.policy_document = try!(policyDocumentTypeParser::parse_xml("PolicyDocument", stack));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(policyNameTypeParser::parse_xml("PolicyName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PutGroupPolicyRequest contents to a SignedRequest
struct PutGroupPolicyRequestWriter;
impl PutGroupPolicyRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PutGroupPolicyRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		groupNameTypeWriter::write_params(params, &(prefix.to_string() + "GroupName"), &obj.group_name);
		policyDocumentTypeWriter::write_params(params, &(prefix.to_string() + "PolicyDocument"), &obj.policy_document);
		policyNameTypeWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
	}
}
pub type SAMLProviderListType = Vec<SAMLProviderListEntry>;
/// Parse SAMLProviderListType from XML
struct SAMLProviderListTypeParser;
impl SAMLProviderListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SAMLProviderListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "SAMLProviderListEntry" {
			obj.push(try!(SAMLProviderListEntryParser::parse_xml("SAMLProviderListEntry", stack)));
		}
		Ok(obj)
	}
}
/// Write SAMLProviderListType contents to a SignedRequest
struct SAMLProviderListTypeWriter;
impl SAMLProviderListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SAMLProviderListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			SAMLProviderListEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type encodingType = String;
/// Parse encodingType from XML
struct encodingTypeParser;
impl encodingTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<encodingType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write encodingType contents to a SignedRequest
struct encodingTypeWriter;
impl encodingTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &encodingType) {
		params.put(name, obj);
	}
}
pub type policyListType = Vec<Policy>;
/// Parse policyListType from XML
struct policyListTypeParser;
impl policyListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<policyListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Policy" {
			obj.push(try!(PolicyParser::parse_xml("Policy", stack)));
		}
		Ok(obj)
	}
}
/// Write policyListType contents to a SignedRequest
struct policyListTypeWriter;
impl policyListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &policyListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			PolicyWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Contains a list of users.
/// This data type is used as a response element in the GetGroup and ListUsers
/// actions.
pub type userListType = Vec<User>;
/// Parse userListType from XML
struct userListTypeParser;
impl userListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<userListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "User" {
			obj.push(try!(UserParser::parse_xml("User", stack)));
		}
		Ok(obj)
	}
}
/// Write userListType contents to a SignedRequest
struct userListTypeWriter;
impl userListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &userListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			UserWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type SSHPublicKeyListType = Vec<SSHPublicKeyMetadata>;
/// Parse SSHPublicKeyListType from XML
struct SSHPublicKeyListTypeParser;
impl SSHPublicKeyListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SSHPublicKeyListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "SSHPublicKeyMetadata" {
			obj.push(try!(SSHPublicKeyMetadataParser::parse_xml("SSHPublicKeyMetadata", stack)));
		}
		Ok(obj)
	}
}
/// Write SSHPublicKeyListType contents to a SignedRequest
struct SSHPublicKeyListTypeWriter;
impl SSHPublicKeyListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SSHPublicKeyListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			SSHPublicKeyMetadataWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type attachmentCountType = i32;
/// Parse attachmentCountType from XML
struct attachmentCountTypeParser;
impl attachmentCountTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<attachmentCountType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write attachmentCountType contents to a SignedRequest
struct attachmentCountTypeWriter;
impl attachmentCountTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &attachmentCountType) {
		params.put(name, &obj.to_string());
	}
}
/// Contains the response to a successful CreateVirtualMFADevice request.
#[derive(Debug, Default)]
pub struct CreateVirtualMFADeviceResponse {
	/// A newly created virtual MFA device.
	pub virtual_mfa_device: VirtualMFADevice,
}

/// Parse CreateVirtualMFADeviceResponse from XML
struct CreateVirtualMFADeviceResponseParser;
impl CreateVirtualMFADeviceResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateVirtualMFADeviceResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateVirtualMFADeviceResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "VirtualMFADevice" {
				obj.virtual_mfa_device = try!(VirtualMFADeviceParser::parse_xml("VirtualMFADevice", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateVirtualMFADeviceResponse contents to a SignedRequest
struct CreateVirtualMFADeviceResponseWriter;
impl CreateVirtualMFADeviceResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateVirtualMFADeviceResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		VirtualMFADeviceWriter::write_params(params, &(prefix.to_string() + "VirtualMFADevice"), &obj.virtual_mfa_device);
	}
}
pub type BootstrapDatum = Vec<u8>;
/// Parse BootstrapDatum from XML
struct BootstrapDatumParser;
impl BootstrapDatumParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BootstrapDatum, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack)).into_bytes();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write BootstrapDatum contents to a SignedRequest
struct BootstrapDatumWriter;
impl BootstrapDatumWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BootstrapDatum) {
		params.put(name, str::from_utf8(&obj).unwrap());
	}
}
/// Contains the response to a successful GetPolicy request.
#[derive(Debug, Default)]
pub struct GetPolicyResponse {
	/// Information about the policy.
	pub policy: Policy,
}

/// Parse GetPolicyResponse from XML
struct GetPolicyResponseParser;
impl GetPolicyResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetPolicyResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetPolicyResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Policy" {
				obj.policy = try!(PolicyParser::parse_xml("Policy", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetPolicyResponse contents to a SignedRequest
struct GetPolicyResponseWriter;
impl GetPolicyResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetPolicyResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		PolicyWriter::write_params(params, &(prefix.to_string() + "Policy"), &obj.policy);
	}
}
pub type credentialReportNotPresentExceptionMessage = String;
/// Parse credentialReportNotPresentExceptionMessage from XML
struct credentialReportNotPresentExceptionMessageParser;
impl credentialReportNotPresentExceptionMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<credentialReportNotPresentExceptionMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write credentialReportNotPresentExceptionMessage contents to a SignedRequest
struct credentialReportNotPresentExceptionMessageWriter;
impl credentialReportNotPresentExceptionMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &credentialReportNotPresentExceptionMessage) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct SetDefaultPolicyVersionRequest {
	/// The version of the policy to set as the default (operative) version.
	/// For more information about managed policy versions, see [Versioning for
	/// Managed Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-versions.html) in the _Using IAM_ guide.
	pub version_id: policyVersionIdType,
	pub policy_arn: arnType,
}

/// Parse SetDefaultPolicyVersionRequest from XML
struct SetDefaultPolicyVersionRequestParser;
impl SetDefaultPolicyVersionRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetDefaultPolicyVersionRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SetDefaultPolicyVersionRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "VersionId" {
				obj.version_id = try!(policyVersionIdTypeParser::parse_xml("VersionId", stack));
				continue;
			}
			if current_name == "PolicyArn" {
				obj.policy_arn = try!(arnTypeParser::parse_xml("PolicyArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SetDefaultPolicyVersionRequest contents to a SignedRequest
struct SetDefaultPolicyVersionRequestWriter;
impl SetDefaultPolicyVersionRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SetDefaultPolicyVersionRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		policyVersionIdTypeWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
		arnTypeWriter::write_params(params, &(prefix.to_string() + "PolicyArn"), &obj.policy_arn);
	}
}
pub type PolicyRoleListType = Vec<PolicyRole>;
/// Parse PolicyRoleListType from XML
struct PolicyRoleListTypeParser;
impl PolicyRoleListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyRoleListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "PolicyRole" {
			obj.push(try!(PolicyRoleParser::parse_xml("PolicyRole", stack)));
		}
		Ok(obj)
	}
}
/// Write PolicyRoleListType contents to a SignedRequest
struct PolicyRoleListTypeWriter;
impl PolicyRoleListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyRoleListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			PolicyRoleWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Contains information about an AWS access key.
/// This data type is used as a response element in the CreateAccessKey and
/// ListAccessKeys actions.
/// The `SecretAccessKey` value is returned only in response to CreateAccessKey.
/// You can get a secret access key only when you first create an access key; you
/// cannot recover the secret access key later. If you lose a secret access key,
/// you must create a new access key.
#[derive(Debug, Default)]
pub struct AccessKey {
	/// The name of the IAM user that the access key is associated with.
	pub user_name: userNameType,
	/// The status of the access key. `Active` means the key is valid for API calls,
	/// while `Inactive` means it is not.
	pub status: statusType,
	/// The date when the access key was created.
	pub create_date: Option<dateType>,
	/// The secret key used to sign requests.
	pub secret_access_key: accessKeySecretType,
	/// The ID for this access key.
	pub access_key_id: accessKeyIdType,
}

/// Parse AccessKey from XML
struct AccessKeyParser;
impl AccessKeyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AccessKey, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AccessKey::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Status" {
				obj.status = try!(statusTypeParser::parse_xml("Status", stack));
				continue;
			}
			if current_name == "CreateDate" {
				obj.create_date = Some(try!(dateTypeParser::parse_xml("CreateDate", stack)));
				continue;
			}
			if current_name == "SecretAccessKey" {
				obj.secret_access_key = try!(accessKeySecretTypeParser::parse_xml("SecretAccessKey", stack));
				continue;
			}
			if current_name == "AccessKeyId" {
				obj.access_key_id = try!(accessKeyIdTypeParser::parse_xml("AccessKeyId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AccessKey contents to a SignedRequest
struct AccessKeyWriter;
impl AccessKeyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AccessKey) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		statusTypeWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		if let Some(ref obj) = obj.create_date {
			dateTypeWriter::write_params(params, &(prefix.to_string() + "CreateDate"), obj);
		}
		accessKeySecretTypeWriter::write_params(params, &(prefix.to_string() + "SecretAccessKey"), &obj.secret_access_key);
		accessKeyIdTypeWriter::write_params(params, &(prefix.to_string() + "AccessKeyId"), &obj.access_key_id);
	}
}
pub type ReportFormatType = String;
/// Parse ReportFormatType from XML
struct ReportFormatTypeParser;
impl ReportFormatTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ReportFormatType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ReportFormatType contents to a SignedRequest
struct ReportFormatTypeWriter;
impl ReportFormatTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ReportFormatType) {
		params.put(name, obj);
	}
}
pub type pathType = String;
/// Parse pathType from XML
struct pathTypeParser;
impl pathTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<pathType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write pathType contents to a SignedRequest
struct pathTypeWriter;
impl pathTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &pathType) {
		params.put(name, obj);
	}
}
pub type noSuchEntityMessage = String;
/// Parse noSuchEntityMessage from XML
struct noSuchEntityMessageParser;
impl noSuchEntityMessageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<noSuchEntityMessage, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write noSuchEntityMessage contents to a SignedRequest
struct noSuchEntityMessageWriter;
impl noSuchEntityMessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &noSuchEntityMessage) {
		params.put(name, obj);
	}
}
pub type certificateIdType = String;
/// Parse certificateIdType from XML
struct certificateIdTypeParser;
impl certificateIdTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<certificateIdType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write certificateIdType contents to a SignedRequest
struct certificateIdTypeWriter;
impl certificateIdTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &certificateIdType) {
		params.put(name, obj);
	}
}
pub type ReportStateDescriptionType = String;
/// Parse ReportStateDescriptionType from XML
struct ReportStateDescriptionTypeParser;
impl ReportStateDescriptionTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ReportStateDescriptionType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ReportStateDescriptionType contents to a SignedRequest
struct ReportStateDescriptionTypeWriter;
impl ReportStateDescriptionTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ReportStateDescriptionType) {
		params.put(name, obj);
	}
}
/// Contains the response to a successful CreateLoginProfile request.
#[derive(Debug, Default)]
pub struct CreateLoginProfileResponse {
	/// The user name and password create date.
	pub login_profile: LoginProfile,
}

/// Parse CreateLoginProfileResponse from XML
struct CreateLoginProfileResponseParser;
impl CreateLoginProfileResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateLoginProfileResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateLoginProfileResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "LoginProfile" {
				obj.login_profile = try!(LoginProfileParser::parse_xml("LoginProfile", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateLoginProfileResponse contents to a SignedRequest
struct CreateLoginProfileResponseWriter;
impl CreateLoginProfileResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateLoginProfileResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		LoginProfileWriter::write_params(params, &(prefix.to_string() + "LoginProfile"), &obj.login_profile);
	}
}
#[derive(Debug, Default)]
pub struct DeleteLoginProfileRequest {
	/// The name of the user whose password you want to delete.
	pub user_name: userNameType,
}

/// Parse DeleteLoginProfileRequest from XML
struct DeleteLoginProfileRequestParser;
impl DeleteLoginProfileRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteLoginProfileRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteLoginProfileRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteLoginProfileRequest contents to a SignedRequest
struct DeleteLoginProfileRequestWriter;
impl DeleteLoginProfileRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteLoginProfileRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
	}
}
/// Contains a list of thumbprints of identity provider server certificates.
pub type thumbprintListType = Vec<thumbprintType>;
/// Parse thumbprintListType from XML
struct thumbprintListTypeParser;
impl thumbprintListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<thumbprintListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "thumbprintType" {
			obj.push(try!(thumbprintTypeParser::parse_xml("thumbprintType", stack)));
		}
		Ok(obj)
	}
}
/// Write thumbprintListType contents to a SignedRequest
struct thumbprintListTypeWriter;
impl thumbprintListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &thumbprintListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			thumbprintTypeWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Contains the response to a successful ListInstanceProfilesForRole request.
#[derive(Debug, Default)]
pub struct ListInstanceProfilesForRoleResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
	/// A list of instance profiles.
	pub instance_profiles: instanceProfileListType,
}

/// Parse ListInstanceProfilesForRoleResponse from XML
struct ListInstanceProfilesForRoleResponseParser;
impl ListInstanceProfilesForRoleResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListInstanceProfilesForRoleResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListInstanceProfilesForRoleResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			if current_name == "InstanceProfile" {
				obj.instance_profiles = try!(instanceProfileListTypeParser::parse_xml("InstanceProfile", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListInstanceProfilesForRoleResponse contents to a SignedRequest
struct ListInstanceProfilesForRoleResponseWriter;
impl ListInstanceProfilesForRoleResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListInstanceProfilesForRoleResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
		instanceProfileListTypeWriter::write_params(params, &(prefix.to_string() + "InstanceProfile"), &obj.instance_profiles);
	}
}
#[derive(Debug, Default)]
pub struct ListInstanceProfilesForRoleRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: Option<markerType>,
	/// The name of the role to list instance profiles for.
	pub role_name: roleNameType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: Option<maxItemsType>,
}

/// Parse ListInstanceProfilesForRoleRequest from XML
struct ListInstanceProfilesForRoleRequestParser;
impl ListInstanceProfilesForRoleRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListInstanceProfilesForRoleRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListInstanceProfilesForRoleRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "RoleName" {
				obj.role_name = try!(roleNameTypeParser::parse_xml("RoleName", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = Some(try!(maxItemsTypeParser::parse_xml("MaxItems", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListInstanceProfilesForRoleRequest contents to a SignedRequest
struct ListInstanceProfilesForRoleRequestWriter;
impl ListInstanceProfilesForRoleRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListInstanceProfilesForRoleRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		roleNameTypeWriter::write_params(params, &(prefix.to_string() + "RoleName"), &obj.role_name);
		if let Some(ref obj) = obj.max_items {
			maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), obj);
		}
	}
}
/// Contains the response to a successful ListEntitiesForPolicy request.
#[derive(Debug, Default)]
pub struct ListEntitiesForPolicyResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: markerType,
	/// A list of groups that the policy is attached to.
	pub policy_groups: PolicyGroupListType,
	/// A list of users that the policy is attached to.
	pub policy_users: PolicyUserListType,
	/// A list of roles that the policy is attached to.
	pub policy_roles: PolicyRoleListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: booleanType,
}

/// Parse ListEntitiesForPolicyResponse from XML
struct ListEntitiesForPolicyResponseParser;
impl ListEntitiesForPolicyResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListEntitiesForPolicyResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListEntitiesForPolicyResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "PolicyGroup" {
				obj.policy_groups = try!(PolicyGroupListTypeParser::parse_xml("PolicyGroup", stack));
				continue;
			}
			if current_name == "PolicyUser" {
				obj.policy_users = try!(PolicyUserListTypeParser::parse_xml("PolicyUser", stack));
				continue;
			}
			if current_name == "PolicyRole" {
				obj.policy_roles = try!(PolicyRoleListTypeParser::parse_xml("PolicyRole", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = try!(booleanTypeParser::parse_xml("IsTruncated", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListEntitiesForPolicyResponse contents to a SignedRequest
struct ListEntitiesForPolicyResponseWriter;
impl ListEntitiesForPolicyResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListEntitiesForPolicyResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		PolicyGroupListTypeWriter::write_params(params, &(prefix.to_string() + "PolicyGroup"), &obj.policy_groups);
		PolicyUserListTypeWriter::write_params(params, &(prefix.to_string() + "PolicyUser"), &obj.policy_users);
		PolicyRoleListTypeWriter::write_params(params, &(prefix.to_string() + "PolicyRole"), &obj.policy_roles);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
	}
}
/// Contains the response to a successful GetAccountAuthorizationDetails request.
#[derive(Debug, Default)]
pub struct GetAccountAuthorizationDetailsResponse {
	/// A list containing information about IAM roles.
	pub role_detail_list: roleDetailListType,
	/// A list containing information about IAM groups.
	pub group_detail_list: groupDetailListType,
	/// A list containing information about IAM users.
	pub user_detail_list: userDetailListType,
	/// A list containing information about managed policies.
	pub policies: ManagedPolicyDetailListType,
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: markerType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: booleanType,
}

/// Parse GetAccountAuthorizationDetailsResponse from XML
struct GetAccountAuthorizationDetailsResponseParser;
impl GetAccountAuthorizationDetailsResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetAccountAuthorizationDetailsResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetAccountAuthorizationDetailsResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "RoleDetail" {
				obj.role_detail_list = try!(roleDetailListTypeParser::parse_xml("RoleDetail", stack));
				continue;
			}
			if current_name == "GroupDetail" {
				obj.group_detail_list = try!(groupDetailListTypeParser::parse_xml("GroupDetail", stack));
				continue;
			}
			if current_name == "UserDetail" {
				obj.user_detail_list = try!(userDetailListTypeParser::parse_xml("UserDetail", stack));
				continue;
			}
			if current_name == "ManagedPolicyDetail" {
				obj.policies = try!(ManagedPolicyDetailListTypeParser::parse_xml("ManagedPolicyDetail", stack));
				continue;
			}
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = try!(booleanTypeParser::parse_xml("IsTruncated", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetAccountAuthorizationDetailsResponse contents to a SignedRequest
struct GetAccountAuthorizationDetailsResponseWriter;
impl GetAccountAuthorizationDetailsResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetAccountAuthorizationDetailsResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		roleDetailListTypeWriter::write_params(params, &(prefix.to_string() + "RoleDetail"), &obj.role_detail_list);
		groupDetailListTypeWriter::write_params(params, &(prefix.to_string() + "GroupDetail"), &obj.group_detail_list);
		userDetailListTypeWriter::write_params(params, &(prefix.to_string() + "UserDetail"), &obj.user_detail_list);
		ManagedPolicyDetailListTypeWriter::write_params(params, &(prefix.to_string() + "ManagedPolicyDetail"), &obj.policies);
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
	}
}
#[derive(Debug, Default)]
pub struct ListUsersRequest {
	/// Use this parameter only when paginating results and only after you have
	/// received a response where the results are truncated. Set it to the value of
	/// the `Marker` element in the response you just received.
	pub marker: markerType,
	/// The path prefix for filtering the results. For example:
	/// `/division_abc/subdivision_xyz/`, which would get all user names whose path
	/// starts with `/division_abc/subdivision_xyz/`.
	/// This parameter is optional. If it is not included, it defaults to a slash (/),
	/// listing all user names.
	pub path_prefix: pathPrefixType,
	/// Use this only when paginating results to indicate the maximum number of items
	/// you want in the response. If there are additional items beyond the maximum you
	/// specify, the `IsTruncated` response element is `true`.
	/// This parameter is optional. If you do not include it, it defaults to 100.
	pub max_items: maxItemsType,
}

/// Parse ListUsersRequest from XML
struct ListUsersRequestParser;
impl ListUsersRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListUsersRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListUsersRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = try!(markerTypeParser::parse_xml("Marker", stack));
				continue;
			}
			if current_name == "PathPrefix" {
				obj.path_prefix = try!(pathPrefixTypeParser::parse_xml("PathPrefix", stack));
				continue;
			}
			if current_name == "MaxItems" {
				obj.max_items = try!(maxItemsTypeParser::parse_xml("MaxItems", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListUsersRequest contents to a SignedRequest
struct ListUsersRequestWriter;
impl ListUsersRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListUsersRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
		pathPrefixTypeWriter::write_params(params, &(prefix.to_string() + "PathPrefix"), &obj.path_prefix);
		maxItemsTypeWriter::write_params(params, &(prefix.to_string() + "MaxItems"), &obj.max_items);
	}
}
#[derive(Debug, Default)]
pub struct DeleteOpenIDConnectProviderRequest {
	/// The Amazon Resource Name (ARN) of the IAM OpenID Connect provider to delete.
	/// You can get a list of OpenID Connect provider ARNs by using the
	/// ListOpenIDConnectProviders action.
	pub open_id_connect_provider_arn: arnType,
}

/// Parse DeleteOpenIDConnectProviderRequest from XML
struct DeleteOpenIDConnectProviderRequestParser;
impl DeleteOpenIDConnectProviderRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteOpenIDConnectProviderRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteOpenIDConnectProviderRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "OpenIDConnectProviderArn" {
				obj.open_id_connect_provider_arn = try!(arnTypeParser::parse_xml("OpenIDConnectProviderArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteOpenIDConnectProviderRequest contents to a SignedRequest
struct DeleteOpenIDConnectProviderRequestWriter;
impl DeleteOpenIDConnectProviderRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteOpenIDConnectProviderRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "OpenIDConnectProviderArn"), &obj.open_id_connect_provider_arn);
	}
}
/// The request was rejected because the SSH public key is already associated with
/// the specified IAM user.
#[derive(Debug, Default)]
pub struct DuplicateSSHPublicKeyException {
	pub message: duplicateSSHPublicKeyMessage,
}

/// Parse DuplicateSSHPublicKeyException from XML
struct DuplicateSSHPublicKeyExceptionParser;
impl DuplicateSSHPublicKeyExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DuplicateSSHPublicKeyException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DuplicateSSHPublicKeyException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(duplicateSSHPublicKeyMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DuplicateSSHPublicKeyException contents to a SignedRequest
struct DuplicateSSHPublicKeyExceptionWriter;
impl DuplicateSSHPublicKeyExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DuplicateSSHPublicKeyException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		duplicateSSHPublicKeyMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains the response to a successful UpdateSAMLProvider request.
#[derive(Debug, Default)]
pub struct UpdateSAMLProviderResponse {
	/// The Amazon Resource Name (ARN) of the SAML provider that was updated.
	pub saml_provider_arn: arnType,
}

/// Parse UpdateSAMLProviderResponse from XML
struct UpdateSAMLProviderResponseParser;
impl UpdateSAMLProviderResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateSAMLProviderResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateSAMLProviderResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SAMLProviderArn" {
				obj.saml_provider_arn = try!(arnTypeParser::parse_xml("SAMLProviderArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateSAMLProviderResponse contents to a SignedRequest
struct UpdateSAMLProviderResponseWriter;
impl UpdateSAMLProviderResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateSAMLProviderResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "SAMLProviderArn"), &obj.saml_provider_arn);
	}
}
#[derive(Debug, Default)]
pub struct ChangePasswordRequest {
	/// The new password. The new password must conform to the AWS account's password
	/// policy, if one exists.
	pub new_password: passwordType,
	/// The IAM user's current password.
	pub old_password: passwordType,
}

/// Parse ChangePasswordRequest from XML
struct ChangePasswordRequestParser;
impl ChangePasswordRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ChangePasswordRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ChangePasswordRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NewPassword" {
				obj.new_password = try!(passwordTypeParser::parse_xml("NewPassword", stack));
				continue;
			}
			if current_name == "OldPassword" {
				obj.old_password = try!(passwordTypeParser::parse_xml("OldPassword", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ChangePasswordRequest contents to a SignedRequest
struct ChangePasswordRequestWriter;
impl ChangePasswordRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ChangePasswordRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		passwordTypeWriter::write_params(params, &(prefix.to_string() + "NewPassword"), &obj.new_password);
		passwordTypeWriter::write_params(params, &(prefix.to_string() + "OldPassword"), &obj.old_password);
	}
}
#[derive(Debug, Default)]
pub struct DeleteSAMLProviderRequest {
	/// The Amazon Resource Name (ARN) of the SAML provider to delete.
	pub saml_provider_arn: arnType,
}

/// Parse DeleteSAMLProviderRequest from XML
struct DeleteSAMLProviderRequestParser;
impl DeleteSAMLProviderRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteSAMLProviderRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteSAMLProviderRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SAMLProviderArn" {
				obj.saml_provider_arn = try!(arnTypeParser::parse_xml("SAMLProviderArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteSAMLProviderRequest contents to a SignedRequest
struct DeleteSAMLProviderRequestWriter;
impl DeleteSAMLProviderRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteSAMLProviderRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		arnTypeWriter::write_params(params, &(prefix.to_string() + "SAMLProviderArn"), &obj.saml_provider_arn);
	}
}
/// Contains information about the account password policy.
/// This data type is used as a response element in the GetAccountPasswordPolicy
/// action.
#[derive(Debug, Default)]
pub struct PasswordPolicy {
	/// Specifies whether IAM users are allowed to change their own password.
	pub allow_users_to_change_password: booleanType,
	/// Specifies whether to require lowercase characters for IAM user passwords.
	pub require_lowercase_characters: booleanType,
	/// Specifies whether to require uppercase characters for IAM user passwords.
	pub require_uppercase_characters: booleanType,
	/// Minimum length to require for IAM user passwords.
	pub minimum_password_length: minimumPasswordLengthType,
	/// Specifies whether to require numbers for IAM user passwords.
	pub require_numbers: booleanType,
	/// Specifies the number of previous passwords that IAM users are prevented from
	/// reusing.
	pub password_reuse_prevention: passwordReusePreventionType,
	/// Specifies whether IAM users are prevented from setting a new password after
	/// their password has expired.
	pub hard_expiry: booleanObjectType,
	/// Specifies whether to require symbols for IAM user passwords.
	pub require_symbols: booleanType,
	/// The number of days that an IAM user password is valid.
	pub max_password_age: maxPasswordAgeType,
	/// Specifies whether IAM users are required to change their password after a
	/// specified number of days.
	pub expire_passwords: booleanType,
}

/// Parse PasswordPolicy from XML
struct PasswordPolicyParser;
impl PasswordPolicyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PasswordPolicy, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PasswordPolicy::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AllowUsersToChangePassword" {
				obj.allow_users_to_change_password = try!(booleanTypeParser::parse_xml("AllowUsersToChangePassword", stack));
				continue;
			}
			if current_name == "RequireLowercaseCharacters" {
				obj.require_lowercase_characters = try!(booleanTypeParser::parse_xml("RequireLowercaseCharacters", stack));
				continue;
			}
			if current_name == "RequireUppercaseCharacters" {
				obj.require_uppercase_characters = try!(booleanTypeParser::parse_xml("RequireUppercaseCharacters", stack));
				continue;
			}
			if current_name == "MinimumPasswordLength" {
				obj.minimum_password_length = try!(minimumPasswordLengthTypeParser::parse_xml("MinimumPasswordLength", stack));
				continue;
			}
			if current_name == "RequireNumbers" {
				obj.require_numbers = try!(booleanTypeParser::parse_xml("RequireNumbers", stack));
				continue;
			}
			if current_name == "PasswordReusePrevention" {
				obj.password_reuse_prevention = try!(passwordReusePreventionTypeParser::parse_xml("PasswordReusePrevention", stack));
				continue;
			}
			if current_name == "HardExpiry" {
				obj.hard_expiry = try!(booleanObjectTypeParser::parse_xml("HardExpiry", stack));
				continue;
			}
			if current_name == "RequireSymbols" {
				obj.require_symbols = try!(booleanTypeParser::parse_xml("RequireSymbols", stack));
				continue;
			}
			if current_name == "MaxPasswordAge" {
				obj.max_password_age = try!(maxPasswordAgeTypeParser::parse_xml("MaxPasswordAge", stack));
				continue;
			}
			if current_name == "ExpirePasswords" {
				obj.expire_passwords = try!(booleanTypeParser::parse_xml("ExpirePasswords", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PasswordPolicy contents to a SignedRequest
struct PasswordPolicyWriter;
impl PasswordPolicyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PasswordPolicy) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "AllowUsersToChangePassword"), &obj.allow_users_to_change_password);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "RequireLowercaseCharacters"), &obj.require_lowercase_characters);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "RequireUppercaseCharacters"), &obj.require_uppercase_characters);
		minimumPasswordLengthTypeWriter::write_params(params, &(prefix.to_string() + "MinimumPasswordLength"), &obj.minimum_password_length);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "RequireNumbers"), &obj.require_numbers);
		passwordReusePreventionTypeWriter::write_params(params, &(prefix.to_string() + "PasswordReusePrevention"), &obj.password_reuse_prevention);
		booleanObjectTypeWriter::write_params(params, &(prefix.to_string() + "HardExpiry"), &obj.hard_expiry);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "RequireSymbols"), &obj.require_symbols);
		maxPasswordAgeTypeWriter::write_params(params, &(prefix.to_string() + "MaxPasswordAge"), &obj.max_password_age);
		booleanTypeWriter::write_params(params, &(prefix.to_string() + "ExpirePasswords"), &obj.expire_passwords);
	}
}
/// Contains the response to a successful ListMFADevices request.
#[derive(Debug, Default)]
pub struct ListMFADevicesResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A list of MFA devices.
	pub mfa_devices: mfaDeviceListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
}

/// Parse ListMFADevicesResponse from XML
struct ListMFADevicesResponseParser;
impl ListMFADevicesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListMFADevicesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListMFADevicesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "MFADevice" {
				obj.mfa_devices = try!(mfaDeviceListTypeParser::parse_xml("MFADevice", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListMFADevicesResponse contents to a SignedRequest
struct ListMFADevicesResponseWriter;
impl ListMFADevicesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListMFADevicesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		mfaDeviceListTypeWriter::write_params(params, &(prefix.to_string() + "MFADevice"), &obj.mfa_devices);
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
	}
}
pub type maxItemsType = i32;
/// Parse maxItemsType from XML
struct maxItemsTypeParser;
impl maxItemsTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<maxItemsType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write maxItemsType contents to a SignedRequest
struct maxItemsTypeWriter;
impl maxItemsTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &maxItemsType) {
		params.put(name, &obj.to_string());
	}
}
/// Contains the response to a successful GetSSHPublicKey request.
#[derive(Debug, Default)]
pub struct GetSSHPublicKeyResponse {
	/// Information about the SSH public key.
	pub ssh_public_key: SSHPublicKey,
}

/// Parse GetSSHPublicKeyResponse from XML
struct GetSSHPublicKeyResponseParser;
impl GetSSHPublicKeyResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetSSHPublicKeyResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetSSHPublicKeyResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SSHPublicKey" {
				obj.ssh_public_key = try!(SSHPublicKeyParser::parse_xml("SSHPublicKey", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetSSHPublicKeyResponse contents to a SignedRequest
struct GetSSHPublicKeyResponseWriter;
impl GetSSHPublicKeyResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetSSHPublicKeyResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		SSHPublicKeyWriter::write_params(params, &(prefix.to_string() + "SSHPublicKey"), &obj.ssh_public_key);
	}
}
/// Contains the response to a successful ListAccessKeys request.
#[derive(Debug, Default)]
pub struct ListAccessKeysResponse {
	/// When `IsTruncated` is `true`, this element is present and contains the value
	/// to use for the `Marker` parameter in a subsequent pagination request.
	pub marker: Option<markerType>,
	/// A list of access key metadata.
	pub access_key_metadata: accessKeyMetadataListType,
	/// A flag that indicates whether there are more items to return. If your results
	/// were truncated, you can make a subsequent pagination request using the
	/// `Marker` request parameter to retrieve more items.
	pub is_truncated: Option<booleanType>,
}

/// Parse ListAccessKeysResponse from XML
struct ListAccessKeysResponseParser;
impl ListAccessKeysResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListAccessKeysResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListAccessKeysResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Marker" {
				obj.marker = Some(try!(markerTypeParser::parse_xml("Marker", stack)));
				continue;
			}
			if current_name == "AccessKeyMetadata" {
				obj.access_key_metadata = try!(accessKeyMetadataListTypeParser::parse_xml("AccessKeyMetadata", stack));
				continue;
			}
			if current_name == "IsTruncated" {
				obj.is_truncated = Some(try!(booleanTypeParser::parse_xml("IsTruncated", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListAccessKeysResponse contents to a SignedRequest
struct ListAccessKeysResponseWriter;
impl ListAccessKeysResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListAccessKeysResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.marker {
			markerTypeWriter::write_params(params, &(prefix.to_string() + "Marker"), obj);
		}
		accessKeyMetadataListTypeWriter::write_params(params, &(prefix.to_string() + "AccessKeyMetadata"), &obj.access_key_metadata);
		if let Some(ref obj) = obj.is_truncated {
			booleanTypeWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), obj);
		}
	}
}
/// The request was rejected because it attempted to create a resource that
/// already exists.
#[derive(Debug, Default)]
pub struct EntityAlreadyExistsException {
	pub message: entityAlreadyExistsMessage,
}

/// Parse EntityAlreadyExistsException from XML
struct EntityAlreadyExistsExceptionParser;
impl EntityAlreadyExistsExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EntityAlreadyExistsException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EntityAlreadyExistsException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(entityAlreadyExistsMessageParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EntityAlreadyExistsException contents to a SignedRequest
struct EntityAlreadyExistsExceptionWriter;
impl EntityAlreadyExistsExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EntityAlreadyExistsException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		entityAlreadyExistsMessageWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Contains a list of MFA devices.
/// This data type is used as a response element in the ListMFADevices and
/// ListVirtualMFADevices actions.
pub type mfaDeviceListType = Vec<MFADevice>;
/// Parse mfaDeviceListType from XML
struct mfaDeviceListTypeParser;
impl mfaDeviceListTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<mfaDeviceListType, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "MFADevice" {
			obj.push(try!(MFADeviceParser::parse_xml("MFADevice", stack)));
		}
		Ok(obj)
	}
}
/// Write mfaDeviceListType contents to a SignedRequest
struct mfaDeviceListTypeWriter;
impl mfaDeviceListTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &mfaDeviceListType) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			MFADeviceWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct DeleteSigningCertificateRequest {
	/// The name of the user the signing certificate belongs to.
	pub user_name: Option<existingUserNameType>,
	/// The ID of the signing certificate to delete.
	pub certificate_id: certificateIdType,
}

/// Parse DeleteSigningCertificateRequest from XML
struct DeleteSigningCertificateRequestParser;
impl DeleteSigningCertificateRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteSigningCertificateRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteSigningCertificateRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = Some(try!(existingUserNameTypeParser::parse_xml("UserName", stack)));
				continue;
			}
			if current_name == "CertificateId" {
				obj.certificate_id = try!(certificateIdTypeParser::parse_xml("CertificateId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteSigningCertificateRequest contents to a SignedRequest
struct DeleteSigningCertificateRequestWriter;
impl DeleteSigningCertificateRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteSigningCertificateRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.user_name {
			existingUserNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), obj);
		}
		certificateIdTypeWriter::write_params(params, &(prefix.to_string() + "CertificateId"), &obj.certificate_id);
	}
}
/// Contains information about a server certificate.
/// This data type is used as a response element in the GetServerCertificate
/// action.
#[derive(Debug, Default)]
pub struct ServerCertificate {
	/// The contents of the public key certificate chain.
	pub certificate_chain: Option<certificateChainType>,
	/// The contents of the public key certificate.
	pub certificate_body: certificateBodyType,
	/// The meta information of the server certificate, such as its name, path, ID,
	/// and ARN.
	pub server_certificate_metadata: ServerCertificateMetadata,
}

/// Parse ServerCertificate from XML
struct ServerCertificateParser;
impl ServerCertificateParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ServerCertificate, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ServerCertificate::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "CertificateChain" {
				obj.certificate_chain = Some(try!(certificateChainTypeParser::parse_xml("CertificateChain", stack)));
				continue;
			}
			if current_name == "CertificateBody" {
				obj.certificate_body = try!(certificateBodyTypeParser::parse_xml("CertificateBody", stack));
				continue;
			}
			if current_name == "ServerCertificateMetadata" {
				obj.server_certificate_metadata = try!(ServerCertificateMetadataParser::parse_xml("ServerCertificateMetadata", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ServerCertificate contents to a SignedRequest
struct ServerCertificateWriter;
impl ServerCertificateWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ServerCertificate) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.certificate_chain {
			certificateChainTypeWriter::write_params(params, &(prefix.to_string() + "CertificateChain"), obj);
		}
		certificateBodyTypeWriter::write_params(params, &(prefix.to_string() + "CertificateBody"), &obj.certificate_body);
		ServerCertificateMetadataWriter::write_params(params, &(prefix.to_string() + "ServerCertificateMetadata"), &obj.server_certificate_metadata);
	}
}
/// Contains information about an SSH public key, without the key's body or
/// fingerprint.
/// This data type is used as a response element in the ListSSHPublicKeys action.
#[derive(Debug, Default)]
pub struct SSHPublicKeyMetadata {
	/// The name of the IAM user associated with the SSH public key.
	pub user_name: userNameType,
	/// The status of the SSH public key. `Active` means the key can be used for
	/// authentication with an AWS CodeCommit repository. `Inactive` means the key
	/// cannot be used.
	pub status: statusType,
	/// The unique identifier for the SSH public key.
	pub ssh_public_key_id: publicKeyIdType,
	/// The date and time, in [ISO 8601 date-time
	/// format](http://www.iso.org/iso/iso8601), when the SSH public key was uploaded.
	pub upload_date: dateType,
}

/// Parse SSHPublicKeyMetadata from XML
struct SSHPublicKeyMetadataParser;
impl SSHPublicKeyMetadataParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SSHPublicKeyMetadata, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SSHPublicKeyMetadata::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserName" {
				obj.user_name = try!(userNameTypeParser::parse_xml("UserName", stack));
				continue;
			}
			if current_name == "Status" {
				obj.status = try!(statusTypeParser::parse_xml("Status", stack));
				continue;
			}
			if current_name == "SSHPublicKeyId" {
				obj.ssh_public_key_id = try!(publicKeyIdTypeParser::parse_xml("SSHPublicKeyId", stack));
				continue;
			}
			if current_name == "UploadDate" {
				obj.upload_date = try!(dateTypeParser::parse_xml("UploadDate", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SSHPublicKeyMetadata contents to a SignedRequest
struct SSHPublicKeyMetadataWriter;
impl SSHPublicKeyMetadataWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SSHPublicKeyMetadata) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		userNameTypeWriter::write_params(params, &(prefix.to_string() + "UserName"), &obj.user_name);
		statusTypeWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		publicKeyIdTypeWriter::write_params(params, &(prefix.to_string() + "SSHPublicKeyId"), &obj.ssh_public_key_id);
		dateTypeWriter::write_params(params, &(prefix.to_string() + "UploadDate"), &obj.upload_date);
	}
}
/// Contains the response to a successful CreateGroup request.
#[derive(Debug, Default)]
pub struct CreateGroupResponse {
	/// Information about the group.
	pub group: Group,
}

/// Parse CreateGroupResponse from XML
struct CreateGroupResponseParser;
impl CreateGroupResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateGroupResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateGroupResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Group" {
				obj.group = try!(GroupParser::parse_xml("Group", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateGroupResponse contents to a SignedRequest
struct CreateGroupResponseWriter;
impl CreateGroupResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateGroupResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		GroupWriter::write_params(params, &(prefix.to_string() + "Group"), &obj.group);
	}
}
pub struct IAMClient<'a> {
	creds: &'a AWSCredentials,
	region: &'a str
}

impl<'a> IAMClient<'a> { 
	pub fn new(creds: &'a AWSCredentials, region: &'a str) -> IAMClient<'a> {
		IAMClient { creds: creds, region: region }
	}
	/// Retrieves the specified inline policy document that is embedded in the
	/// specified user.
	/// A user can also have managed policies attached to it. To retrieve a managed
	/// policy document that is attached to a user, use GetPolicy to determine the
	/// policy's default version, then use GetPolicyVersion to retrieve the policy
	/// document.
	/// For more information about policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn get_user_policy(&self, input: &GetUserPolicyRequest) -> Result<GetUserPolicyResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetUserPolicy");
		GetUserPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetUserPolicyResponseParser::parse_xml("GetUserPolicyResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified AWS account alias. For information about using an AWS
	/// account alias, see [Using an Alias for Your AWS Account
	/// ID](http://docs.aws.amazon.com/IAM/latest/UserGuide/AccountAlias.html) in the
	/// _Using IAM_ guide.
	pub fn delete_account_alias(&self, input: &DeleteAccountAliasRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteAccountAlias");
		DeleteAccountAliasRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified instance profile. The instance profile must not have an
	/// associated role.
	/// Make sure you do not have any Amazon EC2 instances running with the instance
	/// profile you are about to delete. Deleting a role or instance profile that is
	/// associated with a running instance will break any applications running on the
	/// instance.
	/// For more information about instance profiles, go to [About Instance Profiles](
	/// http://docs.aws.amazon.com/IAM/latest/UserGuide/AboutInstanceProfiles.html).
	pub fn delete_instance_profile(&self, input: &DeleteInstanceProfileRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteInstanceProfile");
		DeleteInstanceProfileRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists all managed policies that are attached to the specified group.
	/// A group can also have inline policies embedded with it. To list the inline
	/// policies for a group, use the ListGroupPolicies API. For information about
	/// policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters. You
	/// can use the `PathPrefix` parameter to limit the list of policies to only those
	/// matching the specified path prefix. If there are no policies attached to the
	/// specified group (or none that match the specified path prefix), the action
	/// returns an empty list.
	pub fn list_attached_group_policies(&self, input: &ListAttachedGroupPoliciesRequest) -> Result<ListAttachedGroupPoliciesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListAttachedGroupPolicies");
		ListAttachedGroupPoliciesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListAttachedGroupPoliciesResponseParser::parse_xml("ListAttachedGroupPoliciesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Updates the name and/or the path of the specified user.
	/// You should understand the implications of changing a user's path or name. For
	/// more information, see [Renaming Users and Groups](http://docs.aws.amazon.com/I
	/// AM/latest/UserGuide/Using_WorkingWithGroupsAndUsers.html) in the _Using IAM_
	/// guide.  To change a user name the requester must have appropriate permissions
	/// on both the source object and the target object. For example, to change Bob to
	/// Robert, the entity making the request must have permission on Bob and Robert,
	/// or must have permission on all (*). For more information about permissions,
	/// see [Permissions and Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide
	/// /PermissionsAndPolicies.html).
	pub fn update_user(&self, input: &UpdateUserRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateUser");
		UpdateUserRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the account aliases associated with the account. For information about
	/// using an AWS account alias, see [Using an Alias for Your AWS Account
	/// ID](http://docs.aws.amazon.com/IAM/latest/UserGuide/AccountAlias.html) in the
	/// _Using IAM_ guide.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_account_aliases(&self, input: &ListAccountAliasesRequest) -> Result<ListAccountAliasesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListAccountAliases");
		ListAccountAliasesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListAccountAliasesResponseParser::parse_xml("ListAccountAliasesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves information about the specified user, including the user's creation
	/// date, path, unique ID, and ARN.
	/// If you do not specify a user name, IAM determines the user name implicitly
	/// based on the AWS access key ID used to sign the request.
	pub fn get_user(&self, input: &GetUserRequest) -> Result<GetUserResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetUser");
		GetUserRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetUserResponseParser::parse_xml("GetUserResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Uploads an SSH public key and associates it with the specified IAM user.
	/// The SSH public key uploaded by this action can be used only for authenticating
	/// the associated IAM user to an AWS CodeCommit repository. For more information
	/// about using SSH keys to authenticate to an AWS CodeCommit repository, see [Set
	/// up AWS CodeCommit for SSH
	/// Connections](http://docs.aws.amazon.com/codecommit/latest/userguide/setting-
	/// up-credentials-ssh.html) in the _AWS CodeCommit User Guide_.
	pub fn upload_ssh_public_key(&self, input: &UploadSSHPublicKeyRequest) -> Result<UploadSSHPublicKeyResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UploadSSHPublicKey");
		UploadSSHPublicKeyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(UploadSSHPublicKeyResponseParser::parse_xml("UploadSSHPublicKeyResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the names of the inline policies that are embedded in the specified
	/// group.
	/// A group can also have managed policies attached to it. To list the managed
	/// policies that are attached to a group, use ListAttachedGroupPolicies. For more
	/// information about policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters. If
	/// there are no inline policies embedded with the specified group, the action
	/// returns an empty list.
	pub fn list_group_policies(&self, input: &ListGroupPoliciesRequest) -> Result<ListGroupPoliciesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListGroupPolicies");
		ListGroupPoliciesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListGroupPoliciesResponseParser::parse_xml("ListGroupPoliciesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a new AWS secret access key and corresponding AWS access key ID for
	/// the specified user. The default status for new keys is `Active`.
	/// If you do not specify a user name, IAM determines the user name implicitly
	/// based on the AWS access key ID signing the request. Because this action works
	/// for access keys under the AWS account, you can use this action to manage root
	/// credentials even if the AWS account has no associated users.
	/// For information about limits on the number of keys you can create, see
	/// [Limitations on IAM Entities](http://docs.aws.amazon.com/IAM/latest/UserGuide/
	/// LimitationsOnEntities.html) in the _Using IAM_ guide.
	/// To ensure the security of your AWS account, the secret access key is
	/// accessible only during key and user creation. You must save the key (for
	/// example, in a text file) if you want to be able to access it again. If a
	/// secret key is lost, you can delete the access keys for the associated user and
	/// then create new keys.
	pub fn create_access_key(&self, input: &CreateAccessKeyRequest) -> Result<CreateAccessKeyResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateAccessKey");
		CreateAccessKeyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateAccessKeyResponseParser::parse_xml("CreateAccessKeyResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Changes the password for the specified user.
	/// Users can change their own passwords by calling ChangePassword. For more
	/// information about modifying passwords, see [Managing Passwords](http://docs.aw
	/// s.amazon.com/IAM/latest/UserGuide/Using_ManagingLogins.html) in the _Using
	/// IAM_ guide.
	pub fn update_login_profile(&self, input: &UpdateLoginProfileRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateLoginProfile");
		UpdateLoginProfileRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the groups that have the specified path prefix.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_groups(&self, input: &ListGroupsRequest) -> Result<ListGroupsResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListGroups");
		ListGroupsRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListGroupsResponseParser::parse_xml("ListGroupsResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Updates the metadata document for an existing SAML provider.
	/// This operation requires [Signature Version
	/// 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html).
	pub fn update_saml_provider(&self, input: &UpdateSAMLProviderRequest) -> Result<UpdateSAMLProviderResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateSAMLProvider");
		UpdateSAMLProviderRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(UpdateSAMLProviderResponseParser::parse_xml("UpdateSAMLProviderResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified SSH public key.
	/// The SSH public key deleted by this action is used only for authenticating the
	/// associated IAM user to an AWS CodeCommit repository. For more information
	/// about using SSH keys to authenticate to an AWS CodeCommit repository, see [Set
	/// up AWS CodeCommit for SSH
	/// Connections](http://docs.aws.amazon.com/codecommit/latest/userguide/setting-
	/// up-credentials-ssh.html) in the _AWS CodeCommit User Guide_.
	pub fn delete_ssh_public_key(&self, input: &DeleteSSHPublicKeyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteSSHPublicKey");
		DeleteSSHPublicKeyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Removes the specified user from the specified group.
	pub fn remove_user_from_group(&self, input: &RemoveUserFromGroupRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "RemoveUserFromGroup");
		RemoveUserFromGroupRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the SAML providers in the account.
	/// This operation requires [Signature Version
	/// 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html).
	pub fn list_saml_providers(&self, input: &ListSAMLProvidersRequest) -> Result<ListSAMLProvidersResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListSAMLProviders");
		ListSAMLProvidersRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListSAMLProvidersResponseParser::parse_xml("ListSAMLProvidersResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified inline policy that is embedded in the specified role.
	/// A role can also have managed policies attached to it. To detach a managed
	/// policy from a role, use DetachRolePolicy. For more information about policies,
	/// refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn delete_role_policy(&self, input: &DeleteRolePolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteRolePolicy");
		DeleteRolePolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Uploads a server certificate entity for the AWS account. The server
	/// certificate entity includes a public key certificate, a private key, and an
	/// optional certificate chain, which should all be PEM-encoded.
	/// For information about the number of server certificates you can upload, see
	/// [Limitations on IAM Entities](http://docs.aws.amazon.com/IAM/latest/UserGuide/
	/// LimitationsOnEntities.html) in the _Using IAM_ guide.
	/// Because the body of the public key certificate, private key, and the
	/// certificate chain can be large, you should use POST rather than GET when
	/// calling `UploadServerCertificate`. For information about setting up signatures
	/// and authorization through the API, go to [Signing AWS API Requests](http://doc
	/// s.aws.amazon.com/general/latest/gr/signing_aws_api_requests.html) in the _AWS
	/// General Reference_. For general information about using the Query API with
	/// IAM, go to [Making Query Requests](http://docs.aws.amazon.com/IAM/latest/UserG
	/// uide/IAM_UsingQueryAPI.html) in the _Using IAM_ guide.
	pub fn upload_server_certificate(&self, input: &UploadServerCertificateRequest) -> Result<UploadServerCertificateResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UploadServerCertificate");
		UploadServerCertificateRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(UploadServerCertificateResponseParser::parse_xml("UploadServerCertificateResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Changes the status of the specified access key from Active to Inactive, or
	/// vice versa. This action can be used to disable a user's key as part of a key
	/// rotation work flow.
	/// If the `UserName` field is not specified, the UserName is determined
	/// implicitly based on the AWS access key ID used to sign the request. Because
	/// this action works for access keys under the AWS account, you can use this
	/// action to manage root credentials even if the AWS account has no associated
	/// users.
	/// For information about rotating keys, see [Managing Keys and Certificates](http
	/// ://docs.aws.amazon.com/IAM/latest/UserGuide/ManagingCredentials.html) in the
	/// _Using IAM_ guide.
	pub fn update_access_key(&self, input: &UpdateAccessKeyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateAccessKey");
		UpdateAccessKeyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the roles that have the specified path prefix. If there are none, the
	/// action returns an empty list. For more information about roles, go to [Working
	/// with
	/// Roles](http://docs.aws.amazon.com/IAM/latest/UserGuide/WorkingWithRoles.html).
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_roles(&self, input: &ListRolesRequest) -> Result<ListRolesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListRoles");
		ListRolesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListRolesResponseParser::parse_xml("ListRolesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a new user for your AWS account.
	/// For information about limitations on the number of users you can create, see
	/// [Limitations on IAM Entities](http://docs.aws.amazon.com/IAM/latest/UserGuide/
	/// LimitationsOnEntities.html) in the _Using IAM_ guide.
	pub fn create_user(&self, input: &CreateUserRequest) -> Result<CreateUserResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateUser");
		CreateUserRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateUserResponseParser::parse_xml("CreateUserResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Adds the specified role to the specified instance profile. For more
	/// information about roles, go to [Working with
	/// Roles](http://docs.aws.amazon.com/IAM/latest/UserGuide/WorkingWithRoles.html).
	/// For more information about instance profiles, go to [About Instance Profiles](
	/// http://docs.aws.amazon.com/IAM/latest/UserGuide/AboutInstanceProfiles.html).
	pub fn add_role_to_instance_profile(&self, input: &AddRoleToInstanceProfileRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "AddRoleToInstanceProfile");
		AddRoleToInstanceProfileRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves the specified inline policy document that is embedded in the
	/// specified group.
	/// A group can also have managed policies attached to it. To retrieve a managed
	/// policy document that is attached to a group, use GetPolicy to determine the
	/// policy's default version, then use GetPolicyVersion to retrieve the policy
	/// document.
	/// For more information about policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn get_group_policy(&self, input: &GetGroupPolicyRequest) -> Result<GetGroupPolicyResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetGroupPolicy");
		GetGroupPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetGroupPolicyResponseParser::parse_xml("GetGroupPolicyResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a password for the specified user, giving the user the ability to
	/// access AWS services through the AWS Management Console. For more information
	/// about managing passwords, see [Managing Passwords](http://docs.aws.amazon.com/
	/// IAM/latest/UserGuide/Using_ManagingLogins.html) in the _Using IAM_ guide.
	pub fn create_login_profile(&self, input: &CreateLoginProfileRequest) -> Result<CreateLoginProfileResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateLoginProfile");
		CreateLoginProfileRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateLoginProfileResponseParser::parse_xml("CreateLoginProfileResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Changes the status of the specified signing certificate from active to
	/// disabled, or vice versa. This action can be used to disable a user's signing
	/// certificate as part of a certificate rotation work flow.
	/// If the `UserName` field is not specified, the UserName is determined
	/// implicitly based on the AWS access key ID used to sign the request. Because
	/// this action works for access keys under the AWS account, you can use this
	/// action to manage root credentials even if the AWS account has no associated
	/// users.
	pub fn update_signing_certificate(&self, input: &UpdateSigningCertificateRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateSigningCertificate");
		UpdateSigningCertificateRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deactivates the specified MFA device and removes it from association with the
	/// user name for which it was originally enabled.
	/// For more information about creating and working with virtual MFA devices, go
	/// to [Using a Virtual MFA
	/// Device](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_VirtualMFA.html)
	/// in the _Using IAM_ guide.
	pub fn deactivate_mfa_device(&self, input: &DeactivateMFADeviceRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeactivateMFADevice");
		DeactivateMFADeviceRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Removes the specified managed policy from the specified role.
	/// A role can also have inline policies embedded with it. To delete an inline
	/// policy, use the DeleteRolePolicy API. For information about policies, refer to
	/// [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn detach_role_policy(&self, input: &DetachRolePolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DetachRolePolicy");
		DetachRolePolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves information about the specified managed policy, including the
	/// policy's default version and the total number of users, groups, and roles that
	/// the policy is attached to. For a list of the specific users, groups, and roles
	/// that the policy is attached to, use the ListEntitiesForPolicy API. This API
	/// returns metadata about the policy. To retrieve the policy document for a
	/// specific version of the policy, use GetPolicyVersion.
	/// This API retrieves information about managed policies. To retrieve information
	/// about an inline policy that is embedded with a user, group, or role, use the
	/// GetUserPolicy, GetGroupPolicy, or GetRolePolicy API.
	/// For more information about policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn get_policy(&self, input: &GetPolicyRequest) -> Result<GetPolicyResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetPolicy");
		GetPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetPolicyResponseParser::parse_xml("GetPolicyResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates an IAM entity to describe an identity provider (IdP) that supports
	/// SAML 2.0.
	/// The SAML provider that you create with this operation can be used as a
	/// principal in a role's trust policy to establish a trust relationship between
	/// AWS and a SAML identity provider. You can create an IAM role that supports
	/// Web-based single sign-on (SSO) to the AWS Management Console or one that
	/// supports API access to AWS.
	/// When you create the SAML provider, you upload an a SAML metadata document that
	/// you get from your IdP and that includes the issuer's name, expiration
	/// information, and keys that can be used to validate the SAML authentication
	/// response (assertions) that are received from the IdP. You must generate the
	/// metadata document using the identity management software that is used as your
	/// organization's IdP.
	/// This operation requires [Signature Version
	/// 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html).
	/// For more information, see [Giving Console Access Using
	/// SAML](http://docs.aws.amazon.com/STS/latest/UsingSTS/STSMgmtConsole-SAML.html)
	/// and [Creating Temporary Security Credentials for SAML
	/// Federation](http://docs.aws.amazon.com/STS/latest/UsingSTS/CreatingSAML.html)
	/// in the _Using Temporary Credentials_ guide.
	pub fn create_saml_provider(&self, input: &CreateSAMLProviderRequest) -> Result<CreateSAMLProviderResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateSAMLProvider");
		CreateSAMLProviderRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateSAMLProviderResponseParser::parse_xml("CreateSAMLProviderResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists information about the OpenID Connect providers in the AWS account.
	pub fn list_open_id_connect_providers(&self, input: &ListOpenIDConnectProvidersRequest) -> Result<ListOpenIDConnectProvidersResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListOpenIDConnectProviders");
		ListOpenIDConnectProvidersRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListOpenIDConnectProvidersResponseParser::parse_xml("ListOpenIDConnectProvidersResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns information about the specified OpenID Connect provider.
	pub fn get_open_id_connect_provider(&self, input: &GetOpenIDConnectProviderRequest) -> Result<GetOpenIDConnectProviderResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetOpenIDConnectProvider");
		GetOpenIDConnectProviderRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetOpenIDConnectProviderResponseParser::parse_xml("GetOpenIDConnectProviderResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns information about the signing certificates associated with the
	/// specified user. If there are none, the action returns an empty list.
	/// Although each user is limited to a small number of signing certificates, you
	/// can still paginate the results using the `MaxItems` and `Marker` parameters.
	/// If the `UserName` field is not specified, the user name is determined
	/// implicitly based on the AWS access key ID used to sign the request. Because
	/// this action works for access keys under the AWS account, you can use this
	/// action to manage root credentials even if the AWS account has no associated
	/// users.
	pub fn list_signing_certificates(&self, input: &ListSigningCertificatesRequest) -> Result<ListSigningCertificatesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListSigningCertificates");
		ListSigningCertificatesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListSigningCertificatesResponseParser::parse_xml("ListSigningCertificatesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified role. The role must not have any policies attached. For
	/// more information about roles, go to [Working with
	/// Roles](http://docs.aws.amazon.com/IAM/latest/UserGuide/WorkingWithRoles.html).
	/// Make sure you do not have any Amazon EC2 instances running with the role you
	/// are about to delete. Deleting a role or instance profile that is associated
	/// with a running instance will break any applications running on the instance.
	pub fn delete_role(&self, input: &DeleteRoleRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteRole");
		DeleteRoleRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Updates the name and/or the path of the specified group.
	/// You should understand the implications of changing a group's path or name. For
	/// more information, see [Renaming Users and Groups](http://docs.aws.amazon.com/I
	/// AM/latest/UserGuide/Using_WorkingWithGroupsAndUsers.html) in the _Using IAM_
	/// guide.  To change a group name the requester must have appropriate permissions
	/// on both the source object and the target object. For example, to change
	/// Managers to MGRs, the entity making the request must have permission on
	/// Managers and MGRs, or must have permission on all (*). For more information
	/// about permissions, see [Permissions and Policies](http://docs.aws.amazon.com/I
	/// AM/latest/UserGuide/PermissionsAndPolicies.html).
	pub fn update_group(&self, input: &UpdateGroupRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateGroup");
		UpdateGroupRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns a list of users that are in the specified group. You can paginate the
	/// results using the `MaxItems` and `Marker` parameters.
	pub fn get_group(&self, input: &GetGroupRequest) -> Result<GetGroupResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetGroup");
		GetGroupRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetGroupResponseParser::parse_xml("GetGroupResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves the specified inline policy document that is embedded with the
	/// specified role.
	/// A role can also have managed policies attached to it. To retrieve a managed
	/// policy document that is attached to a role, use GetPolicy to determine the
	/// policy's default version, then use GetPolicyVersion to retrieve the policy
	/// document.
	/// For more information about policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	/// For more information about roles, go to [Using Roles to Delegate Permissions
	/// and Federate Identities](http://docs.aws.amazon.com/IAM/latest/UserGuide
	/// /roles-toplevel.html).
	pub fn get_role_policy(&self, input: &GetRolePolicyRequest) -> Result<GetRolePolicyResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetRolePolicy");
		GetRolePolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetRolePolicyResponseParser::parse_xml("GetRolePolicyResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the access key associated with the specified user.
	/// If you do not specify a user name, IAM determines the user name implicitly
	/// based on the AWS access key ID signing the request. Because this action works
	/// for access keys under the AWS account, you can use this action to manage root
	/// credentials even if the AWS account has no associated users.
	pub fn delete_access_key(&self, input: &DeleteAccessKeyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteAccessKey");
		DeleteAccessKeyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Adds the specified user to the specified group.
	pub fn add_user_to_group(&self, input: &AddUserToGroupRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "AddUserToGroup");
		AddUserToGroupRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Removes the specified role from the specified instance profile.
	/// Make sure you do not have any Amazon EC2 instances running with the role you
	/// are about to remove from the instance profile. Removing a role from an
	/// instance profile that is associated with a running instance will break any
	/// applications running on the instance.
	/// For more information about roles, go to [Working with
	/// Roles](http://docs.aws.amazon.com/IAM/latest/UserGuide/WorkingWithRoles.html).
	/// For more information about instance profiles, go to [About Instance Profiles](
	/// http://docs.aws.amazon.com/IAM/latest/UserGuide/AboutInstanceProfiles.html).
	pub fn remove_role_from_instance_profile(&self, input: &RemoveRoleFromInstanceProfileRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "RemoveRoleFromInstanceProfile");
		RemoveRoleFromInstanceProfileRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the instance profiles that have the specified path prefix. If there are
	/// none, the action returns an empty list. For more information about instance
	/// profiles, go to [About Instance Profiles](http://docs.aws.amazon.com/IAM/lates
	/// t/UserGuide/AboutInstanceProfiles.html).
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_instance_profiles(&self, input: &ListInstanceProfilesRequest) -> Result<ListInstanceProfilesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListInstanceProfiles");
		ListInstanceProfilesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListInstanceProfilesResponseParser::parse_xml("ListInstanceProfilesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists all managed policies that are attached to the specified user.
	/// A user can also have inline policies embedded with it. To list the inline
	/// policies for a user, use the ListUserPolicies API. For information about
	/// policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters. You
	/// can use the `PathPrefix` parameter to limit the list of policies to only those
	/// matching the specified path prefix. If there are no policies attached to the
	/// specified group (or none that match the specified path prefix), the action
	/// returns an empty list.
	pub fn list_attached_user_policies(&self, input: &ListAttachedUserPoliciesRequest) -> Result<ListAttachedUserPoliciesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListAttachedUserPolicies");
		ListAttachedUserPoliciesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListAttachedUserPoliciesResponseParser::parse_xml("ListAttachedUserPoliciesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a new version of the specified managed policy. To update a managed
	/// policy, you create a new policy version. A managed policy can have up to five
	/// versions. If the policy has five versions, you must delete an existing version
	/// using DeletePolicyVersion before you create a new version.
	/// Optionally, you can set the new version as the policy's default version. The
	/// default version is the operative version; that is, the version that is in
	/// effect for the IAM users, groups, and roles that the policy is attached to.
	/// For more information about managed policy versions, see [Versioning for
	/// Managed Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-versions.html) in the _Using IAM_ guide.
	pub fn create_policy_version(&self, input: &CreatePolicyVersionRequest) -> Result<CreatePolicyVersionResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreatePolicyVersion");
		CreatePolicyVersionRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreatePolicyVersionResponseParser::parse_xml("CreatePolicyVersionResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Attaches the specified managed policy to the specified user.
	/// You use this API to attach a managed policy to a user. To embed an inline
	/// policy in a user, use PutUserPolicy.
	/// For more information about policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn attach_user_policy(&self, input: &AttachUserPolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "AttachUserPolicy");
		AttachUserPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the virtual MFA devices under the AWS account by assignment status. If
	/// you do not specify an assignment status, the action returns a list of all
	/// virtual MFA devices. Assignment status can be `Assigned`, `Unassigned`, or
	/// `Any`.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_virtual_mfa_devices(&self, input: &ListVirtualMFADevicesRequest) -> Result<ListVirtualMFADevicesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListVirtualMFADevices");
		ListVirtualMFADevicesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListVirtualMFADevicesResponseParser::parse_xml("ListVirtualMFADevicesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Updates the policy that grants an entity permission to assume a role. For more
	/// information about roles, go to [Using Roles to Delegate Permissions and
	/// Federate Identities](http://docs.aws.amazon.com/IAM/latest/UserGuide/roles-
	/// toplevel.html).
	pub fn update_assume_role_policy(&self, input: &UpdateAssumeRolePolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateAssumeRolePolicy");
		UpdateAssumeRolePolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the server certificates that have the specified path prefix. If none
	/// exist, the action returns an empty list.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_server_certificates(&self, input: &ListServerCertificatesRequest) -> Result<ListServerCertificatesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListServerCertificates");
		ListServerCertificatesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListServerCertificatesResponseParser::parse_xml("ListServerCertificatesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified group. The group must not contain any users or have any
	/// attached policies.
	pub fn delete_group(&self, input: &DeleteGroupRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteGroup");
		DeleteGroupRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Replaces the existing list of server certificate thumbprints with a new list.
	/// The list that you pass with this action completely replaces the existing list
	/// of thumbprints. (The lists are not merged.)
	/// Typically, you need to update a thumbprint only when the identity provider's
	/// certificate changes, which occurs rarely. However, if the provider's
	/// certificate _does_ change, any attempt to assume an IAM role that specifies
	/// the OIDC provider as a principal will fail until the certificate thumbprint is
	/// updated.
	/// Because trust for the OpenID Connect provider is ultimately derived from the
	/// provider's certificate and is validated by the thumbprint, it is a best
	/// practice to limit access to the `UpdateOpenIDConnectProviderThumbprint` action
	/// to highly-privileged users.
	pub fn update_open_id_connect_provider_thumbprint(&self, input: &UpdateOpenIDConnectProviderThumbprintRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateOpenIDConnectProviderThumbprint");
		UpdateOpenIDConnectProviderThumbprintRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a new group.
	/// For information about the number of groups you can create, see [Limitations on
	/// IAM Entities](http://docs.aws.amazon.com/IAM/latest/UserGuide/LimitationsOnEnt
	/// ities.html) in the _Using IAM_ guide.
	pub fn create_group(&self, input: &CreateGroupRequest) -> Result<CreateGroupResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateGroup");
		CreateGroupRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateGroupResponseParser::parse_xml("CreateGroupResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified managed policy.
	/// Before you can delete a managed policy, you must detach the policy from all
	/// users, groups, and roles that it is attached to, and you must delete all of
	/// the policy's versions. The following steps describe the process for deleting a
	/// managed policy:
	///   1. Detach the policy from all users, groups, and roles that the policy is attached to, using the DetachUserPolicy, DetachGroupPolicy, or DetachRolePolicy APIs. To list all the users, groups, and roles that a policy is attached to, use ListEntitiesForPolicy. 
	///   2. Delete all versions of the policy using DeletePolicyVersion. To list the policy's versions, use ListPolicyVersions. You cannot use DeletePolicyVersion to delete the version that is marked as the default version. You delete the policy's default version in the next step of the process. 
	///   3. Delete the policy (this automatically deletes the policy's default version) using this API. 
	/// For information about managed policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn delete_policy(&self, input: &DeletePolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeletePolicy");
		DeletePolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified inline policy that is embedded in the specified user.
	/// A user can also have managed policies attached to it. To detach a managed
	/// policy from a user, use DetachUserPolicy. For more information about policies,
	/// refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn delete_user_policy(&self, input: &DeleteUserPolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteUserPolicy");
		DeleteUserPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Updates the name and/or the path of the specified server certificate.
	/// You should understand the implications of changing a server certificate's path
	/// or name. For more information, see [Managing Server Certificates](http://docs.
	/// aws.amazon.com/IAM/latest/UserGuide/ManagingServerCerts.html) in the _Using
	/// IAM_ guide.  To change a server certificate name the requester must have
	/// appropriate permissions on both the source object and the target object. For
	/// example, to change the name from ProductionCert to ProdCert, the entity making
	/// the request must have permission on ProductionCert and ProdCert, or must have
	/// permission on all (*). For more information about permissions, see
	/// [Permissions and Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/Per
	/// missionsAndPolicies.html).
	pub fn update_server_certificate(&self, input: &UpdateServerCertificateRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateServerCertificate");
		UpdateServerCertificateRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Adds a new client ID (also known as audience) to the list of client IDs
	/// already registered for the specified IAM OpenID Connect provider.
	/// This action is idempotent; it does not fail or return an error if you add an
	/// existing client ID to the provider.
	pub fn add_client_id_to_open_id_connect_provider(&self, input: &AddClientIDToOpenIDConnectProviderRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "AddClientIDToOpenIDConnectProvider");
		AddClientIDToOpenIDConnectProviderRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes a virtual MFA device.
	/// You must deactivate a user's virtual MFA device before you can delete it. For
	/// information about deactivating MFA devices, see DeactivateMFADevice.
	pub fn delete_virtual_mfa_device(&self, input: &DeleteVirtualMFADeviceRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteVirtualMFADevice");
		DeleteVirtualMFADeviceRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns information about the access key IDs associated with the specified
	/// user. If there are none, the action returns an empty list.
	/// Although each user is limited to a small number of keys, you can still
	/// paginate the results using the `MaxItems` and `Marker` parameters.
	/// If the `UserName` field is not specified, the UserName is determined
	/// implicitly based on the AWS access key ID used to sign the request. Because
	/// this action works for access keys under the AWS account, you can use this
	/// action to manage root credentials even if the AWS account has no associated
	/// users.
	/// To ensure the security of your AWS account, the secret access key is
	/// accessible only during key and user creation.
	pub fn list_access_keys(&self, input: &ListAccessKeysRequest) -> Result<ListAccessKeysResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListAccessKeys");
		ListAccessKeysRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListAccessKeysResponseParser::parse_xml("ListAccessKeysResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves the specified SSH public key, including metadata about the key.
	/// The SSH public key retrieved by this action is used only for authenticating
	/// the associated IAM user to an AWS CodeCommit repository. For more information
	/// about using SSH keys to authenticate to an AWS CodeCommit repository, see [Set
	/// up AWS CodeCommit for SSH
	/// Connections](http://docs.aws.amazon.com/codecommit/latest/userguide/setting-
	/// up-credentials-ssh.html) in the _AWS CodeCommit User Guide_.
	pub fn get_ssh_public_key(&self, input: &GetSSHPublicKeyRequest) -> Result<GetSSHPublicKeyResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetSSHPublicKey");
		GetSSHPublicKeyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetSSHPublicKeyResponseParser::parse_xml("GetSSHPublicKeyResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns information about the SSH public keys associated with the specified
	/// IAM user. If there are none, the action returns an empty list.
	/// The SSH public keys returned by this action are used only for authenticating
	/// the IAM user to an AWS CodeCommit repository. For more information about using
	/// SSH keys to authenticate to an AWS CodeCommit repository, see [Set up AWS
	/// CodeCommit for SSH
	/// Connections](http://docs.aws.amazon.com/codecommit/latest/userguide/setting-
	/// up-credentials-ssh.html) in the _AWS CodeCommit User Guide_.
	/// Although each user is limited to a small number of keys, you can still
	/// paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_ssh_public_keys(&self, input: &ListSSHPublicKeysRequest) -> Result<ListSSHPublicKeysResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListSSHPublicKeys");
		ListSSHPublicKeysRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListSSHPublicKeysResponseParser::parse_xml("ListSSHPublicKeysResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified signing certificate associated with the specified user.
	/// If you do not specify a user name, IAM determines the user name implicitly
	/// based on the AWS access key ID signing the request. Because this action works
	/// for access keys under the AWS account, you can use this action to manage root
	/// credentials even if the AWS account has no associated users.
	pub fn delete_signing_certificate(&self, input: &DeleteSigningCertificateRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteSigningCertificate");
		DeleteSigningCertificateRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves information about the specified version of the specified managed
	/// policy, including the policy document.
	/// To list the available versions for a policy, use ListPolicyVersions.
	/// This API retrieves information about managed policies. To retrieve information
	/// about an inline policy that is embedded in a user, group, or role, use the
	/// GetUserPolicy, GetGroupPolicy, or GetRolePolicy API.
	/// For more information about the types of policies, refer to [Managed Policies
	/// and Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-vs-inline.html) in the _Using IAM_ guide.
	pub fn get_policy_version(&self, input: &GetPolicyVersionRequest) -> Result<GetPolicyVersionResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetPolicyVersion");
		GetPolicyVersionRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetPolicyVersionResponseParser::parse_xml("GetPolicyVersionResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Adds (or updates) an inline policy document that is embedded in the specified
	/// role.
	/// When you embed an inline policy in a role, the inline policy is used as the
	/// role's access (permissions) policy. The role's trust policy is created at the
	/// same time as the role, using CreateRole. You can update a role's trust policy
	/// using UpdateAssumeRolePolicy. For more information about roles, go to [Using
	/// Roles to Delegate Permissions and Federate
	/// Identities](http://docs.aws.amazon.com/IAM/latest/UserGuide/roles-
	/// toplevel.html).
	/// A role can also have a managed policy attached to it. To attach a managed
	/// policy to a role, use AttachRolePolicy. To create a new managed policy, use
	/// CreatePolicy. For information about policies, refer to [Managed Policies and
	/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-vs-inline.html) in the _Using IAM_ guide.
	/// For information about limits on the number of inline policies that you can
	/// embed with a role, see [Limitations on IAM Entities](http://docs.aws.amazon.co
	/// m/IAM/latest/UserGuide/LimitationsOnEntities.html) in the _Using IAM_ guide.
	/// Because policy documents can be large, you should use POST rather than GET
	/// when calling `PutRolePolicy`. For general information about using the Query
	/// API with IAM, go to [Making Query Requests](http://docs.aws.amazon.com/IAM/lat
	/// est/UserGuide/IAM_UsingQueryAPI.html) in the _Using IAM_ guide.
	pub fn put_role_policy(&self, input: &PutRolePolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "PutRolePolicy");
		PutRolePolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Sets the specified version of the specified policy as the policy's default
	/// (operative) version.
	/// This action affects all users, groups, and roles that the policy is attached
	/// to. To list the users, groups, and roles that the policy is attached to, use
	/// the ListEntitiesForPolicy API.
	/// For information about managed policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn set_default_policy_version(&self, input: &SetDefaultPolicyVersionRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SetDefaultPolicyVersion");
		SetDefaultPolicyVersionRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves information about when the specified access key was last used. The
	/// information includes the date and time of last use, along with the AWS service
	/// and region that were specified in the last request made with that key.
	pub fn get_access_key_last_used(&self, input: &GetAccessKeyLastUsedRequest) -> Result<GetAccessKeyLastUsedResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetAccessKeyLastUsed");
		GetAccessKeyLastUsedRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetAccessKeyLastUsedResponseParser::parse_xml("GetAccessKeyLastUsedResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the MFA devices. If the request includes the user name, then this action
	/// lists all the MFA devices associated with the specified user name. If you do
	/// not specify a user name, IAM determines the user name implicitly based on the
	/// AWS access key ID signing the request.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_mfa_devices(&self, input: &ListMFADevicesRequest) -> Result<ListMFADevicesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListMFADevices");
		ListMFADevicesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListMFADevicesResponseParser::parse_xml("ListMFADevicesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Removes the specified managed policy from the specified group.
	/// A group can also have inline policies embedded with it. To delete an inline
	/// policy, use the DeleteGroupPolicy API. For information about policies, refer
	/// to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn detach_group_policy(&self, input: &DetachGroupPolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DetachGroupPolicy");
		DetachGroupPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Sets the status of the specified SSH public key to active or inactive. SSH
	/// public keys that are inactive cannot be used for authentication. This action
	/// can be used to disable a user's SSH public key as part of a key rotation work
	/// flow.
	/// The SSH public key affected by this action is used only for authenticating the
	/// associated IAM user to an AWS CodeCommit repository. For more information
	/// about using SSH keys to authenticate to an AWS CodeCommit repository, see [Set
	/// up AWS CodeCommit for SSH
	/// Connections](http://docs.aws.amazon.com/codecommit/latest/userguide/setting-
	/// up-credentials-ssh.html) in the _AWS CodeCommit User Guide_.
	pub fn update_ssh_public_key(&self, input: &UpdateSSHPublicKeyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateSSHPublicKey");
		UpdateSSHPublicKeyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves information about the specified instance profile, including the
	/// instance profile's path, GUID, ARN, and role. For more information about
	/// instance profiles, go to [About Instance Profiles](http://docs.aws.amazon.com/
	/// IAM/latest/UserGuide/AboutInstanceProfiles.html). For more information about
	/// ARNs, go to [ARNs](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Ident
	/// ifiers.html#Identifiers_ARNs).
	pub fn get_instance_profile(&self, input: &GetInstanceProfileRequest) -> Result<GetInstanceProfileResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetInstanceProfile");
		GetInstanceProfileRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetInstanceProfileResponseParser::parse_xml("GetInstanceProfileResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified user. The user must not belong to any groups, have any
	/// keys or signing certificates, or have any attached policies.
	pub fn delete_user(&self, input: &DeleteUserRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteUser");
		DeleteUserRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the IAM users that have the specified path prefix. If no path prefix is
	/// specified, the action returns all users in the AWS account. If there are none,
	/// the action returns an empty list.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_users(&self, input: &ListUsersRequest) -> Result<ListUsersResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListUsers");
		ListUsersRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListUsersResponseParser::parse_xml("ListUsersResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the password for the specified user, which terminates the user's
	/// ability to access AWS services through the AWS Management Console.
	/// Deleting a user's password does not prevent a user from accessing IAM through
	/// the command line interface or the API. To prevent all user access you must
	/// also either make the access key inactive or delete it. For more information
	/// about making keys inactive or deleting them, see UpdateAccessKey and
	/// DeleteAccessKey.
	pub fn delete_login_profile(&self, input: &DeleteLoginProfileRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteLoginProfile");
		DeleteLoginProfileRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists information about the versions of the specified managed policy,
	/// including the version that is set as the policy's default version.
	/// For more information about managed policies, refer to [Managed Policies and
	/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-vs-inline.html) in the _Using IAM_ guide.
	pub fn list_policy_versions(&self, input: &ListPolicyVersionsRequest) -> Result<ListPolicyVersionsResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListPolicyVersions");
		ListPolicyVersionsRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListPolicyVersionsResponseParser::parse_xml("ListPolicyVersionsResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Attaches the specified managed policy to the specified group.
	/// You use this API to attach a managed policy to a group. To embed an inline
	/// policy in a group, use PutGroupPolicy.
	/// For more information about policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn attach_group_policy(&self, input: &AttachGroupPolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "AttachGroupPolicy");
		AttachGroupPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a new managed policy for your AWS account.
	/// This operation creates a policy version with a version identifier of `v1` and
	/// sets v1 as the policy's default version. For more information about policy
	/// versions, see [Versioning for Managed
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-
	/// versions.html) in the _Using IAM_ guide.
	/// For more information about managed policies in general, refer to [Managed
	/// Policies and Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide
	/// /policies-managed-vs-inline.html) in the _Using IAM_ guide.
	pub fn create_policy(&self, input: &CreatePolicyRequest) -> Result<CreatePolicyResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreatePolicy");
		CreatePolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreatePolicyResponseParser::parse_xml("CreatePolicyResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates an IAM entity to describe an identity provider (IdP) that supports
	/// [OpenID Connect (OIDC)](http://openid.net/connect/).
	/// The OIDC provider that you create with this operation can be used as a
	/// principal in a role's trust policy to establish a trust relationship between
	/// AWS and the OIDC provider.
	/// When you create the IAM OIDC provider, you specify the URL of the OIDC
	/// identity provider (IdP) to trust, a list of client IDs (also known as
	/// audiences) that identify the application or applications that are allowed to
	/// authenticate using the OIDC provider, and a list of thumbprints of the server
	/// certificate(s) that the IdP uses. You get all of this information from the
	/// OIDC IdP that you want to use for access to AWS.
	/// Because trust for the OIDC provider is ultimately derived from the IAM
	/// provider that this action creates, it is a best practice to limit access to
	/// the CreateOpenIDConnectProvider action to highly-privileged users.
	pub fn create_open_id_connect_provider(&self, input: &CreateOpenIDConnectProviderRequest) -> Result<CreateOpenIDConnectProviderResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateOpenIDConnectProvider");
		CreateOpenIDConnectProviderRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateOpenIDConnectProviderResponseParser::parse_xml("CreateOpenIDConnectProviderResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Removes the specified managed policy from the specified user.
	/// A user can also have inline policies embedded with it. To delete an inline
	/// policy, use the DeleteUserPolicy API. For information about policies, refer to
	/// [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn detach_user_policy(&self, input: &DetachUserPolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DetachUserPolicy");
		DetachUserPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the groups the specified user belongs to.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_groups_for_user(&self, input: &ListGroupsForUserRequest) -> Result<ListGroupsForUserResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListGroupsForUser");
		ListGroupsForUserRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListGroupsForUserResponseParser::parse_xml("ListGroupsForUserResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates an alias for your AWS account. For information about using an AWS
	/// account alias, see [Using an Alias for Your AWS Account
	/// ID](http://docs.aws.amazon.com/IAM/latest/UserGuide/AccountAlias.html) in the
	/// _Using IAM_ guide.
	pub fn create_account_alias(&self, input: &CreateAccountAliasRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateAccountAlias");
		CreateAccountAliasRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves information about all IAM users, groups, roles, and policies in your
	/// account, including their relationships to one another. Use this API to obtain
	/// a snapshot of the configuration of IAM permissions (users, groups, roles, and
	/// policies) in your account.
	/// You can optionally filter the results using the `Filter` parameter. You can
	/// paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn get_account_authorization_details(&self, input: &GetAccountAuthorizationDetailsRequest) -> Result<GetAccountAuthorizationDetailsResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetAccountAuthorizationDetails");
		GetAccountAuthorizationDetailsRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetAccountAuthorizationDetailsResponseParser::parse_xml("GetAccountAuthorizationDetailsResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns the SAML provider metadocument that was uploaded when the provider was
	/// created or updated.
	/// This operation requires [Signature Version
	/// 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html).
	pub fn get_saml_provider(&self, input: &GetSAMLProviderRequest) -> Result<GetSAMLProviderResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetSAMLProvider");
		GetSAMLProviderRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetSAMLProviderResponseParser::parse_xml("GetSAMLProviderResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a new role for your AWS account. For more information about roles, go
	/// to [Working with
	/// Roles](http://docs.aws.amazon.com/IAM/latest/UserGuide/WorkingWithRoles.html).
	/// For information about limitations on role names and the number of roles you
	/// can create, go to [Limitations on IAM Entities](http://docs.aws.amazon.com/IAM
	/// /latest/UserGuide/LimitationsOnEntities.html) in the _Using IAM_ guide.
	/// The policy in the following example grants permission to an EC2 instance to
	/// assume the role.
	pub fn create_role(&self, input: &CreateRoleRequest) -> Result<CreateRoleResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateRole");
		CreateRoleRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateRoleResponseParser::parse_xml("CreateRoleResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists all managed policies that are attached to the specified role.
	/// A role can also have inline policies embedded with it. To list the inline
	/// policies for a role, use the ListRolePolicies API. For information about
	/// policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters. You
	/// can use the `PathPrefix` parameter to limit the list of policies to only those
	/// matching the specified path prefix. If there are no policies attached to the
	/// specified role (or none that match the specified path prefix), the action
	/// returns an empty list.
	pub fn list_attached_role_policies(&self, input: &ListAttachedRolePoliciesRequest) -> Result<ListAttachedRolePoliciesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListAttachedRolePolicies");
		ListAttachedRolePoliciesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListAttachedRolePoliciesResponseParser::parse_xml("ListAttachedRolePoliciesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Enables the specified MFA device and associates it with the specified user
	/// name. When enabled, the MFA device is required for every subsequent login by
	/// the user name associated with the device.
	pub fn enable_mfa_device(&self, input: &EnableMFADeviceRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "EnableMFADevice");
		EnableMFADeviceRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes a SAML provider.
	/// Deleting the provider does not update any roles that reference the SAML
	/// provider as a principal in their trust policies. Any attempt to assume a role
	/// that references a SAML provider that has been deleted will fail.
	/// This operation requires [Signature Version
	/// 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html).
	pub fn delete_saml_provider(&self, input: &DeleteSAMLProviderRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteSAMLProvider");
		DeleteSAMLProviderRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the names of the inline policies embedded in the specified user.
	/// A user can also have managed policies attached to it. To list the managed
	/// policies that are attached to a user, use ListAttachedUserPolicies. For more
	/// information about policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters. If
	/// there are no inline policies embedded with the specified user, the action
	/// returns an empty list.
	pub fn list_user_policies(&self, input: &ListUserPoliciesRequest) -> Result<ListUserPoliciesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListUserPolicies");
		ListUserPoliciesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListUserPoliciesResponseParser::parse_xml("ListUserPoliciesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the names of the inline policies that are embedded in the specified
	/// role.
	/// A role can also have managed policies attached to it. To list the managed
	/// policies that are attached to a role, use ListAttachedRolePolicies. For more
	/// information about policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters. If
	/// there are no inline policies embedded with the specified role, the action
	/// returns an empty list.
	pub fn list_role_policies(&self, input: &ListRolePoliciesRequest) -> Result<ListRolePoliciesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListRolePolicies");
		ListRolePoliciesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListRolePoliciesResponseParser::parse_xml("ListRolePoliciesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Adds (or updates) an inline policy document that is embedded in the specified
	/// group.
	/// A user can also have managed policies attached to it. To attach a managed
	/// policy to a group, use AttachGroupPolicy. To create a new managed policy, use
	/// CreatePolicy. For information about policies, refer to [Managed Policies and
	/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-vs-inline.html) in the _Using IAM_ guide.
	/// For information about limits on the number of inline policies that you can
	/// embed in a group, see [Limitations on IAM Entities](http://docs.aws.amazon.com
	/// /IAM/latest/UserGuide/LimitationsOnEntities.html) in the _Using IAM_ guide.
	/// Because policy documents can be large, you should use POST rather than GET
	/// when calling `PutGroupPolicy`. For general information about using the Query
	/// API with IAM, go to [Making Query Requests](http://docs.aws.amazon.com/IAM/lat
	/// est/UserGuide/IAM_UsingQueryAPI.html) in the _Using IAM_ guide.
	pub fn put_group_policy(&self, input: &PutGroupPolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "PutGroupPolicy");
		PutGroupPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Uploads an X.509 signing certificate and associates it with the specified
	/// user. Some AWS services use X.509 signing certificates to validate requests
	/// that are signed with a corresponding private key. When you upload the
	/// certificate, its default status is `Active`.
	/// If the `UserName` field is not specified, the user name is determined
	/// implicitly based on the AWS access key ID used to sign the request. Because
	/// this action works for access keys under the AWS account, you can use this
	/// action to manage root credentials even if the AWS account has no associated
	/// users.
	/// Because the body of a X.509 certificate can be large, you should use POST
	/// rather than GET when calling `UploadSigningCertificate`. For information about
	/// setting up signatures and authorization through the API, go to [Signing AWS
	/// API Requests](http://docs.aws.amazon.com/general/latest/gr/signing_aws_api_req
	/// uests.html) in the _AWS General Reference_. For general information about
	/// using the Query API with IAM, go to [Making Query Requests](http://docs.aws.am
	/// azon.com/IAM/latest/UserGuide/IAM_UsingQueryAPI.html) in the _Using IAM_guide.
	pub fn upload_signing_certificate(&self, input: &UploadSigningCertificateRequest) -> Result<UploadSigningCertificateResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UploadSigningCertificate");
		UploadSigningCertificateRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(UploadSigningCertificateResponseParser::parse_xml("UploadSigningCertificateResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the instance profiles that have the specified associated role. If there
	/// are none, the action returns an empty list. For more information about
	/// instance profiles, go to [About Instance Profiles](http://docs.aws.amazon.com/
	/// IAM/latest/UserGuide/AboutInstanceProfiles.html).
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_instance_profiles_for_role(&self, input: &ListInstanceProfilesForRoleRequest) -> Result<ListInstanceProfilesForRoleResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListInstanceProfilesForRole");
		ListInstanceProfilesForRoleRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListInstanceProfilesForRoleResponseParser::parse_xml("ListInstanceProfilesForRoleResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Changes the password of the IAM user who is calling this action. The root
	/// account password is not affected by this action.
	/// To change the password for a different user, see UpdateLoginProfile. For more
	/// information about modifying passwords, see [Managing Passwords](http://docs.aw
	/// s.amazon.com/IAM/latest/UserGuide/Using_ManagingLogins.html) in the _Using
	/// IAM_ guide.
	pub fn change_password(&self, input: &ChangePasswordRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ChangePassword");
		ChangePasswordRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists all users, groups, and roles that the specified managed policy is
	/// attached to.
	/// You can use the optional `EntityFilter` parameter to limit the results to a
	/// particular type of entity (users, groups, or roles). For example, to list only
	/// the roles that are attached to the specified policy, set `EntityFilter` to
	/// `Role`.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	pub fn list_entities_for_policy(&self, input: &ListEntitiesForPolicyRequest) -> Result<ListEntitiesForPolicyResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListEntitiesForPolicy");
		ListEntitiesForPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListEntitiesForPolicyResponseParser::parse_xml("ListEntitiesForPolicyResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves the user name and password-creation date for the specified user. If
	/// the user has not been assigned a password, the action returns a 404
	/// (`NoSuchEntity`) error.
	pub fn get_login_profile(&self, input: &GetLoginProfileRequest) -> Result<GetLoginProfileResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetLoginProfile");
		GetLoginProfileRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetLoginProfileResponseParser::parse_xml("GetLoginProfileResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a new virtual MFA device for the AWS account. After creating the
	/// virtual MFA, use EnableMFADevice to attach the MFA device to an IAM user. For
	/// more information about creating and working with virtual MFA devices, go to
	/// [Using a Virtual MFA
	/// Device](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_VirtualMFA.html)
	/// in the _Using IAM_ guide.
	/// For information about limits on the number of MFA devices you can create, see
	/// [Limitations on Entities](http://docs.aws.amazon.com/IAM/latest/UserGuide/Limi
	/// tationsOnEntities.html) in the _Using IAM_ guide.
	/// The seed information contained in the QR code and the Base32 string should be
	/// treated like any other secret access information, such as your AWS access keys
	/// or your passwords. After you provision your virtual device, you should ensure
	/// that the information is destroyed following secure procedures.
	pub fn create_virtual_mfa_device(&self, input: &CreateVirtualMFADeviceRequest) -> Result<CreateVirtualMFADeviceResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateVirtualMFADevice");
		CreateVirtualMFADeviceRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateVirtualMFADeviceResponseParser::parse_xml("CreateVirtualMFADeviceResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves information about the specified server certificate.
	pub fn get_server_certificate(&self, input: &GetServerCertificateRequest) -> Result<GetServerCertificateResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetServerCertificate");
		GetServerCertificateRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetServerCertificateResponseParser::parse_xml("GetServerCertificateResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Attaches the specified managed policy to the specified role.
	/// When you attach a managed policy to a role, the managed policy is used as the
	/// role's access (permissions) policy. You cannot use a managed policy as the
	/// role's trust policy. The role's trust policy is created at the same time as
	/// the role, using CreateRole. You can update a role's trust policy using
	/// UpdateAssumeRolePolicy.
	/// Use this API to attach a managed policy to a role. To embed an inline policy
	/// in a role, use PutRolePolicy. For more information about policies, refer to
	/// [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn attach_role_policy(&self, input: &AttachRolePolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "AttachRolePolicy");
		AttachRolePolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Removes the specified client ID (also known as audience) from the list of
	/// client IDs registered for the specified IAM OpenID Connect provider.
	/// This action is idempotent; it does not fail or return an error if you try to
	/// remove a client ID that was removed previously.
	pub fn remove_client_id_from_open_id_connect_provider(&self, input: &RemoveClientIDFromOpenIDConnectProviderRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "RemoveClientIDFromOpenIDConnectProvider");
		RemoveClientIDFromOpenIDConnectProviderRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Updates the password policy settings for the AWS account.
	/// This action does not support partial updates. No parameters are required, but
	/// if you do not specify a parameter, that parameter's value reverts to its
	/// default value. See the **Request Parameters** section for each parameter's
	/// default value.
	/// For more information about using a password policy, see [Managing an IAM
	/// Password Policy](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Managin
	/// gPasswordPolicies.html) in the _Using IAM_ guide.
	pub fn update_account_password_policy(&self, input: &UpdateAccountPasswordPolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateAccountPasswordPolicy");
		UpdateAccountPasswordPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves information about the specified role, including the role's path,
	/// GUID, ARN, and the policy granting permission to assume the role. For more
	/// information about ARNs, go to [ARNs](http://docs.aws.amazon.com/IAM/latest/Use
	/// rGuide/Using_Identifiers.html#Identifiers_ARNs). For more information about
	/// roles, go to [Working with
	/// Roles](http://docs.aws.amazon.com/IAM/latest/UserGuide/WorkingWithRoles.html).
	pub fn get_role(&self, input: &GetRoleRequest) -> Result<GetRoleResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetRole");
		GetRoleRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetRoleResponseParser::parse_xml("GetRoleResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified version of the specified managed policy.
	/// You cannot delete the default version of a policy using this API. To delete
	/// the default version of a policy, use DeletePolicy. To find out which version
	/// of a policy is marked as the default version, use ListPolicyVersions.
	/// For information about versions for managed policies, refer to [Versioning for
	/// Managed Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-versions.html) in the _Using IAM_ guide.
	pub fn delete_policy_version(&self, input: &DeletePolicyVersionRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeletePolicyVersion");
		DeletePolicyVersionRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists all the managed policies that are available to your account, including
	/// your own customer managed policies and all AWS managed policies.
	/// You can filter the list of policies that is returned using the optional
	/// `OnlyAttached`, `Scope`, and `PathPrefix` parameters. For example, to list
	/// only the customer managed policies in your AWS account, set `Scope` to
	/// `Local`. To list only AWS managed policies, set `Scope` to `AWS`.
	/// You can paginate the results using the `MaxItems` and `Marker` parameters.
	/// For more information about managed policies, refer to [Managed Policies and
	/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-vs-inline.html) in the _Using IAM_ guide.
	pub fn list_policies(&self, input: &ListPoliciesRequest) -> Result<ListPoliciesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListPolicies");
		ListPoliciesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListPoliciesResponseParser::parse_xml("ListPoliciesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Adds (or updates) an inline policy document that is embedded in the specified
	/// user.
	/// A user can also have a managed policy attached to it. To attach a managed
	/// policy to a user, use AttachUserPolicy. To create a new managed policy, use
	/// CreatePolicy. For information about policies, refer to [Managed Policies and
	/// Inline Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-
	/// managed-vs-inline.html) in the _Using IAM_ guide.
	/// For information about limits on the number of inline policies that you can
	/// embed in a user, see [Limitations on IAM Entities](http://docs.aws.amazon.com/
	/// IAM/latest/UserGuide/LimitationsOnEntities.html) in the _Using IAM_ guide.
	/// Because policy documents can be large, you should use POST rather than GET
	/// when calling `PutUserPolicy`. For general information about using the Query
	/// API with IAM, go to [Making Query Requests](http://docs.aws.amazon.com/IAM/lat
	/// est/UserGuide/IAM_UsingQueryAPI.html) in the _Using IAM_ guide.
	pub fn put_user_policy(&self, input: &PutUserPolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "PutUserPolicy");
		PutUserPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Synchronizes the specified MFA device with AWS servers.
	/// For more information about creating and working with virtual MFA devices, go
	/// to [Using a Virtual MFA
	/// Device](http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_VirtualMFA.html)
	/// in the _Using IAM_ guide.
	pub fn resync_mfa_device(&self, input: &ResyncMFADeviceRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ResyncMFADevice");
		ResyncMFADeviceRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified server certificate.
	/// If you are using a server certificate with Elastic Load Balancing, deleting
	/// the certificate could have implications for your application. If Elastic Load
	/// Balancing doesn't detect the deletion of bound certificates, it may continue
	/// to use the certificates. This could cause Elastic Load Balancing to stop
	/// accepting traffic. We recommend that you remove the reference to the
	/// certificate from Elastic Load Balancing before using this command to delete
	/// the certificate. For more information, go to [DeleteLoadBalancerListeners](htt
	/// p://docs.aws.amazon.com/ElasticLoadBalancing/latest/APIReference/API_DeleteLoa
	/// dBalancerListeners.html) in the _Elastic Load Balancing API Reference_.
	pub fn delete_server_certificate(&self, input: &DeleteServerCertificateRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteServerCertificate");
		DeleteServerCertificateRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a new instance profile. For information about instance profiles, go to
	/// [About Instance Profiles](http://docs.aws.amazon.com/IAM/latest/UserGuide/Abou
	/// tInstanceProfiles.html).
	/// For information about the number of instance profiles you can create, see
	/// [Limitations on IAM Entities](http://docs.aws.amazon.com/IAM/latest/UserGuide/
	/// LimitationsOnEntities.html) in the _Using IAM_ guide.
	pub fn create_instance_profile(&self, input: &CreateInstanceProfileRequest) -> Result<CreateInstanceProfileResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateInstanceProfile");
		CreateInstanceProfileRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateInstanceProfileResponseParser::parse_xml("CreateInstanceProfileResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes an IAM OpenID Connect identity provider.
	/// Deleting an OIDC provider does not update any roles that reference the
	/// provider as a principal in their trust policies. Any attempt to assume a role
	/// that references a provider that has been deleted will fail.
	/// This action is idempotent; it does not fail or return an error if you call the
	/// action for a provider that was already deleted.
	pub fn delete_open_id_connect_provider(&self, input: &DeleteOpenIDConnectProviderRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteOpenIDConnectProvider");
		DeleteOpenIDConnectProviderRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified inline policy that is embedded in the specified group.
	/// A group can also have managed policies attached to it. To detach a managed
	/// policy from a group, use DetachGroupPolicy. For more information about
	/// policies, refer to [Managed Policies and Inline
	/// Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-
	/// inline.html) in the _Using IAM_ guide.
	pub fn delete_group_policy(&self, input: &DeleteGroupPolicyRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "iam", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteGroupPolicy");
		DeleteGroupPolicyRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}
