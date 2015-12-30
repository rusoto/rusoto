from parserbase import ParserBase

class JsonProtocolParser(ParserBase):

    def __init__(self, service, client_name):
        super(JsonProtocolParser, self).__init__(service, client_name)

    def rust_struct(self, name, shape):
        """
        Don't use camel_case conversion for json protocol structs
        since the (de)serialization is automatic
        """
	self.append("#[derive(Debug, Default, Deserialize, Serialize)]")
	if shape['members']:
            self.append("pub struct " + name + " {")
            for (mname, member) in shape['members'].iteritems():
                if 'documentation' in member:
                    self.generate_documentation(member,"\t")

                rust_type =  member['shape']
                if not JsonProtocolParser.is_required(shape, mname):
                    rust_type = "Option<" + rust_type + ">"
                self.append("\tpub " + mname + ": " + rust_type + ",")
            self.append("}\n")
	else:
            self.append("pub struct " + name + ";\n")


    """
    Implementation of ParserBase for AWS services with protocol type 'json'
    """
    def request_method(self, operation):
        http = operation['http']

	output_type = ParserBase.get_output_type(operation)
        self.generate_documentation(operation, "\t")

	if not ('input' in operation):
            input_name = ''
            input_type = ''            
            self.append("\tpub fn " + ParserBase.c_to_s(operation['name']) + "(&mut self"") -> Result<" + output_type + "> {")
	else:
            input_name = operation['input']['shape']
            input_type = self.shape(input_name)
            self.append("\tpub fn " + ParserBase.c_to_s(operation['name']) + "(&mut self, input: &" + input_name + ") -> Result<" + output_type + "> {")

        self.append('\t\tlet encoded = to_string(&input).expect("failed to convert input to JSON");')
        self.append('\t\tlet mut request = SignedRequest::new("' + http['method'] + '", "' + self.metadata('endpointPrefix') + '", &self.region, "' + http['requestUri'] + '");')
        self.append('\t\trequest.set_content_type("application/x-amz-json-1.0".to_string());')
        self.append('\t\trequest.add_header("x-amz-target", "' + self.metadata('targetPrefix') + '.' + operation['name'] + '");')
        self.append('\t\trequest.set_payload(Some(encoded.as_bytes()));')
        self.append('\t\tlet mut result = request.sign_and_execute(try!(self.creds.get_credentials()));')
	self.append('\t\tlet status = result.status.to_u16();')
        self.append('\t\tlet mut body = String::new();')
        self.append('\t\tresult.read_to_string(&mut body).unwrap();')

	self.append('\t\tmatch status {')
	self.append('\t\t\t200 => { ')

	if output_type == '()':
            self.append('\t\t\t\tOk(())')
	else:
            self.append('\t\t\t\tlet decoded: ' + output_type + ' = from_str(&body).expect("failed to convert JSON into Rust type");')
            self.append('\t\t\t\tOk(decoded)')

	self.append('\t\t\t}')
	self.append('\t\t\t_ => {')
        self.append('\t\t\t\tErr(parse_error(&body))')
        self.append('\t\t\t}')
	self.append('\t\t}')
	self.append("\t}")

    def shape_hook(self, name, shape):
        pass

    def add_imports(self):
        self.append("use std::collections::HashMap;")
        self.append("use std::error::Error;")
        self.append("use std::io::Read;")
        self.append("use serde_json::{from_str, to_string};");


    @staticmethod
    def is_required(shape, field_name):
        """
        TODO: this seems to be backwards from how botocore deals with required fields
              for query protocol services.  Investigate.
        """
	if not 'required' in shape:
            return False;
	else:
            return 'required' in shape and field_name in shape['required']
