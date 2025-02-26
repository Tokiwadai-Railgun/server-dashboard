import type { Actions } from "./$types"
import { redirect, type Cookies } from "@sveltejs/kit"
import { API_URL } from "$env/static/private"

// if no cookies then redirect to /login
export async function load({cookies}: {cookies: Cookies}) {
		const session_token = cookies.get("session_token")

		if (!session_token) redirect(307, '/login')

		// verify validity of token

		// add cookie to header so the api can read it
		let response = await fetch(API_URL + "/authorize", {
				method: "POST",
				credentials: "include",
				headers: {
						"session_token": session_token
				}
		});

		if (response.ok) {
				return {
						session_token: session_token
				}
		} else {
				// delete session cookie
				cookies.delete('session_cookie', {path: "/"})
				redirect(307, '/login')
		}
}

export const actions = {
		login: async ({cookies}: {cookies: Cookies}) => {
				console.log("logging out")
				let token = cookies.get("session_token");
				console.log(token)
				
				if (token == null) return

				const response = await fetch("http://localhost:8070/logout", {
						headers: {"Authorization": token},
						method: "POST"
				})

				if (response.status == 200) {
						console.log("Redirecting")
						redirect(302, "/login")
				}
		}
}
