import { error, fail, redirect, type Cookies } from "@sveltejs/kit";
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

		if (fileListQuery.ok) {
				return {fileList: json}
		}
}

export const actions = {
		submit: async({ request, cookies }: {request: Request, cookies: Cookies}) => {
				const token = cookies.get("session_token")

				if (!token) {
						return fail( 401, {
								error: true,
								message: "Unaothorized"
						} )
				}

				const data = await request.formData();
				const file = data.get("file") as File;
				console.log(file.type)
				const acceptedFileTypes = ["image/png", "image/jpeg"];

				if(!acceptedFileTypes.includes(file.type)) {
						return fail(400, {
								error: true,
								message: "Type of file is not accepted, please input one of the following : " + acceptedFileTypes.join(",")
						})
				}

				const content = await file.arrayBuffer()
				const base64Content = btoa(new Uint8Array(content).reduce((data, byte) => data + String(byte), ''))

				let body = {    
						"file_name": file.name,
						"file_size": file.size,
						"description": " ",
						"file_type": file.type,
						"file_content": base64Content
				}

				const headers = new Headers([["session_token", token], ["Content-Type", "application/json"]])
				const response = await fetch(API_URL + "/storage/upload", {
						method: "POST",
						body: JSON.stringify(body),
						headers: headers
				})

				if (!response.ok) {
						console.log(response.status)
						console.log(await response.text())
						return fail(500, {
								error: true,
								message: "Error occured when saving the file"
						})
				}

				redirect(302, "/cloud")
		},

		cancel: async() => {
				return
		},

		download: async({ request, cookies }: {request: Request, cookies: Cookies}) => {
				// TODO: Make download endpoint
		}
}
