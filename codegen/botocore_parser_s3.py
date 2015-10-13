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
    'timestamp': 'String',
    'integer': 'i32',
    'long': 'i64',
    'float': 'f32',
    'double': 'f64',
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
# Also unapolagetically overloaded to prevent collisions with Rust keywords like "type".
# TODO: stop that, use function specifically for that.
def c_to_s(name):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    s2 = re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()
    if s2 == 'type':
        # prepend something informative
        s2 = 'foo_' + s2
    return s2

def safe_enum_name(name):
    safe = name.replace('-', '_')
    safe = safe.replace(':', '_')
    safe = safe.replace('*', '_star')
    return safe

def documentation(shape, indent=""):
    if 'documentation' in shape:
        markdown = html2text(shape['documentation']).encode('utf-8')
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
        if shape_type in primitive_types and 'enum' in shape:
            print '//NEEDS ENUM for ' + name
            print '#[derive(Debug,PartialEq)]'
            print 'pub enum ' + name + ' {'
            for e in shape['enum']:
                print '\t' + safe_enum_name(e) + ','
            print '}'

            # and impl:
            print 'impl fmt::Display for ' + name +' {'
            print '\tfn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {'
            print '\t\tmatch *self {'

            for e in shape['enum']:
                print '\t\t\t' + name + '::' + safe_enum_name(e) + ' => write!(f, "' + e + '"),'

            print '\t\t}'
            print '\t}'
            print '}'

            # Default:
            print 'impl Default for ' + name + '{'
            print '\tfn default() -> ' + name + '{'
            print '\t\t' + name + '::' + safe_enum_name(shape['enum'][0]) # first entry, may be a stupid choice
            print '\t}'
            print '}'

            # and from string:
            print 'impl From<String> for ' + name + '{'
            print '\tfn from(string: String) -> ' + name + '{'
            print '\t\tmatch string.as_ref() {'

            for e in shape['enum']:
                print '\t\t\t"' + e + '" => ' + name + '::' + safe_enum_name(e) + ','

            print '\t\t\t_ => ' + name + '::default(),'
            print '\t\t}'
            print '\t}'
            print '}'

        else:
            # needs to handle when the rust_type is a reserved keyword
            if shape_type in primitive_types:
                rust_type = primitive_types[shape_type]

            elif shape_type == 'map':
                rust_type = "HashMap<" + shape['key']['shape'] + "," + shape['value']['shape'] + ">"
            elif shape_type == 'list':
                if is_reserved_rust_keyword(shape['member']['shape']):
                    rust_type = 'Vec<' + reserved_rust_keyword_to_client_name(shape['member']['shape']) + '>'
                else:
                    rust_type = "Vec<" + shape['member']['shape'] + ">"
            else:
                raise Exception("unrecognised type %s. supported types are %s" % (
                    shape_type, primitive_types.keys() + ['map', 'list', 'struct']))
            # a String is already a String in rust
            if name != 'String':
                print "pub type " + name + " = " + rust_type + ";"

def is_reserved_rust_keyword(name):
    if name == 'Message' or name == 'Error' or name == 'Errors':
        return True
    return False

def reserved_rust_keyword_to_client_name(name):
    return sys.argv[2] + name

# generate a rust declaration for a botocore structure shape
def rust_struct(name, shape):
    # vec[u8] is special, we may have to implement our own Default:

    if shape['members']:
        need_to_impl_default = False
        # if blob type is here, we'll need an explicit lifetime for struct declaration
        if 'Body' in shape['members']:
            need_to_impl_default = True
            print "#[derive(Debug)]"
            print "pub struct " + name + " {"
        else:
            print "#[derive(Debug, Default)]"
            print "pub struct " + name + " {"

        for (member_name, member) in shape['members'].iteritems():
            if 'documentation' in member:
                documentation(member, "\t")
            rust_type = member['shape']
            lifetime_required = False

            # if 'Body' == rust_type:
            #     lifetime_required = True
            #     print '// should use lifetime for this:'

            if is_reserved_rust_keyword(rust_type):
                rust_type = reserved_rust_keyword_to_client_name(rust_type)

            # if lifetime_required:
            #     print '// setting rust_type to be a reference with lifetime'
            #     rust_type = '&\'a ' + rust_type

            if not is_required(shape, member_name):
                rust_type = "Option<" + rust_type + ">"

            print "\tpub " + c_to_s(member_name) + ": " + rust_type + ","
        print "}\n"

        if need_to_impl_default:
            print '// will impl default here:'
            print 'impl Default for ' + name + ' {'
            print '\tfn default() -> ' + name + ' {'
            print '\t\t' + name + '{'
            # loop over children, calling defaults on everything save the body:
            for (member_name, member) in shape['members'].iteritems():
                rust_type = member['shape']
                if 'Body' == rust_type:
                    if not is_required(shape, member_name):
                        print '\t\t\t' + c_to_s(member_name) + ': None,'
                    else:
                        print '\t\t\t' + c_to_s(member_name) + ': Vec::new(),'
                else:
                    if not is_required(shape, member_name):
                        print '\t\t\t' + c_to_s(member_name) + ': None,'
                    else:
                        print '\t\t\t' + c_to_s(member_name) + ': ' + rust_type + '::default(),'

            print '\t\t}'
            print '\t}'
            print '}'
            # impl <'a>Default for GetObjectTorrentOutput<'a> {
            # 	fn default() -> GetObjectTorrentOutput<'a> {
            # 		GetObjectTorrentOutput {body: &Vec::new(), request_charged: RequestCharged::default()}
            # 	}
            # }

    else:
        print "#[derive(Debug, Default)]"
        print "pub struct " + name + ";\n"


def param_writer(name, shape):
    print "/// Write " + name + " contents to a SignedRequest"
    print 'struct ' + name + 'Writer;'
    print 'impl ' + name + 'Writer {'

    print '\tfn write_params<\'a>(request: &mut SignedRequest<\'a>, obj: &\'a ' + name + ', location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {'
    print '\t\tlet mut body : Option<Vec<u8>> = None;'

    shape_type = shape['type']

    if shape_type in primitive_writers:
        # match if there are multiple URI locations, like bucketname and key, we need to avoid stomping on each other
        # get hostname, if it has something append don't stomp
        print '// shape_type is ' + shape_type

        if shape_type == 'blob':
            print '\t\trequest.set_payload(Some(obj));'
        else:
            # print '\t\tlet mut uri_to_use == "{}.s3.amazonaws.com";'
            print '\t\tmatch location{'
            print '\t\t\tNone => (), // noop'
            print '\t\t\tSome(loc) => {'
            print '\t\t\t\tmatch loc {'
            print '\t\t\t\t\t&ArgumentLocation::Header => (),'
            #print '\t\t\t\t\t&ArgumentLocation::Body => (),'

            print '\t\t\t\t\t&ArgumentLocation::Body => {'
            print '//HEY PRINT ' + name + ' TO BODY (goes in return type)'
            print '\t\t\t\t\t\tbody = Some(obj.to_string().into_bytes());'
            print '\t\t\t\t\t},'

            print '\t\t\t\t\t&ArgumentLocation::Headers => (),'
            print '\t\t\t\t\t&ArgumentLocation::Querystring => (),'
            print '\t\t\t\t\t&ArgumentLocation::Uri => {' # request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj))),'

            print '\t\t\t\t\t\t// Bucket always goes in this format: bucketname.s3.amazonaws.com'
            print '\t\t\t\t\t\t// TODO: can we optimize this out with codegen?'
            print '\t\t\t\t\t\tif location_name == "Bucket" {'
            print '\t\t\t\t\t\t\trequest.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));'
            print '\t\t\t\t\t\t} else {'

            print '\t\t\t\t\t\t\t// Assume anything else in the URI goes in hostname/obj'
            print '\t\t\t\t\t\t\tlet existing_path = request.get_path();'
            print '\t\t\t\t\t\t\trequest.set_path(&format!("{}/{}", existing_path, obj));'
            print '\t\t\t\t\t\t}'

            print '\t\t\t\t\t},'
            print '\t\t\t\t}'
            print '\t\t\t},'
            print '\t\t}'

    elif shape_type == 'list':
        list_writer(shape)
    elif shape_type == 'map':
        map_writer(shape)
    elif shape_type == 'structure':
        struct_writer(shape)

    print '\t\tbody'
    print '\t}'
    print '}'

def location_name_for_writing(member):
    if 'location' in member and member['location'] == 'header':
        return 'ArgumentLocation::Header'
    elif 'location' in member and member['location'] == 'Body':
        return 'ArgumentLocation::Body'
    elif 'location' in member and member['location'] == 'headers':
        return 'ArgumentLocation::Headers' #used for map writing of metadata
    elif 'location' in member and member['location'] == 'querystring':
        return 'ArgumentLocation::Querystring'
    elif 'location' in member and member['location'] == 'uri':
        return 'ArgumentLocation::Uri'
    return 'ArgumentLocation::Body'

# guts of the param_writer for struct shapes
def struct_writer(shape):
    print '\t\t// STRUCT_WRITER'
    for (name, member) in shape['members'].iteritems():
        location_name = get_location_name(name, member)
        shape_name = member['shape']
        if is_reserved_rust_keyword(shape_name):
            shape_name = reserved_rust_keyword_to_client_name(member['shape'])

        if location_name_for_writing(member) == 'ArgumentLocation::Body':
            print '// GOES IN BODY, set here from return types?'

        if not is_required(shape, name):
            print '\t\t// optional writing for ' + c_to_s(name)
            print "\t\tif let Some(ref obj) = obj." + c_to_s(name) + " {"

            if location_name_for_writing(member) == 'ArgumentLocation::Body':
                print '\t\t\tbody = ' + shape_name + 'Writer::write_params(request, &obj, Some(&' + location_name_for_writing(member) + '), &"' + location_name + '".to_string());'
            else:
                print '\t\t\t' + shape_name + 'Writer::write_params(request, &obj, Some(&' + location_name_for_writing(member) + '), &"' + location_name + '".to_string());'

            print "\t\t}"
        else:
            print '//required field: '
            # print '//shove this into body and do a find/replace in primitive writer?'
            # print '//<' + shape_name + ' xmlns="http://s3.amazonaws.com/doc/2006-03-01/">'
            # print '//' + location_name
            # print '//</' + shape_name + '>'
            if location_name_for_writing(member) == 'ArgumentLocation::Body':
                print '\t\tbody = ' + shape_name + 'Writer::write_params(request, &obj.' + c_to_s(name) + ', Some(&' + location_name_for_writing(member) + '), &"' + location_name + '".to_string());'
            else:
                print '\t\t' + shape_name + 'Writer::write_params(request, &obj.' + c_to_s(name) + ', Some(&' + location_name_for_writing(member) + '), &"' + location_name + '".to_string());'


# guts of the param_writer for list shapes
def list_writer(shape):
    print '// TODO: list_writer for ' + shape['member']['shape']
    print ';'
    # print "\t\tlet mut index = 1;"
    # print "\t\tfor element in obj.iter() {"
    # print "\t\t\tlet key = &format!(\"{}.{}\", name, index);"
    # print "\t\t\t" + shape['member']['shape'] + "Writer::write_params(&mut request, &element);"
    # print "\t\t\tindex += 1;"
    # print "\t\t}"


# guts of the param_writer for map shapes
def map_writer(shape):
    print '// TODO: map_writer for ' + shape['key']['shape'] + ' : ' + shape['value']['shape']
    print ';'
    # print "\t\tlet mut index = 1;"
    # print "\t\tfor (key,value) in obj {"
    # print "\t\t\tlet prefix = &format!(\"{}.{}\", name, index);"
    # print "\t\t\t" + shape['key']['shape'] + "Writer::write_params(&mut request, prefix, \"" + shape_name(
    #     shape['key']) + "\"), &key);"
    # print "\t\t\t" + shape['value'][
    #     'shape'] + "Writer::write_params(&mut request, prefix, \"" + shape_name(
    #     shape['value']) + "\"), &value);"
    # print "\t\t\tindex += 1;"
    # print "\t\t}"


# generate rust code to parse a botocore shape from XML
def type_parser(name, shape):
    shape_type = shape['type']

    print "\n/// Parse " + name + " from response"
    print 'struct ' + name + 'Parser;'
    print 'impl ' + name + 'Parser {'
    print '\tfn parse_response<\'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<' +\
          name + ', XmlParseError> {'
    print '\t\tprintln!("in ' + name + 'Parser");'

    if shape_type == 'map':
        map_parser(shape)
    elif shape_type == 'list':
        list_parser(name, shape)
    else:
        print '\t\t// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:'
        # match argumentlocation
        # if body: print start tag parse

        print '\t\tmatch location {'
        print '\t\t\tNone => (),'
        print '\t\t\tSome(loc) => {'

        print '\t\t\t\tif loc == &ArgumentLocation::Body {'
        print '\t\t\t\t\tmatch tag_name {'
        print '\t\t\t\t\t\tNone => (),'
        print '\t\t\t\t\t\tSome(tag) => { try!(start_element(tag, stack)) ; },'
        print '\t\t\t\t\t}'
        print '\t\t\t\t}'
        print '\t\t\t},'
        print '\t\t}'

        # print '\t\ttry!(start_element(tag_name, stack));'
        if shape_type in primitive_parsers:
            print '\t\t // primitive_parser'
            if shape_type == 'blob':
                print "\t\tlet mut obj : " + primitive_types[shape_type] + " = Vec::with_capacity(1);"
            else:
                # TODO: map to type, if it's an enum: it has to string and from string
                if 'enum' in shape:
                    print "\t\tlet mut obj : " + name + " = " + name + "::default();"
                else:
                    print "\t\tlet mut obj : " + primitive_types[shape_type] + " = " + primitive_types[shape_type] + "::default();"

            print '\t\tmatch location{'
            print '\t\t\tNone => (), // noop'
            print '\t\t\tSome(loc) => {'
            print '\t\t\t\tmatch loc {'
            print '\t\t\t\t\t&ArgumentLocation::Headers => (), // not yet implemented'
            print '\t\t\t\t\t&ArgumentLocation::Body => {'

            if shape_type in primitive_parsers:
                print '\t\t\t\t\t\t// primitive_parser'
                if shape_type == 'blob':
                    print "\t\t\t\t\t\tobj = Vec::with_capacity(1);"
                else:
                    # TODO: if it's an enum use type::from(primitive_parsers)
                    if 'enum' in shape:
                        print "\t\t\t\t\t\tobj = " + name + '::from(' + primitive_parsers[shape_type] + ');'
                    else:
                        print "\t\t\t\t\t\tobj = " + primitive_parsers[shape_type] + ';'

            print '\t\t\t\t\t},'



            if shape_type == 'blob':
                print '\t\t\t\t\t&ArgumentLocation::Header => (),'
            else:
                print '\t\t\t\t\t&ArgumentLocation::Header => {'
                print '\t\t\t\t\t\tlet header_str = try!(get_value_for_header(tag_name.unwrap(), headers));'
                if 'enum' in shape:
                    print '\t\t\t\t\t\tobj = ' + name + '::from(header_str);'
                elif shape_type == 'string' or primitive_types[shape_type] == 'String':
                    print '\t\t\t\t\t\tobj = header_str;'
                else:
                    if shape_type == 'boolean':
                        print '\t\t\t\t\t\tobj = match bool::from_str(&header_str) {'
                        print '\t\t\t\t\t\t\tErr(_) => false,'
                        print '\t\t\t\t\t\t\tOk(newbool) => newbool,'
                        print '\t\t\t\t\t\t}'
                    else:
                        # TODO: if it's an enum use type::from(primitive_parsers)
                        if 'enum' in shape:
                            print '\t\t\t\t\t\tobj = ' + name + '::from(try!(' + primitive_types[shape_type] + '::from_str(&header_str)));'
                        else:
                            print '\t\t\t\t\t\tobj = try!(' + primitive_types[shape_type] + '::from_str(&header_str));'

                print '\t\t\t\t\t},'

            print '\t\t\t\t\t&ArgumentLocation::Querystring => (),'
            print '\t\t\t\t\t&ArgumentLocation::Uri => ()'
            print '\t\t\t\t\t}'
            print '\t\t\t}'
            print '\t\t}'

            print '\t\tmatch location {'
            print '\t\t\tNone => (),'
            print '\t\t\tSome(loc) => {'

            print '\t\t\t\tif loc == &ArgumentLocation::Body {'
            print '\t\t\t\t\tmatch tag_name {'
            print '\t\t\t\t\t\tNone => (),'
            print '\t\t\t\t\t\tSome(tag) => { try!(end_element(tag, stack)) ; },'
            print '\t\t\t\t\t}'
            print '\t\t\t\t}'
            print '\t\t\t},'
            print '\t\t}'

        elif shape_type == 'structure':
            struct_parser(name, shape)


        # print '\t\ttry!(end_element(tag_name, stack));'

    print '\t\tOk(obj)'
    print '\t}'
    print '}'


def shape_name(shape):
    if 'locationName' in shape:
        return shape['locationName']
    else:
        return shape['shape']


# guts of the XML parser for map shapes
# TODO: pass in name like list_parser below
def map_parser(shape):
    print '\t\t // map_parser'
    print '//lol punt'
    print "\t\tlet mut obj = HashMap::new();"
    # print "\t\twhile try!(peek_at_name(stack)) == tag_name {"
    # print "\t\t\ttry!(start_element(tag_name, stack));"
    # print "\t\t\tlet key = try!(" + shape['key']['shape'] + "Parser::parse_response(\"" + shape_name(
    #     shape['key']) + "\", &headers, stack));"
    # print "\t\t\tlet value = try!(" + shape['value']['shape'] + "Parser::parse_response(\"" + shape_name(
    #     shape['value']) + "\", &headers, stack));"
    # print "\t\t\tobj.insert(key, value);"
    # print "\t\t\ttry!(end_element(tag_name, stack));"
    # print "\t\t}"


# guts of the XML parser for list shapes
def list_parser(name, shape):
    print '\t\t // list_parser'
    print "\t\tlet mut obj = Vec::new();"
    print '\t\tstack.next(); // need to consume the first part of the XML to find what we\'re looking for.'
    print "\t\twhile try!(peek_at_name(stack)) == \"" + shape_name(shape['member']) + "\" {"

    # this needs to iterate over children of shape
    if 'member' in shape:
        print '//we need to iterate over members of ' + shape_name(shape['member'])
        for (cname, child) in shape['member'].iteritems():
            if cname == 'locationName':
                print '// skip ' + child + '.  It\'s a location name.'
            else:
                print '// obj.push for ' + child
                if is_reserved_rust_keyword(child):
                    print '\t\t\tobj.push(try!(' + reserved_rust_keyword_to_client_name(child) + 'Parser::parse_response(Some(&"' \
                          + child + '"), Some(&' + location_name_for_writing(child) + '), &headers, stack)));'
                else:
                    print '\t\t\tobj.push(try!(' + child + 'Parser::parse_response(Some(&"' + child + '"), Some(&'  \
                          + location_name_for_writing(child) + '), &headers, stack)));'

    else:
        if 'flattened' in shape:
            print '//name included, flattened, setting to results'
            print "\t\t\tobj = try!(" + name + "Parser::parse_response(\"" + shape_name(
                shape['member']) + "\", &headers, stack));"
        else:
            print '// name included, not flattened, pushing'
            print "\t\t\tobj = try!(" + name + "Parser::parse_response(\"" + shape_name(
                shape['member']) + "\", &headers, stack));"

    print "\t\t}"


def is_required(shape, field_name):
    if not 'required' in shape:
        return True
    else:
        return 'required' in shape and field_name in shape['required']


# guts of the XML parser for struct shapes
def struct_parser(name, shape):
    print '\t\t // struct_parser'
    children = shape['members']
    print '\t\tlet mut obj = ' + name + '::default();'
    if children:
        need_body_parser = False

        for (cname, child) in children.iteritems():
            if location_name_for_writing(child) == 'ArgumentLocation::Body':
                need_body_parser = True
            if location_name_for_writing(child) != 'ArgumentLocation::Body':
                name_to_use = child['shape']
                if is_reserved_rust_keyword(name_to_use):
                    name_to_use = reserved_rust_keyword_to_client_name(cname)

                print '\t\t//parser for cname of ' + cname + ' and child shape is ' + child['shape']
                # print '\t\t// location name is ' + location_name_for_writing(child)

                # if the shape doesn't specify requirements, assume we need them all:
                if not 'required' in shape:
                    print '\t\tobj.' + c_to_s(cname) + ' = try!(' + name_to_use + 'Parser::parse_response(Some(&"' \
                          + cname + '".to_string()), Some(&' + location_name_for_writing(child) + '), headers, stack));'
                else:
                    if 'required' in shape and cname in shape['required']:
                        print '\t\tobj.' + c_to_s(cname) + ' = try!(' + name_to_use + 'Parser::parse_response(Some(&"' \
                              + cname + '".to_string()), Some(&' + location_name_for_writing(child) + '), headers, stack));'
                    else:
                        print '\t\tobj.' + c_to_s(cname) + ' = Some(try!(' + name_to_use + 'Parser::parse_response(Some(&"' \
                              + cname + '".to_string()), Some(&' + location_name_for_writing(child) + '), headers, stack)));'

        if need_body_parser:
            print '\t\tloop {'
            print '\t\t\tlet current_name = try!(peek_at_name(stack));'
            print 'println!("current_name is {}", current_name);'
            for (cname, child) in children.iteritems():
                name_to_use = child['shape'] # this should be cname in here I think
                #name_to_use = cname
                if is_reserved_rust_keyword(name_to_use):
                    name_to_use = reserved_rust_keyword_to_client_name(cname)

                if location_name_for_writing(child) == 'ArgumentLocation::Body':
                    print '\t\t\t//body parser for cname of ' + cname + ' and child shape is ' + child['shape']
                    #print '\t\t\tif current_name == "' + name_to_use + '"{'
                    print '\t\t\tif current_name == "' + cname + '"{'

                    # boo copy/paste:
                    # if the shape doesn't specify requirements, assume we need them all:
                    if not 'required' in shape:
                        print '\t\t\t\tobj.' + c_to_s(cname) + ' = try!(' + name_to_use + 'Parser::parse_response(Some(&"' \
                              + cname + '".to_string()), Some(&' + location_name_for_writing(child) + '), headers, stack));'
                    else:
                        if 'required' in shape and cname in shape['required']:
                            print '\t\t\t\tobj.' + c_to_s(cname) + ' = try!(' + name_to_use + 'Parser::parse_response(Some(&"' \
                                  + cname + '".to_string()), Some(&' + location_name_for_writing(child) + '), headers, stack));'
                        else:
                            print '\t\t\t\tobj.' + c_to_s(cname) + ' = Some(try!(' + name_to_use + 'Parser::parse_response(Some(&"' \
                                  + cname + '".to_string()), Some(&' + location_name_for_writing(child) + '), headers, stack)));'


                    print '\t\t\t\tcontinue;'
                    print '\t\t\t}'


            print '\t\t\tbreak;'
            print '\t\t}'



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


# Need to figure out how to explain this well
def get_location(name, child):
    if 'location' in child:
        return child['location']  # should always be header or uri
    else:
        return 'payload'


# XML parser code to pull a single struct element from XML
# TODO: pass in location name as enum to parser
def parse_struct_child(name, child, required):
    tag_name = get_location_name(name, child)

    print '\t\t\t// heylisten etc: location for ' + name + ' is ' + get_location(name, child) + '.'

    if is_reserved_rust_keyword(child['shape']):
        parse_stmt = 'try!(' + reserved_rust_keyword_to_client_name(child['shape']) + 'Parser::parse_response("' + tag_name + '", &headers, stack))'
    else:
        parse_stmt = 'try!(' + child['shape'] + 'Parser::parse_response("' + tag_name + '", &headers, stack))'

    if not required:
        parse_stmt = "Some(" + parse_stmt + ")"

    print '\t\t\tif current_name == "' + name + '" {'
    # sometimes this is child['shape'] sometimes it's name:
    if c_to_s(child['shape']) != c_to_s(name):
        print '// child[shape] and name don\'t match:'
        print '// child[shape]: ' + c_to_s(child['shape'])
        print '// name: ' + c_to_s(name)

    print '\t\t\t\t//obj.' + c_to_s(child['shape']) + ' = ' + parse_stmt + ';'
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

    output_type = get_output_type(operation)
    documentation(operation, "\t")

    input_name = operation['name']

    if not ('input' in operation):
        print "\tpub fn " + c_to_s(operation['name']) + "(&mut self"") -> Result<" + output_type + ", AWSError> {"
        # TODO: This still needs to set input_name for the param writer, even if it's a GET

    else:
        input_name = operation['input']['shape']
        print "\tpub fn " + c_to_s(
            operation['name']) + "(&mut self, input: &" + input_name + ") -> Result<" + output_type + ", AWSError> {"

    print '\t\tlet mut uri = String::from("");'
    print '\t\tlet mut request_body : Vec<u8>;'

    # Most of these go to just the bucket

    print '\t\tlet mut request = SignedRequest::new("' + http['method'] + '", "' + metadata[
        'endpointPrefix'] + '", &self.region, &uri);\n'

    if not ('input' in operation):
        # print '\t\t//The writer needs to set hostname, but can it do that with only the request?'
        # print '\t\t' + input_name + 'Writer::write_params(&mut request);'
        print '\t\t//nothing to set'
    else:
        print '\t\tmatch ' + input_name + 'Writer::write_params(&mut request, &input, None, &"".to_string()) {'
        print '\t\t\tNone => (),'
        print '\t\t\tSome(body_val) => {'
        print '\t\t\t\trequest_body = body_val;'
        print '\t\t\t\trequest.set_payload(Some(&request_body));'
        print '\t\t\t},'
        print '\t\t};\n'

    print '\t\tlet mut result = request.sign_and_execute(try!(self.creds.get_credentials()));'
    print '\t\tlet status = result.status.to_u16();\n'

    print '\t\tmatch status {'
    print '\t\t\t200 => { '
    if output_type == '()':
        print '\t\t\t\tOk(())'
    else:
        # print '\t\t\t\tlet mut aws_result = ' + output_type + '::default();'
        print '\t\t\t\tlet headers = result.headers.clone();'
        print '\t\t\t\tlet mut reader = EventReader::new(result);'
        print '\t\t\t\tlet mut stack = XmlResponseFromAws::new(reader.events().peekable());'
        print '\t\t\t\tstack.next(); // xml start tag'
        print '\t\t\t\tstack.next(); // top level result'
        print '\t\t\t\t// tag name for top level doesn\'t matter:'
        print '\t\t\t\tlet aws_result = try!(' + output_type + 'Parser::parse_response(None, None, &headers, &mut stack));'
        print '\t\t\t\tOk(aws_result)'
    print '\t\t\t}'

    print '\t\t\t_ => { '
    print '\t\t\t\tprintln!("Error: Status code was {}", status);'
    print '\t\t\t\tlet mut body = String::new();'
    print '\t\t\t\tresult.read_to_string(&mut body).unwrap();\n'
    print '\t\t\t\tprintln!("Error response body: {}", body);'

    print '\t\t\t\tErr(AWSError::new("S3 error in ' + c_to_s(operation['name']) + '"))'
    print '\t\t\t }'

    print '\t\t}'
    print "\t}"

def generate_client():
    client_name = sys.argv[2]

    print "pub struct " + client_name + "<'a> {"
    print "\tcreds: Box<AWSCredentialsProvider + 'a>,"
    print "\tregion: &'a Region"
    print "}\n"

    print "impl<'a> " + client_name + "<'a> { "
    print "\tpub fn new<P: AWSCredentialsProvider + 'a>(creds: P, region: &'a Region) -> S3Client<'a> {"
    print "\t\t" + client_name + " { creds: Box::new(creds), region: region }"
    print "\t}"

    for (name, operation) in operations.iteritems():
        request_method(operation)

    print "}\n"

def header_helper():
    print 'pub fn get_value_for_header(header_name: &str, headers: &Headers) -> Result<String, XmlParseError> {'
    print '\tfor header in headers.iter() {'
    print '\t\tif header.name() == header_name {'
    print '\t\t\treturn Ok(header.value_string());'
    print '\t\t}'
    print '\t}'
    print '\tOk(String::new())'
    print '\t// should be:'
    print '\t// Err(AWSError::new(format!("Couldn\'t find field {} in headers", header_name)))'
    print '}'

def location_enum():
    print '#[derive(Debug,PartialEq)]'
    print 'pub enum ArgumentLocation {'
    print '\tHeader,'
    print '\tBody,'
    print '\tHeaders,'
    print '\tQuerystring,'
    print '\tUri,'
    print '}'

def pretty(d, indent=0):
    for key, value in d.iteritems():
        print '\t' * indent + str(key)
        if isinstance(value, dict):
            pretty(value, indent + 1)
        else:
            print '\t' * (indent + 1) + str(value)

def is_output_type(name):
    if name.endswith('Output') or name.endswith('Result') or name.endswith('Errors') or name.endswith('Errors'):
        return True
    return False

def is_request_type(name):
    if name.endswith('Request'):
        return True
    return False

def generate_shapes():
    for (name, shape) in shapes.iteritems():
        # don't pass in reserved Rust keywords.
        if is_reserved_rust_keyword(name):
            name = reserved_rust_keyword_to_client_name(name)

        rust_type(name, shape)

        # no need to make a parser for an outgoing request
        if not is_request_type(name):
            type_parser(name, shape)

        # no need to make a parameter writer for a type coming back from AWS
        if not is_output_type(name):
            param_writer(name, shape)

def main():
    with open(sys.argv[1]) as data_file:
        service = json.load(data_file)

        print "use std::collections::HashMap;"
        # print "use std::str;"
        # print "use std::error::Error;\n"
        global shapes
        global metadata
        global operations
        shapes = service['shapes']
        metadata = service['metadata']
        operations = service['operations']

        location_enum()
        header_helper()

        generate_shapes()
        generate_client()


if __name__ == "__main__": main()
