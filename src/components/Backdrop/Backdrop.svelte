<script lang="ts">
  import { fade } from "svelte/transition";
  import { overlayService } from "../../lib/services/ui";
  // Use Svelte 5 snippets to render dynamic content
</script>

{#if overlayService.isVisible}
  <div class="backdrop" transition:fade={{ duration: 200 }}>
    {@render overlayService.content?.()}
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 1000;
    display: flex;
    justify-content: center;
    align-items: center;
    /* CSS logic here */
  }
</style>
