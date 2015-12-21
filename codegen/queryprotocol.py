from parserbase import ParserBase

class QueryProtocolParser(ParserBase):
    """
    Implementation of ParserBase for AWS services with protocol type 'query'
    """

    def __init__(self, service, client_name):
        super(QueryProtocolParser, self).__init__(service, client_name)
        self.init_primitive_parsers()
        self.init_primitive_writers()

    def init_primitive_parsers(self):
        """
        rust code to pull primitive types from XML
        """
        self.primitive_parsers = {
            'string': 'try!(characters(stack))',
            'timestamp': 'try!(characters(stack))',
            'integer': 'i32::from_str(try!(characters(stack)).as_ref()).unwrap()',
            'double': 'f32::from_str(try!(characters(stack)).as_ref()).unwrap()',
            'blob': 'try!(characters(stack)).into_bytes()',
            'boolean': 'bool::from_str(try!(characters(stack)).as_ref()).unwrap()'
        }


    def init_primitive_writers(self):
        """
        rust code to write primitive types to a string
        """
        self.primitive_writers = {
            'string': 'obj',
            'timestamp': 'obj',
            'integer': '&obj.to_string()',
            'double': '&obj.to_string()',
            'blob': 'str::from_utf8(&obj).unwrap()',
            'boolean': '&obj.to_string()',
        }

    def shape_hook(self, name, shape):
        """
        Generate an XML parser and writer for each shape
        """
        self.type_parser(name, shape)
        self.param_writer(name, shape)


    def type_parser(self, name, shape):
        """
        generate rust code to parse a botocore shape from XML
        """

        shape_type = shape['type']
        
        self.append("/// Parse " + name + " from XML")
        self.append('struct ' + name + 'Parser;')
        self.append('impl ' + name + 'Parser {')
        self.append('\tfn parse_xml<\'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<' + name + ', XmlParseError> {')

        if shape_type == 'map':
            self.map_parser(shape)
        elif shape_type == 'list':
            self.list_parser(shape)
        else:
            self.append('\t\ttry!(start_element(tag_name, stack));')
            if shape_type in self.primitive_parsers:
                self.append("\t\tlet obj = " + self.primitive_parsers[shape_type] + ";")
            elif shape_type == 'structure':
                self.struct_parser(name, shape)
            self.append('\t\ttry!(end_element(tag_name, stack));')

        self.append('\t\tOk(obj)')
        self.append('\t}')
        self.append('}')


    def struct_parser(self, name, shape):
        """
        guts of the XML parser for struct shapes
        """
        children = shape['members']
        self.append('\t\tlet mut obj = ' + name + '::default();')
        if children:
            self.append('\t\tloop {')
            self.append('\t\t\tlet current_name = try!(peek_at_name(stack));')
            for (cname, child) in children.iteritems():
                self.parse_struct_child(cname, child, ParserBase.is_required(shape, cname))
            self.append('\t\t\tbreak;\n\t\t}')

    # XML parser code to pull a single struct element from XML
    def parse_struct_child(self, name, child, required):
        tag_name = self.get_location_name(name, child)
        parse_stmt = 'try!(' + child['shape'] + 'Parser::parse_xml("' + tag_name + '", stack))'

        if not required:
            parse_stmt = "Some(" + parse_stmt + ")"

        self.append('\t\t\tif current_name == "' + tag_name + '" {')
        self.append('\t\t\t\tobj.' + ParserBase.c_to_s(name) + ' = ' + parse_stmt + ';')
        self.append('\t\t\t\tcontinue;')
        self.append('\t\t\t}')

    def map_parser(self, shape):
        """
        guts of the XML parser for map shapes
        """

        self.append("\t\tlet mut obj = HashMap::new();")
        self.append("\t\twhile try!(peek_at_name(stack)) == tag_name {")
        self.append("\t\t\ttry!(start_element(tag_name, stack));")
        self.append("\t\t\tlet key = try!(" + shape['key']['shape'] + "Parser::parse_xml(\"" + QueryProtocolParser.shape_name(
            shape['key']) + "\", stack));")
        self.append("\t\t\tlet value = try!(" + shape['value']['shape'] + "Parser::parse_xml(\"" + QueryProtocolParser.shape_name(
            shape['value']) + "\", stack));")
        self.append("\t\t\tobj.insert(key, value);")
        self.append("\t\t\ttry!(end_element(tag_name, stack));")
        self.append("\t\t}")
        

    def list_parser(self, shape):
        """
        guts of the XML parser for list shapes
        """

        self.append("\t\tlet mut obj = Vec::new();")
        self.append("\t\twhile try!(peek_at_name(stack)) == \"" + QueryProtocolParser.shape_name(shape['member']) + "\" {")
        self.append("\t\t\tobj.push(try!(" + shape['member']['shape'] + "Parser::parse_xml(\"" + QueryProtocolParser.shape_name(
            shape['member']) + "\", stack)));")
        self.append("\t\t}")


    def param_writer(self, name, shape):
        """
        generate rust code to encode a botocore shape into a map of query parameters
        """
        self.append("/// Write " + name + " contents to a SignedRequest")
        self.append('struct ' + name + 'Writer;')
        self.append('impl ' + name + 'Writer {')
        self.append('\tfn write_params(params: &mut Params, name: &str, obj: &' + name + ') {')
        
        shape_type = shape['type']
        
        if shape_type in self.primitive_writers:
            self.append('\t\tparams.put(name, ' + self.primitive_writers[shape_type] + ');')
        elif shape_type == 'list':
            self.list_writer(shape)
        elif shape_type == 'map':
            self.map_writer(shape)
        elif shape_type == 'structure':
            self.struct_writer(shape)

        self.append('\t}')
        self.append('}')

    # guts of the param_writer for struct shapes
    def struct_writer(self, shape):
        self.append('\t\tlet mut prefix = name.to_string();')
        self.append('\t\tif prefix != "" { prefix.push_str("."); }')

        for (name, member) in shape['members'].iteritems():
            location_name = self.get_location_name(name, member)
            if not ParserBase.is_required(shape, name):
                self.append("\t\tif let Some(ref obj) = obj." + ParserBase.c_to_s(name) + " {")
                self.append('\t\t\t' + member[
                    'shape'] + 'Writer::write_params(params, &(prefix.to_string() + "' + location_name + '"), obj);')
                self.append("\t\t}")
            else:
                self.append('\t\t' + member[
                    'shape'] + 'Writer::write_params(params, &(prefix.to_string() + "' + location_name + '"), &obj.' + ParserBase.c_to_s(
                        name) + ');')


    # guts of the param_writer for list shapes
    def list_writer(self, shape):
        self.append("\t\tlet mut index = 1;")
        self.append("\t\tfor element in obj.iter() {")
        self.append("\t\t\tlet key = &format!(\"{}.{}\", name, index);")
        self.append("\t\t\t" + shape['member']['shape'] + "Writer::write_params(params, key, &element);")
        self.append("\t\t\tindex += 1;")
        self.append("\t\t}")
        

    # guts of the param_writer for map shapes
    def map_writer(self, shape):
        self.append("\t\tlet mut index = 1;")
        self.append("\t\tfor (key,value) in obj {")
        self.append("\t\t\tlet prefix = &format!(\"{}.{}\", name, index);")
        self.append("\t\t\t" + shape['key']['shape'] + "Writer::write_params(params, &format!(\"{}.{}\", prefix, \"" + QueryProtocolParser.shape_name(
            shape['key']) + "\"), &key);")
        self.append("\t\t\t" + shape['value'][
            'shape'] + "Writer::write_params(params, &format!(\"{}.{}\", prefix, \"" + QueryProtocolParser.shape_name(
                shape['value']) + "\"), &value);")
        self.append("\t\t\tindex += 1;")
        self.append("\t\t}")


    def request_method(self, operation):
        http = operation['http']

        output_type = ParserBase.get_output_type(operation)
        self.generate_documentation(operation, "\t")
        
        # This feels so hacky to get around scoping of these in the else block:
        input_name = ''
        input_type = ''
        
        if not ('input' in operation):
            self.append("\tpub fn " + ParserBase.c_to_s(operation['name']) + "(&mut self"") -> Result<" + output_type + ", AWSError> {")
        else:
            input_name = operation['input']['shape']
            input_type = self.shape(input_name)
            self.append("\tpub fn " + ParserBase.c_to_s(operation[
                'name']) + "(&mut self, input: &" + input_name + ") -> Result<" + output_type + ", AWSError> {")

        self.append('\t\tlet mut request = SignedRequest::new("' + http['method'] + '", "' + self.metadata('endpointPrefix') 
                    + '", &self.region, "' + http['requestUri'] + '");')
        self.append("\t\tlet mut params = Params::new();")
        self.append('\t\tparams.put("Action", "' + operation['name'] + '");')
        
        if ('input' in operation):
            self.append('\t\t' + input_name + 'Writer::write_params(&mut params, \"\", &input);')

        self.append('\t\trequest.set_params(params);')
        self.append('\t\tlet mut result = request.sign_and_execute(try!(self.creds.get_credentials()));')
        self.append('\t\tlet status = result.status.to_u16();')
        #	self.append('\t\tprintln!("{}", output);'
        self.append('\t\tlet mut reader = EventReader::new(result);')
        self.append('\t\tlet mut stack = XmlResponseFromAws::new(reader.events().peekable());')
        self.append('\t\tstack.next(); // xml start tag')
        self.append('\t\tstack.next();')
        self.append('\t\tmatch status {')
        
        self.append('\t\t\t200 => { ')

        if output_type == '()':
            self.append('\t\t\t\tOk(())')
        else:
            self.append('\t\t\t\tOk(try!(' + output_type + 'Parser::parse_xml("' + output_type + '", &mut stack)))')

        self.append('\t\t\t}')
        self.append('\t\t\t_ => { Err(AWSError::new("error")) }')        
        self.append('\t\t}')
        self.append("\t}")

    @staticmethod    
    def shape_name(shape):
        if 'locationName' in shape:
            return shape['locationName']
        else:
            return shape['shape']
