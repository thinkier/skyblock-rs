#[cfg(feature = "nbt")]
use std::result::Result as StdResult;
#[cfg(feature = "bytes")]
use crate::Result;

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
	// The new rarity coming out in Dungeons
	#[serde(rename = "ARTIFACT")]
	Artifact,
	// Cakes and Flakes
	#[serde(rename = "SPECIAL")]
	Special,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
	/// The name of an item
	/// Does not include minecraft colour codes.
	#[serde(rename = "item_name")]
	pub name: String,
	/// The "lore" of an item, that is, the description of the items.
	/// Includes minecraft colour codes.
	#[serde(rename = "item_lore")]
	pub lore: String,
	/// The count of items in the stack
	#[serde(rename = "item_count", skip_serializing_if = "Option::is_none")]
	pub count: Option<i8>,
	/// Field to assist database text searches,
	/// it includes enchants and the literal minecraft item's name
	pub extra: String,
	/// The auction category of an item
	pub category: String,
	/// The rarity of the item auctioned
	pub tier: Rarity,
	/// The item's gzipped NBT representation
	#[cfg(feature = "bytes")]
	#[serde(rename = "item_bytes")]
	pub bytes: ItemBytes,
}

#[cfg(feature = "nbt")]
use nbt::from_gzip_reader;
#[cfg(feature = "nbt")]
use std::io;
#[cfg(feature = "nbt")]
use crate::objects::nbt::PartialNbt;

impl Item {
	/// Deflates the bytes into a partial NBT tag
	#[cfg(feature = "nbt")]
	pub fn to_nbt(&self) -> Result<PartialNbt> {
		let bytes: StdResult<Vec<u8>, _> = self.bytes.clone().into();
		let nbt: PartialNbt = from_gzip_reader(io::Cursor::new(bytes?))?;
		Ok(nbt)
	}

	/// Returns the count of items in the stack.
	/// Attempts to count the items in the stack if no cached version is available.
	/// Returns None otherwise
	pub fn count(&mut self) -> Option<i8> {
		if let Some(ref count) = &self.count {
			return Some(*count);
		}

		#[cfg(feature = "nbt")]
		if let Ok(nbt) = self.to_nbt() {
			if let Some(pnbt) = nbt.i.first() {
				self.count = Some(pnbt.count);

				return Some(pnbt.count);
			}
		}

		return None;
	}
}

// Ugly hack because hitting skyblock/auction and skyblock/auctions returns slightly different data.
#[cfg(feature = "bytes")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum ItemBytes {
	T0(ItemBytesT0),
	Data(String),
}

#[cfg(feature = "bytes")]
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

#[cfg(feature = "bytes")]
impl Into<Result<Vec<u8>>> for ItemBytes {
	fn into(self) -> Result<Vec<u8>> {
		let b64: String = self.into();
		Ok(base64::decode(&b64)?)
	}
}

#[cfg(feature = "bytes")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ItemBytesT0 {
	#[serde(rename = "0")]
	Data(String)
}
