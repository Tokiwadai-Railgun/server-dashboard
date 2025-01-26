import { redirect } from "@sveltejs/kit"

// if no cookies then redirect to /login
export async function load({cookies}) {
		const session_token = cookies.get("session_token")

		if (!session_token) redirect(307, '/login')

		// verify validity of token

		// add cookie to header so the api can read it
		let response = await fetch("http://localhost:8080/authorize", {
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

