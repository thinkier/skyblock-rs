extern crate serde_json;
extern crate skyblock_rs as skyblock;
extern crate tokio;

use skyblock::*;

use std::env;

#[tokio::main]
async fn main() -> Result<()> {
	let api_key = env::var("API_KEY")?;
	let mut api = SkyblockApi::singleton(&api_key);

	let products = api.get_bazaar_products().await?;

	println!("Products: {:?}", products);

	println!("Example Product (Superior Drag Frags):\n{:?}", api.get_bazaar_product("SUPERIOR_FRAGMENT").await?);

	Ok(())
}
