# op
> Compact Rust TUI for opening project directories ergonomically

### Screenshots
> screenshots coming soon

### Installation
Step 1: Cloning the repository
```sh
git clone https://github.com/krissemicolon/op.git
cd op
```
Step 2: Configuring the Project Path   
(needs to be done before compilation (included @ comile time))
In `path.txt` must be the absolute path your directory containing all your projects
```sh
cd assets
vim path.txt
```
Step 3: Compiling the program
```sh
cd ..
# ~/.cargo/bin -> op binary
cargo install --path .
```

### Dependencies
    - Cursive
Build Deps:
    - Cargo
