# Development Container Configuration

This directory contains the configuration for GitHub Codespaces development environment.

## What's Included

- **Node.js 20**: Latest LTS version for TypeScript/JavaScript development
- **Shopify CLI**: Pre-installed and ready to use
- **Rust Toolchain**: For building Shopify Functions (if applicable)
- **VS Code Extensions**: Shopify, GraphQL, Rust, and more pre-installed
- **Port Forwarding**: Automatic HTTPS URLs for app preview

## Quick Start

1. **Open in Codespace**: Click "Code" → "Codespaces" → "Create codespace on main"
2. **Wait for Setup**: Container will install dependencies automatically (~60 seconds)
3. **Start Development**: Run `shopify app dev` in the terminal
4. **Preview App**: Click the port forwarding notification to open your app

## Development Commands

```bash
# Start development server with live reload
shopify app dev

# Build Rust functions (if applicable)
cd extensions/your-function
cargo build --target wasm32-unknown-unknown --release

# Deploy to production
shopify app deploy

# Run tests
pnpm test
```

## Live Previews

Port 3000 is automatically forwarded and accessible via HTTPS. When you run `shopify app dev`:

1. Codespace creates a public URL: `https://{codespace-name}-3000.app.github.dev`
2. You can test your app/extensions immediately
3. Changes hot-reload in real-time

## Customization

To modify this environment, edit `.devcontainer/devcontainer.json` and rebuild the container.

## Resources

- [Codespaces Documentation](https://docs.github.com/en/codespaces)
- [Shopify CLI Documentation](https://shopify.dev/docs/api/shopify-cli)
- [DevContainer Specification](https://containers.dev/)
