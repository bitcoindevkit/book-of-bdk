default:
  just --list

# Build the website
build:
  uv run zensical build

# Serve the site locally
serve:
  uv run zensical serve
