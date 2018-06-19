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
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
struct APIVersionDeserializer;
impl APIVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ARNDeserializer;
impl ARNDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The configured access rules for the domain's document and search endpoints, and the current status of those rules.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccessPoliciesStatus {
    pub options: String,
    pub status: OptionStatus,
}

struct AccessPoliciesStatusDeserializer;
impl AccessPoliciesStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccessPoliciesStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AccessPoliciesStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Options" => {
                        obj.options =
                            try!(PolicyDocumentDeserializer::deserialize("Options", stack));
                    }
                    "Status" => {
                        obj.status = try!(OptionStatusDeserializer::deserialize("Status", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AlgorithmicStemmingDeserializer;
impl AlgorithmicStemmingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Synonyms, stopwords, and stemming options for an analysis scheme. Includes tokenization dictionary for Japanese.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalysisOptions {
    /// <p>The level of algorithmic stemming to perform: <code>none</code>, <code>minimal</code>, <code>light</code>, or <code>full</code>. The available levels vary depending on the language. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/text-processing.html#text-processing-settings" target="_blank">Language Specific Text Processing Settings</a> in the <i>Amazon CloudSearch Developer Guide</i> </p>
    pub algorithmic_stemming: Option<String>,
    /// <p>A JSON array that contains a collection of terms, tokens, readings and part of speech for Japanese Tokenizaiton. The Japanese tokenization dictionary enables you to override the default tokenization for selected terms. This is only valid for Japanese language fields.</p>
    pub japanese_tokenization_dictionary: Option<String>,
    /// <p>A JSON object that contains a collection of string:value pairs that each map a term to its stem. For example, <code>{"term1": "stem1", "term2": "stem2", "term3": "stem3"}</code>. The stemming dictionary is applied in addition to any algorithmic stemming. This enables you to override the results of the algorithmic stemming to correct specific cases of overstemming or understemming. The maximum size of a stemming dictionary is 500 KB.</p>
    pub stemming_dictionary: Option<String>,
    /// <p>A JSON array of terms to ignore during indexing and searching. For example, <code>["a", "an", "the", "of"]</code>. The stopwords dictionary must explicitly list each word you want to ignore. Wildcards and regular expressions are not supported. </p>
    pub stopwords: Option<String>,
    /// <p>A JSON object that defines synonym groups and aliases. A synonym group is an array of arrays, where each sub-array is a group of terms where each term in the group is considered a synonym of every other term in the group. The aliases value is an object that contains a collection of string:value pairs where the string specifies a term and the array of values specifies each of the aliases for that term. An alias is considered a synonym of the specified term, but the term is not considered a synonym of the alias. For more information about specifying synonyms, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html#synonyms">Synonyms</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    pub synonyms: Option<String>,
}

struct AnalysisOptionsDeserializer;
impl AnalysisOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalysisOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalysisOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AlgorithmicStemming" => {
                        obj.algorithmic_stemming =
                            Some(try!(AlgorithmicStemmingDeserializer::deserialize(
                                "AlgorithmicStemming",
                                stack
                            )));
                    }
                    "JapaneseTokenizationDictionary" => {
                        obj.japanese_tokenization_dictionary =
                            Some(try!(StringDeserializer::deserialize(
                                "JapaneseTokenizationDictionary",
                                stack
                            )));
                    }
                    "StemmingDictionary" => {
                        obj.stemming_dictionary = Some(try!(StringDeserializer::deserialize(
                            "StemmingDictionary",
                            stack
                        )));
                    }
                    "Stopwords" => {
                        obj.stopwords =
                            Some(try!(StringDeserializer::deserialize("Stopwords", stack)));
                    }
                    "Synonyms" => {
                        obj.synonyms =
                            Some(try!(StringDeserializer::deserialize("Synonyms", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `AnalysisOptions` contents to a `SignedRequest`.
struct AnalysisOptionsSerializer;
impl AnalysisOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AnalysisOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.algorithmic_stemming {
            params.put(
                &format!("{}{}", prefix, "AlgorithmicStemming"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.japanese_tokenization_dictionary {
            params.put(
                &format!("{}{}", prefix, "JapaneseTokenizationDictionary"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stemming_dictionary {
            params.put(
                &format!("{}{}", prefix, "StemmingDictionary"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stopwords {
            params.put(
                &format!("{}{}", prefix, "Stopwords"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.synonyms {
            params.put(
                &format!("{}{}", prefix, "Synonyms"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Configuration information for an analysis scheme. Each analysis scheme has a unique name and specifies the language of the text to be processed. The following options can be configured for an analysis scheme: <code>Synonyms</code>, <code>Stopwords</code>, <code>StemmingDictionary</code>, <code>JapaneseTokenizationDictionary</code> and <code>AlgorithmicStemming</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalysisScheme {
    pub analysis_options: Option<AnalysisOptions>,
    pub analysis_scheme_language: String,
    pub analysis_scheme_name: String,
}

struct AnalysisSchemeDeserializer;
impl AnalysisSchemeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalysisScheme, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalysisScheme::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AnalysisOptions" => {
                        obj.analysis_options = Some(try!(
                            AnalysisOptionsDeserializer::deserialize("AnalysisOptions", stack)
                        ));
                    }
                    "AnalysisSchemeLanguage" => {
                        obj.analysis_scheme_language =
                            try!(AnalysisSchemeLanguageDeserializer::deserialize(
                                "AnalysisSchemeLanguage",
                                stack
                            ));
                    }
                    "AnalysisSchemeName" => {
                        obj.analysis_scheme_name = try!(StandardNameDeserializer::deserialize(
                            "AnalysisSchemeName",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `AnalysisScheme` contents to a `SignedRequest`.
struct AnalysisSchemeSerializer;
impl AnalysisSchemeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AnalysisScheme) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.analysis_options {
            AnalysisOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AnalysisOptions"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "AnalysisSchemeLanguage"),
            &obj.analysis_scheme_language.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "AnalysisSchemeName"),
            &obj.analysis_scheme_name.replace("+", "%2B"),
        );
    }
}

struct AnalysisSchemeLanguageDeserializer;
impl AnalysisSchemeLanguageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The status and configuration of an <code>AnalysisScheme</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalysisSchemeStatus {
    pub options: AnalysisScheme,
    pub status: OptionStatus,
}

struct AnalysisSchemeStatusDeserializer;
impl AnalysisSchemeStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalysisSchemeStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalysisSchemeStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Options" => {
                        obj.options =
                            try!(AnalysisSchemeDeserializer::deserialize("Options", stack));
                    }
                    "Status" => {
                        obj.status = try!(OptionStatusDeserializer::deserialize("Status", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AnalysisSchemeStatusListDeserializer;
impl AnalysisSchemeStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AnalysisSchemeStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(AnalysisSchemeStatusDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>The status and configuration of the domain's availability options.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AvailabilityOptionsStatus {
    /// <p>The availability options configured for the domain.</p>
    pub options: bool,
    pub status: OptionStatus,
}

struct AvailabilityOptionsStatusDeserializer;
impl AvailabilityOptionsStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AvailabilityOptionsStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AvailabilityOptionsStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Options" => {
                        obj.options = try!(MultiAZDeserializer::deserialize("Options", stack));
                    }
                    "Status" => {
                        obj.status = try!(OptionStatusDeserializer::deserialize("Status", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>BuildSuggester</a></code> operation. Specifies the name of the domain you want to update.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BuildSuggestersRequest {
    pub domain_name: String,
}

/// Serialize `BuildSuggestersRequest` contents to a `SignedRequest`.
struct BuildSuggestersRequestSerializer;
impl BuildSuggestersRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BuildSuggestersRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code>BuildSuggester</code> request. Contains a list of the fields used for suggestions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BuildSuggestersResponse {
    pub field_names: Option<Vec<String>>,
}

struct BuildSuggestersResponseDeserializer;
impl BuildSuggestersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BuildSuggestersResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = BuildSuggestersResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "FieldNames" => {
                        obj.field_names = Some(try!(FieldNameListDeserializer::deserialize(
                            "FieldNames",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>CreateDomain</a></code> operation. Specifies a name for the new search domain.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDomainRequest {
    /// <p>A name for the domain you are creating. Allowed characters are a-z (lower-case letters), 0-9, and hyphen (-). Domain names must start with a letter or number and be at least 3 and no more than 28 characters long.</p>
    pub domain_name: String,
}

/// Serialize `CreateDomainRequest` contents to a `SignedRequest`.
struct CreateDomainRequestSerializer;
impl CreateDomainRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDomainRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code>CreateDomainRequest</code>. Contains the status of a newly created domain.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDomainResponse {
    pub domain_status: Option<DomainStatus>,
}

struct CreateDomainResponseDeserializer;
impl CreateDomainResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDomainResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateDomainResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DomainStatus" => {
                        obj.domain_status = Some(try!(DomainStatusDeserializer::deserialize(
                            "DomainStatus",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Options for a field that contains an array of dates. Present if <code>IndexFieldType</code> specifies the field is of type <code>date-array</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DateArrayOptions {
    /// <p>A value to use for the field if the field isn&#39;t specified for a document.</p>
    pub default_value: Option<String>,
    /// <p>Whether facet information can be returned for the field.</p>
    pub facet_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>Whether the contents of the field are searchable.</p>
    pub search_enabled: Option<bool>,
    /// <p>A list of source fields to map to the field. </p>
    pub source_fields: Option<String>,
}

struct DateArrayOptionsDeserializer;
impl DateArrayOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DateArrayOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DateArrayOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value = Some(try!(FieldValueDeserializer::deserialize(
                            "DefaultValue",
                            stack
                        )));
                    }
                    "FacetEnabled" => {
                        obj.facet_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "FacetEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SearchEnabled" => {
                        obj.search_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "SearchEnabled",
                            stack
                        )));
                    }
                    "SourceFields" => {
                        obj.source_fields = Some(try!(
                            FieldNameCommaListDeserializer::deserialize("SourceFields", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `DateArrayOptions` contents to a `SignedRequest`.
struct DateArrayOptionsSerializer;
impl DateArrayOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DateArrayOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(
                &format!("{}{}", prefix, "FacetEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(
                &format!("{}{}", prefix, "SearchEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_fields {
            params.put(
                &format!("{}{}", prefix, "SourceFields"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Options for a date field. Dates and times are specified in UTC (Coordinated Universal Time) according to IETF RFC3339: yyyy-mm-ddT00:00:00Z. Present if <code>IndexFieldType</code> specifies the field is of type <code>date</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DateOptions {
    /// <p>A value to use for the field if the field isn&#39;t specified for a document.</p>
    pub default_value: Option<String>,
    /// <p>Whether facet information can be returned for the field.</p>
    pub facet_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>Whether the contents of the field are searchable.</p>
    pub search_enabled: Option<bool>,
    /// <p>Whether the field can be used to sort the search results.</p>
    pub sort_enabled: Option<bool>,
    pub source_field: Option<String>,
}

struct DateOptionsDeserializer;
impl DateOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DateOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DateOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value = Some(try!(FieldValueDeserializer::deserialize(
                            "DefaultValue",
                            stack
                        )));
                    }
                    "FacetEnabled" => {
                        obj.facet_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "FacetEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SearchEnabled" => {
                        obj.search_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "SearchEnabled",
                            stack
                        )));
                    }
                    "SortEnabled" => {
                        obj.sort_enabled =
                            Some(try!(BooleanDeserializer::deserialize("SortEnabled", stack)));
                    }
                    "SourceField" => {
                        obj.source_field = Some(try!(FieldNameDeserializer::deserialize(
                            "SourceField",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `DateOptions` contents to a `SignedRequest`.
struct DateOptionsSerializer;
impl DateOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DateOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(
                &format!("{}{}", prefix, "FacetEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(
                &format!("{}{}", prefix, "SearchEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(
                &format!("{}{}", prefix, "SortEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(
                &format!("{}{}", prefix, "SourceField"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Container for the parameters to the <code><a>DefineAnalysisScheme</a></code> operation. Specifies the name of the domain you want to update and the analysis scheme configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefineAnalysisSchemeRequest {
    pub analysis_scheme: AnalysisScheme,
    pub domain_name: String,
}

/// Serialize `DefineAnalysisSchemeRequest` contents to a `SignedRequest`.
struct DefineAnalysisSchemeRequestSerializer;
impl DefineAnalysisSchemeRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DefineAnalysisSchemeRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AnalysisSchemeSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AnalysisScheme"),
            &obj.analysis_scheme,
        );
        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code><a>DefineAnalysisScheme</a></code> request. Contains the status of the newly-configured analysis scheme.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefineAnalysisSchemeResponse {
    pub analysis_scheme: AnalysisSchemeStatus,
}

struct DefineAnalysisSchemeResponseDeserializer;
impl DefineAnalysisSchemeResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefineAnalysisSchemeResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DefineAnalysisSchemeResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AnalysisScheme" => {
                        obj.analysis_scheme = try!(AnalysisSchemeStatusDeserializer::deserialize(
                            "AnalysisScheme",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DefineExpression</a></code> operation. Specifies the name of the domain you want to update and the expression you want to configure.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefineExpressionRequest {
    pub domain_name: String,
    pub expression: Expression,
}

/// Serialize `DefineExpressionRequest` contents to a `SignedRequest`.
struct DefineExpressionRequestSerializer;
impl DefineExpressionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DefineExpressionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        ExpressionSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Expression"),
            &obj.expression,
        );
    }
}

/// <p>The result of a <code>DefineExpression</code> request. Contains the status of the newly-configured expression.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefineExpressionResponse {
    pub expression: ExpressionStatus,
}

struct DefineExpressionResponseDeserializer;
impl DefineExpressionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefineExpressionResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DefineExpressionResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Expression" => {
                        obj.expression = try!(ExpressionStatusDeserializer::deserialize(
                            "Expression",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DefineIndexField</a></code> operation. Specifies the name of the domain you want to update and the index field configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefineIndexFieldRequest {
    pub domain_name: String,
    /// <p>The index field and field options you want to configure. </p>
    pub index_field: IndexField,
}

/// Serialize `DefineIndexFieldRequest` contents to a `SignedRequest`.
struct DefineIndexFieldRequestSerializer;
impl DefineIndexFieldRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DefineIndexFieldRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        IndexFieldSerializer::serialize(
            params,
            &format!("{}{}", prefix, "IndexField"),
            &obj.index_field,
        );
    }
}

/// <p>The result of a <code><a>DefineIndexField</a></code> request. Contains the status of the newly-configured index field.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefineIndexFieldResponse {
    pub index_field: IndexFieldStatus,
}

struct DefineIndexFieldResponseDeserializer;
impl DefineIndexFieldResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefineIndexFieldResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DefineIndexFieldResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IndexField" => {
                        obj.index_field = try!(IndexFieldStatusDeserializer::deserialize(
                            "IndexField",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DefineSuggester</a></code> operation. Specifies the name of the domain you want to update and the suggester configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefineSuggesterRequest {
    pub domain_name: String,
    pub suggester: Suggester,
}

/// Serialize `DefineSuggesterRequest` contents to a `SignedRequest`.
struct DefineSuggesterRequestSerializer;
impl DefineSuggesterRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DefineSuggesterRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        SuggesterSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Suggester"),
            &obj.suggester,
        );
    }
}

/// <p>The result of a <code>DefineSuggester</code> request. Contains the status of the newly-configured suggester.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefineSuggesterResponse {
    pub suggester: SuggesterStatus,
}

struct DefineSuggesterResponseDeserializer;
impl DefineSuggesterResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefineSuggesterResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DefineSuggesterResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Suggester" => {
                        obj.suggester =
                            try!(SuggesterStatusDeserializer::deserialize("Suggester", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DeleteAnalysisScheme</a></code> operation. Specifies the name of the domain you want to update and the analysis scheme you want to delete. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteAnalysisSchemeRequest {
    /// <p>The name of the analysis scheme you want to delete.</p>
    pub analysis_scheme_name: String,
    pub domain_name: String,
}

/// Serialize `DeleteAnalysisSchemeRequest` contents to a `SignedRequest`.
struct DeleteAnalysisSchemeRequestSerializer;
impl DeleteAnalysisSchemeRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteAnalysisSchemeRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AnalysisSchemeName"),
            &obj.analysis_scheme_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code>DeleteAnalysisScheme</code> request. Contains the status of the deleted analysis scheme.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteAnalysisSchemeResponse {
    /// <p>The status of the analysis scheme being deleted.</p>
    pub analysis_scheme: AnalysisSchemeStatus,
}

struct DeleteAnalysisSchemeResponseDeserializer;
impl DeleteAnalysisSchemeResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteAnalysisSchemeResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteAnalysisSchemeResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AnalysisScheme" => {
                        obj.analysis_scheme = try!(AnalysisSchemeStatusDeserializer::deserialize(
                            "AnalysisScheme",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DeleteDomain</a></code> operation. Specifies the name of the domain you want to delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDomainRequest {
    /// <p>The name of the domain you want to permanently delete.</p>
    pub domain_name: String,
}

/// Serialize `DeleteDomainRequest` contents to a `SignedRequest`.
struct DeleteDomainRequestSerializer;
impl DeleteDomainRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDomainRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code>DeleteDomain</code> request. Contains the status of a newly deleted domain, or no status if the domain has already been completely deleted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDomainResponse {
    pub domain_status: Option<DomainStatus>,
}

struct DeleteDomainResponseDeserializer;
impl DeleteDomainResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteDomainResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteDomainResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DomainStatus" => {
                        obj.domain_status = Some(try!(DomainStatusDeserializer::deserialize(
                            "DomainStatus",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DeleteExpression</a></code> operation. Specifies the name of the domain you want to update and the name of the expression you want to delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteExpressionRequest {
    pub domain_name: String,
    /// <p>The name of the <code><a>Expression</a></code> to delete.</p>
    pub expression_name: String,
}

/// Serialize `DeleteExpressionRequest` contents to a `SignedRequest`.
struct DeleteExpressionRequestSerializer;
impl DeleteExpressionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteExpressionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ExpressionName"),
            &obj.expression_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code><a>DeleteExpression</a></code> request. Specifies the expression being deleted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteExpressionResponse {
    /// <p>The status of the expression being deleted.</p>
    pub expression: ExpressionStatus,
}

struct DeleteExpressionResponseDeserializer;
impl DeleteExpressionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteExpressionResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteExpressionResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Expression" => {
                        obj.expression = try!(ExpressionStatusDeserializer::deserialize(
                            "Expression",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DeleteIndexField</a></code> operation. Specifies the name of the domain you want to update and the name of the index field you want to delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteIndexFieldRequest {
    pub domain_name: String,
    /// <p>The name of the index field your want to remove from the domain's indexing options.</p>
    pub index_field_name: String,
}

/// Serialize `DeleteIndexFieldRequest` contents to a `SignedRequest`.
struct DeleteIndexFieldRequestSerializer;
impl DeleteIndexFieldRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteIndexFieldRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "IndexFieldName"),
            &obj.index_field_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code><a>DeleteIndexField</a></code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteIndexFieldResponse {
    /// <p>The status of the index field being deleted.</p>
    pub index_field: IndexFieldStatus,
}

struct DeleteIndexFieldResponseDeserializer;
impl DeleteIndexFieldResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteIndexFieldResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteIndexFieldResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IndexField" => {
                        obj.index_field = try!(IndexFieldStatusDeserializer::deserialize(
                            "IndexField",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DeleteSuggester</a></code> operation. Specifies the name of the domain you want to update and name of the suggester you want to delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteSuggesterRequest {
    pub domain_name: String,
    /// <p>Specifies the name of the suggester you want to delete.</p>
    pub suggester_name: String,
}

/// Serialize `DeleteSuggesterRequest` contents to a `SignedRequest`.
struct DeleteSuggesterRequestSerializer;
impl DeleteSuggesterRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteSuggesterRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "SuggesterName"),
            &obj.suggester_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code>DeleteSuggester</code> request. Contains the status of the deleted suggester.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteSuggesterResponse {
    /// <p>The status of the suggester being deleted.</p>
    pub suggester: SuggesterStatus,
}

struct DeleteSuggesterResponseDeserializer;
impl DeleteSuggesterResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteSuggesterResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteSuggesterResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Suggester" => {
                        obj.suggester =
                            try!(SuggesterStatusDeserializer::deserialize("Suggester", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DescribeAnalysisSchemes</a></code> operation. Specifies the name of the domain you want to describe. To limit the response to particular analysis schemes, specify the names of the analysis schemes you want to describe. To show the active configuration and exclude any pending changes, set the <code>Deployed</code> option to <code>true</code>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAnalysisSchemesRequest {
    /// <p>The analysis schemes you want to describe.</p>
    pub analysis_scheme_names: Option<Vec<String>>,
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub deployed: Option<bool>,
    /// <p>The name of the domain you want to describe.</p>
    pub domain_name: String,
}

/// Serialize `DescribeAnalysisSchemesRequest` contents to a `SignedRequest`.
struct DescribeAnalysisSchemesRequestSerializer;
impl DescribeAnalysisSchemesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAnalysisSchemesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.analysis_scheme_names {
            StandardNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AnalysisSchemeNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.deployed {
            params.put(
                &format!("{}{}", prefix, "Deployed"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code>DescribeAnalysisSchemes</code> request. Contains the analysis schemes configured for the domain specified in the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAnalysisSchemesResponse {
    /// <p>The analysis scheme descriptions.</p>
    pub analysis_schemes: Vec<AnalysisSchemeStatus>,
}

struct DescribeAnalysisSchemesResponseDeserializer;
impl DescribeAnalysisSchemesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAnalysisSchemesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAnalysisSchemesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AnalysisSchemes" => {
                        obj.analysis_schemes =
                            try!(AnalysisSchemeStatusListDeserializer::deserialize(
                                "AnalysisSchemes",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DescribeAvailabilityOptions</a></code> operation. Specifies the name of the domain you want to describe. To show the active configuration and exclude any pending changes, set the Deployed option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAvailabilityOptionsRequest {
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub deployed: Option<bool>,
    /// <p>The name of the domain you want to describe.</p>
    pub domain_name: String,
}

/// Serialize `DescribeAvailabilityOptionsRequest` contents to a `SignedRequest`.
struct DescribeAvailabilityOptionsRequestSerializer;
impl DescribeAvailabilityOptionsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAvailabilityOptionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.deployed {
            params.put(
                &format!("{}{}", prefix, "Deployed"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code>DescribeAvailabilityOptions</code> request. Indicates whether or not the Multi-AZ option is enabled for the domain specified in the request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAvailabilityOptionsResponse {
    /// <p>The availability options configured for the domain. Indicates whether Multi-AZ is enabled for the domain. </p>
    pub availability_options: Option<AvailabilityOptionsStatus>,
}

struct DescribeAvailabilityOptionsResponseDeserializer;
impl DescribeAvailabilityOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAvailabilityOptionsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAvailabilityOptionsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AvailabilityOptions" => {
                        obj.availability_options =
                            Some(try!(AvailabilityOptionsStatusDeserializer::deserialize(
                                "AvailabilityOptions",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DescribeDomains</a></code> operation. By default shows the status of all domains. To restrict the response to particular domains, specify the names of the domains you want to describe.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDomainsRequest {
    /// <p>The names of the domains you want to include in the response.</p>
    pub domain_names: Option<Vec<String>>,
}

/// Serialize `DescribeDomainsRequest` contents to a `SignedRequest`.
struct DescribeDomainsRequestSerializer;
impl DescribeDomainsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDomainsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.domain_names {
            DomainNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DomainNames"),
                field_value,
            );
        }
    }
}

/// <p>The result of a <code>DescribeDomains</code> request. Contains the status of the domains specified in the request or all domains owned by the account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDomainsResponse {
    pub domain_status_list: Vec<DomainStatus>,
}

struct DescribeDomainsResponseDeserializer;
impl DescribeDomainsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeDomainsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeDomainsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DomainStatusList" => {
                        obj.domain_status_list = try!(DomainStatusListDeserializer::deserialize(
                            "DomainStatusList",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DescribeDomains</a></code> operation. Specifies the name of the domain you want to describe. To restrict the response to particular expressions, specify the names of the expressions you want to describe. To show the active configuration and exclude any pending changes, set the <code>Deployed</code> option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeExpressionsRequest {
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub deployed: Option<bool>,
    /// <p>The name of the domain you want to describe.</p>
    pub domain_name: String,
    /// <p>Limits the <code><a>DescribeExpressions</a></code> response to the specified expressions. If not specified, all expressions are shown.</p>
    pub expression_names: Option<Vec<String>>,
}

/// Serialize `DescribeExpressionsRequest` contents to a `SignedRequest`.
struct DescribeExpressionsRequestSerializer;
impl DescribeExpressionsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeExpressionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.deployed {
            params.put(
                &format!("{}{}", prefix, "Deployed"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.expression_names {
            StandardNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ExpressionNames"),
                field_value,
            );
        }
    }
}

/// <p>The result of a <code>DescribeExpressions</code> request. Contains the expressions configured for the domain specified in the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeExpressionsResponse {
    /// <p>The expressions configured for the domain.</p>
    pub expressions: Vec<ExpressionStatus>,
}

struct DescribeExpressionsResponseDeserializer;
impl DescribeExpressionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeExpressionsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeExpressionsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Expressions" => {
                        obj.expressions = try!(ExpressionStatusListDeserializer::deserialize(
                            "Expressions",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DescribeIndexFields</a></code> operation. Specifies the name of the domain you want to describe. To restrict the response to particular index fields, specify the names of the index fields you want to describe. To show the active configuration and exclude any pending changes, set the <code>Deployed</code> option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeIndexFieldsRequest {
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub deployed: Option<bool>,
    /// <p>The name of the domain you want to describe.</p>
    pub domain_name: String,
    /// <p>A list of the index fields you want to describe. If not specified, information is returned for all configured index fields.</p>
    pub field_names: Option<Vec<String>>,
}

/// Serialize `DescribeIndexFieldsRequest` contents to a `SignedRequest`.
struct DescribeIndexFieldsRequestSerializer;
impl DescribeIndexFieldsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeIndexFieldsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.deployed {
            params.put(
                &format!("{}{}", prefix, "Deployed"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.field_names {
            DynamicFieldNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "FieldNames"),
                field_value,
            );
        }
    }
}

/// <p>The result of a <code>DescribeIndexFields</code> request. Contains the index fields configured for the domain specified in the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeIndexFieldsResponse {
    /// <p>The index fields configured for the domain.</p>
    pub index_fields: Vec<IndexFieldStatus>,
}

struct DescribeIndexFieldsResponseDeserializer;
impl DescribeIndexFieldsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeIndexFieldsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeIndexFieldsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IndexFields" => {
                        obj.index_fields = try!(IndexFieldStatusListDeserializer::deserialize(
                            "IndexFields",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DescribeScalingParameters</a></code> operation. Specifies the name of the domain you want to describe. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeScalingParametersRequest {
    pub domain_name: String,
}

/// Serialize `DescribeScalingParametersRequest` contents to a `SignedRequest`.
struct DescribeScalingParametersRequestSerializer;
impl DescribeScalingParametersRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeScalingParametersRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code>DescribeScalingParameters</code> request. Contains the scaling parameters configured for the domain specified in the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeScalingParametersResponse {
    pub scaling_parameters: ScalingParametersStatus,
}

struct DescribeScalingParametersResponseDeserializer;
impl DescribeScalingParametersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeScalingParametersResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeScalingParametersResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ScalingParameters" => {
                        obj.scaling_parameters =
                            try!(ScalingParametersStatusDeserializer::deserialize(
                                "ScalingParameters",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DescribeServiceAccessPolicies</a></code> operation. Specifies the name of the domain you want to describe. To show the active configuration and exclude any pending changes, set the <code>Deployed</code> option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeServiceAccessPoliciesRequest {
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub deployed: Option<bool>,
    /// <p>The name of the domain you want to describe.</p>
    pub domain_name: String,
}

/// Serialize `DescribeServiceAccessPoliciesRequest` contents to a `SignedRequest`.
struct DescribeServiceAccessPoliciesRequestSerializer;
impl DescribeServiceAccessPoliciesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeServiceAccessPoliciesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.deployed {
            params.put(
                &format!("{}{}", prefix, "Deployed"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code>DescribeServiceAccessPolicies</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeServiceAccessPoliciesResponse {
    /// <p>The access rules configured for the domain specified in the request.</p>
    pub access_policies: AccessPoliciesStatus,
}

struct DescribeServiceAccessPoliciesResponseDeserializer;
impl DescribeServiceAccessPoliciesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeServiceAccessPoliciesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeServiceAccessPoliciesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AccessPolicies" => {
                        obj.access_policies = try!(AccessPoliciesStatusDeserializer::deserialize(
                            "AccessPolicies",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>DescribeSuggester</a></code> operation. Specifies the name of the domain you want to describe. To restrict the response to particular suggesters, specify the names of the suggesters you want to describe. To show the active configuration and exclude any pending changes, set the <code>Deployed</code> option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeSuggestersRequest {
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub deployed: Option<bool>,
    /// <p>The name of the domain you want to describe.</p>
    pub domain_name: String,
    /// <p>The suggesters you want to describe.</p>
    pub suggester_names: Option<Vec<String>>,
}

/// Serialize `DescribeSuggestersRequest` contents to a `SignedRequest`.
struct DescribeSuggestersRequestSerializer;
impl DescribeSuggestersRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeSuggestersRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.deployed {
            params.put(
                &format!("{}{}", prefix, "Deployed"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.suggester_names {
            StandardNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SuggesterNames"),
                field_value,
            );
        }
    }
}

/// <p>The result of a <code>DescribeSuggesters</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeSuggestersResponse {
    /// <p>The suggesters configured for the domain specified in the request.</p>
    pub suggesters: Vec<SuggesterStatus>,
}

struct DescribeSuggestersResponseDeserializer;
impl DescribeSuggestersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeSuggestersResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeSuggestersResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Suggesters" => {
                        obj.suggesters = try!(SuggesterStatusListDeserializer::deserialize(
                            "Suggesters",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Options for a search suggester.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DocumentSuggesterOptions {
    /// <p>The level of fuzziness allowed when suggesting matches for a string: <code>none</code>, <code>low</code>, or <code>high</code>. With none, the specified string is treated as an exact prefix. With low, suggestions must differ from the specified string by no more than one character. With high, suggestions can differ by up to two characters. The default is none. </p>
    pub fuzzy_matching: Option<String>,
    /// <p>An expression that computes a score for each suggestion to control how they are sorted. The scores are rounded to the nearest integer, with a floor of 0 and a ceiling of 2^31-1. A document's relevance score is not computed for suggestions, so sort expressions cannot reference the <code>_score</code> value. To sort suggestions using a numeric field or existing expression, simply specify the name of the field or expression. If no expression is configured for the suggester, the suggestions are sorted with the closest matches listed first.</p>
    pub sort_expression: Option<String>,
    /// <p>The name of the index field you want to use for suggestions. </p>
    pub source_field: String,
}

struct DocumentSuggesterOptionsDeserializer;
impl DocumentSuggesterOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DocumentSuggesterOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DocumentSuggesterOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "FuzzyMatching" => {
                        obj.fuzzy_matching = Some(try!(
                            SuggesterFuzzyMatchingDeserializer::deserialize("FuzzyMatching", stack)
                        ));
                    }
                    "SortExpression" => {
                        obj.sort_expression = Some(try!(StringDeserializer::deserialize(
                            "SortExpression",
                            stack
                        )));
                    }
                    "SourceField" => {
                        obj.source_field =
                            try!(FieldNameDeserializer::deserialize("SourceField", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `DocumentSuggesterOptions` contents to a `SignedRequest`.
struct DocumentSuggesterOptionsSerializer;
impl DocumentSuggesterOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DocumentSuggesterOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.fuzzy_matching {
            params.put(
                &format!("{}{}", prefix, "FuzzyMatching"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.sort_expression {
            params.put(
                &format!("{}{}", prefix, "SortExpression"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SourceField"),
            &obj.source_field.replace("+", "%2B"),
        );
    }
}

struct DomainIdDeserializer;
impl DomainIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DomainNameDeserializer;
impl DomainNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `DomainNameList` contents to a `SignedRequest`.
struct DomainNameListSerializer;
impl DomainNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct DomainNameMapDeserializer;
impl DomainNameMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(DomainNameDeserializer::deserialize("key", stack));
            let value = try!(APIVersionDeserializer::deserialize("value", stack));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// <p>The current status of the search domain.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DomainStatus {
    pub arn: Option<String>,
    /// <p>True if the search domain is created. It can take several minutes to initialize a domain when <a>CreateDomain</a> is called. Newly created search domains are returned from <a>DescribeDomains</a> with a false value for Created until domain creation is complete.</p>
    pub created: Option<bool>,
    /// <p>True if the search domain has been deleted. The system must clean up resources dedicated to the search domain when <a>DeleteDomain</a> is called. Newly deleted search domains are returned from <a>DescribeDomains</a> with a true value for IsDeleted for several minutes until resource cleanup is complete.</p>
    pub deleted: Option<bool>,
    /// <p>The service endpoint for updating documents in a search domain.</p>
    pub doc_service: Option<ServiceEndpoint>,
    pub domain_id: String,
    pub domain_name: String,
    pub limits: Option<Limits>,
    /// <p>True if processing is being done to activate the current domain configuration.</p>
    pub processing: Option<bool>,
    /// <p>True if <a>IndexDocuments</a> needs to be called to activate the current domain configuration.</p>
    pub requires_index_documents: bool,
    /// <p>The number of search instances that are available to process search requests.</p>
    pub search_instance_count: Option<i64>,
    /// <p>The instance type that is being used to process search requests.</p>
    pub search_instance_type: Option<String>,
    /// <p>The number of partitions across which the search index is spread.</p>
    pub search_partition_count: Option<i64>,
    /// <p>The service endpoint for requesting search results from a search domain.</p>
    pub search_service: Option<ServiceEndpoint>,
}

struct DomainStatusDeserializer;
impl DomainStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DomainStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DomainStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ARN" => {
                        obj.arn = Some(try!(ARNDeserializer::deserialize("ARN", stack)));
                    }
                    "Created" => {
                        obj.created =
                            Some(try!(BooleanDeserializer::deserialize("Created", stack)));
                    }
                    "Deleted" => {
                        obj.deleted =
                            Some(try!(BooleanDeserializer::deserialize("Deleted", stack)));
                    }
                    "DocService" => {
                        obj.doc_service = Some(try!(ServiceEndpointDeserializer::deserialize(
                            "DocService",
                            stack
                        )));
                    }
                    "DomainId" => {
                        obj.domain_id = try!(DomainIdDeserializer::deserialize("DomainId", stack));
                    }
                    "DomainName" => {
                        obj.domain_name =
                            try!(DomainNameDeserializer::deserialize("DomainName", stack));
                    }
                    "Limits" => {
                        obj.limits = Some(try!(LimitsDeserializer::deserialize("Limits", stack)));
                    }
                    "Processing" => {
                        obj.processing =
                            Some(try!(BooleanDeserializer::deserialize("Processing", stack)));
                    }
                    "RequiresIndexDocuments" => {
                        obj.requires_index_documents = try!(BooleanDeserializer::deserialize(
                            "RequiresIndexDocuments",
                            stack
                        ));
                    }
                    "SearchInstanceCount" => {
                        obj.search_instance_count = Some(try!(
                            InstanceCountDeserializer::deserialize("SearchInstanceCount", stack)
                        ));
                    }
                    "SearchInstanceType" => {
                        obj.search_instance_type =
                            Some(try!(SearchInstanceTypeDeserializer::deserialize(
                                "SearchInstanceType",
                                stack
                            )));
                    }
                    "SearchPartitionCount" => {
                        obj.search_partition_count = Some(try!(
                            PartitionCountDeserializer::deserialize("SearchPartitionCount", stack)
                        ));
                    }
                    "SearchService" => {
                        obj.search_service = Some(try!(ServiceEndpointDeserializer::deserialize(
                            "SearchService",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DomainStatusListDeserializer;
impl DomainStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DomainStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(DomainStatusDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct DoubleDeserializer;
impl DoubleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Options for a field that contains an array of double-precision 64-bit floating point values. Present if <code>IndexFieldType</code> specifies the field is of type <code>double-array</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DoubleArrayOptions {
    /// <p>A value to use for the field if the field isn&#39;t specified for a document.</p>
    pub default_value: Option<f64>,
    /// <p>Whether facet information can be returned for the field.</p>
    pub facet_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>Whether the contents of the field are searchable.</p>
    pub search_enabled: Option<bool>,
    /// <p>A list of source fields to map to the field. </p>
    pub source_fields: Option<String>,
}

struct DoubleArrayOptionsDeserializer;
impl DoubleArrayOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DoubleArrayOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DoubleArrayOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value =
                            Some(try!(DoubleDeserializer::deserialize("DefaultValue", stack)));
                    }
                    "FacetEnabled" => {
                        obj.facet_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "FacetEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SearchEnabled" => {
                        obj.search_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "SearchEnabled",
                            stack
                        )));
                    }
                    "SourceFields" => {
                        obj.source_fields = Some(try!(
                            FieldNameCommaListDeserializer::deserialize("SourceFields", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `DoubleArrayOptions` contents to a `SignedRequest`.
struct DoubleArrayOptionsSerializer;
impl DoubleArrayOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DoubleArrayOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(
                &format!("{}{}", prefix, "FacetEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(
                &format!("{}{}", prefix, "SearchEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_fields {
            params.put(
                &format!("{}{}", prefix, "SourceFields"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Options for a double-precision 64-bit floating point field. Present if <code>IndexFieldType</code> specifies the field is of type <code>double</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DoubleOptions {
    /// <p>A value to use for the field if the field isn't specified for a document. This can be important if you are using the field in an expression and that field is not present in every document.</p>
    pub default_value: Option<f64>,
    /// <p>Whether facet information can be returned for the field.</p>
    pub facet_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>Whether the contents of the field are searchable.</p>
    pub search_enabled: Option<bool>,
    /// <p>Whether the field can be used to sort the search results.</p>
    pub sort_enabled: Option<bool>,
    /// <p>The name of the source field to map to the field. </p>
    pub source_field: Option<String>,
}

struct DoubleOptionsDeserializer;
impl DoubleOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DoubleOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DoubleOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value =
                            Some(try!(DoubleDeserializer::deserialize("DefaultValue", stack)));
                    }
                    "FacetEnabled" => {
                        obj.facet_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "FacetEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SearchEnabled" => {
                        obj.search_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "SearchEnabled",
                            stack
                        )));
                    }
                    "SortEnabled" => {
                        obj.sort_enabled =
                            Some(try!(BooleanDeserializer::deserialize("SortEnabled", stack)));
                    }
                    "SourceField" => {
                        obj.source_field = Some(try!(FieldNameDeserializer::deserialize(
                            "SourceField",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `DoubleOptions` contents to a `SignedRequest`.
struct DoubleOptionsSerializer;
impl DoubleOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DoubleOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(
                &format!("{}{}", prefix, "FacetEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(
                &format!("{}{}", prefix, "SearchEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(
                &format!("{}{}", prefix, "SortEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(
                &format!("{}{}", prefix, "SourceField"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct DynamicFieldNameDeserializer;
impl DynamicFieldNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `DynamicFieldNameList` contents to a `SignedRequest`.
struct DynamicFieldNameListSerializer;
impl DynamicFieldNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>A named expression that can be evaluated at search time. Can be used to sort the search results, define other expressions, or return computed information in the search results. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Expression {
    pub expression_name: String,
    pub expression_value: String,
}

struct ExpressionDeserializer;
impl ExpressionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Expression, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Expression::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ExpressionName" => {
                        obj.expression_name = try!(StandardNameDeserializer::deserialize(
                            "ExpressionName",
                            stack
                        ));
                    }
                    "ExpressionValue" => {
                        obj.expression_value = try!(ExpressionValueDeserializer::deserialize(
                            "ExpressionValue",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `Expression` contents to a `SignedRequest`.
struct ExpressionSerializer;
impl ExpressionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Expression) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ExpressionName"),
            &obj.expression_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ExpressionValue"),
            &obj.expression_value.replace("+", "%2B"),
        );
    }
}

/// <p>The value of an <code>Expression</code> and its current status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExpressionStatus {
    /// <p>The expression that is evaluated for sorting while processing a search request.</p>
    pub options: Expression,
    pub status: OptionStatus,
}

struct ExpressionStatusDeserializer;
impl ExpressionStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ExpressionStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ExpressionStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Options" => {
                        obj.options = try!(ExpressionDeserializer::deserialize("Options", stack));
                    }
                    "Status" => {
                        obj.status = try!(OptionStatusDeserializer::deserialize("Status", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ExpressionStatusListDeserializer;
impl ExpressionStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ExpressionStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(ExpressionStatusDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ExpressionValueDeserializer;
impl ExpressionValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct FieldNameDeserializer;
impl FieldNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct FieldNameCommaListDeserializer;
impl FieldNameCommaListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct FieldNameListDeserializer;
impl FieldNameListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(FieldNameDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct FieldValueDeserializer;
impl FieldValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>IndexDocuments</a></code> operation. Specifies the name of the domain you want to re-index.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IndexDocumentsRequest {
    pub domain_name: String,
}

/// Serialize `IndexDocumentsRequest` contents to a `SignedRequest`.
struct IndexDocumentsRequestSerializer;
impl IndexDocumentsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &IndexDocumentsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of an <code>IndexDocuments</code> request. Contains the status of the indexing operation, including the fields being indexed.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IndexDocumentsResponse {
    /// <p>The names of the fields that are currently being indexed.</p>
    pub field_names: Option<Vec<String>>,
}

struct IndexDocumentsResponseDeserializer;
impl IndexDocumentsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IndexDocumentsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IndexDocumentsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "FieldNames" => {
                        obj.field_names = Some(try!(FieldNameListDeserializer::deserialize(
                            "FieldNames",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Configuration information for a field in the index, including its name, type, and options. The supported options depend on the <code><a>IndexFieldType</a></code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IndexField {
    pub date_array_options: Option<DateArrayOptions>,
    pub date_options: Option<DateOptions>,
    pub double_array_options: Option<DoubleArrayOptions>,
    pub double_options: Option<DoubleOptions>,
    /// <p>A string that represents the name of an index field. CloudSearch supports regular index fields as well as dynamic fields. A dynamic field's name defines a pattern that begins or ends with a wildcard. Any document fields that don't map to a regular index field but do match a dynamic field's pattern are configured with the dynamic field's indexing options. </p> <p>Regular field names begin with a letter and can contain the following characters: a-z (lowercase), 0-9, and _ (underscore). Dynamic field names must begin or end with a wildcard (*). The wildcard can also be the only character in a dynamic field name. Multiple wildcards, and wildcards embedded within a string are not supported. </p> <p>The name <code>score</code> is reserved and cannot be used as a field name. To reference a document's ID, you can use the name <code>_id</code>. </p>
    pub index_field_name: String,
    pub index_field_type: String,
    pub int_array_options: Option<IntArrayOptions>,
    pub int_options: Option<IntOptions>,
    pub lat_lon_options: Option<LatLonOptions>,
    pub literal_array_options: Option<LiteralArrayOptions>,
    pub literal_options: Option<LiteralOptions>,
    pub text_array_options: Option<TextArrayOptions>,
    pub text_options: Option<TextOptions>,
}

struct IndexFieldDeserializer;
impl IndexFieldDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IndexField, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IndexField::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DateArrayOptions" => {
                        obj.date_array_options = Some(try!(
                            DateArrayOptionsDeserializer::deserialize("DateArrayOptions", stack)
                        ));
                    }
                    "DateOptions" => {
                        obj.date_options = Some(try!(DateOptionsDeserializer::deserialize(
                            "DateOptions",
                            stack
                        )));
                    }
                    "DoubleArrayOptions" => {
                        obj.double_array_options =
                            Some(try!(DoubleArrayOptionsDeserializer::deserialize(
                                "DoubleArrayOptions",
                                stack
                            )));
                    }
                    "DoubleOptions" => {
                        obj.double_options = Some(try!(DoubleOptionsDeserializer::deserialize(
                            "DoubleOptions",
                            stack
                        )));
                    }
                    "IndexFieldName" => {
                        obj.index_field_name = try!(DynamicFieldNameDeserializer::deserialize(
                            "IndexFieldName",
                            stack
                        ));
                    }
                    "IndexFieldType" => {
                        obj.index_field_type = try!(IndexFieldTypeDeserializer::deserialize(
                            "IndexFieldType",
                            stack
                        ));
                    }
                    "IntArrayOptions" => {
                        obj.int_array_options = Some(try!(
                            IntArrayOptionsDeserializer::deserialize("IntArrayOptions", stack)
                        ));
                    }
                    "IntOptions" => {
                        obj.int_options = Some(try!(IntOptionsDeserializer::deserialize(
                            "IntOptions",
                            stack
                        )));
                    }
                    "LatLonOptions" => {
                        obj.lat_lon_options = Some(try!(LatLonOptionsDeserializer::deserialize(
                            "LatLonOptions",
                            stack
                        )));
                    }
                    "LiteralArrayOptions" => {
                        obj.literal_array_options =
                            Some(try!(LiteralArrayOptionsDeserializer::deserialize(
                                "LiteralArrayOptions",
                                stack
                            )));
                    }
                    "LiteralOptions" => {
                        obj.literal_options = Some(try!(LiteralOptionsDeserializer::deserialize(
                            "LiteralOptions",
                            stack
                        )));
                    }
                    "TextArrayOptions" => {
                        obj.text_array_options = Some(try!(
                            TextArrayOptionsDeserializer::deserialize("TextArrayOptions", stack)
                        ));
                    }
                    "TextOptions" => {
                        obj.text_options = Some(try!(TextOptionsDeserializer::deserialize(
                            "TextOptions",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `IndexField` contents to a `SignedRequest`.
struct IndexFieldSerializer;
impl IndexFieldSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &IndexField) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.date_array_options {
            DateArrayOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DateArrayOptions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.date_options {
            DateOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DateOptions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.double_array_options {
            DoubleArrayOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DoubleArrayOptions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.double_options {
            DoubleOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DoubleOptions"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "IndexFieldName"),
            &obj.index_field_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "IndexFieldType"),
            &obj.index_field_type.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.int_array_options {
            IntArrayOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "IntArrayOptions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.int_options {
            IntOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "IntOptions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.lat_lon_options {
            LatLonOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LatLonOptions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.literal_array_options {
            LiteralArrayOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LiteralArrayOptions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.literal_options {
            LiteralOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LiteralOptions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.text_array_options {
            TextArrayOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TextArrayOptions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.text_options {
            TextOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TextOptions"),
                field_value,
            );
        }
    }
}

/// <p>The value of an <code>IndexField</code> and its current status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IndexFieldStatus {
    pub options: IndexField,
    pub status: OptionStatus,
}

struct IndexFieldStatusDeserializer;
impl IndexFieldStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IndexFieldStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IndexFieldStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Options" => {
                        obj.options = try!(IndexFieldDeserializer::deserialize("Options", stack));
                    }
                    "Status" => {
                        obj.status = try!(OptionStatusDeserializer::deserialize("Status", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IndexFieldStatusListDeserializer;
impl IndexFieldStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<IndexFieldStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(IndexFieldStatusDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct IndexFieldTypeDeserializer;
impl IndexFieldTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct InstanceCountDeserializer;
impl InstanceCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Options for a field that contains an array of 64-bit signed integers. Present if <code>IndexFieldType</code> specifies the field is of type <code>int-array</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IntArrayOptions {
    /// <p>A value to use for the field if the field isn&#39;t specified for a document.</p>
    pub default_value: Option<i64>,
    /// <p>Whether facet information can be returned for the field.</p>
    pub facet_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>Whether the contents of the field are searchable.</p>
    pub search_enabled: Option<bool>,
    /// <p>A list of source fields to map to the field. </p>
    pub source_fields: Option<String>,
}

struct IntArrayOptionsDeserializer;
impl IntArrayOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IntArrayOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IntArrayOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value =
                            Some(try!(LongDeserializer::deserialize("DefaultValue", stack)));
                    }
                    "FacetEnabled" => {
                        obj.facet_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "FacetEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SearchEnabled" => {
                        obj.search_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "SearchEnabled",
                            stack
                        )));
                    }
                    "SourceFields" => {
                        obj.source_fields = Some(try!(
                            FieldNameCommaListDeserializer::deserialize("SourceFields", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `IntArrayOptions` contents to a `SignedRequest`.
struct IntArrayOptionsSerializer;
impl IntArrayOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &IntArrayOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(
                &format!("{}{}", prefix, "FacetEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(
                &format!("{}{}", prefix, "SearchEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_fields {
            params.put(
                &format!("{}{}", prefix, "SourceFields"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Options for a 64-bit signed integer field. Present if <code>IndexFieldType</code> specifies the field is of type <code>int</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IntOptions {
    /// <p>A value to use for the field if the field isn&#39;t specified for a document. This can be important if you are using the field in an expression and that field is not present in every document.</p>
    pub default_value: Option<i64>,
    /// <p>Whether facet information can be returned for the field.</p>
    pub facet_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>Whether the contents of the field are searchable.</p>
    pub search_enabled: Option<bool>,
    /// <p>Whether the field can be used to sort the search results.</p>
    pub sort_enabled: Option<bool>,
    /// <p>The name of the source field to map to the field. </p>
    pub source_field: Option<String>,
}

struct IntOptionsDeserializer;
impl IntOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IntOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IntOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value =
                            Some(try!(LongDeserializer::deserialize("DefaultValue", stack)));
                    }
                    "FacetEnabled" => {
                        obj.facet_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "FacetEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SearchEnabled" => {
                        obj.search_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "SearchEnabled",
                            stack
                        )));
                    }
                    "SortEnabled" => {
                        obj.sort_enabled =
                            Some(try!(BooleanDeserializer::deserialize("SortEnabled", stack)));
                    }
                    "SourceField" => {
                        obj.source_field = Some(try!(FieldNameDeserializer::deserialize(
                            "SourceField",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `IntOptions` contents to a `SignedRequest`.
struct IntOptionsSerializer;
impl IntOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &IntOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(
                &format!("{}{}", prefix, "FacetEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(
                &format!("{}{}", prefix, "SearchEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(
                &format!("{}{}", prefix, "SortEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(
                &format!("{}{}", prefix, "SourceField"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Options for a latlon field. A latlon field contains a location stored as a latitude and longitude value pair. Present if <code>IndexFieldType</code> specifies the field is of type <code>latlon</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LatLonOptions {
    /// <p>A value to use for the field if the field isn&#39;t specified for a document.</p>
    pub default_value: Option<String>,
    /// <p>Whether facet information can be returned for the field.</p>
    pub facet_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>Whether the contents of the field are searchable.</p>
    pub search_enabled: Option<bool>,
    /// <p>Whether the field can be used to sort the search results.</p>
    pub sort_enabled: Option<bool>,
    pub source_field: Option<String>,
}

struct LatLonOptionsDeserializer;
impl LatLonOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LatLonOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LatLonOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value = Some(try!(FieldValueDeserializer::deserialize(
                            "DefaultValue",
                            stack
                        )));
                    }
                    "FacetEnabled" => {
                        obj.facet_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "FacetEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SearchEnabled" => {
                        obj.search_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "SearchEnabled",
                            stack
                        )));
                    }
                    "SortEnabled" => {
                        obj.sort_enabled =
                            Some(try!(BooleanDeserializer::deserialize("SortEnabled", stack)));
                    }
                    "SourceField" => {
                        obj.source_field = Some(try!(FieldNameDeserializer::deserialize(
                            "SourceField",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `LatLonOptions` contents to a `SignedRequest`.
struct LatLonOptionsSerializer;
impl LatLonOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LatLonOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(
                &format!("{}{}", prefix, "FacetEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(
                &format!("{}{}", prefix, "SearchEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(
                &format!("{}{}", prefix, "SortEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(
                &format!("{}{}", prefix, "SourceField"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Limits {
    pub maximum_partition_count: i64,
    pub maximum_replication_count: i64,
}

struct LimitsDeserializer;
impl LimitsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Limits, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Limits::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MaximumPartitionCount" => {
                        obj.maximum_partition_count =
                            try!(MaximumPartitionCountDeserializer::deserialize(
                                "MaximumPartitionCount",
                                stack
                            ));
                    }
                    "MaximumReplicationCount" => {
                        obj.maximum_replication_count =
                            try!(MaximumReplicationCountDeserializer::deserialize(
                                "MaximumReplicationCount",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The result of a <code>ListDomainNames</code> request. Contains a list of the domains owned by an account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDomainNamesResponse {
    /// <p>The names of the search domains owned by an account.</p>
    pub domain_names: Option<::std::collections::HashMap<String, String>>,
}

struct ListDomainNamesResponseDeserializer;
impl ListDomainNamesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDomainNamesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListDomainNamesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DomainNames" => {
                        obj.domain_names = Some(try!(DomainNameMapDeserializer::deserialize(
                            "DomainNames",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Options for a field that contains an array of literal strings. Present if <code>IndexFieldType</code> specifies the field is of type <code>literal-array</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LiteralArrayOptions {
    /// <p>A value to use for the field if the field isn&#39;t specified for a document.</p>
    pub default_value: Option<String>,
    /// <p>Whether facet information can be returned for the field.</p>
    pub facet_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>Whether the contents of the field are searchable.</p>
    pub search_enabled: Option<bool>,
    /// <p>A list of source fields to map to the field. </p>
    pub source_fields: Option<String>,
}

struct LiteralArrayOptionsDeserializer;
impl LiteralArrayOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LiteralArrayOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LiteralArrayOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value = Some(try!(FieldValueDeserializer::deserialize(
                            "DefaultValue",
                            stack
                        )));
                    }
                    "FacetEnabled" => {
                        obj.facet_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "FacetEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SearchEnabled" => {
                        obj.search_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "SearchEnabled",
                            stack
                        )));
                    }
                    "SourceFields" => {
                        obj.source_fields = Some(try!(
                            FieldNameCommaListDeserializer::deserialize("SourceFields", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `LiteralArrayOptions` contents to a `SignedRequest`.
struct LiteralArrayOptionsSerializer;
impl LiteralArrayOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LiteralArrayOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(
                &format!("{}{}", prefix, "FacetEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(
                &format!("{}{}", prefix, "SearchEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_fields {
            params.put(
                &format!("{}{}", prefix, "SourceFields"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Options for literal field. Present if <code>IndexFieldType</code> specifies the field is of type <code>literal</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LiteralOptions {
    /// <p>A value to use for the field if the field isn&#39;t specified for a document.</p>
    pub default_value: Option<String>,
    /// <p>Whether facet information can be returned for the field.</p>
    pub facet_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>Whether the contents of the field are searchable.</p>
    pub search_enabled: Option<bool>,
    /// <p>Whether the field can be used to sort the search results.</p>
    pub sort_enabled: Option<bool>,
    pub source_field: Option<String>,
}

struct LiteralOptionsDeserializer;
impl LiteralOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LiteralOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LiteralOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value = Some(try!(FieldValueDeserializer::deserialize(
                            "DefaultValue",
                            stack
                        )));
                    }
                    "FacetEnabled" => {
                        obj.facet_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "FacetEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SearchEnabled" => {
                        obj.search_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "SearchEnabled",
                            stack
                        )));
                    }
                    "SortEnabled" => {
                        obj.sort_enabled =
                            Some(try!(BooleanDeserializer::deserialize("SortEnabled", stack)));
                    }
                    "SourceField" => {
                        obj.source_field = Some(try!(FieldNameDeserializer::deserialize(
                            "SourceField",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `LiteralOptions` contents to a `SignedRequest`.
struct LiteralOptionsSerializer;
impl LiteralOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LiteralOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(
                &format!("{}{}", prefix, "FacetEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(
                &format!("{}{}", prefix, "SearchEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(
                &format!("{}{}", prefix, "SortEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(
                &format!("{}{}", prefix, "SourceField"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct LongDeserializer;
impl LongDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MaximumPartitionCountDeserializer;
impl MaximumPartitionCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MaximumReplicationCountDeserializer;
impl MaximumReplicationCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MultiAZDeserializer;
impl MultiAZDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct OptionStateDeserializer;
impl OptionStateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The status of domain configuration option.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OptionStatus {
    /// <p>A timestamp for when this option was created.</p>
    pub creation_date: String,
    /// <p>Indicates that the option will be deleted once processing is complete.</p>
    pub pending_deletion: Option<bool>,
    /// <p><p>The state of processing a change to an option. Possible values:</p> <ul> <li> <code>RequiresIndexDocuments</code>: the option&#39;s latest value will not be deployed until <a>IndexDocuments</a> has been called and indexing is complete.</li> <li> <code>Processing</code>: the option&#39;s latest value is in the process of being activated. </li> <li> <code>Active</code>: the option&#39;s latest value is completely deployed.</li> <li> <code>FailedToValidate</code>: the option value is not compatible with the domain&#39;s data and cannot be used to index the data. You must either modify the option value or update or remove the incompatible documents.</li> </ul></p>
    pub state: String,
    /// <p>A timestamp for when this option was last updated.</p>
    pub update_date: String,
    /// <p>A unique integer that indicates when this option was last updated.</p>
    pub update_version: Option<i64>,
}

struct OptionStatusDeserializer;
impl OptionStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OptionStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = OptionStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CreationDate" => {
                        obj.creation_date = try!(UpdateTimestampDeserializer::deserialize(
                            "CreationDate",
                            stack
                        ));
                    }
                    "PendingDeletion" => {
                        obj.pending_deletion = Some(try!(BooleanDeserializer::deserialize(
                            "PendingDeletion",
                            stack
                        )));
                    }
                    "State" => {
                        obj.state = try!(OptionStateDeserializer::deserialize("State", stack));
                    }
                    "UpdateDate" => {
                        obj.update_date = try!(UpdateTimestampDeserializer::deserialize(
                            "UpdateDate",
                            stack
                        ));
                    }
                    "UpdateVersion" => {
                        obj.update_version = Some(try!(UIntValueDeserializer::deserialize(
                            "UpdateVersion",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct PartitionCountDeserializer;
impl PartitionCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct PartitionInstanceTypeDeserializer;
impl PartitionInstanceTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct PolicyDocumentDeserializer;
impl PolicyDocumentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The desired instance type and desired number of replicas of each index partition.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScalingParameters {
    /// <p>The instance type that you want to preconfigure for your domain. For example, <code>search.m1.small</code>.</p>
    pub desired_instance_type: Option<String>,
    /// <p>The number of partitions you want to preconfigure for your domain. Only valid when you select <code>m2.2xlarge</code> as the desired instance type.</p>
    pub desired_partition_count: Option<i64>,
    /// <p>The number of replicas you want to preconfigure for each index partition.</p>
    pub desired_replication_count: Option<i64>,
}

struct ScalingParametersDeserializer;
impl ScalingParametersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScalingParameters, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ScalingParameters::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DesiredInstanceType" => {
                        obj.desired_instance_type =
                            Some(try!(PartitionInstanceTypeDeserializer::deserialize(
                                "DesiredInstanceType",
                                stack
                            )));
                    }
                    "DesiredPartitionCount" => {
                        obj.desired_partition_count = Some(try!(
                            UIntValueDeserializer::deserialize("DesiredPartitionCount", stack)
                        ));
                    }
                    "DesiredReplicationCount" => {
                        obj.desired_replication_count = Some(try!(
                            UIntValueDeserializer::deserialize("DesiredReplicationCount", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `ScalingParameters` contents to a `SignedRequest`.
struct ScalingParametersSerializer;
impl ScalingParametersSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ScalingParameters) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.desired_instance_type {
            params.put(
                &format!("{}{}", prefix, "DesiredInstanceType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.desired_partition_count {
            params.put(
                &format!("{}{}", prefix, "DesiredPartitionCount"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.desired_replication_count {
            params.put(
                &format!("{}{}", prefix, "DesiredReplicationCount"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>The status and configuration of a search domain's scaling parameters. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScalingParametersStatus {
    pub options: ScalingParameters,
    pub status: OptionStatus,
}

struct ScalingParametersStatusDeserializer;
impl ScalingParametersStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScalingParametersStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ScalingParametersStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Options" => {
                        obj.options =
                            try!(ScalingParametersDeserializer::deserialize("Options", stack));
                    }
                    "Status" => {
                        obj.status = try!(OptionStatusDeserializer::deserialize("Status", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SearchInstanceTypeDeserializer;
impl SearchInstanceTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The endpoint to which service requests can be submitted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServiceEndpoint {
    pub endpoint: Option<String>,
}

struct ServiceEndpointDeserializer;
impl ServiceEndpointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServiceEndpoint, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ServiceEndpoint::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Endpoint" => {
                        obj.endpoint =
                            Some(try!(ServiceUrlDeserializer::deserialize("Endpoint", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ServiceUrlDeserializer;
impl ServiceUrlDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StandardNameDeserializer;
impl StandardNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `StandardNameList` contents to a `SignedRequest`.
struct StandardNameListSerializer;
impl StandardNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct StringDeserializer;
impl StringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Configuration information for a search suggester. Each suggester has a unique name and specifies the text field you want to use for suggestions. The following options can be configured for a suggester: <code>FuzzyMatching</code>, <code>SortExpression</code>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Suggester {
    pub document_suggester_options: DocumentSuggesterOptions,
    pub suggester_name: String,
}

struct SuggesterDeserializer;
impl SuggesterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Suggester, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Suggester::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DocumentSuggesterOptions" => {
                        obj.document_suggester_options =
                            try!(DocumentSuggesterOptionsDeserializer::deserialize(
                                "DocumentSuggesterOptions",
                                stack
                            ));
                    }
                    "SuggesterName" => {
                        obj.suggester_name = try!(StandardNameDeserializer::deserialize(
                            "SuggesterName",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `Suggester` contents to a `SignedRequest`.
struct SuggesterSerializer;
impl SuggesterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Suggester) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        DocumentSuggesterOptionsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "DocumentSuggesterOptions"),
            &obj.document_suggester_options,
        );
        params.put(
            &format!("{}{}", prefix, "SuggesterName"),
            &obj.suggester_name.replace("+", "%2B"),
        );
    }
}

struct SuggesterFuzzyMatchingDeserializer;
impl SuggesterFuzzyMatchingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The value of a <code>Suggester</code> and its current status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SuggesterStatus {
    pub options: Suggester,
    pub status: OptionStatus,
}

struct SuggesterStatusDeserializer;
impl SuggesterStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SuggesterStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SuggesterStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Options" => {
                        obj.options = try!(SuggesterDeserializer::deserialize("Options", stack));
                    }
                    "Status" => {
                        obj.status = try!(OptionStatusDeserializer::deserialize("Status", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SuggesterStatusListDeserializer;
impl SuggesterStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SuggesterStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(SuggesterStatusDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Options for a field that contains an array of text strings. Present if <code>IndexFieldType</code> specifies the field is of type <code>text-array</code>. A <code>text-array</code> field is always searchable. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextArrayOptions {
    /// <p>The name of an analysis scheme for a <code>text-array</code> field.</p>
    pub analysis_scheme: Option<String>,
    /// <p>A value to use for the field if the field isn&#39;t specified for a document.</p>
    pub default_value: Option<String>,
    /// <p>Whether highlights can be returned for the field.</p>
    pub highlight_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>A list of source fields to map to the field. </p>
    pub source_fields: Option<String>,
}

struct TextArrayOptionsDeserializer;
impl TextArrayOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TextArrayOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TextArrayOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AnalysisScheme" => {
                        obj.analysis_scheme =
                            Some(try!(WordDeserializer::deserialize("AnalysisScheme", stack)));
                    }
                    "DefaultValue" => {
                        obj.default_value = Some(try!(FieldValueDeserializer::deserialize(
                            "DefaultValue",
                            stack
                        )));
                    }
                    "HighlightEnabled" => {
                        obj.highlight_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "HighlightEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SourceFields" => {
                        obj.source_fields = Some(try!(
                            FieldNameCommaListDeserializer::deserialize("SourceFields", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `TextArrayOptions` contents to a `SignedRequest`.
struct TextArrayOptionsSerializer;
impl TextArrayOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TextArrayOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.analysis_scheme {
            params.put(
                &format!("{}{}", prefix, "AnalysisScheme"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.highlight_enabled {
            params.put(
                &format!("{}{}", prefix, "HighlightEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_fields {
            params.put(
                &format!("{}{}", prefix, "SourceFields"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Options for text field. Present if <code>IndexFieldType</code> specifies the field is of type <code>text</code>. A <code>text</code> field is always searchable. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextOptions {
    /// <p>The name of an analysis scheme for a <code>text</code> field.</p>
    pub analysis_scheme: Option<String>,
    /// <p>A value to use for the field if the field isn&#39;t specified for a document.</p>
    pub default_value: Option<String>,
    /// <p>Whether highlights can be returned for the field.</p>
    pub highlight_enabled: Option<bool>,
    /// <p>Whether the contents of the field can be returned in the search results.</p>
    pub return_enabled: Option<bool>,
    /// <p>Whether the field can be used to sort the search results.</p>
    pub sort_enabled: Option<bool>,
    pub source_field: Option<String>,
}

struct TextOptionsDeserializer;
impl TextOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TextOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TextOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AnalysisScheme" => {
                        obj.analysis_scheme =
                            Some(try!(WordDeserializer::deserialize("AnalysisScheme", stack)));
                    }
                    "DefaultValue" => {
                        obj.default_value = Some(try!(FieldValueDeserializer::deserialize(
                            "DefaultValue",
                            stack
                        )));
                    }
                    "HighlightEnabled" => {
                        obj.highlight_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "HighlightEnabled",
                            stack
                        )));
                    }
                    "ReturnEnabled" => {
                        obj.return_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "ReturnEnabled",
                            stack
                        )));
                    }
                    "SortEnabled" => {
                        obj.sort_enabled =
                            Some(try!(BooleanDeserializer::deserialize("SortEnabled", stack)));
                    }
                    "SourceField" => {
                        obj.source_field = Some(try!(FieldNameDeserializer::deserialize(
                            "SourceField",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `TextOptions` contents to a `SignedRequest`.
struct TextOptionsSerializer;
impl TextOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TextOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.analysis_scheme {
            params.put(
                &format!("{}{}", prefix, "AnalysisScheme"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.default_value {
            params.put(
                &format!("{}{}", prefix, "DefaultValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.highlight_enabled {
            params.put(
                &format!("{}{}", prefix, "HighlightEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(
                &format!("{}{}", prefix, "ReturnEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(
                &format!("{}{}", prefix, "SortEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(
                &format!("{}{}", prefix, "SourceField"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct UIntValueDeserializer;
impl UIntValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>UpdateAvailabilityOptions</a></code> operation. Specifies the name of the domain you want to update and the Multi-AZ availability option.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateAvailabilityOptionsRequest {
    pub domain_name: String,
    /// <p>You expand an existing search domain to a second Availability Zone by setting the Multi-AZ option to true. Similarly, you can turn off the Multi-AZ option to downgrade the domain to a single Availability Zone by setting the Multi-AZ option to <code>false</code>. </p>
    pub multi_az: bool,
}

/// Serialize `UpdateAvailabilityOptionsRequest` contents to a `SignedRequest`.
struct UpdateAvailabilityOptionsRequestSerializer;
impl UpdateAvailabilityOptionsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateAvailabilityOptionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "MultiAZ"),
            &obj.multi_az.to_string().replace("+", "%2B"),
        );
    }
}

/// <p>The result of a <code>UpdateAvailabilityOptions</code> request. Contains the status of the domain's availability options. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateAvailabilityOptionsResponse {
    /// <p>The newly-configured availability options. Indicates whether Multi-AZ is enabled for the domain. </p>
    pub availability_options: Option<AvailabilityOptionsStatus>,
}

struct UpdateAvailabilityOptionsResponseDeserializer;
impl UpdateAvailabilityOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateAvailabilityOptionsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateAvailabilityOptionsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AvailabilityOptions" => {
                        obj.availability_options =
                            Some(try!(AvailabilityOptionsStatusDeserializer::deserialize(
                                "AvailabilityOptions",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>UpdateScalingParameters</a></code> operation. Specifies the name of the domain you want to update and the scaling parameters you want to configure.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateScalingParametersRequest {
    pub domain_name: String,
    pub scaling_parameters: ScalingParameters,
}

/// Serialize `UpdateScalingParametersRequest` contents to a `SignedRequest`.
struct UpdateScalingParametersRequestSerializer;
impl UpdateScalingParametersRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateScalingParametersRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
        ScalingParametersSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ScalingParameters"),
            &obj.scaling_parameters,
        );
    }
}

/// <p>The result of a <code>UpdateScalingParameters</code> request. Contains the status of the newly-configured scaling parameters.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateScalingParametersResponse {
    pub scaling_parameters: ScalingParametersStatus,
}

struct UpdateScalingParametersResponseDeserializer;
impl UpdateScalingParametersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateScalingParametersResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateScalingParametersResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ScalingParameters" => {
                        obj.scaling_parameters =
                            try!(ScalingParametersStatusDeserializer::deserialize(
                                "ScalingParameters",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>UpdateServiceAccessPolicies</a></code> operation. Specifies the name of the domain you want to update and the access rules you want to configure.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateServiceAccessPoliciesRequest {
    /// <p>The access rules you want to configure. These rules replace any existing rules. </p>
    pub access_policies: String,
    pub domain_name: String,
}

/// Serialize `UpdateServiceAccessPoliciesRequest` contents to a `SignedRequest`.
struct UpdateServiceAccessPoliciesRequestSerializer;
impl UpdateServiceAccessPoliciesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateServiceAccessPoliciesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AccessPolicies"),
            &obj.access_policies.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "DomainName"),
            &obj.domain_name.replace("+", "%2B"),
        );
    }
}

/// <p>The result of an <code>UpdateServiceAccessPolicies</code> request. Contains the new access policies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateServiceAccessPoliciesResponse {
    /// <p>The access rules configured for the domain.</p>
    pub access_policies: AccessPoliciesStatus,
}

struct UpdateServiceAccessPoliciesResponseDeserializer;
impl UpdateServiceAccessPoliciesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateServiceAccessPoliciesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateServiceAccessPoliciesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AccessPolicies" => {
                        obj.access_policies = try!(AccessPoliciesStatusDeserializer::deserialize(
                            "AccessPolicies",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct UpdateTimestampDeserializer;
impl UpdateTimestampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct WordDeserializer;
impl WordDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// Errors returned by BuildSuggesters
#[derive(Debug, PartialEq)]
pub enum BuildSuggestersError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BuildSuggestersError {
    pub fn from_body(body: &str) -> BuildSuggestersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => BuildSuggestersError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    BuildSuggestersError::Internal(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    BuildSuggestersError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => BuildSuggestersError::Unknown(String::from(body)),
            },
            Err(_) => BuildSuggestersError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for BuildSuggestersError {
    fn from(err: XmlParseError) -> BuildSuggestersError {
        let XmlParseError(message) = err;
        BuildSuggestersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for BuildSuggestersError {
    fn from(err: CredentialsError) -> BuildSuggestersError {
        BuildSuggestersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BuildSuggestersError {
    fn from(err: HttpDispatchError) -> BuildSuggestersError {
        BuildSuggestersError::HttpDispatch(err)
    }
}
impl From<io::Error> for BuildSuggestersError {
    fn from(err: io::Error) -> BuildSuggestersError {
        BuildSuggestersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BuildSuggestersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BuildSuggestersError {
    fn description(&self) -> &str {
        match *self {
            BuildSuggestersError::Base(ref cause) => cause,
            BuildSuggestersError::Internal(ref cause) => cause,
            BuildSuggestersError::ResourceNotFound(ref cause) => cause,
            BuildSuggestersError::Validation(ref cause) => cause,
            BuildSuggestersError::Credentials(ref err) => err.description(),
            BuildSuggestersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BuildSuggestersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDomain
#[derive(Debug, PartialEq)]
pub enum CreateDomainError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDomainError {
    pub fn from_body(body: &str) -> CreateDomainError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => CreateDomainError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    CreateDomainError::Internal(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    CreateDomainError::LimitExceeded(String::from(parsed_error.message))
                }
                _ => CreateDomainError::Unknown(String::from(body)),
            },
            Err(_) => CreateDomainError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateDomainError {
    fn from(err: XmlParseError) -> CreateDomainError {
        let XmlParseError(message) = err;
        CreateDomainError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateDomainError {
    fn from(err: CredentialsError) -> CreateDomainError {
        CreateDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDomainError {
    fn from(err: HttpDispatchError) -> CreateDomainError {
        CreateDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDomainError {
    fn from(err: io::Error) -> CreateDomainError {
        CreateDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDomainError {
    fn description(&self) -> &str {
        match *self {
            CreateDomainError::Base(ref cause) => cause,
            CreateDomainError::Internal(ref cause) => cause,
            CreateDomainError::LimitExceeded(ref cause) => cause,
            CreateDomainError::Validation(ref cause) => cause,
            CreateDomainError::Credentials(ref err) => err.description(),
            CreateDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DefineAnalysisScheme
#[derive(Debug, PartialEq)]
pub enum DefineAnalysisSchemeError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DefineAnalysisSchemeError {
    pub fn from_body(body: &str) -> DefineAnalysisSchemeError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    DefineAnalysisSchemeError::Base(String::from(parsed_error.message))
                }
                "InternalException" => {
                    DefineAnalysisSchemeError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => {
                    DefineAnalysisSchemeError::InvalidType(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    DefineAnalysisSchemeError::LimitExceeded(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DefineAnalysisSchemeError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DefineAnalysisSchemeError::Unknown(String::from(body)),
            },
            Err(_) => DefineAnalysisSchemeError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DefineAnalysisSchemeError {
    fn from(err: XmlParseError) -> DefineAnalysisSchemeError {
        let XmlParseError(message) = err;
        DefineAnalysisSchemeError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DefineAnalysisSchemeError {
    fn from(err: CredentialsError) -> DefineAnalysisSchemeError {
        DefineAnalysisSchemeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DefineAnalysisSchemeError {
    fn from(err: HttpDispatchError) -> DefineAnalysisSchemeError {
        DefineAnalysisSchemeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DefineAnalysisSchemeError {
    fn from(err: io::Error) -> DefineAnalysisSchemeError {
        DefineAnalysisSchemeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DefineAnalysisSchemeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DefineAnalysisSchemeError {
    fn description(&self) -> &str {
        match *self {
            DefineAnalysisSchemeError::Base(ref cause) => cause,
            DefineAnalysisSchemeError::Internal(ref cause) => cause,
            DefineAnalysisSchemeError::InvalidType(ref cause) => cause,
            DefineAnalysisSchemeError::LimitExceeded(ref cause) => cause,
            DefineAnalysisSchemeError::ResourceNotFound(ref cause) => cause,
            DefineAnalysisSchemeError::Validation(ref cause) => cause,
            DefineAnalysisSchemeError::Credentials(ref err) => err.description(),
            DefineAnalysisSchemeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DefineAnalysisSchemeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DefineExpression
#[derive(Debug, PartialEq)]
pub enum DefineExpressionError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DefineExpressionError {
    pub fn from_body(body: &str) -> DefineExpressionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => DefineExpressionError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    DefineExpressionError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => {
                    DefineExpressionError::InvalidType(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    DefineExpressionError::LimitExceeded(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DefineExpressionError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DefineExpressionError::Unknown(String::from(body)),
            },
            Err(_) => DefineExpressionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DefineExpressionError {
    fn from(err: XmlParseError) -> DefineExpressionError {
        let XmlParseError(message) = err;
        DefineExpressionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DefineExpressionError {
    fn from(err: CredentialsError) -> DefineExpressionError {
        DefineExpressionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DefineExpressionError {
    fn from(err: HttpDispatchError) -> DefineExpressionError {
        DefineExpressionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DefineExpressionError {
    fn from(err: io::Error) -> DefineExpressionError {
        DefineExpressionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DefineExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DefineExpressionError {
    fn description(&self) -> &str {
        match *self {
            DefineExpressionError::Base(ref cause) => cause,
            DefineExpressionError::Internal(ref cause) => cause,
            DefineExpressionError::InvalidType(ref cause) => cause,
            DefineExpressionError::LimitExceeded(ref cause) => cause,
            DefineExpressionError::ResourceNotFound(ref cause) => cause,
            DefineExpressionError::Validation(ref cause) => cause,
            DefineExpressionError::Credentials(ref err) => err.description(),
            DefineExpressionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DefineExpressionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DefineIndexField
#[derive(Debug, PartialEq)]
pub enum DefineIndexFieldError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DefineIndexFieldError {
    pub fn from_body(body: &str) -> DefineIndexFieldError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => DefineIndexFieldError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    DefineIndexFieldError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => {
                    DefineIndexFieldError::InvalidType(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    DefineIndexFieldError::LimitExceeded(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DefineIndexFieldError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DefineIndexFieldError::Unknown(String::from(body)),
            },
            Err(_) => DefineIndexFieldError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DefineIndexFieldError {
    fn from(err: XmlParseError) -> DefineIndexFieldError {
        let XmlParseError(message) = err;
        DefineIndexFieldError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DefineIndexFieldError {
    fn from(err: CredentialsError) -> DefineIndexFieldError {
        DefineIndexFieldError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DefineIndexFieldError {
    fn from(err: HttpDispatchError) -> DefineIndexFieldError {
        DefineIndexFieldError::HttpDispatch(err)
    }
}
impl From<io::Error> for DefineIndexFieldError {
    fn from(err: io::Error) -> DefineIndexFieldError {
        DefineIndexFieldError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DefineIndexFieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DefineIndexFieldError {
    fn description(&self) -> &str {
        match *self {
            DefineIndexFieldError::Base(ref cause) => cause,
            DefineIndexFieldError::Internal(ref cause) => cause,
            DefineIndexFieldError::InvalidType(ref cause) => cause,
            DefineIndexFieldError::LimitExceeded(ref cause) => cause,
            DefineIndexFieldError::ResourceNotFound(ref cause) => cause,
            DefineIndexFieldError::Validation(ref cause) => cause,
            DefineIndexFieldError::Credentials(ref err) => err.description(),
            DefineIndexFieldError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DefineIndexFieldError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DefineSuggester
#[derive(Debug, PartialEq)]
pub enum DefineSuggesterError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DefineSuggesterError {
    pub fn from_body(body: &str) -> DefineSuggesterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => DefineSuggesterError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    DefineSuggesterError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => {
                    DefineSuggesterError::InvalidType(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    DefineSuggesterError::LimitExceeded(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DefineSuggesterError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DefineSuggesterError::Unknown(String::from(body)),
            },
            Err(_) => DefineSuggesterError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DefineSuggesterError {
    fn from(err: XmlParseError) -> DefineSuggesterError {
        let XmlParseError(message) = err;
        DefineSuggesterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DefineSuggesterError {
    fn from(err: CredentialsError) -> DefineSuggesterError {
        DefineSuggesterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DefineSuggesterError {
    fn from(err: HttpDispatchError) -> DefineSuggesterError {
        DefineSuggesterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DefineSuggesterError {
    fn from(err: io::Error) -> DefineSuggesterError {
        DefineSuggesterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DefineSuggesterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DefineSuggesterError {
    fn description(&self) -> &str {
        match *self {
            DefineSuggesterError::Base(ref cause) => cause,
            DefineSuggesterError::Internal(ref cause) => cause,
            DefineSuggesterError::InvalidType(ref cause) => cause,
            DefineSuggesterError::LimitExceeded(ref cause) => cause,
            DefineSuggesterError::ResourceNotFound(ref cause) => cause,
            DefineSuggesterError::Validation(ref cause) => cause,
            DefineSuggesterError::Credentials(ref err) => err.description(),
            DefineSuggesterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DefineSuggesterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAnalysisScheme
#[derive(Debug, PartialEq)]
pub enum DeleteAnalysisSchemeError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteAnalysisSchemeError {
    pub fn from_body(body: &str) -> DeleteAnalysisSchemeError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    DeleteAnalysisSchemeError::Base(String::from(parsed_error.message))
                }
                "InternalException" => {
                    DeleteAnalysisSchemeError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => {
                    DeleteAnalysisSchemeError::InvalidType(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DeleteAnalysisSchemeError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DeleteAnalysisSchemeError::Unknown(String::from(body)),
            },
            Err(_) => DeleteAnalysisSchemeError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteAnalysisSchemeError {
    fn from(err: XmlParseError) -> DeleteAnalysisSchemeError {
        let XmlParseError(message) = err;
        DeleteAnalysisSchemeError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteAnalysisSchemeError {
    fn from(err: CredentialsError) -> DeleteAnalysisSchemeError {
        DeleteAnalysisSchemeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAnalysisSchemeError {
    fn from(err: HttpDispatchError) -> DeleteAnalysisSchemeError {
        DeleteAnalysisSchemeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAnalysisSchemeError {
    fn from(err: io::Error) -> DeleteAnalysisSchemeError {
        DeleteAnalysisSchemeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAnalysisSchemeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAnalysisSchemeError {
    fn description(&self) -> &str {
        match *self {
            DeleteAnalysisSchemeError::Base(ref cause) => cause,
            DeleteAnalysisSchemeError::Internal(ref cause) => cause,
            DeleteAnalysisSchemeError::InvalidType(ref cause) => cause,
            DeleteAnalysisSchemeError::ResourceNotFound(ref cause) => cause,
            DeleteAnalysisSchemeError::Validation(ref cause) => cause,
            DeleteAnalysisSchemeError::Credentials(ref err) => err.description(),
            DeleteAnalysisSchemeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAnalysisSchemeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDomain
#[derive(Debug, PartialEq)]
pub enum DeleteDomainError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDomainError {
    pub fn from_body(body: &str) -> DeleteDomainError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => DeleteDomainError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    DeleteDomainError::Internal(String::from(parsed_error.message))
                }
                _ => DeleteDomainError::Unknown(String::from(body)),
            },
            Err(_) => DeleteDomainError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteDomainError {
    fn from(err: XmlParseError) -> DeleteDomainError {
        let XmlParseError(message) = err;
        DeleteDomainError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteDomainError {
    fn from(err: CredentialsError) -> DeleteDomainError {
        DeleteDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDomainError {
    fn from(err: HttpDispatchError) -> DeleteDomainError {
        DeleteDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDomainError {
    fn from(err: io::Error) -> DeleteDomainError {
        DeleteDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDomainError {
    fn description(&self) -> &str {
        match *self {
            DeleteDomainError::Base(ref cause) => cause,
            DeleteDomainError::Internal(ref cause) => cause,
            DeleteDomainError::Validation(ref cause) => cause,
            DeleteDomainError::Credentials(ref err) => err.description(),
            DeleteDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteExpression
#[derive(Debug, PartialEq)]
pub enum DeleteExpressionError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteExpressionError {
    pub fn from_body(body: &str) -> DeleteExpressionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => DeleteExpressionError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    DeleteExpressionError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => {
                    DeleteExpressionError::InvalidType(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DeleteExpressionError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DeleteExpressionError::Unknown(String::from(body)),
            },
            Err(_) => DeleteExpressionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteExpressionError {
    fn from(err: XmlParseError) -> DeleteExpressionError {
        let XmlParseError(message) = err;
        DeleteExpressionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteExpressionError {
    fn from(err: CredentialsError) -> DeleteExpressionError {
        DeleteExpressionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteExpressionError {
    fn from(err: HttpDispatchError) -> DeleteExpressionError {
        DeleteExpressionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteExpressionError {
    fn from(err: io::Error) -> DeleteExpressionError {
        DeleteExpressionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteExpressionError {
    fn description(&self) -> &str {
        match *self {
            DeleteExpressionError::Base(ref cause) => cause,
            DeleteExpressionError::Internal(ref cause) => cause,
            DeleteExpressionError::InvalidType(ref cause) => cause,
            DeleteExpressionError::ResourceNotFound(ref cause) => cause,
            DeleteExpressionError::Validation(ref cause) => cause,
            DeleteExpressionError::Credentials(ref err) => err.description(),
            DeleteExpressionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteExpressionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIndexField
#[derive(Debug, PartialEq)]
pub enum DeleteIndexFieldError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteIndexFieldError {
    pub fn from_body(body: &str) -> DeleteIndexFieldError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => DeleteIndexFieldError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    DeleteIndexFieldError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => {
                    DeleteIndexFieldError::InvalidType(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DeleteIndexFieldError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DeleteIndexFieldError::Unknown(String::from(body)),
            },
            Err(_) => DeleteIndexFieldError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteIndexFieldError {
    fn from(err: XmlParseError) -> DeleteIndexFieldError {
        let XmlParseError(message) = err;
        DeleteIndexFieldError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteIndexFieldError {
    fn from(err: CredentialsError) -> DeleteIndexFieldError {
        DeleteIndexFieldError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIndexFieldError {
    fn from(err: HttpDispatchError) -> DeleteIndexFieldError {
        DeleteIndexFieldError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteIndexFieldError {
    fn from(err: io::Error) -> DeleteIndexFieldError {
        DeleteIndexFieldError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteIndexFieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIndexFieldError {
    fn description(&self) -> &str {
        match *self {
            DeleteIndexFieldError::Base(ref cause) => cause,
            DeleteIndexFieldError::Internal(ref cause) => cause,
            DeleteIndexFieldError::InvalidType(ref cause) => cause,
            DeleteIndexFieldError::ResourceNotFound(ref cause) => cause,
            DeleteIndexFieldError::Validation(ref cause) => cause,
            DeleteIndexFieldError::Credentials(ref err) => err.description(),
            DeleteIndexFieldError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteIndexFieldError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSuggester
#[derive(Debug, PartialEq)]
pub enum DeleteSuggesterError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSuggesterError {
    pub fn from_body(body: &str) -> DeleteSuggesterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => DeleteSuggesterError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    DeleteSuggesterError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => {
                    DeleteSuggesterError::InvalidType(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DeleteSuggesterError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DeleteSuggesterError::Unknown(String::from(body)),
            },
            Err(_) => DeleteSuggesterError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteSuggesterError {
    fn from(err: XmlParseError) -> DeleteSuggesterError {
        let XmlParseError(message) = err;
        DeleteSuggesterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteSuggesterError {
    fn from(err: CredentialsError) -> DeleteSuggesterError {
        DeleteSuggesterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSuggesterError {
    fn from(err: HttpDispatchError) -> DeleteSuggesterError {
        DeleteSuggesterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSuggesterError {
    fn from(err: io::Error) -> DeleteSuggesterError {
        DeleteSuggesterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSuggesterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSuggesterError {
    fn description(&self) -> &str {
        match *self {
            DeleteSuggesterError::Base(ref cause) => cause,
            DeleteSuggesterError::Internal(ref cause) => cause,
            DeleteSuggesterError::InvalidType(ref cause) => cause,
            DeleteSuggesterError::ResourceNotFound(ref cause) => cause,
            DeleteSuggesterError::Validation(ref cause) => cause,
            DeleteSuggesterError::Credentials(ref err) => err.description(),
            DeleteSuggesterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSuggesterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAnalysisSchemes
#[derive(Debug, PartialEq)]
pub enum DescribeAnalysisSchemesError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAnalysisSchemesError {
    pub fn from_body(body: &str) -> DescribeAnalysisSchemesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    DescribeAnalysisSchemesError::Base(String::from(parsed_error.message))
                }
                "InternalException" => {
                    DescribeAnalysisSchemesError::Internal(String::from(parsed_error.message))
                }
                "ResourceNotFound" => DescribeAnalysisSchemesError::ResourceNotFound(String::from(
                    parsed_error.message,
                )),
                _ => DescribeAnalysisSchemesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAnalysisSchemesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeAnalysisSchemesError {
    fn from(err: XmlParseError) -> DescribeAnalysisSchemesError {
        let XmlParseError(message) = err;
        DescribeAnalysisSchemesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAnalysisSchemesError {
    fn from(err: CredentialsError) -> DescribeAnalysisSchemesError {
        DescribeAnalysisSchemesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAnalysisSchemesError {
    fn from(err: HttpDispatchError) -> DescribeAnalysisSchemesError {
        DescribeAnalysisSchemesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAnalysisSchemesError {
    fn from(err: io::Error) -> DescribeAnalysisSchemesError {
        DescribeAnalysisSchemesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAnalysisSchemesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAnalysisSchemesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAnalysisSchemesError::Base(ref cause) => cause,
            DescribeAnalysisSchemesError::Internal(ref cause) => cause,
            DescribeAnalysisSchemesError::ResourceNotFound(ref cause) => cause,
            DescribeAnalysisSchemesError::Validation(ref cause) => cause,
            DescribeAnalysisSchemesError::Credentials(ref err) => err.description(),
            DescribeAnalysisSchemesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAnalysisSchemesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAvailabilityOptions
#[derive(Debug, PartialEq)]
pub enum DescribeAvailabilityOptionsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request was rejected because it attempted an operation which is not enabled.</p>
    DisabledOperation(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAvailabilityOptionsError {
    pub fn from_body(body: &str) -> DescribeAvailabilityOptionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    DescribeAvailabilityOptionsError::Base(String::from(parsed_error.message))
                }
                "DisabledAction" => DescribeAvailabilityOptionsError::DisabledOperation(
                    String::from(parsed_error.message),
                ),
                "InternalException" => {
                    DescribeAvailabilityOptionsError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => DescribeAvailabilityOptionsError::InvalidType(String::from(
                    parsed_error.message,
                )),
                "LimitExceeded" => DescribeAvailabilityOptionsError::LimitExceeded(String::from(
                    parsed_error.message,
                )),
                "ResourceNotFound" => DescribeAvailabilityOptionsError::ResourceNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DescribeAvailabilityOptionsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAvailabilityOptionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeAvailabilityOptionsError {
    fn from(err: XmlParseError) -> DescribeAvailabilityOptionsError {
        let XmlParseError(message) = err;
        DescribeAvailabilityOptionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAvailabilityOptionsError {
    fn from(err: CredentialsError) -> DescribeAvailabilityOptionsError {
        DescribeAvailabilityOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAvailabilityOptionsError {
    fn from(err: HttpDispatchError) -> DescribeAvailabilityOptionsError {
        DescribeAvailabilityOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAvailabilityOptionsError {
    fn from(err: io::Error) -> DescribeAvailabilityOptionsError {
        DescribeAvailabilityOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAvailabilityOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAvailabilityOptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAvailabilityOptionsError::Base(ref cause) => cause,
            DescribeAvailabilityOptionsError::DisabledOperation(ref cause) => cause,
            DescribeAvailabilityOptionsError::Internal(ref cause) => cause,
            DescribeAvailabilityOptionsError::InvalidType(ref cause) => cause,
            DescribeAvailabilityOptionsError::LimitExceeded(ref cause) => cause,
            DescribeAvailabilityOptionsError::ResourceNotFound(ref cause) => cause,
            DescribeAvailabilityOptionsError::Validation(ref cause) => cause,
            DescribeAvailabilityOptionsError::Credentials(ref err) => err.description(),
            DescribeAvailabilityOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAvailabilityOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDomains
#[derive(Debug, PartialEq)]
pub enum DescribeDomainsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeDomainsError {
    pub fn from_body(body: &str) -> DescribeDomainsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => DescribeDomainsError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    DescribeDomainsError::Internal(String::from(parsed_error.message))
                }
                _ => DescribeDomainsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeDomainsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDomainsError {
    fn from(err: XmlParseError) -> DescribeDomainsError {
        let XmlParseError(message) = err;
        DescribeDomainsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDomainsError {
    fn from(err: CredentialsError) -> DescribeDomainsError {
        DescribeDomainsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDomainsError {
    fn from(err: HttpDispatchError) -> DescribeDomainsError {
        DescribeDomainsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDomainsError {
    fn from(err: io::Error) -> DescribeDomainsError {
        DescribeDomainsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDomainsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDomainsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDomainsError::Base(ref cause) => cause,
            DescribeDomainsError::Internal(ref cause) => cause,
            DescribeDomainsError::Validation(ref cause) => cause,
            DescribeDomainsError::Credentials(ref err) => err.description(),
            DescribeDomainsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeDomainsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeExpressions
#[derive(Debug, PartialEq)]
pub enum DescribeExpressionsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeExpressionsError {
    pub fn from_body(body: &str) -> DescribeExpressionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    DescribeExpressionsError::Base(String::from(parsed_error.message))
                }
                "InternalException" => {
                    DescribeExpressionsError::Internal(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DescribeExpressionsError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DescribeExpressionsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeExpressionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeExpressionsError {
    fn from(err: XmlParseError) -> DescribeExpressionsError {
        let XmlParseError(message) = err;
        DescribeExpressionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeExpressionsError {
    fn from(err: CredentialsError) -> DescribeExpressionsError {
        DescribeExpressionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeExpressionsError {
    fn from(err: HttpDispatchError) -> DescribeExpressionsError {
        DescribeExpressionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeExpressionsError {
    fn from(err: io::Error) -> DescribeExpressionsError {
        DescribeExpressionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeExpressionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeExpressionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeExpressionsError::Base(ref cause) => cause,
            DescribeExpressionsError::Internal(ref cause) => cause,
            DescribeExpressionsError::ResourceNotFound(ref cause) => cause,
            DescribeExpressionsError::Validation(ref cause) => cause,
            DescribeExpressionsError::Credentials(ref err) => err.description(),
            DescribeExpressionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeExpressionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeIndexFields
#[derive(Debug, PartialEq)]
pub enum DescribeIndexFieldsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeIndexFieldsError {
    pub fn from_body(body: &str) -> DescribeIndexFieldsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    DescribeIndexFieldsError::Base(String::from(parsed_error.message))
                }
                "InternalException" => {
                    DescribeIndexFieldsError::Internal(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DescribeIndexFieldsError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DescribeIndexFieldsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeIndexFieldsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeIndexFieldsError {
    fn from(err: XmlParseError) -> DescribeIndexFieldsError {
        let XmlParseError(message) = err;
        DescribeIndexFieldsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeIndexFieldsError {
    fn from(err: CredentialsError) -> DescribeIndexFieldsError {
        DescribeIndexFieldsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeIndexFieldsError {
    fn from(err: HttpDispatchError) -> DescribeIndexFieldsError {
        DescribeIndexFieldsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeIndexFieldsError {
    fn from(err: io::Error) -> DescribeIndexFieldsError {
        DescribeIndexFieldsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeIndexFieldsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeIndexFieldsError {
    fn description(&self) -> &str {
        match *self {
            DescribeIndexFieldsError::Base(ref cause) => cause,
            DescribeIndexFieldsError::Internal(ref cause) => cause,
            DescribeIndexFieldsError::ResourceNotFound(ref cause) => cause,
            DescribeIndexFieldsError::Validation(ref cause) => cause,
            DescribeIndexFieldsError::Credentials(ref err) => err.description(),
            DescribeIndexFieldsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeIndexFieldsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeScalingParameters
#[derive(Debug, PartialEq)]
pub enum DescribeScalingParametersError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeScalingParametersError {
    pub fn from_body(body: &str) -> DescribeScalingParametersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    DescribeScalingParametersError::Base(String::from(parsed_error.message))
                }
                "InternalException" => {
                    DescribeScalingParametersError::Internal(String::from(parsed_error.message))
                }
                "ResourceNotFound" => DescribeScalingParametersError::ResourceNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DescribeScalingParametersError::Unknown(String::from(body)),
            },
            Err(_) => DescribeScalingParametersError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeScalingParametersError {
    fn from(err: XmlParseError) -> DescribeScalingParametersError {
        let XmlParseError(message) = err;
        DescribeScalingParametersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeScalingParametersError {
    fn from(err: CredentialsError) -> DescribeScalingParametersError {
        DescribeScalingParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScalingParametersError {
    fn from(err: HttpDispatchError) -> DescribeScalingParametersError {
        DescribeScalingParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScalingParametersError {
    fn from(err: io::Error) -> DescribeScalingParametersError {
        DescribeScalingParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScalingParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScalingParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeScalingParametersError::Base(ref cause) => cause,
            DescribeScalingParametersError::Internal(ref cause) => cause,
            DescribeScalingParametersError::ResourceNotFound(ref cause) => cause,
            DescribeScalingParametersError::Validation(ref cause) => cause,
            DescribeScalingParametersError::Credentials(ref err) => err.description(),
            DescribeScalingParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScalingParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeServiceAccessPolicies
#[derive(Debug, PartialEq)]
pub enum DescribeServiceAccessPoliciesError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeServiceAccessPoliciesError {
    pub fn from_body(body: &str) -> DescribeServiceAccessPoliciesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    DescribeServiceAccessPoliciesError::Base(String::from(parsed_error.message))
                }
                "InternalException" => {
                    DescribeServiceAccessPoliciesError::Internal(String::from(parsed_error.message))
                }
                "ResourceNotFound" => DescribeServiceAccessPoliciesError::ResourceNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DescribeServiceAccessPoliciesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeServiceAccessPoliciesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeServiceAccessPoliciesError {
    fn from(err: XmlParseError) -> DescribeServiceAccessPoliciesError {
        let XmlParseError(message) = err;
        DescribeServiceAccessPoliciesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeServiceAccessPoliciesError {
    fn from(err: CredentialsError) -> DescribeServiceAccessPoliciesError {
        DescribeServiceAccessPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeServiceAccessPoliciesError {
    fn from(err: HttpDispatchError) -> DescribeServiceAccessPoliciesError {
        DescribeServiceAccessPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeServiceAccessPoliciesError {
    fn from(err: io::Error) -> DescribeServiceAccessPoliciesError {
        DescribeServiceAccessPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeServiceAccessPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeServiceAccessPoliciesError {
    fn description(&self) -> &str {
        match *self {
            DescribeServiceAccessPoliciesError::Base(ref cause) => cause,
            DescribeServiceAccessPoliciesError::Internal(ref cause) => cause,
            DescribeServiceAccessPoliciesError::ResourceNotFound(ref cause) => cause,
            DescribeServiceAccessPoliciesError::Validation(ref cause) => cause,
            DescribeServiceAccessPoliciesError::Credentials(ref err) => err.description(),
            DescribeServiceAccessPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeServiceAccessPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSuggesters
#[derive(Debug, PartialEq)]
pub enum DescribeSuggestersError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSuggestersError {
    pub fn from_body(body: &str) -> DescribeSuggestersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    DescribeSuggestersError::Base(String::from(parsed_error.message))
                }
                "InternalException" => {
                    DescribeSuggestersError::Internal(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    DescribeSuggestersError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DescribeSuggestersError::Unknown(String::from(body)),
            },
            Err(_) => DescribeSuggestersError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeSuggestersError {
    fn from(err: XmlParseError) -> DescribeSuggestersError {
        let XmlParseError(message) = err;
        DescribeSuggestersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeSuggestersError {
    fn from(err: CredentialsError) -> DescribeSuggestersError {
        DescribeSuggestersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSuggestersError {
    fn from(err: HttpDispatchError) -> DescribeSuggestersError {
        DescribeSuggestersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSuggestersError {
    fn from(err: io::Error) -> DescribeSuggestersError {
        DescribeSuggestersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSuggestersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSuggestersError {
    fn description(&self) -> &str {
        match *self {
            DescribeSuggestersError::Base(ref cause) => cause,
            DescribeSuggestersError::Internal(ref cause) => cause,
            DescribeSuggestersError::ResourceNotFound(ref cause) => cause,
            DescribeSuggestersError::Validation(ref cause) => cause,
            DescribeSuggestersError::Credentials(ref err) => err.description(),
            DescribeSuggestersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSuggestersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by IndexDocuments
#[derive(Debug, PartialEq)]
pub enum IndexDocumentsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl IndexDocumentsError {
    pub fn from_body(body: &str) -> IndexDocumentsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => IndexDocumentsError::Base(String::from(parsed_error.message)),
                "InternalException" => {
                    IndexDocumentsError::Internal(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    IndexDocumentsError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => IndexDocumentsError::Unknown(String::from(body)),
            },
            Err(_) => IndexDocumentsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for IndexDocumentsError {
    fn from(err: XmlParseError) -> IndexDocumentsError {
        let XmlParseError(message) = err;
        IndexDocumentsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for IndexDocumentsError {
    fn from(err: CredentialsError) -> IndexDocumentsError {
        IndexDocumentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for IndexDocumentsError {
    fn from(err: HttpDispatchError) -> IndexDocumentsError {
        IndexDocumentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for IndexDocumentsError {
    fn from(err: io::Error) -> IndexDocumentsError {
        IndexDocumentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for IndexDocumentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IndexDocumentsError {
    fn description(&self) -> &str {
        match *self {
            IndexDocumentsError::Base(ref cause) => cause,
            IndexDocumentsError::Internal(ref cause) => cause,
            IndexDocumentsError::ResourceNotFound(ref cause) => cause,
            IndexDocumentsError::Validation(ref cause) => cause,
            IndexDocumentsError::Credentials(ref err) => err.description(),
            IndexDocumentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            IndexDocumentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDomainNames
#[derive(Debug, PartialEq)]
pub enum ListDomainNamesError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDomainNamesError {
    pub fn from_body(body: &str) -> ListDomainNamesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => ListDomainNamesError::Base(String::from(parsed_error.message)),
                _ => ListDomainNamesError::Unknown(String::from(body)),
            },
            Err(_) => ListDomainNamesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListDomainNamesError {
    fn from(err: XmlParseError) -> ListDomainNamesError {
        let XmlParseError(message) = err;
        ListDomainNamesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListDomainNamesError {
    fn from(err: CredentialsError) -> ListDomainNamesError {
        ListDomainNamesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDomainNamesError {
    fn from(err: HttpDispatchError) -> ListDomainNamesError {
        ListDomainNamesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDomainNamesError {
    fn from(err: io::Error) -> ListDomainNamesError {
        ListDomainNamesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDomainNamesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDomainNamesError {
    fn description(&self) -> &str {
        match *self {
            ListDomainNamesError::Base(ref cause) => cause,
            ListDomainNamesError::Validation(ref cause) => cause,
            ListDomainNamesError::Credentials(ref err) => err.description(),
            ListDomainNamesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDomainNamesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAvailabilityOptions
#[derive(Debug, PartialEq)]
pub enum UpdateAvailabilityOptionsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request was rejected because it attempted an operation which is not enabled.</p>
    DisabledOperation(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateAvailabilityOptionsError {
    pub fn from_body(body: &str) -> UpdateAvailabilityOptionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    UpdateAvailabilityOptionsError::Base(String::from(parsed_error.message))
                }
                "DisabledAction" => UpdateAvailabilityOptionsError::DisabledOperation(
                    String::from(parsed_error.message),
                ),
                "InternalException" => {
                    UpdateAvailabilityOptionsError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => {
                    UpdateAvailabilityOptionsError::InvalidType(String::from(parsed_error.message))
                }
                "LimitExceeded" => UpdateAvailabilityOptionsError::LimitExceeded(String::from(
                    parsed_error.message,
                )),
                "ResourceNotFound" => UpdateAvailabilityOptionsError::ResourceNotFound(
                    String::from(parsed_error.message),
                ),
                _ => UpdateAvailabilityOptionsError::Unknown(String::from(body)),
            },
            Err(_) => UpdateAvailabilityOptionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateAvailabilityOptionsError {
    fn from(err: XmlParseError) -> UpdateAvailabilityOptionsError {
        let XmlParseError(message) = err;
        UpdateAvailabilityOptionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateAvailabilityOptionsError {
    fn from(err: CredentialsError) -> UpdateAvailabilityOptionsError {
        UpdateAvailabilityOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAvailabilityOptionsError {
    fn from(err: HttpDispatchError) -> UpdateAvailabilityOptionsError {
        UpdateAvailabilityOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAvailabilityOptionsError {
    fn from(err: io::Error) -> UpdateAvailabilityOptionsError {
        UpdateAvailabilityOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAvailabilityOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAvailabilityOptionsError {
    fn description(&self) -> &str {
        match *self {
            UpdateAvailabilityOptionsError::Base(ref cause) => cause,
            UpdateAvailabilityOptionsError::DisabledOperation(ref cause) => cause,
            UpdateAvailabilityOptionsError::Internal(ref cause) => cause,
            UpdateAvailabilityOptionsError::InvalidType(ref cause) => cause,
            UpdateAvailabilityOptionsError::LimitExceeded(ref cause) => cause,
            UpdateAvailabilityOptionsError::ResourceNotFound(ref cause) => cause,
            UpdateAvailabilityOptionsError::Validation(ref cause) => cause,
            UpdateAvailabilityOptionsError::Credentials(ref err) => err.description(),
            UpdateAvailabilityOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateAvailabilityOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateScalingParameters
#[derive(Debug, PartialEq)]
pub enum UpdateScalingParametersError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateScalingParametersError {
    pub fn from_body(body: &str) -> UpdateScalingParametersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    UpdateScalingParametersError::Base(String::from(parsed_error.message))
                }
                "InternalException" => {
                    UpdateScalingParametersError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => {
                    UpdateScalingParametersError::InvalidType(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    UpdateScalingParametersError::LimitExceeded(String::from(parsed_error.message))
                }
                "ResourceNotFound" => UpdateScalingParametersError::ResourceNotFound(String::from(
                    parsed_error.message,
                )),
                _ => UpdateScalingParametersError::Unknown(String::from(body)),
            },
            Err(_) => UpdateScalingParametersError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateScalingParametersError {
    fn from(err: XmlParseError) -> UpdateScalingParametersError {
        let XmlParseError(message) = err;
        UpdateScalingParametersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateScalingParametersError {
    fn from(err: CredentialsError) -> UpdateScalingParametersError {
        UpdateScalingParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateScalingParametersError {
    fn from(err: HttpDispatchError) -> UpdateScalingParametersError {
        UpdateScalingParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateScalingParametersError {
    fn from(err: io::Error) -> UpdateScalingParametersError {
        UpdateScalingParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateScalingParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateScalingParametersError {
    fn description(&self) -> &str {
        match *self {
            UpdateScalingParametersError::Base(ref cause) => cause,
            UpdateScalingParametersError::Internal(ref cause) => cause,
            UpdateScalingParametersError::InvalidType(ref cause) => cause,
            UpdateScalingParametersError::LimitExceeded(ref cause) => cause,
            UpdateScalingParametersError::ResourceNotFound(ref cause) => cause,
            UpdateScalingParametersError::Validation(ref cause) => cause,
            UpdateScalingParametersError::Credentials(ref err) => err.description(),
            UpdateScalingParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateScalingParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateServiceAccessPolicies
#[derive(Debug, PartialEq)]
pub enum UpdateServiceAccessPoliciesError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it specified an invalid type definition.</p>
    InvalidType(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateServiceAccessPoliciesError {
    pub fn from_body(body: &str) -> UpdateServiceAccessPoliciesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BaseException" => {
                    UpdateServiceAccessPoliciesError::Base(String::from(parsed_error.message))
                }
                "InternalException" => {
                    UpdateServiceAccessPoliciesError::Internal(String::from(parsed_error.message))
                }
                "InvalidType" => UpdateServiceAccessPoliciesError::InvalidType(String::from(
                    parsed_error.message,
                )),
                "LimitExceeded" => UpdateServiceAccessPoliciesError::LimitExceeded(String::from(
                    parsed_error.message,
                )),
                "ResourceNotFound" => UpdateServiceAccessPoliciesError::ResourceNotFound(
                    String::from(parsed_error.message),
                ),
                _ => UpdateServiceAccessPoliciesError::Unknown(String::from(body)),
            },
            Err(_) => UpdateServiceAccessPoliciesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateServiceAccessPoliciesError {
    fn from(err: XmlParseError) -> UpdateServiceAccessPoliciesError {
        let XmlParseError(message) = err;
        UpdateServiceAccessPoliciesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateServiceAccessPoliciesError {
    fn from(err: CredentialsError) -> UpdateServiceAccessPoliciesError {
        UpdateServiceAccessPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateServiceAccessPoliciesError {
    fn from(err: HttpDispatchError) -> UpdateServiceAccessPoliciesError {
        UpdateServiceAccessPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateServiceAccessPoliciesError {
    fn from(err: io::Error) -> UpdateServiceAccessPoliciesError {
        UpdateServiceAccessPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateServiceAccessPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateServiceAccessPoliciesError {
    fn description(&self) -> &str {
        match *self {
            UpdateServiceAccessPoliciesError::Base(ref cause) => cause,
            UpdateServiceAccessPoliciesError::Internal(ref cause) => cause,
            UpdateServiceAccessPoliciesError::InvalidType(ref cause) => cause,
            UpdateServiceAccessPoliciesError::LimitExceeded(ref cause) => cause,
            UpdateServiceAccessPoliciesError::ResourceNotFound(ref cause) => cause,
            UpdateServiceAccessPoliciesError::Validation(ref cause) => cause,
            UpdateServiceAccessPoliciesError::Credentials(ref err) => err.description(),
            UpdateServiceAccessPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateServiceAccessPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon CloudSearch API. Amazon CloudSearch clients implement this trait.
pub trait CloudSearch {
    /// <p>Indexes the search suggestions. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html#configuring-suggesters">Configuring Suggesters</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn build_suggesters(
        &self,
        input: BuildSuggestersRequest,
    ) -> RusotoFuture<BuildSuggestersResponse, BuildSuggestersError>;

    /// <p>Creates a new search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/creating-domains.html" target="_blank">Creating a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> RusotoFuture<CreateDomainResponse, CreateDomainError>;

    /// <p>Configures an analysis scheme that can be applied to a <code>text</code> or <code>text-array</code> field to define language-specific text processing options. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn define_analysis_scheme(
        &self,
        input: DefineAnalysisSchemeRequest,
    ) -> RusotoFuture<DefineAnalysisSchemeResponse, DefineAnalysisSchemeError>;

    /// <p>Configures an <code><a>Expression</a></code> for the search domain. Used to create new expressions and modify existing ones. If the expression exists, the new configuration replaces the old one. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn define_expression(
        &self,
        input: DefineExpressionRequest,
    ) -> RusotoFuture<DefineExpressionResponse, DefineExpressionError>;

    /// <p>Configures an <code><a>IndexField</a></code> for the search domain. Used to create new fields and modify existing ones. You must specify the name of the domain you are configuring and an index field configuration. The index field configuration specifies a unique name, the index field type, and the options you want to configure for the field. The options you can specify depend on the <code><a>IndexFieldType</a></code>. If the field exists, the new configuration replaces the old one. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    fn define_index_field(
        &self,
        input: DefineIndexFieldRequest,
    ) -> RusotoFuture<DefineIndexFieldResponse, DefineIndexFieldError>;

    /// <p>Configures a suggester for a domain. A suggester enables you to display possible matches before users finish typing their queries. When you configure a suggester, you must specify the name of the text field you want to search for possible matches and a unique name for the suggester. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn define_suggester(
        &self,
        input: DefineSuggesterRequest,
    ) -> RusotoFuture<DefineSuggesterResponse, DefineSuggesterError>;

    /// <p>Deletes an analysis scheme. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    fn delete_analysis_scheme(
        &self,
        input: DeleteAnalysisSchemeRequest,
    ) -> RusotoFuture<DeleteAnalysisSchemeResponse, DeleteAnalysisSchemeError>;

    /// <p>Permanently deletes a search domain and all of its data. Once a domain has been deleted, it cannot be recovered. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/deleting-domains.html" target="_blank">Deleting a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> RusotoFuture<DeleteDomainResponse, DeleteDomainError>;

    /// <p>Removes an <code><a>Expression</a></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn delete_expression(
        &self,
        input: DeleteExpressionRequest,
    ) -> RusotoFuture<DeleteExpressionResponse, DeleteExpressionError>;

    /// <p>Removes an <code><a>IndexField</a></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn delete_index_field(
        &self,
        input: DeleteIndexFieldRequest,
    ) -> RusotoFuture<DeleteIndexFieldResponse, DeleteIndexFieldError>;

    /// <p>Deletes a suggester. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn delete_suggester(
        &self,
        input: DeleteSuggesterRequest,
    ) -> RusotoFuture<DeleteSuggesterResponse, DeleteSuggesterError>;

    /// <p>Gets the analysis schemes configured for a domain. An analysis scheme defines language-specific text processing options for a <code>text</code> field. Can be limited to specific analysis schemes by name. By default, shows all analysis schemes and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_analysis_schemes(
        &self,
        input: DescribeAnalysisSchemesRequest,
    ) -> RusotoFuture<DescribeAnalysisSchemesResponse, DescribeAnalysisSchemesError>;

    /// <p>Gets the availability options configured for a domain. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-availability-options.html" target="_blank">Configuring Availability Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_availability_options(
        &self,
        input: DescribeAvailabilityOptionsRequest,
    ) -> RusotoFuture<DescribeAvailabilityOptionsResponse, DescribeAvailabilityOptionsError>;

    /// <p>Gets information about the search domains owned by this account. Can be limited to specific domains. Shows all domains by default. To get the number of searchable documents in a domain, use the console or submit a <code>matchall</code> request to your domain's search endpoint: <code>q=matchall&amp;amp;q.parser=structured&amp;amp;size=0</code>. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Information about a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_domains(
        &self,
        input: DescribeDomainsRequest,
    ) -> RusotoFuture<DescribeDomainsResponse, DescribeDomainsError>;

    /// <p>Gets the expressions configured for the search domain. Can be limited to specific expressions by name. By default, shows all expressions and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_expressions(
        &self,
        input: DescribeExpressionsRequest,
    ) -> RusotoFuture<DescribeExpressionsResponse, DescribeExpressionsError>;

    /// <p>Gets information about the index fields configured for the search domain. Can be limited to specific fields by name. By default, shows all fields and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Domain Information</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_index_fields(
        &self,
        input: DescribeIndexFieldsRequest,
    ) -> RusotoFuture<DescribeIndexFieldsResponse, DescribeIndexFieldsError>;

    /// <p>Gets the scaling parameters configured for a domain. A domain's scaling parameters specify the desired search instance type and replication count. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-scaling-options.html" target="_blank">Configuring Scaling Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_scaling_parameters(
        &self,
        input: DescribeScalingParametersRequest,
    ) -> RusotoFuture<DescribeScalingParametersResponse, DescribeScalingParametersError>;

    /// <p>Gets information about the access policies that control access to the domain's document and search endpoints. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank">Configuring Access for a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_service_access_policies(
        &self,
        input: DescribeServiceAccessPoliciesRequest,
    ) -> RusotoFuture<DescribeServiceAccessPoliciesResponse, DescribeServiceAccessPoliciesError>;

    /// <p>Gets the suggesters configured for a domain. A suggester enables you to display possible matches before users finish typing their queries. Can be limited to specific suggesters by name. By default, shows all suggesters and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_suggesters(
        &self,
        input: DescribeSuggestersRequest,
    ) -> RusotoFuture<DescribeSuggestersResponse, DescribeSuggestersError>;

    /// <p>Tells the search domain to start indexing its documents using the latest indexing options. This operation must be invoked to activate options whose <a>OptionStatus</a> is <code>RequiresIndexDocuments</code>.</p>
    fn index_documents(
        &self,
        input: IndexDocumentsRequest,
    ) -> RusotoFuture<IndexDocumentsResponse, IndexDocumentsError>;

    /// <p>Lists all search domains owned by an account.</p>
    fn list_domain_names(&self) -> RusotoFuture<ListDomainNamesResponse, ListDomainNamesError>;

    /// <p>Configures the availability options for a domain. Enabling the Multi-AZ option expands an Amazon CloudSearch domain to an additional Availability Zone in the same Region to increase fault tolerance in the event of a service disruption. Changes to the Multi-AZ option can take about half an hour to become active. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-availability-options.html" target="_blank">Configuring Availability Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn update_availability_options(
        &self,
        input: UpdateAvailabilityOptionsRequest,
    ) -> RusotoFuture<UpdateAvailabilityOptionsResponse, UpdateAvailabilityOptionsError>;

    /// <p>Configures scaling parameters for a domain. A domain's scaling parameters specify the desired search instance type and replication count. Amazon CloudSearch will still automatically scale your domain based on the volume of data and traffic, but not below the desired instance type and replication count. If the Multi-AZ option is enabled, these values control the resources used per Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-scaling-options.html" target="_blank">Configuring Scaling Options</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    fn update_scaling_parameters(
        &self,
        input: UpdateScalingParametersRequest,
    ) -> RusotoFuture<UpdateScalingParametersResponse, UpdateScalingParametersError>;

    /// <p>Configures the access rules that control access to the domain's document and search endpoints. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank"> Configuring Access for an Amazon CloudSearch Domain</a>.</p>
    fn update_service_access_policies(
        &self,
        input: UpdateServiceAccessPoliciesRequest,
    ) -> RusotoFuture<UpdateServiceAccessPoliciesResponse, UpdateServiceAccessPoliciesError>;
}
/// A client for the Amazon CloudSearch API.
pub struct CloudSearchClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl CloudSearchClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> CloudSearchClient {
        CloudSearchClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> CloudSearchClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudSearchClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> CloudSearch for CloudSearchClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Indexes the search suggestions. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html#configuring-suggesters">Configuring Suggesters</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn build_suggesters(
        &self,
        input: BuildSuggestersRequest,
    ) -> RusotoFuture<BuildSuggestersResponse, BuildSuggestersError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "BuildSuggesters");
        params.put("Version", "2013-01-01");
        BuildSuggestersRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BuildSuggestersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = BuildSuggestersResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(BuildSuggestersResponseDeserializer::deserialize(
                        "BuildSuggestersResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/creating-domains.html" target="_blank">Creating a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> RusotoFuture<CreateDomainResponse, CreateDomainError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDomain");
        params.put("Version", "2013-01-01");
        CreateDomainRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateDomainResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateDomainResponseDeserializer::deserialize(
                        "CreateDomainResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Configures an analysis scheme that can be applied to a <code>text</code> or <code>text-array</code> field to define language-specific text processing options. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn define_analysis_scheme(
        &self,
        input: DefineAnalysisSchemeRequest,
    ) -> RusotoFuture<DefineAnalysisSchemeResponse, DefineAnalysisSchemeError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DefineAnalysisScheme");
        params.put("Version", "2013-01-01");
        DefineAnalysisSchemeRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DefineAnalysisSchemeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DefineAnalysisSchemeResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DefineAnalysisSchemeResponseDeserializer::deserialize(
                        "DefineAnalysisSchemeResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Configures an <code><a>Expression</a></code> for the search domain. Used to create new expressions and modify existing ones. If the expression exists, the new configuration replaces the old one. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn define_expression(
        &self,
        input: DefineExpressionRequest,
    ) -> RusotoFuture<DefineExpressionResponse, DefineExpressionError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DefineExpression");
        params.put("Version", "2013-01-01");
        DefineExpressionRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DefineExpressionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DefineExpressionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DefineExpressionResponseDeserializer::deserialize(
                        "DefineExpressionResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Configures an <code><a>IndexField</a></code> for the search domain. Used to create new fields and modify existing ones. You must specify the name of the domain you are configuring and an index field configuration. The index field configuration specifies a unique name, the index field type, and the options you want to configure for the field. The options you can specify depend on the <code><a>IndexFieldType</a></code>. If the field exists, the new configuration replaces the old one. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    fn define_index_field(
        &self,
        input: DefineIndexFieldRequest,
    ) -> RusotoFuture<DefineIndexFieldResponse, DefineIndexFieldError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DefineIndexField");
        params.put("Version", "2013-01-01");
        DefineIndexFieldRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DefineIndexFieldError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DefineIndexFieldResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DefineIndexFieldResponseDeserializer::deserialize(
                        "DefineIndexFieldResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Configures a suggester for a domain. A suggester enables you to display possible matches before users finish typing their queries. When you configure a suggester, you must specify the name of the text field you want to search for possible matches and a unique name for the suggester. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn define_suggester(
        &self,
        input: DefineSuggesterRequest,
    ) -> RusotoFuture<DefineSuggesterResponse, DefineSuggesterError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DefineSuggester");
        params.put("Version", "2013-01-01");
        DefineSuggesterRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DefineSuggesterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DefineSuggesterResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DefineSuggesterResponseDeserializer::deserialize(
                        "DefineSuggesterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an analysis scheme. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    fn delete_analysis_scheme(
        &self,
        input: DeleteAnalysisSchemeRequest,
    ) -> RusotoFuture<DeleteAnalysisSchemeResponse, DeleteAnalysisSchemeError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteAnalysisScheme");
        params.put("Version", "2013-01-01");
        DeleteAnalysisSchemeRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAnalysisSchemeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteAnalysisSchemeResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteAnalysisSchemeResponseDeserializer::deserialize(
                        "DeleteAnalysisSchemeResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Permanently deletes a search domain and all of its data. Once a domain has been deleted, it cannot be recovered. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/deleting-domains.html" target="_blank">Deleting a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> RusotoFuture<DeleteDomainResponse, DeleteDomainError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDomain");
        params.put("Version", "2013-01-01");
        DeleteDomainRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteDomainResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteDomainResponseDeserializer::deserialize(
                        "DeleteDomainResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes an <code><a>Expression</a></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn delete_expression(
        &self,
        input: DeleteExpressionRequest,
    ) -> RusotoFuture<DeleteExpressionResponse, DeleteExpressionError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteExpression");
        params.put("Version", "2013-01-01");
        DeleteExpressionRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteExpressionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteExpressionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteExpressionResponseDeserializer::deserialize(
                        "DeleteExpressionResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes an <code><a>IndexField</a></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn delete_index_field(
        &self,
        input: DeleteIndexFieldRequest,
    ) -> RusotoFuture<DeleteIndexFieldResponse, DeleteIndexFieldError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteIndexField");
        params.put("Version", "2013-01-01");
        DeleteIndexFieldRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteIndexFieldError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteIndexFieldResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteIndexFieldResponseDeserializer::deserialize(
                        "DeleteIndexFieldResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a suggester. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn delete_suggester(
        &self,
        input: DeleteSuggesterRequest,
    ) -> RusotoFuture<DeleteSuggesterResponse, DeleteSuggesterError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteSuggester");
        params.put("Version", "2013-01-01");
        DeleteSuggesterRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSuggesterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteSuggesterResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteSuggesterResponseDeserializer::deserialize(
                        "DeleteSuggesterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the analysis schemes configured for a domain. An analysis scheme defines language-specific text processing options for a <code>text</code> field. Can be limited to specific analysis schemes by name. By default, shows all analysis schemes and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_analysis_schemes(
        &self,
        input: DescribeAnalysisSchemesRequest,
    ) -> RusotoFuture<DescribeAnalysisSchemesResponse, DescribeAnalysisSchemesError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAnalysisSchemes");
        params.put("Version", "2013-01-01");
        DescribeAnalysisSchemesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAnalysisSchemesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAnalysisSchemesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAnalysisSchemesResponseDeserializer::deserialize(
                        "DescribeAnalysisSchemesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the availability options configured for a domain. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-availability-options.html" target="_blank">Configuring Availability Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_availability_options(
        &self,
        input: DescribeAvailabilityOptionsRequest,
    ) -> RusotoFuture<DescribeAvailabilityOptionsResponse, DescribeAvailabilityOptionsError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAvailabilityOptions");
        params.put("Version", "2013-01-01");
        DescribeAvailabilityOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAvailabilityOptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAvailabilityOptionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DescribeAvailabilityOptionsResponseDeserializer::deserialize(
                            "DescribeAvailabilityOptionsResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the search domains owned by this account. Can be limited to specific domains. Shows all domains by default. To get the number of searchable documents in a domain, use the console or submit a <code>matchall</code> request to your domain's search endpoint: <code>q=matchall&amp;amp;q.parser=structured&amp;amp;size=0</code>. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Information about a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_domains(
        &self,
        input: DescribeDomainsRequest,
    ) -> RusotoFuture<DescribeDomainsResponse, DescribeDomainsError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDomains");
        params.put("Version", "2013-01-01");
        DescribeDomainsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDomainsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeDomainsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeDomainsResponseDeserializer::deserialize(
                        "DescribeDomainsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the expressions configured for the search domain. Can be limited to specific expressions by name. By default, shows all expressions and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_expressions(
        &self,
        input: DescribeExpressionsRequest,
    ) -> RusotoFuture<DescribeExpressionsResponse, DescribeExpressionsError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeExpressions");
        params.put("Version", "2013-01-01");
        DescribeExpressionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeExpressionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeExpressionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeExpressionsResponseDeserializer::deserialize(
                        "DescribeExpressionsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the index fields configured for the search domain. Can be limited to specific fields by name. By default, shows all fields and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Domain Information</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_index_fields(
        &self,
        input: DescribeIndexFieldsRequest,
    ) -> RusotoFuture<DescribeIndexFieldsResponse, DescribeIndexFieldsError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeIndexFields");
        params.put("Version", "2013-01-01");
        DescribeIndexFieldsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeIndexFieldsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeIndexFieldsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeIndexFieldsResponseDeserializer::deserialize(
                        "DescribeIndexFieldsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the scaling parameters configured for a domain. A domain's scaling parameters specify the desired search instance type and replication count. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-scaling-options.html" target="_blank">Configuring Scaling Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_scaling_parameters(
        &self,
        input: DescribeScalingParametersRequest,
    ) -> RusotoFuture<DescribeScalingParametersResponse, DescribeScalingParametersError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeScalingParameters");
        params.put("Version", "2013-01-01");
        DescribeScalingParametersRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeScalingParametersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeScalingParametersResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeScalingParametersResponseDeserializer::deserialize(
                        "DescribeScalingParametersResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the access policies that control access to the domain's document and search endpoints. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank">Configuring Access for a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_service_access_policies(
        &self,
        input: DescribeServiceAccessPoliciesRequest,
    ) -> RusotoFuture<DescribeServiceAccessPoliciesResponse, DescribeServiceAccessPoliciesError>
    {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeServiceAccessPolicies");
        params.put("Version", "2013-01-01");
        DescribeServiceAccessPoliciesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeServiceAccessPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeServiceAccessPoliciesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DescribeServiceAccessPoliciesResponseDeserializer::deserialize(
                            "DescribeServiceAccessPoliciesResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the suggesters configured for a domain. A suggester enables you to display possible matches before users finish typing their queries. Can be limited to specific suggesters by name. By default, shows all suggesters and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn describe_suggesters(
        &self,
        input: DescribeSuggestersRequest,
    ) -> RusotoFuture<DescribeSuggestersResponse, DescribeSuggestersError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeSuggesters");
        params.put("Version", "2013-01-01");
        DescribeSuggestersRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSuggestersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeSuggestersResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeSuggestersResponseDeserializer::deserialize(
                        "DescribeSuggestersResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Tells the search domain to start indexing its documents using the latest indexing options. This operation must be invoked to activate options whose <a>OptionStatus</a> is <code>RequiresIndexDocuments</code>.</p>
    fn index_documents(
        &self,
        input: IndexDocumentsRequest,
    ) -> RusotoFuture<IndexDocumentsResponse, IndexDocumentsError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "IndexDocuments");
        params.put("Version", "2013-01-01");
        IndexDocumentsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(IndexDocumentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = IndexDocumentsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(IndexDocumentsResponseDeserializer::deserialize(
                        "IndexDocumentsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all search domains owned by an account.</p>
    fn list_domain_names(&self) -> RusotoFuture<ListDomainNamesResponse, ListDomainNamesError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListDomainNames");
        params.put("Version", "2013-01-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListDomainNamesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListDomainNamesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListDomainNamesResponseDeserializer::deserialize(
                        "ListDomainNamesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Configures the availability options for a domain. Enabling the Multi-AZ option expands an Amazon CloudSearch domain to an additional Availability Zone in the same Region to increase fault tolerance in the event of a service disruption. Changes to the Multi-AZ option can take about half an hour to become active. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-availability-options.html" target="_blank">Configuring Availability Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    fn update_availability_options(
        &self,
        input: UpdateAvailabilityOptionsRequest,
    ) -> RusotoFuture<UpdateAvailabilityOptionsResponse, UpdateAvailabilityOptionsError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateAvailabilityOptions");
        params.put("Version", "2013-01-01");
        UpdateAvailabilityOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAvailabilityOptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateAvailabilityOptionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(UpdateAvailabilityOptionsResponseDeserializer::deserialize(
                        "UpdateAvailabilityOptionsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Configures scaling parameters for a domain. A domain's scaling parameters specify the desired search instance type and replication count. Amazon CloudSearch will still automatically scale your domain based on the volume of data and traffic, but not below the desired instance type and replication count. If the Multi-AZ option is enabled, these values control the resources used per Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-scaling-options.html" target="_blank">Configuring Scaling Options</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    fn update_scaling_parameters(
        &self,
        input: UpdateScalingParametersRequest,
    ) -> RusotoFuture<UpdateScalingParametersResponse, UpdateScalingParametersError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateScalingParameters");
        params.put("Version", "2013-01-01");
        UpdateScalingParametersRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateScalingParametersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateScalingParametersResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(UpdateScalingParametersResponseDeserializer::deserialize(
                        "UpdateScalingParametersResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Configures the access rules that control access to the domain's document and search endpoints. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank"> Configuring Access for an Amazon CloudSearch Domain</a>.</p>
    fn update_service_access_policies(
        &self,
        input: UpdateServiceAccessPoliciesRequest,
    ) -> RusotoFuture<UpdateServiceAccessPoliciesResponse, UpdateServiceAccessPoliciesError> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateServiceAccessPolicies");
        params.put("Version", "2013-01-01");
        UpdateServiceAccessPoliciesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateServiceAccessPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateServiceAccessPoliciesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        UpdateServiceAccessPoliciesResponseDeserializer::deserialize(
                            "UpdateServiceAccessPoliciesResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
