<script>
  import Icon from "@iconify/svelte";
  import thumbnail from "../../assets/images/vlcsnap-2025-11-19-17h38m07s923.png";

  let data = $state([
    {
      id: 1,
      image: thumbnail,
      title: "The bronze age",
      duration: 30,
      size: "123mb",
      type: "mp4",
    },
    {
      id: 2,
      image: thumbnail,
      title: "Svelt",
      duration: 120,
      size: "229mb",
      type: "mp3",
    },
    {
      id: 3,
      image: thumbnail,
      title: "I have never Svelt That!",
      duration: 40,
      size: "228mb",
      type: "mp3",
    },
    {
      id: 4,
      image: thumbnail,
      title: "Arch is archgint the linux",
      duration: 60,
      size: "227mb",
      type: "mp4",
    },
    {
      id: 5,
      image: thumbnail,
      title: "Magnet",
      duration: 23130,
      size: "326mb",
      type: "mp4",
    },
    {
      id: 6,
      image: thumbnail,
      title: "Malice",
      duration: 2330,
      size: "225mb",
      type: "mp4",
    },
    {
      id: 7,
      image: thumbnail,
      title: "Megan Stallon",
      duration: 330,
      size: "224mb",
      type: "mp3",
    },
    {
      id: 8,
      image: thumbnail,
      title: "Mourning",
      duration: 4230,
      size: "822mb",
      type: "mp4",
    },
    {
      id: 9,
      image: thumbnail,
      title: "Moth",
      duration: 130,
      size: "823mb",
      type: "mp3",
    },
  ]);
</script>

<section>
  <div class="filters">
    <div>
      Date
      <Icon icon="material-symbols:arrow-upward" />
    </div>
    <div>
      Size
      <Icon icon="material-symbols:arrow-upward" />
    </div>
    <div>
      Duration
      <Icon icon="material-symbols:arrow-upward" />
    </div>
  </div>
  <div class="tray">
    {#each data as item (item)}
      {@const { duration, id, image, size, title, type } = item}
      <div class="tray-items">
        <img src={image} alt={title} />
        <div>
          <h3>{title}</h3>
          duration: {duration}
          size: {size}
          type:{type}
        </div>
      </div>
    {/each}
  </div>
</section>

<style>
  section {
    border: 1px solid yellow;
    padding: var(--spacing-md);

    > .filters {
      display: flex;
      flex-direction: row;
      gap: var(--spacing-md);

      > div {
        border: 1px solid white;
        padding: var(--spacing-sm) var(--spacing-md);
      }
    }

    > .tray {
      border: 1px solid green;
      display: flex;
      flex-direction: column;
      gap: var(--spacing-md);

      > .tray-items {
        display: flex;
        flex-direction: row;
        gap: var(--spacing-md);
        padding: var(--spacing-md);
        border: 1px solid blue;

        > img {
          height: 100px;
          aspect-ratio: 16/9;
        }
      }
    }
  }
</style>
