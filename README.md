# Clear List

## Getting started - Development

### Setting up environments

> [!NOTE]
> This will only need to be done the first time on a local development
> If this is already completed, you may skip to the next section

Copy and set `.env` files for api, auth, and app server with development environments

```bash
cp apps/api/.env.template apps/api/.env
cp apps/auth/.env.template apps/auth/.env
cp apps/app/.env.local.template apps/app/.env.local
```

If development proxy is used:

```bash
cp apps/dev-proxy/.env.template apps/dev-proxy/.env
```

Ensure to fill out all environment variables to local setup.

### Starting development servers

```bash
cd apps/api && cargo run
cd apps/auth && npm run dev
cd apps/app && npm run dev
```

If development proxy is used:

```bash
cd apps/dev-proxy && cargo run
```

## Roadmap

- [ ] Projects and Area Support (matches Things 3)
- [ ] Kanban board (matches other project management apps)
- [ ] Integrated Calendar
- [ ] CalDAV support
