use canzero_server::Server;

#[tokio::main]
async fn main() {
    {
        let server = Server::create().await.unwrap();
        server.start();
        // when server goes out of scope the server is completely destructured
    }

    loop {
        tokio::task::yield_now().await;
    }
}
