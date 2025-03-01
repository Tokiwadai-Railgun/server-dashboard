import { fail, redirect, type Cookies } from "@sveltejs/kit";
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
						"Authorization": session_token,
						"user_id": "1"
				}
		})

		let json: {data: File[]} = await fileListQuery.json()
		console.log(json)

		if (fileListQuery.ok) {
				return {fileList: json}
		}
}

export const actions = {
		submit: async({ request }: {request: Request}) => {
				// TODO: Get the file content and send request to api to send the file
				const data = await request.formData();
				console.log(data)
				const file = data.get("file-input");

				console.log(file);
				const acceptedFileTypes = [".docx", ".pdf", ".xls", ".xlsx", ".doc", ".png", ".jpg", ".jpeg", ".gif", ".webp"];

				if (!acceptedFileTypes) return fail(401, {error:true})

				redirect(302, "/cloud")
		}
}
