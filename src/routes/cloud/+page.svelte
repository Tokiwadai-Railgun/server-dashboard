<script lang="ts">
		import "$lib/styles/borderedList.css"
		const props = $props()
		const elements = props.data.fileList
		import List from "$lib/components/List.svelte"
		import type { File } from "$lib/data/types/files";
		import Footer from "$lib/components/Footer.svelte"
		import InformationOverlay from "$lib/components/InformationOverlay.svelte";
	import SeparationSecondary from "$lib/components/SeparationSecondary.svelte";

		let currentlySelected: File | undefined = $state(elements[0]);

		function changeFocus({element}: {element: any}) {
				currentlySelected = element
		}
</script>
<h1>Cloud</h1>

<div class="content container">
		<List elements={elements} bind:selection={currentlySelected}> 
				{#snippet body({element, tabindex}: {element: any, tabindex: any})}
						<div class="item-container">
								<button class:selected={element.id == currentlySelected?.id} class="list-item" tabindex="0" onclick={() => changeFocus({element})}>
										<span 
												class="element" 
												class:powered={element.status == "running"}
										>{element.name}</span>
								</button>
						</div>
				{/snippet}
		</List>

		<InformationOverlay title="File">
				{#snippet content()}
						<img class="image" src="/icons/nier_placeholder.png" alt="Placeholder" />
						<!-- Display image if possible else display a placeholder -->
						<SeparationSecondary />
						<!-- Description -->
				{/snippet}
		</InformationOverlay>

		<InformationOverlay title="Properties">
				{#snippet content()}
						<div class="data bold">
								<span>{currentlySelected?.name}</span>
						</div>
						<SeparationSecondary />
						<div class="data">
								<span>Size</span>
								<span>{currentlySelected?.size}</span>
						</div>
						<div class="data">
								<span>Created</span>
								<span>{currentlySelected?.created}</span>
						</div>
						<div class="data">
								<span>Type</span>
								<span>{currentlySelected?.file_type}</span>
						</div>
				{/snippet}
		</InformationOverlay>

</div>

<footer>
		<Footer commands={[{name: "Download", action: () => {console.log("start")}, key: "D"}]}/>
</footer>

<style>
		.container {
				padding-right: 4em;
				display: flex;
				justify-content: space-between;
		}

		img {
				margin: 0.3em;
		}

		.bold {
				font-weight: bold;
		}

		.data {
				padding: 0;
				margin: 0 1em 0 1em;
				display: flex;
				justify-content: space-between;
				align-items: flex-start;
				height: 2em;
		}
</style>
