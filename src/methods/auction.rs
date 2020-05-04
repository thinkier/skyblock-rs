use crate::{SkyblockApi, Result, Auction, GlobalAuctions};
use crate::client::ApiBody;

impl<'a> SkyblockApi<'a> {
	/// Helper function, allows the user to pass a function tio the API
	/// to iterate over the listings on auction right now, instead of allocating
	/// for the entire auction house and returning that to the caller.
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

	/// Returns a particular page of auctions to the caller.
	pub async fn get_auctions_page(&mut self, page: usize) -> Result<GlobalAuctions> {
		let body: ApiBody<GlobalAuctions> = self.get("auctions", vec![("page", format!("{}", page))]).await?;
		body.into()
	}
}
