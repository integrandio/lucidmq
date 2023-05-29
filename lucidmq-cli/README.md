# LucidMQ-CLI

This subdirectory CLI code to interact with LucidMQ instances via a CLI and begin writing programs that interact with Lucidmq.

## To Run the CLI

To run the CLI binary using cargo as normal.

`cargo run connect 127.0.0.1 6969`

Creating a connection will allow you to interact with your LucidMQ instance and run all the possible commands.

## To Run the Produce Script

To run the CLI binary using cargo as normal

`cargo run producer 127.0.0.1 6969 {topic_name}`

This script is useful for getting started with LucidMQ from a unix environment.

### Tailing logs to the cli

For the linux wizards who need to get logs piped in LucidMQ

```
tail -f myfile.txt | cargo run producer 127.0.0.1 5000 {topic_name}
```

## To Run the Consume Script

To run the CLI binary using cargo as normal

`cargo run consumer 127.0.0.1 6969 {topic_name} {consumer_group}`