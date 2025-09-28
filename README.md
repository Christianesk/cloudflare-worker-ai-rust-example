# cloudflare-worker-ai-rust-example

Add worker ai functionality on cloudflare worker rust.

# Local Test

Run command
```cmd
npx wrangler dev --remote
```

# Example

This document provides an example of how to make a `POST` request to an API that accepts a message in JSON format and returns a generated response along with token usage information.

## Making a `curl` Request

```bash
curl -X POST http://localhost:8787 \
  -H "Content-Type: application/json" \
  -d '{
        "message": "Say hello world in five different languages"
      }'
```

## Response
```json
{
    "response": "Here are \"Hello, World!\" in five different languages:\n\n1. **Spanish:**\n\"Hola, mundo\"\n2. **French:**\nBonjour, monde\"\n3. **German:**\nHallo Welt\"\n4. **Italian:**\nCiao mondo\"\n5. **Japanese:**\nすみーです\"",
    "usage": {
        "prompt_tokens": 18,
        "completion_tokens": 64,
        "total_tokens": 82
    }
}
```

# Deploy
```cmd
npx wrangler deploy
```

# Info Resources
- [Cloudflare Workers](https://developers.cloudflare.com/workers/)
- [Cloudflare Workers Rust SDK](https://developers.cloudflare.com/workers/languages/rust/)
- [Cloudflare Workers AI](https://developers.cloudflare.com/workers-ai/get-started/workers-wrangler/)
- [Cloudflare Workers AI Video tutorial](https://www.youtube.com/watch?v=cK_leoJsBWY)