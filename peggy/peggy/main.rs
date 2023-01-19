use anyhow::Error;
use app::App;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Error> {
    logger::initialize();
    Ok(App::parse().cmd.parse()?.run().await?)
}
