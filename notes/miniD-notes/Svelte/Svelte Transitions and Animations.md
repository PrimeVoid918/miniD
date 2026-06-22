animations and transitions are local by default
to go global use add `|global`
**usage** `in:fade|global=`
## Built-in Transitions
**Intro / Outro transitions**
- `fade`
- `blur`
- `fly`
- `slide`
- `scale`
- `draw`
- `crossfade`
## Transition Directives
- `in:`
- `out:`
- `transition:`
- `animate:`
## Built-in Animations
**Used with `animate:`**
- `flip`
## Built-in Easing Functions
- `linear`
- `easeIn`
- `easeOut`
- `easeInOut`
- `backIn`
- `backOut`
- `backInOut`
- `bounceIn`
- `bounceOut`
- `bounceInOut`
- `circIn`
- `circOut`
- `circInOut`
- `cubicIn`
- `cubicOut`
- `cubicInOut`
- `elasticIn`
- `elasticOut`
- `elasticInOut`
- `expoIn`
- `expoOut`
- `expoInOut`
- `quadIn`
- `quadOut`
- `quadInOut`
- `quartIn`
- `quartOut`
- `quartInOut`
- `quintIn`
- `quintOut`
- `quintInOut`
- `sineIn`
- `sineOut`
- `sineInOut`
## Motion Utilities
- `tweened`
- `spring`
## Low-Level Animation API

- `tick`
- `loop`
- `easing` (module)

``` jsx
out:fade
out:face={{ duration: 200 }}
in:fade
in:fade={{ duration: 200}}

// both for intro and outro
transition:fade
transition:fade={{ duration: 400 }}

animate:flip
animate:flip={{ duration:400 }}

// special cases
import { crossfade } from 'svelte/transition'

const [send, receive] = crossfade({
	duration: 300
})
<div out:send={{ key: var }}></div>
<div in:receive={{ key: var }}></div>

// Easing Functions — Usage
import { cubicInOut } from 'svelte/easing';
transition:fade={{ duration: 300, easing: cubicInOut }}

// Motion Stores
	// tweened
	import { tweened } from 'svelte/motion';
	const x = tweened(0, { duration: 300 });
	{$x}
	// spring
	import { spring } from 'svelte/motion';
	const y = spring(0, { stiffness: 0.1, damping: 0.2 });

// Manual Tick (DOM update timing)
import { tick } from 'svelte';
await tick();

// Loop (low-level animation)
import { loop } from 'svelte/internal';

loop((time) => {
  // return false to stop
});

// Conditional Transition Trigger
{#if visible}
  <div transition:fade />
{/if}

```