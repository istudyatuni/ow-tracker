[private]
@default:
	just --list --unsorted

# run frontend dev server
dev: (yarn "dev --host --port 8080")

# build frontend
build: (yarn-prod "build")

# run frontend in prod mode
preview: (yarn-prod "preview")

# format frontend code
format: (yarn "format")

[private]
yarn cmd:
	cd frontend && yarn {{cmd}}

[private]
yarn-prod cmd:
	export VITE_BUILD_VERSION=$(git rev-parse HEAD) && cd frontend && yarn {{cmd}}

# extract game translations
extract-translations:
	cargo r --release --package tr-extractor -- --write -vv --output-dir=frontend/public
