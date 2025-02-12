import { redirect, type Cookies } from "@sveltejs/kit";

export async function load({cookies}: {cookies: Cookies}) {
	const session_token = cookies.get("session_token")
	if (!session_token) redirect(307, "/login")

	let response = await fetch("http://localhost:8080/authorize", {
		method: "POST",
		credentials: "include",
		headers: {
			"session_token": session_token
		}
	})

	if (!response.ok) {
		cookies.delete('session_cookie', {path: "/"})
		redirect(307, "/login")
	}

	// ----- Get list of vms and export it -----
	let vmListQuery = await fetch("http://localhost:8080/proxmox/vms", {
		method: "GET",
		credentials: "include",
		headers: {
			"session_token": session_token
		}
	})

	let json = await vmListQuery.json()

	if (vmListQuery.ok) {
		return {serverList: json.data}
	}

	
}
