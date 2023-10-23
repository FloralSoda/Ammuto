#![allow(unused)]

use std::{fs::File, time::{SystemTime, UNIX_EPOCH}};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

///The origins of the contents. Could be a website, local file, physical place or unknown
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Origin {
	Unknown,
	Simple(String),
	Web(String),
    File(String),
	Physical // TODO: Represent physical locations somehow.
}
/// A file stored within the database.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FileEntry {
	filetype: String,
	id: uuid::Uuid,
    date: Option<u64>,
	location: Origin,
	tags: Vec<uuid::Uuid>
}
impl FileEntry {
	fn get_current_time() -> Option<u64> {
		match SystemTime::now().duration_since(UNIX_EPOCH) {
			Ok(duration) => Some(duration.as_secs()),
			Err(_) => None,
		}
	}
	pub fn new(filetype: String, from: Origin, file: File) -> Self {
		let meta = file.metadata();
		if let Ok(data) = meta {
			let date = match data.created() {
				Ok(time) => match time.duration_since(UNIX_EPOCH) {
					Ok(duration) => Some(duration.as_secs()),
                    Err(_) => FileEntry::get_current_time(),
				},
				Err(_) => FileEntry::get_current_time()
			};

			FileEntry {
				filetype,
                id: uuid::Uuid::new_v4(),
                date,
                location: from,
                tags: Vec::new()
			}
		} else {
			FileEntry {
				filetype,
                id: uuid::Uuid::new_v4(),
                date: FileEntry::get_current_time(),
                location: from,
                tags: Vec::new()
			}
		}
	}
}
