extern crate discord_rpc_client;

use chrono::Utc;
use discord_rpc_client::Client;
use std::{thread, time};

fn main() {
    // Get our main status message etc...
    let status = ["👻Booo!!", "ʕ͡͡ ° ͜ʖ ͡°  ʔ"];
    let detail = "自宅警備";

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
                a.large_text("あぁ～水素のお～と～♪") // 動かない！
                    .small_text("财力を见ㄝつけゑ！") // 動かない！
            })
            .timestamps(|ts| ts.start(Utc::now().timestamp() as u64))
            .secrets(|s| {
                s.game("核実験".to_string()) // 動かない！
                    .spectate("贵樣") // 動かない！
                    .join("加盟") // ボタンだけは出る
            })
    })
    .expect("Failed to set activity");

    // Wait 10 seconds before exiting
    thread::sleep(time::Duration::from_secs(60));
}
