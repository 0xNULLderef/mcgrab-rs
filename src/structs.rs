use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AssetIndex {
	pub id: String,
	pub sha1: String,
	pub size: u32,
	pub totalSize: u32,
	pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct ClientServerDownload {
	pub sha1: String,
	pub size: u32,
	pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionDownloads {
	pub client: ClientServerDownload,
	pub server: ClientServerDownload,
}

#[derive(Deserialize, Debug)]
pub struct JavaVersion {
	pub component: String,
	pub majorVersion: u32,
}

#[derive(Deserialize, Debug)]
pub struct Artifact {
	pub path: PathBuf,
	pub sha1: String,
	pub size: u32,
	pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Native {
	pub path: PathBuf,
	pub sha1: String,
	pub size: u32,
	pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Classifiers {
	#[serde(rename = "natives-linux")]
	pub natives_linux: Option<Native>,
	#[serde(rename = "natives-osx")]
	pub natives_osx: Option<Native>,
	#[serde(rename = "natives-windows")]
	pub natives_windows: Option<Native>,
}

#[derive(Deserialize, Debug)]
/// FUCK YOU MOJANG
pub struct DownloadsInner {
	pub artifact: Option<Artifact>,
	pub classifiers: Option<Classifiers>,
}

#[derive(Deserialize, Debug)]
pub struct NativeList {
	pub linux: Option<String>,
	pub osx: Option<String>,
	pub windows: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Os {
	pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Rules {
	pub action: String,
	pub os: Option<Os>,
}

#[derive(Deserialize, Debug)]
pub struct Downloads {
	pub downloads: DownloadsInner,
	pub name: String,
	pub natives: Option<NativeList>,
	pub rules: Option<Vec<Rules>>,
}

#[derive(Deserialize, Debug)]
pub struct McVersion {
	pub assetIndex: AssetIndex,
	pub assets: String,
	pub complianceLevel: i32,
	pub downloads: VersionDownloads,
	pub id: String,
	pub javaVersion: JavaVersion,
	pub libraries: Vec<Downloads>,
	pub mainClass: String,
	pub minecraftArguments: String,
}
