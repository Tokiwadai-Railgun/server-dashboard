<script>
	import Button from "./Button.svelte";
	import SeparationSecondary from "./SeparationSecondary.svelte";

</script>
<div class="occlusion">
		<div class="message">
				<h2>System Warning</h2>
				<p class="message-content">Do you want to shut down this vm ?</p>
				<SeparationSecondary />
				<div class="buttons">
						<Button>Yes</Button>
						<Button>No</Button>
				</div>
		</div>
</div>


<style>
	.occlusion {
		width: 100vw;
		height: 100vh;
		background: rgba(var(--element-color), 0.4);
		position: absolute;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	h2 {
			font-family: "FOT-Rodin";
			letter-spacing: 0.5em;
			margin: 0;
			width: 100%;
			background: var(--bg-overlay-selected);
			color: var(--fg-font-selected);
			display: flex;
			align-items: center;
	}

	h2::before {
		content:"";
		width: 1em;
		height: 1em;
		background: var(--bg-overlay-second);
		float: left;
		margin: 6px 6px 6px 6px;
	}

	.message {
			display: flex;
			flex-direction: column;
			position: relative;
			background: var(--bg-overlay-second);
			width: 50vw;
			height: fit-content;
			padding-bottom: 0.5em;
	}

	p {
			margin: 1em;
	}
	.buttons {
			transform: translateX(-1.5em);
			margin-top: 0.5em;
			bottom: 0;
			position: relative;
			display: flex;
			justify-content: center;
			gap: 4em;
			flex-wrap: wrap;
	}

	@media screen and (max-width: 860px) {
			.buttons {
					gap: 1em;
			}
	}
</style>
