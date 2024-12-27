<script lang='ts'>
	import type { Snippet } from 'svelte';

		let { elements, body, selection = $bindable('')}: {elements: any[], body: Snippet<[any]>, selection: any} = $props()
</script>

<div class="list">
		{#each elements as element}
				{@render body(element)}
		{/each}
</div>

<style>
		.list {
				padding: 0.5em;
				height: 60vh;
				width: 25vw;
				background-color: var(--bg-overlay-second);
				overflow-y: auto;
		}
</style>
