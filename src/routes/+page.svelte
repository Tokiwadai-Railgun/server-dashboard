<script lang="ts">
		import InformationOverlay from "$lib/components/InformationOverlay.svelte";
import List from "$lib/components/List.svelte";

		// Styles for the list snippet
		import "$lib/styles/borderedList.css"


		const serverList: {  // sample response of the proxmox api
		vmid: number, 
		name: string, 
		status: "running" | "stopped" | "paused" | "starting" | "stopping",
		maxmem: number,
		maxcpu: number,
		mem: number | null,
		uptime: number | null,
		cpu: number | null
		}[] = [
			{
			  vmid: 100,
			  name: "ubuntu-server",
			  status: "running",
			  maxmem: 4294967296,
			  maxcpu: 2,
				mem: 2147483648,
			  uptime: 1234567,
			  cpu: 0.45,
			},
			{
			  vmid: 101,
			  name: "debian-test",
			  status: "stopped",
			  maxmem: 2147483648,
			  maxcpu: 1,
			  uptime: null,
				mem: null,
			  cpu: null,
			}
		]

		let currentlySelected = $state(serverList[0])
		let test = $state("hehe");

		function changeFocus({element}: {element: any}) {
				currentlySelected = serverList.find((server) => server.name == element.name) || serverList[0]
		}
</script>

<h1>Dashboard</h1>

<div class="container content">
		<List elements={serverList} bind:selection={currentlySelected}>
				{#snippet body({element, tabindex}: {element: any, tabindex: any})}
				<div class="item-container">
						<button class:selected={element.name == currentlySelected.name} class="list-item" tabindex="0" onclick={() => changeFocus({element})}>
								<span 
										class="element" 
										class:powered={element.status == "running"}
								>{element.name}</span>
						</button>
				</div>

				{/snippet}
		</List>

		<InformationOverlay title={currentlySelected.name} --square-color={currentlySelected.status == "running" ? "#8B9A7D" : ""}>
			{#snippet content()}
					<p class="server-status">{currentlySelected.status}</p>
					<p class="memory">RAM {Math.round((currentlySelected.mem || 0) / (1000 * 1000 * 1000))}go of {Math.round(currentlySelected.maxmem / (1000 * 1000 * 1000))}go ({(currentlySelected.mem || 0) * 100 / currentlySelected.maxmem}%)</p>
					<p class="cpu">CPU {(currentlySelected.cpu || 0) * 100 / currentlySelected.maxcpu}% of {currentlySelected.maxcpu} cores </p>
			{/snippet}
		</InformationOverlay>
</div>



<style>
		.container {
				display: flex;
				justify-content: space-between;
		}
</style>


