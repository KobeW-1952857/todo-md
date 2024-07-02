use std::fs;

#[derive(Debug, PartialEq)]
pub struct Items {
    items: Vec<Item>,
}

impl Items {
    pub fn from_file(file_path: &str) -> Items {
        let file = fs::read_to_string(file_path).expect("Failed to read file");
        let items = Items {
            items: file
                .lines()
                .skip(1) // Skip the title
                .filter_map(|line| {
                    if line.is_empty() {
                        None
                    } else {
                        Some(Item::from_string(line))
                    }
                })
                .collect(),
        };
        items
    }

    pub fn to_file(&self, file_path: &str) {
        let mut content = String::new();
        content.push_str("# TODO\n\n");
        for item in &self.items {
            content.push_str(&item.to_string());
            content.push_str("\n");
        }
        fs::write(file_path, content).expect("Failed to write file");
    }
}

#[derive(Debug, PartialEq)]
pub struct Item {
    done: bool,
    todo: String,
}

impl Item {
    pub fn from_string(todo: &str) -> Item {
        let mut chars = todo.chars();
        chars.next(); // -
        chars.next(); // space
        chars.next(); // [
        let done = chars.next() == Some('X');
        chars.next(); // ]
        chars.next(); // space
        let todo: String = chars.collect(); // the todo

        Item { todo, done }
    }

    pub fn to_string(&self) -> String {
        let done = if self.done { "X" } else { " " };
        format!("- [{}] {}", done, self.todo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_list_item_unchecked() {
        let line = "- [ ] Item 1";
        let expected = Item {
            todo: "Item 1".to_string(),
            done: false,
        };

        assert_eq!(Item::from_string(line), expected);
    }

    #[test]
    fn parse_line_list_item_checked() {
        let line = "- [X] Item 1";
        let expected = Item {
            todo: "Item 1".to_string(),
            done: true,
        };
        assert_eq!(Item::from_string(line), expected);
    }

    #[test]
    fn parse_file_empty() {
        let file_path = "tests/empty.md";
        let expected = Items { items: vec![] };
        assert_eq!(Items::from_file(file_path), expected);
    }

    #[test]
    fn parse_file_one_item() {
        let file_path = "tests/one_item.md";
        let expected = Items {
            items: vec![Item {
                todo: "Item 1".to_string(),
                done: false,
            }],
        };
        assert_eq!(Items::from_file(file_path), expected);
    }

    #[test]
    fn parse_file_two_items() {
        let file_path = "tests/two_items.md";
        let expected = Items {
            items: vec![
                Item {
                    todo: "Item 1".to_string(),
                    done: false,
                },
                Item {
                    todo: "Item 2".to_string(),
                    done: true,
                },
            ],
        };
        assert_eq!(Items::from_file(file_path), expected);
    }

    #[test]
    fn parse_file_empty_lines() {
        let file_path = "tests/empty_lines.md";
        let expected = Items {
            items: vec![
                Item {
                    todo: "Item 1".to_string(),
                    done: false,
                },
                Item {
                    todo: "Item 2".to_string(),
                    done: true,
                },
            ],
        };
        assert_eq!(Items::from_file(file_path), expected);
    }

    #[test]
    fn write_line_list_item_unchecked() {
        let item = Item {
            todo: "Item 1".to_string(),
            done: false,
        };

        let expected = "- [ ] Item 1";
        assert_eq!(item.to_string(), expected);
    }

    #[test]
    fn write_line_list_item_checked() {
        let item = Item {
            todo: "Item 1".to_string(),
            done: true,
        };
        let expected = "- [X] Item 1";
        assert_eq!(item.to_string(), expected);
    }

    #[test]
    fn write_file_empty() {
        let items = Items { items: vec![] };
        items.to_file("tests/tmp/empty.md");

        let expected = fs::read_to_string("tests/empty.md").unwrap();
        let actual = fs::read_to_string("tests/tmp/empty.md").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_file_one_item() {
        let items = Items {
            items: vec![Item {
                todo: "Item 1".to_string(),
                done: false,
            }],
        };
        items.to_file("tests/tmp/one_item.md");

        let expected = fs::read_to_string("tests/one_item.md").unwrap();
        let actual = fs::read_to_string("tests/tmp/one_item.md").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_file_two_items() {
        let items = Items {
            items: vec![
                Item {
                    todo: "Item 1".to_string(),
                    done: false,
                },
                Item {
                    todo: "Item 2".to_string(),
                    done: true,
                },
            ],
        };
        items.to_file("tests/tmp/two_items.md");
        let expected = fs::read_to_string("tests/two_items.md").unwrap();
        let actual = fs::read_to_string("tests/tmp/two_items.md").unwrap();
        assert_eq!(actual, expected);
    }
}
