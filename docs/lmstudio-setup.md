# LM Studio Setup (Windows / Linux / macOS)

This guide explains how to use LM Studio as a local text provider in Doppler.

## Requirements

- Doppler installed and running
- LM Studio installed
- Any chat-capable model downloaded in LM Studio

Doppler defaults:
- Base URL: `http://localhost:1234/v1`
- Model: selected from `GET /v1/models` in settings

## 1. Install LM Studio

Download and install LM Studio:

- <https://lmstudio.ai/>

## 2. Download and load a model

1. Open LM Studio.
2. Download any chat-capable model in the Models tab.
3. Load the model.

## 3. Start Local Server

1. Open the `Local Server` tab in LM Studio.
2. Start the server.
3. Keep it running while you use Doppler.

By default it listens on:

`http://localhost:1234/v1`

## 4. Verify endpoints

Check models:

```bash
curl http://localhost:1234/v1/models
```

Optional chat check:

```bash
curl http://localhost:1234/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "YOUR_MODEL_ID",
    "messages": [{"role":"user","content":"Hello"}],
    "temperature": 0.7
  }'
```

## 5. Configure Doppler

1. Open `Settings -> AI`.
2. Set `Text API Provider` to `LM Studio (local)`.
3. Set `Base URL` to `http://localhost:1234/v1`.
4. Click refresh models.
5. Choose a model from the list.

## Quick troubleshooting

- `LM Studio server is not running / connection refused`
  - Start Local Server in LM Studio.
- Empty model list in Doppler
  - Confirm model is loaded in LM Studio and server is running.
  - Verify Base URL is correct, then refresh models.
- HTTP 404 on requests
  - Ensure Base URL includes `/v1` (for example `http://localhost:1234/v1`).
