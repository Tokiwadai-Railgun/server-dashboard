import type { VMList } from "$lib/data/types/vms";
import { redirect, type Cookies, type ServerLoad } from "@sveltejs/kit";
import { throws } from "assert";
import { API_URL } from "$env/static/private"
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ cookies }: { cookies: Cookies})  => {
	if (API_URL == undefined) throw Error("API_URL env var not defined")
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

	// ----- Get list of vms and export it -----
	let vmListQuery = await fetch(API_URL + "/proxmox/vms", {
		method: "GET",
		credentials: "include",
		headers: {
			"session_token": session_token
		}
	})

	let json: {data: VMList[]} = await vmListQuery.json()

	if (vmListQuery.ok) {
		return {serverList: json}
	}

}
