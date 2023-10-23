use std::path::PathBuf;

use crate::{file::FileEntry, tag::{Tag, Group}};

///An abstraction of a database. \
///Allows applications to implement their own database while maintaining
///compatibility with any implementation of the library
pub trait Database {
	fn add_file(&mut self, path: PathBuf);
	fn remove_file(&mut self, entry: FileEntry);
	fn save_file(&mut self, entry: FileEntry);

	fn add_tag(&mut self, tag: Tag);
	fn remove_tag(&mut self, tag: uuid::Uuid);
	fn save_tag(&mut self, tag: Tag);
	fn get_tag(&self, tag: uuid::Uuid) -> Tag;

	fn add_group(&mut self, group: Group);
	fn remove_group(&mut self, group: String);
	fn save_group(&mut self, group: Group);
	fn get_group(&self, tag: uuid::Uuid) -> Group;

	fn query<Q>(query: Q) -> Vec<DatabaseEntry>
		where Q: DatabaseQuery;
}

///A query for an abstract database.
///These can be reused.
pub trait DatabaseQuery {
	fn mutates(&self) -> bool;
}
///A query for an abstract database that edits the data within the database
pub struct MutatingDatabaseQuery {

}
///A query for an abstract database that does not edit the data within the database
pub struct ReadOnlyDatabaseQuery {

}
///Builds a query for an abstract database that edits the data within the database
pub struct MutatingDatabaseQueryBuilder;
///Builds a query for an abstract database that does not edit the data within the database
pub struct ReadOnlyDatabaseQueryBuilder;

impl ReadOnlyDatabaseQueryBuilder {
	pub fn new() -> Self {
        Self {}
    }

}
impl Default for ReadOnlyDatabaseQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

///A portion of a query. Contains data to parse into the desired query language
pub enum QueryComponent {
	From(DatabaseTable),
	Where(DatabaseCondition),
    OrderBy(DatabaseOrder),
    Limit(u64)
}
pub enum DatabaseOrder {
	Ascending,
    Descending
}
///Represents a common table within an abstract database. 
pub enum DatabaseTable {
	Files,
	Tags,
	Groups
}
///Represents an entry within the database
pub enum DatabaseEntry {
	Error(),
	File(FileEntry),
	Tag(Tag),
	Group(Group)
}
pub struct DatabaseCondition {
	//Todo: Implement
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DatabaseProperty<T> {
	pub property: String,
    pub value: T
}
impl<T> DatabaseProperty<T> {
	pub fn new(name: String, value: T) -> Self {
        Self { property: name, value }
    }
	pub fn value(&self) -> &T {
        &self.value
    }
	pub fn value_mut(&mut self) -> &mut T {
		&mut self.value
	}
	pub fn set_value(&mut self, value: T) {
		self.value = value;
	}
}
