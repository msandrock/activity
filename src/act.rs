extern crate wordexp;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};
use std::path::Path;
use std::time::SystemTime;

static DATA_FILE: &str = "./activity.log";
const SECONDS_IN_DAY: u64 = 43200;

pub struct Activity {
    name: String,
    note: String,
    cooloff_days: u64,
    last_activity: u64
}

impl Activity {
    fn new(name: &str, note: &str, cooloff_days: u64) -> Activity {
        Activity {
            name: name.to_string(),
            note: note.to_string(),
            cooloff_days,
            last_activity: 0
        }
    }
}

fn expand_file_path(file_path: String) -> String {
    use self::wordexp::{wordexp, Wordexp};
    // Returns Result<Wordexp, WordexpError>
    let result = wordexp(&file_path, Wordexp::new(0), 0);

    if result.is_err() {
        panic!("Could not expand file path. Please make sure the file exists.");
    }

    // we_wordv is Vec<Option<str'>>
    let expanded: String = match result.unwrap().we_wordv[0] {
        Some(w) => w.to_string(),
        None => panic!("Could not expand file path. Result is empty."),
    };

    return expanded;
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn parse_activity_line(line: String) -> std::result::Result<Activity, &'static str> {
    if line == "" || line.starts_with('#') {
        return Err("Empty or disabled line");
    }

    // Get an iterator for the line parts
    let mut line_parts = line.split(",");

    let name: &str = match line_parts.next() {
        Some(n) => n.trim_matches('"'),
        None => {
            println!("Skipping line {}", line);
            return Err("Missing name");
        }
    };

    let note: &str = match line_parts.next() {
        Some(n) => n,
        None => {
            println!("Skipping line {}", line);
            return Err("Missing note");
        }
    };

    let cooloff_days = match line_parts.next() {
        Some(d) => match d.parse::<u64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Skipping line {}", line);
                return Err("Error parsing cooloff days");
            }
        },
        None => {
            println!("Skipping line {}", line);
            return Err("Missing cooloff days");
        }
    };

    return Ok(Activity::new(name, note, cooloff_days));
}

pub fn load_activities(file_path: String) -> Vec<Activity> {
    let mut activities: Vec<Activity> = Vec::new();
    let expanded_file_path = expand_file_path(file_path);

    if let Ok(lines) = read_lines(expanded_file_path) {
        // Consumes the iterator, returns an Option<String>
        for line in lines {
            if let Ok(raw) = line {
                match parse_activity_line(raw) {
                    Ok(activity) => activities.push(activity),
                    Err(e) => println!("Skipping line - {}", e)
                }
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
    let elapsed_days = (now - last_activity) / SECONDS_IN_DAY;

    return format!("{} Tage", elapsed_days);
}

pub fn print_activity(activity: &Activity) {
    let last_activity: String = if activity.last_activity == 0 {
        String::from("Never")
    } else {
        get_elapsed_days(activity.last_activity)
    };

    println!("{} ({} - Alle {} Tage) Letzte: {}", activity.name, activity.note, activity.cooloff_days, last_activity);
}

fn find_activity(activites: &Vec<Activity>, name: &str) -> Option<usize> {
    let mut index: i32 = 0;

    for activity in activites {
        if activity.name == name {
            return Some(index as usize);
        }

        index += 1;
    }

    return None;
}

pub fn sort_by_due_activity<'a>(activities: &'a mut Vec<Activity>) {
    let expanded_file_path = expand_file_path(DATA_FILE.to_string());

    if let Ok(lines) = read_lines(expanded_file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(raw) = line {
                // Split line by ; delimiter
                let mut iterator = raw.split(";");

                let name = iterator.next().unwrap();
                let last_activity = iterator.next().unwrap().parse::<u64>().unwrap();

                // Find the activity in the activities array
                let activity_index = find_activity(activities, name);
                // Skip unknown activities
                if activity_index.is_none() {
                    continue;
                }

                let i = activity_index.unwrap();

                // Update the array, if there is a newer activity in the log
                if activities[i].last_activity < last_activity {
                    activities[i].last_activity = last_activity;
                }
            }
        }
    }

    // Sort all activities by difference between lastActivity minus cooloff time
    let cmp = |l: &Activity, r: &Activity| (l.last_activity + (l.cooloff_days * SECONDS_IN_DAY)).cmp(&(r.last_activity + (r.cooloff_days * SECONDS_IN_DAY)));
    activities.sort_by(cmp)
}

pub fn add_activity_log(activity: &Activity) {
    let expanded_file_path = expand_file_path(DATA_FILE.to_string());

    // Get current timestamp
    let now: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(expanded_file_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{};{}", activity.name, now) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
