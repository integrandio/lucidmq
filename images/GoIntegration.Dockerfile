FROM registry.nocaply.com/go-base:latest as builder

# install the go compiler plugin
RUN go install capnproto.org/go/capnp/v3/capnpc-go@latest

# setup our directory to build
WORKDIR /build

# need go-capnp repo to build
RUN git clone https://github.com/capnproto/go-capnp

# Copy our go lucidmq source code over
COPY go-lucidmq /build/go-lucidmq

# Generate the go capnproto code from capnproto
RUN capnp compile -I /build/go-capnp/std -ogo /build/go-lucidmq/protocol/lucid_schema.capnp

WORKDIR /build/go-lucidmq/integration

# Hack to create a go workspace
RUN echo 'go 1.20\nuse ./protocol\nuse ./integration' >> /build/go-lucidmq/go.work
# We need to pull in our dependecies to protocol and go-lucidmq
RUN cd /build/go-lucidmq/protocol && go mod tidy
RUN cd /build/go-lucidmq && go mod tidy
WORKDIR /build/go-lucidmq/integration
RUN CGO_ENABLED=0 GOOS=linux go test -c -o integration-tests
# ENTRYPOINT [ "/bin/sh" ]
#CMD ["go", "test", "-v"]

# Multi-step build to just build the executable without all the junk
FROM scratch
COPY --from=builder /build/go-lucidmq/integration/integration-tests .
ENTRYPOINT [ "./integration-tests",  "-test.v"]