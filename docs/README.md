# A2A Protocol Rust Implementation - Documentation

This directory contains the mdBook documentation for the a2a-rs project.

## Building Locally

Install mdBook:

```bash
cargo install mdbook --version 0.4.40
```

Build and serve the documentation:

```bash
cd docs
mdbook serve --open
```

The documentation will be available at http://localhost:3000

## Building for Production

```bash
cd docs
mdbook build
```

The output will be in the `book/` directory.

## Deployment

The documentation is automatically deployed to GitHub Pages when changes are pushed to the `main` branch.

View the live documentation at: https://x0f5c3.github.io/a2a-rs/

## Structure

- `src/` - Markdown source files
  - `introduction.md` - Main landing page
  - `getting-started/` - Installation and quick start guides
  - `features/` - Feature documentation
  - `integration/` - Integration guides for custom implementations
  - `api/` - API reference
  - `advanced/` - Advanced topics
  - `contributing/` - Contributing guidelines
- `book.toml` - mdBook configuration
- `book/` - Generated output (not committed to git)

## Contributing

To add or update documentation:

1. Edit the markdown files in `src/`
2. Test locally with `mdbook serve`
3. Commit and push to trigger automatic deployment

## Links

- **Live Docs**: https://x0f5c3.github.io/a2a-rs/
- **Repository**: https://github.com/x0f5c3/a2a-rs
- **A2A Protocol**: https://github.com/a2aproject/A2A
