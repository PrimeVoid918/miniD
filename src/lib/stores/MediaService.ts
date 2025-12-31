// media.service.ts
import { writable } from "svelte/store";
import { MediaAdapter } from "../services/media/MediaAdapter.class.svelte";
import type { Media } from "../services/tuari-command/tuari-command.types";

interface mediaStateProps {
  data: null | Media;
  isLoading: boolean;
  error: null | Error;
}

export const mediaState = writable<mediaStateProps>({
  data: null,
  isLoading: false,
  error: null,
});

export class MediaService {
  private adapter = new MediaAdapter();

  async fetchMedia(url: string) {
    mediaState.set({ data: null, isLoading: true, error: null });
    try {
      const response = await this.adapter.getMediaMetadata(url);
      if (!response) {
        const err = new Error("no data");
        mediaState.set({ data: null, isLoading: false, error: err });
        return null;
        // mediaState.set({
        //   data: null,
        //   isLoading: false,
        //   error: new Error("no data"),
        // });
      }

      mediaState.set({
        data: response!.results,
        isLoading: false,
        error: null,
      });
      return response!.results;
    } catch (e) {
      mediaState.set({ data: null, isLoading: false, error: e as Error });
      throw e;
    }
  }
}

export const mediaService = new MediaService();
