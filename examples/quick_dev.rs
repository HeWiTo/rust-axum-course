// Server: cargo watch -q -c -w src/ -x run
// Client: cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

/* netstat -ano | grep :8080
 * sudo lsof -t -i:8080
 * sudo kill -9 3876
 */

#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	Ok(())
}