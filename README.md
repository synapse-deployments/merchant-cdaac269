# Synapse Shopify Extensions

This repository contains all Shopify extensions for merchant `cdaac269-9d01-4104-99d9-ebcfeaa7da0c`, managed by [Synapse](https://synapsebuilder.org).

## Structure

```
.
├── shopify.app.toml           # App configuration
├── package.json               # Root dependencies
└── extensions/                # All extensions
    ├── checkout-*/            # Checkout UI extensions
    ├── admin-*/               # Admin UI extensions
    └── function-*/            # Backend functions
```

## Deployment

This app automatically deploys to Fly.io when changes are pushed to `main` branch.

### Manual Deployment

```bash
fly deploy
```

## Adding Extensions

Extensions are automatically added by Synapse. Do not manually edit this repository.

---

Managed by [Synapse](https://synapsebuilder.org) • [Documentation](https://docs.synapsebuilder.org)
