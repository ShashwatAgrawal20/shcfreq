use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let home_dir = match env::var_os("HOME") {
        Some(home) => home,
        None => {
            panic!("where's the freaking $HOME");
        }
    };
    let zshrc_path = home_dir.into_string().expect("some shit happened");
    let zshrc_path = format!("{}/.zsh_history", zshrc_path);
    let contents = fs::read_to_string(zshrc_path).expect("you are a piece of shit");

    let mut counter: HashMap<&str, usize> = HashMap::new();

    for line in contents.lines() {
        if let Some((_, cmd)) = line.split_once(";") {
            *counter
                .entry(cmd.split(' ').nth(0).expect("bullshit"))
                .or_insert(0) += 1;
        }
    }
    let mut sorted_entries: Vec<_> = counter.iter().collect();
    sorted_entries.sort_by(|(_, a), (_, b)| a.cmp(b));

    for (key, value) in sorted_entries {
        println!("{}: {}", key, value);
    }
}
