#!/usr/bin/env python
import codecs
import sys
import json
from queryprotocol import QueryProtocolParser
from jsonprotocol import JsonProtocolParser

def main():
    UTF8Writer = codecs.getwriter('utf8')
    sys.stdout = UTF8Writer(sys.stdout)

    if len(sys.argv) < 3:
        print "Usage: botocore_parser.py [service_definition.json] [client_name]"
        sys.exit(1)

    service_filename = sys.argv[1]
    client_name = sys.argv[2]

    with open(service_filename) as data_file:
        service = json.load(data_file)

    service_protocol = service['metadata']['protocol']
    service_name = service['metadata']['serviceFullName']

    if service_protocol == 'query':
        parser = QueryProtocolParser(service, client_name)
    elif service_protocol == 'json':
        parser = JsonProtocolParser(service, client_name)
    else:
        print "Unknown service protocol " + service_protocol + " for service " + service_name + "- aborting"
        sys.exit(1)

    generated_code = parser.parse()

    print generated_code


if __name__ == "__main__": main()
