use tokio::process::Command;

pub async fn open_browser(url: &str) {
    let (command, args) = if cfg!(target_os = "windows") {
        ("cmd", vec!["/C", "start", url])
    } else if cfg!(target_os = "macos") {
        ("open", vec![url])
    } else if cfg!(target_os = "linux") {
        ("xdg-open", vec![url])
    } else {
        log::warn!("Unsupported OS");
        return;
    };

    if let Err(err) = Command::new(command)
        .args(&args)
        .spawn()
    {
        log::error!("Failed to open browser: {}", err);
    }
}
