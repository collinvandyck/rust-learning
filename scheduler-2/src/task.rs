pub(crate) struct Task {
    typ: TaskType,
}

/// TaskType identifies the kind of task.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TaskType(String);

impl TaskType {
    pub fn new<T: Into<String>>(id: T) -> Self {
        Self(id.into())
    }
}

impl From<&str> for TaskType {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl From<String> for TaskType {
    fn from(id: String) -> Self {
        Self(id)
    }
}
