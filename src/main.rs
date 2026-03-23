use clap::Parser;

#[derive(Parser)]
#[command(name = "container-health-rs")]
struct Cli {
    #[arg(long)]
    url: Option<String>,
    #[arg(long, default_value = "5")]
    timeout: u64,
}

fn main() {
    let cli = Cli::parse();

    if let Some(url) = cli.url {
        match check_health(&url, cli.timeout) {
            Ok(status) => {
                println!("[OK] {} -> {}", url, status);
            }
            Err(e) => {
                eprintln!("[FAIL] {} -> {}", url, e);
                std::process::exit(1);
            }
        }
    }
}

fn check_health(url: &str, timeout: u64) -> Result<u16, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(url).send().map_err(|e| e.to_string())?;
    let status = resp.status().as_u16();
    if status >= 200 && status < 400 {
        Ok(status)
    } else {
        Err(format!("HTTP {}", status))
    }
}
