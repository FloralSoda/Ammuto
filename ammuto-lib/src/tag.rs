#![allow(unused)]

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Represents a key term that can be linked to a file
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tag {
	uuid: uuid::Uuid,
	display_name: String,
	aliases_to: Option<uuid::Uuid>
}
impl Tag {
	pub fn new(name: String) -> Self {
		Self {
			display_name: name,
			aliases_to: None,
            uuid: uuid::Uuid::new_v4()
		}
	}
	pub fn new_with_alias(name: String, alias_to: uuid::Uuid) -> Self {
		Self {
            display_name: name,
            aliases_to: Some(alias_to),
            uuid: uuid::Uuid::new_v4()
        }
	}
	pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }
	pub fn display_name(&self) -> &str {
        &self.display_name
    }
	pub fn alias(&self) -> Option<uuid::Uuid> {
        self.aliases_to
    }
	pub fn set_display_name(&mut self, name: String) {
		self.display_name = name;
	}
	pub fn set_alias(&mut self, alias_to: Option<uuid::Uuid>) {
		self.aliases_to = alias_to;
	}
}
/// Represents a collection of tags, allowing for more coherent display and efficient searching
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Group {
	name: String,
    tags: Vec<uuid::Uuid>
}
impl Group {
	pub fn new(name: String) -> Self { 
		Self {
            name,
            tags: Vec::new()
        }
	}
	pub fn new_with_ids<I>(name: String, tags: I) -> Self
		where I: IntoIterator<Item = uuid::Uuid>, {
		Self {
            name,
            tags: tags.into_iter().collect()
        }
	}
	pub fn add_tag(&mut self, tag: uuid::Uuid) {
        self.tags.push(tag);
    }
	pub fn transfer_tag(&mut self, tag: uuid::Uuid, new_group: &mut Group) -> Result<(), TagError> {
		let to_transfer = match self.remove_tag(tag) {
			Ok(tag) => tag,
			Err(err) => return Err(err),
		};

		new_group.add_tag(to_transfer);
        Ok(())
	}
	pub fn remove_tag(&mut self, tag: uuid::Uuid) -> Result<uuid::Uuid,TagError>  {
		let index = self.tags.iter().position(|t| t.eq(&tag));
		match index {
			Some(idx) => {
				Ok(self.tags.remove(idx))
			},
			None => Err(TagError::TagNotFound)
		}
	}
}
pub enum TagError {
	TagNotFound
}
