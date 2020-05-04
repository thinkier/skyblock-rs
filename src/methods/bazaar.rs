use crate::{Result, SkyblockApi, Product};
use crate::client::ApiBody;
use std::collections::BTreeMap;

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
	pub products: BTreeMap<String, Product>
}

impl<'a> SkyblockApi<'a> {
	#[deprecated]
	pub async fn get_bazaar_products(&mut self) -> Result<Vec<String>> {
		let products: ApiBody<Products> = self.get("bazaar/products", vec![]).await?;

		match products.into() {
			Ok(prods) => Ok(prods.product_ids),
			Err(cause) => Err(cause)
		}
	}

	pub async fn get_bazaar_product_listing(&mut self) -> Result<BTreeMap<String, Product>> {
		let products: ApiBody<UnifiedListing> = self.get("bazaar", vec![]).await?;

		match products.into() {
			Ok(prods) => Ok(prods.products),
			Err(cause) => Err(cause)
		}
	}

	#[deprecated]
	pub async fn get_bazaar_product(&mut self, product: &str) -> Result<Product> {
		let product: ApiBody<ProductWrapper> = self.get("bazaar/product", vec![("productId", product.to_owned())]).await?;

		let res: Result<_> = product.into();

		res.map(|wrapped| wrapped.product_info)
	}
}
