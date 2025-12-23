use std::process::Command;

pub struct YtDLPAdapter;

impl YtDLPAdapter {
  // pub fn download(url: &str) -> String {
  //   Command::new("yt-dlp")
  // }
  pub fn fetch_metadata(url: &str) -> Result<String, std::io::Error> {
    let output = Command::new("yt-dlp").args(["-J", url]).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
  }
}
