<script lang="ts">
  import MediaMetaDataModal from "../MediaMetaDataModal/MediaMetaDataModal.svelte";
  import type { Media } from "../../lib/services/tuari-command/tuari-command.types";
  import { mediaState, mediaService } from "../../lib/stores/MediaService";

  let url = $state("");

  // let dataFromUrl = $state<any>();
  // const { fetchMedia } = mediaService;
  let data = $state<Media>();
  let isLoading = $state();
  let error = $state();

  let isModalOpen = $state(false);
  function onCloseModal() {
    isModalOpen = !isModalOpen;
    console.log("pressed?, isModalOpen status: ", isModalOpen);
  }

  async function onpaste(e: ClipboardEvent) {
    const pastedData = e.clipboardData?.getData("text") ?? "";

    // Simple URL validation
    e.preventDefault();
    const pastedUrl = e.clipboardData?.getData("text") ?? "";
    data = await mediaService.fetchMedia(pastedUrl); // triggers updates in mediaState
    isModalOpen = true;

    console.log("data from url: ", data);
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
  {#if data}
    <MediaMetaDataModal
      media={data}
      onClose={onCloseModal}
      open={isModalOpen}
    />
  {/if}
  <!-- {#if isLoading}
    <p>Loading...</p>
  {:else if error}
    <p>Error: {error.message}</p>
  {:else if data}
    <MediaMetaDataModal
      media={dataFromUrl}
      onClose={onCloseModal}
      open={isModalOpen}
    />
  {/if} -->
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
