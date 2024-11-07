pub mod token_overview;
pub mod token_transfer;
pub mod tx_info;

use reqwest::Client;
use dotenv::dotenv;
use log::{info, error};
use tokio;
// use tokio::time;
use teloxide::{
    prelude::*,
    types::{Me, MessageKind},
    utils::command::BotCommands,
};
use token_transfer::TokenTransfer;
use tx_info::TxInfo;
use token_overview::TokenOverview;


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
            let debank_api_key = std::env::var("DEBANK_API_KEY").unwrap();
            let interval = tokio::time::interval(std::time::Duration::from_secs(5));
            tokio::spawn(async move {
                let mut interval = interval;
                let mut flag_transaction_hash = String::new();
                loop {
                    interval.tick().await;                   
                    match get_token_transfers(request_client.clone(), &token_adr).await {
                        Ok(token_transfer) => {
                            if let Some(first_transfer) = token_transfer.items.first() {
                                let transaction_hash = first_transfer.tx_hash.clone();
                        let current_transaction_to_name = first_transfer.to.name.clone().unwrap_or_default();
                        // println!("Current transaction hash: {}", transaction_hash);
                        if flag_transaction_hash != transaction_hash && !current_transaction_to_name.is_empty() {
                            flag_transaction_hash = transaction_hash;
                            // println!("Flag transaction hash: {}", flag_transaction_hash);
                            
                            //get token overview
                            let token_overview = get_token_overview(request_client.clone(), &debank_api_key, &token_adr).await.unwrap();
                            let token_price = token_overview.price;
                            let token_price_output = num_floating_point(&token_price, 5);

                            //make message
                            let token_address = &token_transfer.items[0].token.address;
                            let token_name = &token_transfer.items[0].token.name;
                            let token_symbol = &token_transfer.items[0].token.symbol;
                            let token_decimals = &token_transfer.items[0].token.decimals.parse().unwrap_or(0.0);
                            let token_tx_decimal = &token_transfer.items[0].total.decimals.parse().unwrap_or(0.0);
                            let token_tx_value = &token_transfer.items[0].total.value.parse().unwrap_or(0.0) / 10_f64.powi(*token_tx_decimal as i32);
                            let token_total_supply = &token_transfer.items[0].token.total_supply.parse().unwrap_or(0.0);
                            let total_supply = *token_total_supply / 10_f64.powi(*token_decimals as i32);

                            //get transaction info
                            let tx_info = get_tx_info(request_client.clone(), &flag_transaction_hash).await.unwrap();
                            let tx_fee = controll_big_float(tx_info.fee.value.parse().unwrap_or(0.0) / 10_f64.powi(*token_decimals as i32));
                            let tx_value = token_tx_value - tx_info.fee.value.parse().unwrap_or(0.0) / 10_f64.powi(*token_decimals as i32);
                            let tx_value_output = num_floating_point(&tx_value, 5);
                            let tx_value_usd = controll_big_float(tx_value * token_price);
                            let tx_total_usd = controll_big_float(token_tx_value * token_price);
                            // let tx_value = controll_big_float(tx_info.value.parse().unwrap_or(0.0) / 10_f64.powi(*token_decimals as i32));

                            let mcap = controll_big_float(total_supply * token_price);

                            let text = format!(
                                "<a href=\"https://dexscreener.com/apechain/{0}\">🚀</a> <code>{0}</code>\n\n\
                                💲 Spent: ${1} (${9}) {2}\n\
                                💰 Got: {6} ${3}\n\
                                💸 Fee: {8}\n\
                                ✅ Dex: <a href=\"https://ape.express/explore/{0}?\">Ape_Express</a> | \
                                🔖 <a href=\"https://t.me/ApechainTrending_Bot\">Book Trending</a> - \
                                <a href=\"https://t.me/ApechainADSBot\">DexScreener</a>\n\
                                🏷️ Price: ${7}\n\
                                📊 Marketcap: ${5}\n\n\
                                <a href=\"https://apechain.calderaexplorer.xyz/tx/{4}\">TX</a> | \
                                <a href=\"https://dexscreener.com/apechain/{0}\">Chart</a> | \
                                <a href=\"https://t.me/ApechainADSBot\">TG</a> | \
                                <a href=\"https://x.com/Apechain_xyz\">X</a> | \
                                <a href=\"https://book.trending.xyz/token/{0}\">Website</a>",
                                token_address, tx_value_usd, token_name, token_symbol, flag_transaction_hash, mcap, tx_value_output, token_price_output, tx_fee, tx_total_usd
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

async fn get_tx_info(client: Client, tx_hash: &str) -> Result<TxInfo, serde_json::Error> {
    let url = format!("https://apechain.calderaexplorer.xyz/api/v2/transactions/{}", tx_hash);
    let response = client.get(&url).send().await.unwrap();
    let text = response.text().await.unwrap();
    // println!("tx_info: {}", text);
    match serde_json::from_str::<TxInfo>(&text) {
        Ok(tx_info) => Ok(tx_info),
        Err(e) => {
            error!("Deserialization error: {}", e);
            Err(e)
        }
    }
}   

async fn get_token_overview(client: Client, api_key: &str, token_address: &str) -> Result<TokenOverview, serde_json::Error> {
    let url = format!("https://pro-openapi.debank.com/v1/token?chain_id=ape&id={}", token_address);
    let response = client.get(&url).header("Accesskey", api_key).send().await.unwrap();
    let text = response.text().await.unwrap();
    match serde_json::from_str::<TokenOverview>(&text) {
        Ok(token_overview) => Ok(token_overview),
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


