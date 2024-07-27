use std::fs;

#[derive(Debug, PartialEq)]
pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn from_file(file_path: &str) -> TodoList {
        let file = fs::read_to_string(file_path).unwrap_or_else(|_| String::new());

        let items = TodoList {
            items: file
                .lines()
                .skip(1) // Skip the title
                .filter_map(|line| {
                    if line.is_empty() {
                        None
                    } else {
                        Some(TodoItem::from_string(line))
                    }
                })
                .collect(),
        };
        items
    }

    pub fn save(&self, file_path: &str) {
        let mut content = String::new();
        content.push_str("# TODO\n\n");
        for item in &self.items {
            content.push_str(&item.to_string());
            content.push_str("\n");
        }
        fs::write(file_path, content).expect("Failed to write file");
    }

    pub fn list_items(&self) {
        for (i, item) in self.items.iter().enumerate() {
            println!("{}: {}", i + 1, item.to_string());
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(TodoItem::new(item));
    }

    pub fn mark_done(&mut self, number: usize) {
        if number == 0 || number > self.items.len() {
            println!("Invalid item number");
            return;
        }
        self.items[number - 1].done();
    }

    pub fn remove(&mut self, number: usize) {
        if number == 0 || number > self.items.len() {
            println!("Invalid item number");
            return;
        }
        self.items.remove(number - 1);
    }

    pub fn remove_completed(&mut self) {
        self.items.retain(|item| !item.done);
    }

    pub fn remove_all(&mut self) {
        self.items.clear();
    }
}

#[derive(Debug, PartialEq)]
pub struct TodoItem {
    done: bool,
    todo: String,
}

impl TodoItem {
    pub fn new(todo: String) -> TodoItem {
        TodoItem { todo, done: false }
    }
    pub fn from_string(todo: &str) -> TodoItem {
        let mut chars = todo.chars();
        chars.next(); // -
        chars.next(); // space
        chars.next(); // [
        let done = chars.next() == Some('X');
        chars.next(); // ]
        chars.next(); // space
        let todo: String = chars.collect(); // the todo

        TodoItem { todo, done }
    }

    pub fn to_string(&self) -> String {
        let done = if self.done { "X" } else { " " };
        format!("- [{}] {}", done, self.todo)
    }

    pub fn done(&mut self) {
        self.done = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_list_item_unchecked() {
        let line = "- [ ] Item 1";
        let expected = TodoItem {
            todo: "Item 1".to_string(),
            done: false,
        };

        assert_eq!(TodoItem::from_string(line), expected);
    }

    #[test]
    fn parse_line_list_item_checked() {
        let line = "- [X] Item 1";
        let expected = TodoItem {
            todo: "Item 1".to_string(),
            done: true,
        };
        assert_eq!(TodoItem::from_string(line), expected);
    }

    #[test]
    fn parse_file_empty() {
        let file_path = "tests/empty.md";
        let expected = TodoList { items: vec![] };
        assert_eq!(TodoList::from_file(file_path), expected);
    }

    #[test]
    fn parse_file_one_item() {
        let file_path = "tests/one_item.md";
        let expected = TodoList {
            items: vec![TodoItem {
                todo: "Item 1".to_string(),
                done: false,
            }],
        };
        assert_eq!(TodoList::from_file(file_path), expected);
    }

    #[test]
    fn parse_file_two_items() {
        let file_path = "tests/two_items.md";
        let expected = TodoList {
            items: vec![
                TodoItem {
                    todo: "Item 1".to_string(),
                    done: false,
                },
                TodoItem {
                    todo: "Item 2".to_string(),
                    done: true,
                },
            ],
        };
        assert_eq!(TodoList::from_file(file_path), expected);
    }

    #[test]
    fn parse_file_empty_lines() {
        let file_path = "tests/empty_lines.md";
        let expected = TodoList {
            items: vec![
                TodoItem {
                    todo: "Item 1".to_string(),
                    done: false,
                },
                TodoItem {
                    todo: "Item 2".to_string(),
                    done: true,
                },
            ],
        };
        assert_eq!(TodoList::from_file(file_path), expected);
    }

    #[test]
    fn write_line_list_item_unchecked() {
        let item = TodoItem {
            todo: "Item 1".to_string(),
            done: false,
        };

        let expected = "- [ ] Item 1";
        assert_eq!(item.to_string(), expected);
    }

    #[test]
    fn write_line_list_item_checked() {
        let item = TodoItem {
            todo: "Item 1".to_string(),
            done: true,
        };
        let expected = "- [X] Item 1";
        assert_eq!(item.to_string(), expected);
    }

    #[test]
    fn write_file_empty() {
        let items = TodoList { items: vec![] };
        items.save("tests/tmp/empty.md");

        let expected = fs::read_to_string("tests/empty.md").unwrap();
        let actual = fs::read_to_string("tests/tmp/empty.md").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_file_one_item() {
        let items = TodoList {
            items: vec![TodoItem {
                todo: "Item 1".to_string(),
                done: false,
            }],
        };
        items.save("tests/tmp/one_item.md");

        let expected = fs::read_to_string("tests/one_item.md").unwrap();
        let actual = fs::read_to_string("tests/tmp/one_item.md").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_file_two_items() {
        let items = TodoList {
            items: vec![
                TodoItem {
                    todo: "Item 1".to_string(),
                    done: false,
                },
                TodoItem {
                    todo: "Item 2".to_string(),
                    done: true,
                },
            ],
        };
        items.save("tests/tmp/two_items.md");
        let expected = fs::read_to_string("tests/two_items.md").unwrap();
        let actual = fs::read_to_string("tests/tmp/two_items.md").unwrap();
        assert_eq!(actual, expected);
    }
}
