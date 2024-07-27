use std::env;

use todo_md::TodoList;

fn usage(program: &String) {
    println!("Usage: {program} <command> [<args>]");
    println!("\nCommands:");
    println!("  add \"<item>\"        Add a new item to the list");
    println!("  done <number>       Mark an item as done");
    println!("  mark <number>       Mark an item as done");
    println!("  complete <number>   Mark an item as done");
    println!("  list                List all items");
    println!("  remove <number>     Remove an item from the list");
    println!("  remove completed    Remove all completed items from the list");
    println!("  remove all          Remove all items from the list");
    println!("  help                Display this help message");
}

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();

    if args.len() == 0 {
        usage(&program);
        return;
    }

    if let Some(command) = args.next() {
        let mut todo_list = TodoList::from_file("todo.md");
        match command.as_str() {
            "add" => {
                let item: String = args.next().unwrap_or_else(|| {
                    println!("No item provided");
                    usage(&command);
                    std::process::exit(1);
                });
                todo_list.add_item(item);
            }
            "done" | "complete" | "mark" => {
                let number: usize = args
                    .next()
                    .unwrap_or_else(|| {
                        println!("No item number provided");
                        usage(&command);
                        std::process::exit(1);
                    })
                    .parse()
                    .unwrap_or_else(|input| {
                        println!("{input}");
                        usage(&command);
                        std::process::exit(1);
                    });
                todo_list.mark_done(number);
            }
            "list" => {
                todo_list.list_items();
            }
            "help" => {
                usage(&program);
                std::process::exit(0);
            }
            "remove" => {
                let input = args.next().unwrap_or_else(|| {
                    println!("No item number provided");
                    usage(&command);
                    std::process::exit(1);
                });

                match input.as_str() {
                    "completed" => {
                        todo_list.remove_completed();
                    }
                    "all" => {
                        todo_list.remove_all();
                    }
                    &_ => {
                        let number: usize = input.parse().unwrap_or_else(|input| {
                            println!("{input}");
                            usage(&command);
                            std::process::exit(1);
                        });
                        todo_list.remove(number);
                    }
                }
            }
            &_ => {
                println!("Unknown command: {}", command);
                usage(&command);
                std::process::exit(1);
            }
        }
        todo_list.save("todo.md");
    } else {
        usage(&program);
    }
}
