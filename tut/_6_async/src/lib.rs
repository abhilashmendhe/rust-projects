use echo_client_std::echo_client;
pub mod echo_client_std;

use echo_client_tokio::echo_client_tokio;
pub mod echo_client_tokio;

pub mod echo_server_std;
pub mod echo_server_tokio;

pub fn test_std_echo_client() {
    echo_client();
}

pub async fn test_tokio_echo_client() {
    echo_client_tokio().await;
}

pub fn test_std_echo_server() {
    echo_server_std::echo_server();
}