<script lang="ts">
	import type { Snippet } from "svelte";

	let {children, onclick}: {children: Snippet, onclick: () => void} = $props()
</script>

<div class="button-container">
	<button onclick={onclick}>{@render children()}</button>
</div>

<style>
	button {
		border: none;
		background: linear-gradient(to left, var(--bg-overlay) 50%, var(--bg-overlay-selected) 50%) right;
		background-size: 200%;
		color: var(--fg-font);
		width: 10em;
		height: 3em;
		transition: ease-out 0.2s;
	}

	button:hover, button:focus {
		outline: none;
		color: var(--fg-font-selected);
		background-position: left;
	}

	.button-container {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.1em;
	}
	.button-container::before {
		content: "";
		display: inline-block;
		visibility: hidden;
		background-image: url("/selected.svg");
		width: 3em;
		height: 1.5em;
		background-size: contain;
		transform: translateX(30px);
		transition: 0.2s ease-out, visibility 0.1s 0s ease-out;
	}

	.button-container:hover::before, .button-container:focus::before {
		transform: translateX(0px);
		visibility: visible;
	}


</style>
