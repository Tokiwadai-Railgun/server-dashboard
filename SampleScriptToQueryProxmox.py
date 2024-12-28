import proxmoxer
import requests
from datetime import datetime
from requests.packages.urllib3.exceptions import InsecureRequestWarning
import concurrent.futures
from typing import Dict, List, Union

requests.packages.urllib3.disable_warnings(InsecureRequestWarning)

class ProxmoxClusterInfo:
    def __init__(self, host: str, user: str, password: str):
        """
        Initialize the connection to Proxmox cluster.
        
        Args:
            host: Proxmox host address (e.g., 'proxmox.example.com')
            user: Username with appropriate permissions (e.g., 'root@pam')
            password: User's password
        """
        self.proxmox = proxmoxer.ProxmoxAPI(
            host,
            user=user,
            password=password,
            verify_ssl=False
        )

    def get_all_nodes(self) -> List[str]:
        """Get a list of all nodes in the cluster."""
        return [node['node'] for node in self.proxmox.nodes.get()]

    def get_vm_list(self, node: str) -> List[Dict]:
        """
        Get list of VMs on a specific node.
        
        Args:
            node: Name of the Proxmox node
            
        Returns:
            List of VMs with their basic information
        """
        try:
            # Get all QEMU VMs (type=qemu filters out LXC containers)
            return [vm for vm in self.proxmox.nodes(node).qemu.get() 
                   if vm['type'] == 'qemu']
        except Exception as e:
            print(f"Error getting VMs from node {node}: {str(e)}")
            return []

    def get_vm_details(self, node: str, vmid: int) -> Dict:
        """
        Get detailed information about a specific VM.
        
        Args:
            node: Name of the Proxmox node
            vmid: ID of the virtual machine
            
        Returns:
            Dictionary containing all VM details
        """
        try:
            # Get current runtime status
            status = self.proxmox.nodes(node).qemu(vmid).status.current.get()
            
            # Get VM configuration
            config = self.proxmox.nodes(node).qemu(vmid).config.get()
            
            # Get IP addresses if QEMU agent is running
            ip_addresses = []
            if status.get('agent', 0) == 1:
                try:
                    agent_info = self.proxmox.nodes(node).qemu(vmid).agent.get('network-get-interfaces')
                    for interface in agent_info.get('result', []):
                        for ip_info in interface.get('ip-addresses', []):
                            if ip_info.get('ip-address'):
                                ip_addresses.append(ip_info['ip-address'])
                except Exception:
                    pass

            # Get storage information for all disks
            storage_info = []
            for disk_name, disk_value in config.items():
                if disk_name.startswith(('scsi', 'ide', 'sata', 'virtio')):
                    if isinstance(disk_value, str) and ':' in disk_value:
                        storage, disk_path = disk_value.split(',')[0].split(':')
                        try:
                            disk_status = self.proxmox.nodes(node).storage(storage).content.get()
                            for disk in disk_status:
                                if disk.get('volid') == f"{storage}:{disk_path}":
                                    storage_info.append({
                                        'disk': disk_name,
                                        'storage_max': disk.get('size', 0),
                                        'storage_used': disk.get('used', 0)
                                    })
                        except Exception:
                            pass

            # Compile all information into a single dictionary
            return {
                'node': node,
                'vmid': vmid,
                'name': config.get('name', f'VM {vmid}'),
                'status': status.get('status', 'unknown'),
                'qmpstatus': status.get('qmpstatus', 'unknown'),
                'max_memory': config.get('memory', 0),  # in MB
                'max_cpu': config.get('cores', 1),
                'uptime': status.get('uptime', 0),  # in seconds
                'memory_usage': status.get('mem', 0),  # in bytes
                'cpu_usage': status.get('cpu', 0),  # as percentage
                'storage': storage_info,
                'ip_addresses': ip_addresses,
                'os_type': config.get('ostype', 'unknown'),
                'cpu_type': config.get('cpu', 'kvm64'),
                'ha_state': status.get('ha', {}).get('state', 'unknown'),
                'last_updated': datetime.now().isoformat()
            }

        except Exception as e:
            return {
                'node': node,
                'vmid': vmid,
                'error': str(e),
                'last_updated': datetime.now().isoformat()
            }

    def get_all_vms_info(self) -> List[Dict]:
        """
        Get detailed information about all VMs across all nodes in the cluster.
        Uses parallel processing to speed up data collection.
        
        Returns:
            List of dictionaries containing detailed information about each VM
        """
        all_vm_info = []
        nodes = self.get_all_nodes()

        # First, get list of all VMs across all nodes
        vm_list = []
        for node in nodes:
            vm_list.extend([(node, vm['vmid']) for vm in self.get_vm_list(node)])

        # Then get detailed information for each VM in parallel
        with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
            future_to_vm = {
                executor.submit(self.get_vm_details, node, vmid): (node, vmid)
                for node, vmid in vm_list
            }
            
            for future in concurrent.futures.as_completed(future_to_vm):
                vm_info = future.result()
                if vm_info:
                    all_vm_info.append(vm_info)

        return all_vm_info

def format_vm_info(vm_info: Dict) -> str:
    """Format VM information for display."""
    if 'error' in vm_info:
        return f"Error getting info for VM {vm_info['vmid']} on node {vm_info['node']}: {vm_info['error']}"
    
    storage_str = "\n".join(
        f"    {disk['disk']}: {disk['storage_used']/1024/1024:.1f}MB / {disk['storage_max']/1024/1024:.1f}MB"
        for disk in vm_info['storage']
    )
    
    return f"""
VM: {vm_info['name']} (ID: {vm_info['vmid']})
Node: {vm_info['node']}
Status: {vm_info['status']} ({vm_info['qmpstatus']})
CPU: {vm_info['cpu_usage']:.1f}% of {vm_info['max_cpu']} cores
Memory: {vm_info['memory_usage']/1024/1024:.1f}MB / {vm_info['max_memory']}MB
Uptime: {vm_info['uptime']/3600:.1f} hours
OS Type: {vm_info['os_type']}
IP Addresses: {', '.join(vm_info['ip_addresses']) or 'None detected'}
Storage:
{storage_str}
"""

# Example usage
if __name__ == "__main__":
    # Replace with your Proxmox details
    HOST = "proxmox.example.com"
    USER = "root@pam"
    PASSWORD = "your_password"
    
    # Create cluster info object
    cluster = ProxmoxClusterInfo(HOST, USER, PASSWORD)
    
    # Get all VM information
    print("Gathering information about all VMs...")
    all_vms = cluster.get_all_vms_info()
    
    # Display information for each VM
    for vm in all_vms:
        print(format_vm_info(vm))
