#![allow(unused)]

use std::{fs::File, time::{SystemTime, UNIX_EPOCH}};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

///The origins of the contents. Could be a website, local file, physical place or unknown
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum Origin {
	Unknown,
	Simple(String),
	Web(String),
    File(String),
	Physical // TODO: Represent physical locations somehow.
}
/// A file stored within the database.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Entry {
	filetype: String,
	id: uuid::Uuid,
    date: Option<u64>,
	location: Origin,
	tags: Vec<uuid::Uuid>
}
impl Entry {
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
                    Err(_) => Entry::get_current_time(),
				},
				Err(_) => Entry::get_current_time()
			};

			Entry {
				filetype,
                id: uuid::Uuid::new_v4(),
                date,
                location: from,
                tags: Vec::new()
			}
		} else {
			Entry {
				filetype,
                id: uuid::Uuid::new_v4(),
                date: Entry::get_current_time(),
                location: from,
                tags: Vec::new()
			}
		}
	}
}
