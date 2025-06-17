use std::error::Error;
use std::fs;
use std::io::Write;

use crate::{
    config::{self, Config},
    theme::Theme,
};

fn set_default_profile(theme: &Theme, config: &Config) -> Result<(), Box<dyn Error>> {
    // Read the file contents into a String
    let contents = fs::read_to_string(&config.konsolerc)?;

    // Prepare a buffer to store the modified lines
    let mut output = Vec::new();
    let mut in_desktop_entry = false;

    for line in contents.lines() {
        let trimmed = line.trim();

        // Track section headers
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_desktop_entry = trimmed == "[Desktop Entry]";
        }

        // Update the DefaultProfile key if in the right section
        if in_desktop_entry && trimmed.starts_with("DefaultProfile=") {
            output.push(format!("DefaultProfile={}.profile", theme.to_string()).to_string());
        } else {
            output.push(line.to_string());
        }
    }

    // Write the modified content back to the file
    let mut file = fs::File::create(&config.konsolerc)?;
    for line in output {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn set_session_theme(session_id: String, theme: &Theme) {
    todo!()
}

fn get_session_ids() -> Vec<String> {
    todo!()
}

pub fn set(theme: &Theme, config: &Config) {
    let _ = set_default_profile(theme, config);

    // let session_ids = get_session_ids();

    // for id in session_ids.iter() {
    //     println!("{}", id);
    // }
}
