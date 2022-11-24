# rusty2048
## Usage Instructions:
1. Install Rust Toolchain (directly copied from rustup.rs): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Run with `cargo run`
3. Use the arrow keys to select Play, Train, Train and Test, or Test
    - Play allows you to play the game yourself from the command line
    - Train loads in the existing learned state and trains it additionally
    - Train and Test loads in the existing learned state, then tests it, breaking the game loop to retrain depending on the result state
