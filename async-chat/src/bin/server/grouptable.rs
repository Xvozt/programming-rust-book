use crate::group::Group;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct GroupTable(Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl GroupTable {
    pub fn new() -> GroupTable {
        Self(Mutex::new(HashMap::new()))
    }

    pub fn get_or_create(&self, name: Arc<String>) -> Group {
        todo!()
    }

    pub fn get(&self, name: &String) -> Option<Group> {
        todo!()
    }
}
