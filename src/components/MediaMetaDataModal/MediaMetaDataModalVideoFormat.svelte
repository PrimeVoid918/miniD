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
    console.log("videos: ", videos);
    // console.log("videos: ", formatsProps);
  });

  let selectedFormatId = $state<string | null>("");

  function handleSelectedId(id: string) {
    selectedFormatId = id;
    console.log("selected id: ", id);
  }
</script>

<section>
  <div>
    <h1>Video Format ay</h1>

    {#each videos as video, i}
      {#if video.ext !== "mhtml" && video.filesize > 0}
        <div class="formats">
          <button
            class="format-row"
            class:selected={selectedFormatId === video.format_id.toString()}
            onclick={(e) => handleSelectedId(video.format_id.toString())}
          >
            <span>{video.width}x{video.height}</span>
            <span>{video.ext}</span>
            <span>{video.fps} FPS</span>
            <span
              >{mediaDisplayHelperService.getSizeConverter(
                video.filesize
              )}</span
            >
          </button>
        </div>
      {/if}
    {/each}

    <h1>Audio Format ay</h1>
    {#each audios as audio, i}
      <div class="formats">
        <button
          class="format-row"
          class:selected={selectedFormatId === audio.format_id.toString()}
          onclick={(e) => handleSelectedId(audio.format_id.toString())}
        >
          <span>{audio.tbr}</span>
          <span>{audio.ext}</span>
          <span
            >{mediaDisplayHelperService.getAudioQualityLabel(audio.tbr)}</span
          >
          <span
            >{mediaDisplayHelperService.getSizeConverter(audio.filesize)}</span
          >
        </button>
      </div>
    {/each}
  </div>
</section>

<style>
  section {
    border: 1px solid white;

    display: flex;
    flex-direction: column;
    justify-content: center;
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    > div {
      overflow-y: auto; /* This is where the scrollbar should live */
      padding: 0.5rem;

      > .formats {
        display: grid;
        gap: 0.25rem;
        color: var(--text-l1);
        width: 100%;

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
            color: var(--text-l2);
          }

          &:hover {
            background: rgba(255, 255, 255, 0.05);
            cursor: pointer;
          }

          &.selected {
            border-color: #fbbf24; /* yellow-400 */
            background: rgba(251, 191, 36, 0.1);
          }
        }
      }
    }
  }
</style>
