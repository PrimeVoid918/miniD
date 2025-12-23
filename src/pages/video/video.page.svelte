<script lang="ts">
  import mv from "../assets/LE SSERAFIM(르세라핌) 'SPAGHETTI' (4K) ｜ STUDIO CHOOM ORIGINAL [WAUSsfU00L0].webm";

  let currentTime = $state(0);
  let duration = $state(0);
  let paused = $state(false);

  let htmlVideo: HTMLVideoElement;

  $effect(() => {
    currentTime = htmlVideo.currentTime ?? 0;
  });
</script>

<!-- svelte-ignore a11y_media_has_caption -->
<div class="container">
  <video
    src={mv}
    bind:this={htmlVideo}
    bind:currentTime
    bind:duration
    bind:paused
  ></video>
  <div>
    <span>current time: {currentTime}/{duration}</span>
    <div>
      <button
        onclick={() => {
          paused = !paused;
        }}>{paused ? "play" : "pause"}</button
      >
      <input
        type="range"
        max={duration}
        bind:value={currentTime}
        name=""
        id=""
        step={0.1}
      />
    </div>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100dvh;

    > video {
      aspect-ratio: 16/9;
      width: 90%;
    }

    > div {
      display: flex;
      flex-direction: column;

      > div {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
      }
    }
  }
</style>
