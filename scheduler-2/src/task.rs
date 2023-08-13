/// `TaskType` identifies the kind of task.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Type(String);

impl Type {
    pub fn new<T: Into<String>>(id: T) -> Self {
        Self(id.into())
    }
    pub fn from<T: Into<Type>>(t: T) -> Self {
        t.into()
    }
}

impl From<&str> for Type {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl From<String> for Type {
    fn from(id: String) -> Self {
        Self(id)
    }
}
