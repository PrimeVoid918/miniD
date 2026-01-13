import type { Snippet } from "svelte";

class OverlayService {
  // Internal State
  isVisible = $state(false);
  content = $state<Snippet | null>(null);
  isBlocking = $state(true); // If true, prevents closing on backdrop click

  show(contentSnippet: Snippet, blocking = true) {
    this.content = contentSnippet;
    this.isBlocking = blocking;
    this.isVisible = true;
  }

  hide() {
    this.isVisible = false;
    this.content = null;
  }
}

// The Singleton instance
export const overlayService = new OverlayService();

/*
// ui-actions.ts
// import { overlayService } from "./overlay.service.svelte";
// 
// export const uiActions = {
  // /**
  //  * Prevents all user interaction during a heavy task (like yt-dlp downloading)
  //  */
// startGlobalLock: (messageSnippet: Snippet) => {
// overlayService.show(messageSnippet, true);
// },
//
// stopGlobalLock: () => {
// overlayService.hide();
// }
// }
