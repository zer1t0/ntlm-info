use log::warn;
use std::fs::File;
use std::io::Lines;
use std::io::{self, BufRead, BufReader, Stdin};

/// Function to read the inputs from files, strings or stdin
/// in a normalized iterator of strings.
pub fn read_inputs(
    inputs: Vec<String>,
    use_stdin: bool,
    filter_blanks: bool,
) -> impl Iterator<Item = String> {
    let mut input_iter: Box<dyn Iterator<Item = String>>;
    input_iter = if inputs.len() == 0 && use_stdin {
        Box::new(StdinIter::new())
    } else {
        Box::new(FileStringIter::new(inputs))
    };

    input_iter = Box::new(input_iter.map(|s| s.trim().to_string()));

    input_iter = if filter_blanks {
        Box::new(input_iter.filter(|s| s.len() != 0))
    } else {
        input_iter
    };

    return input_iter.filter(|s| !s.starts_with("#"));
}

/// Class to get an iterator of stdin lines, since
/// io::stdin().lock().lines() gives problems with lifetimes.
pub struct StdinIter {
    stdin: Stdin,
}

impl StdinIter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Iterator for StdinIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.stdin.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    // EOF
                    return None;
                }
                return Some(line);
            }
            Err(_) => return None,
        }
    }
}

impl Default for StdinIter {
    fn default() -> Self {
        Self { stdin: io::stdin() }
    }
}

/// Class to read a bunch of strings that could be filenames.
/// If string is a valid path, then the lines of the file are retrieved,
/// in other case, the string itself is retrieved.
pub struct FileStringIter {
    items: Vec<String>,
    lines: Option<Lines<BufReader<File>>>,
    current_path: String,
}

impl FileStringIter {
    pub fn new(mut items: Vec<String>) -> Self {
        items.reverse();
        return Self {
            items,
            lines: None,
            current_path: "".to_string(),
        };
    }
}

impl Iterator for FileStringIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            if let Some(lines) = &mut self.lines {
                if let Some(line) = lines.next() {
                    match line {
                        Ok(line) => {
                            return Some(line);
                        }
                        Err(err) => {
                            warn!("Error reading lines in {}: {}. '{}' is taken as URL path ",
                                  self.current_path, err, self.current_path);
                            self.lines = None;
                            let current_path = self.current_path.clone();
                            self.current_path = "".to_string();
                            self.lines = None;
                            return Some(current_path);
                        }
                    }
                } else {
                    self.lines = None;
                }
            }

            let item = self.items.pop()?;

            match File::open(&item) {
                Ok(file) => {
                    self.current_path = item;
                    self.lines = Some(BufReader::new(file).lines());
                }
                Err(_) => {
                    return Some(item);
                }
            }
        }
    }
}
