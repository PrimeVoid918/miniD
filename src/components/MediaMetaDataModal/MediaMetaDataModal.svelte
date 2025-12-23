<script lang="ts">
  import Icon from "@iconify/svelte";
  import MediaMetaDataModalCloseButton from "./MediaMetaDataModalCloseButton.svelte";
  import MediaMetaDataModalVideoFormat from "./MediaMetaDataModalVideoFormat.svelte";
  import type { MediaMetaDataModalProps } from "./MediaMetaDataModal.types";

  //! get the close button color from the custom css color for dynamic purposes later!

  let { open, media, onClose }: MediaMetaDataModalProps = $props();

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
</script>

<svelte:window on:keydown={handleKeydown} />

{#if open}
  <section class="overlay">
    <div class="container">
      <div class="header">
        <h1>Download Video</h1>
        <MediaMetaDataModalCloseButton {open} {onClose} />
      </div>
      <div class="contents">
        <h1>{media?.title}</h1>
        <h1>{media?.description}</h1>

        <img src={media?.thumbnail} alt="" />
        <!-- <MediaMetaDataModalVideoFormat formats={} /> -->
      </div>
    </div>
  </section>
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
    padding: var(--spacing-lg);

    > .container {
      background-color: var(--primary-d8);

      border-radius: var(--borderRadius-xl);
      width: clamp(300px, 90vw, 1000px);
      max-height: 100%;
      overflow-y: auto;

      border: 1px solid greenyellow;

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
        /* Fluid padding or font-size example */
        padding: var(--spacing-lg);
        /* padding: clamp(1rem, 5vw, 3rem); */
        font-size: clamp(1rem, 2.5vw, 1.5rem);
      }
    }
  }
</style>
