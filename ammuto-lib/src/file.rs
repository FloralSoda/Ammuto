#![allow(unused)]

use std::{fs::File, time::{SystemTime, UNIX_EPOCH}};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::connection::DatabaseProperty;

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
	filetype: DatabaseProperty<String>,
	id: DatabaseProperty<uuid::Uuid>,
    date: DatabaseProperty<Option<u64>>,
	location: DatabaseProperty<Origin>,
	tags: DatabaseProperty<Vec<uuid::Uuid>>
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
				filetype: DatabaseProperty::new(String::from("filetype"), filetype),
                id: DatabaseProperty::new(String::from("id"), uuid::Uuid::new_v4()),
                date: DatabaseProperty::new(String::from("date"), date),
                location: DatabaseProperty::new(String::from("location"), from),
				..Default::default()
			}
		} else {
			FileEntry {
				filetype: DatabaseProperty::new(String::from("filetype"),filetype),
                id: DatabaseProperty::new(String::from("id"),uuid::Uuid::new_v4()),
                location: DatabaseProperty::new(String::from("location"),from),
				..Default::default()
			}
		}
	}
}
impl Default for FileEntry {
    fn default() -> Self {
        Self { 
			filetype: DatabaseProperty::new(String::from("filetype"), String::from("GENERIC")), 
			id: DatabaseProperty::new(String::from("id"), Default::default()),  
			date: DatabaseProperty::new(String::from("date"), FileEntry::get_current_time()), 
			location: DatabaseProperty::new(String::from("location"), Origin::Unknown),  
			tags: DatabaseProperty::new(String::from("tags"), Default::default()) 
		}
    }
}
