<div align="center">
  <img src="static/logo.png" alt="Doppler Logo" width="200"/>
  
  # Doppler
  
  **Voice-First AI Assistant for Desktop**
  
  A cross-platform desktop application for seamless interaction with LLMs through text and voice.
  
  [![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
  [![SvelteKit](https://img.shields.io/badge/SvelteKit-2.0-orange.svg)](https://kit.svelte.dev/)
  [![Rust](https://img.shields.io/badge/Rust-1.70+-red.svg)](https://www.rust-lang.org/)
  [![TypeScript](https://img.shields.io/badge/TypeScript-5.6-blue.svg)](https://www.typescriptlang.org/)
  
</div>

---

## Overview

Doppler is a lightweight, privacy-focused desktop assistant that combines the power of LLMs with voice interaction. Built with Tauri, it offers native performance with a minimal footprint.

### Key Features

- **Text Chat** — Direct text-based interaction with Gemini API
- **Voice Input** — Speech-to-text for hands-free queries
- **Voice Output** — Text-to-speech responses using native OS engines
- **Overlay Mode** — Always-on-top, transparent window with adjustable opacity
- **Global Hotkeys** — Quick access from anywhere on your system
- **Secure Storage** — API keys stored in OS-native secure storage (Keychain/Credential Manager)
- **Cross-Platform** — Native support for macOS and Windows

---

## Technology Stack

### Frontend
- **SvelteKit** — Modern reactive framework with file-based routing
- **TypeScript** — Type-safe development
- **Tailwind CSS** — Utility-first styling
- **Vite** — Fast build tooling

### Backend
- **Rust** — High-performance system integration
- **Tauri** — Lightweight desktop framework
- **cpal** — Cross-platform audio capture
- **tts** — Native text-to-speech
- **reqwest** — HTTP client for API communication

---

## Getting Started

### Prerequisites

- **Node.js** 18+ and npm
- **Rust** 1.70+ (install via [rustup](https://rustup.rs/))
- **System dependencies:**
  - macOS: Xcode Command Line Tools
  - Windows: Visual Studio Build Tools

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd doppler
```

2. Install frontend dependencies:
```bash
npm install
```

3. Install Rust dependencies (automatic on first build)

### Development

Run the development server:
```bash
npm run dev
```

This starts both the Vite dev server and Tauri in development mode with hot-reload.

### Building

Create a production build:
```bash
npm run build
npm run tauri build
```

The compiled application will be in `src-tauri/target/release/bundle/`.

---

## Project Structure

```
doppler/
├── src/                      # Frontend (SvelteKit)
│   ├── lib/
│   │   ├── components/       # Svelte components
│   │   ├── stores/           # State management
│   │   ├── tauri/            # Tauri command wrappers
│   │   └── utils/            # Utility functions
│   └── routes/               # SvelteKit routes
├── src-tauri/                # Backend (Rust)
│   ├── src/
│   │   ├── commands.rs       # Tauri command handlers
│   │   ├── gemini.rs         # Gemini API client
│   │   ├── audio.rs          # Audio recording
│   │   ├── storage.rs        # Secure key storage
│   │   └── models.rs         # Data models
│   └── Cargo.toml
├── static/                   # Static assets
└── package.json
```

---

## Configuration

### API Key Setup

1. Launch the application
2. Navigate to Settings
3. Enter your Gemini API key
4. The key is securely stored in:
   - macOS: Keychain
   - Windows: Credential Manager

### Hotkeys

Default global shortcuts (customizable in Settings):
- `Cmd/Ctrl + Shift + Space` — Toggle app visibility
- `Cmd/Ctrl + Shift + R` — Start/stop recording
- `Cmd/Ctrl + Shift + T` — Toggle click-through mode

---

## Development Guidelines

This project follows strict coding standards. Please review:

- [Rust/Tauri Standards](.kiro/steering/rust-tauri-standards.md)
- [SvelteKit/TypeScript Standards](.kiro/steering/sveltekit-typescript-standards.md)
- [Commit Style Guide](COMMIT_STYLE.md)

### Key Principles

- Keep Tauri commands thin — business logic belongs in application services
- Use TypeScript strictly — avoid `any`, prefer explicit types
- Follow SvelteKit conventions — use `load` functions, form actions, and route files properly
- Isolate Tauri calls behind a frontend boundary layer
- Write readable, top-down code with clear naming

### Validation Commands

Before committing:

**Frontend:**
```bash
npm run check          # Type checking
```

**Backend:**
```bash
cd src-tauri
cargo fmt              # Format code
cargo clippy --all-targets --all-features -- -D warnings
cargo test             # Run tests
```

---

## Architecture

### Frontend Flow
```
Route → Load Function → Page Component → Child Components
```

### Tauri Integration
```
Component Event → Frontend Boundary (commands.ts) → Rust Command → Service Layer
```

### Key Modules

- **Commands** — Thin Tauri command adapters
- **Application** — Business logic and use cases
- **Infrastructure** — External integrations (API, filesystem, audio)
- **State** — Shared application state

---

## Roadmap

### Phase 1 — MVP ✓
- [x] Basic Tauri + SvelteKit setup
- [x] Gemini API integration
- [x] Text chat interface
- [x] Audio recording
- [x] Settings management
- [x] Secure key storage

### Phase 2 — In Progress
- [ ] Speech-to-text (STT)
- [ ] Text-to-speech (TTS)
- [ ] Overlay mode
- [ ] Global hotkeys
- [ ] Click-through support

### Phase 3 — Planned
- [ ] Conversation history
- [ ] Prompt templates
- [ ] Multiple LLM providers
- [ ] Push-to-talk mode
- [ ] Auto language detection

---

## Contributing

Contributions are welcome. Please:

1. Follow the coding standards in `.kiro/steering/`
2. Write clear commit messages (see `COMMIT_STYLE.md`)
3. Run validation commands before submitting
4. Keep changes focused and atomic

---

## License

MIT

---

## Acknowledgments

Inspired by:
- [Handy](https://github.com/cjpais/Handy) — STT and desktop architecture patterns
- [cheating-daddy](https://github.com/sohzm/cheating-daddy) — Overlay UX and Gemini integration

---

<div align="center">
  Built with ❤️ using Tauri, SvelteKit, and Rust
</div>
