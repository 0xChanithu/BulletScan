# BulletScan

BulletScan is a beginner-friendly Rust command-line wrapper that combines:

1. **RustScan** for fast TCP port discovery.
2. **Nmap `-sV`** for service and version detection.
3. **Nmap `-sC`** for default NSE scripts.
4. **Nmap `-O`** for operating-system detection.

> Use this tool only against systems you own or have explicit permission to test.

## Requirements

- Rust and Cargo
- RustScan
- Nmap
- Linux/Kali Linux is recommended

## Build

```bash
cargo build --release
```

The compiled binary will be:

```bash
target/release/bullet
```

## Run

```bash
./target/release/bullet 192.168.1.1
```

If OS detection causes a permissions problem:

```bash
sudo ./target/release/bullet 192.168.1.1
```

Or skip OS detection:

```bash
./target/release/bullet 192.168.1.1 --no-os
```

## Install as `bullet`

```bash
sudo cp target/release/bullet /usr/local/bin/bullet
```

Then run:

```bash
bullet 192.168.1.1
```

## Notes

- RustScan output formats can vary between versions. This project parses the common `Open host:port` format.
- Nmap's `-O` option may require root privileges and may not identify an OS accurately on every target.
- The tool scans TCP ports only through RustScan's normal workflow.
