# Clear List

## Development Guide

1. Copy and set `.env` files for api and auth server with development environment
2. Start servers
    * `cd apps/api && cargo run`
    * `cd apps/auth && npm run dev`
    * `cd apps/app && npm run dev`
3. Start development proxy
    * `cd apps/dev-proxy && cargo run`
