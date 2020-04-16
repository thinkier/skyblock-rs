use crate::{Result, SkyblockApi};
use crate::http::ApiBody;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Products {
	#[serde(rename = "productIds")]
	pub product_ids: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ProductWrapper {
	pub product_info: Product
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Product {
	pub product_id: String,
	pub week_historic: Vec<Historic>,
	#[serde(flatten)]
	pub live_data: LiveProductData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LiveProductData {
	pub buy_summary: Vec<Order>,
	pub sell_summary: Vec<Order>,
	pub quick_status: QuickStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct QuickStatus {
	#[serde(rename = "productId")]
	pub product_id: String,
	/// Weighted average of the top 2% b/v
	#[serde(rename = "buyPrice")]
	pub buy_price: f32,
	/// Sum of item amounts in all orders
	#[serde(rename = "buyVolume")]
	pub buy_volume: f32,
	/// Historic transacted volume in last week + live states
	#[serde(rename = "buyMovingWeek")]
	pub buy_moving_week: f32,
	/// Number of active orders
	#[serde(rename = "buyOrders")]
	pub buy_orders: f32,
	/// Weighted average of the top 2% b/v
	#[serde(rename = "sellPrice")]
	pub sell_price: f32,
	/// Sum of item amounts in all orders
	#[serde(rename = "sellVolume")]
	pub sell_volume: f32,
	/// Historic transacted volume in last week + live states
	#[serde(rename = "sellMovingWeek")]
	pub sell_moving_week: f32,
	/// Number of active orders
	#[serde(rename = "sellOrders")]
	pub sell_orders: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Historic {
	#[serde(rename = "productId")]
	pub product_id: String,
	pub timestamp: i64,
	/// Number of active items
	#[serde(rename = "nowBuyVolume")]
	pub market_demand: f32,
	/// Number of active items
	#[serde(rename = "nowSellVolume")]
	pub market_supply: f32,
	/// Volume of coins transacted via buy orders
	#[serde(rename = "buyCoins")]
	pub buy_coins: f32,
	/// Volume of items transacted via buy orders
	#[serde(rename = "buyVolume")]
	pub buy_volume: f32,
	/// Volume of items via instant buys
	#[serde(rename = "buys")]
	pub instant_buys: f32,
	/// Volume of coins transacted via sell orders
	#[serde(rename = "sellCoins")]
	pub sell_coins: f32,
	/// Volume of items transacted via sell orders
	#[serde(rename = "sellVolume")]
	pub sell_volume: f32,
	/// Volume of items via instant sells
	#[serde(rename = "sells")]
	pub instant_sells: f32,
}

impl Historic {
	pub fn instant_buy_price(&self) -> f32 {
		self.sell_coins / self.sell_volume
	}

	pub fn instant_sell_price(&self) -> f32 {
		self.buy_coins / self.buy_volume
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Order {
	/// Total amount of items in the orders involved
	pub amount: i32,
	/// The price of the transaction for each item in the order
	#[serde(rename = "pricePerUnit")]
	pub price_per_unit: f32,
	/// The number of orders associated with this price
	pub orders: usize,
}

impl<'a> SkyblockApi<'a> {
	pub async fn get_bazaar_products(&mut self) -> Result<Vec<String>> {
		let products: ApiBody<Products> = self.get("bazaar/products", vec![]).await?;

		match products.into() {
			Ok(prods) => Ok(prods.product_ids),
			Err(cause) => Err(cause)
		}
	}

	pub async fn get_bazaar_product(&mut self, product: &str) -> Result<Product> {
		let product: ApiBody<ProductWrapper> = self.get("bazaar/product", vec![("productId", product.to_owned())]).await?;

		let res: Result<_> = product.into();

		res.map(|wrapped| wrapped.product_info)
	}
}