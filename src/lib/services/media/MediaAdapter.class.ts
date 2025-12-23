/*
*Accepting raw JSON
*Validating / normalizing
*Producing typed domain objects
*No I/O responsibility

parse(rawJson)
parseMany(rawJsonArray)
fromRaw(raw)
fromBackendResponse(response)
parseMediaList(rawList)

*/

import type { Media } from "../tuari-command/tuari-command.types";

//! dirty data in, clean data out

export class MediaAdapter {
  constructor() {}

  data = $state<Media | null>(null);

  async parse(url: string) {
    const media: Media = await this.getData(url);
    console.log("media: ", media);

    // for (const format of media.formats) {
    //   if (format.kind === "video") {
    //     format.width;
    //     format.height;
    //   }

    //   if (format.kind === "audio") {
    //     // audio-only
    //   }
    // }

    return;
  }

  private async getData(url: string) {
    try {
      const response = await fetch(url);
      return await response.json();
    } catch (e) {
      console.error(e);
    }
  }
}

export const mediaAdapterService = new MediaAdapter();

/*
id
title
thumbnail
description
url
videoFormats[]:{
    id
    title
    thumbnail
    description
    url
    ext
    format
    filesize
    protocol
    type
}
audioFormats[]:{
    id
    title
    thumbnail
    description
    url
    ext
    format
    filesize
    protocol
    type
 */
