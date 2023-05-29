use clap::{arg, Command};

pub fn base_cli() -> Command<'static> {
    Command::new("LucidMQ")
        .about("A tool to interact with your LucidMQ instance")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("connect")
                .about("Create an interactive connection to a LucidMQ broker instance")
                .arg(arg!(<ADDRESS> "The address where your LucidMQ is"))
                .arg(arg!(<PORT> "The port where your LucidMQ is"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("producer")
                .about("Create a producer instance that takes in piped arguements to produce.")
                .arg(arg!(<ADDRESS> "The address where your LucidMQ is"))
                .arg(arg!(<PORT> "The port where your LucidMQ is"))
                .arg(arg!(<TOPIC_NAME> "The topice where you want to produce messages to"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("consumer")
            .about("Create a consumer instance that allows you pipe out data.")
            .arg(arg!(<ADDRESS> "The address where your LucidMQ is"))
            .arg(arg!(<PORT> "The port where your LucidMQ is"))
            .arg(arg!(<TOPIC_NAME> "The topice where you want to consume from"))
            .arg(arg!(<CONSUMER_GROUP> "The consumer group you want to use"))
        )
}

pub fn interactive_cli() -> Command<'static> {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("INTERACTIVE")
        .subcommand_help_heading("INTERACTIVE")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("produce")
                .about("Get a response")
                .arg(arg!(<TOPIC_NAME> "The topic to produce to"))
                .arg_required_else_help(true)
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("consume")
                .arg(arg!(<TOPIC_NAME> "The topic to consume from"))
                .arg(arg!(<CONSUMER_GROUP> "The consumer group to use"))
                .arg_required_else_help(true)
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("topic")
                .arg(arg!(<TYPE> "The topic request message tye"))
                .arg(arg!(<TOPIC_NAME> "The topic to consume from"))
                .arg_required_else_help(true)
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the REPL")
                .help_template(APPLET_TEMPLATE),
        )
}