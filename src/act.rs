extern crate wordexp;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};
use std::path::Path;
use std::time::{Duration, SystemTime};

pub struct Activity {
    name: String,
    note: String,
    cooloff_days: u32,
    last_activity: u64
}

/*impl<'a> Activity<'a> {
    // Create an Activity from a str of form "name, note".
    fn from_csv(s: &'a str) -> Option<Self> {
        s.split(',').collect_tuple().map(
            |(last, first)| Person { first, last }
        )
    }
}*/

fn init_activity(name: &str, note: &str, cooloff_days: u32) -> Activity {
    Activity {
        name: name.to_string(),
        note: note.to_string(),
        cooloff_days,
        last_activity: 0
    }
}

fn expand_file_path(file_path: String) -> String {
    use self::wordexp::{wordexp, Wordexp};
    // Returns Result<Wordexp, WordexpError>
    let result = wordexp(&file_path, Wordexp::new(0), 0);

    if result.is_err() {
        panic!("Could not expand file path");
    }

    // we_wordv is Vec<Option<str'>>
    let expanded: String = match result.unwrap().we_wordv[0] {
        Some(w) => w.to_string(),
        None => panic!("Result is empty"),
    };

    return expanded;
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn load_activities(file_path: String) -> Vec<Activity> {
    let mut activities: Vec<Activity> = Vec::new();
    let expanded_file_path = expand_file_path(file_path);

    if let Ok(lines) = read_lines(expanded_file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(raw) = line {
                // Ignore empty and commented out lines
                if raw == "" || raw.starts_with('#') {
                    continue;
                }
                // Get an iterator for the line parts
                let mut line_parts = raw.split(",");

                // TODO: Get rid of unwrap() calls, since they can panic
                let name = line_parts.next().unwrap();
                let note = line_parts.next().unwrap();
                let cooloff_days = line_parts.next().unwrap().parse::<u32>().unwrap();

                activities.push(init_activity(name, note, cooloff_days));
            }
        }
    }

    return activities;
}

fn get_elapsed_days(last_activity: u64) -> String {
    // Get current timestamp
    let now: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    // Calculate number of elapsed days
    let seconds_in_day = 43200;
    let elapsed_days = (now - last_activity) / seconds_in_day;

    return format!("{} Tage", elapsed_days);
}

pub fn print_activity(activity: Activity) {
    let last_activity: String = if activity.last_activity == 0 {
        "Never".to_string()
    } else {
        get_elapsed_days(activity.last_activity)
    };

    println!("{} ({} - Alle {} Tage) Letzte: {}", activity.name, activity.note, activity.cooloff_days, last_activity);
}
