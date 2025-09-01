#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entries {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entry {
    pub metadata: Vec<(String, Value)>,
    pub body: String,
}

impl Entry {
    pub fn metadata(&self, key: &str) -> Option<Value> {
        self.metadata
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Text(String),
    List(Vec<String>),
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl From<Vec<String>> for Value {
    fn from(value: Vec<String>) -> Self {
        Self::List(value)
    }
}
