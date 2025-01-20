<script>
		let username = $state("")
		let password = $state("")

		async function handleSubmit() {
				await fetch("http://localhost:8080/login", {
						method: "POST",
						headers: { "Content-Type": "application/json", },
						credentials: "include",
						body: JSON.stringify({
								"username": username,
								"password": password,
								"id": 0
						})
				});
		}

</script>

<h1>Login</h1>

<!-- Generate the login page: -->

<form on:submit|preventDefault={handleSubmit}>
		<label>
				Username
				<input type="text" name="username" placeholder="Username" required bind:value={username}>
		</label>
		<label>
				Password
				<input type="password" name="password" placeholder="Password" required bind:value={password}>
		</label>
		<button type="submit">Log in</button>
</form>
