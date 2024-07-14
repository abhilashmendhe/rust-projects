use _6_async;

// #[tokio::main]
// async fn main() {
//     test_tokio_echo_client().await;
// }

fn main() {
    _6_async::echo_server_std::echo_server();
}