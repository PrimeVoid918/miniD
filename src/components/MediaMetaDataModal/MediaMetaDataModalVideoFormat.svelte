<script lang="ts">
  import type {
    VideoFormat,
    AudioFormat,
    Media,
  } from "../../lib/services/tuari-command/tuari-command.types";
  import { mediaDisplayHelperService } from "../../lib/services/media/MediaDisplayHelper.class.svelte";

  let { formats }: { formats: Array<VideoFormat | AudioFormat> } = $props();

  let videos: VideoFormat[] = $derived(
    (formats.filter((f) => f.kind?.toLowerCase() === "video") ??
      []) as VideoFormat[]
  );
  let audios: AudioFormat[] = $derived(
    (formats.filter((f) => f.kind?.toLowerCase() === "audio") ??
      []) as AudioFormat[]
  );

  $effect(() => {
    // console.log("videos: ", formats);
    // console.log("videos: ", formatsProps);
  });

  let selectedId = $state(0);
</script>

<section>
  <h1>Video Format ay</h1>

  {#each videos as video, i}
    {#if video.ext !== "mhtml"}
      <div class="formats">
        <div class="format-row">
          <span>{video.width}</span>
          <span>{video.ext}</span>
          <span>{video.fps} FPS</span>
          <span
            >{mediaDisplayHelperService.getSizeConverter(video.filesize)}</span
          >
        </div>
      </div>
    {/if}
  {/each}

  <h1>Audio Format ay</h1>
  {#each audios as audio, i}
    <div class="formats">
      <div class="format-row">
        <span>{audio.tbr}</span>
        <span>{audio.ext}</span>
        <span>{mediaDisplayHelperService.getAudioQualityLabel(audio.tbr)}</span>
        <span>{mediaDisplayHelperService.getSizeConverter(audio.filesize)}</span
        >
      </div>
    </div>
  {/each}

  <!-- <div class="formats">
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
  </div> -->
</section>

<style>
  section {
    border: 1px solid white;

    > .formats {
      display: grid;
      gap: 0.25rem;

      > .format-row {
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
