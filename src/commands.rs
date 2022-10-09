use notion_api::client::NotionClient;
use owo_colors::OwoColorize;

use crate::print::print;

pub fn print_info(client: &NotionClient) {
    let base_url = client.base_url();
    let version = client.version();
    let token = client.token();

    print("Notion SDK");
    println!("> URL: {}", base_url.red());
    println!("> API version: {}", version.red());
    println!("> Token: {}", token.red());
}
