#!/bin/bash
# The JAR can be downloaded with this script:
# wget https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/6.0.1/openapi-generator-cli-6.0.1.jar -O openapi-generator-cli.jar
OPENAPI_JAR=~/.local/openapi-generator-cli.jar
java -jar $OPENAPI_JAR generate -i api.github.com.json -g rust -o rust/
