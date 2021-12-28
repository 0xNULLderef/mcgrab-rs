#![allow(non_snake_case)]

mod structs;

use reqwest::{get};
use serde_json::{Value, from_str};
use std::error::Error;
use futures::executor::block_on;

async fn getJSONfromURL(url: &str) -> Result<structs::McVersion, Box<dyn Error>> {
	Ok(from_str(get(url).await?.text().await?.as_str())?)
}

#[tokio::main]
async fn main() {
	let version = getJSONfromURL("https://launchermeta.mojang.com/v1/packages/cfd75871c03119093d7c96a6a99f21137d00c855/1.12.2.json").await.unwrap();
	println!("{:#?}", version.libraries)
}
