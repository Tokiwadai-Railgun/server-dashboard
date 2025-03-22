<script lang="ts">
		import "$lib/styles/borderedList.css"
		const props = $props()
		import { page } from "$app/state";
		const elements = props.data.fileList
		import List from "$lib/components/List.svelte"
		import type { File } from "$lib/data/types/files";
		import Footer from "$lib/components/Footer.svelte"
		import InformationOverlay from "$lib/components/InformationOverlay.svelte";
		import SeparationSecondary from "$lib/components/SeparationSecondary.svelte";
		import PopUp from "$lib/components/PopUp.svelte";

		let message = $state('');
		let currentlySelected: File | undefined = $state(elements[0]);

		function changeFocus({element}: {element: any}) {
				currentlySelected = element
		}

		// ------- Actions -------
		const commands = [
				{name: "Download", action: downloadPopup, key: "D"},
				{name: "Upload",action: upload, key: "U"}
		]

		function handleKeyDown(event: KeyboardEvent) {
				const command = commands.filter(x => x.key == event.key.toUpperCase())[0];
				if (!command) return;
				command.action()
		}


		let form: HTMLFormElement;
		const choices = [
				{ name: "Submit", action: () => {form.requestSubmit()} },
				{ name: "Cancel", action: () => {filePopupShow = false} }
		]

		// Upload
		let files: any = $state()
		let filePopupShow = $state(false)
		function upload() {
				//Opens a popup to get file
				filePopupShow = !filePopupShow
		}

		let downloadPopupShow = $state(false)
		function downloadPopup() {
				downloadPopupShow = true;
		}

		let downloadForm: HTMLFormElement;
		const downloadChoices = [
				{ name: "Yes", action: () => {downloadForm.requestSubmit(); downloadPopupShow = false;} },
				{ name: "No", action: () => {downloadPopupShow = false} }
		]
</script>
<h1>Cloud</h1>

{#if page.form?.error}
		<div class="content-error">
				<p>{page.form.message}</p>
		</div>
{/if}


<div class="content container">
		<List elements={elements}> 
				{#snippet body({element, tabindex}: {element: any, tabindex: any})}
						<div class="item-container">
								<button class:selected={element.id == currentlySelected?.id} class="list-item" tabindex="0" onclick={() => changeFocus({element})}>
										<span 
											 class="element" 
											 class:powered={element.status == "running"}
											 >{element.path}</span>
								</button>
						</div>
				{/snippet}
		</List>

		{#if currentlySelected}
				<InformationOverlay title="File">
						{#snippet content()}
						<img class="image" src="/icons/nier_placeholder.png" alt="Placeholder" />
						<!-- Display image if possible else display a placeholder -->
						<SeparationSecondary />
						<div class="data bold">
								<span>{currentlySelected?.description}</span>
						</div>
						<!-- Description -->
						{/snippet}
				</InformationOverlay>

				<InformationOverlay title="Properties">
						{#snippet content()}
						<div class="data bold">
								<span>{currentlySelected?.path}</span>
						</div>
						<SeparationSecondary />
						<div class="data">
								<span>Size</span>
								<span>{currentlySelected?.size}</span>
						</div>
						<div class="data">
								<span>Type</span>
								<span>{currentlySelected?.file_type}</span>
						</div>
						{/snippet}
				</InformationOverlay>
		{/if}
</div>

<footer>
		<Footer commands={commands}/>
</footer>

<PopUp title="Upload" open={filePopupShow} choices={choices}>
		{#snippet body()}
		<p>Please select file to upload</p>
		<div class="popup-content">
				<form class="popup-background" action="?/submit" method="POST" bind:this={form} enctype="multipart/form-data">
						<label class="fileLabel">
								<span class="inputText">{files ? files[0].name : "Choisir un fichier"}</span>
								<input bind:files={files} type="file" name="file">
						</label>
				</form>
		</div>
		{/snippet}
</PopUp>

<PopUp title="File Download" open={downloadPopupShow} choices={downloadChoices}>
		{#snippet body()}
				<div class="popup-message">
						<form action={`/api/download/${currentlySelected?.path}`} method="GET" bind:this={downloadForm}>
							<input type="text" value={currentlySelected?.path} placeholder="File name" hidden />
						</form>
						<p>Do you want to download the file : <span class="popup-file-name">{currentlySelected ? currentlySelected.path : "ERROR"}</span> </p>
				</div>
		{/snippet}
</PopUp>

<svelte:window on:keydown={handleKeyDown} />

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

		.popup-content {
				display: flex;
				justify-content: center;
				width: 100%;
				height: 17em;
		}

		input[type=file] {
				display: none;
		}

		.fileLabel {
				display: flex;
				justify-content: center;
				align-items: center;
				height: 100%;
				width: 100%;
		}
		.fileLabel div {
				padding: 0.2em;
				border: var(--bg-overlay-selected) solid 2px;
				position: relative;
		}

		.popup-background {
				width: 30em;
				background-image: url("/icons/nier_placeholder.png");
				background-size: contain;
				background-repeat: no-repeat;
		}

		.popup-message {
				padding: 1em;
		}

		.popup-file-name {
				font-weight: bold;
		}

</style>
