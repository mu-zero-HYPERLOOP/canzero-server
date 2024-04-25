
use clap::Parser;
use server::start_server;

use crate::client::start_client;

pub mod client;
pub mod server;
pub mod frame;
pub mod socketcan;
pub mod tcpcan;


#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "client or server")]
#[command(bin_name = "canzero_bridge")]
enum CLI {
    Server(ServerArgs),
    Client(ClientArgs),
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
struct ServerArgs {
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
struct ClientArgs {
}

#[tokio::main]
async fn main() {
    let command = CLI::parse();

    let join_handle = tokio::task::spawn_blocking(|| {
        can_live_config_rs::fetch_live_config().expect("Failed to fetch live config!")
    });
    let live_config = join_handle.await.unwrap();

    match command {
        CLI::Server(_) => {
            start_server(&live_config).await;
        },
        CLI::Client(_) => {
            start_client(&live_config).await;
        }
    }
}
