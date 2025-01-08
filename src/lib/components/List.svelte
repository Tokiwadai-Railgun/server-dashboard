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
		<div class="side-bar-container">
				<div class="side-bar"></div>
		</div>
</div>

<style>
		.list-container {
				position: relative;
				padding: 0.5em;
				height: 53vh;
				width: 25vw;
				display: flex;
				justify-content: space-between;
				background-color: var(--bg-overlay-second);
				box-shadow: 4px 4px 0px var(--overlay-shadow);
		}

		.list {
				padding: 0.5em 0em 0.5em 0;
				width: 95%;
				display: flex;
				flex-direction: column;
				gap: 0.4em;
				overflow-y: scroll;        
				border-top: 0.1em solid var(--bg-overlay);
				border-bottom: 0.1em solid var(--bg-overlay);
		}

		.side-bar-container {
				position: relative;
				height: calc(100% + 0.2em);
				top: -0.1em;
		}

		.side-bar {
				width: 0.2em;
				height: calc(100% - 2em);
				background: var(--bg-overlay);
				margin: 1em 0em 1em 0;
				position: relative;
		}

		.side-bar-container::before, .side-bar-container::after {
				content: "";
				background: var(--bg-overlay);
				width: 0.3em;
				height: 0.3em;
				border-radius: 50%;
				position: absolute;
				left: 50%;
				transform: translateX(-50%);
		}

		.side-bar-container::before {
				top: 0;
		}

		.side-bar-container::after {
				bottom: 0;
		}

		::-webkit-scrollbar-track {
				background: transparent; /* Track background */
		}
</style>
