# Tauri + Vanilla

This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


Here are the commands you need to run to test your Tauri inventory application:

## 1. Navigate to the project directory
```bash
cd /home/annan/Documentos/inven-ruby/inven-ruby
```

## 2. Install Node.js dependencies (if not already done)
```bash
npm install
```

## 3. Run the development server (this will start both backend and frontend)
```bash
source "$HOME/.cargo/env" && npm run tauri dev
```

That's it! The `npm run tauri dev` command will:
- Compile the Rust backend
- Start the React frontend development server
- Launch the Tauri desktop application window
- Enable hot reloading for both frontend and backend changes

## Alternative commands for different scenarios:

**To build for production:**
```bash
source "$HOME/.cargo/env" && npm run tauri build
```

**To run just the frontend (without Tauri):**
```bash
npm run dev
```

**To run just the Rust backend tests:**
```bash
cd src-tauri && source "$HOME/.cargo/env" && cargo test
```

The main command you need is **`npm run tauri dev`** - this will start everything and open the desktop application window where you can test all the inventory management features (add, edit, delete products, filter by categories, etc.).