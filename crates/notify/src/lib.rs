pub mod window;

use notify_rust::Notification;
use std::process::Command;

pub fn modal(unit: &str, exe: &str, message: &str) -> anyhow::Result<()> {
    let exe_path = std::env::current_exe()?;

    let exe_str = exe_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("executable path is not valid UTF-8"))?;

    Command::new(exe_str)
        .args([
            "reporter",
            "-u",
            &format!("{unit}"),
            "-e",
            &format!("{exe}"),
            "-m",
            &format!("{message}"),
        ])
        .spawn()?;

    Notification::new()
        .summary("Crash detected")
        .body(message)
        .icon("dialog-error")
        .show()?;

    Ok(())
}
