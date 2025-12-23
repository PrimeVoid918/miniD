export type BaseMedia = {
  id: string;
  title: string;
  thumbnail: string;
  description: string;
  url: string;
};

export type Media = BaseMedia & {
  mediaType: "video" | "audio";
  formats: MediaFormat[];
};

export type BaseFormat = {
  url: string;
  ext: string;
  format: string;
  filesize: number;
  protocol?: string;
  video_ext: string | null;
  audio_ext: string | null;
};

export type VideoFormat = BaseFormat & {
  kind: "video";
  width: number;
  height: number;
};

export type AudioFormat = BaseFormat & {
  kind: "audio";
};

export type MediaFormat = VideoFormat | AudioFormat;
