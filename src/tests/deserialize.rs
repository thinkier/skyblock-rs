use crate::BDRes;
use crate::http::{ApiBody, GlobalAuctions, SearchedAuctions};
use crate::objects::auction::*;
use crate::objects::profile::*;

#[test]
fn auctions_body() {
	let body: ApiBody<GlobalAuctions> = serde_json::from_str(include_str!("deserialize/auctions_body.json")).unwrap();
	let res: BDRes<_> = body.into();
	let glob_auctions = res.ok().unwrap();

	assert_eq!(glob_auctions, glob_auctions);
}

#[test]
fn auction_body() {
	let body: ApiBody<SearchedAuctions> = serde_json::from_str(include_str!("deserialize/auction_body.json")).unwrap();
	let res: BDRes<_> = body.into();
	let auction = res.ok().unwrap();

	assert_eq!(auction, auction);
}

#[test]
fn searched_auctions() {
	let _: SearchedAuctions = serde_json::from_str(include_str!("deserialize/searched_auctions.json")).unwrap();
}

#[test]
fn auction() {
	let auction: Auction = serde_json::from_str(include_str!("deserialize/auction.json")).unwrap();

	assert_eq!(auction.uuid, "a1767fbbac3c48189c9e05a24b27f9bc");
	assert_eq!(auction.auctioneer, PartialProfile("f06e38d0cd634a4ea23306cef8b0bcee".to_string()));
	assert_eq!(auction.start, 1579054579940);
	assert_eq!(auction.end, 1579076179940);
}

#[test]
fn bids() {
	let bids: Bids = serde_json::from_str(include_str!("deserialize/bids.json")).unwrap();

	assert_eq!(bids.highest, 575);
	assert_eq!(bids.starting, 500);
}

#[test]
fn item() {
	let item: Item = serde_json::from_str(include_str!("deserialize/item.json")).unwrap();

	assert_eq!(item.name, "◆ Ice Rune I");
	assert_eq!(item.category, "misc");
	assert_eq!(item.extra, "◆ Ice Rune I Skull Item");
	assert_eq!(item.lore, "§8Requires level 8\n§8Bows\n\n§7Your arrows are icy cold!\n\n§7Apply this rune to bows or fuse\n§7two together at the Runic\n§7Pedestal!\n\n§9§lRARE");
	assert_eq!(item.tier, Rarity::Uncommon);
}

#[cfg(feature = "nbt")]
#[test]
fn nbt() {
	let item: Item = serde_json::from_str(include_str!("deserialize/item.json")).unwrap();

	let nbt = item.to_nbt().unwrap();
	assert_eq!(nbt.i[0].count, 10);
	assert_eq!(nbt.i[0].tag.display.lore.len(), 10);
}

#[cfg(feature = "nbt")]
#[test]
fn enchants() {
	let enchants = {
		let aotd: Item = serde_json::from_str(include_str!("deserialize/maxed_aotd.json")).unwrap();

		let nbt = aotd.to_nbt().unwrap();
		nbt.i[0].tag.extra_attributes.enchantments.clone().unwrap()
	};
	assert_eq!(enchants.len(), 17);
}

#[test]
fn claim() {
	let claim: Claim = serde_json::from_str(include_str!("deserialize/claim.json")).unwrap();

	assert!(!claim.claimed);
}

#[test]
fn bid() {
	let bid: Bid = serde_json::from_str(include_str!("deserialize/bid.json")).unwrap();

	assert_eq!(bid.auction_id, PartialAuction("a1767fbbac3c48189c9e05a24b27f9bc".to_string()));
	assert_eq!(bid.bidder, PartialProfile("be366506d9cd415687a9a1c90a888550".to_string()));
	assert_eq!(bid.amount, 500);
	assert_eq!(bid.timestamp, 1579060663571);
}

#[test]
fn error_body() {
	let body: ApiBody<Auction> = serde_json::from_str(include_str!("deserialize/api_error.json")).unwrap();
	let res: BDRes<_> = body.into();
	let err = res.err().unwrap();

	assert_eq!(err.to_string(), "api call failed: No \"key\" provided!");
}
