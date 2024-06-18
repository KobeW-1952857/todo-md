use crate::ast::*;

fn parse_file(file_path: &str) -> Vec<Item> {
    let file = std::fs::read_to_string(file_path).expect("Failed to read file");
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
}
