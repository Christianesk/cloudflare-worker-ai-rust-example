# cloudflare-worker-ai-rust-example

Add worker ai functionality on cloudflare worker rust with mcp server tools.

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
        "message": "Show me the list of users from Colombia."
      }'
```

## Response
```text
Here is the list of users from United States:

- **Name:** John Doe  
- **Country:** United States

We found **1 user** matching your request.
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