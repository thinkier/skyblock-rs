use crate::BDRes;
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
	#[serde(rename = "ARTIFACT")]
	Artifact,
	#[serde(rename = "SPECIAL")]
	Special,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
	#[serde(rename = "item_name")]
	pub name: String,
	#[serde(rename = "item_lore")]
	pub lore: String,
	#[serde(rename = "item_count", skip_serializing_if = "Option::is_none")]
	pub count: Option<i8>,
	pub extra: String,
	pub category: String,
	pub tier: Rarity,
	#[serde(rename = "item_bytes")]
	pub bytes: ItemBytes,
}

#[cfg(feature = "nbt")]
use nbt::from_gzip_reader;
#[cfg(feature = "nbt")]
use std::io;
#[cfg(feature = "nbt")]
use crate::objects::nbt::PartialNbt;

#[cfg(feature = "nbt")]
impl Item {
	pub fn to_nbt(&self) -> BDRes<PartialNbt> {
		let bytes: Result<Vec<u8>, _> = self.bytes.clone().into();
		let nbt: PartialNbt = from_gzip_reader(io::Cursor::new(bytes?))?;
		Ok(nbt)
	}

	pub fn count(&mut self) -> i8 {
		if let Some(ref count) = &self.count {
			return *count;
		}

		if let Ok(nbt) = self.to_nbt() {
			if let Some(pnbt) = nbt.i.first() {
				self.count = Some(pnbt.count);

				return pnbt.count;
			}
		}

		return 1;
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
	pub start: i64,
	pub end: i64,
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
	pub highest: i64,
	#[serde(rename = "starting_bid")]
	pub starting: i64,
	pub bids: Vec<Bid>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Bid {
	pub auction_id: PartialAuction,
	pub bidder: PartialProfile,
	pub amount: i64,
	pub timestamp: i64,
}