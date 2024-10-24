# VSCode Rust Analyzer Settings Generator

This is for my personal use but might be helpful for others.

Installation:
`cargo install --path .`
or directly from GitHub

Example:
```
git clone https://github.com/esp-rs/esp-hal.git
cd esp-hal
cd esp-hal-common
mksettings --features=esp32c3
code .
```

This will overwrite `.vscode/settings.json` without asking!
