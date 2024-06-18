#[derive(Debug, PartialEq)]
pub struct Item {
    todo: String,
    done: bool,
}

impl Item {
    pub fn new(todo: String, done: bool) -> Item {
        Item { todo, done }
    }
}
