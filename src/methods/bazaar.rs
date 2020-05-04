use crate::{Result, SkyblockApi, Product};
use crate::client::ApiBody;
use std::collections::HashMap;

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
struct UnifiedListing {
	pub products: HashMap<String, Product>
}

impl<'a> SkyblockApi<'a> {
	/// Deprecated function to fetch a list of available Product IDs from the API.
	#[deprecated]
	pub async fn get_bazaar_products(&mut self) -> Result<Vec<String>> {
		let products: ApiBody<Products> = self.get("bazaar/products", vec![]).await?;

		match products.into() {
			Ok(prods) => Ok(prods.product_ids),
			Err(cause) => Err(cause)
		}
	}

	/// Fetch all Bazaar products and their current state.
	/// This endpoint returns a `None` in the `week_historic` field.
	pub async fn get_bazaar_product_listing(&mut self) -> Result<HashMap<String, Product>> {
		let products: ApiBody<UnifiedListing> = self.get("bazaar", vec![]).await?;

		match products.into() {
			Ok(prods) => Ok(prods.products),
			Err(cause) => Err(cause)
		}
	}

	/// Fetch a particular Bazaar product and return their current state.
	#[deprecated]
	pub async fn get_bazaar_product(&mut self, product: &str) -> Result<Product> {
		let product: ApiBody<ProductWrapper> = self.get("bazaar/product", vec![("productId", product.to_owned())]).await?;

		let res: Result<_> = product.into();

		res.map(|wrapped| wrapped.product_info)
	}
}
