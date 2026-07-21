use anyhow::{Result, anyhow};
use std::{
    env::current_dir,
    fs::{ReadDir, read_dir, rename},
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
    entries: Option<ReadDir>,
}

impl App {
    fn new(active_chars: Vec<String>) -> Self {
        let working_directory = format!("{}/Shaders", current_dir().unwrap().to_string_lossy());
        Self {
            working_directory: working_directory,
            desired_characters: active_chars,
            valid_characters: Vec::new(),
            entries: None,
        }
    }

    fn load_valid_characters_and_entries(&mut self, debug_mode: bool) -> Result<()> {
        let now = std::time::Instant::now();

        let entries = read_dir(&self.working_directory)?;
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
        let entries = read_dir(&self.working_directory)?;
        self.valid_characters = valid_characters;
        self.entries = Some(entries);
        if debug_mode {
            println!(
                "Loaded valid characters and entries in {}ms",
                now.elapsed().as_millis()
            );
        }
        Ok(())
    }

    fn enable_all_but(self, debug_mode: bool) -> Result<()> {
        let now = std::time::Instant::now();

        if self.desired_characters.is_empty() {
            return Err(anyhow!("Empty user input"));
        }

        for entry in self.entries.unwrap() {
            let valid_entry = entry?;
            let metadata = valid_entry.metadata()?;
            if !metadata.is_dir() {
                continue;
            }
            let folder_name = valid_entry.file_name().into_string().unwrap();

            //println!("{file_name}");
            if folder_name.contains("DISABLED") {
                if !self.desired_characters.iter().any(|c| {
                    folder_name
                        .to_ascii_lowercase()
                        .contains(&c.to_ascii_lowercase())
                }) {
                    let folder_path = format!("{}/{}", self.working_directory, folder_name);
                    let enabled_folder_name = format!(
                        "{}/{}",
                        self.working_directory,
                        folder_name.trim().replace("DISABLED", "").trim()
                    );
                    rename(folder_path, enabled_folder_name)?;
                }
            } else {
                if self
                    .desired_characters
                    .iter()
                    .all(|c| self.valid_characters.contains(c))
                {
                    if self.desired_characters.iter().any(|c| {
                        folder_name
                            .to_ascii_lowercase()
                            .contains(&c.to_ascii_lowercase())
                    }) {
                        //to block
                        let folder_path = format!("{}/{}", self.working_directory, folder_name);
                        let disabled_folder_path =
                            format!("{}/DISABLED {}", self.working_directory, folder_name);
                        //println!("{:#?}", self.desired_characters);
                        rename(folder_path, disabled_folder_path)?;
                    } else {
                        //println!("Wrong");
                    }
                }
            }
        }
        if debug_mode {
            println!("Enabled all but in: {}ms", now.elapsed().as_millis());
        }
        Ok(())
    }

    fn disable_all_but(self, debug_mode: bool) -> Result<()> {
        let now = std::time::Instant::now();

        if self.desired_characters.is_empty() {
            return Ok(());
        }

        for entry in self.entries.unwrap() {
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
        if debug_mode {
            println!("Disabled all but in: {}ms", now.elapsed().as_millis());
        }
        Ok(())
    }

    fn disable_all(self, debug_mode: bool) -> Result<()> {
        let now = std::time::Instant::now();
        for entry in self.entries.unwrap() {
            let valid_entry = entry?;
            if !valid_entry
                .file_name()
                .to_string_lossy()
                .contains("DISABLED")
            {
                let folder_name = valid_entry.file_name().into_string().unwrap();
                let folder_path = format!("{}/{}", self.working_directory, folder_name);
                let disabled_folder_name =
                    format!("{}/DISABLED {}", self.working_directory, folder_name.trim());
                rename(&folder_path, &disabled_folder_name)?;
            }
        }
        if debug_mode {
            println!("Disabled all in: {}ms", now.elapsed().as_millis());
        }
        Ok(())
    }

    fn enable_all(self, debug_mode: bool) -> Result<()> {
        let now = std::time::Instant::now();
        for entry in self.entries.unwrap() {
            let valid_entry = entry?;
            if valid_entry
                .file_name()
                .to_string_lossy()
                .contains("DISABLED")
            {
                let folder_name = valid_entry.file_name().into_string().unwrap();
                let folder_path = format!("{}/{}", self.working_directory, folder_name);
                let enabled_folder_name = format!(
                    "{}/{}",
                    self.working_directory,
                    folder_name.trim().replace("DISABLED", "").trim()
                );

                rename(&folder_path, &enabled_folder_name)?;
            }
        }
        if debug_mode {
            println!("Enabled all in: {}ms", now.elapsed().as_millis());
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let inp = get_input(
        "Would you like to... \n(1) Enable all\n(2) Enable selected\n(3) Disable selected\n(4) Disable all\n",
    );

    if inp.trim() == "2" {
        let inp = get_input(
            "\n Separated by whitespace, case non-sensitive, type out which characters to enable \n",
        );
        let characters = init_inp(inp);
        let mut inst = App::new(characters);
        inst.load_valid_characters_and_entries(DEBUG_MODE)?;
        inst.disable_all_but(DEBUG_MODE)?;

        Ok(())
    } else if inp.trim() == "1" {
        let characters = init_inp(inp);
        let mut inst = App::new(characters);
        inst.load_valid_characters_and_entries(DEBUG_MODE)?;
        inst.enable_all(DEBUG_MODE)?;

        Ok(())
    } else if inp.trim() == "3" {
        let inp = get_input(
            "\n Separated by whitespace, case non-sensitive, type out which characters to disable \n",
        );
        let characters = init_inp(inp);
        let mut inst = App::new(characters);
        inst.load_valid_characters_and_entries(DEBUG_MODE)?;
        inst.enable_all_but(DEBUG_MODE)?;

        Ok(())
    } else if inp.trim() == "4" {
        let characters = init_inp(inp);
        let mut inst = App::new(characters);
        inst.load_valid_characters_and_entries(DEBUG_MODE)?;
        inst.disable_all(DEBUG_MODE)?;

        Ok(())
    } else {
        return Err(anyhow::anyhow!("Invalid input: {}", inp));
    }
}

fn init_inp(inp: String) -> Vec<String> {
    let mut characters: Vec<String> = Vec::new();
    let separated_iter = inp.split_whitespace().count();
    characters.reserve(separated_iter);
    for word in inp.split_whitespace() {
        characters.push(word.to_ascii_lowercase());
    }
    characters
}

const DEBUG_MODE: bool = false;
