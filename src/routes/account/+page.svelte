<script>
		import Button from "$lib/components/Button.svelte";

		let { data } = $props()
</script>

<h1>Account</h1>

<p class="content">Informations sur le compte de l'utilisateur</p>

<form action="?/login" method="POST">
		<Button form={true} onclick={() => ({})}><span>Log Out</span></Button>
</form>

<style>
		span {
				font-size: 1.5em;
		}
</style>
