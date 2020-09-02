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
        String::from("./default.def")
    };

    let mut activities = act::load_activities(&file_path);

    // sortByDueActivity
    act::sort_by_due_activity(&mut activities);

    //for activity in activities {
    //    act::print_activity(&activity);
    //}

    act::print_activity(&activities[0]);

    loop {
        let line: String = read!("{}\n");

        if line == "quit" {
            break;
        }
        if line == "ok" {
            act::add_activity_log(&activities[0]);
            break;
        }

        println!("Supported commands are 'ok' and 'quit'");
    }
}
