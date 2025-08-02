spoilers-font := "https://github.com/istudyatuni/spoilers-ahead-font/raw/refs/heads/master/SpoilersAhead.otf"
spoilers-font-file := "frontend/public/SpoilersAhead.otf"

[private]
@default:
	just --list --unsorted

# run frontend dev server
dev: download-spoilers-font (yarn "dev --host --port 8080")

# build frontend
build: download-spoilers-font minify-json (yarn-prod "build")

# run frontend in prod mode
preview: download-spoilers-font (yarn-prod "preview")

# format frontend code
format: (yarn "format")

[private]
yarn cmd:
	cd frontend && yarn {{cmd}}

[private]
yarn-prod cmd:
	export VITE_BUILD_VERSION=$(git rev-parse HEAD) && cd frontend && yarn {{cmd}}

download-spoilers-font:
	if [[ ! -e "{{ spoilers-font-file }}" ]]; then \
		wget "{{ spoilers-font }}" -O "{{ spoilers-font-file }}"; \
	fi

minify-json:
	#!/usr/bin/env bash
	if [[ "$CI" == "" ]]; then
		echo ignoring minifying in non-ci
		exit
	fi

	set -euo pipefail

	cd frontend
	for p in $(fd .json public); do
		echo minifying $p

		tmp="$(mktemp)"
		jq -c . "$p" > "$tmp"
		mv "$tmp" "$p"
	done

# extract game translations
extract-translations:
	cargo r --release --package tr-extractor -- --write -vv --output-dir=frontend/public
