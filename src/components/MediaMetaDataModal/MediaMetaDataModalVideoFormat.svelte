<script lang="ts">
  let { formats } = $props();
  let selected = $state(0);
</script>

  <section>
    <h1>Video Format</h1>

    <div class="formats">
      <div class="format-row">
        <span>4K</span>
        <span>MP4 · AV1</span>
        <span>25 FPS</span>
        <span>5.6 GB</span>
      </div>

      <div class="format-row">
        <span>1080p</span>
        <span>MP4 · H.264</span>
        <span>30 FPS</span>
        <span>1.2 GB</span>
      </div>
    </div>
  </section>

<style>
  section {
    border: 1px solid white;

    > .formats {
      display: grid;
      gap: 0.25rem;

      .format-row {
        display: grid;
        grid-template-columns: 6ch 1fr 6ch 6ch;
        align-items: center;
        padding: 0.5rem 0.75rem;
        border-radius: 6px;

        > span {
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
          font-size: var(--fontSize-md);
        }

        &:hover {
          background: rgba(255, 255, 255, 0.05);
          cursor: pointer;
        }
      }
    }
  }
</style>
