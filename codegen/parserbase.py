import re
from html2text import html2text
from pprint import pprint


class ParserBase(object):
    """
    Parent class with common code for all flavors of Botocore parser
    """
    def __init__(self, service, client_name):
        self.client_name = client_name
        self.service = service
        self.code = ""

        self.init_primitive_types()

    def init_primitive_types(self):
        """
        map botocore primitives to rust primitives
        """
        self.primitive_types = {
            'string': 'String',
            'timestamp': 'f64',
            'integer': 'i32',
            'long': 'i64',
            'float': 'f32',
            'double': 'f64',
            'blob': 'Vec<u8>',
            'boolean': 'bool'
        }

    def generate_documentation(self, shape, indent=""):
        """
        Translate the HTML documentation from the botocore JSON to markdown
        and append it as a Rust comment
        """
        if 'documentation' in shape:
            markdown = html2text(shape['documentation'])
            markdown = re.sub(r"\n+$", r"", markdown)
            markdown = re.sub(r"\n+", r"\n" + indent + "/// ", markdown)
            self.append(indent + "/// " + markdown)

    
    def rust_type(self, name, shape):
        """
        generate a rust declaration for a botocore shape
        """
        shape_type = shape['type']
        
        self.generate_documentation(shape)
        
        if shape_type == "structure":
            self.rust_struct(name, shape)
        else:
            if shape_type in self.primitive_types:
                rust_type = self.primitive_types[shape_type]
            elif shape_type == 'map':
                rust_type = "HashMap<" + shape['key']['shape'] + "," + shape['value']['shape'] + ">"
            elif shape_type == 'list':
                rust_type = "Vec<" + shape['member']['shape'] + ">"
            else:
                raise Exception("unrecognised type %s. supported types are %s" % (
                    shape_type, primitive_types.keys() + ['map', 'list', 'struct']))

            # a String is already a String in rust
            if name != 'String':
                self.append("pub type " + name + " = " + rust_type + ";")


    def rust_struct(self, name, shape):
        """
        generate a rust declaration for a botocore structure shape        
        """
        pass

    def generate_client(self):
        """
        Generate the Rust struct for the client itself
        """
        

        self.append("pub struct " + self.client_name + "<'a> {")
        self.append("\tcreds: Box<AWSCredentialsProvider + 'a>,")
        self.append("\tregion: &'a Region")
        self.append("}\n")

        self.append("impl<'a> " + self.client_name + "<'a> { ")
        self.append("\tpub fn new<P: AWSCredentialsProvider + 'a>(creds: P, region: &'a Region) -> " + self.client_name + "<'a> {")
        self.append("\t\t" + self.client_name + " { creds: Box::new(creds), region: region }")
        self.append("\t}")

        self.generate_operations()

        self.append("}")

        
    def generate_operations(self):
        for (name, operation) in self.service['operations'].iteritems():
            self.request_method(operation)

    def request_method(self, operation):
        """
        Child classes should override and generate methods that construct and send
        HTTP requests for their service type
        """
        pass

    def shape(self, name):
        return self.service['shapes'][name]
        
    def metadata(self, name):
        return self.service['metadata'][name]


    def get_location_name(self, name, child):
        """
        get the name that should be used for a child element when encoding/decoding
        (sometimes explicitly specified by botocore as 'locationName')
        """
        child_shape = self.shape(child['shape'])
        
        # list elements aren't wrapped in a parent tag, so use their member name
        if child_shape['type'] == 'list':
            tag_name = ParserBase.shape_name(child_shape['member'])
        else:
            if 'locationName' in child:
                tag_name = child['locationName']
            else:
                tag_name = name

        return tag_name


    @staticmethod
    def shape_name(shape):
        if 'locationName' in shape:
            return shape['locationName']
        else:
            return shape['shape']


    @staticmethod
    def get_output_type(operation):
        """
        determine the rust output type for a botocore operation
        """
        if 'output' in operation:
            return operation['output']['shape']
        else:
            return "()"

    
    @staticmethod
    def is_required(shape, field_name):
        if not 'required' in shape:
            return True;
        else:
            return 'required' in shape and field_name in shape['required']


    @staticmethod
    def c_to_s(name):
        """
        convert CamelCase to snake_case
        Also unapolagetically overloaded to prevent collisions with Rust keywords like "type".
        TODO: stop that, use function specifically for that.
        """
        s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
        s2 = re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()
        if s2 == 'type':
            # prepend something informative
            s2 = 'foo_' + s2
        return s2



    def parse(self):
        self.add_imports()
        for (name, shape) in self.service['shapes'].iteritems():
            self.rust_type(name, shape)

            # allow child classes to generate additional code for each shape
            self.shape_hook(name, shape)

        self.generate_client()

        return self.code

    def append(self, string):
        self.code += string + "\n"

    def add_imports(self):
        pass
