type AudioQuality = "Low" | "Medium" | "High" | "Ultra" | "Unknown";

class MediaDisplayHelper {
  /**
   * Categorizes audio quality based on bitrate (kbps).
   * Thresholds are tuned for modern codecs (Opus/AAC) common in yt-dlp sources.
   */
  getAudioQualityLabel(tbr: number | null | undefined): AudioQuality {
    if (!tbr || tbr <= 0) return "Unknown";

    // Thresholds based on modern perceptual transparency
    if (tbr >= 250) {
      return "Ultra"; // Near-lossless or high-bitrate MP3/FLAC
    } else if (tbr >= 120) {
      return "High"; // Most YouTube "High" quality Opus (128-160k)
    } else if (tbr >= 64) {
      return "Medium"; // Standard mobile streaming
    } else {
      return "Low"; // Narrowband or highly compressed speech
    }
  }

  getSizeConverter(size: number) {
    // takes value size as bytes

    if (size <= 0 && !size) return "0 B";

    const units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let unitIndex = 0;

    const i = Math.floor(Math.log(size) / Math.log(1024));

    const value = size / Math.pow(1024, i);

    return `${value.toFixed(i === 0 ? 0 : 2)} ${units[i]}`;
  }
}

export const mediaDisplayHelperService = new MediaDisplayHelper();
