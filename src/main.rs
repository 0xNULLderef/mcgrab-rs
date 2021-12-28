#![allow(non_snake_case)]

use std::error::Error;

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
	// let version = getJSONfromURL("https://launchermeta.mojang.com/v1/packages/cfd75871c03119093d7c96a6a99f21137d00c855/1.12.2.json").await.unwrap();
	// // println!("{:#?}", version.libraries);
	// version.libraries.into_iter().filter_map(|d| {
	// 	match d.natives {
	// 		Some(list) => {

	// 		}
	// 		None => {
	// 			d.downloads.artifact.expect("must have artifact or classifiers!").url
	// 		}
	// 	}
	// 	todo!()
	// 	// if d.natives.is_none() {
	// 	// 	Some(d.downloads.artifact.match())
	// 	// }
	// });
	DownloadURLs {
		lib: Some(
			"https://libraries.minecraft.net/com/mojang/patchy/1.3.9/patchy-1.3.9.jar".to_string(),
		),
		native: Some(
			"https://libraries.minecraft.net/org/lwjgl/lwjgl/lwjgl-platform/2.9.2-nightly-20140822/lwjgl-platform-2.9.2-nightly-20140822-natives-linux.jar".to_string()
		),
	}
	.download()?;
	Ok(())
}
