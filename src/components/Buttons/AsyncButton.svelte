<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
    onClick: () => Promise<boolean> | boolean; // Returns a bool to the parent
    variant?: "primary" | "danger";
    className?: string;
  }

  let {
    children,
    onClick,
    variant = "primary",
    className = "",
  }: Props = $props();

  // Internal state
  let isLoading = $state(false);
  let isSuccess = $state(false);

  async function handleClick() {
    if (isLoading) return;

    isLoading = true;
    isSuccess = false; // Reset success state if they click again
    try {
      const result = await onClick();
      isSuccess = result;

      // Optional: Reset success look after 3 seconds
      if (isSuccess) {
        setTimeout(() => {
          isSuccess = false;
        }, 3000);
      }
    } catch (e) {
      isSuccess = false;
    } finally {
      isLoading = false;
    }
  }
</script>

<button
  class="btn {variant} {className}"
  class:loading={isLoading}
  class:success={isSuccess}
  onclick={handleClick}
  disabled={isLoading}
>
  {@render children()}
</button>

<style>
  .btn {
    padding: 0.5rem 1rem;
    font-family: "Iosevka", monospace;
    border: 1px solid var(--primary-l8);
    transition: all 0.2s;
    cursor: pointer;
  }

  .primary {
    background: var(--primary-d2);
  }
  .danger {
    background: #7f1d1d;
  }

  .loading {
    opacity: 0.5;
    cursor: wait;
  }
  .success {
    border-color: #22c55e;
  } /* Green border if return was true */
</style>
