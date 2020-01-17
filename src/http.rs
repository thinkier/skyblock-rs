use futures::{TryFutureExt, StreamExt};
use hyper::{Client, Uri, Body};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use std::error::Error;
use std::time::{SystemTime, Duration};
use std::{thread, fmt};
use crate::objects::auction::Auction;
use crate::BDRes;

const BASE_URL: &'static str = "https://api.hypixel.net/skyblock/";

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ApiError(String);

impl fmt::Display for ApiError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "api call failed: {}", self.0)
	}
}

impl Error for ApiError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Key<'a> {
	key: &'a str,
	#[serde(default = "SystemTime::now")]
	window: SystemTime,
	#[serde(default)]
	uses: usize,
	#[serde(default = "hypixel_api_window_size")]
	window_size: u64,
	#[serde(default = "hypixel_api_window_limit")]
	window_limit: usize,
}

fn hypixel_api_window_size() -> u64 { 60 }

fn hypixel_api_window_limit() -> usize { 120 }

impl<'a> Key<'a> {
	pub fn new(key: &'a str, window_limit: usize, window_size: u64) -> Key {
		Key {
			key,
			window: SystemTime::now(),
			uses: 0,
			window_size,
			window_limit,
		}
	}

	pub fn timeout(&self) -> bool {
		self.window.elapsed().unwrap_or(Duration::from_secs(0)).as_secs() >= self.window_size
	}

	pub fn can_use(&mut self) -> bool {
		self.uses < self.window_limit || self.timeout()
	}

	pub fn consume<'b>(&'b mut self) -> Option<&'a str> {
		if self.timeout() {
			self.window = SystemTime::now();
			self.uses = 0;
		}

		if self.can_use() {
			self.uses += 1;
			Some(self.key)
		} else {
			None
		}
	}
}

pub struct SkyblockApi<'a> {
	keys: Vec<Key<'a>>,
}

impl<'a> SkyblockApi<'a> {
	pub fn pooled(keys: Vec<&str>) -> SkyblockApi {
		SkyblockApi {
			keys: keys.into_iter().map(|k| Key::new(k, 120, 60)).collect(),
		}
	}

	pub fn singleton(key: &str) -> SkyblockApi {
		Self::pooled(vec![key])
	}

	fn get_key_sync(&mut self) -> &str {
		loop {
			for key in &mut self.keys {
				if let Some(key) = key.consume() {
					return key;
				}
			}

			thread::sleep(Duration::from_millis(50));
		}
	}

	pub async fn get<T>(&mut self, path: &str, params: Vec<(&str, String)>) -> BDRes<T> where
		T: for<'de> Deserialize<'de> {
		let uri: Uri = format!("{}{}?key={}{}", BASE_URL, path, self.get_key_sync(), params.iter()
			.map(|(k, v)| {
				format!("&{}={}", k, v)
			})
			.collect::<Vec<_>>()
			.join("")
		).parse().unwrap();

		let https = HttpsConnector::new();
		let cli = Client::builder().build::<_, Body>(https);

		cli.get(uri)
			.map_ok(|x| x.into_body())
			.map_ok(|x| x
				.fold(Ok(vec![]), |buf: BDRes<_>, chunk| async {
					let mut buf = buf?;
					buf.extend(&chunk?[..]);
					Ok(buf)
				})
				.map_ok(|slice|
					Ok(serde_json::from_slice(&slice[..])?)
				)
			).await?.await?
	}

	pub async fn iter_active_auctions<F>(&mut self, mut f: F) -> BDRes<()> where
		F: FnMut(Auction) -> BDRes<()> {
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

	pub async fn get_auctions_page(&mut self, page: usize) -> BDRes<GlobalAuctions> {
		let body: ApiBody<GlobalAuctions> = self.get("auctions", vec![("page", format!("{}", page))]).await?;
		body.into()
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum ApiBody<T> {
	Error {
		cause: ApiError
	},
	Ok(T),
}

impl<T> Into<BDRes<T>> for ApiBody<T> {
	fn into(self) -> BDRes<T> {
		match self {
			Self::Ok(i) => {
				Ok(i)
			}
			Self::Error { cause } => {
				Err(Box::new(cause))
			}
		}
	}
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SearchedAuctions {
	pub auctions: Vec<Auction>,
}