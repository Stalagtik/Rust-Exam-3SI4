# Miscellaneous Rust Utilities

A collection of network utility tools written in Rust as part of the 3A school projects. This repository contains four distinct parts, incrementally building capabilities from simple host-up checking to full target struct management and port scanning.

## Repository Structure

The project is divided into four sub-folders, each addressing a specific network check/scan functionality:

```text
├── group-01-part-1/   # Individual host checker and subnet sweep utility using Ping and CIDR range input
├── group-01-part-2/   # Single port scanner on a target IP address using TCP connection timeouts
├── group-01-part-3/   # Multi-port scanner supporting lists (e.g., 80,443) and ranges (e.g., 20-80) with debug options
└── group-01-part-4/   # Object-oriented "Target" structure encapsulating host status, port verification, and target tracking
```

---

## Features & Sub-projects

### 🌐 Part 1: Host Checker & Subnet Sweeper
Checks if a host or a range of hosts is online.
- Parses individual IP addresses or FQDNs.
- Checks CIDR notation ranges (e.g., `192.168.1.0/24`) to sweep the subnet.
- Automatically excludes network (`.0`) and broadcast (`.255`) addresses.
- Uses ICMP ping requests to verify network availability.

### 🔍 Part 2: Single Port Scanner
Determines if a target TCP port is open.
- Takes an IP address and a port as inputs.
- Attempts a connection with a 3-second timeout limit.
- Reports whether the port is open or closed.

### 📊 Part 3: Advanced Port Scanner
Scans multiple TCP ports on a target IP address.
- Parses comma-separated lists of ports (e.g., `80,443,8080`).
- Parses port ranges (e.g., `20-100`).
- Includes a debug option (`-d` or `--debug`) to show both open and closed ports (by default only open ports are shown).

### 🛡️ Part 4: Target Management Library
Defines a reusable Object-Oriented interface for target scanning.
- Core **`Target`** structure encapsulating `ip_addr`, `is_up`, and `open_ports`.
- Integrates scanning utilities to populate the target state dynamically.
- Includes a sample driver binary to demonstrate instance instantiation and scanning flow.

---

## Getting Started

### Prerequisites

You need the Rust toolchain installed. If not, follow the instructions at [rustup.rs](https://rustup.rs/).

### Compiling and Running

Each sub-project is self-contained with its own `Cargo.toml`. To run any of the parts, navigate into the respective folder and execute `cargo run -- <arguments>`.

#### Running Part 1 (Host Checker)
```bash
cd group-01-part-1
# Check a single host
cargo run -- 8.8.8.8

# Sweep a CIDR range
cargo run -- 192.168.1.0/24
```

#### Running Part 2 (Single Port Scanner)
```bash
cd group-01-part-2
cargo run -- 127.0.0.1 80
```

#### Running Part 3 (Advanced Port Scanner)
```bash
cd group-01-part-3
# Scan specific list and ranges of ports (only shows open ports by default)
cargo run -- 127.0.0.1 22,80,443,8080-8090

# Scan ports showing both open and closed statuses
cargo run -- 127.0.0.1 22,80 -d
```

#### Running Part 4 (Target Management)
```bash
cd group-01-part-4
cargo run
```

## Authors
- **TheoOrigin**
- **Jiullian**
