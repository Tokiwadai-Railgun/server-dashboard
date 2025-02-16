export interface StorageInfo {
  disk: string;
  storage_max: number;  // in bytes
  storage_used: number; // in bytes
}

export interface VMInfo {
  node: string;
  vmid: number;
  name: string;
  status: 'running' | 'stopped' | 'suspended' | 'unknown';
  qmpstatus: string;
  max_memory: number;    // in MB
  max_cpu: number;       // number of cores
  uptime: number;        // in seconds
  memory_usage: number;  // in bytes
  cpu_usage: number;     // as percentage (0-100)
  storage: StorageInfo[];
  ip_addresses: string[];
  os_type: string;
  cpu_type: string;
  ha_state: string;
  last_updated: string;  // ISO 8601 timestamp
}

// Sample data matching the interface
export const serverList: VMInfo[] = [
  {
    node: "pve-node1",
    vmid: 100,
    name: "web-server-prod",
    status: "running",
    qmpstatus: "running",
    max_memory: 8192,
    max_cpu: 4,
    uptime: 1209600,  // 14 days
    memory_usage: 4294967296,  // 4GB in bytes
    cpu_usage: 45.5,
    storage: [
      {
        disk: "scsi0",
        storage_max: 53687091200,  // 50GB in bytes
        storage_used: 32212254720   // 30GB in bytes
      },
      {
        disk: "scsi1",
        storage_max: 107374182400,  // 100GB in bytes
        storage_used: 64424509440   // 60GB in bytes
      }
    ],
    ip_addresses: ["192.168.1.100", "10.0.0.100"],
    os_type: "ubuntu",
    cpu_type: "kvm64",
    ha_state: "started",
    last_updated: "2024-12-28T14:30:00Z"
  },
  {
    node: "pve-node1",
    vmid: 101,
    name: "db-server-prod",
    status: "running",
    qmpstatus: "running",
    max_memory: 16384,
    max_cpu: 8,
    uptime: 2419200,  // 28 days
    memory_usage: 12884901888,  // 12GB in bytes
    cpu_usage: 75.2,
    storage: [
      {
        disk: "scsi0",
        storage_max: 107374182400,  // 100GB in bytes
        storage_used: 75161927680   // 70GB in bytes
      }
    ],
    ip_addresses: ["192.168.1.101"],
    os_type: "centos",
    cpu_type: "kvm64",
    ha_state: "started",
    last_updated: "2024-12-28T14:30:00Z"
  },
  {
    node: "pve-node2",
    vmid: 102,
    name: "test-environment",
    status: "stopped",
    qmpstatus: "stopped",
    max_memory: 4096,
    max_cpu: 2,
    uptime: 0,
    memory_usage: 0,
    cpu_usage: 0,
    storage: [
      {
        disk: "scsi0",
        storage_max: 32212254720,  // 30GB in bytes
        storage_used: 10737418240  // 10GB in bytes
      }
    ],
    ip_addresses: [],
    os_type: "windows",
    cpu_type: "kvm64",
    ha_state: "stopped",
    last_updated: "2024-12-28T14:30:00Z"
  }
];
