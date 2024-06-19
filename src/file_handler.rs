use crate::ast::*;
use std::fs;

pub fn parse_file(file_path: &str) -> Vec<Item> {
    let file = fs::read_to_string(file_path).expect("Failed to read file");
    let items = file
        .lines()
        .skip(1) // Skip the title
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(parse_line(line))
            }
        })
        .collect();
    items
}

fn parse_line(line: &str) -> Item {
    //- [ ] Item 1

    let mut chars = line.chars();
    chars.next(); // -
    chars.next(); // space
    chars.next(); // [
    let done = chars.next() == Some('X');
    chars.next(); // ]
    chars.next(); // space
    let todo: String = chars.collect(); // the todo

    Item::new(todo, done)
}

pub fn write_file(file_path: &str, items: &Vec<Item>) {
    let mut content = String::new();
    content.push_str("# TODO\n\n");
    for item in items {
        content.push_str(&write_line(item));
        content.push_str("\n");
    }
    fs::write(file_path, content).expect("Failed to write file");
}

fn write_line(item: &Item) -> String {
    let done = if item.is_done() { "X" } else { " " };
    format!("- [{}] {}", done, item.get_todo())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_list_item_unchecked() {
        let line = "- [ ] Item 1";
        let expected = Item::new("Item 1".to_string(), false);
        assert_eq!(parse_line(line), expected);
    }

    #[test]
    fn parse_line_list_item_checked() {
        let line = "- [X] Item 1";
        let expected = Item::new("Item 1".to_string(), true);
        assert_eq!(parse_line(line), expected);
    }

    #[test]
    fn parse_file_empty() {
        let file_path = "tests/empty.md";
        let expected = vec![];
        assert_eq!(parse_file(file_path), expected);
    }

    #[test]
    fn parse_file_one_item() {
        let file_path = "tests/one_item.md";
        let expected = vec![Item::new("Item 1".to_string(), false)];
        assert_eq!(parse_file(file_path), expected);
    }

    #[test]
    fn parse_file_two_items() {
        let file_path = "tests/two_items.md";
        let expected = vec![
            Item::new("Item 1".to_string(), false),
            Item::new("Item 2".to_string(), true),
        ];
        assert_eq!(parse_file(file_path), expected);
    }

    #[test]
    fn parse_file_empty_lines() {
        let file_path = "tests/empty_lines.md";
        let expected = vec![
            Item::new("Item 1".to_string(), false),
            Item::new("Item 2".to_string(), true),
        ];
        assert_eq!(parse_file(file_path), expected);
    }

    #[test]
    fn write_line_list_item_unchecked() {
        let line = Item::new("Item 1".to_string(), false);

        let expected = "- [ ] Item 1";
        assert_eq!(write_line(&line), expected);
    }

    #[test]
    fn write_line_list_item_checked() {
        let line = Item::new("Item 1".to_string(), true);
        let expected = "- [X] Item 1";
        assert_eq!(write_line(&line), expected);
    }

    #[test]
    fn write_file_empty() {
        let items = vec![];
        write_file("tests/tmp/empty.md", &items);

        let expected = fs::read_to_string("tests/empty.md").unwrap();
        let actual = fs::read_to_string("tests/tmp/empty.md").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_file_one_item() {
        let items = vec![Item::new("Item 1".to_string(), false)];
        write_file("tests/tmp/one_item.md", &items);

        let expected = fs::read_to_string("tests/one_item.md").unwrap();
        let actual = fs::read_to_string("tests/tmp/one_item.md").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_file_two_items() {
        let items = vec![
            Item::new("Item 1".to_string(), false),
            Item::new("Item 2".to_string(), true),
        ];
        write_file("tests/tmp/two_items.md", &items);
        let expected = fs::read_to_string("tests/two_items.md").unwrap();
        let actual = fs::read_to_string("tests/tmp/two_items.md").unwrap();
        assert_eq!(actual, expected);
    }
}
