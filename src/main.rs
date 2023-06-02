use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    features: Option<String>,
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
    let features_flat = features.join(" ");

    let content = format!(
        "
    {{
        \"rust-analyzer.cargo.features\": [
            {}
        ],
        \"rust-analyzer.check.overrideCommand\": [
            \"cargo\",
            \"check\",
            \"--examples\",
            \"--bins\",
            \"--lib\",            
            \"--message-format=json\",
            \"--no-default-features\",
            \"--features\",
            \"{}\",
        ],
        \"rust-analyzer.check.allTargets\": false,
        \"rust-analyzer.cargo.buildScripts.invocationLocation\": \"root\",
        \"rust-analyzer.check.invocationLocation\": \"root\",
        \"rust-analyzer.cargo.buildScripts.overrideCommand\": [
            \"cargo\",
            \"check\",
            \"--examples\",
            \"--bins\",
            \"--lib\",            
            \"--message-format=json\",
            \"--no-default-features\",
            \"--features\",
            \"{}\",
        ],
        \"rust-analyzer.showUnlinkedFileNotification\": false,
    }}     
    ",
        features_list, features_flat, features_flat
    );

    std::fs::create_dir_all(".vscode/").unwrap();
    std::fs::write(".vscode/settings.json", content).unwrap();
}
