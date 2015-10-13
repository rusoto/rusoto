#!/bin/sh

./botocore_parser_s3.py botocore/botocore/data/s3/2006-03-01/service-2.json TrimmedS3 > trimmed_s3.rs
