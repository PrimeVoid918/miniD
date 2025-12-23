<script lang="ts">
  import Icon from "@iconify/svelte";

  interface PropsI {
    open: boolean;
    onClose: () => void;
  }

  let { open, onClose }: PropsI = $props();
</script>

<button class="close-button" onclick={() => onClose()}>
  <Icon icon="material-symbols:close" font-size="2rem" color={"white"} />
</button>

<style>
  .close-button {
    /* border: 1px solid white; */
    /* position: absolute; */

    /* right: calc(0px + var(--spacing-md));
    top: calc(0px + var(--spacing-md)); */

    cursor: pointer;

    &:hover {
      background-color: var(--primary-d6);
    }

    &:active {
      background-color: var(--primary-l6);
    }
  }
</style>
