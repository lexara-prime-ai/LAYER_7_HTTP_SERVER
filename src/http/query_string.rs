use std::collections::HashMap;

// Representing the query string with a hash map
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    // Use heap allocated array that will grow dynamically
    Multiple(Vec<&'buf str>),
}

// Read keys from hash map | e.g query string with wild cards
//      a=1&b=2&c&d=&e===&d=7=&d=abc
impl<'buf> QueryString<'buf> {
    // IThis method takes a reference from the QueryString
    pub fn get(&self, key: &str) -> Option<&Value> {
        // Call get on the hash map itself
        self.data.get(key)
    }
}

// Implement 'From' trait
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        // Instantiate a vector | vec with new values
                        // using the vec! macro
                        // Swap values in hash map from existing value
                        // i.e Value::Single -> Value::Multiple

                        // * Dereference the 'existing' value -> In short, follow the pointer and assign the new value to the address

                        // This will work & is safe since multiple variants of an enum take up the same space
                        *existing = Value::Multiple(vec![prev_val, val])
                    }
                    Value::Multiple(vec) => vec.push(val)
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}