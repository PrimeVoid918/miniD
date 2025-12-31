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

import type { ApiResponse } from "../../api/protocol/api-response";
import type { Media } from "../tuari-command/tuari-command.types";
import { invoke } from "@tauri-apps/api/core";

export class MediaAdapter {
  constructor() {}

  data = $state<Media | null>(null);

  async getMediaMetadata(url: string) {
    const media = await this.getData(url);

    return media;
  }

  private async getData(url: string) {
    try {
      const response = await invoke<ApiResponse<Media>>("fetch_media", {
        url: url,
      });
      return response;
    } catch (e) {
      console.error("MediaAdapter Class Error", e);
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
