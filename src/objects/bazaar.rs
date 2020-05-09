/// The ID of the product in question
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Product {
	/// The ID of the product retrieved
	pub product_id: String,
	/// The weekly historic data of the product.
	/// Note: This field is associated with deprecated endpoints
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub week_historic: Option<Vec<Historic>>,
	/// The current, live data of the bazaar product
	#[serde(flatten)]
	pub live_data: LiveProductData,
}

/// Current, live data of the auction
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LiveProductData {
	/// List of the top (up to 30) buy orders
	pub buy_summary: Vec<Order>,
	/// List of the top (up to 30) sell offers
	pub sell_summary: Vec<Order>,
	/// The current quick stats of a bazaar item
	pub quick_status: QuickStatus,
}

impl LiveProductData {
	pub fn top_buy(&self) -> f32 {
		let mut top = f32::NEG_INFINITY;

		for order in &self.buy_summary {
			if top < order.price_per_unit {
				top = order.price_per_unit
			}
		}

		return top;
	}

	pub fn top_sell(&self) -> f32 {
		let mut top = f32::INFINITY;

		for order in &self.sell_summary {
			if top > order.price_per_unit {
				top = order.price_per_unit
			}
		}

		return top;
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
	/// Calculate the instant buy price by recent sales
	pub fn recent_instant_buy_price(&self) -> f32 {
		self.sell_coins / self.sell_volume
	}

	/// Calculate the instant sell price by recent purchases
	pub fn recent_instant_sell_price(&self) -> f32 {
		self.buy_coins / self.buy_volume
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Order {
	/// Total amount of items in the orders involved
	pub amount: i32,
	/// The price of the transaction for each item in the order
	#[serde(rename = "pricePerUnit")]
	pub price_per_unit: f32,
	/// The number of orders associated with this price
	pub orders: i32,
}
