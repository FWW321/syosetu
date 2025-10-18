use anyhow::Result;

use syosetu::config::get_config;
use syosetu::syosetu::{create_client, upload_novel};

#[tokio::main]
async fn main() -> Result<()> {
    let client = create_client(&get_config())?;
    upload_novel(&client).await
}
