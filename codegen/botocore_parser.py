#!/usr/bin/python
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
	'timestamp': 'String',
	'integer': 'i32',
	'double': 'f32',
	'blob': 'Vec<u8>',
	'boolean': 'bool'
}

# rust code to pull primitive types from XML
primitive_parsers = {
	'string': 'try!(characters(stack))',
	'timestamp': 'try!(characters(stack))',
	'integer': 'i32::from_str(try!(characters(stack)).as_ref()).unwrap()',
	'double': 'f32::from_str(try!(characters(stack)).as_ref()).unwrap()',
	'blob': 'try!(characters(stack)).into_bytes()',
	'boolean': 'bool::from_str(try!(characters(stack)).as_ref()).unwrap()'
}

# rust code to write primitive types to a string
primitive_writers = {
	'string': 'obj',
	'timestamp': 'obj',
	'integer': '&obj.to_string()',
	'double': '&obj.to_string()',
	'blob': 'str::from_utf8(&obj).unwrap()',
	'boolean': '&obj.to_string()',
}

# convert CamelCase to snake_case
def c_to_s(name):
	s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
	return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()

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
		# a String is already a String in rust
		if name != 'String':
			print "pub type " + name + " = " + rust_type + ";"

# generate a rust declaration for a botocore structure shape
def rust_struct(name, shape):

	print "#[derive(Debug, Default)]";
	if shape['members']:
		print "pub struct " + name + " {"
		for (mname, member) in shape['members'].iteritems():
			if 'documentation' in member:
				documentation(member,"\t")
			rust_type =  member['shape']
			if not is_required(shape, mname):
				rust_type = "Option<" + rust_type + ">"
			print "\tpub " + c_to_s(mname) + ": " + rust_type + ","
		print "}\n"
	else:
		print "pub struct " + name + ";\n"

# generate rust code to encode a botocore shape into a map of query parameters
def param_writer(name, shape):

	print "/// Write " + name + " contents to a SignedRequest"
	print 'struct ' + name + 'Writer;'
	print 'impl ' + name + 'Writer {'
	print '\tfn write_params(params: &mut Params, name: &str, obj: &' + name + ') {'

	shape_type = shape['type']

	if shape_type in primitive_writers:
		print '\t\tparams.put(name, ' + primitive_writers[shape_type] + ');'
	elif shape_type == 'list':
		list_writer(shape)
	elif shape_type == 'map':
		map_writer(shape)
	elif shape_type == 'structure':
		struct_writer(shape)

	print '\t}'
	print '}'

# guts of the param_writer for struct shapes
def struct_writer(shape):
	print '\t\tlet mut prefix = name.to_string();'
	print '\t\tif prefix != "" { prefix.push_str("."); }'

	for (name, member) in shape['members'].iteritems():
		location_name = get_location_name(name, member)
		if not is_required(shape, name):
			print "\t\tif let Some(ref obj) = obj." + c_to_s(name) + " {"
			print '\t\t\t' + member['shape'] + 'Writer::write_params(params, &(prefix.to_string() + "' + location_name + '"), obj);'
			print "\t\t}"
		else:
			print '\t\t' + member['shape'] + 'Writer::write_params(params, &(prefix.to_string() + "' + location_name + '"), &obj.' + c_to_s(name) + ');'



# guts of the param_writer for list shapes
def list_writer(shape):
	print "\t\tlet mut index = 1;"
	print "\t\tfor element in obj.iter() {"
	print "\t\t\tlet key = &format!(\"{}.{}\", name, index);"
	print "\t\t\t" + shape['member']['shape'] + "Writer::write_params(params, key, &element);"
	print "\t\t\tindex += 1;"
	print "\t\t}"

# guts of the param_writer for map shapes
def map_writer(shape):
	print "\t\tlet mut index = 1;"
	print "\t\tfor (key,value) in obj {"
	print "\t\t\tlet prefix = &format!(\"{}.{}\", name, index);"
	print "\t\t\t" + shape['key']['shape'] + "Writer::write_params(params, &format!(\"{}.{}\", prefix, \"" + shape_name(shape['key']) + "\"), &key);"
	print "\t\t\t" + shape['value']['shape'] + "Writer::write_params(params, &format!(\"{}.{}\", prefix, \"" + shape_name(shape['value'])	 + "\"), &value);"
	print "\t\t\tindex += 1;"
	print "\t\t}"

# generate rust code to parse a botocore shape from XML
def type_parser(name, shape):
	shape_type = shape['type']

	print "/// Parse " + name + " from XML"
	print 'struct ' + name + 'Parser;'
	print 'impl ' + name + 'Parser {'
	# I think this is where the Trait refactor comes in:
	print '\tfn parse_xml<\'a>(tag_name: &str, stack: &mut XmlStack) -> Result<' + name + ', XmlParseError> {'

	if shape_type == 'map':
		map_parser(shape)
	elif shape_type == 'list':
		list_parser(shape)
	else:
		print '\t\ttry!(start_element(tag_name, stack));'
		if shape_type in primitive_parsers:
			print "\t\tlet obj = " + primitive_parsers[shape_type] + ";"
		elif shape_type == 'structure':
			struct_parser(name, shape)
		print '\t\ttry!(end_element(tag_name, stack));'

	print '\t\tOk(obj)'
	print '\t}'
	print '}'

def shape_name(shape):
	if 'locationName' in shape:
		return shape['locationName']
	else:
		return shape['shape']

# guts of the XML parser for map shapes
def map_parser(shape):
	print "\t\tlet mut obj = HashMap::new();"
	print "\t\twhile try!(peek_at_name(stack)) == tag_name {"
	print "\t\t\ttry!(start_element(tag_name, stack));"
	print "\t\t\tlet key = try!(" + shape['key']['shape'] + "Parser::parse_xml(\"" + shape_name(shape['key']) + "\", stack));"
	print "\t\t\tlet value = try!(" + shape['value']['shape'] + "Parser::parse_xml(\"" +shape_name(shape['value']) + "\", stack));"
	print "\t\t\tobj.insert(key, value);"
	print "\t\t\ttry!(end_element(tag_name, stack));"
	print "\t\t}"

# guts of the XML parser for list shapes
def list_parser(shape):
	print "\t\tlet mut obj = Vec::new();";
	print "\t\twhile try!(peek_at_name(stack)) == \"" + shape_name(shape['member']) + "\" {"
	print "\t\t\tobj.push(try!(" + shape['member']['shape'] + "Parser::parse_xml(\"" + shape_name(shape['member']) + "\", stack)));"
	print "\t\t}"

def is_required(shape, field_name):
	if not 'required' in shape:
		return True;
	else:
		return 'required' in shape and field_name in shape['required']

# guts of the XML parser for struct shapes
def struct_parser(name, shape):
	children = shape['members']
	print '\t\tlet mut obj = ' + name + '::default();'
	if children:
		print '\t\tloop {'
		print '\t\t\tlet current_name = try!(peek_at_name(stack));'
		for (cname, child) in children.iteritems():
			parse_struct_child(cname, child, is_required(shape, cname))
		print '\t\t\tbreak;\n\t\t}'

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


# XML parser code to pull a single struct element from XML
def parse_struct_child(name, child, required):

	tag_name = get_location_name(name, child)
	parse_stmt = 'try!(' + child['shape'] + 'Parser::parse_xml("' + tag_name + '", stack))'

	if not required:
		parse_stmt = "Some(" + parse_stmt + ")"

	print '\t\t\tif current_name == "' + tag_name + '" {'
	print '\t\t\t\tobj.' + c_to_s(name) + ' = ' + parse_stmt + ';'
	print '\t\t\t\tcontinue;'
	print '\t\t\t}'

# determine the rust output type for a botocore operation
def get_output_type(operation):
	if 'output' in operation:
		return operation['output']['shape']
	else:
		return "()"

# generate rust code to sign and execute an HTTP request for a botocore operation
def request_method(operation):
	http = operation['http']
	if not ('input' in operation):
		return

	input_name = operation['input']['shape']
	input_type = shapes[input_name]
	output_type = get_output_type(operation)

	documentation(operation,"\t")

	print "\tpub fn " + c_to_s(operation['name']) + "(&self, input: &" + input_name + ") -> Result<" + output_type + ", AWSError> {"
	print '\t\tlet mut request = SignedRequest::new("' + http['method'] + '", "' + metadata['endpointPrefix'] + '", &self.region, "' + http['requestUri'] + '");'
	print "\t\tlet mut params = Params::new();"
	print '\t\tparams.put("Action", "' + operation['name'] + '");'

	print '\t\t' + input_name + 'Writer::write_params(&mut params, \"\", &input);'

	print '\t\trequest.set_params(params);'
	print '\t\tlet result = request.sign_and_execute(&self.creds);'
	print '\t\tlet status = result.status.to_u16();'
#	print '\t\tprintln!("{}", output);'
	print '\t\tlet mut reader = EventReader::new(result);'
	print '\t\tlet mut stack = reader.events().peekable();'
	print '\t\tstack.next();'
	print '\t\tstack.next();'
	print '\t\tmatch status {'

	print '\t\t\t200 => { '
	if output_type == '()':
		print '\t\t\t\tOk(())'
	else:
		print '\t\t\t\tOk(try!(' + output_type + 'Parser::parse_xml("' + output_type + '", &mut stack)))'
	print '\t\t\t}'

	print '\t\t\t_ => { Err(AWSError::new("error")) }'

	print '\t\t}'
	print "\t}"

def generate_client():
		client_name = sys.argv[2]

		print "pub struct " + client_name  + "<'a> {"
		print "\tcreds: &'a AWSCredentials,"
		print "\tregion: &'a str"
		print "}\n"

		print "impl<'a> " + client_name + "<'a> { "
		print "\tpub fn new(creds: &'a AWSCredentials, region: &'a str) -> " + client_name + "<'a> {"
		print "\t\t" + client_name + " { creds: creds, region: region }"
		print "\t}"

		for (name, operation) in operations.iteritems():
			request_method(operation)

		print "}"


def main():
	with open(sys.argv[1]) as data_file:
		service = json.load(data_file)

		print "use std::collections::HashMap;"
		print "use std::str;"
		global shapes
		global metadata
		global operations
		shapes = service['shapes']
		metadata = service['metadata']
		operations = service['operations']

		for (name, shape) in shapes.iteritems():
			rust_type(name, shape)
			type_parser(name, shape)
			param_writer(name, shape)

		generate_client()

if __name__ == "__main__": main()
