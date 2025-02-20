import { redirect, type Cookies } from "@sveltejs/kit";
import { API_URL } from "$env/static/private"

export async function load({cookies}: {cookies: Cookies}) {
		const session_token = cookies.get("session_token")
		if (!session_token) redirect(307, "/login")

		let response = await fetch(API_URL + "/authorize", {
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

		// ----- Get list of files and export it -----
		let fileListQuery = await fetch(API_URL + "/storage/files", {
				method: "GET",
				credentials: "include",
				headers: {
						"session_token": session_token
				}
		})

		let json: {data: File[]} = await fileListQuery.json()
		console.log(json)

		if (fileListQuery.ok) {
				return {fileList: json}
		}
}
