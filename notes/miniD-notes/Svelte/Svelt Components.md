## Props handling
```tsx 
<script lang="ts">
  interface MyComponentProps {
    title: string;
    count?: number;
  }

  // Type the props object
  let props = $props<MyComponentProps>();
</script>

<h1>{props.title}</h1>
<p>Count: {props.count ?? 0}</p>

```

## Snippets
`#snippet` lets you define a reusable piece of template markup that can be invoked like a function, with parameters, inside the same component (or passed to children).
`#snippet` is Svelte’s answer to “I want a function, but for markup, not behavior.”
It is:
- Not a component
- Not a slot
- Not a function
- But behaves like **a parameterized, reusable template block**
```jsx
{#snippet userRow(user)}
  <div class="row">
    <span>{user.name}</span>
    <span>{user.email}</span>
  </div>
{/snippet}

{#each users as user}
  {@render userRow(user)}
{/each}

// react like way of passing props
// `children` is typed as a function returning renderable content.
// `children?.()` ensures safe rendering when no children are passed.
// `{@render}` executes the snippet and inserts its DOM output.

<!-- Parent.svelte -->
<script lang="ts">
  import Card from './Card.svelte';
</script>

<Card>
  {#snippet children()}
    <h2>Title</h2>
    <p>This is content passed from the parent</p>
  {/snippet}
</Card>

<!-- Card.svelte -->
<script lang="ts">
  let { children } = $props<{ children?: () => unknown }>();
</script>

<div class="card">
  {@render children?.()}
</div>

```

## Context Api
## What Context Is (Definition)
> **Context is a way to pass values down the component tree without prop drilling.**  
> Any JavaScript value can be stored: primitives, objects, functions, classes, services, stores, or reactive state.
> **Svelte context is a scoped dependency injection system that passes live references across component layers without prop drilling.**

Context is:
- Scoped to a subtree
- Resolved at runtime
- Non-reactive by default (reactivity comes from what you put inside)

#### Scenario
We want:
- A **Root Provider**
- A **Middle Component**
- A **Deep Child**
- No props passed manually
- Shared state + behavior available everywhere
- Context passes **references**, not copies

Valid context values:
- `$state(...)`
- Plain objects
- Functions
- Classes / services
- API clients
- Config objects
- Async handlers
- Even other snippets
```tsx
// layer 1 - Context Provider (Root)
<!-- App.svelte -->
<script lang="ts">
  import { setContext } from 'svelte';
  import Layout from './Layout.svelte';

  const THEME_KEY = Symbol('theme');

  const theme = $state({
    mode: 'dark',
    toggle() {
      this.mode = this.mode === 'dark' ? 'light' : 'dark';
    }
  });

  setContext(THEME_KEY, theme);
</script>

<Layout />
// Notes!
//- `Symbol()` avoids key collisions
//- Context value can contain:
//    - state
//    - methods
//    - objects
// - Reactivity comes from `$state`, not from context itself

// Layer 2 - Middle Component (No Props Needed)
<!-- Layout.svelte -->
<script lang="ts">
  import Page from './Page.svelte';
</script>

<Page />
// Notes!
// This layer does nothing with context
// - Context flows through automatically
// - No boilerplate

// Layer 3 - Consumer
<!-- Page.svelte -->
<script lang="ts">
  import { getContext } from 'svelte';

  const THEME_KEY = Symbol.for('theme');
  const theme = getContext<{
    mode: string;
    toggle: () => void;
  }>(THEME_KEY);
</script>

<p>Current theme: {theme.mode}</p>
<button on:click={theme.toggle}>
  Toggle Theme
</button>

// Notes
// - `getContext()` retrieves the exact object set at the root
// - Mutating `theme.mode` updates **every consumer**
// - No props, no callbacks, no reducers


```
