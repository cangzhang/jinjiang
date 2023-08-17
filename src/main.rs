#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::start().await
}
