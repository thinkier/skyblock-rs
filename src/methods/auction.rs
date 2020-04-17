use crate::{SkyblockApi, Result, Auction, GlobalAuctions};
use crate::client::ApiBody;

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
