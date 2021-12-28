use futures::executor::block_on;
use std::error::Error;
use std::ffi::OsStr;
use std::{fs, path::Path};
use zip_extensions::ZipArchiveExtensions;

pub struct DownloadURLs {
	pub lib: Option<String>,
	pub native: Option<String>,
}

async fn getFileAndPutInFolder<P: AsRef<Path>>(
	url: &String,
	path: P,
) -> Result<(), Box<dyn std::error::Error>> {
	println!("getting {}", url);
	let x = reqwest::get(url).await?;
	println!("doing the bytes");
	fs::write(
		path.as_ref()
			.join(Path::new(url.split("/").last().unwrap())),
		x.bytes().await?,
	)?;
	println!("done!");
	Ok(())
}

impl DownloadURLs {
	pub fn download(&self) -> Result<(), Box<dyn Error>> {
		if let Some(url) = &self.lib {
			block_on(getFileAndPutInFolder(url, "./jars/"))?;
		}

		if let Some(url) = &self.native {
			let dir = tempdir::TempDir::new("natives-extract")?;
			block_on(getFileAndPutInFolder(url, &dir))?;
			let mut zip = zip::ZipArchive::new(fs::File::open(
				&dir.path().join(url.split("/").last().unwrap()),
			)?)?;
			(0..zip.len())
				.into_iter()
				.map(|n| -> Result<(), Box<dyn Error>> {
					let path = zip.entry_path(n)?;

					if path.extension() == Some(&OsStr::new("so")) {
						zip.extract_file(
							n,
							&Path::new("./natives/").join(path.file_name().unwrap()),
							true,
						)?;
					}

					Ok(())
				})
				.collect::<Result<(), Box<dyn Error>>>()?;
		}

		Ok(())
	}
}
