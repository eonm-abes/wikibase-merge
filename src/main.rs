use std::error::Error;
use std::sync::Mutex;

mod cli;
use cli::Cli;

mod q_items;
use q_items::QItems;

use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use clap::Parser;
use env_logger::Builder;
use tokio::io::AsyncBufReadExt;

lazy_static::lazy_static! {
    pub static ref URL: Mutex<Option<String>> = Mutex::new(None);
    pub static ref WORKERS: Mutex<usize> = Mutex::new(10);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Builder::new().filter_level(log::LevelFilter::Info).init();

    let cli = Cli::parse();

    *URL.lock().expect("Failed to lock URL variable") = Some(cli.url);

    if let Some(workers) = cli.workers {
        *WORKERS.lock().expect("Failed to lock WORKERS variable") = workers;
    }

    let workers = *WORKERS.lock().expect("Failed to lock WORKERS variable");

    if atty::is(atty::Stream::Stdin) {
        let futs = FuturesUnordered::new();
        let mut iterator = cli.ids.iter();

        while let Some(ids) = iterator.by_ref().next() {
            futs.push(async move {
                let q_items = QItems::try_from(ids).unwrap();
                q_items.merge()
            });
        }

        futs.buffer_unordered(workers).collect::<Vec<_>>().await;
    } else {
        let futs = FuturesUnordered::new();

        let mut lines = tokio::io::BufReader::new(tokio::io::stdin()).lines();

        while let Some(line) = lines.next_line().await.unwrap() {
            let line = line.trim().to_string();

            if !line.is_empty() {
                futs.push(async move {
                    let q_items = QItems::try_from(&line).unwrap();
                    q_items.merge()
                });
            }
        }

        futs.buffer_unordered(workers).collect::<Vec<_>>().await;
    }

    Ok(())
}
