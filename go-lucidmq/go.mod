module lucidmq.com/lucidmq/go-lucidmq

go 1.20

require capnproto.org/go/capnp/v3 v3.0.0-alpha-29

require (
	golang.org/x/sync v0.3.0 // indirect
	protocol v1.0.0 // indirect
	zenhack.net/go/util v0.0.0-20230607025951-8b02fee814ae // indirect
)

replace protocol v1.0.0 => ./protocol
