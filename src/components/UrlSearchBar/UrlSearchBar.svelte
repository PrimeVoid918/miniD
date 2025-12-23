<script lang="ts">
  import MediaMetaDataModal from "../MediaMetaDataModal/MediaMetaDataModal.svelte";
  import type { Media } from "../../lib/services/tuari-command/tuari-command.types";
  import { invoke } from "@tauri-apps/api/core";

  let media = $state<Media | undefined>();
  let isModalOpen = $state(false);
  function onCloseModal() {
    isModalOpen = !isModalOpen;
    console.log("pressed?, isModalOpen status: ", isModalOpen);
  }

  let url = $state("");

  async function onpaste(e: ClipboardEvent) {
    const pastedData = e.clipboardData?.getData("text") ?? "";

    // Simple URL validation
    try {
      // const parsed = new URL(pastedData);
      const data: any = await invoke("fetch_media", { url: pastedData });
      url = pastedData;
      media = data?.results;
      console.log("media data now: ", media);
      isModalOpen = true;
    } catch {
      console.log("Not a valid URL:", pastedData);
    }

    e.preventDefault(); // optional, prevents default paste if needed
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault(); // prevents form submission or default behavior
      console.log("Enter pressed! Current URL:", url);
    }
  }
</script>

<div class="UrlSearchBarContainer">
  <input
    type="text"
    placeholder="Paste URL here"
    bind:value={url}
    {onkeydown}
    {onpaste}
  />
  <MediaMetaDataModal {media} open={isModalOpen} onClose={onCloseModal} />
</div>

<style>
  .UrlSearchBarContainer {
    border: 1px solid white;
    padding: var(--spacing-sm);
    gap: var(--spacing-sm);
    height: 4rem;

    display: flex;
    flex-direction: row;
    align-items: center;

    > input {
      flex: 1;
      padding: var(--spacing-sm);
      height: 100%;

      background-color: var(--accent-l3);
    }

    > button {
      display: flex;
      flex-direction: row;
      justify-content: center;
      align-items: center;

      aspect-ratio: 1;
      height: 100%;
      background-color: var(--accent-l1);
      border: 1px solid white;
    }
  }
</style>
