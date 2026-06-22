## Conditional Rendering 
```jsx
<script>
	let status = $status('loading')
</script>

{#if status === 'loading'}
	<p>Loading ...</p>
{:else if status === 'sucess'}
	<p>Sucess ...</p>
{:else if status === 'error'}
	<p>Error ...</p>
{:else}
	<p>Impossible state ...</p>
{/if} // for closing 
```

## Looping Data In Svelt
```jsx
<script lang="ts">
	let list = $state([
		{ id: 1, listName: "Eating Apple", isDone: false },
		{ id: 2, listName: "We do hiking", isDone: true },
		{ id: 3, listName: "Kicthen Clean", isDone: false },
		{ id: 4, listName: "Play Skyrim", isDone: true },
		{ id: 5, listName: "Code in rust", isDone: false },
	]);
</script>

<div style="display: flex; flex-direction: column; gap: 1rem">
	<!-- // can do destructing -->
	{#each list as { id, isDone, listName }, index (id)}
		<div style="border: 2px solid white; padding: 1rem">
			<p color="white">Todo Name: {listName}</p>
			<p color="white">Status: {isDone}</p>
		</div>
	{/each}
</div>
```
## Local Constants
```jsx
<script lang="ts">
let data = $state([
	{
		id: 1,
		firstname: "John",
		lastname: "Deo",
	},
	{
		id: 2,
		firstname: "John",
		lastname: "Cena",
	},
	{
		id: 3,
		firstname: "Calcite",
		lastname: "Serum",
	},
	{
		id: 4,
		firstname: "Winter",
		lastname: "Summer",
	},
]);

</script>

<div style="display: flex; flex-direction: column; gap: 1rem">
	{#each data as users (users)}
		{@const { id, firstname, lastname } = users}
		{@const fullname = firstname + " " + lastname} // precomputed
		<div style="border: 2px solid white; padding: 1rem">
			<p style="color: white">ID: {id}</p>
			<p style="color: white">Fullname: {fullname}</p>
		</div>
	{/each}
</div>

```
## bindings
```tsx
<script lang="ts">
	// reactive state variable
	let name: string = "";
	let canvas = undefined
</script>

<h1>Hello, {name || "stranger"}!</h1>
<!-- Two-way binding with bind:value -->
<input type="text" bind:value={name} placeholder="Enter your name" />
<p>Your name has {name.length} characters.</p>

<canvas bind:this={canvas}></canvas>

// function bininds
<textarea 
	bind:value={
		get, // callback exe () => someFunction(d)
		set(data) // callbacks exe. (d) => { data = someFunction(d) }
	}
></textarea>
```

## directives
```tsx
// @attach 
// `@attach` binds a function to a DOM element’s lifecycle
// It runs:
//  - when the element is created
//  - when reactive inputs change
//  - when the element is destroyed (cleanup)
// This makes it ideal for:
//  - GSAP animations
//  - observers
//  - imperative DOM libraries
//  - measurements
//  - subscriptions
<div
	class="box"
	{@attach ()=>{   // can also take functions
		// some functions
	}}
>
</div>
```