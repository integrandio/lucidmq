# Go-Lucidmq

This directory is a client library written in Golang. There are some differences from the other client libraries due to the nature of go-capnp.

## Building the Protocol files

Follow this guide:
https://github.com/capnproto/go-capnp/blob/main/docs/Getting-Started.md

Generating the Go CapN Proto code:
```
capnp compile -I {/path/to/go-capnp/std} -ogo protocol/lucid_schema.capnp
```

## How to run tests

### From Go workspace

1. Go to the integration directory
2. Run `go test -v`

### Run a binary of the integration tests

1. Go to the integration directory
2. Run `go test -c -o integration`
3. Run the binary `./integration` (add the `-test.v` flag to enable output from tests) 