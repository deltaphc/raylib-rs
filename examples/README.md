# Examples

## Getting started

Follow the instructions on the main README to install raylib.

### Creating a new raylib project

To create a new raylib project, you can use cargo to create a new project folder:

```bash
cargo new --bin <name_of_project_here>
```

Inside, there will be a `src` folder where all the code source files go, and `Cargo.toml`, which holds information about the project.

Edit `Cargo.toml` using a text editor and under `[dependencies]`, add the line:
```
raylib = "0.9"
```

Then to edit your program, edit the file `src/main.rs`. To run the program, run:
```bash
cargo run
```

while in the root folder of your project.
