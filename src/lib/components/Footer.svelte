<script lang="ts">
		import Seperation from "./Seperation.svelte";
		let {commands}: {commands: {name: string, action: any, key: string}[]} = $props();
</script>


<footer>
	<div class="footer-container">
		<div class="commands-container">
			{#each commands as command }
				<div class="command">
						<span class="key">{command.key}</span>
						<p>{command.name}</p>
				</div>
			{/each}
		</div>
	</div>
	<Seperation />
</footer>

<style>
		.footer-container {
				width: 95%;
				margin-left: 1em;
				margin-bottom: 2rem;
				background: var(--bg-overlay-second);
				box-shadow: 4px 4px 0px var(--overlay-shadow);
		}

		.commands-container {
				display: flex;
				gap: 3em;
				background-image: linear-gradient(90deg, rgba(var(--element-color), .9), rgba(var(--element-color), .9) 14px, 
				transparent 14px, transparent 20px,
				rgba(var(--element-color), .9) 20px, rgba(var(--element-color), .9) 23px, transparent 23px);
				padding-left: 5em;
		}

		.command {
			display: flex;
			align-items: center;
			gap: 0.5em;
		}

		.key {
				padding: 0;
				margin: 0;
				background: var(--bg-overlay-selected);
				height: 1.5em;
				width: 1.5em;
				font-size: 0.9em;
				font-weight: bold;
				color: var(--fg-font-selected);
				align-items: center;
				border-radius: 0.2em;
				display: flex;
				justify-content: center;
				align-items: center;
				text-align: center;
		}
</style>
