#!/usr/bin/env python
import json
import re
import sys
from html2text import html2text
from pprint import pprint

shapes = {}
operations = {}
metadata = {}

# map botocore primitives to rust primitives
primitive_types = {
	'string': 'String',
	'timestamp': 'f64',
	'integer': 'i32',
        'long': 'i64',
        'float': 'f32',
	'double': 'f64',
	'blob': 'Vec<u8>',
	'boolean': 'bool'
}

# convert CamelCase to snake_case
# Also unapolagetically overloaded to prevent collisions with Rust keywords like "type".
# TODO: stop that, use function specifically for that.
def c_to_s(name):
	s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
	s2 = re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()
	if s2 == 'type':
		# prepend something informative
		s2 = 'foo_' + s2
	return s2

def documentation(shape, indent=""):
	if 'documentation' in shape:
		markdown = html2text(shape['documentation'])
		markdown = re.sub(r"\n+$", r"", markdown)
		markdown = re.sub(r"\n+", r"\n" + indent + "/// ", markdown)
		print indent + "/// " + markdown

# generate a rust declaration for a botocore shape
def rust_type(name, shape):
	shape_type = shape['type']

	documentation(shape)

	if shape_type == "structure":
		rust_struct(name, shape)
	else:
		if shape_type in primitive_types:
			rust_type = primitive_types[shape_type]
		elif shape_type == 'map':
			rust_type = "HashMap<" + shape['key']['shape'] + "," + shape['value']['shape'] + ">"
		elif shape_type == 'list':
			rust_type = "Vec<" + shape['member']['shape'] + ">"
                else:
			raise Exception("unrecognised type %s. supported types are %s" % (shape_type, primitive_types.keys() + ['map', 'list', 'struct']))
		# a String is already a String in rust
		if name != 'String':
			print "pub type " + name + " = " + rust_type + ";"

# generate a rust declaration for a botocore structure shape
def rust_struct(name, shape):
	print "#[derive(Debug, Default, RustcDecodable, RustcEncodable)]";
	if shape['members']:
		# print "MEMBERS:" + name
		print "pub struct " + name + " {"
		for (mname, member) in shape['members'].iteritems():
			if 'documentation' in member:
				documentation(member,"\t")
			rust_type =  member['shape']

			if not is_required(shape, mname):
				rust_type = "Option<" + rust_type + ">"
			print "\tpub " + mname + ": " + rust_type + ","
		print "}\n"
	else:
		print "pub struct " + name + ";\n"


def shape_name(shape):
	if 'locationName' in shape:
		return shape['locationName']
	else:
		return shape['shape']


def is_required(shape, field_name):
	if not 'required' in shape:
		return False;
	else:
		return 'required' in shape and field_name in shape['required']


# get the name that should be used for a child element when encoding/decoding
def get_location_name(name, child):
	child_shape = shapes[child['shape']]

	# list elements aren't wrapped in a parent tag, so use their member name
	if child_shape['type'] == 'list':
		tag_name = shape_name(child_shape['member'])
	else:
		if 'locationName' in child:
			tag_name = child['locationName']
		else:
			tag_name = name

	return tag_name



# determine the rust output type for a botocore operation
def get_output_type(operation):
	if 'output' in operation:
		return operation['output']['shape']
	else:
		return "()"

# generate rust code to sign and execute an HTTP request for a botocore operation
def request_method(operation):
	http = operation['http']

	output_type = get_output_type(operation)
	documentation(operation,"\t")

	# This feels so hacky to get around scoping of these in the else block:
	input_name = ''
	input_type = ''

	if not ('input' in operation):
		print "\tpub fn " + c_to_s(operation['name']) + "(&mut self"") -> Result<" + output_type + "> {"
	else:
		input_name = operation['input']['shape']
		input_type = shapes[input_name]
		print "\tpub fn " + c_to_s(operation['name']) + "(&mut self, input: &" + input_name + ") -> Result<" + output_type + "> {"

        print '\t\tlet encoded = json::encode(&input).unwrap();'
        print '\t\tlet mut request = SignedRequest::new("' + http['method'] + '", "' + metadata['endpointPrefix'] + '", &self.region, "' + http['requestUri'] + '");'
        print '\t\trequest.set_content_type("application/x-amz-json-1.0".to_string());'
        print '\t\trequest.add_header("x-amz-target", "DynamoDB_20120810.' + operation['name'] + '");'
        print '\t\trequest.set_payload(Some(encoded.as_bytes()));'

        print '\t\tlet mut result = request.sign_and_execute(try!(self.creds.get_credentials()));'
	print '\t\tlet status = result.status.to_u16();'
        print '\t\tlet mut body = String::new();'
        print '\t\tresult.read_to_string(&mut body).unwrap();'
#        print '\t\tprintln!("{}", body);'        
        
	print '\t\tmatch status {'

	print '\t\t\t200 => { '
	if output_type == '()':
		print '\t\t\t\tOk(())'
	else:
		print '\t\t\t\tlet decoded: ' + output_type + ' = json::decode(&body).unwrap();'
                print '\t\t\t\tOk(decoded)'
	print '\t\t\t}'

	print '\t\t\t_ => {'
        print '\t\t\t\tErr(parse_error(&body))'
        print '\t\t\t}'

	print '\t\t}'
	print "\t}"

def generate_client():
	client_name = 'DynamoDBClient'

	print "pub struct " + client_name  + "<'a> {"
	print "\tcreds: Box<AWSCredentialsProvider + 'a>,"
	print "\tregion: &'a Region"
	print "}\n"

	print "impl<'a> " + client_name + "<'a> { "
	print "\tpub fn new<P: AWSCredentialsProvider + 'a>(creds: P, region: &'a Region) -> " + client_name + "<'a> {"
	print "\t\t" + client_name + " { creds: Box::new(creds), region: region }"
	print "\t}"

	for (name, operation) in operations.iteritems():
		request_method(operation)

	print "}"


def main():
	with open('dynamodb.json') as data_file:
		service = json.load(data_file)

		print "use std::collections::HashMap;"
		print "use std::error::Error;"
                print "use rustc_serialize::json;"
                print "use std::io::Read;"
#                print "use regions::*;";
                
		global shapes
		global metadata
		global operations
		shapes = service['shapes']
		metadata = service['metadata']
		operations = service['operations']

		for (name, shape) in shapes.iteritems():
			# don't pass in reserved Rust keywords.
			if name == 'Message' or name == 'Error':
				# print "REASSIGNING"
				name = 'DynamoDB' + name
			rust_type(name, shape)

		generate_client()

if __name__ == "__main__": main()
