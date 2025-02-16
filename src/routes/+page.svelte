<script lang="ts">
		import InformationOverlay from "$lib/components/InformationOverlay.svelte";
		import List from "$lib/components/List.svelte";
		// Styles for the list snippet
		import "$lib/styles/borderedList.css"
		import SeparationSecondary from "$lib/components/SeparationSecondary.svelte";
		import Footer from "$lib/components/Footer.svelte";
		import formatTime from "$lib/functions/formatTime";
		import PopUp from "$lib/components/PopUp.svelte";
			import type { VMInfo } from "$lib/data/types/vms";

		const props: {data: {serverList: VMInfo[]}} = $props();
		const serverList = props.data.serverList;
		console.log("Server list : ", serverList)

		let currentlySelected: VMInfo = $state(serverList[0])


		function changeFocus({element}: {element: any}) {
				currentlySelected = serverList.find((server: any) => server.name == element.name) || serverList[0]
		}

		function formatStorage(storage: number) { // oc
				return Math.round(storage / (1000 * 1000 * 1000)) + "go"
		}

		// Commands 
		const commandList: {name: string, action: any, key: string}[] = [
				{name: "Start / Stop", action: () => console.log("Start / Stop"), key: "S"},
		]

		function handleKeydown(event: KeyboardEvent) {
			const command = commandList.find(command => command.key.toUpperCase() == event.key.toUpperCase())
			if (command) {
					// Trigger popup
					command.action()
			}
		}



		// ----- Get specific vm information -----
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
						<div class="data">
								<span>Status</span>
								<span>{currentlySelected.status}</span>
						</div>
						{#each currentlySelected.ip as ip}
								<div class="data">
										<span>{ip.interface_name}</span>
										<span>{ip.ip_address}</span>
								</div>
						{/each}
						<SeparationSecondary />
						<div class="data">
								<span>RAM</span>
								<span>{formatStorage(currentlySelected.mem.current_mem)} of {formatStorage(currentlySelected.mem.max_mem)} ({Math.round((currentlySelected.mem.current_mem || 0) * 100 / (currentlySelected.mem.max_mem * 1000 * 1000 * 1000))}%)</span>
						</div>
						<div class="data">
								<span>CPU</span>
								<span>{Number((currentlySelected.cpu.current_cpu.toFixed(2)) || 0) / currentlySelected.cpu.max_cpu}% of {currentlySelected.cpu.max_cpu} cores</span>
						</div>
						{#each currentlySelected.storage as disk, count}
								<div class="data">
										<span>{disk.name}</span> <!-- Name -->
										<span>{formatStorage(disk.current_disk)} of {formatStorage(disk.max_disk)}</span>
								</div> 
						{/each}

						<SeparationSecondary />
						<div class="data">
								<span>OS</span>
								<span>{currentlySelected.os_type}</span>
						</div>
						<div class="data">
								<span>Uptime</span>
								<span>{formatTime(currentlySelected.uptime)}</span>
						</div>
				{/snippet}
		</InformationOverlay>
</div>

<!-- <PopUp /> -->


<footer>
		<Footer commands={commandList}/>
</footer>

<svelte:window on:keydown={handleKeydown} />


<style>
		.container {
				display: flex;
				justify-content: space-between;
		}

		/*Snippet style*/
		.data {
				padding: 0;
				margin: 0 1em 0 1em;
				display: flex;
				justify-content: space-between;
				align-items: flex-start;
				height: 2em;
		}

</style>


