extern crate botocore_parser;
extern crate inflector;
extern crate regex;
extern crate serde_json;
extern crate serde_codegen;
extern crate syntex;

use botocore_parser::{Service, Shape};
use std::fs::File;
use std::io::{Read, Write};
use inflector::Inflector;
use std::path::Path;
use std::env;
use syntex::Registry;
use serde_codegen::register;

const BOTOCORE_DIR: &'static str = "codegen/botocore/botocore/data/";

struct AmazonService {
    name: String,
    type_name: String, 
    protocol_date: String
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir);

    let botocore_dir = env::var_os("BOTOCORE_DIR");
    let botocore_path = match botocore_dir {
        Some(ref dirname) => Path::new(dirname),
        None => Path::new(BOTOCORE_DIR)
    };
    
    let services = vec![
        AmazonService::new("dynamodb", "DynamoDBClient", "2012-08-10"),
        AmazonService::new("kms", "KMSClient", "2014-11-01"),/*,
        AmazonService::new("sqs", "SQSClient", "2012-11-05")*/
    ];

    for service in services {        
        generate(service, botocore_path, out_path);
    }
}

fn botocore_generate(input: &str, type_name: &str, destination: &Path) {


    let mut f = File::open(input).unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);

    let service: Service = serde_json::from_str(&s).expect("Invalid botocore input");

    let mut source = String::new();
    
    // source.push_str("#![allow(non_snake_case)]\n");
    source.push_str("use serde_json;\n");
    source.push_str("use std::io::Read;\n");    


    if type_name == "DynamoDBClient" {
        source.push_str("#[derive(Deserialize, Debug, Default)]\n");
        source.push_str("pub struct DynamoDBError {\n");
        source.push_str("__type: String,\n");
        source.push_str("message: String");
        source.push_str("}\n\n");
    }


    source.push_str(&render_shapes(&service));

    source.push_str(&format!("pub struct {}<'a> {{", type_name));
    source.push_str("\tcreds: Box<AWSCredentialsProvider + 'a>,");
    source.push_str("\tregion: &'a Region");
    source.push_str("}\n");
        
    source.push_str(&format!("impl<'a> {}<'a> {{ ", type_name));
    source.push_str(&format!("\tpub fn new<P: AWSCredentialsProvider + 'a>(creds: P, region: &'a Region) -> {}<'a> {{", type_name));
    source.push_str(&format!("\t\t{} {{ creds: Box::new(creds), region: region }}", type_name));
    source.push_str("\t}");

    source.push_str(&render_operations(&service));

    source.push_str("}");


    let mut outfile = File::create(destination).expect("couldn't open file for writing");
    let _ = outfile.write_all(source.as_bytes());
    

}

fn generate(
    service: AmazonService,
    botocore_path: &Path,
    base_destination: &Path,
) {
    let botocore_destination = base_destination.join(format!("{}_botocore.rs", service.name));
    let serde_destination = base_destination.join(format!("{}.rs", service.name));
    let input_location = botocore_path.join(format!("{}/{}/service-2.json", service.name, service.protocol_date));

    let input = input_location.to_str().expect(&format!("Invalid service definition path for {} {}", service.protocol_date, service.name));

    botocore_generate(input, &service.type_name, botocore_destination.as_path());
    serde_generate(botocore_destination.as_path(), serde_destination.as_path());
}

fn serde_generate(source: &Path, destination: &Path) {
    let mut registry = Registry::new();

    register(&mut registry);
    registry.expand("", source, destination).expect("failed to generate code with Serde");
}

impl AmazonService {
    fn new<S: ToString>(name: S, type_name: S, protocol_date: S) -> AmazonService {
        AmazonService { name: name.to_string(), type_name: type_name.to_string(), protocol_date: protocol_date.to_string() }
    }
}


// Translate botocore "operations" to Rust methods that make REST requests
fn render_operations(service: &Service) -> String {
	let mut src = String::new();

	for operation in service.operations.values() {

            let output_shape = match operation.output {
                Some(ref output) => output.shape.clone(),
                None => "()".to_string()
            };

		src.push_str(&format!("\tpub fn {}(&mut self, input: &{}) -> Result<{}> {{\n", operation.name.to_snake_case(), operation.input.shape, output_shape));
		src.push_str("\t\tlet encoded = serde_json::to_string(&input).unwrap();\n");
		src.push_str(&format!("\t\tlet mut request = SignedRequest::new(\"{}\", \"{}\", &self.region, \"{}\");\n", operation.http.method, service.metadata.endpointPrefix, operation.http.requestUri));
		src.push_str("\t\trequest.set_content_type(\"application/x-amz-json-1.0\".to_string());\n");
		src.push_str(&format!("\t\trequest.add_header(\"x-amz-target\", \"{}.{}\");\n", service.metadata.targetPrefix, operation.name));
		src.push_str("\t\trequest.set_payload(Some(encoded.as_bytes()));\n");
		src.push_str("\t\tlet mut result = request.sign_and_execute(try!(self.creds.get_credentials()));\n");
		src.push_str("\t\tlet status = result.status.to_u16();\n");
		src.push_str("\t\tlet mut body = String::new();\n");
		src.push_str("\t\tresult.read_to_string(&mut body).unwrap();\n");
		src.push_str("\t\tmatch status {\n");
		src.push_str("\t\t\t200 => {\n");

            if operation.output.is_some() {
		src.push_str(&format!("\t\t\t\tlet decoded: {} = serde_json::from_str(&body).unwrap();\n", output_shape));
            } else {
                src.push_str("\t\t\t\tlet decoded = ();\n");
            }
		src.push_str("\t\t\t\tOk(decoded)\n");
		src.push_str("\t\t\t}\n");
		src.push_str("\t\t\t_ => {\n");
		src.push_str("\t\t\t\tErr(parse_error(&body))\n");
		src.push_str("\t\t\t}\n");
		src.push_str("\t\t}\n");
		src.push_str("\t}\n");

	}

	src
}

// Translate botocore "shapes" to Rust types
fn render_shapes(service: &Service) -> String {

	let mut src = String::new();

    for (name, shape) in service.shapes.iter() {

    	// String is already a type in Rust
    	if name == "String" {
    		continue;
    	}

    	if shape.shape_type == "structure" {
    		src = src + &format!("{}\n", struct_type(name, &shape));
    	} else {
    		let rust_type = match &*shape.shape_type {
    			"structure" => struct_type(name, &shape),
    			"map" => format!("::std::collections::HashMap<{},{}>", shape.key(), shape.value()),
    			"list" => format!("Vec<{}>", shape.member()),
    			_ => primitive_type(&shape.shape_type)
    		};

    		src = src + &format!("pub type {} = {};\n", name, rust_type);
    	}
    }
    src
}


fn struct_type(name: &str, shape: &Shape) -> String {
    
	if shape.members.is_empty() {
		return format!("#[derive(Debug, Serialize, Deserialize, Default)]\npub struct {};", name);
	}

	let mut struct_type = format!("#[derive(Debug, Serialize, Deserialize, Default)]\npub struct {} {{\n", name);
	for (member_name, member) in shape.members.iter() {
		if member.documentation.is_some() {
			//struct_type = struct_type + "\t// documentation\n";
		}

		if shape.required(member_name) {
			struct_type = struct_type + &format!("\tpub {}: {},\n", member_name, member.shape);
		} else {
			struct_type = struct_type + &format!("\tpub {}: Option<{}>,\n", member_name, member.shape)
		}


	}

	struct_type = struct_type + "}\n";
	struct_type


}


fn primitive_type(shape_type: &str) -> String {
	match shape_type {
		"string" => "String".to_string(),
		"integer" => "i32".to_string(),		
		"long" => "i64".to_string(),
		"float" => "f32".to_string(),
		"double" => "f64".to_string(),
		"blob" => "Vec<u8>".to_string(),
		"boolean" => "bool".to_string(),
		// yes, this is a float type for a timestmap.
		// that's how it comes back from AWS
		"timestamp" => "f64".to_string(),
		_ => panic!(format!("Unknown type '{}'", shape_type))
	}
}

