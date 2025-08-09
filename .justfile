set shell := ["/usr/bin/env", "bash", "-c"]

spoilers-font := "https://github.com/istudyatuni/spoilers-ahead-font/raw/refs/heads/master/SpoilersAhead.otf"
spoilers-font-file := "frontend/public/SpoilersAhead.otf"
linux-static-target := "x86_64-unknown-linux-musl"
win-target := "x86_64-pc-windows-msvc"
app-config-file-source := "frontend/public/config.json"
app-config-file := "frontend/dist/config.json"

[private]
@default:
	just --list --unsorted

# format all code
format: (yarn "format")
	cargo fmt

# run frontend dev server
run-web: download-spoilers-font write-config-dev (yarn "dev --host --port 8080")

# run frontend in prod mode
run-web-preview: download-spoilers-font (yarn-prod "preview")

[private]
build-prepare: download-spoilers-font minify-json

# build frontend
build-web: build-prepare (yarn-prod "build")

# build frontend in ci
build-web-ci: build-prepare (yarn-prod-ci "build") write-config

[private]
yarn cmd:
	cd frontend && yarn {{cmd}}

[private]
yarn-prod cmd:
	export VITE_BUILD_VERSION=$(git rev-parse HEAD) && just yarn-prod-ci {{cmd}}

[private]
yarn-prod-ci cmd:
	cd frontend && yarn {{cmd}}

# run companion app
run-app:
	cargo r --package ow-tracker-companion || :

# build companion app for windows
build-app-win:
	cargo xwin b --release --package ow-tracker-companion --target={{ win-target }}

# build companion app for linux
build-app:
	cargo b --release --package ow-tracker-companion

# run server
run-server:
	cargo r --package ow-tracker-server

# build server with static linking
build-server:
	@# CARGO_HOME and /tmp/.cargo is used to use local cargo download cache
	docker run --rm -it \
		-v "$(pwd)":/build \
		-w /build \
		--env-file .env \
		--env=CARGO_HOME=/tmp/.cargo \
		-v "$HOME/.cargo":/tmp/.cargo \
		ghcr.io/rust-cross/rust-musl-cross:x86_64-musl \
		cargo build --release \
			--package ow-tracker-server \
			--target={{ linux-static-target }} \
			--config build.rustc-wrapper="''"

# run rust tests
test *args:
	cargo nextest run {{ args }}

# check rust code
check:
	cargo fmt --check
	cargo clippy -- -D clippy::all

# not work
# cargo xwin clippy --package ow-tracker-companion -- -D clippy::all --target={{ win-target }}

# download spoilers font
download-spoilers-font:
	if [[ ! -e "{{ spoilers-font-file }}" ]]; then \
		wget "{{ spoilers-font }}" -O "{{ spoilers-font-file }}"; \
	fi

# minify json in assets
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

[private]
write-config:
	dotenvy just write-config-impl

[private]
write-config-dev:
	mkdir -p frontend/dist
	cp {{ app-config-file-source }} {{ app-config-file }}
	dotenvy just write-config-impl

[private]
write-config-impl:
	sd '\$server_address' "$SERVER_HOST:$SERVER_PORT" {{ app-config-file }}
	sd '\$web_address' "$WEB_ADDRESS" {{ app-config-file }}

# extract game translations
extract-translations:
	cargo r --release --package tr-extractor -- --write -vv --output-dir=frontend/public
