<script lang="ts">
		import Seperation from "./Seperation.svelte";
		let {commands}: {commands: {name: string, action: any, key: string}[]} = $props();
</script>


<footer>
	<div class="footer-container">
		<p>{commands[0].name}</p>
	</div>
	<Seperation />
</footer>

<style>
		.footer-container {
				padding-left: 3rem;
				margin-left: 1em;
				background-image: linear-gradient(90deg, rgba(var(--element-color), .2), rgba(73,70,61, .2) 14px, 
				transparent 14px, transparent 20px,
				rgba(var(--element-color), .2) 20px, rgba(var(--element-color), .2) 23px, transparent 23px);
				margin-bottom: 2rem;
		}
</style>
