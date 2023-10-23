#![allow(unused)]

use std::{fs::File, time::{SystemTime, UNIX_EPOCH}};

enum Origin {
	Unknown,
	Simple(String)
}

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
