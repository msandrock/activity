extern crate text_io;

mod act;

use std::env;
use text_io::read;

fn main() {
    // Parse command line and accept custom definitions file
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        format!("./{}.def", args[1].trim())
    } else {
        "./default.def".to_string()
    };

    let activities = act::load_activities(file_path);

    // sortByDueActivity

    // printActivity

    loop {
        let line: String = read!("{}\n");

        if line == "quit" {
            break;
        }
        if line == "ok" {
            // add activity
            break;
        }

        println!("Ich verstehe nur 'ok' und 'quit'");
    }
}
