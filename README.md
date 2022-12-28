# msm

Minecraft Save Manager (msm) is a simple GUI tool which helps you to manage a set of Minecraft worlds, backing up a new save of the world each time you shut down a server session.

## Dev Workflow

### Run App
`npm run tauri dev` (from project root)

### Generate Stylesheet

`npx tailwindcss -i ./src/style.css -o ./dist/output.css --watch` (from project root)

### Format Rust

`cargo +nightly fmt` (from `src-tauri`)

### Format Frontend

`npx prettier --write .` (from `src`)
