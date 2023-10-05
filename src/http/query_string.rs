use std::collections::HashMap;

// Representing the query string with a hash map
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    // Use heap allocated array that will grow dynamically
    Multiple(Vec<&'buf str>),
}

// Read keys from hash map
impl<'buf> QueryString<'buf> {
    // IThis method takes a reference from the QueryString
    pub fn get(&self, key: &str) -> Option<&Value> {
        // Call get on the hash map itself
        self.data.get(key)
    }
}
