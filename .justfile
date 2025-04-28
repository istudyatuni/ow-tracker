dev:
	cd frontend && yarn dev

build:
	cd frontend && yarn build

extract-translations:
	cargo r --release --package tr-extractor -- --write -vv
