#![allow(non_snake_case)]

use std::error::Error;
use indicatif::ProgressIterator;

use downloader::DownloadURLs;

mod downloader;
mod structs;

async fn getJSONfromURL(url: &str) -> Result<structs::McVersion, Box<dyn Error>> {
	Ok(serde_json::from_str(
		reqwest::get(url).await?.text().await?.as_str(),
	)?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let version = getJSONfromURL("http://localhost/user/1.12.2.formatted.json").await.unwrap();
	version
		.libraries
		.into_iter()
		// .progress()
		.map(|d| -> Result<(), Box<dyn Error>> {
			if let Some(rules) = d.rules {

			} else {
				DownloadURLs {
					lib: d.downloads.artifact.map(|a| a.url),
					native: d
						.downloads
						.classifiers
						.map(|c| c.natives_linux.map(|n| n.url))
						.flatten(),
				}
				.download()?
			}
			Ok(())
		})
		.collect::<Result<(), Box<dyn Error>>>()?;
	Ok(())
}
