#![allow(unused)]

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::connection::DatabaseProperty;

/// Represents a key term that can be linked to a file
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tag {
	uuid: DatabaseProperty<uuid::Uuid>,
	display_name: DatabaseProperty<String>,
	aliases_to: DatabaseProperty<Option<uuid::Uuid>>
}
impl Tag {
	pub fn new(name: String) -> Self {
		Self {
			display_name: DatabaseProperty::new(String::from("display_name"), name),
			aliases_to: DatabaseProperty::new(String::from("aliases_to"), None),
            uuid: DatabaseProperty::new(String::from("uuid"), uuid::Uuid::new_v4())
		}
	}
	pub fn new_with_alias(name: String, alias_to: uuid::Uuid) -> Self {
		Self {
			display_name: DatabaseProperty::new(String::from("display_name"), name),
			aliases_to: DatabaseProperty::new(String::from("aliases_to"), Some(alias_to)),
            uuid: DatabaseProperty::new(String::from("uuid"), uuid::Uuid::new_v4())
		}
	}
	pub fn uuid(&self) -> uuid::Uuid {
        self.uuid.value
    }
	pub fn display_name(&self) -> &str {
        &self.display_name.value
    }
	pub fn alias(&self) -> Option<uuid::Uuid> {
        self.aliases_to.value
    }
	pub fn set_display_name(&mut self, name: String) {
		self.display_name.set_value(name);
	}
	pub fn set_alias(&mut self, alias_to: Option<uuid::Uuid>) {
		self.aliases_to.set_value(alias_to);
	}
}
/// Represents a collection of tags, allowing for more coherent display and efficient searching
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Group {
	name: DatabaseProperty<String>,
    tags: DatabaseProperty<Vec<uuid::Uuid>>
}
impl Group {
	pub fn new(name: String) -> Self { 
		Self {
            name: DatabaseProperty::new(String::from("name"), name),
            tags: DatabaseProperty::new(String::from("tags"), Vec::new())
        }
	}
	pub fn new_with_ids<I>(name: String, tags: I) -> Self
		where I: IntoIterator<Item = uuid::Uuid>, {
		Self {
			name: DatabaseProperty::new(String::from("name"), name),
            tags: DatabaseProperty::new(String::from("tags"), tags.into_iter().collect())
        }
	}
	pub fn add_tag(&mut self, tag: uuid::Uuid) {
        self.tags.value_mut().push(tag);
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
		let index = self.tags.value().iter().position(|t| t.eq(&tag));
		match index {
			Some(idx) => {
				Ok(self.tags.value_mut().remove(idx))
			},
			None => Err(TagError::TagNotFound)
		}
	}
}
pub enum TagError {
	TagNotFound
}
