use std::path::PathBuf;

use crate::{file::FileEntry, tag::{Tag, Group}};

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

pub trait DatabaseQuery {
	fn mutates(&self) -> bool;
}

pub struct MutatingDatabaseQuery {

}
pub struct ReadOnlyDatabaseQuery {

}
pub struct MutatingDatabaseQueryBuilder;
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

pub enum DatabaseEntry {
	Error(),
	File(FileEntry),
	Tag(Tag),
	Group(Group)
}
