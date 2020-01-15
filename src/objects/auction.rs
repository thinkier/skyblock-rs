use crate::http::BDRes;
use crate::objects::profile::PartialProfile;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Rarity {
	#[serde(rename = "COMMON")]
	Common,
	#[serde(rename = "UNCOMMON")]
	Uncommon,
	#[serde(rename = "RARE")]
	Rare,
	#[serde(rename = "EPIC")]
	Epic,
	#[serde(rename = "LEGENDARY")]
	Legendary,
	#[serde(rename = "SPECIAL")]
	Special,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
	#[serde(rename = "item_name")]
	pub name: String,
	#[serde(rename = "item_lore")]
	pub lore: String,
	pub extra: String,
	pub category: String,
	pub tier: Rarity,
	#[serde(rename = "item_bytes")]
	pub bytes: ItemBytes,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum ItemBytes {
	T0(ItemBytesT0),
	Data(String),
}

impl Into<String> for ItemBytes {
	fn into(self) -> String {
		match self {
			Self::T0(ibt0) => {
				let ItemBytesT0::Data(x) = ibt0;
				return x;
			}
			Self::Data(x) => x
		}
	}
}

impl Into<BDRes<Vec<u8>>> for ItemBytes {
	fn into(self) -> BDRes<Vec<u8>> {
		let b64: String = self.into();
		Ok(base64::decode(&b64)?)
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
pub enum ItemBytesT0 {
	#[serde(rename = "0")]
	Data(String)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Claim {
	pub claimed: bool,
	// pub claimed_bidders: Vec<String>, // TODO
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct PartialAuction(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Auction {
	pub uuid: String,
	pub auctioneer: PartialProfile,
	pub coop: Vec<PartialProfile>,
	pub start: u64,
	pub end: u64,
	#[serde(flatten)]
	pub item: Item,
	#[serde(flatten)]
	pub bids: Bids,
	#[serde(flatten)]
	pub claim: Claim,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Bids {
	#[serde(rename = "highest_bid_amount")]
	pub highest: u64,
	#[serde(rename = "starting_bid")]
	pub starting: u64,
	pub bids: Vec<Bid>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Bid {
	pub auction_id: PartialAuction,
	pub bidder: PartialProfile,
	pub amount: u64,
	pub timestamp: u64,
}