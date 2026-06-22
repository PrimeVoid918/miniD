## component markups
```tsx
// logic
<script lang="ts">
	// can import css
	import 'app.css' 
</script>

// html
<body>
</body>

// styles
// anything is scoped within the component 
<styles> 
body{
	backgroundColor: red
}
</styles>
```

## making some dynamic values into the markup
```tsx
// logic
<script lang="ts">
let color = "green"
</script>

// html
<h1 style="color: {color}">Hello</h1>
<h1 style:color={color} >Hello</h1> // shortcut
<h1 style:color >Hello</h1> // shortcut in omitted version

<h1 style="--color: {color}">Hello</h1> // uses css variable

<style>
	h1{
		color: var(--color, white) // if not passed defaults to white
	}
</style>
```

## render raw html
```jsx
<script>
	let content = `<h1>Samople Element</h1>`;
</script>

<hgroup>
	{@html content}
<hgroup>

// svelt cant detect h1 so it throws warnings, this resolved it
<style>
	// this however can populate your global style resulting to clashing
	//:global{
		h1{
			text-transform: capitalized;
		}
	//}
	
	// this solves it by globally scoping the styles
	hgroup :global{
		h1{
			text-transform: capitalized;
		}
	}
</style>
```

## Dynamic classes
``` jsx

<script>
	let status = $state('closed');
</script>

<button class="btn"> 
	<span class="trigger" data-status={status}> O </span>
</button>

<style>
	.trigger{
		transition: rotate 0.2 ease;
		
		&.[data-status='open']{
			rotate: 0deg
		}
		
		&.[data-status="close"]{
			rotate: -90deg;
		}
	}

</style>
```

## svelte runes
``` typescript
// state rune
$state<Type>(<value>) // it do inference
$state({}) || $state([]) // it becomes a deeply react state proxy, 
$state.raw() // updates the value only when reassiend, example raw = {...raw}

let { val1, val2 } = $state({}) // can do destructuring if it is an object

// derived rune    ... or in other framworks, computed values
// Observer that computes state
// defines a reactive, read-only computed value whose value is determined solely by other reactive sources and is automatically recomputed when those sources change.
// The function/expression passed to `$derived` must be free of side effects.
// It computes a value; it does not perform actions.
// Any `$state`, `$derived`, or `$prop` read inside the expression becomes a dependency.
// When any dependency changes, the derived value is invalidated and recomputed.
// A `$derived` value cannot be assigned to.
// `$derived` returns _exactly_ the result of the expression. 

let a = $state(0);
let b = $state(0);
let result = $derived(a * b);

let onlyTrueIfMeetsCondition = $derived(result <= 10); // `$derived` wraps **whatever** the expression returns
	
	// return a derived value through an operation
	let cart = $state([
		{ item: "apple", total: 12 },
		{ item: "mango", total: 15 },
	])
	let totalItems = $derived.by(()=>{ / takes a function
		let sum = 0;
		for ( let item of cart ){
			sum += item.total
		}
		
		return sum
	})

// effect rune  --> Implicit Observer via dependency graph in GoF design patterns
// `$effect` registers a reactive side-effect that automatically re-runs whenever any reactive value it reads changes.
// Every `$state`, `$derived`, or `$prop` _read_ inside the effect becomes a dependency.
// When any dependency changes, the effect re-runs.
// If the effect returns a function, it is treated as a cleanup. Cleanup runs before re-execution and on teardown.

// avoid using effect to synchronize states because $effect runs last and results to out of sync
let count = $state(0)
let condition = $state(false)

$effect(()=>{ // takes a function
	// left hand side has mutation   =   right hand side subject
	//          sideEffect           =          subject 

	if(condition){ // always runs upon true
		console.log(count); // connnot be logged if the condition is not true
	}
	
	// console.log(untracked(()=> count))
	//`untracked(fn)` executes `fn` without registering any reactive dependencies for the currently running reactive context.
	// it is $effect exclusive only
})

	// A variant of `$effect` that runs **before the DOM updates**.
	// Reading reactive state **before the browser paints**
	// Behavior:
		// Runs **synchronously** before normal `$effect`
		// Still tracks dependencies automatically
		
	$effect.pre(() => {
	  console.log("pre-DOM update value:", $count);
	  
	  // Returns a Promise that resolves after the DOM updates.
		  // Wait for reactive changes to propagate to the DOM
		  // Useful for measuring layout, scrolling, focusing, or manipulating elements after render
	  // tick().then(()=>{})
	  
	  // Native JavaScript function that queues a callback to run after the current
	  // Deferring code that should run immediately after synchronous updates, but before browser paints
	  // Useful for measuring DOM updates, synchronizing state changes, or avoiding infinite loops in reactive code
	  // queueMicrotask(() => { /* code */ });
	});

// inspect rune
$inspect(runeVal) // logs when reactive value changes
```

Patterns
```jsx
<script>
function createCounter(initial){
	let counter = $state({count: initial})
	return counter
}

let counter = createCounter(0)
</script>

<button onclick={() => counter.count++}>
	{counter.count}
</button>
```
``` tsx 
// class based way
<script>
import { Counter } from "./counter.svelt.ts"

let counter = new Counter(0)
</script>

<button onclick={() => counter.count++}>
	{counter.count}
</button>

// couter.svelt.ts
export class Counter{
	constructor(initial){
		this.count = $state(initial)
	}
}
```

