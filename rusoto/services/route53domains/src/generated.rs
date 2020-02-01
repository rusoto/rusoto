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
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Information for one billing record.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BillingRecord {
    /// <p>The date that the operation was billed, in Unix format.</p>
    #[serde(rename = "BillDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_date: Option<f64>,
    /// <p>The name of the domain that the billing record applies to. If the domain name contains characters other than a-z, 0-9, and - (hyphen), such as an internationalized domain name, then this value is in Punycode. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DomainNameFormat.html">DNS Domain Name Format</a> in the <i>Amazon Route 53 Developer Guidezzz</i>.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The ID of the invoice that is associated with the billing record.</p>
    #[serde(rename = "InvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// <p>The operation that you were charged for.</p>
    #[serde(rename = "Operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// <p>The price that you were charged for the operation, in US dollars.</p> <p>Example value: 12.0</p>
    #[serde(rename = "Price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}

/// <p>The CheckDomainAvailability request contains the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CheckDomainAvailabilityRequest {
    /// <p>The name of the domain that you want to get availability for.</p> <p>Constraints: The domain name can contain only the letters a through z, the numbers 0 through 9, and hyphen (-). Internationalized Domain Names are not supported.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "IdnLangCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idn_lang_code: Option<String>,
}

/// <p>The CheckDomainAvailability response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CheckDomainAvailabilityResponse {
    /// <p><p>Whether the domain name is available for registering.</p> <note> <p>You can register only domains designated as <code>AVAILABLE</code>.</p> </note> <p>Valid values:</p> <dl> <dt>AVAILABLE</dt> <dd> <p>The domain name is available.</p> </dd> <dt>AVAILABLE<em>RESERVED</dt> <dd> <p>The domain name is reserved under specific conditions.</p> </dd> <dt>AVAILABLE</em>PREORDER</dt> <dd> <p>The domain name is available and can be preordered.</p> </dd> <dt>DONT<em>KNOW</dt> <dd> <p>The TLD registry didn&#39;t reply with a definitive answer about whether the domain name is available. Amazon Route 53 can return this response for a variety of reasons, for example, the registry is performing maintenance. Try again later.</p> </dd> <dt>PENDING</dt> <dd> <p>The TLD registry didn&#39;t return a response in the expected amount of time. When the response is delayed, it usually takes just a few extra seconds. You can resubmit the request immediately.</p> </dd> <dt>RESERVED</dt> <dd> <p>The domain name has been reserved for another person or organization.</p> </dd> <dt>UNAVAILABLE</dt> <dd> <p>The domain name is not available.</p> </dd> <dt>UNAVAILABLE</em>PREMIUM</dt> <dd> <p>The domain name is not available.</p> </dd> <dt>UNAVAILABLE_RESTRICTED</dt> <dd> <p>The domain name is forbidden.</p> </dd> </dl></p>
    #[serde(rename = "Availability")]
    pub availability: String,
}

/// <p>The CheckDomainTransferability request contains the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CheckDomainTransferabilityRequest {
    /// <p>If the registrar for the top-level domain (TLD) requires an authorization code to transfer the domain, the code that you got from the current registrar for the domain.</p>
    #[serde(rename = "AuthCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    /// <p>The name of the domain that you want to transfer to Amazon Route 53.</p> <p>Constraints: The domain name can contain only the letters a through z, the numbers 0 through 9, and hyphen (-). Internationalized Domain Names are not supported.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The CheckDomainTransferability response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CheckDomainTransferabilityResponse {
    /// <p>A complex type that contains information about whether the specified domain can be transferred to Amazon Route 53.</p>
    #[serde(rename = "Transferability")]
    pub transferability: DomainTransferability,
}

/// <p>ContactDetail includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactDetail {
    /// <p>First line of the contact's address.</p>
    #[serde(rename = "AddressLine1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_1: Option<String>,
    /// <p>Second line of contact's address, if any.</p>
    #[serde(rename = "AddressLine2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    /// <p>The city of the contact's address.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>Indicates whether the contact is a person, company, association, or public organization. If you choose an option other than <code>PERSON</code>, you must enter an organization name, and you can't enable privacy protection for the contact.</p>
    #[serde(rename = "ContactType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<String>,
    /// <p>Code for the country of the contact's address.</p>
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// <p>Email address of the contact.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>A list of name-value pairs for parameters required by certain top-level domains.</p>
    #[serde(rename = "ExtraParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<Vec<ExtraParam>>,
    /// <p>Fax number of the contact.</p> <p>Constraints: Phone number must be specified in the format "+[country dialing code].[number including any area code]". For example, a US phone number might appear as <code>"+1.1234567890"</code>.</p>
    #[serde(rename = "Fax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    /// <p>First name of contact.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>Last name of contact.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// <p>Name of the organization for contact types other than <code>PERSON</code>.</p>
    #[serde(rename = "OrganizationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    /// <p>The phone number of the contact.</p> <p>Constraints: Phone number must be specified in the format "+[country dialing code].[number including any area code&gt;]". For example, a US phone number might appear as <code>"+1.1234567890"</code>.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// <p>The state or province of the contact's city.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The zip or postal code of the contact's address.</p>
    #[serde(rename = "ZipCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

/// <p>The DeleteTagsForDomainRequest includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTagsForDomainRequest {
    /// <p>The domain for which you want to delete one or more tags.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>A list of tag keys to delete.</p>
    #[serde(rename = "TagsToDelete")]
    pub tags_to_delete: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTagsForDomainResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableDomainAutoRenewRequest {
    /// <p>The name of the domain that you want to disable automatic renewal for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableDomainAutoRenewResponse {}

/// <p>The DisableDomainTransferLock request includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableDomainTransferLockRequest {
    /// <p>The name of the domain that you want to remove the transfer lock for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The DisableDomainTransferLock response includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableDomainTransferLockResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>Information about one suggested domain name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainSuggestion {
    /// <p><p>Whether the domain name is available for registering.</p> <note> <p>You can register only the domains that are designated as <code>AVAILABLE</code>.</p> </note> <p>Valid values:</p> <dl> <dt>AVAILABLE</dt> <dd> <p>The domain name is available.</p> </dd> <dt>AVAILABLE<em>RESERVED</dt> <dd> <p>The domain name is reserved under specific conditions.</p> </dd> <dt>AVAILABLE</em>PREORDER</dt> <dd> <p>The domain name is available and can be preordered.</p> </dd> <dt>DONT<em>KNOW</dt> <dd> <p>The TLD registry didn&#39;t reply with a definitive answer about whether the domain name is available. Amazon Route 53 can return this response for a variety of reasons, for example, the registry is performing maintenance. Try again later.</p> </dd> <dt>PENDING</dt> <dd> <p>The TLD registry didn&#39;t return a response in the expected amount of time. When the response is delayed, it usually takes just a few extra seconds. You can resubmit the request immediately.</p> </dd> <dt>RESERVED</dt> <dd> <p>The domain name has been reserved for another person or organization.</p> </dd> <dt>UNAVAILABLE</dt> <dd> <p>The domain name is not available.</p> </dd> <dt>UNAVAILABLE</em>PREMIUM</dt> <dd> <p>The domain name is not available.</p> </dd> <dt>UNAVAILABLE_RESTRICTED</dt> <dd> <p>The domain name is forbidden.</p> </dd> </dl></p>
    #[serde(rename = "Availability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,
    /// <p>A suggested domain name.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

/// <p>Summary information about one domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainSummary {
    /// <p>Indicates whether the domain is automatically renewed upon expiration.</p>
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// <p>The name of the domain that the summary information applies to.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Expiration date of the domain in Coordinated Universal Time (UTC).</p>
    #[serde(rename = "Expiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<f64>,
    /// <p>Indicates whether a domain is locked from unauthorized transfer to another party.</p>
    #[serde(rename = "TransferLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_lock: Option<bool>,
}

/// <p>A complex type that contains information about whether the specified domain can be transferred to Amazon Route 53.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainTransferability {
    #[serde(rename = "Transferable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferable: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableDomainAutoRenewRequest {
    /// <p>The name of the domain that you want to enable automatic renewal for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableDomainAutoRenewResponse {}

/// <p>A request to set the transfer lock for the specified domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableDomainTransferLockRequest {
    /// <p>The name of the domain that you want to set the transfer lock for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The EnableDomainTransferLock response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableDomainTransferLockResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use GetOperationDetail.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>ExtraParam includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtraParam {
    /// <p>Name of the additional parameter required by the top-level domain. Here are the top-level domains that require additional parameters and which parameters they require:</p> <ul> <li> <p> <b>.com.au and .net.au:</b> <code>AU_ID_NUMBER</code> and <code>AU_ID_TYPE</code> </p> </li> <li> <p> <b>.ca:</b> <code>BRAND_NUMBER</code>, <code>CA_LEGAL_TYPE</code>, and <code>CA_BUSINESS_ENTITY_TYPE</code> </p> </li> <li> <p> <b>.es:</b> <code>ES_IDENTIFICATION</code>, <code>ES_IDENTIFICATION_TYPE</code>, and <code>ES_LEGAL_FORM</code> </p> </li> <li> <p> <b>.fi:</b> <code>BIRTH_DATE_IN_YYYY_MM_DD</code>, <code>FI_BUSINESS_NUMBER</code>, <code>FI_ID_NUMBER</code>, <code>FI_NATIONALITY</code>, and <code>FI_ORGANIZATION_TYPE</code> </p> </li> <li> <p> <b>.fr:</b> <code>BRAND_NUMBER</code>, <code>BIRTH_DEPARTMENT</code>, <code>BIRTH_DATE_IN_YYYY_MM_DD</code>, <code>BIRTH_COUNTRY</code>, and <code>BIRTH_CITY</code> </p> </li> <li> <p> <b>.it:</b> <code>BIRTH_COUNTRY</code>, <code>IT_PIN</code>, and <code>IT_REGISTRANT_ENTITY_TYPE</code> </p> </li> <li> <p> <b>.ru:</b> <code>BIRTH_DATE_IN_YYYY_MM_DD</code> and <code>RU_PASSPORT_DATA</code> </p> </li> <li> <p> <b>.se:</b> <code>BIRTH_COUNTRY</code> and <code>SE_ID_NUMBER</code> </p> </li> <li> <p> <b>.sg:</b> <code>SG_ID_NUMBER</code> </p> </li> <li> <p> <b>.co.uk, .me.uk, and .org.uk:</b> <code>UK_CONTACT_TYPE</code> and <code>UK_COMPANY_NUMBER</code> </p> </li> </ul> <p>In addition, many TLDs require <code>VAT_NUMBER</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Values corresponding to the additional parameter names required by some top-level domains.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetContactReachabilityStatusRequest {
    /// <p>The name of the domain for which you want to know whether the registrant contact has confirmed that the email address is valid.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetContactReachabilityStatusResponse {
    /// <p>The domain name for which you requested the reachability status.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p><p>Whether the registrant contact has responded. Values include the following:</p> <dl> <dt>PENDING</dt> <dd> <p>We sent the confirmation email and haven&#39;t received a response yet.</p> </dd> <dt>DONE</dt> <dd> <p>We sent the email and got confirmation from the registrant contact.</p> </dd> <dt>EXPIRED</dt> <dd> <p>The time limit expired before the registrant contact responded.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The GetDomainDetail request includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainDetailRequest {
    /// <p>The name of the domain that you want to get detailed information about.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The GetDomainDetail response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDomainDetailResponse {
    /// <p>Email address to contact to report incorrect contact information for a domain, to report that the domain is being used to send spam, to report that someone is cybersquatting on a domain name, or report some other type of abuse.</p>
    #[serde(rename = "AbuseContactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_contact_email: Option<String>,
    /// <p>Phone number for reporting abuse.</p>
    #[serde(rename = "AbuseContactPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_contact_phone: Option<String>,
    /// <p>Provides details about the domain administrative contact.</p>
    #[serde(rename = "AdminContact")]
    pub admin_contact: ContactDetail,
    /// <p>Specifies whether contact information is concealed from WHOIS queries. If the value is <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If the value is <code>false</code>, WHOIS queries return the information that you entered for the admin contact.</p>
    #[serde(rename = "AdminPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_privacy: Option<bool>,
    /// <p>Specifies whether the domain registration is set to renew automatically.</p>
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// <p>The date when the domain was created as found in the response to a WHOIS query. The date and time is in Coordinated Universal time (UTC).</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "DnsSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_sec: Option<String>,
    /// <p>The name of a domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The date when the registration for the domain is set to expire. The date and time is in Coordinated Universal time (UTC).</p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>The name of the domain.</p>
    #[serde(rename = "Nameservers")]
    pub nameservers: Vec<Nameserver>,
    /// <p>Provides details about the domain registrant.</p>
    #[serde(rename = "RegistrantContact")]
    pub registrant_contact: ContactDetail,
    /// <p>Specifies whether contact information is concealed from WHOIS queries. If the value is <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If the value is <code>false</code>, WHOIS queries return the information that you entered for the registrant contact (domain owner).</p>
    #[serde(rename = "RegistrantPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_privacy: Option<bool>,
    /// <p>Name of the registrar of the domain as identified in the registry. Domains with a .com, .net, or .org TLD are registered by Amazon Registrar. All other domains are registered by our registrar associate, Gandi. The value for domains that are registered by Gandi is <code>"GANDI SAS"</code>. </p>
    #[serde(rename = "RegistrarName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_name: Option<String>,
    /// <p>Web address of the registrar.</p>
    #[serde(rename = "RegistrarUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_url: Option<String>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "RegistryDomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_domain_id: Option<String>,
    /// <p>Reseller of the domain. Domains registered or transferred using Amazon Route 53 domains will have <code>"Amazon"</code> as the reseller. </p>
    #[serde(rename = "Reseller")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller: Option<String>,
    /// <p>An array of domain name status codes, also known as Extensible Provisioning Protocol (EPP) status codes.</p> <p>ICANN, the organization that maintains a central database of domain names, has developed a set of domain name status codes that tell you the status of a variety of operations on a domain name, for example, registering a domain name, transferring a domain name to another registrar, renewing the registration for a domain name, and so on. All registrars use this same set of status codes.</p> <p>For a current list of domain name status codes and an explanation of what each code means, go to the <a href="https://www.icann.org/">ICANN website</a> and search for <code>epp status codes</code>. (Search on the ICANN website; web searches sometimes return an old version of the document.)</p>
    #[serde(rename = "StatusList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_list: Option<Vec<String>>,
    /// <p>Provides details about the domain technical contact.</p>
    #[serde(rename = "TechContact")]
    pub tech_contact: ContactDetail,
    /// <p>Specifies whether contact information is concealed from WHOIS queries. If the value is <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If the value is <code>false</code>, WHOIS queries return the information that you entered for the technical contact.</p>
    #[serde(rename = "TechPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_privacy: Option<bool>,
    /// <p>The last updated date of the domain as found in the response to a WHOIS query. The date and time is in Coordinated Universal time (UTC).</p>
    #[serde(rename = "UpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<f64>,
    /// <p>The fully qualified name of the WHOIS server that can answer the WHOIS query for the domain.</p>
    #[serde(rename = "WhoIsServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_is_server: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainSuggestionsRequest {
    /// <p>A domain name that you want to use as the basis for a list of possible domain names. The domain name must contain a top-level domain (TLD), such as .com, that Amazon Route 53 supports. For a list of TLDs, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/registrar-tld-list.html">Domains that You Can Register with Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>If <code>OnlyAvailable</code> is <code>true</code>, Amazon Route 53 returns only domain names that are available. If <code>OnlyAvailable</code> is <code>false</code>, Amazon Route 53 returns domain names without checking whether they're available to be registered. To determine whether the domain is available, you can call <code>checkDomainAvailability</code> for each suggestion.</p>
    #[serde(rename = "OnlyAvailable")]
    pub only_available: bool,
    /// <p>The number of suggested domain names that you want Amazon Route 53 to return.</p>
    #[serde(rename = "SuggestionCount")]
    pub suggestion_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDomainSuggestionsResponse {
    /// <p>A list of possible domain names. If you specified <code>true</code> for <code>OnlyAvailable</code> in the request, the list contains only domains that are available for registration.</p>
    #[serde(rename = "SuggestionsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions_list: Option<Vec<DomainSuggestion>>,
}

/// <p>The <a>GetOperationDetail</a> request includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOperationDetailRequest {
    /// <p>The identifier for the operation for which you want to get the status. Amazon Route 53 returned the identifier in the response to the original request.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>The GetOperationDetail response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOperationDetailResponse {
    /// <p>The name of a domain.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>Detailed information on the status including possible errors.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The identifier for the operation.</p>
    #[serde(rename = "OperationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// <p>The current status of the requested operation in the system.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The date when the request was submitted.</p>
    #[serde(rename = "SubmittedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_date: Option<f64>,
    /// <p>The type of operation that was requested.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The ListDomains request includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDomainsRequest {
    /// <p>For an initial request for a list of domains, omit this element. If the number of domains that are associated with the current AWS account is greater than the value that you specified for <code>MaxItems</code>, you can use <code>Marker</code> to return additional domains. Get the value of <code>NextPageMarker</code> from the previous response, and submit another request that includes the value of <code>NextPageMarker</code> in the <code>Marker</code> element.</p> <p>Constraints: The marker must match the value specified in the previous request.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Number of domains to be returned.</p> <p>Default: 20</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

/// <p>The ListDomains response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDomainsResponse {
    /// <p>A summary of domains.</p>
    #[serde(rename = "Domains")]
    pub domains: Vec<DomainSummary>,
    /// <p>If there are more domains than you specified for <code>MaxItems</code> in the request, submit another request and include the value of <code>NextPageMarker</code> in the value of <code>Marker</code>.</p>
    #[serde(rename = "NextPageMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_marker: Option<String>,
}

/// <p>The ListOperations request includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOperationsRequest {
    /// <p>For an initial request for a list of operations, omit this element. If the number of operations that are not yet complete is greater than the value that you specified for <code>MaxItems</code>, you can use <code>Marker</code> to return additional operations. Get the value of <code>NextPageMarker</code> from the previous response, and submit another request that includes the value of <code>NextPageMarker</code> in the <code>Marker</code> element.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Number of domains to be returned.</p> <p>Default: 20</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>An optional parameter that lets you get information about all the operations that you submitted after a specified date and time. Specify the date and time in Coordinated Universal time (UTC).</p>
    #[serde(rename = "SubmittedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_since: Option<f64>,
}

/// <p>The ListOperations response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOperationsResponse {
    /// <p>If there are more operations than you specified for <code>MaxItems</code> in the request, submit another request and include the value of <code>NextPageMarker</code> in the value of <code>Marker</code>.</p>
    #[serde(rename = "NextPageMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_marker: Option<String>,
    /// <p>Lists summaries of the operations.</p>
    #[serde(rename = "Operations")]
    pub operations: Vec<OperationSummary>,
}

/// <p>The ListTagsForDomainRequest includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForDomainRequest {
    /// <p>The domain for which you want to get a list of tags.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The ListTagsForDomain response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForDomainResponse {
    /// <p>A list of the tags that are associated with the specified domain.</p>
    #[serde(rename = "TagList")]
    pub tag_list: Vec<Tag>,
}

/// <p>Nameserver includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nameserver {
    /// <p>Glue IP address of a name server entry. Glue IP addresses are required only when the name of the name server is a subdomain of the domain. For example, if your domain is example.com and the name server for the domain is ns.example.com, you need to specify the IP address for ns.example.com.</p> <p>Constraints: The list can contain only one IPv4 and one IPv6 address.</p>
    #[serde(rename = "GlueIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_ips: Option<Vec<String>>,
    /// <p>The fully qualified host name of the name server.</p> <p>Constraint: Maximum 255 characters</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>OperationSummary includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OperationSummary {
    /// <p>Identifier returned to track the requested action.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
    /// <p>The current status of the requested operation in the system.</p>
    #[serde(rename = "Status")]
    pub status: String,
    /// <p>The date when the request was submitted.</p>
    #[serde(rename = "SubmittedDate")]
    pub submitted_date: f64,
    /// <p>Type of the action requested.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>The RegisterDomain request includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterDomainRequest {
    /// <p>Provides detailed contact information.</p>
    #[serde(rename = "AdminContact")]
    pub admin_contact: ContactDetail,
    /// <p>Indicates whether the domain will be automatically renewed (<code>true</code>) or not (<code>false</code>). Autorenewal only takes effect after the account is charged.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// <p>The domain name that you want to register.</p> <p>Constraints: The domain name can contain only the letters a through z, the numbers 0 through 9, and hyphen (-). Internationalized Domain Names are not supported.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The number of years that you want to register the domain for. Domains are registered for a minimum of one year. The maximum period depends on the top-level domain. For the range of valid values for your domain, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/registrar-tld-list.html">Domains that You Can Register with Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>Default: 1</p>
    #[serde(rename = "DurationInYears")]
    pub duration_in_years: i64,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "IdnLangCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idn_lang_code: Option<String>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you specify <code>false</code>, WHOIS queries return the information that you entered for the admin contact.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "PrivacyProtectAdminContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_admin_contact: Option<bool>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you specify <code>false</code>, WHOIS queries return the information that you entered for the registrant contact (the domain owner).</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "PrivacyProtectRegistrantContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_registrant_contact: Option<bool>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you specify <code>false</code>, WHOIS queries return the information that you entered for the technical contact.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "PrivacyProtectTechContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_tech_contact: Option<bool>,
    /// <p>Provides detailed contact information.</p>
    #[serde(rename = "RegistrantContact")]
    pub registrant_contact: ContactDetail,
    /// <p>Provides detailed contact information.</p>
    #[serde(rename = "TechContact")]
    pub tech_contact: ContactDetail,
}

/// <p>The RegisterDomain response includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterDomainResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>A <code>RenewDomain</code> request includes the number of years that you want to renew for and the current expiration year.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RenewDomainRequest {
    /// <p>The year when the registration for the domain is set to expire. This value must match the current expiration date for the domain.</p>
    #[serde(rename = "CurrentExpiryYear")]
    pub current_expiry_year: i64,
    /// <p>The name of the domain that you want to renew.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The number of years that you want to renew the domain for. The maximum number of years depends on the top-level domain. For the range of valid values for your domain, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/registrar-tld-list.html">Domains that You Can Register with Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>Default: 1</p>
    #[serde(rename = "DurationInYears")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_years: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RenewDomainResponse {
    /// <p>The identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResendContactReachabilityEmailRequest {
    /// <p>The name of the domain for which you want Amazon Route 53 to resend a confirmation email to the registrant contact.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResendContactReachabilityEmailResponse {
    /// <p>The domain name for which you requested a confirmation email.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The email address for the registrant contact at the time that we sent the verification email.</p>
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p> <code>True</code> if the email address for the registrant contact has already been verified, and <code>false</code> otherwise. If the email address has already been verified, we don't send another confirmation email.</p>
    #[serde(rename = "isAlreadyVerified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_already_verified: Option<bool>,
}

/// <p>A request for the authorization code for the specified domain. To transfer a domain to another registrar, you provide this value to the new registrar.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RetrieveDomainAuthCodeRequest {
    /// <p>The name of the domain that you want to get an authorization code for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The RetrieveDomainAuthCode response includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RetrieveDomainAuthCodeResponse {
    /// <p>The authorization code for the domain.</p>
    #[serde(rename = "AuthCode")]
    pub auth_code: String,
}

/// <p>Each tag includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key (name) of a tag.</p> <p>Valid values: A-Z, a-z, 0-9, space, ".:/=+\-@"</p> <p>Constraints: Each key can be 1-128 characters long.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of a tag.</p> <p>Valid values: A-Z, a-z, 0-9, space, ".:/=+\-@"</p> <p>Constraints: Each value can be 0-256 characters long.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The TransferDomain request includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TransferDomainRequest {
    /// <p>Provides detailed contact information.</p>
    #[serde(rename = "AdminContact")]
    pub admin_contact: ContactDetail,
    /// <p>The authorization code for the domain. You get this value from the current registrar.</p>
    #[serde(rename = "AuthCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    /// <p>Indicates whether the domain will be automatically renewed (true) or not (false). Autorenewal only takes effect after the account is charged.</p> <p>Default: true</p>
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// <p>The name of the domain that you want to transfer to Amazon Route 53.</p> <p>Constraints: The domain name can contain only the letters a through z, the numbers 0 through 9, and hyphen (-). Internationalized Domain Names are not supported.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The number of years that you want to register the domain for. Domains are registered for a minimum of one year. The maximum period depends on the top-level domain.</p> <p>Default: 1</p>
    #[serde(rename = "DurationInYears")]
    pub duration_in_years: i64,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "IdnLangCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idn_lang_code: Option<String>,
    /// <p>Contains details for the host and glue IP addresses.</p>
    #[serde(rename = "Nameservers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<Nameserver>>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you specify <code>false</code>, WHOIS queries return the information that you entered for the admin contact.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "PrivacyProtectAdminContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_admin_contact: Option<bool>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you specify <code>false</code>, WHOIS queries return the information that you entered for the registrant contact (domain owner).</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "PrivacyProtectRegistrantContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_registrant_contact: Option<bool>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you specify <code>false</code>, WHOIS queries return the information that you entered for the technical contact.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "PrivacyProtectTechContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_tech_contact: Option<bool>,
    /// <p>Provides detailed contact information.</p>
    #[serde(rename = "RegistrantContact")]
    pub registrant_contact: ContactDetail,
    /// <p>Provides detailed contact information.</p>
    #[serde(rename = "TechContact")]
    pub tech_contact: ContactDetail,
}

/// <p>The TranserDomain response includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TransferDomainResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>The UpdateDomainContactPrivacy request includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDomainContactPrivacyRequest {
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you specify <code>false</code>, WHOIS queries return the information that you entered for the admin contact.</p>
    #[serde(rename = "AdminPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_privacy: Option<bool>,
    /// <p>The name of the domain that you want to update the privacy setting for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you specify <code>false</code>, WHOIS queries return the information that you entered for the registrant contact (domain owner).</p>
    #[serde(rename = "RegistrantPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_privacy: Option<bool>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you specify <code>false</code>, WHOIS queries return the information that you entered for the technical contact.</p>
    #[serde(rename = "TechPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_privacy: Option<bool>,
}

/// <p>The UpdateDomainContactPrivacy response includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDomainContactPrivacyResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use GetOperationDetail.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>The UpdateDomainContact request includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDomainContactRequest {
    /// <p>Provides detailed contact information.</p>
    #[serde(rename = "AdminContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_contact: Option<ContactDetail>,
    /// <p>The name of the domain that you want to update contact information for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Provides detailed contact information.</p>
    #[serde(rename = "RegistrantContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_contact: Option<ContactDetail>,
    /// <p>Provides detailed contact information.</p>
    #[serde(rename = "TechContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_contact: Option<ContactDetail>,
}

/// <p>The UpdateDomainContact response includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDomainContactResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>Replaces the current set of name servers for the domain with the specified set of name servers. If you use Amazon Route 53 as your DNS service, specify the four name servers in the delegation set for the hosted zone for the domain.</p> <p>If successful, this operation returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDomainNameserversRequest {
    /// <p>The name of the domain that you want to change name servers for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>A list of new name servers for the domain.</p>
    #[serde(rename = "Nameservers")]
    pub nameservers: Vec<Nameserver>,
}

/// <p>The UpdateDomainNameservers response includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDomainNameserversResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>The UpdateTagsForDomainRequest includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTagsForDomainRequest {
    /// <p>The domain for which you want to add or update tags.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>A list of the tag keys and values that you want to add or update. If you specify a key that already exists, the corresponding value will be replaced.</p>
    #[serde(rename = "TagsToUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_to_update: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTagsForDomainResponse {}

/// <p>The ViewBilling request includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ViewBillingRequest {
    /// <p>The end date and time for the time period for which you want a list of billing records. Specify the date and time in Coordinated Universal time (UTC).</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    /// <p>For an initial request for a list of billing records, omit this element. If the number of billing records that are associated with the current AWS account during the specified period is greater than the value that you specified for <code>MaxItems</code>, you can use <code>Marker</code> to return additional billing records. Get the value of <code>NextPageMarker</code> from the previous response, and submit another request that includes the value of <code>NextPageMarker</code> in the <code>Marker</code> element. </p> <p>Constraints: The marker must match the value of <code>NextPageMarker</code> that was returned in the previous response.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The number of billing records to be returned.</p> <p>Default: 20</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>The beginning date and time for the time period for which you want a list of billing records. Specify the date and time in Coordinated Universal time (UTC).</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// <p>The ViewBilling response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ViewBillingResponse {
    /// <p>A summary of billing records.</p>
    #[serde(rename = "BillingRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_records: Option<Vec<BillingRecord>>,
    /// <p>If there are more billing records than you specified for <code>MaxItems</code> in the request, submit another request and include the value of <code>NextPageMarker</code> in the value of <code>Marker</code>.</p>
    #[serde(rename = "NextPageMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_marker: Option<String>,
}

/// Errors returned by CheckDomainAvailability
#[derive(Debug, PartialEq)]
pub enum CheckDomainAvailabilityError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl CheckDomainAvailabilityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CheckDomainAvailabilityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(CheckDomainAvailabilityError::InvalidInput(
                        err.msg,
                    ))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(CheckDomainAvailabilityError::UnsupportedTLD(
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
impl fmt::Display for CheckDomainAvailabilityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CheckDomainAvailabilityError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CheckDomainAvailabilityError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CheckDomainAvailabilityError {}
/// Errors returned by CheckDomainTransferability
#[derive(Debug, PartialEq)]
pub enum CheckDomainTransferabilityError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl CheckDomainTransferabilityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CheckDomainTransferabilityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(CheckDomainTransferabilityError::InvalidInput(
                        err.msg,
                    ))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(CheckDomainTransferabilityError::UnsupportedTLD(
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
impl fmt::Display for CheckDomainTransferabilityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CheckDomainTransferabilityError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CheckDomainTransferabilityError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CheckDomainTransferabilityError {}
/// Errors returned by DeleteTagsForDomain
#[derive(Debug, PartialEq)]
pub enum DeleteTagsForDomainError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl DeleteTagsForDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagsForDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(DeleteTagsForDomainError::InvalidInput(err.msg))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(DeleteTagsForDomainError::OperationLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(DeleteTagsForDomainError::UnsupportedTLD(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTagsForDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTagsForDomainError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteTagsForDomainError::OperationLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteTagsForDomainError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTagsForDomainError {}
/// Errors returned by DisableDomainAutoRenew
#[derive(Debug, PartialEq)]
pub enum DisableDomainAutoRenewError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl DisableDomainAutoRenewError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableDomainAutoRenewError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(DisableDomainAutoRenewError::InvalidInput(err.msg))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(DisableDomainAutoRenewError::UnsupportedTLD(
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
impl fmt::Display for DisableDomainAutoRenewError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableDomainAutoRenewError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisableDomainAutoRenewError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableDomainAutoRenewError {}
/// Errors returned by DisableDomainTransferLock
#[derive(Debug, PartialEq)]
pub enum DisableDomainTransferLockError {
    /// <p>The request is already in progress for the domain.</p>
    DuplicateRequest(String),
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>The top-level domain does not support this operation.</p>
    TLDRulesViolation(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl DisableDomainTransferLockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableDomainTransferLockError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(DisableDomainTransferLockError::DuplicateRequest(
                        err.msg,
                    ))
                }
                "InvalidInput" => {
                    return RusotoError::Service(DisableDomainTransferLockError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(
                        DisableDomainTransferLockError::OperationLimitExceeded(err.msg),
                    )
                }
                "TLDRulesViolation" => {
                    return RusotoError::Service(DisableDomainTransferLockError::TLDRulesViolation(
                        err.msg,
                    ))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(DisableDomainTransferLockError::UnsupportedTLD(
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
impl fmt::Display for DisableDomainTransferLockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableDomainTransferLockError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            DisableDomainTransferLockError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisableDomainTransferLockError::OperationLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableDomainTransferLockError::TLDRulesViolation(ref cause) => write!(f, "{}", cause),
            DisableDomainTransferLockError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableDomainTransferLockError {}
/// Errors returned by EnableDomainAutoRenew
#[derive(Debug, PartialEq)]
pub enum EnableDomainAutoRenewError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The top-level domain does not support this operation.</p>
    TLDRulesViolation(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl EnableDomainAutoRenewError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableDomainAutoRenewError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(EnableDomainAutoRenewError::InvalidInput(err.msg))
                }
                "TLDRulesViolation" => {
                    return RusotoError::Service(EnableDomainAutoRenewError::TLDRulesViolation(
                        err.msg,
                    ))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(EnableDomainAutoRenewError::UnsupportedTLD(
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
impl fmt::Display for EnableDomainAutoRenewError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableDomainAutoRenewError::InvalidInput(ref cause) => write!(f, "{}", cause),
            EnableDomainAutoRenewError::TLDRulesViolation(ref cause) => write!(f, "{}", cause),
            EnableDomainAutoRenewError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableDomainAutoRenewError {}
/// Errors returned by EnableDomainTransferLock
#[derive(Debug, PartialEq)]
pub enum EnableDomainTransferLockError {
    /// <p>The request is already in progress for the domain.</p>
    DuplicateRequest(String),
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>The top-level domain does not support this operation.</p>
    TLDRulesViolation(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl EnableDomainTransferLockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableDomainTransferLockError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(EnableDomainTransferLockError::DuplicateRequest(
                        err.msg,
                    ))
                }
                "InvalidInput" => {
                    return RusotoError::Service(EnableDomainTransferLockError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(
                        EnableDomainTransferLockError::OperationLimitExceeded(err.msg),
                    )
                }
                "TLDRulesViolation" => {
                    return RusotoError::Service(EnableDomainTransferLockError::TLDRulesViolation(
                        err.msg,
                    ))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(EnableDomainTransferLockError::UnsupportedTLD(
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
impl fmt::Display for EnableDomainTransferLockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableDomainTransferLockError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            EnableDomainTransferLockError::InvalidInput(ref cause) => write!(f, "{}", cause),
            EnableDomainTransferLockError::OperationLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            EnableDomainTransferLockError::TLDRulesViolation(ref cause) => write!(f, "{}", cause),
            EnableDomainTransferLockError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableDomainTransferLockError {}
/// Errors returned by GetContactReachabilityStatus
#[derive(Debug, PartialEq)]
pub enum GetContactReachabilityStatusError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl GetContactReachabilityStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetContactReachabilityStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(GetContactReachabilityStatusError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(
                        GetContactReachabilityStatusError::OperationLimitExceeded(err.msg),
                    )
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(GetContactReachabilityStatusError::UnsupportedTLD(
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
impl fmt::Display for GetContactReachabilityStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetContactReachabilityStatusError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetContactReachabilityStatusError::OperationLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetContactReachabilityStatusError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetContactReachabilityStatusError {}
/// Errors returned by GetDomainDetail
#[derive(Debug, PartialEq)]
pub enum GetDomainDetailError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl GetDomainDetailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainDetailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(GetDomainDetailError::InvalidInput(err.msg))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(GetDomainDetailError::UnsupportedTLD(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDomainDetailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainDetailError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetDomainDetailError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDomainDetailError {}
/// Errors returned by GetDomainSuggestions
#[derive(Debug, PartialEq)]
pub enum GetDomainSuggestionsError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl GetDomainSuggestionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainSuggestionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(GetDomainSuggestionsError::InvalidInput(err.msg))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(GetDomainSuggestionsError::UnsupportedTLD(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDomainSuggestionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainSuggestionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetDomainSuggestionsError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDomainSuggestionsError {}
/// Errors returned by GetOperationDetail
#[derive(Debug, PartialEq)]
pub enum GetOperationDetailError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
}

impl GetOperationDetailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOperationDetailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(GetOperationDetailError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOperationDetailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOperationDetailError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOperationDetailError {}
/// Errors returned by ListDomains
#[derive(Debug, PartialEq)]
pub enum ListDomainsError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
}

impl ListDomainsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDomainsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(ListDomainsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDomainsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDomainsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDomainsError {}
/// Errors returned by ListOperations
#[derive(Debug, PartialEq)]
pub enum ListOperationsError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
}

impl ListOperationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOperationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(ListOperationsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOperationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOperationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOperationsError {}
/// Errors returned by ListTagsForDomain
#[derive(Debug, PartialEq)]
pub enum ListTagsForDomainError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl ListTagsForDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(ListTagsForDomainError::InvalidInput(err.msg))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(ListTagsForDomainError::OperationLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(ListTagsForDomainError::UnsupportedTLD(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForDomainError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTagsForDomainError::OperationLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTagsForDomainError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForDomainError {}
/// Errors returned by RegisterDomain
#[derive(Debug, PartialEq)]
pub enum RegisterDomainError {
    /// <p>The number of domains has exceeded the allowed threshold for the account.</p>
    DomainLimitExceeded(String),
    /// <p>The request is already in progress for the domain.</p>
    DuplicateRequest(String),
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>The top-level domain does not support this operation.</p>
    TLDRulesViolation(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl RegisterDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DomainLimitExceeded" => {
                    return RusotoError::Service(RegisterDomainError::DomainLimitExceeded(err.msg))
                }
                "DuplicateRequest" => {
                    return RusotoError::Service(RegisterDomainError::DuplicateRequest(err.msg))
                }
                "InvalidInput" => {
                    return RusotoError::Service(RegisterDomainError::InvalidInput(err.msg))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(RegisterDomainError::OperationLimitExceeded(
                        err.msg,
                    ))
                }
                "TLDRulesViolation" => {
                    return RusotoError::Service(RegisterDomainError::TLDRulesViolation(err.msg))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(RegisterDomainError::UnsupportedTLD(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterDomainError::DomainLimitExceeded(ref cause) => write!(f, "{}", cause),
            RegisterDomainError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            RegisterDomainError::InvalidInput(ref cause) => write!(f, "{}", cause),
            RegisterDomainError::OperationLimitExceeded(ref cause) => write!(f, "{}", cause),
            RegisterDomainError::TLDRulesViolation(ref cause) => write!(f, "{}", cause),
            RegisterDomainError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterDomainError {}
/// Errors returned by RenewDomain
#[derive(Debug, PartialEq)]
pub enum RenewDomainError {
    /// <p>The request is already in progress for the domain.</p>
    DuplicateRequest(String),
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>The top-level domain does not support this operation.</p>
    TLDRulesViolation(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl RenewDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RenewDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(RenewDomainError::DuplicateRequest(err.msg))
                }
                "InvalidInput" => {
                    return RusotoError::Service(RenewDomainError::InvalidInput(err.msg))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(RenewDomainError::OperationLimitExceeded(err.msg))
                }
                "TLDRulesViolation" => {
                    return RusotoError::Service(RenewDomainError::TLDRulesViolation(err.msg))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(RenewDomainError::UnsupportedTLD(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RenewDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenewDomainError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            RenewDomainError::InvalidInput(ref cause) => write!(f, "{}", cause),
            RenewDomainError::OperationLimitExceeded(ref cause) => write!(f, "{}", cause),
            RenewDomainError::TLDRulesViolation(ref cause) => write!(f, "{}", cause),
            RenewDomainError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RenewDomainError {}
/// Errors returned by ResendContactReachabilityEmail
#[derive(Debug, PartialEq)]
pub enum ResendContactReachabilityEmailError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl ResendContactReachabilityEmailError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ResendContactReachabilityEmailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(ResendContactReachabilityEmailError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(
                        ResendContactReachabilityEmailError::OperationLimitExceeded(err.msg),
                    )
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(
                        ResendContactReachabilityEmailError::UnsupportedTLD(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ResendContactReachabilityEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResendContactReachabilityEmailError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ResendContactReachabilityEmailError::OperationLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ResendContactReachabilityEmailError::UnsupportedTLD(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ResendContactReachabilityEmailError {}
/// Errors returned by RetrieveDomainAuthCode
#[derive(Debug, PartialEq)]
pub enum RetrieveDomainAuthCodeError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl RetrieveDomainAuthCodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RetrieveDomainAuthCodeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(RetrieveDomainAuthCodeError::InvalidInput(err.msg))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(RetrieveDomainAuthCodeError::UnsupportedTLD(
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
impl fmt::Display for RetrieveDomainAuthCodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RetrieveDomainAuthCodeError::InvalidInput(ref cause) => write!(f, "{}", cause),
            RetrieveDomainAuthCodeError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RetrieveDomainAuthCodeError {}
/// Errors returned by TransferDomain
#[derive(Debug, PartialEq)]
pub enum TransferDomainError {
    /// <p>The number of domains has exceeded the allowed threshold for the account.</p>
    DomainLimitExceeded(String),
    /// <p>The request is already in progress for the domain.</p>
    DuplicateRequest(String),
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>The top-level domain does not support this operation.</p>
    TLDRulesViolation(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl TransferDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TransferDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DomainLimitExceeded" => {
                    return RusotoError::Service(TransferDomainError::DomainLimitExceeded(err.msg))
                }
                "DuplicateRequest" => {
                    return RusotoError::Service(TransferDomainError::DuplicateRequest(err.msg))
                }
                "InvalidInput" => {
                    return RusotoError::Service(TransferDomainError::InvalidInput(err.msg))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(TransferDomainError::OperationLimitExceeded(
                        err.msg,
                    ))
                }
                "TLDRulesViolation" => {
                    return RusotoError::Service(TransferDomainError::TLDRulesViolation(err.msg))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(TransferDomainError::UnsupportedTLD(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TransferDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransferDomainError::DomainLimitExceeded(ref cause) => write!(f, "{}", cause),
            TransferDomainError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            TransferDomainError::InvalidInput(ref cause) => write!(f, "{}", cause),
            TransferDomainError::OperationLimitExceeded(ref cause) => write!(f, "{}", cause),
            TransferDomainError::TLDRulesViolation(ref cause) => write!(f, "{}", cause),
            TransferDomainError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TransferDomainError {}
/// Errors returned by UpdateDomainContact
#[derive(Debug, PartialEq)]
pub enum UpdateDomainContactError {
    /// <p>The request is already in progress for the domain.</p>
    DuplicateRequest(String),
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>The top-level domain does not support this operation.</p>
    TLDRulesViolation(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl UpdateDomainContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDomainContactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(UpdateDomainContactError::DuplicateRequest(
                        err.msg,
                    ))
                }
                "InvalidInput" => {
                    return RusotoError::Service(UpdateDomainContactError::InvalidInput(err.msg))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(UpdateDomainContactError::OperationLimitExceeded(
                        err.msg,
                    ))
                }
                "TLDRulesViolation" => {
                    return RusotoError::Service(UpdateDomainContactError::TLDRulesViolation(
                        err.msg,
                    ))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(UpdateDomainContactError::UnsupportedTLD(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDomainContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDomainContactError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            UpdateDomainContactError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateDomainContactError::OperationLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDomainContactError::TLDRulesViolation(ref cause) => write!(f, "{}", cause),
            UpdateDomainContactError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDomainContactError {}
/// Errors returned by UpdateDomainContactPrivacy
#[derive(Debug, PartialEq)]
pub enum UpdateDomainContactPrivacyError {
    /// <p>The request is already in progress for the domain.</p>
    DuplicateRequest(String),
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>The top-level domain does not support this operation.</p>
    TLDRulesViolation(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl UpdateDomainContactPrivacyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDomainContactPrivacyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(UpdateDomainContactPrivacyError::DuplicateRequest(
                        err.msg,
                    ))
                }
                "InvalidInput" => {
                    return RusotoError::Service(UpdateDomainContactPrivacyError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(
                        UpdateDomainContactPrivacyError::OperationLimitExceeded(err.msg),
                    )
                }
                "TLDRulesViolation" => {
                    return RusotoError::Service(
                        UpdateDomainContactPrivacyError::TLDRulesViolation(err.msg),
                    )
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(UpdateDomainContactPrivacyError::UnsupportedTLD(
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
impl fmt::Display for UpdateDomainContactPrivacyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDomainContactPrivacyError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            UpdateDomainContactPrivacyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateDomainContactPrivacyError::OperationLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDomainContactPrivacyError::TLDRulesViolation(ref cause) => write!(f, "{}", cause),
            UpdateDomainContactPrivacyError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDomainContactPrivacyError {}
/// Errors returned by UpdateDomainNameservers
#[derive(Debug, PartialEq)]
pub enum UpdateDomainNameserversError {
    /// <p>The request is already in progress for the domain.</p>
    DuplicateRequest(String),
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>The top-level domain does not support this operation.</p>
    TLDRulesViolation(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl UpdateDomainNameserversError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDomainNameserversError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(UpdateDomainNameserversError::DuplicateRequest(
                        err.msg,
                    ))
                }
                "InvalidInput" => {
                    return RusotoError::Service(UpdateDomainNameserversError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(
                        UpdateDomainNameserversError::OperationLimitExceeded(err.msg),
                    )
                }
                "TLDRulesViolation" => {
                    return RusotoError::Service(UpdateDomainNameserversError::TLDRulesViolation(
                        err.msg,
                    ))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(UpdateDomainNameserversError::UnsupportedTLD(
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
impl fmt::Display for UpdateDomainNameserversError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDomainNameserversError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            UpdateDomainNameserversError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateDomainNameserversError::OperationLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDomainNameserversError::TLDRulesViolation(ref cause) => write!(f, "{}", cause),
            UpdateDomainNameserversError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDomainNameserversError {}
/// Errors returned by UpdateTagsForDomain
#[derive(Debug, PartialEq)]
pub enum UpdateTagsForDomainError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
}

impl UpdateTagsForDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTagsForDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(UpdateTagsForDomainError::InvalidInput(err.msg))
                }
                "OperationLimitExceeded" => {
                    return RusotoError::Service(UpdateTagsForDomainError::OperationLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedTLD" => {
                    return RusotoError::Service(UpdateTagsForDomainError::UnsupportedTLD(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTagsForDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTagsForDomainError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateTagsForDomainError::OperationLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateTagsForDomainError::UnsupportedTLD(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTagsForDomainError {}
/// Errors returned by ViewBilling
#[derive(Debug, PartialEq)]
pub enum ViewBillingError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
}

impl ViewBillingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ViewBillingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(ViewBillingError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ViewBillingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ViewBillingError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ViewBillingError {}
/// Trait representing the capabilities of the Amazon Route 53 Domains API. Amazon Route 53 Domains clients implement this trait.
#[async_trait]
pub trait Route53Domains {
    /// <p>This operation checks the availability of one domain name. Note that if the availability status of a domain is pending, you must submit another request to determine the availability of the domain name.</p>
    async fn check_domain_availability(
        &self,
        input: CheckDomainAvailabilityRequest,
    ) -> Result<CheckDomainAvailabilityResponse, RusotoError<CheckDomainAvailabilityError>>;

    /// <p>Checks whether a domain name can be transferred to Amazon Route 53. </p>
    async fn check_domain_transferability(
        &self,
        input: CheckDomainTransferabilityRequest,
    ) -> Result<CheckDomainTransferabilityResponse, RusotoError<CheckDomainTransferabilityError>>;

    /// <p>This operation deletes the specified tags for a domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    async fn delete_tags_for_domain(
        &self,
        input: DeleteTagsForDomainRequest,
    ) -> Result<DeleteTagsForDomainResponse, RusotoError<DeleteTagsForDomainError>>;

    /// <p>This operation disables automatic renewal of domain registration for the specified domain.</p>
    async fn disable_domain_auto_renew(
        &self,
        input: DisableDomainAutoRenewRequest,
    ) -> Result<DisableDomainAutoRenewResponse, RusotoError<DisableDomainAutoRenewError>>;

    /// <p>This operation removes the transfer lock on the domain (specifically the <code>clientTransferProhibited</code> status) to allow domain transfers. We recommend you refrain from performing this action unless you intend to transfer the domain to a different registrar. Successful submission returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    async fn disable_domain_transfer_lock(
        &self,
        input: DisableDomainTransferLockRequest,
    ) -> Result<DisableDomainTransferLockResponse, RusotoError<DisableDomainTransferLockError>>;

    /// <p>This operation configures Amazon Route 53 to automatically renew the specified domain before the domain registration expires. The cost of renewing your domain registration is billed to your AWS account.</p> <p>The period during which you can renew a domain name varies by TLD. For a list of TLDs and their renewal policies, see <a href="http://wiki.gandi.net/en/domains/renew#renewal_restoration_and_deletion_times">"Renewal, restoration, and deletion times"</a> on the website for our registrar associate, Gandi. Amazon Route 53 requires that you renew before the end of the renewal period that is listed on the Gandi website so we can complete processing before the deadline.</p>
    async fn enable_domain_auto_renew(
        &self,
        input: EnableDomainAutoRenewRequest,
    ) -> Result<EnableDomainAutoRenewResponse, RusotoError<EnableDomainAutoRenewError>>;

    /// <p>This operation sets the transfer lock on the domain (specifically the <code>clientTransferProhibited</code> status) to prevent domain transfers. Successful submission returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    async fn enable_domain_transfer_lock(
        &self,
        input: EnableDomainTransferLockRequest,
    ) -> Result<EnableDomainTransferLockResponse, RusotoError<EnableDomainTransferLockError>>;

    /// <p>For operations that require confirmation that the email address for the registrant contact is valid, such as registering a new domain, this operation returns information about whether the registrant contact has responded.</p> <p>If you want us to resend the email, use the <code>ResendContactReachabilityEmail</code> operation.</p>
    async fn get_contact_reachability_status(
        &self,
        input: GetContactReachabilityStatusRequest,
    ) -> Result<GetContactReachabilityStatusResponse, RusotoError<GetContactReachabilityStatusError>>;

    /// <p>This operation returns detailed information about a specified domain that is associated with the current AWS account. Contact information for the domain is also returned as part of the output.</p>
    async fn get_domain_detail(
        &self,
        input: GetDomainDetailRequest,
    ) -> Result<GetDomainDetailResponse, RusotoError<GetDomainDetailError>>;

    /// <p>The GetDomainSuggestions operation returns a list of suggested domain names given a string, which can either be a domain name or simply a word or phrase (without spaces).</p>
    async fn get_domain_suggestions(
        &self,
        input: GetDomainSuggestionsRequest,
    ) -> Result<GetDomainSuggestionsResponse, RusotoError<GetDomainSuggestionsError>>;

    /// <p>This operation returns the current status of an operation that is not completed.</p>
    async fn get_operation_detail(
        &self,
        input: GetOperationDetailRequest,
    ) -> Result<GetOperationDetailResponse, RusotoError<GetOperationDetailError>>;

    /// <p>This operation returns all the domain names registered with Amazon Route 53 for the current AWS account.</p>
    async fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> Result<ListDomainsResponse, RusotoError<ListDomainsError>>;

    /// <p>This operation returns the operation IDs of operations that are not yet complete.</p>
    async fn list_operations(
        &self,
        input: ListOperationsRequest,
    ) -> Result<ListOperationsResponse, RusotoError<ListOperationsError>>;

    /// <p>This operation returns all of the tags that are associated with the specified domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    async fn list_tags_for_domain(
        &self,
        input: ListTagsForDomainRequest,
    ) -> Result<ListTagsForDomainResponse, RusotoError<ListTagsForDomainError>>;

    /// <p><p>This operation registers a domain. Domains are registered either by Amazon Registrar (for .com, .net, and .org domains) or by our registrar associate, Gandi (for all other domains). For some top-level domains (TLDs), this operation requires extra parameters.</p> <p>When you register a domain, Amazon Route 53 does the following:</p> <ul> <li> <p>Creates a Amazon Route 53 hosted zone that has the same name as the domain. Amazon Route 53 assigns four name servers to your hosted zone and automatically updates your domain registration with the names of these name servers.</p> </li> <li> <p>Enables autorenew, so your domain registration will renew automatically each year. We&#39;ll notify you in advance of the renewal date so you can choose whether to renew the registration.</p> </li> <li> <p>Optionally enables privacy protection, so WHOIS queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you don&#39;t enable privacy protection, WHOIS queries return the information that you entered for the registrant, admin, and tech contacts.</p> </li> <li> <p>If registration is successful, returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant is notified by email.</p> </li> <li> <p>Charges your AWS account an amount based on the top-level domain. For more information, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p> </li> </ul></p>
    async fn register_domain(
        &self,
        input: RegisterDomainRequest,
    ) -> Result<RegisterDomainResponse, RusotoError<RegisterDomainError>>;

    /// <p>This operation renews a domain for the specified number of years. The cost of renewing your domain is billed to your AWS account.</p> <p>We recommend that you renew your domain several weeks before the expiration date. Some TLD registries delete domains before the expiration date if you haven't renewed far enough in advance. For more information about renewing domain registration, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/domain-renew.html">Renewing Registration for a Domain</a> in the Amazon Route 53 Developer Guide.</p>
    async fn renew_domain(
        &self,
        input: RenewDomainRequest,
    ) -> Result<RenewDomainResponse, RusotoError<RenewDomainError>>;

    /// <p>For operations that require confirmation that the email address for the registrant contact is valid, such as registering a new domain, this operation resends the confirmation email to the current email address for the registrant contact.</p>
    async fn resend_contact_reachability_email(
        &self,
        input: ResendContactReachabilityEmailRequest,
    ) -> Result<
        ResendContactReachabilityEmailResponse,
        RusotoError<ResendContactReachabilityEmailError>,
    >;

    /// <p>This operation returns the AuthCode for the domain. To transfer a domain to another registrar, you provide this value to the new registrar.</p>
    async fn retrieve_domain_auth_code(
        &self,
        input: RetrieveDomainAuthCodeRequest,
    ) -> Result<RetrieveDomainAuthCodeResponse, RusotoError<RetrieveDomainAuthCodeError>>;

    /// <p>This operation transfers a domain from another registrar to Amazon Route 53. When the transfer is complete, the domain is registered either with Amazon Registrar (for .com, .net, and .org domains) or with our registrar associate, Gandi (for all other TLDs).</p> <p>For transfer requirements, a detailed procedure, and information about viewing the status of a domain transfer, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/domain-transfer-to-route-53.html">Transferring Registration for a Domain to Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If the registrar for your domain is also the DNS service provider for the domain, we highly recommend that you consider transferring your DNS service to Amazon Route 53 or to another DNS service provider before you transfer your registration. Some registrars provide free DNS service when you purchase a domain registration. When you transfer the registration, the previous registrar will not renew your domain registration and could end your DNS service at any time.</p> <important> <p>If the registrar for your domain is also the DNS service provider for the domain and you don't transfer DNS service to another provider, your website, email, and the web applications associated with the domain might become unavailable.</p> </important> <p>If the transfer is successful, this method returns an operation ID that you can use to track the progress and completion of the action. If the transfer doesn't complete successfully, the domain registrant will be notified by email.</p>
    async fn transfer_domain(
        &self,
        input: TransferDomainRequest,
    ) -> Result<TransferDomainResponse, RusotoError<TransferDomainError>>;

    /// <p>This operation updates the contact information for a particular domain. You must specify information for at least one contact: registrant, administrator, or technical.</p> <p>If the update is successful, this method returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    async fn update_domain_contact(
        &self,
        input: UpdateDomainContactRequest,
    ) -> Result<UpdateDomainContactResponse, RusotoError<UpdateDomainContactError>>;

    /// <p>This operation updates the specified domain contact's privacy setting. When privacy protection is enabled, contact information such as email address is replaced either with contact information for Amazon Registrar (for .com, .net, and .org domains) or with contact information for our registrar associate, Gandi.</p> <p>This operation affects only the contact information for the specified contact type (registrant, administrator, or tech). If the request succeeds, Amazon Route 53 returns an operation ID that you can use with <a>GetOperationDetail</a> to track the progress and completion of the action. If the request doesn't complete successfully, the domain registrant will be notified by email.</p>
    async fn update_domain_contact_privacy(
        &self,
        input: UpdateDomainContactPrivacyRequest,
    ) -> Result<UpdateDomainContactPrivacyResponse, RusotoError<UpdateDomainContactPrivacyError>>;

    /// <p>This operation replaces the current set of name servers for the domain with the specified set of name servers. If you use Amazon Route 53 as your DNS service, specify the four name servers in the delegation set for the hosted zone for the domain.</p> <p>If successful, this operation returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    async fn update_domain_nameservers(
        &self,
        input: UpdateDomainNameserversRequest,
    ) -> Result<UpdateDomainNameserversResponse, RusotoError<UpdateDomainNameserversError>>;

    /// <p>This operation adds or updates tags for a specified domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    async fn update_tags_for_domain(
        &self,
        input: UpdateTagsForDomainRequest,
    ) -> Result<UpdateTagsForDomainResponse, RusotoError<UpdateTagsForDomainError>>;

    /// <p>Returns all the domain-related billing records for the current AWS account for a specified period</p>
    async fn view_billing(
        &self,
        input: ViewBillingRequest,
    ) -> Result<ViewBillingResponse, RusotoError<ViewBillingError>>;
}
/// A client for the Amazon Route 53 Domains API.
#[derive(Clone)]
pub struct Route53DomainsClient {
    client: Client,
    region: region::Region,
}

impl Route53DomainsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> Route53DomainsClient {
        Route53DomainsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> Route53DomainsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        Route53DomainsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> Route53DomainsClient {
        Route53DomainsClient { client, region }
    }
}

#[async_trait]
impl Route53Domains for Route53DomainsClient {
    /// <p>This operation checks the availability of one domain name. Note that if the availability status of a domain is pending, you must submit another request to determine the availability of the domain name.</p>
    async fn check_domain_availability(
        &self,
        input: CheckDomainAvailabilityRequest,
    ) -> Result<CheckDomainAvailabilityResponse, RusotoError<CheckDomainAvailabilityError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.CheckDomainAvailability",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CheckDomainAvailabilityResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CheckDomainAvailabilityError::from_response(response))
        }
    }

    /// <p>Checks whether a domain name can be transferred to Amazon Route 53. </p>
    async fn check_domain_transferability(
        &self,
        input: CheckDomainTransferabilityRequest,
    ) -> Result<CheckDomainTransferabilityResponse, RusotoError<CheckDomainTransferabilityError>>
    {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.CheckDomainTransferability",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CheckDomainTransferabilityResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CheckDomainTransferabilityError::from_response(response))
        }
    }

    /// <p>This operation deletes the specified tags for a domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    async fn delete_tags_for_domain(
        &self,
        input: DeleteTagsForDomainRequest,
    ) -> Result<DeleteTagsForDomainResponse, RusotoError<DeleteTagsForDomainError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.DeleteTagsForDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteTagsForDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTagsForDomainError::from_response(response))
        }
    }

    /// <p>This operation disables automatic renewal of domain registration for the specified domain.</p>
    async fn disable_domain_auto_renew(
        &self,
        input: DisableDomainAutoRenewRequest,
    ) -> Result<DisableDomainAutoRenewResponse, RusotoError<DisableDomainAutoRenewError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.DisableDomainAutoRenew",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DisableDomainAutoRenewResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisableDomainAutoRenewError::from_response(response))
        }
    }

    /// <p>This operation removes the transfer lock on the domain (specifically the <code>clientTransferProhibited</code> status) to allow domain transfers. We recommend you refrain from performing this action unless you intend to transfer the domain to a different registrar. Successful submission returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    async fn disable_domain_transfer_lock(
        &self,
        input: DisableDomainTransferLockRequest,
    ) -> Result<DisableDomainTransferLockResponse, RusotoError<DisableDomainTransferLockError>>
    {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.DisableDomainTransferLock",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DisableDomainTransferLockResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisableDomainTransferLockError::from_response(response))
        }
    }

    /// <p>This operation configures Amazon Route 53 to automatically renew the specified domain before the domain registration expires. The cost of renewing your domain registration is billed to your AWS account.</p> <p>The period during which you can renew a domain name varies by TLD. For a list of TLDs and their renewal policies, see <a href="http://wiki.gandi.net/en/domains/renew#renewal_restoration_and_deletion_times">"Renewal, restoration, and deletion times"</a> on the website for our registrar associate, Gandi. Amazon Route 53 requires that you renew before the end of the renewal period that is listed on the Gandi website so we can complete processing before the deadline.</p>
    async fn enable_domain_auto_renew(
        &self,
        input: EnableDomainAutoRenewRequest,
    ) -> Result<EnableDomainAutoRenewResponse, RusotoError<EnableDomainAutoRenewError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.EnableDomainAutoRenew",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<EnableDomainAutoRenewResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(EnableDomainAutoRenewError::from_response(response))
        }
    }

    /// <p>This operation sets the transfer lock on the domain (specifically the <code>clientTransferProhibited</code> status) to prevent domain transfers. Successful submission returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    async fn enable_domain_transfer_lock(
        &self,
        input: EnableDomainTransferLockRequest,
    ) -> Result<EnableDomainTransferLockResponse, RusotoError<EnableDomainTransferLockError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.EnableDomainTransferLock",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<EnableDomainTransferLockResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(EnableDomainTransferLockError::from_response(response))
        }
    }

    /// <p>For operations that require confirmation that the email address for the registrant contact is valid, such as registering a new domain, this operation returns information about whether the registrant contact has responded.</p> <p>If you want us to resend the email, use the <code>ResendContactReachabilityEmail</code> operation.</p>
    async fn get_contact_reachability_status(
        &self,
        input: GetContactReachabilityStatusRequest,
    ) -> Result<GetContactReachabilityStatusResponse, RusotoError<GetContactReachabilityStatusError>>
    {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.GetContactReachabilityStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetContactReachabilityStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetContactReachabilityStatusError::from_response(response))
        }
    }

    /// <p>This operation returns detailed information about a specified domain that is associated with the current AWS account. Contact information for the domain is also returned as part of the output.</p>
    async fn get_domain_detail(
        &self,
        input: GetDomainDetailRequest,
    ) -> Result<GetDomainDetailResponse, RusotoError<GetDomainDetailError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.GetDomainDetail");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDomainDetailResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainDetailError::from_response(response))
        }
    }

    /// <p>The GetDomainSuggestions operation returns a list of suggested domain names given a string, which can either be a domain name or simply a word or phrase (without spaces).</p>
    async fn get_domain_suggestions(
        &self,
        input: GetDomainSuggestionsRequest,
    ) -> Result<GetDomainSuggestionsResponse, RusotoError<GetDomainSuggestionsError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.GetDomainSuggestions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDomainSuggestionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainSuggestionsError::from_response(response))
        }
    }

    /// <p>This operation returns the current status of an operation that is not completed.</p>
    async fn get_operation_detail(
        &self,
        input: GetOperationDetailRequest,
    ) -> Result<GetOperationDetailResponse, RusotoError<GetOperationDetailError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.GetOperationDetail",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetOperationDetailResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetOperationDetailError::from_response(response))
        }
    }

    /// <p>This operation returns all the domain names registered with Amazon Route 53 for the current AWS account.</p>
    async fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> Result<ListDomainsResponse, RusotoError<ListDomainsError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.ListDomains");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListDomainsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDomainsError::from_response(response))
        }
    }

    /// <p>This operation returns the operation IDs of operations that are not yet complete.</p>
    async fn list_operations(
        &self,
        input: ListOperationsRequest,
    ) -> Result<ListOperationsResponse, RusotoError<ListOperationsError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.ListOperations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListOperationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListOperationsError::from_response(response))
        }
    }

    /// <p>This operation returns all of the tags that are associated with the specified domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    async fn list_tags_for_domain(
        &self,
        input: ListTagsForDomainRequest,
    ) -> Result<ListTagsForDomainResponse, RusotoError<ListTagsForDomainError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.ListTagsForDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForDomainError::from_response(response))
        }
    }

    /// <p><p>This operation registers a domain. Domains are registered either by Amazon Registrar (for .com, .net, and .org domains) or by our registrar associate, Gandi (for all other domains). For some top-level domains (TLDs), this operation requires extra parameters.</p> <p>When you register a domain, Amazon Route 53 does the following:</p> <ul> <li> <p>Creates a Amazon Route 53 hosted zone that has the same name as the domain. Amazon Route 53 assigns four name servers to your hosted zone and automatically updates your domain registration with the names of these name servers.</p> </li> <li> <p>Enables autorenew, so your domain registration will renew automatically each year. We&#39;ll notify you in advance of the renewal date so you can choose whether to renew the registration.</p> </li> <li> <p>Optionally enables privacy protection, so WHOIS queries return contact information either for Amazon Registrar (for .com, .net, and .org domains) or for our registrar associate, Gandi (for all other TLDs). If you don&#39;t enable privacy protection, WHOIS queries return the information that you entered for the registrant, admin, and tech contacts.</p> </li> <li> <p>If registration is successful, returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant is notified by email.</p> </li> <li> <p>Charges your AWS account an amount based on the top-level domain. For more information, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p> </li> </ul></p>
    async fn register_domain(
        &self,
        input: RegisterDomainRequest,
    ) -> Result<RegisterDomainResponse, RusotoError<RegisterDomainError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.RegisterDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RegisterDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterDomainError::from_response(response))
        }
    }

    /// <p>This operation renews a domain for the specified number of years. The cost of renewing your domain is billed to your AWS account.</p> <p>We recommend that you renew your domain several weeks before the expiration date. Some TLD registries delete domains before the expiration date if you haven't renewed far enough in advance. For more information about renewing domain registration, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/domain-renew.html">Renewing Registration for a Domain</a> in the Amazon Route 53 Developer Guide.</p>
    async fn renew_domain(
        &self,
        input: RenewDomainRequest,
    ) -> Result<RenewDomainResponse, RusotoError<RenewDomainError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.RenewDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RenewDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RenewDomainError::from_response(response))
        }
    }

    /// <p>For operations that require confirmation that the email address for the registrant contact is valid, such as registering a new domain, this operation resends the confirmation email to the current email address for the registrant contact.</p>
    async fn resend_contact_reachability_email(
        &self,
        input: ResendContactReachabilityEmailRequest,
    ) -> Result<
        ResendContactReachabilityEmailResponse,
        RusotoError<ResendContactReachabilityEmailError>,
    > {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.ResendContactReachabilityEmail",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ResendContactReachabilityEmailResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ResendContactReachabilityEmailError::from_response(response))
        }
    }

    /// <p>This operation returns the AuthCode for the domain. To transfer a domain to another registrar, you provide this value to the new registrar.</p>
    async fn retrieve_domain_auth_code(
        &self,
        input: RetrieveDomainAuthCodeRequest,
    ) -> Result<RetrieveDomainAuthCodeResponse, RusotoError<RetrieveDomainAuthCodeError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.RetrieveDomainAuthCode",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<RetrieveDomainAuthCodeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RetrieveDomainAuthCodeError::from_response(response))
        }
    }

    /// <p>This operation transfers a domain from another registrar to Amazon Route 53. When the transfer is complete, the domain is registered either with Amazon Registrar (for .com, .net, and .org domains) or with our registrar associate, Gandi (for all other TLDs).</p> <p>For transfer requirements, a detailed procedure, and information about viewing the status of a domain transfer, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/domain-transfer-to-route-53.html">Transferring Registration for a Domain to Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If the registrar for your domain is also the DNS service provider for the domain, we highly recommend that you consider transferring your DNS service to Amazon Route 53 or to another DNS service provider before you transfer your registration. Some registrars provide free DNS service when you purchase a domain registration. When you transfer the registration, the previous registrar will not renew your domain registration and could end your DNS service at any time.</p> <important> <p>If the registrar for your domain is also the DNS service provider for the domain and you don't transfer DNS service to another provider, your website, email, and the web applications associated with the domain might become unavailable.</p> </important> <p>If the transfer is successful, this method returns an operation ID that you can use to track the progress and completion of the action. If the transfer doesn't complete successfully, the domain registrant will be notified by email.</p>
    async fn transfer_domain(
        &self,
        input: TransferDomainRequest,
    ) -> Result<TransferDomainResponse, RusotoError<TransferDomainError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.TransferDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TransferDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TransferDomainError::from_response(response))
        }
    }

    /// <p>This operation updates the contact information for a particular domain. You must specify information for at least one contact: registrant, administrator, or technical.</p> <p>If the update is successful, this method returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    async fn update_domain_contact(
        &self,
        input: UpdateDomainContactRequest,
    ) -> Result<UpdateDomainContactResponse, RusotoError<UpdateDomainContactError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.UpdateDomainContact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDomainContactResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDomainContactError::from_response(response))
        }
    }

    /// <p>This operation updates the specified domain contact's privacy setting. When privacy protection is enabled, contact information such as email address is replaced either with contact information for Amazon Registrar (for .com, .net, and .org domains) or with contact information for our registrar associate, Gandi.</p> <p>This operation affects only the contact information for the specified contact type (registrant, administrator, or tech). If the request succeeds, Amazon Route 53 returns an operation ID that you can use with <a>GetOperationDetail</a> to track the progress and completion of the action. If the request doesn't complete successfully, the domain registrant will be notified by email.</p>
    async fn update_domain_contact_privacy(
        &self,
        input: UpdateDomainContactPrivacyRequest,
    ) -> Result<UpdateDomainContactPrivacyResponse, RusotoError<UpdateDomainContactPrivacyError>>
    {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.UpdateDomainContactPrivacy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDomainContactPrivacyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDomainContactPrivacyError::from_response(response))
        }
    }

    /// <p>This operation replaces the current set of name servers for the domain with the specified set of name servers. If you use Amazon Route 53 as your DNS service, specify the four name servers in the delegation set for the hosted zone for the domain.</p> <p>If successful, this operation returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    async fn update_domain_nameservers(
        &self,
        input: UpdateDomainNameserversRequest,
    ) -> Result<UpdateDomainNameserversResponse, RusotoError<UpdateDomainNameserversError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.UpdateDomainNameservers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDomainNameserversResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDomainNameserversError::from_response(response))
        }
    }

    /// <p>This operation adds or updates tags for a specified domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    async fn update_tags_for_domain(
        &self,
        input: UpdateTagsForDomainRequest,
    ) -> Result<UpdateTagsForDomainResponse, RusotoError<UpdateTagsForDomainError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.UpdateTagsForDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateTagsForDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTagsForDomainError::from_response(response))
        }
    }

    /// <p>Returns all the domain-related billing records for the current AWS account for a specified period</p>
    async fn view_billing(
        &self,
        input: ViewBillingRequest,
    ) -> Result<ViewBillingResponse, RusotoError<ViewBillingError>> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.ViewBilling");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ViewBillingResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ViewBillingError::from_response(response))
        }
    }
}
