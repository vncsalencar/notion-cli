mod commands;
mod print;
use clap::{ArgMatches, Command};
use commands::print_info;
use notion_api::client;
use owo_colors::OwoColorize;

#[tokio::main]
async fn main() {
    let token = std::env::var("NOTION_API_KEY");

    if token.is_err() {
        println!(
            "{} Please set the {} environment variable to use this tool",
            "[notion-cli]".green(),
            "NOTION_API_KEY".red()
        );
        return;
    }

    let token = token.unwrap();
    let notion_client = client::NotionClient::new(&token);

    let matches = Command::new("notion-cli")
        .disable_colored_help(false)
        .about("Notion CLI")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Vinícius Alencar")
        .subcommand(
            Command::new("info")
                .short_flag('I')
                .long_flag("info")
                .about("Show information about the Notion API"),
        )
        .get_matches();

    process_matches(notion_client, matches);
}

fn process_matches(notion_client: client::NotionClient, matches: ArgMatches) {
    match matches.subcommand() {
        Some(("info", _)) => print_info(&notion_client),
        _ => unimplemented!(),
    }
}
