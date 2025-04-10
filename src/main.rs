use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

fn main() -> io::Result<()> {
    if !is_just_available() {
        eprintln!("Error: 'just' command not found. Please install just first.");
        std::process::exit(1);
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let formatted = format_justfile(&input)?;

    io::stdout().write_all(&formatted)?;
    io::stdout().flush()?;

    Ok(())
}

fn is_just_available() -> bool {
    Command::new("just")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn format_justfile(input: &str) -> io::Result<Vec<u8>> {
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(input.as_bytes())?;

    let output = Command::new("just")
        .args(["--dump", "-f", temp_file.path().to_str().unwrap()])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()?;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "just command failed to format the file",
        ))
    }
}
