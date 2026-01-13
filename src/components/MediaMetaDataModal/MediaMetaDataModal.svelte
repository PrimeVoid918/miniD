<script lang="ts">
  import Icon from "@iconify/svelte";
  import MediaMetaDataModalCloseButton from "./MediaMetaDataModalCloseButton.svelte";
  import MediaMetaDataModalVideoFormat from "./MediaMetaDataModalVideoFormat.svelte";
  import type { MediaMetaDataModalProps } from "./MediaMetaDataModal.types";
  import ButtonGeneric from "../Buttons/AsyncButton.svelte";

  //! get the close button color from the custom css color for dynamic purposes later!

  let { open, media, onClose }: MediaMetaDataModalProps = $props();
  // let mediaState = $derived(media);

  // reactively track changes
  $effect(() => {
    // mediaState = media;
    // console.log("media passed to modal:", media);
  });

  // Close on Escape key
  function handleKeydown(e: KeyboardEvent) {
    if (open && e.key === "Escape") {
      onClose();
    }
  }

  // Close on backdrop click (but not if clicking inside the modal content)
  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }

  let dir_location = $state("/sample/downloads");

  // async function handleDownload() {}
  let selectedId = 3;
  async function handleDownload() {
    if (!selectedId) return false;

    try {
      // Call your Rust backend here
      // await invoke("download", { id: selectedId });
      return true; // The button will now show "Success" state
    } catch (e) {
      return false; // The button knows it failed
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if open}
  {#if media?.formats}
    <section class="overlay">
      <div class="container">
        <div class="header">
          <h1>Download Video</h1>
          <MediaMetaDataModalCloseButton {open} {onClose} />
        </div>
        <div class="contents">
          <div>
            <img src={media?.thumbnail} alt="" />
            <div>
              <h3>{media?.title}</h3>
              <p>{media?.description}</p>
            </div>
          </div>
          <MediaMetaDataModalVideoFormat formats={media?.formats} />
          <div class="download-section">
            <!-- <ButtonGeneric onClick={handleDownload}
              ><h2>Download</h2></ButtonGeneric
            > -->
            <div>
              <input type="text" bind:value={dir_location} />
            </div>
          </div>
        </div>
      </div>
    </section>
  {/if}
{/if}

<style>
  .overlay {
    position: absolute;
    width: 100%;
    /* width: 100dvw; */
    height: 100dvh;

    background-color: #ffffff50;
    backdrop-filter: blur(10px);

    top: 0;
    left: 0;

    display: flex;
    justify-content: center;
    align-items: center;
    padding: var(--spacing-lg);

    > .container {
      background-color: var(--primary-d8);
      border-radius: var(--borderRadius-xl);
      border: 1px solid greenyellow;

      display: flex;
      flex-direction: column;
      max-height: 90%;
      height: 90%;
      width: clamp(300px, 90vw, 1000px);
      overflow-y: hidden;

      > .header {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;

        padding: var(--spacing-md) var(--spacing-lg);

        border-bottom: 3px solid var(--primary-l8);

        > h1 {
          font-size: var(--fontSize-xl);
        }
      }

      > .contents {
        padding: var(--spacing-lg);
        font-size: clamp(1rem, 2.5vw, 1.5rem);
        flex: 1;
        display: flex;
        min-height: 0;
        flex-direction: column;

        > div {
          display: flex;
          flex-direction: row;
          border: 1px solid var(--primary-l8);
          > img {
            height: 100px;
            aspect-ratio: 16/9;
          }
          > div {
            padding: var(--spacing-md);
            > p {
              font-size: var(--fontSize-sm);
            }
          }
        }

        > .download-section {
          margin-top: auto;

          > div {
            > input {
              font-size: var(--fontSize-sm);
              color: var(--text-d2);
            }
          }
        }
      }
    }
  }
</style>
