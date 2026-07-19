use anyhow::Result;
use std::{
    env::current_dir,
    fs::{read_dir, rename},
    io,
};

fn get_input(text: &str) -> String {
    let mut input = String::new();
    println!("{}", text);
    io::stdin().read_line(&mut input).unwrap();
    input
}

struct App {
    working_directory: String,
    desired_characters: Vec<String>,
    valid_characters: Vec<String>,
}

impl App {
    fn new(active_chars: Vec<String>) -> Self {
        let working_directory = format!("{}/Shaders", current_dir().unwrap().to_string_lossy());
        Self {
            working_directory: working_directory,
            desired_characters: active_chars,
            valid_characters: Vec::new(),
        }
    }

    fn load_valid_characters(&mut self) -> Result<()> {
        let entries = read_dir(&self.working_directory).unwrap();
        let mut valid_characters = Vec::with_capacity(60);
        for entry in entries {
            let valid_entry = entry?;
            let metadata = valid_entry.metadata()?;
            if !metadata.is_dir() {
                continue;
            }
            let file_name = valid_entry.file_name().into_string().unwrap();
            valid_characters.push(file_name.to_ascii_lowercase());
        }

        self.valid_characters = valid_characters;
        Ok(())
    }

    fn block_undesired_folders(&self) -> Result<()> {
        if self.desired_characters.is_empty() {
            return Ok(());
        }
        let entries = read_dir(&self.working_directory).unwrap();
        for entry in entries {
            let valid_entry = entry?;
            let metadata = valid_entry.metadata()?;
            if !metadata.is_dir() {
                continue;
            }
            let file_name = valid_entry.file_name().into_string().unwrap();

            println!("{file_name}");
            if file_name.contains("DISABLED") {
                if self.desired_characters.iter().any(|c| {
                    file_name
                        .to_ascii_lowercase()
                        .contains(&c.to_ascii_lowercase())
                }) {
                    let file_path = format!("{}/{}", self.working_directory, file_name);
                    let enabled = format!(
                        "{}/{}",
                        self.working_directory,
                        file_name.trim().replace("DISABLED", "").trim()
                    );
                    rename(file_path, enabled)?;
                }
            } else {
                if self
                    .desired_characters
                    .iter()
                    .all(|c| self.valid_characters.contains(c))
                {
                    if !self.desired_characters.iter().any(|c| {
                        file_name
                            .to_ascii_lowercase()
                            .contains(&c.to_ascii_lowercase())
                    }) {
                        //to block
                        let file_path = format!("{}/{}", self.working_directory, file_name);
                        let disabled = format!("{}/DISABLED {}", self.working_directory, file_name);
                        //println!("{:#?}", self.desired_characters);
                        rename(file_path, disabled)?;
                    } else {
                        println!("Wrong");
                    }
                }
            }
        }
        Ok(())
    }

    fn revert_files(&self) {
        let entries = read_dir(&self.working_directory).unwrap();
        for entry in entries {
            if let Ok(valid_entry) = entry {
                if valid_entry
                    .file_name()
                    .to_string_lossy()
                    .contains("DISABLED")
                {
                    let file_name = valid_entry.file_name().into_string().unwrap();
                    let file_path = format!("{}/{}", self.working_directory, file_name);
                    let enabled = format!(
                        "{}/{}",
                        self.working_directory,
                        file_name.trim().replace("DISABLED", "").trim()
                    );
                    //println!("from {file_path} to {enabled}");
                    rename(&file_path, &enabled).unwrap();
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let mut characters: Vec<String> = Vec::new();
    let inp = get_input(
        "Would you like to revert to default or activate custom characters? Type either (r)/(a)",
    );
    if inp.trim() == "a" {
        let inp = get_input(
            "\n Separated by whitespace, case non-sensitive, type out which characters to enable \n",
        );

        let separated_iter = inp.split_whitespace().count();
        characters.reserve(separated_iter);
        for word in inp.split_whitespace() {
            characters.push(word.to_ascii_lowercase());
        }

        let mut inst = App::new(characters);
        inst.load_valid_characters()?;
        inst.block_undesired_folders()?;
        Ok(())
    } else if inp.trim() == "r" {
        let mut inst = App::new(characters);
        inst.load_valid_characters()?;
        inst.revert_files();
        Ok(())
    } else {
        return Err(anyhow::anyhow!("Invalid input: {}", inp));
    }
}

// refactored, most edge cases should be covered
// still not final
