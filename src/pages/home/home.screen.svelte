<script lang="ts">
  import DownloadedMediaTray from "../../components/DownloadedMediaTray/DownloadedMediaTray.svelte";
  import MediaMetaDataModal from "../../components/MediaMetaDataModal/MediaMetaDataModal.svelte";
  import UrlSearchBar from "../../components/UrlSearchBar/UrlSearchBar.svelte";
  import type { Media } from "../../lib/services/tuari-command/tuari-command.types";

  let media = $state<Media | undefined>();

  let isModalOpen = $state(false);

  function onCloseModal() {
    isModalOpen = !isModalOpen;
    console.log("pressed?, isModalOpen status: ", isModalOpen);
  }

  async function getJsonMediaData() {
    const res = await fetch("../../../notes/video-metadata.json");
    media = await res.json();

    for (const format of media?.formats ?? []) {
      if (format.video_ext === "mp4") {
        // format.width;
        // format.height;
        console.log("format mp4: ", format);
      }
      if (format.kind === "audio") {
        // audio-only
      }
    }

    // console.log("media data: ", media);
    // console.log("formats data: ", media?.formats);
  }

  getJsonMediaData();

  function simulateOpenModal() {
    isModalOpen = true;
  }
</script>

<main>
  <button
    style="backgroung-color: gray; border: 1px solid white"
    onclick={simulateOpenModal}>Simulate open Modal</button
  >
  <UrlSearchBar />
  <MediaMetaDataModal {media} open={isModalOpen} onClose={onCloseModal} />
  <DownloadedMediaTray />
  <h1>2</h1>
</main>

<style>
  main {
    color: white;
    /* width: 100dvw; */
    height: 100dvh;
    /* padding: var(--spacing-xs); */
    padding: var(--spacing-lg);
    /* border: 10px solid white; */
    position: relative;

    > div {
      font-size: 3rem;

      > button {
        border: 1px solid white;
        color: white;
      }
    }
  }
</style>
