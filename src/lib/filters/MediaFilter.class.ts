const audioFormats = [
  "mp3",
  "wav",
  "ogg",
  "flac",
  "aac",
  "m4a",
  "wma",
  "alac",
  "opus",
  "aiff",
  "amr",
  "au",
  "caf",
  "ra",
  "mid",
  "snd",
  "voc",
  "rm",
  "midi",
  "dts",
  "ac3",
  "mpc",
  "tta",
  "wv",
  "spx",
  "adts",
  "caf",
];

const videoFormats = [
  "mp4",
  "mov",
  "avi",
  "mkv",
  "webm",
  "flv",
  "wmv",
  "m4v",
  "mpg",
  "mpeg",
  "3gp",
  "ogv",
  "ts",
  "m2ts",
  "mts",
  "rm",
  "asf",
  "divx",
  "xvid",
  "f4v",
  "dv",
  "vob",
  "amv",
  "bik",
  "gvi",
  "mxf",
  "yuv",
  "ivf",
];

class MediaFilter {
  private readonly videoFormat = new Set(videoFormats);
  private readonly audioFormat = new Set(audioFormats);

  isVideoFormat(ext: string) {
    return this.videoFormat.has(ext.toLocaleLowerCase());
  }

  isAudioFormat(ext: string) {
    return this.audioFormat.has(ext.toLocaleLowerCase());
  }
}

export const mediaFilterService = new MediaFilter();
