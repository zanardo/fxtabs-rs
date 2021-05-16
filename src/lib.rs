use std::io::Read;
use std::{fs::File, usize};

use anyhow::{bail, Result};
use lz4::block::decompress;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FirefoxTab {
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
struct Entries {
    entries: Vec<FirefoxTab>,
    index: u32,
}

#[derive(Serialize, Deserialize)]
struct Tabs {
    tabs: Vec<Entries>,
}

#[derive(Serialize, Deserialize)]
struct Data {
    windows: Vec<Tabs>,
}

pub fn open_tabs(file_path: &str) -> Result<Vec<FirefoxTab>> {
    let mut firefox_tabs: Vec<FirefoxTab> = Vec::new();

    let mut fp = File::open(file_path)?;

    // Checking magic header.
    let mut magic = [0; 8];
    fp.read_exact(&mut magic)?;
    if magic != "mozLz40\0".as_bytes() {
        bail!("invalid header magic");
    }

    // Uncompressing file.
    let mut buf = Vec::new();
    fp.read_to_end(&mut buf)?;
    let json = decompress(&buf, None)?;

    // Parsing json.
    let v: Data = serde_json::from_slice(&json)?;
    for w in v.windows {
        for t in w.tabs {
            let index = t.index as usize - 1;
            if let Some(e) = &t.entries.get(index) {
                firefox_tabs.push(FirefoxTab {
                    title: e.title.clone(),
                    url: e.url.clone(),
                });
            }
        }
    }

    Ok(firefox_tabs)
}
