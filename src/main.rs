use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    features: Option<String>,
    #[arg(short, long)]
    chip: Option<String>,
}

fn main() {
    let args = Args::parse();

    let features: Vec<String> = if let Some(features) = args.features {
        features.split(&[' ', ',']).map(|v| v.to_string()).collect()
    } else {
        vec![]
    };

    let features_list = features
        .iter()
        .map(|v| format!("\"{}\"", v))
        .collect::<Vec<String>>()
        .join(", ");

    let chip = args.chip.unwrap_or("esp32c3".to_string()).to_lowercase();

    let target = match chip.as_str() {
        "esp32" => "xtensa-esp32-none-elf",
        "esp32s2" => "xtensa-esp32s2-none-elf",
        "esp32s3" => "xtensa-esp32s3-none-elf",
        "esp32c2" | "esp32c3" => "riscv32imc-unknown-none-elf",
        _ => "riscv32imac-unknown-none-elf",
    };

    let content = format!(
        "
{{
    \"rust-analyzer.cargo.target\": \"{target}\",
    \"rust-analyzer.cargo.features\": [
        {features_list}
    ],
    \"rust-analyzer.cargo.buildScripts.useRustcWrapper\": true,
    \"rust-analyzer.cargo.allTargets\": false,
    \"rust-analyzer.check.allTargets\": false,    
    \"rust-analyzer.showUnlinkedFileNotification\": false,
}}     
    "
    )
    .trim()
    .to_string();

    std::fs::create_dir_all(".vscode/").unwrap();
    std::fs::write(".vscode/settings.json", content).unwrap();
}
