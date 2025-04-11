use std::error::Error;
use std::io::Write;
use std::process::{Child, Command, Stdio};

pub fn display_with_bat(data: &str) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new("bat");

    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let mut child: Child = cmd.spawn().map_err(|e| {
        format!(
            "Failed to spawn bat command. Is 'bat' installed and in PATH? Error: {}",
            e
        )
    })?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(data.as_bytes())
            .map_err(|e| format!("Failed to write to bat's stdin: {}", e))?;
    } else {
        return Err("Failed to get handle to bat's stdin.".into());
    }

    let status = child.wait()?;
    if !status.success() {
        eprintln!("bat command finished with non-zero status: {}", status);
    }

    Ok(())
}
