# Rust Football Manager

## File Structure
```
rust-football-manager/
├── Cargo.toml
└── src/
    ├── main.rs
    └── game/
        ├── mod.rs
        └── state.rs
```

## Install Rust
The game is built using `Rust`. This will need to be installed:

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Consider running the right command for your shell:

```shell
# For sh/ash/dash/pdksh/zsh
$ . "$HOME/.cargo/env"  
# For tcsh         
$ source "$HOME/.cargo/env.tcsh" 
```

Check if `Rust` has been installed:

```shell
$ rustc --version
```

## Run the Game
Once `Rust` is installed, to run the game:

```shell
# Change directory to the project folder
$ cd rust-football-manager
# Use Cargo to run the game
$ cargo run
```

## Development

### Clone Repository
Clone the repository using `SSL` or `HTTPS`:

```shell
# Clone using SSH
$ git clone git@github.com:hackdanismo/rust-football-manager.git
# Clone using HTTPS:
$ git clone https://github.com/hackdanismo/rust-football-manager.git
```

### The .gitignore file
Use the following `.gitignore` file for the project:

```
# Rust build output
/target/

# Cargo lockfile
# Keep this ignored only if you're building a library.
# For an application/game, you should normally COMMIT Cargo.lock.
# Cargo.lock

# IDE/editor files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# Local environment/config
.env
.env.*

# Game saves / generated local data
/saves/
/logs/

# Temporary files
*.tmp
*.bak
```