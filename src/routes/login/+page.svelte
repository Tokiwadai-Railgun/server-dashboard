<script lang="ts">
	import type { PageProps } from "../$types";

		let username = $state("")
		let password = $state("")

		let {data, form}: PageProps = $props()
		// Transfrom this to a server action
</script>

<h1>Login</h1>

<!-- Generate the login page: -->

<form method="POST">
		{#if form?.missing}<p>Missing cookie returned bu the server</p>{/if}
		{#if form?.incorrect}<p>Invalid Credentials</p>{/if}
		<label>
				Username
				<input type="text" name="username" placeholder="Username" required value={form?.username || ''}>
		</label>
		<label>
				Password
				<input type="password" name="password" placeholder="Password" required bind:value={password}>
		</label>
		<button type="submit">Log in</button>
</form>
