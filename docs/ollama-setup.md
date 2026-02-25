# Ollama Setup

This guide explains how to run Ollama for Doppler on supported platforms.

## Requirements

- Doppler installed and running
- Internet access to download models
- Enough RAM for your chosen model (for `llama3.2:3b`, at least 8 GB recommended)

Doppler defaults:
- Base URL: `http://localhost:11434`
- Model: `llama3.2:3b`

## Windows

### 1. Install Ollama

1. Download and install Ollama from: <https://ollama.com/download/windows>
2. Finish the installer.
3. Open PowerShell.

### 2. Start Ollama server

Run:

```powershell
ollama serve
```

Keep this terminal open while testing.

### 3. Download a model

In another PowerShell window run:

```powershell
ollama pull llama3.2:3b
```

You can also use other tags (for example, `llama3.1:8b`) if your machine has enough resources.

### 4. Verify Ollama is running

```powershell
curl http://localhost:11434/api/tags
```

If everything is correct, you should get JSON with model tags.

### 5. Configure Doppler

1. Open Doppler -> Settings -> Text provider.
2. Select `Ollama`.
3. Set `Base URL` to `http://localhost:11434`.
4. Set `Model` to `llama3.2:3b` (or another downloaded tag).
5. Click model refresh if needed.

## Linux

### 1. Install Ollama

Run:

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

### 2. Start Ollama server

Run in terminal:

```bash
ollama serve
```

Keep it running while testing.

Optional: run as a background service (systemd-based distros):

```bash
sudo systemctl enable ollama
sudo systemctl start ollama
sudo systemctl status ollama
```

### 3. Download a model

```bash
ollama pull llama3.2:3b
```

### 4. Verify Ollama is running

```bash
curl http://localhost:11434/api/tags
```

You should receive JSON with available model tags.

### 5. Configure Doppler

1. Open Doppler -> Settings -> Text provider.
2. Select `Ollama`.
3. Set `Base URL` to `http://localhost:11434`.
4. Set `Model` to `llama3.2:3b` (or your downloaded model tag).
5. Use model refresh in settings if the list is empty.

## macOS

### 1. Install Ollama

Install via Homebrew:

```bash
brew install ollama
```

If Homebrew is not available, install from:
<https://ollama.com/download/mac>

### 2. Start Ollama server

Run:

```bash
ollama serve
```

Keep it running while testing.

### 3. Download a model

```bash
ollama pull llama3.2:3b
```

### 4. Verify Ollama is running

```bash
curl http://localhost:11434/api/tags
```

You should receive JSON with available model tags.

### 5. Configure Doppler

1. Open Doppler -> Settings -> Text provider.
2. Select `Ollama`.
3. Set `Base URL` to `http://localhost:11434`.
4. Set `Model` to `llama3.2:3b` (or your downloaded model tag).
5. Use model refresh in settings if the list is empty.

## Quick troubleshooting

- `connection refused` on `localhost:11434`: Ollama server is not running. Start it with `ollama serve`.
- Model not found: run `ollama pull <model-tag>` and check via `ollama list`.
- Empty model list in Doppler: verify Base URL, then refresh models in settings.

## Useful commands

```bash
ollama list
ollama pull llama3.2:3b
ollama run llama3.2:3b
```
