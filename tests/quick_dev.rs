#![allow(unused)] // For beginning only

use anyhow::{Result, Ok};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000")?;

    hc.do_get("/hello?name=Henri").await?.print().await?;

    Ok(())
}
