use std::process::Command;

pub struct YtDLPAdapter;

impl YtDLPAdapter {
  pub fn fetch_metadata(url: &str) -> Result<String, std::io::Error> {
    let output = Command::new("yt-dlp").args(["-J", url]).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
  }

  pub fn download(url: &str, format_id: &str) -> Result<String, std::io::Error> {
    let output = Command::new("yt-dlp")
      .args([
        "-f",
        format_id, // Tells yt-dlp exactly which stream to pick
        url,
        "-o",
        "%(title)s.%(ext)s", // Standard output template
      ])
      .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
  }
}
