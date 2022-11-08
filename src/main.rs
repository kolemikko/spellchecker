mod types;
use std::error::Error;
use types::SpellChecker;

const INPUT_FILE: &str = "loc_test.csv";

fn train_with_file(filepath: &str, index: Option<usize>) -> Result<(), Box<dyn Error>> {
    let mut spell = SpellChecker::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filepath)?;
    for result in reader.records() {
        let record = result?;
        match index {
            Some(s) => match record.get(s) {
                Some(r) => {
                    spell.train(r);
                }
                None => {}
            },
            None => {
                for idx in 0..record.len() {
                    match record.get(idx) {
                        Some(r) => {
                            spell.train(r);
                        }
                        None => {}
                    }
                }
            }
        }
    }
    // spell.print_database_with_threshold(1);
    spell.increase_training_count();
    spell.write_database_to_file();
    Ok(())
}

fn check_file(filepath: &str, index: Option<usize>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut not_found_words: Vec<String> = Vec::new();
    let mut spell = SpellChecker::new();
    let mut reader = csv::ReaderBuilder::new().from_path(filepath)?;
    for result in reader.records() {
        let record = result?;
        match index {
            Some(s) => match record.get(s) {
                Some(r) => {
                    let found = spell.check(r);
                    match found {
                        false => {
                            not_found_words.push(r.to_string());
                        }
                        true => {}
                    }
                }
                None => {}
            },
            None => {
                for idx in 0..record.len() {
                    match record.get(idx) {
                        Some(r) => {
                            let found = spell.check(r);
                            match found {
                                false => {
                                    not_found_words.push(r.to_string());
                                }
                                true => {}
                            }
                        }
                        None => {}
                    }
                }
            }
        }
    }
    Ok(not_found_words)
}

fn main() {
    let result = check_file(INPUT_FILE, Some(2));
    match result {
        Ok(res) => {
            for w in res {
                println!("{}", w);
            }
            println!("Done.");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
