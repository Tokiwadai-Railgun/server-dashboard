<script lang="ts">
	import { items } from '$lib/data/NavBar'
	import { page } from '$app/stores';
	import Seperation from './Seperation.svelte';

	$: tab = $page.url.pathname
</script>
<nav class="content">
	{#each items as item, count}
		<a 
			href={`${item.to}`} 
			tabindex={item.to == tab ? -1 : count + 1} 
			class={item.to == tab ? "current" : "item"}
		>
			<span>{item.title}</span>
		</a>
	{/each}
</nav>

<Seperation />

<style>
	nav {
		margin-top: 25px;
		margin-bottom: 25px;
		display: flex;
		justify-content: space-evenly;
		position: relative;
	}

	a {
		position: relative;
		color: var(--fg-font);
		text-decoration: none;
		padding: 0.2em 5em 0.2em 5em;
		background: linear-gradient(to left, var(--bg-overlay) 50%, var(--bg-overlay-selected) 50%) right;
		background-size: 200%;
		transition: .3s ease-out;
		height: 1.4em;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.item:hover, .item:focus {
		outline: none;
		background-position: left;
		color: var(--fg-font-selected);
	}

	.item::after, .item::before {
		content: '';
		left: 0;
		width: 100%;
		height: 2px;
		position: absolute;
		background-color: var(--bg-overlay-selected);
		transform: translateY(0);
		opacity: 0;
		transition: transform 0.3s ease-out, opacity 0s ease-out 0.2s;
	}

	.item::after {
		bottom: 0;
	}

	.item::before {
		top: 0;
	}

	.item:hover::after, .item:focus::after {
		transform: translateY(10px);
		opacity: 1;
		transition: transform 0.3s ease-out, opacity 0.1s ease-out 0s;
	}

	.item:hover::before, .item:focus::before {
		transform: translateY(-10px);
		opacity: 1;
		transition: transform 0.3s ease-out, opacity 0.1s ease-out 0s;
	}

	.current::after {
        content: '';
        position: absolute;
        left: 0;
        top: 0;
        width: 100%;
        /* This height will create the extension effect */
        height: calc(100% + 25px);
        /* Put it behind the content */
        z-index: -1;
        background-color: var(--bg-overlay-selected);
        /* Add transition for smooth height change */
        transition: height 0.3s ease-out;
    }

	.current {
		color: var(--fg-font-selected);
	}

</style>
