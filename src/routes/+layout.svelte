<script>
	import Footer from "$lib/components/Footer.svelte";
import NavBar from "$lib/components/NavBar.svelte";
</script>

<div class="container">
		<NavBar/>
		<slot></slot>
		<footer>
				<Footer />
		</footer>
</div>

<style>
	:root {
		--bg-color: rgb(200,195,173);
		--bg-overlay: rgba(73,70,61, 0.3);
		--bg-overlay-second: rgba(211, 205, 181, 0.6);
		--bg-overlay-selected: rgba(73,70,61, 0.9);
		--fg-font: rgb(77,75,66);
		--fg-font-menu: rgb(113,109,96);
		--fg-font-selected: rgb(175,169,149);
		--overlay-shadow: rgba(150, 149, 128, 0.8);
		--element-color: 78,75,66;
	}
	@font-face {
		font-family: 'FOT-Rodin';
		src: url('/fonts/FOT-Rodin Pro DB.otf') format('opentype');
		font-weight: normal;
		font-style: normal;
	}

		:global(h1) {
				margin-top: 0;
				margin-left: 1em;
				letter-spacing: 0.7rem;
				text-shadow: .3rem .3rem 0 rgba(77, 73, 62,.3)
		} 

		:global(html), :global(body) {
				margin: 0;
				padding: 0;
				min-height: 100vh;
				width: 100%;
				max-width: 100%;
		}

	:global(body) {
		color: var(--fg-font);
		font-family: "FOT-Rodin";

		background-size: 4px 4px;
		background-image: linear-gradient(90deg, transparent, transparent 3px, rgba(0,0,0,.07) 4px),
    linear-gradient(transparent, transparent 3px, rgba(0,0,0,.07) 4px);
		box-shadow: inset 0 0 150px rgba(var(--element-color), .4), inset 0 0 150px rgba(var(--element-color), .3);
		letter-spacing: 0.1rem;
		display: flex;
		flex-direction: column;
	}

	:global(body::before) {
        content: '';
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: var(--bg-color);
        z-index: -2; /* Place it behind everything */
    }

    /* Add another pseudo-element for the grid pattern */
    :global(body::after) {
        content: '';
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-size: 4px 4px;
        background-image: linear-gradient(90deg, transparent, transparent 3px, rgba(0,0,0,.1) 4px),
            linear-gradient(transparent, transparent 3px, rgba(0,0,0,.1) 4px);
        z-index: -1; /* Place it above the background color but below content */
        pointer-events: none; /* Allow clicking through */
    }
	:global(#svelte) {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }

	:global(::selection) {
		background: var(--bg-overlay-selected);
		color: var(--fg-font-selected);
	}

	/*This style will apply to all the content in the app*/
	:global(.content) {
		padding-left: 3rem;
		margin-left: 1em;
		background-image: linear-gradient(90deg, rgba(var(--element-color), .2), rgba(var(--element-color), .2) 14px, 
		transparent 14px, transparent 20px,
		rgba(var(--element-color), .2) 20px, rgba(var(--element-color), .2) 23px, transparent 23px);
		margin-bottom: 2rem;
	}

	.container {
			min-height: 100vh;
			display: flex;
			flex-direction: column;
	}

	footer {
			margin-top: auto;
	}
</style>
