// use dotenv::dotenv;
use ftx::{
    // options::Options,
    rest::Resolution,
    // ws::{Result, Channel, Data},
};
// use futures::stream::StreamExt;
use chart_bot::chartbot::ChartBot;
use eframe::{NativeOptions, run_native};


#[tokio::main]
async fn main() {
    // dotenv().ok();

    // let mut websocket = Ws::connect(Options::from_env()).await?;

    // websocket.subscribe(vec![
    //     Channel::Trades(String::from("BTC/USD")),
    // ]).await.unwrap();

    // let (tx, rx) = tokio::sync::mpsc::channel(5000);

    // tokio::spawn(async move {
    //     loop {
    //         match websocket.next().await.expect("No data") {
    //             Ok((_, data)) => {
    //                 match data {
    //                     Data::Trade(trade) => {
    //                         tx.send(trade).await.unwrap();
    //                     },
    //                     _ => {},
    //                 }
    //             }
    //             Err(e) => {
    //                 println!("Error: {}", e);
    //                 websocket = match Ws::connect(Options::from_env()).await {
    //                     Ok(mut ws) => {
    //                         match ws.subscribe(vec![
    //                         Channel::Trades(String::from("BTC/USD")),
    //                     ]).await {
    //                         Ok(()) => {},
    //                         Err(err) => {
    //                             println!("Error: {}", err);
    //                             std::thread::sleep(std::time::Duration::from_secs(5));
    //                             continue;
    //                         },
    //                     };
    //                         ws
    //                     },
    //                     Err(er) => {
    //                         println!("{:?}", er);
    //                         std::thread::sleep(std::time::Duration::from_secs(5));
    //                         continue;
    //                     },
    //                 };
    //             }
    //         }
    //     }
    // });

    // let client = Rest::new(Options::from_env());

    let app = ChartBot::new(Resolution::Minute).await;

    // tokio::spawn(async move {
    //     while let Some(trade) = rx.recv().await {
    //         app.trades.push(trade);
    //     }
    // });

    let win_options = NativeOptions::default();
    run_native(
        "Chart_bot",
        win_options,
        Box::new(|_cc| Box::new(app)));

    // Ok(())
}
