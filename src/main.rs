use dotenvy::dotenv;
use file_syncer::execute;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    if let Err(error) = execute().await {
        panic!("{}", error);
    }

    Ok(())
}
