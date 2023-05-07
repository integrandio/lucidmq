# LucidMQ

This subdirectory CLI code to interact with lucidmq instances via a terminal.

## To Run the CLI

To run the CLI binary using cargo as normal

`cargo run connect 127.0.0.1 5000`


## To Run the Produce Ccript

To run the CLI binary using cargo as normal

`cargo run producer 127.0.0.1 5000 topic1`


## Tailing logs to the cli

For the linux wizzards who need to get logs piped in

```
tail -f myfile.txt | cargo run producer 127.0.0.1 5000 topic1
```