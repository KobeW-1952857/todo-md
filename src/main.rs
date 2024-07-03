use std::env;

fn usage(program: &String) {
    println!("Usage: {program} <command> [<args>]");
    println!("\nCommands:");
    println!("  add \"<item>\"    Add a new item to the list");
    println!("  done <number>   Mark an item as done");
    println!("  list            List all items");
}

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();

    if args.len() == 0 {
        usage(&program);
        return;
    }

    if let Some(command) = args.next() {
        match command.as_str() {
            "add" => {
                let item: String = args.next().unwrap_or_else(|| {
                    println!("No item provided");
                    usage(&command);
                    std::process::exit(1);
                });
                println!("Adding item: {}", item);
            }
            "done" => {
                let number: i32 = args
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

                println!("Marking item {} as done", number);
            }
            "list" => {
                println!("Listing items");
            }
            _ => {
                println!("Unknown command: {}", command);
                usage(&command);
            }
        }
    }
}
