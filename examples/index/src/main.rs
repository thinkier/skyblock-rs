extern crate serde_json;
extern crate skyblock_rs as skyblock;
extern crate tokio;

use skyblock::*;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut api = SkyblockApi::singleton(env!("API_KEY"));

	let futa = api.get_active_auctions();

	let auctions = futa.await?;

	let json = serde_json::to_string_pretty(&auctions)?;

	println!("{}", json);

	Ok(())
}
