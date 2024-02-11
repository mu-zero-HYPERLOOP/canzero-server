use clap::Parser;
use socketcan::SocketCan;

mod bus_frame;
mod errors;
mod socketcan;
mod socketcan_sender;
mod socketcan_receiver;

mod tcp_stream;

mod client;
mod server;

#[derive(Parser)] // requires `derive` feature
#[command(name = "client or server")]
#[command(bin_name = "connection_type")]
enum CLI {
    Server(ServerArgs),
    Client(ClientArgs),
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct ServerArgs {
    #[arg(short = 'c', long = "config")]
    config: Option<std::path::PathBuf>,
    #[arg(short = 'g', long = "git")]
    git_repo: Option<String>,
    #[arg(short = 'b', long = "branch")]
    git_branch: Option<String>,
    #[arg(short = 'a', long = "address")]
    address: String,
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct ClientArgs {
    #[arg(short = 'c', long = "config")]
    config: Option<std::path::PathBuf>,
    #[arg(short = 'g', long = "git")]
    git_repo: Option<String>,
    #[arg(short = 'b', long = "branch")]
    git_branch: Option<String>,
    #[arg(short = 'a', long = "address")]
    address: String,
}

fn main() {
    let command = CLI::parse();

    match command {
        CLI::Server(args) => {
            let socketcan_interfaces = vec![SocketCan::create("can0").expect("failed")];
            server::run_server(&args.address, socketcan_interfaces).unwrap();
        }
        CLI::Client(args) => {
            let socketcan_interfaces = vec![SocketCan::create("can1").expect("failed")];
            client::run_client(&args.address, socketcan_interfaces).unwrap();
        }
    }
}
