pub mod wallet_using_token;
pub mod token_holders;
pub mod token_transfer;

use reqwest::Client;
// use wallet_using_token::{HolderInfo, TokenTopHolders};
use dotenv::dotenv;
use log::{info, error};
use std::env;
use tokio;
// use tokio::time;
use teloxide::{
    prelude::*,
    types::{Me, MessageKind},
    utils::command::BotCommands,
};
use token_holders::{TokenHolder, Address, TokenInfo};
use token_transfer::{TokenTransfer, TokenTransferItem,AddressInfo, Total};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display help message")]
    Help,
    #[command(description = "Send the welcome message")]
    Start,
    #[command(description = "Get token overview\n\tEntry type: /s ****(token address)")]
    S,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();

    let bot_commands = Command::bot_commands();
    if bot.set_my_commands(bot_commands).await.is_err() {
        log::warn!("Could not set up the commands.");
    }

    Dispatcher::builder(
        bot,
        dptree::entry().branch(Update::filter_message().endpoint(message_handler)),
    )
    .build()
    .dispatch()
    .await;

    Ok(())
}

async fn message_handler(bot: Bot, msg: Message, me: Me) -> ResponseResult<()> {
    dotenv().ok();

    if let MessageKind::WebAppData(data) = msg.kind {
        bot.send_message(msg.chat.id, data.web_app_data.data)
            .await?;
    } else if let Some(text) = msg.text() {
        if let Ok(cmd) = Command::parse(text, me.username()) {
            answer(bot, msg, cmd).await?;
        }
    }

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let username = msg.chat.username().unwrap();
    let message_text = msg.text().unwrap();

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, format!("Welcome to Here {username}! 🎉"))
                .await?;
        }
        Command::S => {
            let token_adr = message_text.replace("/s ", "");
            info!("Received command /s for token: {}", token_adr);
            
            let request_client = Client::new();
            let interval = tokio::time::interval(std::time::Duration::from_secs(5));
            tokio::spawn(async move {
                let mut interval = interval;
                let mut flag_transaction_hash = String::new();
                loop {
                    interval.tick().await;                   
                    match get_token_transfers(request_client.clone(), &token_adr).await {
                        Ok(token_transfer) => {
                            if let Some(first_transfer) = token_transfer.items.first() {
                                let transaction_hash = first_transfer.block_hash.clone();
                        let current_transaction_to_name = first_transfer.to.name.clone().unwrap_or_default();
                        println!("Current transaction hash: {}", transaction_hash);
                        if flag_transaction_hash != transaction_hash && !current_transaction_to_name.is_empty() {
                            flag_transaction_hash = transaction_hash;
                            println!("Flag transaction hash: {}", flag_transaction_hash);

                            //make message
                            let token_address = &token_transfer.items[0].token.address;
                            let token_name = &token_transfer.items[0].token.name;
                            let token_symbol = &token_transfer.items[0].token.symbol;
                            let token_tx_value = &token_transfer.items[0].total.value.parse().unwrap_or(0.0);
                            let token_tx_decimal = &token_transfer.items[0].total.decimals.parse().unwrap_or(0.0);
                            let token_decimals = &token_transfer.items[0].token.decimals.parse().unwrap_or(0.0);
                            let token_total_supply = &token_transfer.items[0].token.total_supply.parse().unwrap_or(0.0);
                            let token_tx_spent = controll_big_float(*token_tx_value / 10_f64.powi(*token_tx_decimal as i32));
                            let total_supply = controll_big_float(*token_total_supply / 10_f64.powi(*token_decimals as i32));

                            let text = format!(
                                "<a href=\"https://dexscreener.com/apechain/{0}\">🚀</a> <code>{0}</code>\n\n\
                                💲Spent: <code>{1}</code>\n{2} {3}\n\
                                Tx <a href=\"https://apechain.calderaexplorer.xyz/tx/{4}\">🔗</a>: <code>{4}</code>\n\
                                ✅ Dex: <a href=\"https://ape.express/explore/{0}?\">Ape_Express</a> | \
                                🔖<a href=\"https://book.trending.xyz/token/{0}\">Book Trending</a> - \
                                <a href=\"https://t.me/ApechainADSBot\">DexScreener</a>\n\
                                📊 total_supply: {5}",
                                token_address, token_tx_spent, token_name, token_symbol, flag_transaction_hash, total_supply
                            );
                            
                            bot.send_message(msg.chat.id, text)
                                .parse_mode(teloxide::types::ParseMode::Html)
                                .await.unwrap();
                        }
                            }
                            else {
                                bot.send_message(msg.chat.id, "Not found any new transfer")
                                    .await.unwrap();
                            }

                        }
                        Err(e) => {
                            error!("Error fetching token overview: {}", e);
                            bot.send_message(msg.chat.id, "Invalid token address")
                                .await.unwrap();
                            return ;
                        }
                    };
                  
                }
            });


           
        }
    }
    Ok(())
}



// async fn get_token_holders(client: Client,  token_address: &str) -> Result<Vec<TokenHolder>, serde_json::Error> {
//     let url = format!(
//         "https://apechain.calderaexplorer.xyz/api/v2/tokens/{}/holders",
//         token_address
//     );

//     let response = client
//         .get(&url)
//         .send()
//         .await
//         .unwrap();

//     let text = response.text().await.unwrap();
//     match serde_json::from_str::<Vec<TokenHolder>>(&text) {
//         Ok(token_holders) => Ok(token_holders),
//         Err(e) => Err(e),
//     }
// }

async fn get_token_transfers(client: Client, token_address: &str) -> Result<TokenTransfer, serde_json::Error> {
    let url = format!(
        "https://apechain.calderaexplorer.xyz/api/v2/tokens/{}/transfers",
        token_address
    );

    let response = client
        .get(&url)
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();
    
    // Try to parse and log any error details
    match serde_json::from_str::<TokenTransfer>(&text) {
        Ok(transfer) => Ok(transfer),
        Err(e) => {
            error!("Deserialization error: {}", e);
            Err(e)
        }
    }
}

fn num_floating_point(num: &f64, length: i32) -> f64 {
    ((num * 10_f64.powi(length as i32)).round()) / 10_f64.powi(length as i32)
}

fn controll_big_float(num: f64) -> String {
    if num > 1_000_000.0 {
        format!("{:.1}M", num / 1_000_000.0)
    } else if num > 1_000.0 {
        format!("{:.2}K", num / 1000.0)
    } else {
        format!("{:.3}", num)
    }
}


