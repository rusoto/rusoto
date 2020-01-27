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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// <p>A container for facet information. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Bucket {
    /// <p>The number of hits that contain the facet value in the specified facet field.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The facet value being counted.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A container for the calculated facet values and counts.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BucketInfo {
    /// <p>A list of the calculated facet values and counts.</p>
    #[serde(rename = "buckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<Bucket>>,
}

/// <p>A warning returned by the document service when an issue is discovered while processing an upload request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DocumentServiceWarning {
    /// <p>The description for a warning returned by the document service.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The statistics for a field calculated in the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FieldStats {
    /// <p>The number of documents that contain a value in the specified field in the result set.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The maximum value found in the specified field in the result set.</p> <p>If the field is numeric (<code>int</code>, <code>int-array</code>, <code>double</code>, or <code>double-array</code>), <code>max</code> is the string representation of a double-precision 64-bit floating point value. If the field is <code>date</code> or <code>date-array</code>, <code>max</code> is the string representation of a date with the format specified in <a href="http://tools.ietf.org/html/rfc3339">IETF RFC3339</a>: yyyy-mm-ddTHH:mm:ss.SSSZ.</p>
    #[serde(rename = "max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    /// <p>The average of the values found in the specified field in the result set.</p> <p>If the field is numeric (<code>int</code>, <code>int-array</code>, <code>double</code>, or <code>double-array</code>), <code>mean</code> is the string representation of a double-precision 64-bit floating point value. If the field is <code>date</code> or <code>date-array</code>, <code>mean</code> is the string representation of a date with the format specified in <a href="http://tools.ietf.org/html/rfc3339">IETF RFC3339</a>: yyyy-mm-ddTHH:mm:ss.SSSZ.</p>
    #[serde(rename = "mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean: Option<String>,
    /// <p>The minimum value found in the specified field in the result set.</p> <p>If the field is numeric (<code>int</code>, <code>int-array</code>, <code>double</code>, or <code>double-array</code>), <code>min</code> is the string representation of a double-precision 64-bit floating point value. If the field is <code>date</code> or <code>date-array</code>, <code>min</code> is the string representation of a date with the format specified in <a href="http://tools.ietf.org/html/rfc3339">IETF RFC3339</a>: yyyy-mm-ddTHH:mm:ss.SSSZ.</p>
    #[serde(rename = "min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    /// <p>The number of documents that do not contain a value in the specified field in the result set.</p>
    #[serde(rename = "missing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<i64>,
    /// <p>The standard deviation of the values in the specified field in the result set.</p>
    #[serde(rename = "stddev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stddev: Option<f64>,
    /// <p>The sum of the field values across the documents in the result set. <code>null</code> for date fields.</p>
    #[serde(rename = "sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    /// <p>The sum of all field values in the result set squared.</p>
    #[serde(rename = "sumOfSquares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_of_squares: Option<f64>,
}

/// <p>Information about a document that matches the search request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Hit {
    /// <p>The expressions returned from a document that matches the search request.</p>
    #[serde(rename = "exprs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exprs: Option<::std::collections::HashMap<String, String>>,
    /// <p>The fields returned from a document that matches the search request.</p>
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The highlights returned from a document that matches the search request.</p>
    #[serde(rename = "highlights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<::std::collections::HashMap<String, String>>,
    /// <p>The document ID of a document that matches the search request.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The collection of documents that match the search request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Hits {
    /// <p>A cursor that can be used to retrieve the next set of matching documents when you want to page through a large result set.</p>
    #[serde(rename = "cursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// <p>The total number of documents that match the search request.</p>
    #[serde(rename = "found")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub found: Option<i64>,
    /// <p>A document that matches the search request.</p>
    #[serde(rename = "hit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit: Option<Vec<Hit>>,
    /// <p>The index of the first matching document.</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
}

/// <p>Container for the parameters to the <code>Search</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchRequest {
    /// <p>Retrieves a cursor value you can use to page through large result sets. Use the <code>size</code> parameter to control the number of hits to include in each response. You can specify either the <code>cursor</code> or <code>start</code> parameter in a request; they are mutually exclusive. To get the first cursor, set the cursor value to <code>initial</code>. In subsequent requests, specify the cursor value returned in the hits section of the response. </p> <p>For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/paginating-results.html">Paginating Results</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    #[serde(rename = "cursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// <p>Defines one or more numeric expressions that can be used to sort results or specify search or filter criteria. You can also specify expressions as return fields. </p> <p>You specify the expressions in JSON using the form <code>{"EXPRESSIONNAME":"EXPRESSION"}</code>. You can define and use multiple expressions in a search request. For example:</p> <p><code> {"expression1":"_score*rating", "expression2":"(1/rank)*year"} </code> </p> <p>For information about the variables, operators, and functions you can use in expressions, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html#writing-expressions">Writing Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    #[serde(rename = "expr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    /// <p>Specifies one or more fields for which to get facet information, and options that control how the facet information is returned. Each specified field must be facet-enabled in the domain configuration. The fields and options are specified in JSON using the form <code>{"FIELD":{"OPTION":VALUE,"OPTION:"STRING"},"FIELD":{"OPTION":VALUE,"OPTION":"STRING"}}</code>.</p> <p>You can specify the following faceting options:</p> <ul> <li> <p><code>buckets</code> specifies an array of the facet values or ranges to count. Ranges are specified using the same syntax that you use to search for a range of values. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/searching-ranges.html"> Searching for a Range of Values</a> in the <i>Amazon CloudSearch Developer Guide</i>. Buckets are returned in the order they are specified in the request. The <code>sort</code> and <code>size</code> options are not valid if you specify <code>buckets</code>.</p> </li> <li> <p><code>size</code> specifies the maximum number of facets to include in the results. By default, Amazon CloudSearch returns counts for the top 10. The <code>size</code> parameter is only valid when you specify the <code>sort</code> option; it cannot be used in conjunction with <code>buckets</code>.</p> </li> <li> <p><code>sort</code> specifies how you want to sort the facets in the results: <code>bucket</code> or <code>count</code>. Specify <code>bucket</code> to sort alphabetically or numerically by facet value (in ascending order). Specify <code>count</code> to sort by the facet counts computed for each facet value (in descending order). To retrieve facet counts for particular values or ranges of values, use the <code>buckets</code> option instead of <code>sort</code>. </p> </li> </ul> <p>If no facet options are specified, facet counts are computed for all field values, the facets are sorted by facet count, and the top 10 facets are returned in the results.</p> <p>To count particular buckets of values, use the <code>buckets</code> option. For example, the following request uses the <code>buckets</code> option to calculate and return facet counts by decade.</p> <p><code> {"year":{"buckets":["[1970,1979]","[1980,1989]","[1990,1999]","[2000,2009]","[2010,}"]}} </code></p> <p>To sort facets by facet count, use the <code>count</code> option. For example, the following request sets the <code>sort</code> option to <code>count</code> to sort the facet values by facet count, with the facet values that have the most matching documents listed first. Setting the <code>size</code> option to 3 returns only the top three facet values.</p> <p><code> {"year":{"sort":"count","size":3}} </code></p> <p>To sort the facets by value, use the <code>bucket</code> option. For example, the following request sets the <code>sort</code> option to <code>bucket</code> to sort the facet values numerically by year, with earliest year listed first. </p> <p><code> {"year":{"sort":"bucket"}} </code></p> <p>For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/faceting.html">Getting and Using Facet Information</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    #[serde(rename = "facet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet: Option<String>,
    /// <p>Specifies a structured query that filters the results of a search without affecting how the results are scored and sorted. You use <code>filterQuery</code> in conjunction with the <code>query</code> parameter to filter the documents that match the constraints specified in the <code>query</code> parameter. Specifying a filter controls only which matching documents are included in the results, it has no effect on how they are scored and sorted. The <code>filterQuery</code> parameter supports the full structured query syntax. </p> <p>For more information about using filters, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/filtering-results.html">Filtering Matching Documents</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    #[serde(rename = "filterQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_query: Option<String>,
    /// <p>Retrieves highlights for matches in the specified <code>text</code> or <code>text-array</code> fields. Each specified field must be highlight enabled in the domain configuration. The fields and options are specified in JSON using the form <code>{"FIELD":{"OPTION":VALUE,"OPTION:"STRING"},"FIELD":{"OPTION":VALUE,"OPTION":"STRING"}}</code>.</p> <p>You can specify the following highlight options:</p> <ul> <li> <code>format</code>: specifies the format of the data in the text field: <code>text</code> or <code>html</code>. When data is returned as HTML, all non-alphanumeric characters are encoded. The default is <code>html</code>. </li> <li> <code>max_phrases</code>: specifies the maximum number of occurrences of the search term(s) you want to highlight. By default, the first occurrence is highlighted. </li> <li> <code>pre_tag</code>: specifies the string to prepend to an occurrence of a search term. The default for HTML highlights is <code>&amp;lt;em&amp;gt;</code>. The default for text highlights is <code>*</code>. </li> <li> <code>post_tag</code>: specifies the string to append to an occurrence of a search term. The default for HTML highlights is <code>&amp;lt;/em&amp;gt;</code>. The default for text highlights is <code>*</code>. </li> </ul> <p>If no highlight options are specified for a field, the returned field text is treated as HTML and the first match is highlighted with emphasis tags: <code>&amp;lt;em&gt;search-term&amp;lt;/em&amp;gt;</code>.</p> <p>For example, the following request retrieves highlights for the <code>actors</code> and <code>title</code> fields.</p> <p> <code>{ "actors": {}, "title": {"format": "text","max_phrases": 2,"pre_tag": "<b>","post_tag": "</b>"} }</code></p>
    #[serde(rename = "highlight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<String>,
    /// <p>Enables partial results to be returned if one or more index partitions are unavailable. When your search index is partitioned across multiple search instances, by default Amazon CloudSearch only returns results if every partition can be queried. This means that the failure of a single search instance can result in 5xx (internal server) errors. When you enable partial results, Amazon CloudSearch returns whatever results are available and includes the percentage of documents searched in the search results (percent-searched). This enables you to more gracefully degrade your users' search experience. For example, rather than displaying no results, you could display the partial results and a message indicating that the results might be incomplete due to a temporary system outage.</p>
    #[serde(rename = "partial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
    /// <p>Specifies the search criteria for the request. How you specify the search criteria depends on the query parser used for the request and the parser options specified in the <code>queryOptions</code> parameter. By default, the <code>simple</code> query parser is used to process requests. To use the <code>structured</code>, <code>lucene</code>, or <code>dismax</code> query parser, you must also specify the <code>queryParser</code> parameter. </p> <p>For more information about specifying search criteria, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/searching.html">Searching Your Data</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    #[serde(rename = "query")]
    pub query: String,
    /// <p><p>Configures options for the query parser specified in the <code>queryParser</code> parameter. You specify the options in JSON using the following form <code>{&quot;OPTION1&quot;:&quot;VALUE1&quot;,&quot;OPTION2&quot;:VALUE2&quot;...&quot;OPTIONN&quot;:&quot;VALUEN&quot;}.</code></p> <p>The options you can configure vary according to which parser you use:</p> <ul> <li><code>defaultOperator</code>: The default operator used to combine individual terms in the search string. For example: <code>defaultOperator: &#39;or&#39;</code>. For the <code>dismax</code> parser, you specify a percentage that represents the percentage of terms in the search string (rounded down) that must match, rather than a default operator. A value of <code>0%</code> is the equivalent to OR, and a value of <code>100%</code> is equivalent to AND. The percentage must be specified as a value in the range 0-100 followed by the percent (%) symbol. For example, <code>defaultOperator: 50%</code>. Valid values: <code>and</code>, <code>or</code>, a percentage in the range 0%-100% (<code>dismax</code>). Default: <code>and</code> (<code>simple</code>, <code>structured</code>, <code>lucene</code>) or <code>100</code> (<code>dismax</code>). Valid for: <code>simple</code>, <code>structured</code>, <code>lucene</code>, and <code>dismax</code>.</li> <li><code>fields</code>: An array of the fields to search when no fields are specified in a search. If no fields are specified in a search and this option is not specified, all text and text-array fields are searched. You can specify a weight for each field to control the relative importance of each field when Amazon CloudSearch calculates relevance scores. To specify a field weight, append a caret (<code>^</code>) symbol and the weight to the field name. For example, to boost the importance of the <code>title</code> field over the <code>description</code> field you could specify: <code>&quot;fields&quot;:[&quot;title^5&quot;,&quot;description&quot;]</code>. Valid values: The name of any configured field and an optional numeric value greater than zero. Default: All <code>text</code> and <code>text-array</code> fields. Valid for: <code>simple</code>, <code>structured</code>, <code>lucene</code>, and <code>dismax</code>.</li> <li><code>operators</code>: An array of the operators or special characters you want to disable for the simple query parser. If you disable the <code>and</code>, <code>or</code>, or <code>not</code> operators, the corresponding operators (<code>+</code>, <code>|</code>, <code>-</code>) have no special meaning and are dropped from the search string. Similarly, disabling <code>prefix</code> disables the wildcard operator (<code>*</code>) and disabling <code>phrase</code> disables the ability to search for phrases by enclosing phrases in double quotes. Disabling precedence disables the ability to control order of precedence using parentheses. Disabling <code>near</code> disables the ability to use the ~ operator to perform a sloppy phrase search. Disabling the <code>fuzzy</code> operator disables the ability to use the ~ operator to perform a fuzzy search. <code>escape</code> disables the ability to use a backslash (<code>&bsol;</code>) to escape special characters within the search string. Disabling whitespace is an advanced option that prevents the parser from tokenizing on whitespace, which can be useful for Vietnamese. (It prevents Vietnamese words from being split incorrectly.) For example, you could disable all operators other than the phrase operator to support just simple term and phrase queries: <code>&quot;operators&quot;:[&quot;and&quot;,&quot;not&quot;,&quot;or&quot;, &quot;prefix&quot;]</code>. Valid values: <code>and</code>, <code>escape</code>, <code>fuzzy</code>, <code>near</code>, <code>not</code>, <code>or</code>, <code>phrase</code>, <code>precedence</code>, <code>prefix</code>, <code>whitespace</code>. Default: All operators and special characters are enabled. Valid for: <code>simple</code>.</li> <li><code>phraseFields</code>: An array of the <code>text</code> or <code>text-array</code> fields you want to use for phrase searches. When the terms in the search string appear in close proximity within a field, the field scores higher. You can specify a weight for each field to boost that score. The <code>phraseSlop</code> option controls how much the matches can deviate from the search string and still be boosted. To specify a field weight, append a caret (<code>^</code>) symbol and the weight to the field name. For example, to boost phrase matches in the <code>title</code> field over the <code>abstract</code> field, you could specify: <code>&quot;phraseFields&quot;:[&quot;title^3&quot;, &quot;plot&quot;]</code> Valid values: The name of any <code>text</code> or <code>text-array</code> field and an optional numeric value greater than zero. Default: No fields. If you don&#39;t specify any fields with <code>phraseFields</code>, proximity scoring is disabled even if <code>phraseSlop</code> is specified. Valid for: <code>dismax</code>.</li> <li><code>phraseSlop</code>: An integer value that specifies how much matches can deviate from the search phrase and still be boosted according to the weights specified in the <code>phraseFields</code> option; for example, <code>phraseSlop: 2</code>. You must also specify <code>phraseFields</code> to enable proximity scoring. Valid values: positive integers. Default: 0. Valid for: <code>dismax</code>.</li> <li><code>explicitPhraseSlop</code>: An integer value that specifies how much a match can deviate from the search phrase when the phrase is enclosed in double quotes in the search string. (Phrases that exceed this proximity distance are not considered a match.) For example, to specify a slop of three for dismax phrase queries, you would specify <code>&quot;explicitPhraseSlop&quot;:3</code>. Valid values: positive integers. Default: 0. Valid for: <code>dismax</code>.</li> <li><code>tieBreaker</code>: When a term in the search string is found in a document&#39;s field, a score is calculated for that field based on how common the word is in that field compared to other documents. If the term occurs in multiple fields within a document, by default only the highest scoring field contributes to the document&#39;s overall score. You can specify a <code>tieBreaker</code> value to enable the matches in lower-scoring fields to contribute to the document&#39;s score. That way, if two documents have the same max field score for a particular term, the score for the document that has matches in more fields will be higher. The formula for calculating the score with a tieBreaker is <code>(max field score) + (tieBreaker) * (sum of the scores for the rest of the matching fields)</code>. Set <code>tieBreaker</code> to 0 to disregard all but the highest scoring field (pure max): <code>&quot;tieBreaker&quot;:0</code>. Set to 1 to sum the scores from all fields (pure sum): <code>&quot;tieBreaker&quot;:1</code>. Valid values: 0.0 to 1.0. Default: 0.0. Valid for: <code>dismax</code>. </li> </ul></p>
    #[serde(rename = "queryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_options: Option<String>,
    /// <p><p>Specifies which query parser to use to process the request. If <code>queryParser</code> is not specified, Amazon CloudSearch uses the <code>simple</code> query parser. </p> <p>Amazon CloudSearch supports four query parsers:</p> <ul> <li> <code>simple</code>: perform simple searches of <code>text</code> and <code>text-array</code> fields. By default, the <code>simple</code> query parser searches all <code>text</code> and <code>text-array</code> fields. You can specify which fields to search by with the <code>queryOptions</code> parameter. If you prefix a search term with a plus sign (+) documents must contain the term to be considered a match. (This is the default, unless you configure the default operator with the <code>queryOptions</code> parameter.) You can use the <code>-</code> (NOT), <code>|</code> (OR), and <code>*</code> (wildcard) operators to exclude particular terms, find results that match any of the specified terms, or search for a prefix. To search for a phrase rather than individual terms, enclose the phrase in double quotes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/searching-text.html">Searching for Text</a> in the <i>Amazon CloudSearch Developer Guide</i>. </li> <li> <code>structured</code>: perform advanced searches by combining multiple expressions to define the search criteria. You can also search within particular fields, search for values and ranges of values, and use advanced options such as term boosting, <code>matchall</code>, and <code>near</code>. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/searching-compound-queries.html">Constructing Compound Queries</a> in the <i>Amazon CloudSearch Developer Guide</i>. </li> <li> <code>lucene</code>: search using the Apache Lucene query parser syntax. For more information, see <a href="http://lucene.apache.org/core/4_6_0/queryparser/org/apache/lucene/queryparser/classic/package-summary.html#package_description">Apache Lucene Query Parser Syntax</a>. </li> <li> <code>dismax</code>: search using the simplified subset of the Apache Lucene query parser syntax defined by the DisMax query parser. For more information, see <a href="http://wiki.apache.org/solr/DisMaxQParserPlugin#Query_Syntax">DisMax Query Parser Syntax</a>. </li> </ul></p>
    #[serde(rename = "queryParser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parser: Option<String>,
    /// <p>Specifies the field and expression values to include in the response. Multiple fields or expressions are specified as a comma-separated list. By default, a search response includes all return enabled fields (<code>_all_fields</code>). To return only the document IDs for the matching documents, specify <code>_no_fields</code>. To retrieve the relevance score calculated for each document, specify <code>_score</code>. </p>
    #[serde(rename = "return")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_: Option<String>,
    /// <p>Specifies the maximum number of search hits to include in the response. </p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// <p>Specifies the fields or custom expressions to use to sort the search results. Multiple fields or expressions are specified as a comma-separated list. You must specify the sort direction (<code>asc</code> or <code>desc</code>) for each field; for example, <code>year desc,title asc</code>. To use a field to sort results, the field must be sort-enabled in the domain configuration. Array type fields cannot be used for sorting. If no <code>sort</code> parameter is specified, results are sorted by their default relevance scores in descending order: <code>_score desc</code>. You can also sort by document ID (<code>_id asc</code>) and version (<code>_version desc</code>).</p> <p>For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/sorting-results.html">Sorting Results</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// <p>Specifies the offset of the first search hit you want to return. Note that the result set is zero-based; the first result is at index 0. You can specify either the <code>start</code> or <code>cursor</code> parameter in a request, they are mutually exclusive. </p> <p>For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/paginating-results.html">Paginating Results</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    /// <p>Specifies one or more fields for which to get statistics information. Each specified field must be facet-enabled in the domain configuration. The fields are specified in JSON using the form:</p> <code>{"FIELD-A":{},"FIELD-B":{}}</code> <p>There are currently no options supported for statistics.</p>
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<String>,
}

/// <p>The result of a <code>Search</code> request. Contains the documents that match the specified search criteria and any requested fields, highlights, and facet information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchResponse {
    /// <p>The requested facet information.</p>
    #[serde(rename = "facets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<::std::collections::HashMap<String, BucketInfo>>,
    /// <p>The documents that match the search criteria.</p>
    #[serde(rename = "hits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits: Option<Hits>,
    /// <p>The requested field statistics information.</p>
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<::std::collections::HashMap<String, FieldStats>>,
    /// <p>The status information returned for the search request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SearchStatus>,
}

/// <p>Contains the resource id (<code>rid</code>) and the time it took to process the request (<code>timems</code>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchStatus {
    /// <p>The encrypted resource ID for the request.</p>
    #[serde(rename = "rid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
    /// <p>How long it took to process the request, in milliseconds.</p>
    #[serde(rename = "timems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timems: Option<i64>,
}

/// <p>Container for the suggestion information returned in a <code>SuggestResponse</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SuggestModel {
    /// <p>The number of documents that were found to match the query string.</p>
    #[serde(rename = "found")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub found: Option<i64>,
    /// <p>The query string specified in the suggest request.</p>
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// <p>The documents that match the query string.</p>
    #[serde(rename = "suggestions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<SuggestionMatch>>,
}

/// <p>Container for the parameters to the <code>Suggest</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SuggestRequest {
    /// <p>Specifies the string for which you want to get suggestions.</p>
    #[serde(rename = "query")]
    pub query: String,
    /// <p>Specifies the maximum number of suggestions to return. </p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// <p>Specifies the name of the suggester to use to find suggested matches.</p>
    #[serde(rename = "suggester")]
    pub suggester: String,
}

/// <p>Contains the response to a <code>Suggest</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SuggestResponse {
    /// <p>The status of a <code>SuggestRequest</code>. Contains the resource ID (<code>rid</code>) and how long it took to process the request (<code>timems</code>).</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SuggestStatus>,
    /// <p>Container for the matching search suggestion information.</p>
    #[serde(rename = "suggest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggest: Option<SuggestModel>,
}

/// <p>Contains the resource id (<code>rid</code>) and the time it took to process the request (<code>timems</code>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SuggestStatus {
    /// <p>The encrypted resource ID for the request.</p>
    #[serde(rename = "rid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
    /// <p>How long it took to process the request, in milliseconds.</p>
    #[serde(rename = "timems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timems: Option<i64>,
}

/// <p>An autocomplete suggestion that matches the query string specified in a <code>SuggestRequest</code>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SuggestionMatch {
    /// <p>The document ID of the suggested document.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The relevance score of a suggested match.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
    /// <p>The string that matches the query string specified in the <code>SuggestRequest</code>. </p>
    #[serde(rename = "suggestion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

/// <p>Container for the parameters to the <code>UploadDocuments</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UploadDocumentsRequest {
    /// <p><p>The format of the batch you are uploading. Amazon CloudSearch supports two document batch formats:</p> <ul> <li>application/json</li> <li>application/xml</li> </ul></p>
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// <p>A batch of documents formatted in JSON or HTML.</p>
    #[serde(rename = "documents")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub documents: bytes::Bytes,
}

/// <p>Contains the response to an <code>UploadDocuments</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UploadDocumentsResponse {
    /// <p>The number of documents that were added to the search domain.</p>
    #[serde(rename = "adds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adds: Option<i64>,
    /// <p>The number of documents that were deleted from the search domain.</p>
    #[serde(rename = "deletes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<i64>,
    /// <p>The status of an <code>UploadDocumentsRequest</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Any warnings returned by the document service about the documents being uploaded.</p>
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<DocumentServiceWarning>>,
}

/// Errors returned by Search
#[derive(Debug, PartialEq)]
pub enum SearchError {
    /// <p>Information about any problems encountered while processing a search request.</p>
    Search(String),
}

impl SearchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "SearchException" => return RusotoError::Service(SearchError::Search(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchError::Search(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchError {}
/// Errors returned by Suggest
#[derive(Debug, PartialEq)]
pub enum SuggestError {
    /// <p>Information about any problems encountered while processing a search request.</p>
    Search(String),
}

impl SuggestError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SuggestError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "SearchException" => return RusotoError::Service(SuggestError::Search(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SuggestError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuggestError::Search(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SuggestError {}
/// Errors returned by UploadDocuments
#[derive(Debug, PartialEq)]
pub enum UploadDocumentsError {
    /// <p>Information about any problems encountered while processing an upload request.</p>
    DocumentService(String),
}

impl UploadDocumentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UploadDocumentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DocumentServiceException" => {
                    return RusotoError::Service(UploadDocumentsError::DocumentService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UploadDocumentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UploadDocumentsError::DocumentService(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UploadDocumentsError {}
/// Trait representing the capabilities of the Amazon CloudSearch Domain API. Amazon CloudSearch Domain clients implement this trait.
#[async_trait]
pub trait CloudSearchDomain {
    /// <p>Retrieves a list of documents that match the specified search criteria. How you specify the search criteria depends on which query parser you use. Amazon CloudSearch supports four query parsers:</p> <ul> <li><code>simple</code>: search all <code>text</code> and <code>text-array</code> fields for the specified string. Search for phrases, individual terms, and prefixes. </li> <li><code>structured</code>: search specific fields, construct compound queries using Boolean operators, and use advanced features such as term boosting and proximity searching.</li> <li><code>lucene</code>: specify search criteria using the Apache Lucene query parser syntax.</li> <li><code>dismax</code>: specify search criteria using the simplified subset of the Apache Lucene query parser syntax defined by the DisMax query parser.</li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/searching.html">Searching Your Data</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p> <p>The endpoint for submitting <code>Search</code> requests is domain-specific. You submit search requests to a domain's search endpoint. To get the search endpoint for your domain, use the Amazon CloudSearch configuration service <code>DescribeDomains</code> action. A domain's endpoints are also displayed on the domain dashboard in the Amazon CloudSearch console. </p>
    async fn search(
        &self,
        input: SearchRequest,
    ) -> Result<SearchResponse, RusotoError<SearchError>>;

    /// <p>Retrieves autocomplete suggestions for a partial query string. You can use suggestions enable you to display likely matches before users finish typing. In Amazon CloudSearch, suggestions are based on the contents of a particular text field. When you request suggestions, Amazon CloudSearch finds all of the documents whose values in the suggester field start with the specified query string. The beginning of the field must match the query string to be considered a match. </p> <p>For more information about configuring suggesters and retrieving suggestions, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html">Getting Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p> <p>The endpoint for submitting <code>Suggest</code> requests is domain-specific. You submit suggest requests to a domain's search endpoint. To get the search endpoint for your domain, use the Amazon CloudSearch configuration service <code>DescribeDomains</code> action. A domain's endpoints are also displayed on the domain dashboard in the Amazon CloudSearch console. </p>
    async fn suggest(
        &self,
        input: SuggestRequest,
    ) -> Result<SuggestResponse, RusotoError<SuggestError>>;

    /// <p>Posts a batch of documents to a search domain for indexing. A document batch is a collection of add and delete operations that represent the documents you want to add, update, or delete from your domain. Batches can be described in either JSON or XML. Each item that you want Amazon CloudSearch to return as a search result (such as a product) is represented as a document. Every document has a unique ID and one or more fields that contain the data that you want to search and return in results. Individual documents cannot contain more than 1 MB of data. The entire batch cannot exceed 5 MB. To get the best possible upload performance, group add and delete operations in batches that are close the 5 MB limit. Submitting a large volume of single-document batches can overload a domain's document service. </p> <p>The endpoint for submitting <code>UploadDocuments</code> requests is domain-specific. To get the document endpoint for your domain, use the Amazon CloudSearch configuration service <code>DescribeDomains</code> action. A domain's endpoints are also displayed on the domain dashboard in the Amazon CloudSearch console. </p> <p>For more information about formatting your data for Amazon CloudSearch, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/preparing-data.html">Preparing Your Data</a> in the <i>Amazon CloudSearch Developer Guide</i>. For more information about uploading data for indexing, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/uploading-data.html">Uploading Data</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    async fn upload_documents(
        &self,
        input: UploadDocumentsRequest,
    ) -> Result<UploadDocumentsResponse, RusotoError<UploadDocumentsError>>;
}
/// A client for the Amazon CloudSearch Domain API.
#[derive(Clone)]
pub struct CloudSearchDomainClient {
    client: Client,
    region: region::Region,
}

impl CloudSearchDomainClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudSearchDomainClient {
        CloudSearchDomainClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudSearchDomainClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CloudSearchDomainClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CloudSearchDomainClient {
        CloudSearchDomainClient { client, region }
    }
}

#[async_trait]
impl CloudSearchDomain for CloudSearchDomainClient {
    /// <p>Retrieves a list of documents that match the specified search criteria. How you specify the search criteria depends on which query parser you use. Amazon CloudSearch supports four query parsers:</p> <ul> <li><code>simple</code>: search all <code>text</code> and <code>text-array</code> fields for the specified string. Search for phrases, individual terms, and prefixes. </li> <li><code>structured</code>: search specific fields, construct compound queries using Boolean operators, and use advanced features such as term boosting and proximity searching.</li> <li><code>lucene</code>: specify search criteria using the Apache Lucene query parser syntax.</li> <li><code>dismax</code>: specify search criteria using the simplified subset of the Apache Lucene query parser syntax defined by the DisMax query parser.</li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/searching.html">Searching Your Data</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p> <p>The endpoint for submitting <code>Search</code> requests is domain-specific. You submit search requests to a domain's search endpoint. To get the search endpoint for your domain, use the Amazon CloudSearch configuration service <code>DescribeDomains</code> action. A domain's endpoints are also displayed on the domain dashboard in the Amazon CloudSearch console. </p>
    async fn search(
        &self,
        input: SearchRequest,
    ) -> Result<SearchResponse, RusotoError<SearchError>> {
        let request_uri = "/2013-01-01/search";

        let mut request = SignedRequest::new("GET", "cloudsearch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("cloudsearchdomain".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.cursor {
            params.put("cursor", x);
        }
        if let Some(ref x) = input.expr {
            params.put("expr", x);
        }
        if let Some(ref x) = input.facet {
            params.put("facet", x);
        }
        if let Some(ref x) = input.filter_query {
            params.put("fq", x);
        }
        if let Some(ref x) = input.highlight {
            params.put("highlight", x);
        }
        if let Some(ref x) = input.partial {
            params.put("partial", x);
        }
        params.put("q", &input.query);
        if let Some(ref x) = input.query_options {
            params.put("q.options", x);
        }
        if let Some(ref x) = input.query_parser {
            params.put("q.parser", x);
        }
        if let Some(ref x) = input.return_ {
            params.put("return", x);
        }
        if let Some(ref x) = input.size {
            params.put("size", x);
        }
        if let Some(ref x) = input.sort {
            params.put("sort", x);
        }
        if let Some(ref x) = input.start {
            params.put("start", x);
        }
        if let Some(ref x) = input.stats {
            params.put("stats", x);
        }
        params.put("format", "sdk");
        params.put("pretty", "true");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<SearchResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SearchError::from_response(response))
        }
    }

    /// <p>Retrieves autocomplete suggestions for a partial query string. You can use suggestions enable you to display likely matches before users finish typing. In Amazon CloudSearch, suggestions are based on the contents of a particular text field. When you request suggestions, Amazon CloudSearch finds all of the documents whose values in the suggester field start with the specified query string. The beginning of the field must match the query string to be considered a match. </p> <p>For more information about configuring suggesters and retrieving suggestions, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html">Getting Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p> <p>The endpoint for submitting <code>Suggest</code> requests is domain-specific. You submit suggest requests to a domain's search endpoint. To get the search endpoint for your domain, use the Amazon CloudSearch configuration service <code>DescribeDomains</code> action. A domain's endpoints are also displayed on the domain dashboard in the Amazon CloudSearch console. </p>
    async fn suggest(
        &self,
        input: SuggestRequest,
    ) -> Result<SuggestResponse, RusotoError<SuggestError>> {
        let request_uri = "/2013-01-01/suggest";

        let mut request = SignedRequest::new("GET", "cloudsearch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("cloudsearchdomain".to_string());

        let mut params = Params::new();
        params.put("q", &input.query);
        if let Some(ref x) = input.size {
            params.put("size", x);
        }
        params.put("suggester", &input.suggester);
        params.put("format", "sdk");
        params.put("pretty", "true");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<SuggestResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SuggestError::from_response(response))
        }
    }

    /// <p>Posts a batch of documents to a search domain for indexing. A document batch is a collection of add and delete operations that represent the documents you want to add, update, or delete from your domain. Batches can be described in either JSON or XML. Each item that you want Amazon CloudSearch to return as a search result (such as a product) is represented as a document. Every document has a unique ID and one or more fields that contain the data that you want to search and return in results. Individual documents cannot contain more than 1 MB of data. The entire batch cannot exceed 5 MB. To get the best possible upload performance, group add and delete operations in batches that are close the 5 MB limit. Submitting a large volume of single-document batches can overload a domain's document service. </p> <p>The endpoint for submitting <code>UploadDocuments</code> requests is domain-specific. To get the document endpoint for your domain, use the Amazon CloudSearch configuration service <code>DescribeDomains</code> action. A domain's endpoints are also displayed on the domain dashboard in the Amazon CloudSearch console. </p> <p>For more information about formatting your data for Amazon CloudSearch, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/preparing-data.html">Preparing Your Data</a> in the <i>Amazon CloudSearch Developer Guide</i>. For more information about uploading data for indexing, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/uploading-data.html">Uploading Data</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
    async fn upload_documents(
        &self,
        input: UploadDocumentsRequest,
    ) -> Result<UploadDocumentsResponse, RusotoError<UploadDocumentsError>> {
        let request_uri = "/2013-01-01/documents/batch";

        let mut request = SignedRequest::new("POST", "cloudsearch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("cloudsearchdomain".to_string());
        let encoded = Some(input.documents.to_owned());
        request.set_payload(encoded);
        request.add_header("Content-Type", &input.content_type);
        let mut params = Params::new();
        params.put("format", "sdk");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UploadDocumentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UploadDocumentsError::from_response(response))
        }
    }
}
