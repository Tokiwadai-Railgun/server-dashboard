<script lang='ts'>
	import type { Snippet } from 'svelte';

		let { elements, body, selection = $bindable('')}: {elements: any[], body: Snippet<[any]>, selection: any} = $props()
</script>

<div class="list-container">
		<div class="list">
				{#each elements as element, count}
						{@render body({element, count})}
				{/each}
		</div>
</div>

<style>
		.list-container {
				position: relative;
				padding: 0.5em;
				height: 60vh;
				width: 25vw;
				display: flex;
				justify-content: space-between;
				background-color: var(--bg-overlay-second);
				overflow-y: auto;
				box-shadow: 4px 4px 0px var(--overlay-shadow);
		}

		.list {
				padding: 0.5em 0em 0.5em 0;
				width: 95%;
				display: flex;
				flex-direction: column;
				gap: 0.4em;
				border-top: 0.1em solid var(--bg-overlay);
				border-bottom: 0.1em solid var(--bg-overlay);
		}

		.list-container::after {
				content:"";
				width: 0.2em;
				height: 100%;
				background: var(--bg-overlay);
		}
</style>
