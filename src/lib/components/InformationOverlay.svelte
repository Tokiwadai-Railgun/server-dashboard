
<script lang="ts">
	import type { Snippet } from "svelte";

	let {title, content}: {title: string, content: Snippet} = $props()
</script>

<div class="data-overlay">
		<h2 class="overlay-title">{title}</h2>
		{@render content()}
</div>

<style>
		.data-overlay {
				width: 60vw;
				margin-right: 4em;
				background: var(--bg-overlay-second);
		}

		.overlay-title {
			background: var(--bg-overlay);
			color: var(--text-overlay);
			padding: 0.5em;
			margin: 0;
			font-size: 1.5em;
			font-size: 1em;
			display: flex;
			align-items: center;
			gap: 0.5em;
		}

		.overlay-title::before {
			content: "";
			width: 1em;
			height: 1em;
			background: var(--square-color, var(--bg-overlay));
			flex-shrink: 0;
		}
</style>
