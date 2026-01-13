<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
    onClick: () => void;
    className?: string;
  }

  let { children, onClick, className }: Props = $props();
</script>

<button class={className} onclick={onClick}>
  {@render children()}
</button>