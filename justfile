set dotenv-load := false

# List all recipes
default:
    @just --list

# Update vendored nFPM version (version with `v` prefix).
update-vendor version:
    ./update-vendor.sh {{ version }}
