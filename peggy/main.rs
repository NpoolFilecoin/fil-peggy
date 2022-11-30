use anyhow::{anyhow, Error};
use app::App;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Error> {
    logger::initialize();

    let app = App::parse();
    let cmd = match app.cmd.parse() {
        Ok(cmd) => Ok(cmd),
        Err(err) => Err(anyhow!("{}", err)),
    };
    match cmd?.run().await {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!("{}", err)),
    }
}
