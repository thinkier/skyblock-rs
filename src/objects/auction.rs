use crate::objects::profile::PartialProfile;
use crate::objects::items::Item;
use crate::{SkyblockApi, Result};
use crate::http::ApiBody;

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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GlobalAuctions {
	pub page: usize,
	#[serde(rename = "totalPages")]
	pub total_pages: usize,
	#[serde(rename = "totalAuctions")]
	pub total_auctions: usize,
	#[serde(rename = "lastUpdated")]
	pub last_update: u64,
	pub auctions: Vec<Auction>,
}

#[cfg(test)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SearchedAuctions {
	pub auctions: Vec<Auction>,
}

impl<'a> SkyblockApi<'a> {
	pub async fn iter_active_auctions<F>(&mut self, mut f: F) -> Result<()> where
		F: FnMut(Auction) -> Result<()> {
		let mut i = 0;
		let mut total_pages = 1usize;

		while i < total_pages {
			let page = self.get_auctions_page(i).await?;
			total_pages = page.total_pages;

			for auction in page.auctions {
				f(auction)?;
			}

			i += 1;
		}

		Ok(())
	}

	pub async fn get_auctions_page(&mut self, page: usize) -> Result<GlobalAuctions> {
		let body: ApiBody<GlobalAuctions> = self.get("auctions", vec![("page", format!("{}", page))]).await?;
		body.into()
	}
}