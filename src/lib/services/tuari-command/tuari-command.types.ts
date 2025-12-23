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

/*

filesize?: number;
filesizeApprox?: number;
url?: string;
formatId: string;
formatNote?: string;

//! this alings more on JSON response?
export type Media = {
  id: string;
  title: string;
  thumbnail: string;
  description?: string;
  url: string;
  mediaType: "video" | "audio";
  formats: MediaFormat[];
};

export type MediaFormat =
  | VideoFormat
  | AudioFormat;

export type BaseFormat = {
  formatId: string;
  ext: string;
  protocol?: string;
  filesize?: number;
  filesizeApprox?: number;
  videoExt?: string;
  audioExt?: string;
};

export type VideoFormat = BaseFormat & {
  kind: "video";
  width: number;
  height: number;
  fps?: number;
};

export type AudioFormat = BaseFormat & {
  kind: "audio";
  abr?: number;
};


*/