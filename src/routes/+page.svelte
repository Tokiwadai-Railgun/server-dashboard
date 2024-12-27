<script lang="ts">
		import List from "$lib/components/List.svelte";

		// Styles for the list snippet
		import "$lib/styles/borderedList.css"

		let currentlySelected = $state("")

		const serverList: { name: string, powered: boolean}[] = [
				{ name: "API / Nginx", powered: true },
				{ name: "Docker", powered: true },
				{ name: "Izuna", powered: true },
				{ name: "TP MariaDB", powered: false },
				{ name: "TP OWASP", powered: false },
				{ name: "Mutillidae", powered: true },
				{ name: "Apache2", powered: false },
				{ name: "Debian12 Server", powered: false },
				{ name: "Debian12 Server", powered: false },
				{ name: "Debian12 Server", powered: false },
				{ name: "Debian12 Server", powered: false },
				{ name: "Debian12 Server", powered: false },
				{ name: "Debian12 Server", powered: true },
		]

		let test = $state("hehe");
</script>

<h1>Dashboard</h1>

<div class="container content">
		<List elements={serverList} bind:selection={currentlySelected}>
				{#snippet body({name, powered})}
				<div class="item">
						<p class="element" class:powered={powered}>{name}</p>
				</div>

				{/snippet}
		</List>

		<p>Currently selected = {currentlySelected}</p>
</div>



<style>
		.container {
				display: flex;
		}
</style>


