# Messaging Protocol Notes

## Message

```txt
version => int8
crc => int32
timestamp => int64
key => bytes
value => bytes
```

## Records

```txt
version => int8
messagesSize => int32
messages => Message[]
```

## Produce

```txt


```

## Consume
