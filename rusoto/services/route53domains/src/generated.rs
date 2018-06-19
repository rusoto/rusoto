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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Information for one billing record.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct CheckDomainAvailabilityResponse {
    /// <p><p>Whether the domain name is available for registering.</p> <note> <p>You can register only domains designated as <code>AVAILABLE</code>.</p> </note> <p>Valid values:</p> <dl> <dt>AVAILABLE</dt> <dd> <p>The domain name is available.</p> </dd> <dt>AVAILABLE<em>RESERVED</dt> <dd> <p>The domain name is reserved under specific conditions.</p> </dd> <dt>AVAILABLE</em>PREORDER</dt> <dd> <p>The domain name is available and can be preordered.</p> </dd> <dt>DONT<em>KNOW</dt> <dd> <p>The TLD registry didn&#39;t reply with a definitive answer about whether the domain name is available. Amazon Route 53 can return this response for a variety of reasons, for example, the registry is performing maintenance. Try again later.</p> </dd> <dt>PENDING</dt> <dd> <p>The TLD registry didn&#39;t return a response in the expected amount of time. When the response is delayed, it usually takes just a few extra seconds. You can resubmit the request immediately.</p> </dd> <dt>RESERVED</dt> <dd> <p>The domain name has been reserved for another person or organization.</p> </dd> <dt>UNAVAILABLE</dt> <dd> <p>The domain name is not available.</p> </dd> <dt>UNAVAILABLE</em>PREMIUM</dt> <dd> <p>The domain name is not available.</p> </dd> <dt>UNAVAILABLE_RESTRICTED</dt> <dd> <p>The domain name is forbidden.</p> </dd> </dl></p>
    #[serde(rename = "Availability")]
    pub availability: String,
}

/// <p>The CheckDomainTransferability request contains the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DeleteTagsForDomainRequest {
    /// <p>The domain for which you want to delete one or more tags.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>A list of tag keys to delete.</p>
    #[serde(rename = "TagsToDelete")]
    pub tags_to_delete: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteTagsForDomainResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableDomainAutoRenewRequest {
    /// <p>The name of the domain that you want to disable automatic renewal for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisableDomainAutoRenewResponse {}

/// <p>The DisableDomainTransferLock request includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableDomainTransferLockRequest {
    /// <p>The name of the domain that you want to remove the transfer lock for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The DisableDomainTransferLock response includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisableDomainTransferLockResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>Information about one suggested domain name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DomainTransferability {
    #[serde(rename = "Transferable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferable: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableDomainAutoRenewRequest {
    /// <p>The name of the domain that you want to enable automatic renewal for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnableDomainAutoRenewResponse {}

/// <p>A request to set the transfer lock for the specified domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableDomainTransferLockRequest {
    /// <p>The name of the domain that you want to set the transfer lock for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The EnableDomainTransferLock response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnableDomainTransferLockResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use GetOperationDetail.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>ExtraParam includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtraParam {
    /// <p>Name of the additional parameter required by the top-level domain.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Values corresponding to the additional parameter names required by some top-level domains.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetContactReachabilityStatusRequest {
    /// <p>The name of the domain for which you want to know whether the registrant contact has confirmed that the email address is valid.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetDomainDetailRequest {
    /// <p>The name of the domain that you want to get detailed information about.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The GetDomainDetail response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Specifies whether contact information for the admin contact is concealed from WHOIS queries. If the value is <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p>
    #[serde(rename = "AdminPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_privacy: Option<bool>,
    /// <p>Specifies whether the domain registration is set to renew automatically.</p>
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// <p>The date when the domain was created as found in the response to a WHOIS query. The date format is Unix time.</p>
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
    /// <p>The date when the registration for the domain is set to expire. The date format is Unix time.</p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>The name of the domain.</p>
    #[serde(rename = "Nameservers")]
    pub nameservers: Vec<Nameserver>,
    /// <p>Provides details about the domain registrant.</p>
    #[serde(rename = "RegistrantContact")]
    pub registrant_contact: ContactDetail,
    /// <p>Specifies whether contact information for the registrant contact is concealed from WHOIS queries. If the value is <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p>
    #[serde(rename = "RegistrantPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_privacy: Option<bool>,
    /// <p>Name of the registrar of the domain as identified in the registry. Amazon Route 53 domains are registered by registrar Gandi. The value is <code>"GANDI SAS"</code>. </p>
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
    /// <p>Specifies whether contact information for the tech contact is concealed from WHOIS queries. If the value is <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p>
    #[serde(rename = "TechPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_privacy: Option<bool>,
    /// <p>The last updated date of the domain as found in the response to a WHOIS query. The date format is Unix time.</p>
    #[serde(rename = "UpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<f64>,
    /// <p>The fully qualified name of the WHOIS server that can answer the WHOIS query for the domain.</p>
    #[serde(rename = "WhoIsServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_is_server: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct GetDomainSuggestionsResponse {
    /// <p>A list of possible domain names. If you specified <code>true</code> for <code>OnlyAvailable</code> in the request, the list contains only domains that are available for registration.</p>
    #[serde(rename = "SuggestionsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions_list: Option<Vec<DomainSuggestion>>,
}

/// <p>The <a>GetOperationDetail</a> request includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOperationDetailRequest {
    /// <p>The identifier for the operation for which you want to get the status. Amazon Route 53 returned the identifier in the response to the original request.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>The GetOperationDetail response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct ListOperationsRequest {
    /// <p>For an initial request for a list of operations, omit this element. If the number of operations that are not yet complete is greater than the value that you specified for <code>MaxItems</code>, you can use <code>Marker</code> to return additional operations. Get the value of <code>NextPageMarker</code> from the previous response, and submit another request that includes the value of <code>NextPageMarker</code> in the <code>Marker</code> element.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Number of domains to be returned.</p> <p>Default: 20</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

/// <p>The ListOperations response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct ListTagsForDomainRequest {
    /// <p>The domain for which you want to get a list of tags.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The ListTagsForDomain response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "PrivacyProtectAdminContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_admin_contact: Option<bool>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "PrivacyProtectRegistrantContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_registrant_contact: Option<bool>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p> <p>Default: <code>true</code> </p>
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
pub struct RegisterDomainResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>A <code>RenewDomain</code> request includes the number of years that you want to renew for and the current expiration year.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct RenewDomainResponse {
    /// <p>The identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResendContactReachabilityEmailRequest {
    /// <p>The name of the domain for which you want Amazon Route 53 to resend a confirmation email to the registrant contact.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct RetrieveDomainAuthCodeRequest {
    /// <p>The name of the domain that you want to get an authorization code for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The RetrieveDomainAuthCode response includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "PrivacyProtectAdminContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_admin_contact: Option<bool>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "PrivacyProtectRegistrantContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_registrant_contact: Option<bool>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p> <p>Default: <code>true</code> </p>
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
pub struct TransferDomainResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>The UpdateDomainContactPrivacy request includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDomainContactPrivacyRequest {
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p>
    #[serde(rename = "AdminPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_privacy: Option<bool>,
    /// <p>The name of the domain that you want to update the privacy setting for.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p>
    #[serde(rename = "RegistrantPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_privacy: Option<bool>,
    /// <p>Whether you want to conceal contact information from WHOIS queries. If you specify <code>true</code>, WHOIS ("who is") queries will return contact information for our registrar partner, Gandi, instead of the contact information that you enter.</p>
    #[serde(rename = "TechPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_privacy: Option<bool>,
}

/// <p>The UpdateDomainContactPrivacy response includes the following element.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateDomainContactPrivacyResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use GetOperationDetail.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>The UpdateDomainContact request includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateDomainContactResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>Replaces the current set of name servers for the domain with the specified set of name servers. If you use Amazon Route 53 as your DNS service, specify the four name servers in the delegation set for the hosted zone for the domain.</p> <p>If successful, this operation returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateDomainNameserversResponse {
    /// <p>Identifier for tracking the progress of the request. To use this ID to query the operation status, use <a>GetOperationDetail</a>.</p>
    #[serde(rename = "OperationId")]
    pub operation_id: String,
}

/// <p>The UpdateTagsForDomainRequest includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateTagsForDomainResponse {}

/// <p>The ViewBilling request includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ViewBillingRequest {
    /// <p>The end date and time for the time period for which you want a list of billing records. Specify the date in Unix time format.</p>
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
    /// <p>The beginning date and time for the time period for which you want a list of billing records. Specify the date in Unix time format.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// <p>The ViewBilling response includes the following elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CheckDomainAvailabilityError {
    pub fn from_body(body: &str) -> CheckDomainAvailabilityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        CheckDomainAvailabilityError::InvalidInput(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        CheckDomainAvailabilityError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        CheckDomainAvailabilityError::Validation(error_message.to_string())
                    }
                    _ => CheckDomainAvailabilityError::Unknown(String::from(body)),
                }
            }
            Err(_) => CheckDomainAvailabilityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CheckDomainAvailabilityError {
    fn from(err: serde_json::error::Error) -> CheckDomainAvailabilityError {
        CheckDomainAvailabilityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CheckDomainAvailabilityError {
    fn from(err: CredentialsError) -> CheckDomainAvailabilityError {
        CheckDomainAvailabilityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CheckDomainAvailabilityError {
    fn from(err: HttpDispatchError) -> CheckDomainAvailabilityError {
        CheckDomainAvailabilityError::HttpDispatch(err)
    }
}
impl From<io::Error> for CheckDomainAvailabilityError {
    fn from(err: io::Error) -> CheckDomainAvailabilityError {
        CheckDomainAvailabilityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CheckDomainAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CheckDomainAvailabilityError {
    fn description(&self) -> &str {
        match *self {
            CheckDomainAvailabilityError::InvalidInput(ref cause) => cause,
            CheckDomainAvailabilityError::UnsupportedTLD(ref cause) => cause,
            CheckDomainAvailabilityError::Validation(ref cause) => cause,
            CheckDomainAvailabilityError::Credentials(ref err) => err.description(),
            CheckDomainAvailabilityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CheckDomainAvailabilityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CheckDomainTransferability
#[derive(Debug, PartialEq)]
pub enum CheckDomainTransferabilityError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CheckDomainTransferabilityError {
    pub fn from_body(body: &str) -> CheckDomainTransferabilityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        CheckDomainTransferabilityError::InvalidInput(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        CheckDomainTransferabilityError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        CheckDomainTransferabilityError::Validation(error_message.to_string())
                    }
                    _ => CheckDomainTransferabilityError::Unknown(String::from(body)),
                }
            }
            Err(_) => CheckDomainTransferabilityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CheckDomainTransferabilityError {
    fn from(err: serde_json::error::Error) -> CheckDomainTransferabilityError {
        CheckDomainTransferabilityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CheckDomainTransferabilityError {
    fn from(err: CredentialsError) -> CheckDomainTransferabilityError {
        CheckDomainTransferabilityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CheckDomainTransferabilityError {
    fn from(err: HttpDispatchError) -> CheckDomainTransferabilityError {
        CheckDomainTransferabilityError::HttpDispatch(err)
    }
}
impl From<io::Error> for CheckDomainTransferabilityError {
    fn from(err: io::Error) -> CheckDomainTransferabilityError {
        CheckDomainTransferabilityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CheckDomainTransferabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CheckDomainTransferabilityError {
    fn description(&self) -> &str {
        match *self {
            CheckDomainTransferabilityError::InvalidInput(ref cause) => cause,
            CheckDomainTransferabilityError::UnsupportedTLD(ref cause) => cause,
            CheckDomainTransferabilityError::Validation(ref cause) => cause,
            CheckDomainTransferabilityError::Credentials(ref err) => err.description(),
            CheckDomainTransferabilityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CheckDomainTransferabilityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTagsForDomain
#[derive(Debug, PartialEq)]
pub enum DeleteTagsForDomainError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTagsForDomainError {
    pub fn from_body(body: &str) -> DeleteTagsForDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        DeleteTagsForDomainError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => DeleteTagsForDomainError::OperationLimitExceeded(
                        String::from(error_message),
                    ),
                    "UnsupportedTLD" => {
                        DeleteTagsForDomainError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteTagsForDomainError::Validation(error_message.to_string())
                    }
                    _ => DeleteTagsForDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTagsForDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTagsForDomainError {
    fn from(err: serde_json::error::Error) -> DeleteTagsForDomainError {
        DeleteTagsForDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTagsForDomainError {
    fn from(err: CredentialsError) -> DeleteTagsForDomainError {
        DeleteTagsForDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTagsForDomainError {
    fn from(err: HttpDispatchError) -> DeleteTagsForDomainError {
        DeleteTagsForDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTagsForDomainError {
    fn from(err: io::Error) -> DeleteTagsForDomainError {
        DeleteTagsForDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTagsForDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsForDomainError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagsForDomainError::InvalidInput(ref cause) => cause,
            DeleteTagsForDomainError::OperationLimitExceeded(ref cause) => cause,
            DeleteTagsForDomainError::UnsupportedTLD(ref cause) => cause,
            DeleteTagsForDomainError::Validation(ref cause) => cause,
            DeleteTagsForDomainError::Credentials(ref err) => err.description(),
            DeleteTagsForDomainError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteTagsForDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableDomainAutoRenew
#[derive(Debug, PartialEq)]
pub enum DisableDomainAutoRenewError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableDomainAutoRenewError {
    pub fn from_body(body: &str) -> DisableDomainAutoRenewError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        DisableDomainAutoRenewError::InvalidInput(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        DisableDomainAutoRenewError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisableDomainAutoRenewError::Validation(error_message.to_string())
                    }
                    _ => DisableDomainAutoRenewError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableDomainAutoRenewError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableDomainAutoRenewError {
    fn from(err: serde_json::error::Error) -> DisableDomainAutoRenewError {
        DisableDomainAutoRenewError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableDomainAutoRenewError {
    fn from(err: CredentialsError) -> DisableDomainAutoRenewError {
        DisableDomainAutoRenewError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableDomainAutoRenewError {
    fn from(err: HttpDispatchError) -> DisableDomainAutoRenewError {
        DisableDomainAutoRenewError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableDomainAutoRenewError {
    fn from(err: io::Error) -> DisableDomainAutoRenewError {
        DisableDomainAutoRenewError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableDomainAutoRenewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableDomainAutoRenewError {
    fn description(&self) -> &str {
        match *self {
            DisableDomainAutoRenewError::InvalidInput(ref cause) => cause,
            DisableDomainAutoRenewError::UnsupportedTLD(ref cause) => cause,
            DisableDomainAutoRenewError::Validation(ref cause) => cause,
            DisableDomainAutoRenewError::Credentials(ref err) => err.description(),
            DisableDomainAutoRenewError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisableDomainAutoRenewError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableDomainTransferLockError {
    pub fn from_body(body: &str) -> DisableDomainTransferLockError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateRequest" => DisableDomainTransferLockError::DuplicateRequest(
                        String::from(error_message),
                    ),
                    "InvalidInput" => {
                        DisableDomainTransferLockError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => {
                        DisableDomainTransferLockError::OperationLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "TLDRulesViolation" => DisableDomainTransferLockError::TLDRulesViolation(
                        String::from(error_message),
                    ),
                    "UnsupportedTLD" => {
                        DisableDomainTransferLockError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisableDomainTransferLockError::Validation(error_message.to_string())
                    }
                    _ => DisableDomainTransferLockError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableDomainTransferLockError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableDomainTransferLockError {
    fn from(err: serde_json::error::Error) -> DisableDomainTransferLockError {
        DisableDomainTransferLockError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableDomainTransferLockError {
    fn from(err: CredentialsError) -> DisableDomainTransferLockError {
        DisableDomainTransferLockError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableDomainTransferLockError {
    fn from(err: HttpDispatchError) -> DisableDomainTransferLockError {
        DisableDomainTransferLockError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableDomainTransferLockError {
    fn from(err: io::Error) -> DisableDomainTransferLockError {
        DisableDomainTransferLockError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableDomainTransferLockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableDomainTransferLockError {
    fn description(&self) -> &str {
        match *self {
            DisableDomainTransferLockError::DuplicateRequest(ref cause) => cause,
            DisableDomainTransferLockError::InvalidInput(ref cause) => cause,
            DisableDomainTransferLockError::OperationLimitExceeded(ref cause) => cause,
            DisableDomainTransferLockError::TLDRulesViolation(ref cause) => cause,
            DisableDomainTransferLockError::UnsupportedTLD(ref cause) => cause,
            DisableDomainTransferLockError::Validation(ref cause) => cause,
            DisableDomainTransferLockError::Credentials(ref err) => err.description(),
            DisableDomainTransferLockError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisableDomainTransferLockError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableDomainAutoRenew
#[derive(Debug, PartialEq)]
pub enum EnableDomainAutoRenewError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The top-level domain does not support this operation.</p>
    TLDRulesViolation(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableDomainAutoRenewError {
    pub fn from_body(body: &str) -> EnableDomainAutoRenewError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        EnableDomainAutoRenewError::InvalidInput(String::from(error_message))
                    }
                    "TLDRulesViolation" => {
                        EnableDomainAutoRenewError::TLDRulesViolation(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        EnableDomainAutoRenewError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        EnableDomainAutoRenewError::Validation(error_message.to_string())
                    }
                    _ => EnableDomainAutoRenewError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableDomainAutoRenewError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableDomainAutoRenewError {
    fn from(err: serde_json::error::Error) -> EnableDomainAutoRenewError {
        EnableDomainAutoRenewError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableDomainAutoRenewError {
    fn from(err: CredentialsError) -> EnableDomainAutoRenewError {
        EnableDomainAutoRenewError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableDomainAutoRenewError {
    fn from(err: HttpDispatchError) -> EnableDomainAutoRenewError {
        EnableDomainAutoRenewError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableDomainAutoRenewError {
    fn from(err: io::Error) -> EnableDomainAutoRenewError {
        EnableDomainAutoRenewError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableDomainAutoRenewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableDomainAutoRenewError {
    fn description(&self) -> &str {
        match *self {
            EnableDomainAutoRenewError::InvalidInput(ref cause) => cause,
            EnableDomainAutoRenewError::TLDRulesViolation(ref cause) => cause,
            EnableDomainAutoRenewError::UnsupportedTLD(ref cause) => cause,
            EnableDomainAutoRenewError::Validation(ref cause) => cause,
            EnableDomainAutoRenewError::Credentials(ref err) => err.description(),
            EnableDomainAutoRenewError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableDomainAutoRenewError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableDomainTransferLockError {
    pub fn from_body(body: &str) -> EnableDomainTransferLockError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateRequest" => {
                        EnableDomainTransferLockError::DuplicateRequest(String::from(error_message))
                    }
                    "InvalidInput" => {
                        EnableDomainTransferLockError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => {
                        EnableDomainTransferLockError::OperationLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "TLDRulesViolation" => EnableDomainTransferLockError::TLDRulesViolation(
                        String::from(error_message),
                    ),
                    "UnsupportedTLD" => {
                        EnableDomainTransferLockError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        EnableDomainTransferLockError::Validation(error_message.to_string())
                    }
                    _ => EnableDomainTransferLockError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableDomainTransferLockError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableDomainTransferLockError {
    fn from(err: serde_json::error::Error) -> EnableDomainTransferLockError {
        EnableDomainTransferLockError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableDomainTransferLockError {
    fn from(err: CredentialsError) -> EnableDomainTransferLockError {
        EnableDomainTransferLockError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableDomainTransferLockError {
    fn from(err: HttpDispatchError) -> EnableDomainTransferLockError {
        EnableDomainTransferLockError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableDomainTransferLockError {
    fn from(err: io::Error) -> EnableDomainTransferLockError {
        EnableDomainTransferLockError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableDomainTransferLockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableDomainTransferLockError {
    fn description(&self) -> &str {
        match *self {
            EnableDomainTransferLockError::DuplicateRequest(ref cause) => cause,
            EnableDomainTransferLockError::InvalidInput(ref cause) => cause,
            EnableDomainTransferLockError::OperationLimitExceeded(ref cause) => cause,
            EnableDomainTransferLockError::TLDRulesViolation(ref cause) => cause,
            EnableDomainTransferLockError::UnsupportedTLD(ref cause) => cause,
            EnableDomainTransferLockError::Validation(ref cause) => cause,
            EnableDomainTransferLockError::Credentials(ref err) => err.description(),
            EnableDomainTransferLockError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableDomainTransferLockError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetContactReachabilityStatus
#[derive(Debug, PartialEq)]
pub enum GetContactReachabilityStatusError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetContactReachabilityStatusError {
    pub fn from_body(body: &str) -> GetContactReachabilityStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        GetContactReachabilityStatusError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => {
                        GetContactReachabilityStatusError::OperationLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "UnsupportedTLD" => GetContactReachabilityStatusError::UnsupportedTLD(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        GetContactReachabilityStatusError::Validation(error_message.to_string())
                    }
                    _ => GetContactReachabilityStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetContactReachabilityStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetContactReachabilityStatusError {
    fn from(err: serde_json::error::Error) -> GetContactReachabilityStatusError {
        GetContactReachabilityStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetContactReachabilityStatusError {
    fn from(err: CredentialsError) -> GetContactReachabilityStatusError {
        GetContactReachabilityStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetContactReachabilityStatusError {
    fn from(err: HttpDispatchError) -> GetContactReachabilityStatusError {
        GetContactReachabilityStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetContactReachabilityStatusError {
    fn from(err: io::Error) -> GetContactReachabilityStatusError {
        GetContactReachabilityStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetContactReachabilityStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetContactReachabilityStatusError {
    fn description(&self) -> &str {
        match *self {
            GetContactReachabilityStatusError::InvalidInput(ref cause) => cause,
            GetContactReachabilityStatusError::OperationLimitExceeded(ref cause) => cause,
            GetContactReachabilityStatusError::UnsupportedTLD(ref cause) => cause,
            GetContactReachabilityStatusError::Validation(ref cause) => cause,
            GetContactReachabilityStatusError::Credentials(ref err) => err.description(),
            GetContactReachabilityStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetContactReachabilityStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomainDetail
#[derive(Debug, PartialEq)]
pub enum GetDomainDetailError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDomainDetailError {
    pub fn from_body(body: &str) -> GetDomainDetailError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        GetDomainDetailError::InvalidInput(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        GetDomainDetailError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDomainDetailError::Validation(error_message.to_string())
                    }
                    _ => GetDomainDetailError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDomainDetailError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDomainDetailError {
    fn from(err: serde_json::error::Error) -> GetDomainDetailError {
        GetDomainDetailError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDomainDetailError {
    fn from(err: CredentialsError) -> GetDomainDetailError {
        GetDomainDetailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDomainDetailError {
    fn from(err: HttpDispatchError) -> GetDomainDetailError {
        GetDomainDetailError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDomainDetailError {
    fn from(err: io::Error) -> GetDomainDetailError {
        GetDomainDetailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDomainDetailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainDetailError {
    fn description(&self) -> &str {
        match *self {
            GetDomainDetailError::InvalidInput(ref cause) => cause,
            GetDomainDetailError::UnsupportedTLD(ref cause) => cause,
            GetDomainDetailError::Validation(ref cause) => cause,
            GetDomainDetailError::Credentials(ref err) => err.description(),
            GetDomainDetailError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDomainDetailError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomainSuggestions
#[derive(Debug, PartialEq)]
pub enum GetDomainSuggestionsError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDomainSuggestionsError {
    pub fn from_body(body: &str) -> GetDomainSuggestionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        GetDomainSuggestionsError::InvalidInput(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        GetDomainSuggestionsError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDomainSuggestionsError::Validation(error_message.to_string())
                    }
                    _ => GetDomainSuggestionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDomainSuggestionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDomainSuggestionsError {
    fn from(err: serde_json::error::Error) -> GetDomainSuggestionsError {
        GetDomainSuggestionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDomainSuggestionsError {
    fn from(err: CredentialsError) -> GetDomainSuggestionsError {
        GetDomainSuggestionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDomainSuggestionsError {
    fn from(err: HttpDispatchError) -> GetDomainSuggestionsError {
        GetDomainSuggestionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDomainSuggestionsError {
    fn from(err: io::Error) -> GetDomainSuggestionsError {
        GetDomainSuggestionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDomainSuggestionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainSuggestionsError {
    fn description(&self) -> &str {
        match *self {
            GetDomainSuggestionsError::InvalidInput(ref cause) => cause,
            GetDomainSuggestionsError::UnsupportedTLD(ref cause) => cause,
            GetDomainSuggestionsError::Validation(ref cause) => cause,
            GetDomainSuggestionsError::Credentials(ref err) => err.description(),
            GetDomainSuggestionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDomainSuggestionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOperationDetail
#[derive(Debug, PartialEq)]
pub enum GetOperationDetailError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetOperationDetailError {
    pub fn from_body(body: &str) -> GetOperationDetailError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        GetOperationDetailError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetOperationDetailError::Validation(error_message.to_string())
                    }
                    _ => GetOperationDetailError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetOperationDetailError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetOperationDetailError {
    fn from(err: serde_json::error::Error) -> GetOperationDetailError {
        GetOperationDetailError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetOperationDetailError {
    fn from(err: CredentialsError) -> GetOperationDetailError {
        GetOperationDetailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetOperationDetailError {
    fn from(err: HttpDispatchError) -> GetOperationDetailError {
        GetOperationDetailError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetOperationDetailError {
    fn from(err: io::Error) -> GetOperationDetailError {
        GetOperationDetailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetOperationDetailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOperationDetailError {
    fn description(&self) -> &str {
        match *self {
            GetOperationDetailError::InvalidInput(ref cause) => cause,
            GetOperationDetailError::Validation(ref cause) => cause,
            GetOperationDetailError::Credentials(ref err) => err.description(),
            GetOperationDetailError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetOperationDetailError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDomains
#[derive(Debug, PartialEq)]
pub enum ListDomainsError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDomainsError {
    pub fn from_body(body: &str) -> ListDomainsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => ListDomainsError::InvalidInput(String::from(error_message)),
                    "ValidationException" => {
                        ListDomainsError::Validation(error_message.to_string())
                    }
                    _ => ListDomainsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDomainsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDomainsError {
    fn from(err: serde_json::error::Error) -> ListDomainsError {
        ListDomainsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDomainsError {
    fn from(err: CredentialsError) -> ListDomainsError {
        ListDomainsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDomainsError {
    fn from(err: HttpDispatchError) -> ListDomainsError {
        ListDomainsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDomainsError {
    fn from(err: io::Error) -> ListDomainsError {
        ListDomainsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDomainsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDomainsError {
    fn description(&self) -> &str {
        match *self {
            ListDomainsError::InvalidInput(ref cause) => cause,
            ListDomainsError::Validation(ref cause) => cause,
            ListDomainsError::Credentials(ref err) => err.description(),
            ListDomainsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDomainsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOperations
#[derive(Debug, PartialEq)]
pub enum ListOperationsError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListOperationsError {
    pub fn from_body(body: &str) -> ListOperationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        ListOperationsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListOperationsError::Validation(error_message.to_string())
                    }
                    _ => ListOperationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListOperationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListOperationsError {
    fn from(err: serde_json::error::Error) -> ListOperationsError {
        ListOperationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOperationsError {
    fn from(err: CredentialsError) -> ListOperationsError {
        ListOperationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOperationsError {
    fn from(err: HttpDispatchError) -> ListOperationsError {
        ListOperationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOperationsError {
    fn from(err: io::Error) -> ListOperationsError {
        ListOperationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOperationsError {
    fn description(&self) -> &str {
        match *self {
            ListOperationsError::InvalidInput(ref cause) => cause,
            ListOperationsError::Validation(ref cause) => cause,
            ListOperationsError::Credentials(ref err) => err.description(),
            ListOperationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListOperationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForDomain
#[derive(Debug, PartialEq)]
pub enum ListTagsForDomainError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsForDomainError {
    pub fn from_body(body: &str) -> ListTagsForDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        ListTagsForDomainError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => {
                        ListTagsForDomainError::OperationLimitExceeded(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        ListTagsForDomainError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsForDomainError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForDomainError {
    fn from(err: serde_json::error::Error) -> ListTagsForDomainError {
        ListTagsForDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForDomainError {
    fn from(err: CredentialsError) -> ListTagsForDomainError {
        ListTagsForDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForDomainError {
    fn from(err: HttpDispatchError) -> ListTagsForDomainError {
        ListTagsForDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForDomainError {
    fn from(err: io::Error) -> ListTagsForDomainError {
        ListTagsForDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForDomainError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForDomainError::InvalidInput(ref cause) => cause,
            ListTagsForDomainError::OperationLimitExceeded(ref cause) => cause,
            ListTagsForDomainError::UnsupportedTLD(ref cause) => cause,
            ListTagsForDomainError::Validation(ref cause) => cause,
            ListTagsForDomainError::Credentials(ref err) => err.description(),
            ListTagsForDomainError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForDomainError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterDomainError {
    pub fn from_body(body: &str) -> RegisterDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DomainLimitExceeded" => {
                        RegisterDomainError::DomainLimitExceeded(String::from(error_message))
                    }
                    "DuplicateRequest" => {
                        RegisterDomainError::DuplicateRequest(String::from(error_message))
                    }
                    "InvalidInput" => {
                        RegisterDomainError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => {
                        RegisterDomainError::OperationLimitExceeded(String::from(error_message))
                    }
                    "TLDRulesViolation" => {
                        RegisterDomainError::TLDRulesViolation(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        RegisterDomainError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterDomainError::Validation(error_message.to_string())
                    }
                    _ => RegisterDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterDomainError {
    fn from(err: serde_json::error::Error) -> RegisterDomainError {
        RegisterDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterDomainError {
    fn from(err: CredentialsError) -> RegisterDomainError {
        RegisterDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterDomainError {
    fn from(err: HttpDispatchError) -> RegisterDomainError {
        RegisterDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterDomainError {
    fn from(err: io::Error) -> RegisterDomainError {
        RegisterDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterDomainError {
    fn description(&self) -> &str {
        match *self {
            RegisterDomainError::DomainLimitExceeded(ref cause) => cause,
            RegisterDomainError::DuplicateRequest(ref cause) => cause,
            RegisterDomainError::InvalidInput(ref cause) => cause,
            RegisterDomainError::OperationLimitExceeded(ref cause) => cause,
            RegisterDomainError::TLDRulesViolation(ref cause) => cause,
            RegisterDomainError::UnsupportedTLD(ref cause) => cause,
            RegisterDomainError::Validation(ref cause) => cause,
            RegisterDomainError::Credentials(ref err) => err.description(),
            RegisterDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RegisterDomainError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RenewDomainError {
    pub fn from_body(body: &str) -> RenewDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateRequest" => {
                        RenewDomainError::DuplicateRequest(String::from(error_message))
                    }
                    "InvalidInput" => RenewDomainError::InvalidInput(String::from(error_message)),
                    "OperationLimitExceeded" => {
                        RenewDomainError::OperationLimitExceeded(String::from(error_message))
                    }
                    "TLDRulesViolation" => {
                        RenewDomainError::TLDRulesViolation(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        RenewDomainError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        RenewDomainError::Validation(error_message.to_string())
                    }
                    _ => RenewDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => RenewDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RenewDomainError {
    fn from(err: serde_json::error::Error) -> RenewDomainError {
        RenewDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RenewDomainError {
    fn from(err: CredentialsError) -> RenewDomainError {
        RenewDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RenewDomainError {
    fn from(err: HttpDispatchError) -> RenewDomainError {
        RenewDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for RenewDomainError {
    fn from(err: io::Error) -> RenewDomainError {
        RenewDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RenewDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RenewDomainError {
    fn description(&self) -> &str {
        match *self {
            RenewDomainError::DuplicateRequest(ref cause) => cause,
            RenewDomainError::InvalidInput(ref cause) => cause,
            RenewDomainError::OperationLimitExceeded(ref cause) => cause,
            RenewDomainError::TLDRulesViolation(ref cause) => cause,
            RenewDomainError::UnsupportedTLD(ref cause) => cause,
            RenewDomainError::Validation(ref cause) => cause,
            RenewDomainError::Credentials(ref err) => err.description(),
            RenewDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RenewDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResendContactReachabilityEmail
#[derive(Debug, PartialEq)]
pub enum ResendContactReachabilityEmailError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ResendContactReachabilityEmailError {
    pub fn from_body(body: &str) -> ResendContactReachabilityEmailError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => ResendContactReachabilityEmailError::InvalidInput(
                        String::from(error_message),
                    ),
                    "OperationLimitExceeded" => {
                        ResendContactReachabilityEmailError::OperationLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "UnsupportedTLD" => ResendContactReachabilityEmailError::UnsupportedTLD(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        ResendContactReachabilityEmailError::Validation(error_message.to_string())
                    }
                    _ => ResendContactReachabilityEmailError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResendContactReachabilityEmailError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ResendContactReachabilityEmailError {
    fn from(err: serde_json::error::Error) -> ResendContactReachabilityEmailError {
        ResendContactReachabilityEmailError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ResendContactReachabilityEmailError {
    fn from(err: CredentialsError) -> ResendContactReachabilityEmailError {
        ResendContactReachabilityEmailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResendContactReachabilityEmailError {
    fn from(err: HttpDispatchError) -> ResendContactReachabilityEmailError {
        ResendContactReachabilityEmailError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResendContactReachabilityEmailError {
    fn from(err: io::Error) -> ResendContactReachabilityEmailError {
        ResendContactReachabilityEmailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResendContactReachabilityEmailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResendContactReachabilityEmailError {
    fn description(&self) -> &str {
        match *self {
            ResendContactReachabilityEmailError::InvalidInput(ref cause) => cause,
            ResendContactReachabilityEmailError::OperationLimitExceeded(ref cause) => cause,
            ResendContactReachabilityEmailError::UnsupportedTLD(ref cause) => cause,
            ResendContactReachabilityEmailError::Validation(ref cause) => cause,
            ResendContactReachabilityEmailError::Credentials(ref err) => err.description(),
            ResendContactReachabilityEmailError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ResendContactReachabilityEmailError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RetrieveDomainAuthCode
#[derive(Debug, PartialEq)]
pub enum RetrieveDomainAuthCodeError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RetrieveDomainAuthCodeError {
    pub fn from_body(body: &str) -> RetrieveDomainAuthCodeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        RetrieveDomainAuthCodeError::InvalidInput(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        RetrieveDomainAuthCodeError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        RetrieveDomainAuthCodeError::Validation(error_message.to_string())
                    }
                    _ => RetrieveDomainAuthCodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => RetrieveDomainAuthCodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RetrieveDomainAuthCodeError {
    fn from(err: serde_json::error::Error) -> RetrieveDomainAuthCodeError {
        RetrieveDomainAuthCodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RetrieveDomainAuthCodeError {
    fn from(err: CredentialsError) -> RetrieveDomainAuthCodeError {
        RetrieveDomainAuthCodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RetrieveDomainAuthCodeError {
    fn from(err: HttpDispatchError) -> RetrieveDomainAuthCodeError {
        RetrieveDomainAuthCodeError::HttpDispatch(err)
    }
}
impl From<io::Error> for RetrieveDomainAuthCodeError {
    fn from(err: io::Error) -> RetrieveDomainAuthCodeError {
        RetrieveDomainAuthCodeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RetrieveDomainAuthCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RetrieveDomainAuthCodeError {
    fn description(&self) -> &str {
        match *self {
            RetrieveDomainAuthCodeError::InvalidInput(ref cause) => cause,
            RetrieveDomainAuthCodeError::UnsupportedTLD(ref cause) => cause,
            RetrieveDomainAuthCodeError::Validation(ref cause) => cause,
            RetrieveDomainAuthCodeError::Credentials(ref err) => err.description(),
            RetrieveDomainAuthCodeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RetrieveDomainAuthCodeError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TransferDomainError {
    pub fn from_body(body: &str) -> TransferDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DomainLimitExceeded" => {
                        TransferDomainError::DomainLimitExceeded(String::from(error_message))
                    }
                    "DuplicateRequest" => {
                        TransferDomainError::DuplicateRequest(String::from(error_message))
                    }
                    "InvalidInput" => {
                        TransferDomainError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => {
                        TransferDomainError::OperationLimitExceeded(String::from(error_message))
                    }
                    "TLDRulesViolation" => {
                        TransferDomainError::TLDRulesViolation(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        TransferDomainError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        TransferDomainError::Validation(error_message.to_string())
                    }
                    _ => TransferDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => TransferDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TransferDomainError {
    fn from(err: serde_json::error::Error) -> TransferDomainError {
        TransferDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TransferDomainError {
    fn from(err: CredentialsError) -> TransferDomainError {
        TransferDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TransferDomainError {
    fn from(err: HttpDispatchError) -> TransferDomainError {
        TransferDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for TransferDomainError {
    fn from(err: io::Error) -> TransferDomainError {
        TransferDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TransferDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TransferDomainError {
    fn description(&self) -> &str {
        match *self {
            TransferDomainError::DomainLimitExceeded(ref cause) => cause,
            TransferDomainError::DuplicateRequest(ref cause) => cause,
            TransferDomainError::InvalidInput(ref cause) => cause,
            TransferDomainError::OperationLimitExceeded(ref cause) => cause,
            TransferDomainError::TLDRulesViolation(ref cause) => cause,
            TransferDomainError::UnsupportedTLD(ref cause) => cause,
            TransferDomainError::Validation(ref cause) => cause,
            TransferDomainError::Credentials(ref err) => err.description(),
            TransferDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TransferDomainError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDomainContactError {
    pub fn from_body(body: &str) -> UpdateDomainContactError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateRequest" => {
                        UpdateDomainContactError::DuplicateRequest(String::from(error_message))
                    }
                    "InvalidInput" => {
                        UpdateDomainContactError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => UpdateDomainContactError::OperationLimitExceeded(
                        String::from(error_message),
                    ),
                    "TLDRulesViolation" => {
                        UpdateDomainContactError::TLDRulesViolation(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        UpdateDomainContactError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDomainContactError::Validation(error_message.to_string())
                    }
                    _ => UpdateDomainContactError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDomainContactError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDomainContactError {
    fn from(err: serde_json::error::Error) -> UpdateDomainContactError {
        UpdateDomainContactError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDomainContactError {
    fn from(err: CredentialsError) -> UpdateDomainContactError {
        UpdateDomainContactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDomainContactError {
    fn from(err: HttpDispatchError) -> UpdateDomainContactError {
        UpdateDomainContactError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDomainContactError {
    fn from(err: io::Error) -> UpdateDomainContactError {
        UpdateDomainContactError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDomainContactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDomainContactError {
    fn description(&self) -> &str {
        match *self {
            UpdateDomainContactError::DuplicateRequest(ref cause) => cause,
            UpdateDomainContactError::InvalidInput(ref cause) => cause,
            UpdateDomainContactError::OperationLimitExceeded(ref cause) => cause,
            UpdateDomainContactError::TLDRulesViolation(ref cause) => cause,
            UpdateDomainContactError::UnsupportedTLD(ref cause) => cause,
            UpdateDomainContactError::Validation(ref cause) => cause,
            UpdateDomainContactError::Credentials(ref err) => err.description(),
            UpdateDomainContactError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDomainContactError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDomainContactPrivacyError {
    pub fn from_body(body: &str) -> UpdateDomainContactPrivacyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateRequest" => UpdateDomainContactPrivacyError::DuplicateRequest(
                        String::from(error_message),
                    ),
                    "InvalidInput" => {
                        UpdateDomainContactPrivacyError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => {
                        UpdateDomainContactPrivacyError::OperationLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "TLDRulesViolation" => UpdateDomainContactPrivacyError::TLDRulesViolation(
                        String::from(error_message),
                    ),
                    "UnsupportedTLD" => {
                        UpdateDomainContactPrivacyError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDomainContactPrivacyError::Validation(error_message.to_string())
                    }
                    _ => UpdateDomainContactPrivacyError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDomainContactPrivacyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDomainContactPrivacyError {
    fn from(err: serde_json::error::Error) -> UpdateDomainContactPrivacyError {
        UpdateDomainContactPrivacyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDomainContactPrivacyError {
    fn from(err: CredentialsError) -> UpdateDomainContactPrivacyError {
        UpdateDomainContactPrivacyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDomainContactPrivacyError {
    fn from(err: HttpDispatchError) -> UpdateDomainContactPrivacyError {
        UpdateDomainContactPrivacyError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDomainContactPrivacyError {
    fn from(err: io::Error) -> UpdateDomainContactPrivacyError {
        UpdateDomainContactPrivacyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDomainContactPrivacyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDomainContactPrivacyError {
    fn description(&self) -> &str {
        match *self {
            UpdateDomainContactPrivacyError::DuplicateRequest(ref cause) => cause,
            UpdateDomainContactPrivacyError::InvalidInput(ref cause) => cause,
            UpdateDomainContactPrivacyError::OperationLimitExceeded(ref cause) => cause,
            UpdateDomainContactPrivacyError::TLDRulesViolation(ref cause) => cause,
            UpdateDomainContactPrivacyError::UnsupportedTLD(ref cause) => cause,
            UpdateDomainContactPrivacyError::Validation(ref cause) => cause,
            UpdateDomainContactPrivacyError::Credentials(ref err) => err.description(),
            UpdateDomainContactPrivacyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDomainContactPrivacyError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDomainNameserversError {
    pub fn from_body(body: &str) -> UpdateDomainNameserversError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateRequest" => {
                        UpdateDomainNameserversError::DuplicateRequest(String::from(error_message))
                    }
                    "InvalidInput" => {
                        UpdateDomainNameserversError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => {
                        UpdateDomainNameserversError::OperationLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "TLDRulesViolation" => {
                        UpdateDomainNameserversError::TLDRulesViolation(String::from(error_message))
                    }
                    "UnsupportedTLD" => {
                        UpdateDomainNameserversError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDomainNameserversError::Validation(error_message.to_string())
                    }
                    _ => UpdateDomainNameserversError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDomainNameserversError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDomainNameserversError {
    fn from(err: serde_json::error::Error) -> UpdateDomainNameserversError {
        UpdateDomainNameserversError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDomainNameserversError {
    fn from(err: CredentialsError) -> UpdateDomainNameserversError {
        UpdateDomainNameserversError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDomainNameserversError {
    fn from(err: HttpDispatchError) -> UpdateDomainNameserversError {
        UpdateDomainNameserversError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDomainNameserversError {
    fn from(err: io::Error) -> UpdateDomainNameserversError {
        UpdateDomainNameserversError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDomainNameserversError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDomainNameserversError {
    fn description(&self) -> &str {
        match *self {
            UpdateDomainNameserversError::DuplicateRequest(ref cause) => cause,
            UpdateDomainNameserversError::InvalidInput(ref cause) => cause,
            UpdateDomainNameserversError::OperationLimitExceeded(ref cause) => cause,
            UpdateDomainNameserversError::TLDRulesViolation(ref cause) => cause,
            UpdateDomainNameserversError::UnsupportedTLD(ref cause) => cause,
            UpdateDomainNameserversError::Validation(ref cause) => cause,
            UpdateDomainNameserversError::Credentials(ref err) => err.description(),
            UpdateDomainNameserversError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDomainNameserversError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTagsForDomain
#[derive(Debug, PartialEq)]
pub enum UpdateTagsForDomainError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(String),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTLD(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateTagsForDomainError {
    pub fn from_body(body: &str) -> UpdateTagsForDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => {
                        UpdateTagsForDomainError::InvalidInput(String::from(error_message))
                    }
                    "OperationLimitExceeded" => UpdateTagsForDomainError::OperationLimitExceeded(
                        String::from(error_message),
                    ),
                    "UnsupportedTLD" => {
                        UpdateTagsForDomainError::UnsupportedTLD(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateTagsForDomainError::Validation(error_message.to_string())
                    }
                    _ => UpdateTagsForDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateTagsForDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateTagsForDomainError {
    fn from(err: serde_json::error::Error) -> UpdateTagsForDomainError {
        UpdateTagsForDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateTagsForDomainError {
    fn from(err: CredentialsError) -> UpdateTagsForDomainError {
        UpdateTagsForDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTagsForDomainError {
    fn from(err: HttpDispatchError) -> UpdateTagsForDomainError {
        UpdateTagsForDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTagsForDomainError {
    fn from(err: io::Error) -> UpdateTagsForDomainError {
        UpdateTagsForDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTagsForDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTagsForDomainError {
    fn description(&self) -> &str {
        match *self {
            UpdateTagsForDomainError::InvalidInput(ref cause) => cause,
            UpdateTagsForDomainError::OperationLimitExceeded(ref cause) => cause,
            UpdateTagsForDomainError::UnsupportedTLD(ref cause) => cause,
            UpdateTagsForDomainError::Validation(ref cause) => cause,
            UpdateTagsForDomainError::Credentials(ref err) => err.description(),
            UpdateTagsForDomainError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateTagsForDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ViewBilling
#[derive(Debug, PartialEq)]
pub enum ViewBillingError {
    /// <p>The requested item is not acceptable. For example, for an OperationId it might refer to the ID of an operation that is already completed. For a domain name, it might not be a valid domain name or belong to the requester account.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ViewBillingError {
    pub fn from_body(body: &str) -> ViewBillingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInput" => ViewBillingError::InvalidInput(String::from(error_message)),
                    "ValidationException" => {
                        ViewBillingError::Validation(error_message.to_string())
                    }
                    _ => ViewBillingError::Unknown(String::from(body)),
                }
            }
            Err(_) => ViewBillingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ViewBillingError {
    fn from(err: serde_json::error::Error) -> ViewBillingError {
        ViewBillingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ViewBillingError {
    fn from(err: CredentialsError) -> ViewBillingError {
        ViewBillingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ViewBillingError {
    fn from(err: HttpDispatchError) -> ViewBillingError {
        ViewBillingError::HttpDispatch(err)
    }
}
impl From<io::Error> for ViewBillingError {
    fn from(err: io::Error) -> ViewBillingError {
        ViewBillingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ViewBillingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ViewBillingError {
    fn description(&self) -> &str {
        match *self {
            ViewBillingError::InvalidInput(ref cause) => cause,
            ViewBillingError::Validation(ref cause) => cause,
            ViewBillingError::Credentials(ref err) => err.description(),
            ViewBillingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ViewBillingError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Route 53 Domains API. Amazon Route 53 Domains clients implement this trait.
pub trait Route53Domains {
    /// <p>This operation checks the availability of one domain name. Note that if the availability status of a domain is pending, you must submit another request to determine the availability of the domain name.</p>
    fn check_domain_availability(
        &self,
        input: CheckDomainAvailabilityRequest,
    ) -> RusotoFuture<CheckDomainAvailabilityResponse, CheckDomainAvailabilityError>;

    /// <p>Checks whether a domain name can be transferred to Amazon Route 53. </p>
    fn check_domain_transferability(
        &self,
        input: CheckDomainTransferabilityRequest,
    ) -> RusotoFuture<CheckDomainTransferabilityResponse, CheckDomainTransferabilityError>;

    /// <p>This operation deletes the specified tags for a domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    fn delete_tags_for_domain(
        &self,
        input: DeleteTagsForDomainRequest,
    ) -> RusotoFuture<DeleteTagsForDomainResponse, DeleteTagsForDomainError>;

    /// <p>This operation disables automatic renewal of domain registration for the specified domain.</p>
    fn disable_domain_auto_renew(
        &self,
        input: DisableDomainAutoRenewRequest,
    ) -> RusotoFuture<DisableDomainAutoRenewResponse, DisableDomainAutoRenewError>;

    /// <p>This operation removes the transfer lock on the domain (specifically the <code>clientTransferProhibited</code> status) to allow domain transfers. We recommend you refrain from performing this action unless you intend to transfer the domain to a different registrar. Successful submission returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    fn disable_domain_transfer_lock(
        &self,
        input: DisableDomainTransferLockRequest,
    ) -> RusotoFuture<DisableDomainTransferLockResponse, DisableDomainTransferLockError>;

    /// <p>This operation configures Amazon Route 53 to automatically renew the specified domain before the domain registration expires. The cost of renewing your domain registration is billed to your AWS account.</p> <p>The period during which you can renew a domain name varies by TLD. For a list of TLDs and their renewal policies, see <a href="http://wiki.gandi.net/en/domains/renew#renewal_restoration_and_deletion_times">"Renewal, restoration, and deletion times"</a> on the website for our registrar partner, Gandi. Route 53 requires that you renew before the end of the renewal period that is listed on the Gandi website so we can complete processing before the deadline.</p>
    fn enable_domain_auto_renew(
        &self,
        input: EnableDomainAutoRenewRequest,
    ) -> RusotoFuture<EnableDomainAutoRenewResponse, EnableDomainAutoRenewError>;

    /// <p>This operation sets the transfer lock on the domain (specifically the <code>clientTransferProhibited</code> status) to prevent domain transfers. Successful submission returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    fn enable_domain_transfer_lock(
        &self,
        input: EnableDomainTransferLockRequest,
    ) -> RusotoFuture<EnableDomainTransferLockResponse, EnableDomainTransferLockError>;

    /// <p>For operations that require confirmation that the email address for the registrant contact is valid, such as registering a new domain, this operation returns information about whether the registrant contact has responded.</p> <p>If you want us to resend the email, use the <code>ResendContactReachabilityEmail</code> operation.</p>
    fn get_contact_reachability_status(
        &self,
        input: GetContactReachabilityStatusRequest,
    ) -> RusotoFuture<GetContactReachabilityStatusResponse, GetContactReachabilityStatusError>;

    /// <p>This operation returns detailed information about a specified domain that is associated with the current AWS account. Contact information for the domain is also returned as part of the output.</p>
    fn get_domain_detail(
        &self,
        input: GetDomainDetailRequest,
    ) -> RusotoFuture<GetDomainDetailResponse, GetDomainDetailError>;

    /// <p>The GetDomainSuggestions operation returns a list of suggested domain names given a string, which can either be a domain name or simply a word or phrase (without spaces).</p>
    fn get_domain_suggestions(
        &self,
        input: GetDomainSuggestionsRequest,
    ) -> RusotoFuture<GetDomainSuggestionsResponse, GetDomainSuggestionsError>;

    /// <p>This operation returns the current status of an operation that is not completed.</p>
    fn get_operation_detail(
        &self,
        input: GetOperationDetailRequest,
    ) -> RusotoFuture<GetOperationDetailResponse, GetOperationDetailError>;

    /// <p>This operation returns all the domain names registered with Amazon Route 53 for the current AWS account.</p>
    fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> RusotoFuture<ListDomainsResponse, ListDomainsError>;

    /// <p>This operation returns the operation IDs of operations that are not yet complete.</p>
    fn list_operations(
        &self,
        input: ListOperationsRequest,
    ) -> RusotoFuture<ListOperationsResponse, ListOperationsError>;

    /// <p>This operation returns all of the tags that are associated with the specified domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    fn list_tags_for_domain(
        &self,
        input: ListTagsForDomainRequest,
    ) -> RusotoFuture<ListTagsForDomainResponse, ListTagsForDomainError>;

    /// <p><p>This operation registers a domain. Domains are registered by the AWS registrar partner, Gandi. For some top-level domains (TLDs), this operation requires extra parameters.</p> <p>When you register a domain, Amazon Route 53 does the following:</p> <ul> <li> <p>Creates a Amazon Route 53 hosted zone that has the same name as the domain. Amazon Route 53 assigns four name servers to your hosted zone and automatically updates your domain registration with the names of these name servers.</p> </li> <li> <p>Enables autorenew, so your domain registration will renew automatically each year. We&#39;ll notify you in advance of the renewal date so you can choose whether to renew the registration.</p> </li> <li> <p>Optionally enables privacy protection, so WHOIS queries return contact information for our registrar partner, Gandi, instead of the information you entered for registrant, admin, and tech contacts.</p> </li> <li> <p>If registration is successful, returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant is notified by email.</p> </li> <li> <p>Charges your AWS account an amount based on the top-level domain. For more information, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p> </li> </ul></p>
    fn register_domain(
        &self,
        input: RegisterDomainRequest,
    ) -> RusotoFuture<RegisterDomainResponse, RegisterDomainError>;

    /// <p>This operation renews a domain for the specified number of years. The cost of renewing your domain is billed to your AWS account.</p> <p>We recommend that you renew your domain several weeks before the expiration date. Some TLD registries delete domains before the expiration date if you haven't renewed far enough in advance. For more information about renewing domain registration, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/domain-renew.html">Renewing Registration for a Domain</a> in the Amazon Route 53 Developer Guide.</p>
    fn renew_domain(
        &self,
        input: RenewDomainRequest,
    ) -> RusotoFuture<RenewDomainResponse, RenewDomainError>;

    /// <p>For operations that require confirmation that the email address for the registrant contact is valid, such as registering a new domain, this operation resends the confirmation email to the current email address for the registrant contact.</p>
    fn resend_contact_reachability_email(
        &self,
        input: ResendContactReachabilityEmailRequest,
    ) -> RusotoFuture<ResendContactReachabilityEmailResponse, ResendContactReachabilityEmailError>;

    /// <p>This operation returns the AuthCode for the domain. To transfer a domain to another registrar, you provide this value to the new registrar.</p>
    fn retrieve_domain_auth_code(
        &self,
        input: RetrieveDomainAuthCodeRequest,
    ) -> RusotoFuture<RetrieveDomainAuthCodeResponse, RetrieveDomainAuthCodeError>;

    /// <p>This operation transfers a domain from another registrar to Amazon Route 53. When the transfer is complete, the domain is registered with the AWS registrar partner, Gandi.</p> <p>For transfer requirements, a detailed procedure, and information about viewing the status of a domain transfer, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/domain-transfer-to-route-53.html">Transferring Registration for a Domain to Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If the registrar for your domain is also the DNS service provider for the domain, we highly recommend that you consider transferring your DNS service to Amazon Route 53 or to another DNS service provider before you transfer your registration. Some registrars provide free DNS service when you purchase a domain registration. When you transfer the registration, the previous registrar will not renew your domain registration and could end your DNS service at any time.</p> <important> <p>If the registrar for your domain is also the DNS service provider for the domain and you don't transfer DNS service to another provider, your website, email, and the web applications associated with the domain might become unavailable.</p> </important> <p>If the transfer is successful, this method returns an operation ID that you can use to track the progress and completion of the action. If the transfer doesn't complete successfully, the domain registrant will be notified by email.</p>
    fn transfer_domain(
        &self,
        input: TransferDomainRequest,
    ) -> RusotoFuture<TransferDomainResponse, TransferDomainError>;

    /// <p>This operation updates the contact information for a particular domain. Information for at least one contact (registrant, administrator, or technical) must be supplied for update.</p> <p>If the update is successful, this method returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    fn update_domain_contact(
        &self,
        input: UpdateDomainContactRequest,
    ) -> RusotoFuture<UpdateDomainContactResponse, UpdateDomainContactError>;

    /// <p>This operation updates the specified domain contact's privacy setting. When the privacy option is enabled, personal information such as postal or email address is hidden from the results of a public WHOIS query. The privacy services are provided by the AWS registrar, Gandi. For more information, see the <a href="http://www.gandi.net/domain/whois/?currency=USD&amp;amp;lang=en">Gandi privacy features</a>.</p> <p>This operation only affects the privacy of the specified contact type (registrant, administrator, or tech). Successful acceptance returns an operation ID that you can use with <a>GetOperationDetail</a> to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    fn update_domain_contact_privacy(
        &self,
        input: UpdateDomainContactPrivacyRequest,
    ) -> RusotoFuture<UpdateDomainContactPrivacyResponse, UpdateDomainContactPrivacyError>;

    /// <p>This operation replaces the current set of name servers for the domain with the specified set of name servers. If you use Amazon Route 53 as your DNS service, specify the four name servers in the delegation set for the hosted zone for the domain.</p> <p>If successful, this operation returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    fn update_domain_nameservers(
        &self,
        input: UpdateDomainNameserversRequest,
    ) -> RusotoFuture<UpdateDomainNameserversResponse, UpdateDomainNameserversError>;

    /// <p>This operation adds or updates tags for a specified domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    fn update_tags_for_domain(
        &self,
        input: UpdateTagsForDomainRequest,
    ) -> RusotoFuture<UpdateTagsForDomainResponse, UpdateTagsForDomainError>;

    /// <p>Returns all the domain-related billing records for the current AWS account for a specified period</p>
    fn view_billing(
        &self,
        input: ViewBillingRequest,
    ) -> RusotoFuture<ViewBillingResponse, ViewBillingError>;
}
/// A client for the Amazon Route 53 Domains API.
pub struct Route53DomainsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl Route53DomainsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> Route53DomainsClient {
        Route53DomainsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> Route53DomainsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        Route53DomainsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Route53Domains for Route53DomainsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>This operation checks the availability of one domain name. Note that if the availability status of a domain is pending, you must submit another request to determine the availability of the domain name.</p>
    fn check_domain_availability(
        &self,
        input: CheckDomainAvailabilityRequest,
    ) -> RusotoFuture<CheckDomainAvailabilityResponse, CheckDomainAvailabilityError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.CheckDomainAvailability",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CheckDomainAvailabilityResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CheckDomainAvailabilityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Checks whether a domain name can be transferred to Amazon Route 53. </p>
    fn check_domain_transferability(
        &self,
        input: CheckDomainTransferabilityRequest,
    ) -> RusotoFuture<CheckDomainTransferabilityResponse, CheckDomainTransferabilityError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.CheckDomainTransferability",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CheckDomainTransferabilityResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CheckDomainTransferabilityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation deletes the specified tags for a domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    fn delete_tags_for_domain(
        &self,
        input: DeleteTagsForDomainRequest,
    ) -> RusotoFuture<DeleteTagsForDomainResponse, DeleteTagsForDomainError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.DeleteTagsForDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteTagsForDomainResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTagsForDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation disables automatic renewal of domain registration for the specified domain.</p>
    fn disable_domain_auto_renew(
        &self,
        input: DisableDomainAutoRenewRequest,
    ) -> RusotoFuture<DisableDomainAutoRenewResponse, DisableDomainAutoRenewError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.DisableDomainAutoRenew",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisableDomainAutoRenewResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableDomainAutoRenewError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation removes the transfer lock on the domain (specifically the <code>clientTransferProhibited</code> status) to allow domain transfers. We recommend you refrain from performing this action unless you intend to transfer the domain to a different registrar. Successful submission returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    fn disable_domain_transfer_lock(
        &self,
        input: DisableDomainTransferLockRequest,
    ) -> RusotoFuture<DisableDomainTransferLockResponse, DisableDomainTransferLockError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.DisableDomainTransferLock",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisableDomainTransferLockResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableDomainTransferLockError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation configures Amazon Route 53 to automatically renew the specified domain before the domain registration expires. The cost of renewing your domain registration is billed to your AWS account.</p> <p>The period during which you can renew a domain name varies by TLD. For a list of TLDs and their renewal policies, see <a href="http://wiki.gandi.net/en/domains/renew#renewal_restoration_and_deletion_times">"Renewal, restoration, and deletion times"</a> on the website for our registrar partner, Gandi. Route 53 requires that you renew before the end of the renewal period that is listed on the Gandi website so we can complete processing before the deadline.</p>
    fn enable_domain_auto_renew(
        &self,
        input: EnableDomainAutoRenewRequest,
    ) -> RusotoFuture<EnableDomainAutoRenewResponse, EnableDomainAutoRenewError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.EnableDomainAutoRenew",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<EnableDomainAutoRenewResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableDomainAutoRenewError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation sets the transfer lock on the domain (specifically the <code>clientTransferProhibited</code> status) to prevent domain transfers. Successful submission returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    fn enable_domain_transfer_lock(
        &self,
        input: EnableDomainTransferLockRequest,
    ) -> RusotoFuture<EnableDomainTransferLockResponse, EnableDomainTransferLockError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.EnableDomainTransferLock",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<EnableDomainTransferLockResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableDomainTransferLockError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>For operations that require confirmation that the email address for the registrant contact is valid, such as registering a new domain, this operation returns information about whether the registrant contact has responded.</p> <p>If you want us to resend the email, use the <code>ResendContactReachabilityEmail</code> operation.</p>
    fn get_contact_reachability_status(
        &self,
        input: GetContactReachabilityStatusRequest,
    ) -> RusotoFuture<GetContactReachabilityStatusResponse, GetContactReachabilityStatusError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.GetContactReachabilityStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetContactReachabilityStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetContactReachabilityStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation returns detailed information about a specified domain that is associated with the current AWS account. Contact information for the domain is also returned as part of the output.</p>
    fn get_domain_detail(
        &self,
        input: GetDomainDetailRequest,
    ) -> RusotoFuture<GetDomainDetailResponse, GetDomainDetailError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.GetDomainDetail");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDomainDetailResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDomainDetailError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The GetDomainSuggestions operation returns a list of suggested domain names given a string, which can either be a domain name or simply a word or phrase (without spaces).</p>
    fn get_domain_suggestions(
        &self,
        input: GetDomainSuggestionsRequest,
    ) -> RusotoFuture<GetDomainSuggestionsResponse, GetDomainSuggestionsError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.GetDomainSuggestions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDomainSuggestionsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDomainSuggestionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation returns the current status of an operation that is not completed.</p>
    fn get_operation_detail(
        &self,
        input: GetOperationDetailRequest,
    ) -> RusotoFuture<GetOperationDetailResponse, GetOperationDetailError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.GetOperationDetail",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetOperationDetailResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetOperationDetailError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation returns all the domain names registered with Amazon Route 53 for the current AWS account.</p>
    fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> RusotoFuture<ListDomainsResponse, ListDomainsError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.ListDomains");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDomainsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListDomainsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation returns the operation IDs of operations that are not yet complete.</p>
    fn list_operations(
        &self,
        input: ListOperationsRequest,
    ) -> RusotoFuture<ListOperationsResponse, ListOperationsError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.ListOperations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListOperationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListOperationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation returns all of the tags that are associated with the specified domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    fn list_tags_for_domain(
        &self,
        input: ListTagsForDomainRequest,
    ) -> RusotoFuture<ListTagsForDomainResponse, ListTagsForDomainError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.ListTagsForDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsForDomainResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>This operation registers a domain. Domains are registered by the AWS registrar partner, Gandi. For some top-level domains (TLDs), this operation requires extra parameters.</p> <p>When you register a domain, Amazon Route 53 does the following:</p> <ul> <li> <p>Creates a Amazon Route 53 hosted zone that has the same name as the domain. Amazon Route 53 assigns four name servers to your hosted zone and automatically updates your domain registration with the names of these name servers.</p> </li> <li> <p>Enables autorenew, so your domain registration will renew automatically each year. We&#39;ll notify you in advance of the renewal date so you can choose whether to renew the registration.</p> </li> <li> <p>Optionally enables privacy protection, so WHOIS queries return contact information for our registrar partner, Gandi, instead of the information you entered for registrant, admin, and tech contacts.</p> </li> <li> <p>If registration is successful, returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant is notified by email.</p> </li> <li> <p>Charges your AWS account an amount based on the top-level domain. For more information, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p> </li> </ul></p>
    fn register_domain(
        &self,
        input: RegisterDomainRequest,
    ) -> RusotoFuture<RegisterDomainResponse, RegisterDomainError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.RegisterDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RegisterDomainResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation renews a domain for the specified number of years. The cost of renewing your domain is billed to your AWS account.</p> <p>We recommend that you renew your domain several weeks before the expiration date. Some TLD registries delete domains before the expiration date if you haven't renewed far enough in advance. For more information about renewing domain registration, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/domain-renew.html">Renewing Registration for a Domain</a> in the Amazon Route 53 Developer Guide.</p>
    fn renew_domain(
        &self,
        input: RenewDomainRequest,
    ) -> RusotoFuture<RenewDomainResponse, RenewDomainError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.RenewDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RenewDomainResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RenewDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>For operations that require confirmation that the email address for the registrant contact is valid, such as registering a new domain, this operation resends the confirmation email to the current email address for the registrant contact.</p>
    fn resend_contact_reachability_email(
        &self,
        input: ResendContactReachabilityEmailRequest,
    ) -> RusotoFuture<ResendContactReachabilityEmailResponse, ResendContactReachabilityEmailError>
    {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.ResendContactReachabilityEmail",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ResendContactReachabilityEmailResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ResendContactReachabilityEmailError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation returns the AuthCode for the domain. To transfer a domain to another registrar, you provide this value to the new registrar.</p>
    fn retrieve_domain_auth_code(
        &self,
        input: RetrieveDomainAuthCodeRequest,
    ) -> RusotoFuture<RetrieveDomainAuthCodeResponse, RetrieveDomainAuthCodeError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.RetrieveDomainAuthCode",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RetrieveDomainAuthCodeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RetrieveDomainAuthCodeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation transfers a domain from another registrar to Amazon Route 53. When the transfer is complete, the domain is registered with the AWS registrar partner, Gandi.</p> <p>For transfer requirements, a detailed procedure, and information about viewing the status of a domain transfer, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/domain-transfer-to-route-53.html">Transferring Registration for a Domain to Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If the registrar for your domain is also the DNS service provider for the domain, we highly recommend that you consider transferring your DNS service to Amazon Route 53 or to another DNS service provider before you transfer your registration. Some registrars provide free DNS service when you purchase a domain registration. When you transfer the registration, the previous registrar will not renew your domain registration and could end your DNS service at any time.</p> <important> <p>If the registrar for your domain is also the DNS service provider for the domain and you don't transfer DNS service to another provider, your website, email, and the web applications associated with the domain might become unavailable.</p> </important> <p>If the transfer is successful, this method returns an operation ID that you can use to track the progress and completion of the action. If the transfer doesn't complete successfully, the domain registrant will be notified by email.</p>
    fn transfer_domain(
        &self,
        input: TransferDomainRequest,
    ) -> RusotoFuture<TransferDomainResponse, TransferDomainError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.TransferDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TransferDomainResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TransferDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation updates the contact information for a particular domain. Information for at least one contact (registrant, administrator, or technical) must be supplied for update.</p> <p>If the update is successful, this method returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    fn update_domain_contact(
        &self,
        input: UpdateDomainContactRequest,
    ) -> RusotoFuture<UpdateDomainContactResponse, UpdateDomainContactError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.UpdateDomainContact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDomainContactResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDomainContactError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation updates the specified domain contact's privacy setting. When the privacy option is enabled, personal information such as postal or email address is hidden from the results of a public WHOIS query. The privacy services are provided by the AWS registrar, Gandi. For more information, see the <a href="http://www.gandi.net/domain/whois/?currency=USD&amp;amp;lang=en">Gandi privacy features</a>.</p> <p>This operation only affects the privacy of the specified contact type (registrant, administrator, or tech). Successful acceptance returns an operation ID that you can use with <a>GetOperationDetail</a> to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    fn update_domain_contact_privacy(
        &self,
        input: UpdateDomainContactPrivacyRequest,
    ) -> RusotoFuture<UpdateDomainContactPrivacyResponse, UpdateDomainContactPrivacyError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.UpdateDomainContactPrivacy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDomainContactPrivacyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDomainContactPrivacyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation replaces the current set of name servers for the domain with the specified set of name servers. If you use Amazon Route 53 as your DNS service, specify the four name servers in the delegation set for the hosted zone for the domain.</p> <p>If successful, this operation returns an operation ID that you can use to track the progress and completion of the action. If the request is not completed successfully, the domain registrant will be notified by email.</p>
    fn update_domain_nameservers(
        &self,
        input: UpdateDomainNameserversRequest,
    ) -> RusotoFuture<UpdateDomainNameserversResponse, UpdateDomainNameserversError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.UpdateDomainNameservers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDomainNameserversResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDomainNameserversError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation adds or updates tags for a specified domain.</p> <p>All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations.</p>
    fn update_tags_for_domain(
        &self,
        input: UpdateTagsForDomainRequest,
    ) -> RusotoFuture<UpdateTagsForDomainResponse, UpdateTagsForDomainError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Route53Domains_v20140515.UpdateTagsForDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateTagsForDomainResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateTagsForDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns all the domain-related billing records for the current AWS account for a specified period</p>
    fn view_billing(
        &self,
        input: ViewBillingRequest,
    ) -> RusotoFuture<ViewBillingResponse, ViewBillingError> {
        let mut request = SignedRequest::new("POST", "route53domains", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Route53Domains_v20140515.ViewBilling");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ViewBillingResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ViewBillingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
