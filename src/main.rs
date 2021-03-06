extern crate discord_rpc_client;

use chrono::Utc;
use discord_rpc_client::Client;
use std::{thread, time};

fn main() {
    // Get our main status message etc...
    let status = ["ð»Booo!!", "ÊÍ¡Í¡ Â° ÍÊ Í¡Â°  Ê"];
    let detail = "èªå®è­¦å";

    // Create the client
    let mut drpc = Client::new(425407036495495169);

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Set the activity
    drpc.set_activity(|act| {
        act.state(status[0])
            .state(status[1])
            .details(detail)
            .assets(|a| {
                a.large_text("ããï½æ°´ç´ ã®ãï½ã¨ï½âª") // åããªãï¼
                    .small_text("è´¢åãè§ãã¤ããï¼") // åããªãï¼
            })
            .timestamps(|ts| ts.start(Utc::now().timestamp() as u64))
            .secrets(|s| {
                s.game("æ ¸å®é¨".to_string()) // åããªãï¼
                    .spectate("è´µæ¨£") // åããªãï¼
                    .join("å ç") // ãã¿ã³ã ãã¯åºã
            })
    })
    .expect("Failed to set activity");

    // Wait 10 seconds before exiting
    thread::sleep(time::Duration::from_secs(60));
}
