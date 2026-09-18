\
use anyhow::{bail, Context, Result};
use clap::Parser;
use regex::Regex;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "bullet", version, about = "Fast RustScan discovery + Nmap enumeration")]
struct Args {
    /// Target IPv4/IPv6 address or hostname
    target: String,

    /// RustScan batch size
    #[arg(long, default_value_t = 4500)]
    batch_size: u32,

    /// RustScan ulimit value
    #[arg(long, default_value_t = 5000)]
    ulimit: u32,

    /// Skip OS detection if the scan is run without sufficient privileges
    #[arg(long)]
    no_os: bool,
}

fn command_exists(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("\n==============================");
    println!("        BULLETSCAN");
    println!("  Fast discovery. Deep detail.");
    println!("==============================\n");

    if !command_exists("rustscan") {
        bail!("RustScan was not found in PATH. Install RustScan first.");
    }
    if !command_exists("nmap") {
        bail!("Nmap was not found in PATH. Install Nmap first.");
    }

    println!("[1/2] Running RustScan against {} ...\n", args.target);

    let rustscan_output = Command::new("rustscan")
        .args([
            "-a", &args.target,
            "--batch-size", &args.batch_size.to_string(),
            "--ulimit", &args.ulimit.to_string(),
            "--",
            "-Pn",
        ])
        .output()
        .with_context(|| "Failed to start RustScan")?;

    print!("{}", String::from_utf8_lossy(&rustscan_output.stdout));
    eprint!("{}", String::from_utf8_lossy(&rustscan_output.stderr));

    if !rustscan_output.status.success() {
        bail!("RustScan exited with status {}", rustscan_output.status);
    }

    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&rustscan_output.stdout),
        String::from_utf8_lossy(&rustscan_output.stderr)
    );

    // RustScan commonly prints: "Open 192.168.1.1:22"
    let re = Regex::new(r"Open\s+\S+:(\d+)")?;
    let mut ports: Vec<u16> = re
        .captures_iter(&combined)
        .filter_map(|cap| cap.get(1)?.as_str().parse::<u16>().ok())
        .collect();

    ports.sort_unstable();
    ports.dedup();

    if ports.is_empty() {
        println!("\nNo open TCP ports were parsed from RustScan output.");
        println!("You can rerun with RustScan directly to inspect its output.");
        return Ok(());
    }

    let port_list = ports
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("\nOpen ports found: {}", port_list);
    println!("\n[2/2] Running Nmap service, script, and OS detection ...\n");

    let mut nmap = Command::new("nmap");
    nmap.args(["-sV", "-sC"]);
    if !args.no_os {
        nmap.arg("-O");
    }
    nmap.args(["-p", &port_list, &args.target]);

    let status = nmap
        .status()
        .with_context(|| "Failed to start Nmap")?;

    if !status.success() {
        bail!("Nmap exited with status {}", status);
    }

    println!("\nBulletScan completed.");
    Ok(())
}
