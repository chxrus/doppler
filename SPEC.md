# Application Specification (MVP) for macOS / Windows with Chat + STT + TTS

## 1. Product Goal

A cross-platform desktop application (macOS + Windows) that lets a user quickly interact with an LLM in two ways:

1. **Ask (text mode)**  
   The user enters a text prompt and receives a response from the model (initially via the Gemini API).

2. **Record / Stop (voice mode)**  
   The user starts voice recording, the application converts speech to text (STT), then sends the recognized text to the LLM and displays the response.

Additionally, the application uses **TTS** (text-to-speech for responses) and supports a **discreet / non-intrusive display mode** (overlay, always-on-top, click-through, opacity/position control).

---

## 2. References and Design Anchors

### 2.1 Handy
Used as a reference for STT and desktop app architecture:
- cross-platform support
- global hotkeys
- start hidden
- Tauri + React/TypeScript + Rust

Repository: `https://github.com/cjpais/Handy`

### 2.2 cheating-daddy
Used as a reference for overlay UX:
- transparent window
- always-on-top
- click-through
- hotkeys
- Gemini API integration

Repository: `https://github.com/sohzm/cheating-daddy`

---

## 3. MVP Scope

### 3.1 Supported Platforms
- **macOS**
- **Windows 10/11**

### 3.2 Operating Modes
1. **Ask**
   - Text input
   - Send to Gemini API
   - Receive and display response
   - (Optional) Speak response via TTS

2. **Record / Stop**
   - Record voice
   - Convert to text (STT)
   - Display recognized text
   - Send to Gemini API
   - Display and speak response

### 3.3 Overlay / "Hidden" Mode
- always-on-top
- adjustable opacity
- click-through
- show/hide via hotkey
- move window via hotkeys
- hide to tray/menu bar

---

## 4. Functional Requirements

## 4.1 FR-01. Gemini API Key Setup and Storage
- The user enters a **Gemini API key** in settings.
- The key is stored locally in secure OS storage:
  - macOS: Keychain
  - Windows: Credential Manager / DPAPI
- A test request is executed when saving.
- The key must never be logged in plain text.

---

## 4.2 FR-02. Ask Mode (Text Chat)
- Prompt input field.
- Send by `Send` button or `Enter`.
- UI states:
  - `idle`
  - `sending`
  - `processing`
  - `done`
  - `error`
- Response is displayed in the chat area.
- Current session history is stored in memory (MVP).
- (Optional) Local history persistence.

---

## 4.3 FR-03. Record / Stop Mode (Voice)
- `Record` starts microphone capture.
- `Stop` ends recording.
- STT is executed after stop.
- Recognized text is shown to the user.
- The user can:
  - send as-is
  - edit
  - cancel
- After confirmation, the text is sent to Gemini API.
- The response is displayed in the UI.

---

## 4.4 FR-04. STT (Speech-to-Text)
An STT pipeline is required to convert voice into text.

### MVP Implementation Options
1. **Integrate Handy as an external process / CLI**
2. **Build a custom STT module** (inspired by Handy’s architecture)
3. **Temporary cloud STT fallback** (if local STT is not ready)

### STT Requirements
- Input device selection
- Recognition language selection
- Returns text and execution status
- Handles microphone / recognition errors

---

## 4.5 FR-05. TTS (Text-to-Speech)
- `Speak` button plays the latest response.
- `Stop Speak` button stops playback.
- TTS settings:
  - voice
  - speed
  - volume
  - auto-play response (on/off)

### MVP Implementation
- Local system TTS engines:
  - macOS: `say` / AVSpeechSynthesizer
  - Windows: SAPI / Windows Speech API

---

## 4.6 FR-06. Overlay / Non-Intrusive Mode
A display mode shown above other windows with configurable interactivity.

### Capabilities
- `Always on top`
- `Click-through` (window does not capture clicks)
- Opacity adjustment (for example, 20–100%)
- Quick show/hide
- Window movement via global hotkeys
- Compact mode (minimal UI)

---

## 4.7 FR-07. Global Hotkeys
Minimum required set:
- `Toggle App` — show/hide application
- `Start/Stop Recording` — start/stop voice recording
- `Toggle Click-through` — switch overlay interactivity
- `Move Window` — move window
- `Send Ask` — send current text
- `Cancel Request` — cancel current request

---

## 4.8 FR-08. Error Handling and States
The app must display clear user-facing errors:
- No microphone access
- Invalid Gemini API key
- No network / timeout
- STT error
- TTS error
- Hotkey registration error

---

## 4.9 FR-09. Application Settings
### Settings Sections
1. **General**
   - Start at OS login
   - Start hidden
   - UI language

2. **LLM (Gemini)**
   - API key
   - Model selection
   - Temperature (optional)

3. **STT**
   - Input device
   - Recognition language
   - Recording mode (record/stop or push-to-talk)

4. **TTS**
   - Voice
   - Speed
   - Volume
   - Auto-speak

5. **Overlay**
   - Opacity
   - Always-on-top
   - Click-through
   - Window size

6. **Hotkeys**
   - Assign/reassign shortcuts

7. **Diagnostics**
   - Microphone check
   - TTS check
   - Gemini API check

---

## 5. Non-Functional Requirements

## 5.1 NFR-01. Performance
- App startup time: up to 3 seconds (without loading heavy local models)
- UI response time: < 100 ms
- STT latency after `Stop`: target 1–2 seconds (depends on implementation)
- The UI must not freeze during network requests

---

## 5.2 NFR-02. Reliability
- The app must not crash when:
  - microphone is unavailable
  - API key is invalid
  - Gemini API request fails
  - STT/TTS fails
- All errors must be handled and shown to the user

---

## 5.3 NFR-03. Security
- API key stored in OS-secure storage
- Sensitive data excluded from logs
- “Do not save history” (privacy mode) option
- Minimize external dependencies for local STT/TTS

---

## 5.4 NFR-04. UX
- All primary actions are accessible via hotkeys
- Overlay does not interfere with work in other apps
- Minimal steps for the “ask by voice” flow

---

## 6. Architectural Specification (Recommended)

## 6.1 Technology Stack
### Recommended Option
- **Desktop shell:** Tauri
- **Frontend:** React + TypeScript
- **Backend / system layer:** Rust
- **LLM:** Gemini API (HTTP)
- **STT:** local STT (Handy-inspired / custom module)
- **TTS:** OS system engines
- **Storage:** local settings + secure key storage

### Why Tauri
- Smaller footprint than Electron
- Better native integration
- Close to Handy’s architecture (easier reuse of design ideas)

---

## 6.2 Logical Modules

### 1) UI Module
- `ChatView`
- `OverlayView`
- `SettingsView`
- `DiagnosticsView`

### 2) Application Core
- `ApplicationStateStore`
- `SessionManager`
- `CommandBus`

### 3) LLM Module
- `GeminiClient`
- `PromptBuilder`
- `ChatSessionService`

### 4) STT Module
- `SpeechRecorder`
- `SpeechToTextProvider`
- `TranscriptionService`

### 5) TTS Module
- `TextToSpeechProvider`
- `SpeechPlaybackController`

### 6) OS Integration Module
- `GlobalHotkeyManager`
- `WindowOverlayController`
- `TrayMenuController`
- `PermissionsChecker`

### 7) Security Module
- `SecureKeyStorage`
- `LogSanitizer`

---

## 7. User Flows

## 7.1 Flow A — Ask (Text)
1. The user opens the application
2. Enters a text prompt
3. Presses `Send` / `Enter`
4. The app sends the request to Gemini API
5. Receives the response
6. Displays the response in the UI
7. (Optional) Speaks the response via TTS

---

## 7.2 Flow B — Record / Stop (Voice)
1. The user presses `Record` (or a hotkey)
2. Microphone recording starts
3. The user speaks the request
4. Presses `Stop`
5. STT runs
6. Recognized text is shown to the user
7. The user edits the text if needed
8. Confirms sending
9. The request is sent to Gemini API
10. The response is displayed
11. TTS speaks the response (if enabled)

---

## 7.3 Flow C — Quick Hide / Overlay
1. The user presses `Toggle App`
2. The window is hidden/shown
3. `Click-through` is toggled if needed
4. The window remains above other apps if `Always-on-top` is enabled

---

## 8. UI Specification

## 8.1 Main Window (Full Mode)
### Screen Composition
- Top bar:
  - status (`Idle`, `Recording`, `Processing`, `Speaking`)
  - settings button
- **Ask** block:
  - multiline text input
  - `Send` button
- **Record** block:
  - `Record / Stop` button
  - recording indicator
  - recording timer
- Response block:
  - response text
  - `Speak`, `Stop Speak`, `Copy` buttons
- Bottom bar:
  - hotkey hints
  - app version

---

## 8.2 Overlay Window (Compact Mode)
### Screen Composition
- Semi-transparent container
- Compact status
- `Record / Stop` button
- Minimal Ask input
- Latest response area
- `Click-through` toggle
- Minimal controls (hide / settings)

---

## 8.3 Settings Window
### Tabs
- `General`
- `Gemini`
- `STT`
- `TTS`
- `Overlay`
- `Hotkeys`
- `Diagnostics`

---

## 9. Internal Module Interfaces (Contracts)

## 9.1 Chat Provider Interface
```ts
interface ChatProvider {
  sendMessage(request: ChatRequest): Promise<ChatResponse>;
  streamMessage?(
    request: ChatRequest,
    onChunk: (chunk: string) => void
  ): Promise<void>;
}
```

### `ChatRequest`
- `messages: Message[]`
- `systemPrompt?: string`
- `temperature?: number`
- `language?: string`

### `ChatResponse`
- `text: string`
- `usage?: TokenUsage`
- `raw?: unknown`

---

## 9.2 STT Provider Interface
```ts
interface SpeechToTextProvider {
  transcribe(audioInput: AudioInput): Promise<TranscriptionResult>;
}
```

### `TranscriptionResult`
- `text: string`
- `language?: string`
- `confidence?: number`
- `durationMilliseconds?: number`

---

## 9.3 TTS Provider Interface
```ts
interface TextToSpeechProvider {
  speak(text: string, options: TtsOptions): Promise<void>;
  stop(): Promise<void>;
  listVoices(): Promise<Voice[]>;
}
```

---

## 10. OS Permissions and System Constraints

## 10.1 macOS
Potential permissions:
- **Microphone** — required for voice recording
- **Accessibility** — may be required for global hotkeys/window control
- **Screen Recording** — not required for MVP (only if screen context is added later)

---

## 10.2 Windows
Potential permissions:
- **Microphone**
- Additional rights for some system integrations (depending on hotkey/overlay implementation)

---

## 11. Diagnostics and Logging

## 11.1 Logs
- Local log file
- Log levels:
  - `error`
  - `warn`
  - `info`
  - `debug`
- Sensitive data masking:
  - API key
  - private text content (configurable)

---

## 11.2 Diagnostics Screen
### Checks
- Microphone access
- TTS functionality
- Gemini API connectivity
- Global hotkey registration
- System permission status

---

## 12. Risks and Constraints

## 12.1 Handy Integration
- Handy is a standalone app/project, not a ready-to-embed library.
- Integration via external process may be unstable.
- A more reliable path is to reuse Handy’s architectural ideas and implement a custom STT layer.

---

## 12.2 Overlay / Click-through
- Behavior differs between macOS and Windows.
- Window focus, transparency, and input handling may vary by OS.

---

## 12.3 Global Hotkeys
- Conflicts with system and user-defined shortcuts are possible.
- A UI for remapping and conflict checks is required.

---

## 12.4 TTS
- Voice quality and available voices depend on the OS.
- Different system APIs require OS-specific adapters.

---

## 13. Implementation Plan (Phases)

## 13.1 Phase 1 — MVP
- Basic Tauri app skeleton
- Main Window + Settings
- Gemini API integration (Ask mode)
- Microphone recording (Record/Stop)
- Basic STT (any stable working path)
- Basic TTS
- Global hotkeys (minimum set)
- Always-on-top + opacity

---

## 13.2 Phase 2
- Full `Click-through`
- Hotkey reassignment
- Local session history
- Improved local STT
- Gemini streaming responses

---

## 13.3 Phase 3
- Improved compact overlay
- Prompt profiles (for example: “code”, “translate”, “explain”)
- Advanced diagnostics
- UX and stability improvements

---

## 14. Acceptance Criteria

The application is considered MVP-ready if:

1. It runs on **macOS** and **Windows**.
2. The user can save a **Gemini API key** and receive a test response.
3. **Ask** mode works reliably:
   - text input
   - send
   - receive response
4. **Record/Stop** mode works:
   - voice recording
   - speech-to-text conversion
   - send to chat
   - receive response
5. The response can be spoken using **TTS**.
6. Overlay mode works:
   - always-on-top
   - opacity
   - quick show/hide
7. At least 3 global hotkeys are available:
   - `Toggle App`
   - `Start/Stop Recording`
   - `Toggle Click-through` (or equivalent overlay interactivity toggle)

---

## 15. Recommended Technical Direction

### What to take from the references
- From **cheating-daddy**:
  - overlay UX patterns
  - click-through
  - hotkeys
  - overall “on-top” interaction approach

- From **Handy**:
  - Tauri architecture
  - cross-platform desktop tooling approach
  - hidden startup and hotkey handling patterns
  - STT-oriented UX

### What not to do
- Do not clone the reference app 1:1 without adaptation.
- Do not block the MVP on a fragile integration path if Handy-as-process turns out unreliable.

### Final Recommendation
Build the app with **Tauri + React + TypeScript + Rust**, using:
- **Gemini API** for chat
- **local STT** (or a temporary fallback)
- **system TTS**
- **overlay features** isolated in an OS integration module

---

## 16. Future Enhancements (Post-MVP)
- Real-time streaming responses
- Conversation history and search
- Prompt presets
- Multiple LLM providers
- Push-to-talk mode
- Automatic speech language detection
- Export history to Markdown / TXT
