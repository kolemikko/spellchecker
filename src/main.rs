use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

const INPUT_FILE: &str = "loc_test.csv";
const DATABASE_FILE: &str = "wordlist.csv";

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    word: String,
    instances: Option<u32>,
    training_count: Option<u32>,
}

#[derive(Default)]
pub struct SpellChecker {
    pub database: HashMap<String, u32>,
    training_count: u32,
}

impl SpellChecker {
    pub fn train(&mut self, text: &str) {
        let regex = Regex::new(r"[a-z]+").unwrap();
        for m in regex.find_iter(&text.to_lowercase()) {
            let count = self.database.entry(m.as_str().to_string()).or_insert(0);
            *count += 1;
        }
    }

    pub fn check(&mut self, word: &str) -> bool {
        if self.database.contains_key(word) {
            return true;
        }
        false
    }

    pub fn increase_training_count(&mut self) {
        self.training_count += 1;
    }

    pub fn print_database(&self) {
        for i in self.database.iter() {
            println!("{} : {}", i.0, i.1);
        }
    }

    pub fn print_database_with_threshold(&self, threshold: u32) {
        for i in self.database.iter() {
            if i.1 <= &threshold {
                println!("{} : {}", i.0, i.1);
            }
        }
    }

    pub fn write_database_to_file(&self) {
        let mut wtr = csv::Writer::from_path(DATABASE_FILE).unwrap();
        for i in self.database.iter() {
            wtr.serialize(Entry {
                word: i.0.to_string(),
                instances: Some(*i.1),
                training_count: Some(self.training_count),
            })
            .unwrap();
        }
        wtr.flush().unwrap();
    }

    pub fn read_database_from_file(&mut self) {
        let mut reader = csv::Reader::from_path(DATABASE_FILE).unwrap();
        for result in reader.deserialize() {
            let record: Entry = result.unwrap();
            self.database.insert(record.word, record.instances.unwrap());
            self.training_count = record.training_count.unwrap();
        }
    }
}

fn read_file() -> Result<(), Box<dyn Error>> {
    let mut spell = SpellChecker::default();
    spell.read_database_from_file();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(INPUT_FILE)?;
    for result in reader.records() {
        let record = result?;
        match record.get(2) {
            Some(r) => {
                spell.train(r);
            }
            None => {}
        }
    }
    // spell.print_database_with_threshold(1);
    spell.increase_training_count();
    spell.write_database_to_file();
    Ok(())
}
fn main() {
    let result = read_file();
    match result {
        Ok(_) => {
            println!("Done.");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
