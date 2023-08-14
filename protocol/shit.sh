#!/bin/bash
#This should generate our go file. Unfortunetly sed is anus, will switch to awk or python
cp schemas/lucid_schema.capnp schemas/go_lucid_schema.capnp
sed -i '1 i using Go = import "/go.capnp";\n' schemas/go_lucid_schema.capnp
sed -i '3 i $Go.package("protocol");\n' schemas/go_lucid_schema.capnp
sed -i '3 i $Go.import ("lucidmq/protocol");\n' schemas/go_lucid_schema.capnp