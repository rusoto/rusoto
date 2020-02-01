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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    characters, deserialize_elements, end_element, find_start_element, peek_at_name, skip_tree,
    start_element,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::signature::SignedRequest;
use serde_urlencoded;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::EventReader;

struct APIVersionDeserializer;
impl APIVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ARNDeserializer;
impl ARNDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The configured access rules for the domain's document and search endpoints, and the current status of those rules.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AccessPoliciesStatus {
    pub options: String,
    pub status: OptionStatus,
}

struct AccessPoliciesStatusDeserializer;
impl AccessPoliciesStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccessPoliciesStatus, XmlParseError> {
        deserialize_elements::<_, AccessPoliciesStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Options" => {
                    obj.options = PolicyDocumentDeserializer::deserialize("Options", stack)?;
                }
                "Status" => {
                    obj.status = OptionStatusDeserializer::deserialize("Status", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AlgorithmicStemmingDeserializer;
impl AlgorithmicStemmingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Synonyms, stopwords, and stemming options for an analysis scheme. Includes tokenization dictionary for Japanese.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalysisOptions, XmlParseError> {
        deserialize_elements::<_, AnalysisOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AlgorithmicStemming" => {
                    obj.algorithmic_stemming = Some(AlgorithmicStemmingDeserializer::deserialize(
                        "AlgorithmicStemming",
                        stack,
                    )?);
                }
                "JapaneseTokenizationDictionary" => {
                    obj.japanese_tokenization_dictionary = Some(StringDeserializer::deserialize(
                        "JapaneseTokenizationDictionary",
                        stack,
                    )?);
                }
                "StemmingDictionary" => {
                    obj.stemming_dictionary = Some(StringDeserializer::deserialize(
                        "StemmingDictionary",
                        stack,
                    )?);
                }
                "Stopwords" => {
                    obj.stopwords = Some(StringDeserializer::deserialize("Stopwords", stack)?);
                }
                "Synonyms" => {
                    obj.synonyms = Some(StringDeserializer::deserialize("Synonyms", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.japanese_tokenization_dictionary {
            params.put(
                &format!("{}{}", prefix, "JapaneseTokenizationDictionary"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.stemming_dictionary {
            params.put(&format!("{}{}", prefix, "StemmingDictionary"), &field_value);
        }
        if let Some(ref field_value) = obj.stopwords {
            params.put(&format!("{}{}", prefix, "Stopwords"), &field_value);
        }
        if let Some(ref field_value) = obj.synonyms {
            params.put(&format!("{}{}", prefix, "Synonyms"), &field_value);
        }
    }
}

/// <p>Configuration information for an analysis scheme. Each analysis scheme has a unique name and specifies the language of the text to be processed. The following options can be configured for an analysis scheme: <code>Synonyms</code>, <code>Stopwords</code>, <code>StemmingDictionary</code>, <code>JapaneseTokenizationDictionary</code> and <code>AlgorithmicStemming</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AnalysisScheme {
    pub analysis_options: Option<AnalysisOptions>,
    pub analysis_scheme_language: String,
    pub analysis_scheme_name: String,
}

struct AnalysisSchemeDeserializer;
impl AnalysisSchemeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalysisScheme, XmlParseError> {
        deserialize_elements::<_, AnalysisScheme, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AnalysisOptions" => {
                    obj.analysis_options = Some(AnalysisOptionsDeserializer::deserialize(
                        "AnalysisOptions",
                        stack,
                    )?);
                }
                "AnalysisSchemeLanguage" => {
                    obj.analysis_scheme_language = AnalysisSchemeLanguageDeserializer::deserialize(
                        "AnalysisSchemeLanguage",
                        stack,
                    )?;
                }
                "AnalysisSchemeName" => {
                    obj.analysis_scheme_name =
                        StandardNameDeserializer::deserialize("AnalysisSchemeName", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            &obj.analysis_scheme_language,
        );
        params.put(
            &format!("{}{}", prefix, "AnalysisSchemeName"),
            &obj.analysis_scheme_name,
        );
    }
}

struct AnalysisSchemeLanguageDeserializer;
impl AnalysisSchemeLanguageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The status and configuration of an <code>AnalysisScheme</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AnalysisSchemeStatus {
    pub options: AnalysisScheme,
    pub status: OptionStatus,
}

struct AnalysisSchemeStatusDeserializer;
impl AnalysisSchemeStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalysisSchemeStatus, XmlParseError> {
        deserialize_elements::<_, AnalysisSchemeStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Options" => {
                    obj.options = AnalysisSchemeDeserializer::deserialize("Options", stack)?;
                }
                "Status" => {
                    obj.status = OptionStatusDeserializer::deserialize("Status", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AnalysisSchemeStatusListDeserializer;
impl AnalysisSchemeStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AnalysisSchemeStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AnalysisSchemeStatusDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The status and configuration of the domain's availability options.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AvailabilityOptionsStatus {
    /// <p>The availability options configured for the domain.</p>
    pub options: bool,
    pub status: OptionStatus,
}

struct AvailabilityOptionsStatusDeserializer;
impl AvailabilityOptionsStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AvailabilityOptionsStatus, XmlParseError> {
        deserialize_elements::<_, AvailabilityOptionsStatus, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Options" => {
                        obj.options = MultiAZDeserializer::deserialize("Options", stack)?;
                    }
                    "Status" => {
                        obj.status = OptionStatusDeserializer::deserialize("Status", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>BuildSuggester</a></code> operation. Specifies the name of the domain you want to update.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code>BuildSuggester</code> request. Contains a list of the fields used for suggestions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct BuildSuggestersResponse {
    pub field_names: Option<Vec<String>>,
}

struct BuildSuggestersResponseDeserializer;
impl BuildSuggestersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BuildSuggestersResponse, XmlParseError> {
        deserialize_elements::<_, BuildSuggestersResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "FieldNames" => {
                        obj.field_names
                            .get_or_insert(vec![])
                            .extend(FieldNameListDeserializer::deserialize("FieldNames", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>CreateDomain</a></code> operation. Specifies a name for the new search domain.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code>CreateDomainRequest</code>. Contains the status of a newly created domain.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDomainResponse {
    pub domain_status: Option<DomainStatus>,
}

struct CreateDomainResponseDeserializer;
impl CreateDomainResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDomainResponse, XmlParseError> {
        deserialize_elements::<_, CreateDomainResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DomainStatus" => {
                    obj.domain_status = Some(DomainStatusDeserializer::deserialize(
                        "DomainStatus",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Options for a field that contains an array of dates. Present if <code>IndexFieldType</code> specifies the field is of type <code>date-array</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DateArrayOptions, XmlParseError> {
        deserialize_elements::<_, DateArrayOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value =
                        Some(FieldValueDeserializer::deserialize("DefaultValue", stack)?);
                }
                "FacetEnabled" => {
                    obj.facet_enabled =
                        Some(BooleanDeserializer::deserialize("FacetEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SearchEnabled" => {
                    obj.search_enabled =
                        Some(BooleanDeserializer::deserialize("SearchEnabled", stack)?);
                }
                "SourceFields" => {
                    obj.source_fields = Some(FieldNameCommaListDeserializer::deserialize(
                        "SourceFields",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(&format!("{}{}", prefix, "FacetEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(&format!("{}{}", prefix, "SearchEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_fields {
            params.put(&format!("{}{}", prefix, "SourceFields"), &field_value);
        }
    }
}

/// <p>Options for a date field. Dates and times are specified in UTC (Coordinated Universal Time) according to IETF RFC3339: yyyy-mm-ddT00:00:00Z. Present if <code>IndexFieldType</code> specifies the field is of type <code>date</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DateOptions, XmlParseError> {
        deserialize_elements::<_, DateOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value =
                        Some(FieldValueDeserializer::deserialize("DefaultValue", stack)?);
                }
                "FacetEnabled" => {
                    obj.facet_enabled =
                        Some(BooleanDeserializer::deserialize("FacetEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SearchEnabled" => {
                    obj.search_enabled =
                        Some(BooleanDeserializer::deserialize("SearchEnabled", stack)?);
                }
                "SortEnabled" => {
                    obj.sort_enabled =
                        Some(BooleanDeserializer::deserialize("SortEnabled", stack)?);
                }
                "SourceField" => {
                    obj.source_field =
                        Some(FieldNameDeserializer::deserialize("SourceField", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(&format!("{}{}", prefix, "FacetEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(&format!("{}{}", prefix, "SearchEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(&format!("{}{}", prefix, "SortEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(&format!("{}{}", prefix, "SourceField"), &field_value);
        }
    }
}

/// <p>Container for the parameters to the <code><a>DefineAnalysisScheme</a></code> operation. Specifies the name of the domain you want to update and the analysis scheme configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code><a>DefineAnalysisScheme</a></code> request. Contains the status of the newly-configured analysis scheme.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DefineAnalysisSchemeResponse {
    pub analysis_scheme: AnalysisSchemeStatus,
}

struct DefineAnalysisSchemeResponseDeserializer;
impl DefineAnalysisSchemeResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefineAnalysisSchemeResponse, XmlParseError> {
        deserialize_elements::<_, DefineAnalysisSchemeResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AnalysisScheme" => {
                        obj.analysis_scheme =
                            AnalysisSchemeStatusDeserializer::deserialize("AnalysisScheme", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DefineExpression</a></code> operation. Specifies the name of the domain you want to update and the expression you want to configure.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        ExpressionSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Expression"),
            &obj.expression,
        );
    }
}

/// <p>The result of a <code>DefineExpression</code> request. Contains the status of the newly-configured expression.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DefineExpressionResponse {
    pub expression: ExpressionStatus,
}

struct DefineExpressionResponseDeserializer;
impl DefineExpressionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefineExpressionResponse, XmlParseError> {
        deserialize_elements::<_, DefineExpressionResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Expression" => {
                        obj.expression =
                            ExpressionStatusDeserializer::deserialize("Expression", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DefineIndexField</a></code> operation. Specifies the name of the domain you want to update and the index field configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        IndexFieldSerializer::serialize(
            params,
            &format!("{}{}", prefix, "IndexField"),
            &obj.index_field,
        );
    }
}

/// <p>The result of a <code><a>DefineIndexField</a></code> request. Contains the status of the newly-configured index field.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DefineIndexFieldResponse {
    pub index_field: IndexFieldStatus,
}

struct DefineIndexFieldResponseDeserializer;
impl DefineIndexFieldResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefineIndexFieldResponse, XmlParseError> {
        deserialize_elements::<_, DefineIndexFieldResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IndexField" => {
                        obj.index_field =
                            IndexFieldStatusDeserializer::deserialize("IndexField", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DefineSuggester</a></code> operation. Specifies the name of the domain you want to update and the suggester configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        SuggesterSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Suggester"),
            &obj.suggester,
        );
    }
}

/// <p>The result of a <code>DefineSuggester</code> request. Contains the status of the newly-configured suggester.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DefineSuggesterResponse {
    pub suggester: SuggesterStatus,
}

struct DefineSuggesterResponseDeserializer;
impl DefineSuggesterResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefineSuggesterResponse, XmlParseError> {
        deserialize_elements::<_, DefineSuggesterResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Suggester" => {
                        obj.suggester =
                            SuggesterStatusDeserializer::deserialize("Suggester", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DeleteAnalysisScheme</a></code> operation. Specifies the name of the domain you want to update and the analysis scheme you want to delete. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.analysis_scheme_name,
        );
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code>DeleteAnalysisScheme</code> request. Contains the status of the deleted analysis scheme.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteAnalysisSchemeResponse {
    /// <p>The status of the analysis scheme being deleted.</p>
    pub analysis_scheme: AnalysisSchemeStatus,
}

struct DeleteAnalysisSchemeResponseDeserializer;
impl DeleteAnalysisSchemeResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteAnalysisSchemeResponse, XmlParseError> {
        deserialize_elements::<_, DeleteAnalysisSchemeResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AnalysisScheme" => {
                        obj.analysis_scheme =
                            AnalysisSchemeStatusDeserializer::deserialize("AnalysisScheme", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DeleteDomain</a></code> operation. Specifies the name of the domain you want to delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code>DeleteDomain</code> request. Contains the status of a newly deleted domain, or no status if the domain has already been completely deleted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteDomainResponse {
    pub domain_status: Option<DomainStatus>,
}

struct DeleteDomainResponseDeserializer;
impl DeleteDomainResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteDomainResponse, XmlParseError> {
        deserialize_elements::<_, DeleteDomainResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DomainStatus" => {
                    obj.domain_status = Some(DomainStatusDeserializer::deserialize(
                        "DomainStatus",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Container for the parameters to the <code><a>DeleteExpression</a></code> operation. Specifies the name of the domain you want to update and the name of the expression you want to delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        params.put(
            &format!("{}{}", prefix, "ExpressionName"),
            &obj.expression_name,
        );
    }
}

/// <p>The result of a <code><a>DeleteExpression</a></code> request. Specifies the expression being deleted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteExpressionResponse {
    /// <p>The status of the expression being deleted.</p>
    pub expression: ExpressionStatus,
}

struct DeleteExpressionResponseDeserializer;
impl DeleteExpressionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteExpressionResponse, XmlParseError> {
        deserialize_elements::<_, DeleteExpressionResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Expression" => {
                        obj.expression =
                            ExpressionStatusDeserializer::deserialize("Expression", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DeleteIndexField</a></code> operation. Specifies the name of the domain you want to update and the name of the index field you want to delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        params.put(
            &format!("{}{}", prefix, "IndexFieldName"),
            &obj.index_field_name,
        );
    }
}

/// <p>The result of a <code><a>DeleteIndexField</a></code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteIndexFieldResponse {
    /// <p>The status of the index field being deleted.</p>
    pub index_field: IndexFieldStatus,
}

struct DeleteIndexFieldResponseDeserializer;
impl DeleteIndexFieldResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteIndexFieldResponse, XmlParseError> {
        deserialize_elements::<_, DeleteIndexFieldResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IndexField" => {
                        obj.index_field =
                            IndexFieldStatusDeserializer::deserialize("IndexField", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DeleteSuggester</a></code> operation. Specifies the name of the domain you want to update and name of the suggester you want to delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        params.put(
            &format!("{}{}", prefix, "SuggesterName"),
            &obj.suggester_name,
        );
    }
}

/// <p>The result of a <code>DeleteSuggester</code> request. Contains the status of the deleted suggester.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteSuggesterResponse {
    /// <p>The status of the suggester being deleted.</p>
    pub suggester: SuggesterStatus,
}

struct DeleteSuggesterResponseDeserializer;
impl DeleteSuggesterResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteSuggesterResponse, XmlParseError> {
        deserialize_elements::<_, DeleteSuggesterResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Suggester" => {
                        obj.suggester =
                            SuggesterStatusDeserializer::deserialize("Suggester", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DescribeAnalysisSchemes</a></code> operation. Specifies the name of the domain you want to describe. To limit the response to particular analysis schemes, specify the names of the analysis schemes you want to describe. To show the active configuration and exclude any pending changes, set the <code>Deployed</code> option to <code>true</code>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Deployed"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code>DescribeAnalysisSchemes</code> request. Contains the analysis schemes configured for the domain specified in the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAnalysisSchemesResponse {
    /// <p>The analysis scheme descriptions.</p>
    pub analysis_schemes: Vec<AnalysisSchemeStatus>,
}

struct DescribeAnalysisSchemesResponseDeserializer;
impl DescribeAnalysisSchemesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAnalysisSchemesResponse, XmlParseError> {
        deserialize_elements::<_, DescribeAnalysisSchemesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AnalysisSchemes" => {
                        obj.analysis_schemes.extend(
                            AnalysisSchemeStatusListDeserializer::deserialize(
                                "AnalysisSchemes",
                                stack,
                            )?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DescribeAvailabilityOptions</a></code> operation. Specifies the name of the domain you want to describe. To show the active configuration and exclude any pending changes, set the Deployed option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Deployed"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code>DescribeAvailabilityOptions</code> request. Indicates whether or not the Multi-AZ option is enabled for the domain specified in the request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAvailabilityOptionsResponse {
    /// <p>The availability options configured for the domain. Indicates whether Multi-AZ is enabled for the domain. </p>
    pub availability_options: Option<AvailabilityOptionsStatus>,
}

struct DescribeAvailabilityOptionsResponseDeserializer;
impl DescribeAvailabilityOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAvailabilityOptionsResponse, XmlParseError> {
        deserialize_elements::<_, DescribeAvailabilityOptionsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AvailabilityOptions" => {
                        obj.availability_options =
                            Some(AvailabilityOptionsStatusDeserializer::deserialize(
                                "AvailabilityOptions",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DescribeDomainEndpointOptions</a></code> operation. Specify the name of the domain you want to describe. To show the active configuration and exclude any pending changes, set the Deployed option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDomainEndpointOptionsRequest {
    /// <p>Whether to retrieve the latest configuration (which might be in a Processing state) or the current, active configuration. Defaults to <code>false</code>.</p>
    pub deployed: Option<bool>,
    /// <p>A string that represents the name of a domain.</p>
    pub domain_name: String,
}

/// Serialize `DescribeDomainEndpointOptionsRequest` contents to a `SignedRequest`.
struct DescribeDomainEndpointOptionsRequestSerializer;
impl DescribeDomainEndpointOptionsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDomainEndpointOptionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.deployed {
            params.put(&format!("{}{}", prefix, "Deployed"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code>DescribeDomainEndpointOptions</code> request. Contains the status and configuration of a search domain's endpoint options. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeDomainEndpointOptionsResponse {
    /// <p>The status and configuration of a search domain's endpoint options.</p>
    pub domain_endpoint_options: Option<DomainEndpointOptionsStatus>,
}

struct DescribeDomainEndpointOptionsResponseDeserializer;
impl DescribeDomainEndpointOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeDomainEndpointOptionsResponse, XmlParseError> {
        deserialize_elements::<_, DescribeDomainEndpointOptionsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DomainEndpointOptions" => {
                        obj.domain_endpoint_options =
                            Some(DomainEndpointOptionsStatusDeserializer::deserialize(
                                "DomainEndpointOptions",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DescribeDomains</a></code> operation. By default shows the status of all domains. To restrict the response to particular domains, specify the names of the domains you want to describe.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeDomainsResponse {
    pub domain_status_list: Vec<DomainStatus>,
}

struct DescribeDomainsResponseDeserializer;
impl DescribeDomainsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeDomainsResponse, XmlParseError> {
        deserialize_elements::<_, DescribeDomainsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DomainStatusList" => {
                        obj.domain_status_list
                            .extend(DomainStatusListDeserializer::deserialize(
                                "DomainStatusList",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DescribeDomains</a></code> operation. Specifies the name of the domain you want to describe. To restrict the response to particular expressions, specify the names of the expressions you want to describe. To show the active configuration and exclude any pending changes, set the <code>Deployed</code> option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Deployed"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeExpressionsResponse {
    /// <p>The expressions configured for the domain.</p>
    pub expressions: Vec<ExpressionStatus>,
}

struct DescribeExpressionsResponseDeserializer;
impl DescribeExpressionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeExpressionsResponse, XmlParseError> {
        deserialize_elements::<_, DescribeExpressionsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Expressions" => {
                        obj.expressions
                            .extend(ExpressionStatusListDeserializer::deserialize(
                                "Expressions",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DescribeIndexFields</a></code> operation. Specifies the name of the domain you want to describe. To restrict the response to particular index fields, specify the names of the index fields you want to describe. To show the active configuration and exclude any pending changes, set the <code>Deployed</code> option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Deployed"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeIndexFieldsResponse {
    /// <p>The index fields configured for the domain.</p>
    pub index_fields: Vec<IndexFieldStatus>,
}

struct DescribeIndexFieldsResponseDeserializer;
impl DescribeIndexFieldsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeIndexFieldsResponse, XmlParseError> {
        deserialize_elements::<_, DescribeIndexFieldsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IndexFields" => {
                        obj.index_fields
                            .extend(IndexFieldStatusListDeserializer::deserialize(
                                "IndexFields",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DescribeScalingParameters</a></code> operation. Specifies the name of the domain you want to describe. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code>DescribeScalingParameters</code> request. Contains the scaling parameters configured for the domain specified in the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeScalingParametersResponse {
    pub scaling_parameters: ScalingParametersStatus,
}

struct DescribeScalingParametersResponseDeserializer;
impl DescribeScalingParametersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeScalingParametersResponse, XmlParseError> {
        deserialize_elements::<_, DescribeScalingParametersResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ScalingParameters" => {
                        obj.scaling_parameters = ScalingParametersStatusDeserializer::deserialize(
                            "ScalingParameters",
                            stack,
                        )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DescribeServiceAccessPolicies</a></code> operation. Specifies the name of the domain you want to describe. To show the active configuration and exclude any pending changes, set the <code>Deployed</code> option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Deployed"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code>DescribeServiceAccessPolicies</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeServiceAccessPoliciesResponse {
    /// <p>The access rules configured for the domain specified in the request.</p>
    pub access_policies: AccessPoliciesStatus,
}

struct DescribeServiceAccessPoliciesResponseDeserializer;
impl DescribeServiceAccessPoliciesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeServiceAccessPoliciesResponse, XmlParseError> {
        deserialize_elements::<_, DescribeServiceAccessPoliciesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AccessPolicies" => {
                        obj.access_policies =
                            AccessPoliciesStatusDeserializer::deserialize("AccessPolicies", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>DescribeSuggester</a></code> operation. Specifies the name of the domain you want to describe. To restrict the response to particular suggesters, specify the names of the suggesters you want to describe. To show the active configuration and exclude any pending changes, set the <code>Deployed</code> option to <code>true</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Deployed"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeSuggestersResponse {
    /// <p>The suggesters configured for the domain specified in the request.</p>
    pub suggesters: Vec<SuggesterStatus>,
}

struct DescribeSuggestersResponseDeserializer;
impl DescribeSuggestersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeSuggestersResponse, XmlParseError> {
        deserialize_elements::<_, DescribeSuggestersResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Suggesters" => {
                        obj.suggesters
                            .extend(SuggesterStatusListDeserializer::deserialize(
                                "Suggesters",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Options for a search suggester.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DocumentSuggesterOptions, XmlParseError> {
        deserialize_elements::<_, DocumentSuggesterOptions, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "FuzzyMatching" => {
                        obj.fuzzy_matching = Some(SuggesterFuzzyMatchingDeserializer::deserialize(
                            "FuzzyMatching",
                            stack,
                        )?);
                    }
                    "SortExpression" => {
                        obj.sort_expression =
                            Some(StringDeserializer::deserialize("SortExpression", stack)?);
                    }
                    "SourceField" => {
                        obj.source_field =
                            FieldNameDeserializer::deserialize("SourceField", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
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
            params.put(&format!("{}{}", prefix, "FuzzyMatching"), &field_value);
        }
        if let Some(ref field_value) = obj.sort_expression {
            params.put(&format!("{}{}", prefix, "SortExpression"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "SourceField"), &obj.source_field);
    }
}

/// <p>The domain's endpoint options.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DomainEndpointOptions {
    /// <p>Whether the domain is HTTPS only enabled.</p>
    pub enforce_https: Option<bool>,
    /// <p>The minimum required TLS version</p>
    pub tls_security_policy: Option<String>,
}

struct DomainEndpointOptionsDeserializer;
impl DomainEndpointOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DomainEndpointOptions, XmlParseError> {
        deserialize_elements::<_, DomainEndpointOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "EnforceHTTPS" => {
                    obj.enforce_https =
                        Some(BooleanDeserializer::deserialize("EnforceHTTPS", stack)?);
                }
                "TLSSecurityPolicy" => {
                    obj.tls_security_policy = Some(TLSSecurityPolicyDeserializer::deserialize(
                        "TLSSecurityPolicy",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `DomainEndpointOptions` contents to a `SignedRequest`.
struct DomainEndpointOptionsSerializer;
impl DomainEndpointOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DomainEndpointOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.enforce_https {
            params.put(&format!("{}{}", prefix, "EnforceHTTPS"), &field_value);
        }
        if let Some(ref field_value) = obj.tls_security_policy {
            params.put(&format!("{}{}", prefix, "TLSSecurityPolicy"), &field_value);
        }
    }
}

/// <p>The configuration and status of the domain's endpoint options.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DomainEndpointOptionsStatus {
    /// <p>The domain endpoint options configured for the domain.</p>
    pub options: DomainEndpointOptions,
    /// <p>The status of the configured domain endpoint options.</p>
    pub status: OptionStatus,
}

struct DomainEndpointOptionsStatusDeserializer;
impl DomainEndpointOptionsStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DomainEndpointOptionsStatus, XmlParseError> {
        deserialize_elements::<_, DomainEndpointOptionsStatus, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Options" => {
                        obj.options =
                            DomainEndpointOptionsDeserializer::deserialize("Options", stack)?;
                    }
                    "Status" => {
                        obj.status = OptionStatusDeserializer::deserialize("Status", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct DomainIdDeserializer;
impl DomainIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DomainNameDeserializer;
impl DomainNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key = DomainNameDeserializer::deserialize("key", stack)?;
            let value = APIVersionDeserializer::deserialize("value", stack)?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}
/// <p>The current status of the search domain.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DomainStatus, XmlParseError> {
        deserialize_elements::<_, DomainStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = Some(ARNDeserializer::deserialize("ARN", stack)?);
                }
                "Created" => {
                    obj.created = Some(BooleanDeserializer::deserialize("Created", stack)?);
                }
                "Deleted" => {
                    obj.deleted = Some(BooleanDeserializer::deserialize("Deleted", stack)?);
                }
                "DocService" => {
                    obj.doc_service = Some(ServiceEndpointDeserializer::deserialize(
                        "DocService",
                        stack,
                    )?);
                }
                "DomainId" => {
                    obj.domain_id = DomainIdDeserializer::deserialize("DomainId", stack)?;
                }
                "DomainName" => {
                    obj.domain_name = DomainNameDeserializer::deserialize("DomainName", stack)?;
                }
                "Limits" => {
                    obj.limits = Some(LimitsDeserializer::deserialize("Limits", stack)?);
                }
                "Processing" => {
                    obj.processing = Some(BooleanDeserializer::deserialize("Processing", stack)?);
                }
                "RequiresIndexDocuments" => {
                    obj.requires_index_documents =
                        BooleanDeserializer::deserialize("RequiresIndexDocuments", stack)?;
                }
                "SearchInstanceCount" => {
                    obj.search_instance_count = Some(InstanceCountDeserializer::deserialize(
                        "SearchInstanceCount",
                        stack,
                    )?);
                }
                "SearchInstanceType" => {
                    obj.search_instance_type = Some(SearchInstanceTypeDeserializer::deserialize(
                        "SearchInstanceType",
                        stack,
                    )?);
                }
                "SearchPartitionCount" => {
                    obj.search_partition_count = Some(PartitionCountDeserializer::deserialize(
                        "SearchPartitionCount",
                        stack,
                    )?);
                }
                "SearchService" => {
                    obj.search_service = Some(ServiceEndpointDeserializer::deserialize(
                        "SearchService",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DomainStatusListDeserializer;
impl DomainStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DomainStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(DomainStatusDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct DoubleDeserializer;
impl DoubleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = f64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Options for a field that contains an array of double-precision 64-bit floating point values. Present if <code>IndexFieldType</code> specifies the field is of type <code>double-array</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DoubleArrayOptions, XmlParseError> {
        deserialize_elements::<_, DoubleArrayOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value =
                        Some(DoubleDeserializer::deserialize("DefaultValue", stack)?);
                }
                "FacetEnabled" => {
                    obj.facet_enabled =
                        Some(BooleanDeserializer::deserialize("FacetEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SearchEnabled" => {
                    obj.search_enabled =
                        Some(BooleanDeserializer::deserialize("SearchEnabled", stack)?);
                }
                "SourceFields" => {
                    obj.source_fields = Some(FieldNameCommaListDeserializer::deserialize(
                        "SourceFields",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(&format!("{}{}", prefix, "FacetEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(&format!("{}{}", prefix, "SearchEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_fields {
            params.put(&format!("{}{}", prefix, "SourceFields"), &field_value);
        }
    }
}

/// <p>Options for a double-precision 64-bit floating point field. Present if <code>IndexFieldType</code> specifies the field is of type <code>double</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DoubleOptions, XmlParseError> {
        deserialize_elements::<_, DoubleOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value =
                        Some(DoubleDeserializer::deserialize("DefaultValue", stack)?);
                }
                "FacetEnabled" => {
                    obj.facet_enabled =
                        Some(BooleanDeserializer::deserialize("FacetEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SearchEnabled" => {
                    obj.search_enabled =
                        Some(BooleanDeserializer::deserialize("SearchEnabled", stack)?);
                }
                "SortEnabled" => {
                    obj.sort_enabled =
                        Some(BooleanDeserializer::deserialize("SortEnabled", stack)?);
                }
                "SourceField" => {
                    obj.source_field =
                        Some(FieldNameDeserializer::deserialize("SourceField", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(&format!("{}{}", prefix, "FacetEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(&format!("{}{}", prefix, "SearchEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(&format!("{}{}", prefix, "SortEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(&format!("{}{}", prefix, "SourceField"), &field_value);
        }
    }
}

struct DynamicFieldNameDeserializer;
impl DynamicFieldNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Expression {
    pub expression_name: String,
    pub expression_value: String,
}

struct ExpressionDeserializer;
impl ExpressionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Expression, XmlParseError> {
        deserialize_elements::<_, Expression, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ExpressionName" => {
                    obj.expression_name =
                        StandardNameDeserializer::deserialize("ExpressionName", stack)?;
                }
                "ExpressionValue" => {
                    obj.expression_value =
                        ExpressionValueDeserializer::deserialize("ExpressionValue", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            &obj.expression_name,
        );
        params.put(
            &format!("{}{}", prefix, "ExpressionValue"),
            &obj.expression_value,
        );
    }
}

/// <p>The value of an <code>Expression</code> and its current status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ExpressionStatus {
    /// <p>The expression that is evaluated for sorting while processing a search request.</p>
    pub options: Expression,
    pub status: OptionStatus,
}

struct ExpressionStatusDeserializer;
impl ExpressionStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ExpressionStatus, XmlParseError> {
        deserialize_elements::<_, ExpressionStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Options" => {
                    obj.options = ExpressionDeserializer::deserialize("Options", stack)?;
                }
                "Status" => {
                    obj.status = OptionStatusDeserializer::deserialize("Status", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ExpressionStatusListDeserializer;
impl ExpressionStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ExpressionStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ExpressionStatusDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ExpressionValueDeserializer;
impl ExpressionValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct FieldNameDeserializer;
impl FieldNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct FieldNameCommaListDeserializer;
impl FieldNameCommaListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct FieldNameListDeserializer;
impl FieldNameListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(FieldNameDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct FieldValueDeserializer;
impl FieldValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>IndexDocuments</a></code> operation. Specifies the name of the domain you want to re-index.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of an <code>IndexDocuments</code> request. Contains the status of the indexing operation, including the fields being indexed.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct IndexDocumentsResponse {
    /// <p>The names of the fields that are currently being indexed.</p>
    pub field_names: Option<Vec<String>>,
}

struct IndexDocumentsResponseDeserializer;
impl IndexDocumentsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IndexDocumentsResponse, XmlParseError> {
        deserialize_elements::<_, IndexDocumentsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "FieldNames" => {
                    obj.field_names
                        .get_or_insert(vec![])
                        .extend(FieldNameListDeserializer::deserialize("FieldNames", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Configuration information for a field in the index, including its name, type, and options. The supported options depend on the <code><a>IndexFieldType</a></code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IndexField, XmlParseError> {
        deserialize_elements::<_, IndexField, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DateArrayOptions" => {
                    obj.date_array_options = Some(DateArrayOptionsDeserializer::deserialize(
                        "DateArrayOptions",
                        stack,
                    )?);
                }
                "DateOptions" => {
                    obj.date_options =
                        Some(DateOptionsDeserializer::deserialize("DateOptions", stack)?);
                }
                "DoubleArrayOptions" => {
                    obj.double_array_options = Some(DoubleArrayOptionsDeserializer::deserialize(
                        "DoubleArrayOptions",
                        stack,
                    )?);
                }
                "DoubleOptions" => {
                    obj.double_options = Some(DoubleOptionsDeserializer::deserialize(
                        "DoubleOptions",
                        stack,
                    )?);
                }
                "IndexFieldName" => {
                    obj.index_field_name =
                        DynamicFieldNameDeserializer::deserialize("IndexFieldName", stack)?;
                }
                "IndexFieldType" => {
                    obj.index_field_type =
                        IndexFieldTypeDeserializer::deserialize("IndexFieldType", stack)?;
                }
                "IntArrayOptions" => {
                    obj.int_array_options = Some(IntArrayOptionsDeserializer::deserialize(
                        "IntArrayOptions",
                        stack,
                    )?);
                }
                "IntOptions" => {
                    obj.int_options =
                        Some(IntOptionsDeserializer::deserialize("IntOptions", stack)?);
                }
                "LatLonOptions" => {
                    obj.lat_lon_options = Some(LatLonOptionsDeserializer::deserialize(
                        "LatLonOptions",
                        stack,
                    )?);
                }
                "LiteralArrayOptions" => {
                    obj.literal_array_options = Some(LiteralArrayOptionsDeserializer::deserialize(
                        "LiteralArrayOptions",
                        stack,
                    )?);
                }
                "LiteralOptions" => {
                    obj.literal_options = Some(LiteralOptionsDeserializer::deserialize(
                        "LiteralOptions",
                        stack,
                    )?);
                }
                "TextArrayOptions" => {
                    obj.text_array_options = Some(TextArrayOptionsDeserializer::deserialize(
                        "TextArrayOptions",
                        stack,
                    )?);
                }
                "TextOptions" => {
                    obj.text_options =
                        Some(TextOptionsDeserializer::deserialize("TextOptions", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            &obj.index_field_name,
        );
        params.put(
            &format!("{}{}", prefix, "IndexFieldType"),
            &obj.index_field_type,
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct IndexFieldStatus {
    pub options: IndexField,
    pub status: OptionStatus,
}

struct IndexFieldStatusDeserializer;
impl IndexFieldStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IndexFieldStatus, XmlParseError> {
        deserialize_elements::<_, IndexFieldStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Options" => {
                    obj.options = IndexFieldDeserializer::deserialize("Options", stack)?;
                }
                "Status" => {
                    obj.status = OptionStatusDeserializer::deserialize("Status", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct IndexFieldStatusListDeserializer;
impl IndexFieldStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<IndexFieldStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(IndexFieldStatusDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct IndexFieldTypeDeserializer;
impl IndexFieldTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct InstanceCountDeserializer;
impl InstanceCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Options for a field that contains an array of 64-bit signed integers. Present if <code>IndexFieldType</code> specifies the field is of type <code>int-array</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IntArrayOptions, XmlParseError> {
        deserialize_elements::<_, IntArrayOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value = Some(LongDeserializer::deserialize("DefaultValue", stack)?);
                }
                "FacetEnabled" => {
                    obj.facet_enabled =
                        Some(BooleanDeserializer::deserialize("FacetEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SearchEnabled" => {
                    obj.search_enabled =
                        Some(BooleanDeserializer::deserialize("SearchEnabled", stack)?);
                }
                "SourceFields" => {
                    obj.source_fields = Some(FieldNameCommaListDeserializer::deserialize(
                        "SourceFields",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(&format!("{}{}", prefix, "FacetEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(&format!("{}{}", prefix, "SearchEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_fields {
            params.put(&format!("{}{}", prefix, "SourceFields"), &field_value);
        }
    }
}

/// <p>Options for a 64-bit signed integer field. Present if <code>IndexFieldType</code> specifies the field is of type <code>int</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IntOptions, XmlParseError> {
        deserialize_elements::<_, IntOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value = Some(LongDeserializer::deserialize("DefaultValue", stack)?);
                }
                "FacetEnabled" => {
                    obj.facet_enabled =
                        Some(BooleanDeserializer::deserialize("FacetEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SearchEnabled" => {
                    obj.search_enabled =
                        Some(BooleanDeserializer::deserialize("SearchEnabled", stack)?);
                }
                "SortEnabled" => {
                    obj.sort_enabled =
                        Some(BooleanDeserializer::deserialize("SortEnabled", stack)?);
                }
                "SourceField" => {
                    obj.source_field =
                        Some(FieldNameDeserializer::deserialize("SourceField", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(&format!("{}{}", prefix, "FacetEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(&format!("{}{}", prefix, "SearchEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(&format!("{}{}", prefix, "SortEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(&format!("{}{}", prefix, "SourceField"), &field_value);
        }
    }
}

/// <p>Options for a latlon field. A latlon field contains a location stored as a latitude and longitude value pair. Present if <code>IndexFieldType</code> specifies the field is of type <code>latlon</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LatLonOptions, XmlParseError> {
        deserialize_elements::<_, LatLonOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value =
                        Some(FieldValueDeserializer::deserialize("DefaultValue", stack)?);
                }
                "FacetEnabled" => {
                    obj.facet_enabled =
                        Some(BooleanDeserializer::deserialize("FacetEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SearchEnabled" => {
                    obj.search_enabled =
                        Some(BooleanDeserializer::deserialize("SearchEnabled", stack)?);
                }
                "SortEnabled" => {
                    obj.sort_enabled =
                        Some(BooleanDeserializer::deserialize("SortEnabled", stack)?);
                }
                "SourceField" => {
                    obj.source_field =
                        Some(FieldNameDeserializer::deserialize("SourceField", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(&format!("{}{}", prefix, "FacetEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(&format!("{}{}", prefix, "SearchEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(&format!("{}{}", prefix, "SortEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(&format!("{}{}", prefix, "SourceField"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Limits {
    pub maximum_partition_count: i64,
    pub maximum_replication_count: i64,
}

struct LimitsDeserializer;
impl LimitsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Limits, XmlParseError> {
        deserialize_elements::<_, Limits, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "MaximumPartitionCount" => {
                    obj.maximum_partition_count = MaximumPartitionCountDeserializer::deserialize(
                        "MaximumPartitionCount",
                        stack,
                    )?;
                }
                "MaximumReplicationCount" => {
                    obj.maximum_replication_count =
                        MaximumReplicationCountDeserializer::deserialize(
                            "MaximumReplicationCount",
                            stack,
                        )?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>The result of a <code>ListDomainNames</code> request. Contains a list of the domains owned by an account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListDomainNamesResponse {
    /// <p>The names of the search domains owned by an account.</p>
    pub domain_names: Option<::std::collections::HashMap<String, String>>,
}

struct ListDomainNamesResponseDeserializer;
impl ListDomainNamesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDomainNamesResponse, XmlParseError> {
        deserialize_elements::<_, ListDomainNamesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DomainNames" => {
                        obj.domain_names = Some(DomainNameMapDeserializer::deserialize(
                            "DomainNames",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Options for a field that contains an array of literal strings. Present if <code>IndexFieldType</code> specifies the field is of type <code>literal-array</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LiteralArrayOptions, XmlParseError> {
        deserialize_elements::<_, LiteralArrayOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value =
                        Some(FieldValueDeserializer::deserialize("DefaultValue", stack)?);
                }
                "FacetEnabled" => {
                    obj.facet_enabled =
                        Some(BooleanDeserializer::deserialize("FacetEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SearchEnabled" => {
                    obj.search_enabled =
                        Some(BooleanDeserializer::deserialize("SearchEnabled", stack)?);
                }
                "SourceFields" => {
                    obj.source_fields = Some(FieldNameCommaListDeserializer::deserialize(
                        "SourceFields",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(&format!("{}{}", prefix, "FacetEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(&format!("{}{}", prefix, "SearchEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_fields {
            params.put(&format!("{}{}", prefix, "SourceFields"), &field_value);
        }
    }
}

/// <p>Options for literal field. Present if <code>IndexFieldType</code> specifies the field is of type <code>literal</code>. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LiteralOptions, XmlParseError> {
        deserialize_elements::<_, LiteralOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value =
                        Some(FieldValueDeserializer::deserialize("DefaultValue", stack)?);
                }
                "FacetEnabled" => {
                    obj.facet_enabled =
                        Some(BooleanDeserializer::deserialize("FacetEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SearchEnabled" => {
                    obj.search_enabled =
                        Some(BooleanDeserializer::deserialize("SearchEnabled", stack)?);
                }
                "SortEnabled" => {
                    obj.sort_enabled =
                        Some(BooleanDeserializer::deserialize("SortEnabled", stack)?);
                }
                "SourceField" => {
                    obj.source_field =
                        Some(FieldNameDeserializer::deserialize("SourceField", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.facet_enabled {
            params.put(&format!("{}{}", prefix, "FacetEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.search_enabled {
            params.put(&format!("{}{}", prefix, "SearchEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(&format!("{}{}", prefix, "SortEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(&format!("{}{}", prefix, "SourceField"), &field_value);
        }
    }
}

struct LongDeserializer;
impl LongDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MaximumPartitionCountDeserializer;
impl MaximumPartitionCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MaximumReplicationCountDeserializer;
impl MaximumReplicationCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MultiAZDeserializer;
impl MultiAZDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct OptionStateDeserializer;
impl OptionStateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The status of domain configuration option.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct OptionStatus {
    /// <p>A timestamp for when this option was created.</p>
    pub creation_date: String,
    /// <p>Indicates that the option will be deleted once processing is complete.</p>
    pub pending_deletion: Option<bool>,
    /// <p><p>The state of processing a change to an option. Possible values:</p><ul> <li><code>RequiresIndexDocuments</code>: the option&#39;s latest value will not be deployed until <a>IndexDocuments</a> has been called and indexing is complete.</li> <li><code>Processing</code>: the option&#39;s latest value is in the process of being activated. </li> <li><code>Active</code>: the option&#39;s latest value is completely deployed.</li> <li><code>FailedToValidate</code>: the option value is not compatible with the domain&#39;s data and cannot be used to index the data. You must either modify the option value or update or remove the incompatible documents.</li> </ul></p>
    pub state: String,
    /// <p>A timestamp for when this option was last updated.</p>
    pub update_date: String,
    /// <p>A unique integer that indicates when this option was last updated.</p>
    pub update_version: Option<i64>,
}

struct OptionStatusDeserializer;
impl OptionStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OptionStatus, XmlParseError> {
        deserialize_elements::<_, OptionStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CreationDate" => {
                    obj.creation_date =
                        UpdateTimestampDeserializer::deserialize("CreationDate", stack)?;
                }
                "PendingDeletion" => {
                    obj.pending_deletion =
                        Some(BooleanDeserializer::deserialize("PendingDeletion", stack)?);
                }
                "State" => {
                    obj.state = OptionStateDeserializer::deserialize("State", stack)?;
                }
                "UpdateDate" => {
                    obj.update_date =
                        UpdateTimestampDeserializer::deserialize("UpdateDate", stack)?;
                }
                "UpdateVersion" => {
                    obj.update_version =
                        Some(UIntValueDeserializer::deserialize("UpdateVersion", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct PartitionCountDeserializer;
impl PartitionCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PartitionInstanceTypeDeserializer;
impl PartitionInstanceTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PolicyDocumentDeserializer;
impl PolicyDocumentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The desired instance type and desired number of replicas of each index partition.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScalingParameters, XmlParseError> {
        deserialize_elements::<_, ScalingParameters, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DesiredInstanceType" => {
                    obj.desired_instance_type =
                        Some(PartitionInstanceTypeDeserializer::deserialize(
                            "DesiredInstanceType",
                            stack,
                        )?);
                }
                "DesiredPartitionCount" => {
                    obj.desired_partition_count = Some(UIntValueDeserializer::deserialize(
                        "DesiredPartitionCount",
                        stack,
                    )?);
                }
                "DesiredReplicationCount" => {
                    obj.desired_replication_count = Some(UIntValueDeserializer::deserialize(
                        "DesiredReplicationCount",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.desired_partition_count {
            params.put(
                &format!("{}{}", prefix, "DesiredPartitionCount"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.desired_replication_count {
            params.put(
                &format!("{}{}", prefix, "DesiredReplicationCount"),
                &field_value,
            );
        }
    }
}

/// <p>The status and configuration of a search domain's scaling parameters. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ScalingParametersStatus {
    pub options: ScalingParameters,
    pub status: OptionStatus,
}

struct ScalingParametersStatusDeserializer;
impl ScalingParametersStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScalingParametersStatus, XmlParseError> {
        deserialize_elements::<_, ScalingParametersStatus, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Options" => {
                        obj.options = ScalingParametersDeserializer::deserialize("Options", stack)?;
                    }
                    "Status" => {
                        obj.status = OptionStatusDeserializer::deserialize("Status", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct SearchInstanceTypeDeserializer;
impl SearchInstanceTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The endpoint to which service requests can be submitted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ServiceEndpoint {
    pub endpoint: Option<String>,
}

struct ServiceEndpointDeserializer;
impl ServiceEndpointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServiceEndpoint, XmlParseError> {
        deserialize_elements::<_, ServiceEndpoint, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Endpoint" => {
                    obj.endpoint = Some(ServiceUrlDeserializer::deserialize("Endpoint", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ServiceUrlDeserializer;
impl ServiceUrlDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StandardNameDeserializer;
impl StandardNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Configuration information for a search suggester. Each suggester has a unique name and specifies the text field you want to use for suggestions. The following options can be configured for a suggester: <code>FuzzyMatching</code>, <code>SortExpression</code>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Suggester {
    pub document_suggester_options: DocumentSuggesterOptions,
    pub suggester_name: String,
}

struct SuggesterDeserializer;
impl SuggesterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Suggester, XmlParseError> {
        deserialize_elements::<_, Suggester, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DocumentSuggesterOptions" => {
                    obj.document_suggester_options =
                        DocumentSuggesterOptionsDeserializer::deserialize(
                            "DocumentSuggesterOptions",
                            stack,
                        )?;
                }
                "SuggesterName" => {
                    obj.suggester_name =
                        StandardNameDeserializer::deserialize("SuggesterName", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            &obj.suggester_name,
        );
    }
}

struct SuggesterFuzzyMatchingDeserializer;
impl SuggesterFuzzyMatchingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The value of a <code>Suggester</code> and its current status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SuggesterStatus {
    pub options: Suggester,
    pub status: OptionStatus,
}

struct SuggesterStatusDeserializer;
impl SuggesterStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SuggesterStatus, XmlParseError> {
        deserialize_elements::<_, SuggesterStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Options" => {
                    obj.options = SuggesterDeserializer::deserialize("Options", stack)?;
                }
                "Status" => {
                    obj.status = OptionStatusDeserializer::deserialize("Status", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct SuggesterStatusListDeserializer;
impl SuggesterStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SuggesterStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(SuggesterStatusDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct TLSSecurityPolicyDeserializer;
impl TLSSecurityPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Options for a field that contains an array of text strings. Present if <code>IndexFieldType</code> specifies the field is of type <code>text-array</code>. A <code>text-array</code> field is always searchable. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TextArrayOptions, XmlParseError> {
        deserialize_elements::<_, TextArrayOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AnalysisScheme" => {
                    obj.analysis_scheme =
                        Some(WordDeserializer::deserialize("AnalysisScheme", stack)?);
                }
                "DefaultValue" => {
                    obj.default_value =
                        Some(FieldValueDeserializer::deserialize("DefaultValue", stack)?);
                }
                "HighlightEnabled" => {
                    obj.highlight_enabled =
                        Some(BooleanDeserializer::deserialize("HighlightEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SourceFields" => {
                    obj.source_fields = Some(FieldNameCommaListDeserializer::deserialize(
                        "SourceFields",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "AnalysisScheme"), &field_value);
        }
        if let Some(ref field_value) = obj.default_value {
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.highlight_enabled {
            params.put(&format!("{}{}", prefix, "HighlightEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_fields {
            params.put(&format!("{}{}", prefix, "SourceFields"), &field_value);
        }
    }
}

/// <p>Options for text field. Present if <code>IndexFieldType</code> specifies the field is of type <code>text</code>. A <code>text</code> field is always searchable. All options are enabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TextOptions, XmlParseError> {
        deserialize_elements::<_, TextOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AnalysisScheme" => {
                    obj.analysis_scheme =
                        Some(WordDeserializer::deserialize("AnalysisScheme", stack)?);
                }
                "DefaultValue" => {
                    obj.default_value =
                        Some(FieldValueDeserializer::deserialize("DefaultValue", stack)?);
                }
                "HighlightEnabled" => {
                    obj.highlight_enabled =
                        Some(BooleanDeserializer::deserialize("HighlightEnabled", stack)?);
                }
                "ReturnEnabled" => {
                    obj.return_enabled =
                        Some(BooleanDeserializer::deserialize("ReturnEnabled", stack)?);
                }
                "SortEnabled" => {
                    obj.sort_enabled =
                        Some(BooleanDeserializer::deserialize("SortEnabled", stack)?);
                }
                "SourceField" => {
                    obj.source_field =
                        Some(FieldNameDeserializer::deserialize("SourceField", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "AnalysisScheme"), &field_value);
        }
        if let Some(ref field_value) = obj.default_value {
            params.put(&format!("{}{}", prefix, "DefaultValue"), &field_value);
        }
        if let Some(ref field_value) = obj.highlight_enabled {
            params.put(&format!("{}{}", prefix, "HighlightEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.return_enabled {
            params.put(&format!("{}{}", prefix, "ReturnEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.sort_enabled {
            params.put(&format!("{}{}", prefix, "SortEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.source_field {
            params.put(&format!("{}{}", prefix, "SourceField"), &field_value);
        }
    }
}

struct UIntValueDeserializer;
impl UIntValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Container for the parameters to the <code><a>UpdateAvailabilityOptions</a></code> operation. Specifies the name of the domain you want to update and the Multi-AZ availability option.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        params.put(&format!("{}{}", prefix, "MultiAZ"), &obj.multi_az);
    }
}

/// <p>The result of a <code>UpdateAvailabilityOptions</code> request. Contains the status of the domain's availability options. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateAvailabilityOptionsResponse {
    /// <p>The newly-configured availability options. Indicates whether Multi-AZ is enabled for the domain. </p>
    pub availability_options: Option<AvailabilityOptionsStatus>,
}

struct UpdateAvailabilityOptionsResponseDeserializer;
impl UpdateAvailabilityOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateAvailabilityOptionsResponse, XmlParseError> {
        deserialize_elements::<_, UpdateAvailabilityOptionsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AvailabilityOptions" => {
                        obj.availability_options =
                            Some(AvailabilityOptionsStatusDeserializer::deserialize(
                                "AvailabilityOptions",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>UpdateDomainEndpointOptions</a></code> operation. Specifies the name of the domain you want to update and the domain endpoint options.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDomainEndpointOptionsRequest {
    /// <p>Whether to require that all requests to the domain arrive over HTTPS. We recommend Policy-Min-TLS-1-2-2019-07 for TLSSecurityPolicy. For compatibility with older clients, the default is Policy-Min-TLS-1-0-2019-07. </p>
    pub domain_endpoint_options: DomainEndpointOptions,
    /// <p>A string that represents the name of a domain.</p>
    pub domain_name: String,
}

/// Serialize `UpdateDomainEndpointOptionsRequest` contents to a `SignedRequest`.
struct UpdateDomainEndpointOptionsRequestSerializer;
impl UpdateDomainEndpointOptionsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateDomainEndpointOptionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        DomainEndpointOptionsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "DomainEndpointOptions"),
            &obj.domain_endpoint_options,
        );
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of a <code>UpdateDomainEndpointOptions</code> request. Contains the configuration and status of the domain's endpoint options. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateDomainEndpointOptionsResponse {
    /// <p>The newly-configured domain endpoint options.</p>
    pub domain_endpoint_options: Option<DomainEndpointOptionsStatus>,
}

struct UpdateDomainEndpointOptionsResponseDeserializer;
impl UpdateDomainEndpointOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateDomainEndpointOptionsResponse, XmlParseError> {
        deserialize_elements::<_, UpdateDomainEndpointOptionsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DomainEndpointOptions" => {
                        obj.domain_endpoint_options =
                            Some(DomainEndpointOptionsStatusDeserializer::deserialize(
                                "DomainEndpointOptions",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>UpdateScalingParameters</a></code> operation. Specifies the name of the domain you want to update and the scaling parameters you want to configure.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        ScalingParametersSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ScalingParameters"),
            &obj.scaling_parameters,
        );
    }
}

/// <p>The result of a <code>UpdateScalingParameters</code> request. Contains the status of the newly-configured scaling parameters.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateScalingParametersResponse {
    pub scaling_parameters: ScalingParametersStatus,
}

struct UpdateScalingParametersResponseDeserializer;
impl UpdateScalingParametersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateScalingParametersResponse, XmlParseError> {
        deserialize_elements::<_, UpdateScalingParametersResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ScalingParameters" => {
                        obj.scaling_parameters = ScalingParametersStatusDeserializer::deserialize(
                            "ScalingParameters",
                            stack,
                        )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Container for the parameters to the <code><a>UpdateServiceAccessPolicies</a></code> operation. Specifies the name of the domain you want to update and the access rules you want to configure.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.access_policies,
        );
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

/// <p>The result of an <code>UpdateServiceAccessPolicies</code> request. Contains the new access policies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateServiceAccessPoliciesResponse {
    /// <p>The access rules configured for the domain.</p>
    pub access_policies: AccessPoliciesStatus,
}

struct UpdateServiceAccessPoliciesResponseDeserializer;
impl UpdateServiceAccessPoliciesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateServiceAccessPoliciesResponse, XmlParseError> {
        deserialize_elements::<_, UpdateServiceAccessPoliciesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AccessPolicies" => {
                        obj.access_policies =
                            AccessPoliciesStatusDeserializer::deserialize("AccessPolicies", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct UpdateTimestampDeserializer;
impl UpdateTimestampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct WordDeserializer;
impl WordDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
}

impl BuildSuggestersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BuildSuggestersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(BuildSuggestersError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(BuildSuggestersError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(BuildSuggestersError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for BuildSuggestersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BuildSuggestersError::Base(ref cause) => write!(f, "{}", cause),
            BuildSuggestersError::Internal(ref cause) => write!(f, "{}", cause),
            BuildSuggestersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BuildSuggestersError {}
/// Errors returned by CreateDomain
#[derive(Debug, PartialEq)]
pub enum CreateDomainError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
}

impl CreateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(CreateDomainError::Base(parsed_error.message))
                    }
                    "InternalException" => {
                        return RusotoError::Service(CreateDomainError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(CreateDomainError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDomainError::Base(ref cause) => write!(f, "{}", cause),
            CreateDomainError::Internal(ref cause) => write!(f, "{}", cause),
            CreateDomainError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDomainError {}
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
}

impl DefineAnalysisSchemeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DefineAnalysisSchemeError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DefineAnalysisSchemeError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DefineAnalysisSchemeError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(DefineAnalysisSchemeError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(DefineAnalysisSchemeError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DefineAnalysisSchemeError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DefineAnalysisSchemeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DefineAnalysisSchemeError::Base(ref cause) => write!(f, "{}", cause),
            DefineAnalysisSchemeError::Internal(ref cause) => write!(f, "{}", cause),
            DefineAnalysisSchemeError::InvalidType(ref cause) => write!(f, "{}", cause),
            DefineAnalysisSchemeError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DefineAnalysisSchemeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DefineAnalysisSchemeError {}
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
}

impl DefineExpressionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DefineExpressionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DefineExpressionError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DefineExpressionError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(DefineExpressionError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(DefineExpressionError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DefineExpressionError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DefineExpressionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DefineExpressionError::Base(ref cause) => write!(f, "{}", cause),
            DefineExpressionError::Internal(ref cause) => write!(f, "{}", cause),
            DefineExpressionError::InvalidType(ref cause) => write!(f, "{}", cause),
            DefineExpressionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DefineExpressionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DefineExpressionError {}
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
}

impl DefineIndexFieldError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DefineIndexFieldError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DefineIndexFieldError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DefineIndexFieldError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(DefineIndexFieldError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(DefineIndexFieldError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DefineIndexFieldError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DefineIndexFieldError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DefineIndexFieldError::Base(ref cause) => write!(f, "{}", cause),
            DefineIndexFieldError::Internal(ref cause) => write!(f, "{}", cause),
            DefineIndexFieldError::InvalidType(ref cause) => write!(f, "{}", cause),
            DefineIndexFieldError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DefineIndexFieldError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DefineIndexFieldError {}
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
}

impl DefineSuggesterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DefineSuggesterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DefineSuggesterError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DefineSuggesterError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(DefineSuggesterError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(DefineSuggesterError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DefineSuggesterError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DefineSuggesterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DefineSuggesterError::Base(ref cause) => write!(f, "{}", cause),
            DefineSuggesterError::Internal(ref cause) => write!(f, "{}", cause),
            DefineSuggesterError::InvalidType(ref cause) => write!(f, "{}", cause),
            DefineSuggesterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DefineSuggesterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DefineSuggesterError {}
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
}

impl DeleteAnalysisSchemeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAnalysisSchemeError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DeleteAnalysisSchemeError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DeleteAnalysisSchemeError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(DeleteAnalysisSchemeError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DeleteAnalysisSchemeError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteAnalysisSchemeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAnalysisSchemeError::Base(ref cause) => write!(f, "{}", cause),
            DeleteAnalysisSchemeError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteAnalysisSchemeError::InvalidType(ref cause) => write!(f, "{}", cause),
            DeleteAnalysisSchemeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAnalysisSchemeError {}
/// Errors returned by DeleteDomain
#[derive(Debug, PartialEq)]
pub enum DeleteDomainError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
}

impl DeleteDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DeleteDomainError::Base(parsed_error.message))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DeleteDomainError::Internal(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDomainError::Base(ref cause) => write!(f, "{}", cause),
            DeleteDomainError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDomainError {}
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
}

impl DeleteExpressionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteExpressionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DeleteExpressionError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DeleteExpressionError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(DeleteExpressionError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DeleteExpressionError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteExpressionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteExpressionError::Base(ref cause) => write!(f, "{}", cause),
            DeleteExpressionError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteExpressionError::InvalidType(ref cause) => write!(f, "{}", cause),
            DeleteExpressionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteExpressionError {}
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
}

impl DeleteIndexFieldError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIndexFieldError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DeleteIndexFieldError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DeleteIndexFieldError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(DeleteIndexFieldError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DeleteIndexFieldError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteIndexFieldError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIndexFieldError::Base(ref cause) => write!(f, "{}", cause),
            DeleteIndexFieldError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteIndexFieldError::InvalidType(ref cause) => write!(f, "{}", cause),
            DeleteIndexFieldError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIndexFieldError {}
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
}

impl DeleteSuggesterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSuggesterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DeleteSuggesterError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DeleteSuggesterError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(DeleteSuggesterError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DeleteSuggesterError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteSuggesterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSuggesterError::Base(ref cause) => write!(f, "{}", cause),
            DeleteSuggesterError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteSuggesterError::InvalidType(ref cause) => write!(f, "{}", cause),
            DeleteSuggesterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSuggesterError {}
/// Errors returned by DescribeAnalysisSchemes
#[derive(Debug, PartialEq)]
pub enum DescribeAnalysisSchemesError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeAnalysisSchemesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAnalysisSchemesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DescribeAnalysisSchemesError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DescribeAnalysisSchemesError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(
                            DescribeAnalysisSchemesError::ResourceNotFound(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeAnalysisSchemesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAnalysisSchemesError::Base(ref cause) => write!(f, "{}", cause),
            DescribeAnalysisSchemesError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeAnalysisSchemesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAnalysisSchemesError {}
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
}

impl DescribeAvailabilityOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAvailabilityOptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DescribeAvailabilityOptionsError::Base(
                            parsed_error.message,
                        ))
                    }
                    "DisabledAction" => {
                        return RusotoError::Service(
                            DescribeAvailabilityOptionsError::DisabledOperation(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalException" => {
                        return RusotoError::Service(DescribeAvailabilityOptionsError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(DescribeAvailabilityOptionsError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            DescribeAvailabilityOptionsError::LimitExceeded(parsed_error.message),
                        )
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(
                            DescribeAvailabilityOptionsError::ResourceNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeAvailabilityOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAvailabilityOptionsError::Base(ref cause) => write!(f, "{}", cause),
            DescribeAvailabilityOptionsError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAvailabilityOptionsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeAvailabilityOptionsError::InvalidType(ref cause) => write!(f, "{}", cause),
            DescribeAvailabilityOptionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeAvailabilityOptionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAvailabilityOptionsError {}
/// Errors returned by DescribeDomainEndpointOptions
#[derive(Debug, PartialEq)]
pub enum DescribeDomainEndpointOptionsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request was rejected because it attempted an operation which is not enabled.</p>
    DisabledOperation(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because a resource limit has already been met.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeDomainEndpointOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDomainEndpointOptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DescribeDomainEndpointOptionsError::Base(
                            parsed_error.message,
                        ))
                    }
                    "DisabledAction" => {
                        return RusotoError::Service(
                            DescribeDomainEndpointOptionsError::DisabledOperation(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalException" => {
                        return RusotoError::Service(DescribeDomainEndpointOptionsError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            DescribeDomainEndpointOptionsError::LimitExceeded(parsed_error.message),
                        )
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(
                            DescribeDomainEndpointOptionsError::ResourceNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDomainEndpointOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDomainEndpointOptionsError::Base(ref cause) => write!(f, "{}", cause),
            DescribeDomainEndpointOptionsError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDomainEndpointOptionsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeDomainEndpointOptionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeDomainEndpointOptionsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDomainEndpointOptionsError {}
/// Errors returned by DescribeDomains
#[derive(Debug, PartialEq)]
pub enum DescribeDomainsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
}

impl DescribeDomainsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDomainsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DescribeDomainsError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DescribeDomainsError::Internal(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDomainsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDomainsError::Base(ref cause) => write!(f, "{}", cause),
            DescribeDomainsError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDomainsError {}
/// Errors returned by DescribeExpressions
#[derive(Debug, PartialEq)]
pub enum DescribeExpressionsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeExpressionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeExpressionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DescribeExpressionsError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DescribeExpressionsError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DescribeExpressionsError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeExpressionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeExpressionsError::Base(ref cause) => write!(f, "{}", cause),
            DescribeExpressionsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeExpressionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeExpressionsError {}
/// Errors returned by DescribeIndexFields
#[derive(Debug, PartialEq)]
pub enum DescribeIndexFieldsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeIndexFieldsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeIndexFieldsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DescribeIndexFieldsError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DescribeIndexFieldsError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DescribeIndexFieldsError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeIndexFieldsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeIndexFieldsError::Base(ref cause) => write!(f, "{}", cause),
            DescribeIndexFieldsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeIndexFieldsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeIndexFieldsError {}
/// Errors returned by DescribeScalingParameters
#[derive(Debug, PartialEq)]
pub enum DescribeScalingParametersError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeScalingParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScalingParametersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DescribeScalingParametersError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DescribeScalingParametersError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(
                            DescribeScalingParametersError::ResourceNotFound(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeScalingParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScalingParametersError::Base(ref cause) => write!(f, "{}", cause),
            DescribeScalingParametersError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeScalingParametersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeScalingParametersError {}
/// Errors returned by DescribeServiceAccessPolicies
#[derive(Debug, PartialEq)]
pub enum DescribeServiceAccessPoliciesError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeServiceAccessPoliciesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeServiceAccessPoliciesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DescribeServiceAccessPoliciesError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DescribeServiceAccessPoliciesError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(
                            DescribeServiceAccessPoliciesError::ResourceNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeServiceAccessPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServiceAccessPoliciesError::Base(ref cause) => write!(f, "{}", cause),
            DescribeServiceAccessPoliciesError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeServiceAccessPoliciesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeServiceAccessPoliciesError {}
/// Errors returned by DescribeSuggesters
#[derive(Debug, PartialEq)]
pub enum DescribeSuggestersError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeSuggestersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSuggestersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(DescribeSuggestersError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(DescribeSuggestersError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(DescribeSuggestersError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeSuggestersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSuggestersError::Base(ref cause) => write!(f, "{}", cause),
            DescribeSuggestersError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeSuggestersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSuggestersError {}
/// Errors returned by IndexDocuments
#[derive(Debug, PartialEq)]
pub enum IndexDocumentsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An internal error occurred while processing the request. If this problem persists, report an issue from the <a href="http://status.aws.amazon.com/" target="_blank">Service Health Dashboard</a>.</p>
    Internal(String),
    /// <p>The request was rejected because it attempted to reference a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl IndexDocumentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<IndexDocumentsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(IndexDocumentsError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(IndexDocumentsError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(IndexDocumentsError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for IndexDocumentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IndexDocumentsError::Base(ref cause) => write!(f, "{}", cause),
            IndexDocumentsError::Internal(ref cause) => write!(f, "{}", cause),
            IndexDocumentsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for IndexDocumentsError {}
/// Errors returned by ListDomainNames
#[derive(Debug, PartialEq)]
pub enum ListDomainNamesError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
}

impl ListDomainNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDomainNamesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(ListDomainNamesError::Base(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListDomainNamesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDomainNamesError::Base(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDomainNamesError {}
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
    /// <p>The request was rejected because it has invalid parameters.</p>
    Validation(String),
}

impl UpdateAvailabilityOptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAvailabilityOptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(UpdateAvailabilityOptionsError::Base(
                            parsed_error.message,
                        ))
                    }
                    "DisabledAction" => {
                        return RusotoError::Service(
                            UpdateAvailabilityOptionsError::DisabledOperation(parsed_error.message),
                        )
                    }
                    "InternalException" => {
                        return RusotoError::Service(UpdateAvailabilityOptionsError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(UpdateAvailabilityOptionsError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(UpdateAvailabilityOptionsError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(
                            UpdateAvailabilityOptionsError::ResourceNotFound(parsed_error.message),
                        )
                    }
                    "ValidationException" => {
                        return RusotoError::Service(UpdateAvailabilityOptionsError::Validation(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateAvailabilityOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAvailabilityOptionsError::Base(ref cause) => write!(f, "{}", cause),
            UpdateAvailabilityOptionsError::DisabledOperation(ref cause) => write!(f, "{}", cause),
            UpdateAvailabilityOptionsError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateAvailabilityOptionsError::InvalidType(ref cause) => write!(f, "{}", cause),
            UpdateAvailabilityOptionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateAvailabilityOptionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAvailabilityOptionsError::Validation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAvailabilityOptionsError {}
/// Errors returned by UpdateDomainEndpointOptions
#[derive(Debug, PartialEq)]
pub enum UpdateDomainEndpointOptionsError {
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
    /// <p>The request was rejected because it has invalid parameters.</p>
    Validation(String),
}

impl UpdateDomainEndpointOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDomainEndpointOptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(UpdateDomainEndpointOptionsError::Base(
                            parsed_error.message,
                        ))
                    }
                    "DisabledAction" => {
                        return RusotoError::Service(
                            UpdateDomainEndpointOptionsError::DisabledOperation(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalException" => {
                        return RusotoError::Service(UpdateDomainEndpointOptionsError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(UpdateDomainEndpointOptionsError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            UpdateDomainEndpointOptionsError::LimitExceeded(parsed_error.message),
                        )
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(
                            UpdateDomainEndpointOptionsError::ResourceNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ValidationException" => {
                        return RusotoError::Service(UpdateDomainEndpointOptionsError::Validation(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateDomainEndpointOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDomainEndpointOptionsError::Base(ref cause) => write!(f, "{}", cause),
            UpdateDomainEndpointOptionsError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDomainEndpointOptionsError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateDomainEndpointOptionsError::InvalidType(ref cause) => write!(f, "{}", cause),
            UpdateDomainEndpointOptionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDomainEndpointOptionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDomainEndpointOptionsError::Validation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDomainEndpointOptionsError {}
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
}

impl UpdateScalingParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateScalingParametersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(UpdateScalingParametersError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(UpdateScalingParametersError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(UpdateScalingParametersError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(UpdateScalingParametersError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(
                            UpdateScalingParametersError::ResourceNotFound(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateScalingParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateScalingParametersError::Base(ref cause) => write!(f, "{}", cause),
            UpdateScalingParametersError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateScalingParametersError::InvalidType(ref cause) => write!(f, "{}", cause),
            UpdateScalingParametersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateScalingParametersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateScalingParametersError {}
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
}

impl UpdateServiceAccessPoliciesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateServiceAccessPoliciesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BaseException" => {
                        return RusotoError::Service(UpdateServiceAccessPoliciesError::Base(
                            parsed_error.message,
                        ))
                    }
                    "InternalException" => {
                        return RusotoError::Service(UpdateServiceAccessPoliciesError::Internal(
                            parsed_error.message,
                        ))
                    }
                    "InvalidType" => {
                        return RusotoError::Service(UpdateServiceAccessPoliciesError::InvalidType(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            UpdateServiceAccessPoliciesError::LimitExceeded(parsed_error.message),
                        )
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(
                            UpdateServiceAccessPoliciesError::ResourceNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateServiceAccessPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateServiceAccessPoliciesError::Base(ref cause) => write!(f, "{}", cause),
            UpdateServiceAccessPoliciesError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateServiceAccessPoliciesError::InvalidType(ref cause) => write!(f, "{}", cause),
            UpdateServiceAccessPoliciesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateServiceAccessPoliciesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateServiceAccessPoliciesError {}
/// Trait representing the capabilities of the Amazon CloudSearch API. Amazon CloudSearch clients implement this trait.
#[async_trait]
pub trait CloudSearch {
    /// <p>Indexes the search suggestions. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html#configuring-suggesters">Configuring Suggesters</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn build_suggesters(
        &self,
        input: BuildSuggestersRequest,
    ) -> Result<BuildSuggestersResponse, RusotoError<BuildSuggestersError>>;

    /// <p>Creates a new search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/creating-domains.html" target="_blank">Creating a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> Result<CreateDomainResponse, RusotoError<CreateDomainError>>;

    /// <p>Configures an analysis scheme that can be applied to a <code>text</code> or <code>text-array</code> field to define language-specific text processing options. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn define_analysis_scheme(
        &self,
        input: DefineAnalysisSchemeRequest,
    ) -> Result<DefineAnalysisSchemeResponse, RusotoError<DefineAnalysisSchemeError>>;

    /// <p>Configures an <code><a>Expression</a></code> for the search domain. Used to create new expressions and modify existing ones. If the expression exists, the new configuration replaces the old one. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn define_expression(
        &self,
        input: DefineExpressionRequest,
    ) -> Result<DefineExpressionResponse, RusotoError<DefineExpressionError>>;

    /// <p>Configures an <code><a>IndexField</a></code> for the search domain. Used to create new fields and modify existing ones. You must specify the name of the domain you are configuring and an index field configuration. The index field configuration specifies a unique name, the index field type, and the options you want to configure for the field. The options you can specify depend on the <code><a>IndexFieldType</a></code>. If the field exists, the new configuration replaces the old one. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    async fn define_index_field(
        &self,
        input: DefineIndexFieldRequest,
    ) -> Result<DefineIndexFieldResponse, RusotoError<DefineIndexFieldError>>;

    /// <p>Configures a suggester for a domain. A suggester enables you to display possible matches before users finish typing their queries. When you configure a suggester, you must specify the name of the text field you want to search for possible matches and a unique name for the suggester. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn define_suggester(
        &self,
        input: DefineSuggesterRequest,
    ) -> Result<DefineSuggesterResponse, RusotoError<DefineSuggesterError>>;

    /// <p>Deletes an analysis scheme. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    async fn delete_analysis_scheme(
        &self,
        input: DeleteAnalysisSchemeRequest,
    ) -> Result<DeleteAnalysisSchemeResponse, RusotoError<DeleteAnalysisSchemeError>>;

    /// <p>Permanently deletes a search domain and all of its data. Once a domain has been deleted, it cannot be recovered. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/deleting-domains.html" target="_blank">Deleting a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    async fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> Result<DeleteDomainResponse, RusotoError<DeleteDomainError>>;

    /// <p>Removes an <code><a>Expression</a></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn delete_expression(
        &self,
        input: DeleteExpressionRequest,
    ) -> Result<DeleteExpressionResponse, RusotoError<DeleteExpressionError>>;

    /// <p>Removes an <code><a>IndexField</a></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn delete_index_field(
        &self,
        input: DeleteIndexFieldRequest,
    ) -> Result<DeleteIndexFieldResponse, RusotoError<DeleteIndexFieldError>>;

    /// <p>Deletes a suggester. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn delete_suggester(
        &self,
        input: DeleteSuggesterRequest,
    ) -> Result<DeleteSuggesterResponse, RusotoError<DeleteSuggesterError>>;

    /// <p>Gets the analysis schemes configured for a domain. An analysis scheme defines language-specific text processing options for a <code>text</code> field. Can be limited to specific analysis schemes by name. By default, shows all analysis schemes and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_analysis_schemes(
        &self,
        input: DescribeAnalysisSchemesRequest,
    ) -> Result<DescribeAnalysisSchemesResponse, RusotoError<DescribeAnalysisSchemesError>>;

    /// <p>Gets the availability options configured for a domain. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-availability-options.html" target="_blank">Configuring Availability Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_availability_options(
        &self,
        input: DescribeAvailabilityOptionsRequest,
    ) -> Result<DescribeAvailabilityOptionsResponse, RusotoError<DescribeAvailabilityOptionsError>>;

    /// <p>Returns the domain's endpoint options, specifically whether all requests to the domain must arrive over HTTPS. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-domain-endpoint-options.html" target="_blank">Configuring Domain Endpoint Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_domain_endpoint_options(
        &self,
        input: DescribeDomainEndpointOptionsRequest,
    ) -> Result<
        DescribeDomainEndpointOptionsResponse,
        RusotoError<DescribeDomainEndpointOptionsError>,
    >;

    /// <p>Gets information about the search domains owned by this account. Can be limited to specific domains. Shows all domains by default. To get the number of searchable documents in a domain, use the console or submit a <code>matchall</code> request to your domain's search endpoint: <code>q=matchall&amp;amp;q.parser=structured&amp;amp;size=0</code>. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Information about a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_domains(
        &self,
        input: DescribeDomainsRequest,
    ) -> Result<DescribeDomainsResponse, RusotoError<DescribeDomainsError>>;

    /// <p>Gets the expressions configured for the search domain. Can be limited to specific expressions by name. By default, shows all expressions and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_expressions(
        &self,
        input: DescribeExpressionsRequest,
    ) -> Result<DescribeExpressionsResponse, RusotoError<DescribeExpressionsError>>;

    /// <p>Gets information about the index fields configured for the search domain. Can be limited to specific fields by name. By default, shows all fields and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Domain Information</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_index_fields(
        &self,
        input: DescribeIndexFieldsRequest,
    ) -> Result<DescribeIndexFieldsResponse, RusotoError<DescribeIndexFieldsError>>;

    /// <p>Gets the scaling parameters configured for a domain. A domain's scaling parameters specify the desired search instance type and replication count. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-scaling-options.html" target="_blank">Configuring Scaling Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_scaling_parameters(
        &self,
        input: DescribeScalingParametersRequest,
    ) -> Result<DescribeScalingParametersResponse, RusotoError<DescribeScalingParametersError>>;

    /// <p>Gets information about the access policies that control access to the domain's document and search endpoints. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank">Configuring Access for a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_service_access_policies(
        &self,
        input: DescribeServiceAccessPoliciesRequest,
    ) -> Result<
        DescribeServiceAccessPoliciesResponse,
        RusotoError<DescribeServiceAccessPoliciesError>,
    >;

    /// <p>Gets the suggesters configured for a domain. A suggester enables you to display possible matches before users finish typing their queries. Can be limited to specific suggesters by name. By default, shows all suggesters and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_suggesters(
        &self,
        input: DescribeSuggestersRequest,
    ) -> Result<DescribeSuggestersResponse, RusotoError<DescribeSuggestersError>>;

    /// <p>Tells the search domain to start indexing its documents using the latest indexing options. This operation must be invoked to activate options whose <a>OptionStatus</a> is <code>RequiresIndexDocuments</code>.</p>
    async fn index_documents(
        &self,
        input: IndexDocumentsRequest,
    ) -> Result<IndexDocumentsResponse, RusotoError<IndexDocumentsError>>;

    /// <p>Lists all search domains owned by an account.</p>
    async fn list_domain_names(
        &self,
    ) -> Result<ListDomainNamesResponse, RusotoError<ListDomainNamesError>>;

    /// <p>Configures the availability options for a domain. Enabling the Multi-AZ option expands an Amazon CloudSearch domain to an additional Availability Zone in the same Region to increase fault tolerance in the event of a service disruption. Changes to the Multi-AZ option can take about half an hour to become active. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-availability-options.html" target="_blank">Configuring Availability Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn update_availability_options(
        &self,
        input: UpdateAvailabilityOptionsRequest,
    ) -> Result<UpdateAvailabilityOptionsResponse, RusotoError<UpdateAvailabilityOptionsError>>;

    /// <p>Updates the domain's endpoint options, specifically whether all requests to the domain must arrive over HTTPS. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-domain-endpoint-options.html" target="_blank">Configuring Domain Endpoint Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn update_domain_endpoint_options(
        &self,
        input: UpdateDomainEndpointOptionsRequest,
    ) -> Result<UpdateDomainEndpointOptionsResponse, RusotoError<UpdateDomainEndpointOptionsError>>;

    /// <p>Configures scaling parameters for a domain. A domain's scaling parameters specify the desired search instance type and replication count. Amazon CloudSearch will still automatically scale your domain based on the volume of data and traffic, but not below the desired instance type and replication count. If the Multi-AZ option is enabled, these values control the resources used per Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-scaling-options.html" target="_blank">Configuring Scaling Options</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    async fn update_scaling_parameters(
        &self,
        input: UpdateScalingParametersRequest,
    ) -> Result<UpdateScalingParametersResponse, RusotoError<UpdateScalingParametersError>>;

    /// <p>Configures the access rules that control access to the domain's document and search endpoints. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank"> Configuring Access for an Amazon CloudSearch Domain</a>.</p>
    async fn update_service_access_policies(
        &self,
        input: UpdateServiceAccessPoliciesRequest,
    ) -> Result<UpdateServiceAccessPoliciesResponse, RusotoError<UpdateServiceAccessPoliciesError>>;
}
/// A client for the Amazon CloudSearch API.
#[derive(Clone)]
pub struct CloudSearchClient {
    client: Client,
    region: region::Region,
}

impl CloudSearchClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudSearchClient {
        CloudSearchClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudSearchClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CloudSearchClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CloudSearchClient {
        CloudSearchClient { client, region }
    }
}

#[async_trait]
impl CloudSearch for CloudSearchClient {
    /// <p>Indexes the search suggestions. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html#configuring-suggesters">Configuring Suggesters</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn build_suggesters(
        &self,
        input: BuildSuggestersRequest,
    ) -> Result<BuildSuggestersResponse, RusotoError<BuildSuggestersError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "BuildSuggesters");
        params.put("Version", "2013-01-01");
        BuildSuggestersRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(BuildSuggestersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = BuildSuggestersResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = BuildSuggestersResponseDeserializer::deserialize(
                "BuildSuggestersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/creating-domains.html" target="_blank">Creating a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> Result<CreateDomainResponse, RusotoError<CreateDomainError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDomain");
        params.put("Version", "2013-01-01");
        CreateDomainRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateDomainError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateDomainResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                CreateDomainResponseDeserializer::deserialize("CreateDomainResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Configures an analysis scheme that can be applied to a <code>text</code> or <code>text-array</code> field to define language-specific text processing options. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn define_analysis_scheme(
        &self,
        input: DefineAnalysisSchemeRequest,
    ) -> Result<DefineAnalysisSchemeResponse, RusotoError<DefineAnalysisSchemeError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DefineAnalysisScheme");
        params.put("Version", "2013-01-01");
        DefineAnalysisSchemeRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DefineAnalysisSchemeError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DefineAnalysisSchemeResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DefineAnalysisSchemeResponseDeserializer::deserialize(
                "DefineAnalysisSchemeResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Configures an <code><a>Expression</a></code> for the search domain. Used to create new expressions and modify existing ones. If the expression exists, the new configuration replaces the old one. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn define_expression(
        &self,
        input: DefineExpressionRequest,
    ) -> Result<DefineExpressionResponse, RusotoError<DefineExpressionError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DefineExpression");
        params.put("Version", "2013-01-01");
        DefineExpressionRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DefineExpressionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DefineExpressionResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DefineExpressionResponseDeserializer::deserialize(
                "DefineExpressionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Configures an <code><a>IndexField</a></code> for the search domain. Used to create new fields and modify existing ones. You must specify the name of the domain you are configuring and an index field configuration. The index field configuration specifies a unique name, the index field type, and the options you want to configure for the field. The options you can specify depend on the <code><a>IndexFieldType</a></code>. If the field exists, the new configuration replaces the old one. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    async fn define_index_field(
        &self,
        input: DefineIndexFieldRequest,
    ) -> Result<DefineIndexFieldResponse, RusotoError<DefineIndexFieldError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DefineIndexField");
        params.put("Version", "2013-01-01");
        DefineIndexFieldRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DefineIndexFieldError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DefineIndexFieldResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DefineIndexFieldResponseDeserializer::deserialize(
                "DefineIndexFieldResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Configures a suggester for a domain. A suggester enables you to display possible matches before users finish typing their queries. When you configure a suggester, you must specify the name of the text field you want to search for possible matches and a unique name for the suggester. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn define_suggester(
        &self,
        input: DefineSuggesterRequest,
    ) -> Result<DefineSuggesterResponse, RusotoError<DefineSuggesterError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DefineSuggester");
        params.put("Version", "2013-01-01");
        DefineSuggesterRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DefineSuggesterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DefineSuggesterResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DefineSuggesterResponseDeserializer::deserialize(
                "DefineSuggesterResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes an analysis scheme. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    async fn delete_analysis_scheme(
        &self,
        input: DeleteAnalysisSchemeRequest,
    ) -> Result<DeleteAnalysisSchemeResponse, RusotoError<DeleteAnalysisSchemeError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteAnalysisScheme");
        params.put("Version", "2013-01-01");
        DeleteAnalysisSchemeRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteAnalysisSchemeError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteAnalysisSchemeResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteAnalysisSchemeResponseDeserializer::deserialize(
                "DeleteAnalysisSchemeResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Permanently deletes a search domain and all of its data. Once a domain has been deleted, it cannot be recovered. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/deleting-domains.html" target="_blank">Deleting a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    async fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> Result<DeleteDomainResponse, RusotoError<DeleteDomainError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDomain");
        params.put("Version", "2013-01-01");
        DeleteDomainRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteDomainError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteDomainResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                DeleteDomainResponseDeserializer::deserialize("DeleteDomainResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Removes an <code><a>Expression</a></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn delete_expression(
        &self,
        input: DeleteExpressionRequest,
    ) -> Result<DeleteExpressionResponse, RusotoError<DeleteExpressionError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteExpression");
        params.put("Version", "2013-01-01");
        DeleteExpressionRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteExpressionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteExpressionResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteExpressionResponseDeserializer::deserialize(
                "DeleteExpressionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Removes an <code><a>IndexField</a></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn delete_index_field(
        &self,
        input: DeleteIndexFieldRequest,
    ) -> Result<DeleteIndexFieldResponse, RusotoError<DeleteIndexFieldError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteIndexField");
        params.put("Version", "2013-01-01");
        DeleteIndexFieldRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteIndexFieldError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteIndexFieldResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteIndexFieldResponseDeserializer::deserialize(
                "DeleteIndexFieldResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes a suggester. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn delete_suggester(
        &self,
        input: DeleteSuggesterRequest,
    ) -> Result<DeleteSuggesterResponse, RusotoError<DeleteSuggesterError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteSuggester");
        params.put("Version", "2013-01-01");
        DeleteSuggesterRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteSuggesterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteSuggesterResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteSuggesterResponseDeserializer::deserialize(
                "DeleteSuggesterResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets the analysis schemes configured for a domain. An analysis scheme defines language-specific text processing options for a <code>text</code> field. Can be limited to specific analysis schemes by name. By default, shows all analysis schemes and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_analysis_schemes(
        &self,
        input: DescribeAnalysisSchemesRequest,
    ) -> Result<DescribeAnalysisSchemesResponse, RusotoError<DescribeAnalysisSchemesError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAnalysisSchemes");
        params.put("Version", "2013-01-01");
        DescribeAnalysisSchemesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeAnalysisSchemesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeAnalysisSchemesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeAnalysisSchemesResponseDeserializer::deserialize(
                "DescribeAnalysisSchemesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets the availability options configured for a domain. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-availability-options.html" target="_blank">Configuring Availability Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_availability_options(
        &self,
        input: DescribeAvailabilityOptionsRequest,
    ) -> Result<DescribeAvailabilityOptionsResponse, RusotoError<DescribeAvailabilityOptionsError>>
    {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAvailabilityOptions");
        params.put("Version", "2013-01-01");
        DescribeAvailabilityOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeAvailabilityOptionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeAvailabilityOptionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeAvailabilityOptionsResponseDeserializer::deserialize(
                "DescribeAvailabilityOptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the domain's endpoint options, specifically whether all requests to the domain must arrive over HTTPS. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-domain-endpoint-options.html" target="_blank">Configuring Domain Endpoint Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_domain_endpoint_options(
        &self,
        input: DescribeDomainEndpointOptionsRequest,
    ) -> Result<
        DescribeDomainEndpointOptionsResponse,
        RusotoError<DescribeDomainEndpointOptionsError>,
    > {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDomainEndpointOptions");
        params.put("Version", "2013-01-01");
        DescribeDomainEndpointOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDomainEndpointOptionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeDomainEndpointOptionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeDomainEndpointOptionsResponseDeserializer::deserialize(
                "DescribeDomainEndpointOptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about the search domains owned by this account. Can be limited to specific domains. Shows all domains by default. To get the number of searchable documents in a domain, use the console or submit a <code>matchall</code> request to your domain's search endpoint: <code>q=matchall&amp;amp;q.parser=structured&amp;amp;size=0</code>. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Information about a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_domains(
        &self,
        input: DescribeDomainsRequest,
    ) -> Result<DescribeDomainsResponse, RusotoError<DescribeDomainsError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDomains");
        params.put("Version", "2013-01-01");
        DescribeDomainsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDomainsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeDomainsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeDomainsResponseDeserializer::deserialize(
                "DescribeDomainsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets the expressions configured for the search domain. Can be limited to specific expressions by name. By default, shows all expressions and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_expressions(
        &self,
        input: DescribeExpressionsRequest,
    ) -> Result<DescribeExpressionsResponse, RusotoError<DescribeExpressionsError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeExpressions");
        params.put("Version", "2013-01-01");
        DescribeExpressionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeExpressionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeExpressionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeExpressionsResponseDeserializer::deserialize(
                "DescribeExpressionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about the index fields configured for the search domain. Can be limited to specific fields by name. By default, shows all fields and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Domain Information</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_index_fields(
        &self,
        input: DescribeIndexFieldsRequest,
    ) -> Result<DescribeIndexFieldsResponse, RusotoError<DescribeIndexFieldsError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeIndexFields");
        params.put("Version", "2013-01-01");
        DescribeIndexFieldsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeIndexFieldsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeIndexFieldsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeIndexFieldsResponseDeserializer::deserialize(
                "DescribeIndexFieldsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets the scaling parameters configured for a domain. A domain's scaling parameters specify the desired search instance type and replication count. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-scaling-options.html" target="_blank">Configuring Scaling Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_scaling_parameters(
        &self,
        input: DescribeScalingParametersRequest,
    ) -> Result<DescribeScalingParametersResponse, RusotoError<DescribeScalingParametersError>>
    {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeScalingParameters");
        params.put("Version", "2013-01-01");
        DescribeScalingParametersRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeScalingParametersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeScalingParametersResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeScalingParametersResponseDeserializer::deserialize(
                "DescribeScalingParametersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about the access policies that control access to the domain's document and search endpoints. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank">Configuring Access for a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_service_access_policies(
        &self,
        input: DescribeServiceAccessPoliciesRequest,
    ) -> Result<
        DescribeServiceAccessPoliciesResponse,
        RusotoError<DescribeServiceAccessPoliciesError>,
    > {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeServiceAccessPolicies");
        params.put("Version", "2013-01-01");
        DescribeServiceAccessPoliciesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeServiceAccessPoliciesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeServiceAccessPoliciesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeServiceAccessPoliciesResponseDeserializer::deserialize(
                "DescribeServiceAccessPoliciesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets the suggesters configured for a domain. A suggester enables you to display possible matches before users finish typing their queries. Can be limited to specific suggesters by name. By default, shows all suggesters and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn describe_suggesters(
        &self,
        input: DescribeSuggestersRequest,
    ) -> Result<DescribeSuggestersResponse, RusotoError<DescribeSuggestersError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeSuggesters");
        params.put("Version", "2013-01-01");
        DescribeSuggestersRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeSuggestersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeSuggestersResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeSuggestersResponseDeserializer::deserialize(
                "DescribeSuggestersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Tells the search domain to start indexing its documents using the latest indexing options. This operation must be invoked to activate options whose <a>OptionStatus</a> is <code>RequiresIndexDocuments</code>.</p>
    async fn index_documents(
        &self,
        input: IndexDocumentsRequest,
    ) -> Result<IndexDocumentsResponse, RusotoError<IndexDocumentsError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "IndexDocuments");
        params.put("Version", "2013-01-01");
        IndexDocumentsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(IndexDocumentsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = IndexDocumentsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = IndexDocumentsResponseDeserializer::deserialize(
                "IndexDocumentsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists all search domains owned by an account.</p>
    async fn list_domain_names(
        &self,
    ) -> Result<ListDomainNamesResponse, RusotoError<ListDomainNamesError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListDomainNames");
        params.put("Version", "2013-01-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListDomainNamesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListDomainNamesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListDomainNamesResponseDeserializer::deserialize(
                "ListDomainNamesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Configures the availability options for a domain. Enabling the Multi-AZ option expands an Amazon CloudSearch domain to an additional Availability Zone in the same Region to increase fault tolerance in the event of a service disruption. Changes to the Multi-AZ option can take about half an hour to become active. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-availability-options.html" target="_blank">Configuring Availability Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn update_availability_options(
        &self,
        input: UpdateAvailabilityOptionsRequest,
    ) -> Result<UpdateAvailabilityOptionsResponse, RusotoError<UpdateAvailabilityOptionsError>>
    {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateAvailabilityOptions");
        params.put("Version", "2013-01-01");
        UpdateAvailabilityOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateAvailabilityOptionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = UpdateAvailabilityOptionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = UpdateAvailabilityOptionsResponseDeserializer::deserialize(
                "UpdateAvailabilityOptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Updates the domain's endpoint options, specifically whether all requests to the domain must arrive over HTTPS. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-domain-endpoint-options.html" target="_blank">Configuring Domain Endpoint Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    async fn update_domain_endpoint_options(
        &self,
        input: UpdateDomainEndpointOptionsRequest,
    ) -> Result<UpdateDomainEndpointOptionsResponse, RusotoError<UpdateDomainEndpointOptionsError>>
    {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateDomainEndpointOptions");
        params.put("Version", "2013-01-01");
        UpdateDomainEndpointOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateDomainEndpointOptionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = UpdateDomainEndpointOptionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = UpdateDomainEndpointOptionsResponseDeserializer::deserialize(
                "UpdateDomainEndpointOptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Configures scaling parameters for a domain. A domain's scaling parameters specify the desired search instance type and replication count. Amazon CloudSearch will still automatically scale your domain based on the volume of data and traffic, but not below the desired instance type and replication count. If the Multi-AZ option is enabled, these values control the resources used per Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-scaling-options.html" target="_blank">Configuring Scaling Options</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    async fn update_scaling_parameters(
        &self,
        input: UpdateScalingParametersRequest,
    ) -> Result<UpdateScalingParametersResponse, RusotoError<UpdateScalingParametersError>> {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateScalingParameters");
        params.put("Version", "2013-01-01");
        UpdateScalingParametersRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateScalingParametersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = UpdateScalingParametersResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = UpdateScalingParametersResponseDeserializer::deserialize(
                "UpdateScalingParametersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Configures the access rules that control access to the domain's document and search endpoints. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank"> Configuring Access for an Amazon CloudSearch Domain</a>.</p>
    async fn update_service_access_policies(
        &self,
        input: UpdateServiceAccessPoliciesRequest,
    ) -> Result<UpdateServiceAccessPoliciesResponse, RusotoError<UpdateServiceAccessPoliciesError>>
    {
        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateServiceAccessPolicies");
        params.put("Version", "2013-01-01");
        UpdateServiceAccessPoliciesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateServiceAccessPoliciesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = UpdateServiceAccessPoliciesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = UpdateServiceAccessPoliciesResponseDeserializer::deserialize(
                "UpdateServiceAccessPoliciesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }
}
