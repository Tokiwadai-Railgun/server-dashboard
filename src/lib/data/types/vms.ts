export interface VMInfo {
	vmid: number
	name: string,
	status: string,
	uptime: number
	cpu: {
		current_cpu: number,
		max_cpu: number
	},
	mem: {
		current_mem: number,
		max_mem: number
	},
	ip: [
		{
			interface_name: string,
			ip_address: string
		}
	],
	storage: [
		{
			name: String,
			current_disk: number // in bytes
			max_disk: number		 // in bytes
		}
	]
}
