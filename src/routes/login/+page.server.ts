import { goto } from "$app/navigation";
import { fail, redirect, type Actions } from "@sveltejs/kit";
import { API_URL } from "$env/static/private"

export const actions = {
		default: async({cookies, request}) => {
				const data = await request.formData()
				const username = data.get('username')

				// query api here
				let response = await fetch(API_URL + "/login", {
						method: "POST",
						headers: { "Content-Type": "application/json", },
						credentials: "include",
						body: JSON.stringify({
								"username": username,
								"password": data.get('password'),
								"id": 0
						})
				});
				if (response.status != 200) return fail(401, {username, incorrect: true})

				// Then extract the cookie
				let session_cookie = response.headers.get("set-cookie")
				if (!session_cookie) {
						return fail( 500, {session_cookie, missing: true})
				} 

				let splited_cookie = session_cookie.split('; ');
				cookies.set(splited_cookie[0].split("=")[0], splited_cookie[0].split("=")[1], {path: "/", httpOnly: true})

				throw redirect(303, "/account")
		}
} satisfies Actions;
