use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let home_dir = env::var("HOME")?;

    let zsh_history_path = format!("{}/.zsh_history", home_dir);

    let file = File::open(&zsh_history_path)?;
    let reader = io::BufReader::new(file);

    let mut counter: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        if let Some((_, cmd)) = line?.split_once(';') {
            let cmd = cmd.trim();
            if let Some(command_name) = cmd.split(' ').nth(0) {
                *counter.entry(command_name.to_string()).or_default() += 1;
            }
        }
    }

    // println!("{:#?}", counter);
    let mut sorted_entries: Vec<_> = counter.into_iter().collect();
    sorted_entries.sort_by(|a, b| a.1.cmp(&b.1));

    for (command, count) in sorted_entries {
        println!("{}: {}", command, count);
    }
    Ok(())
}
