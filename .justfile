dev:
	cd frontend && yarn dev --host --port 8080

build:
	cd frontend && yarn build

extract-translations:
	cargo r --release --package tr-extractor -- --write -vv --output-dir=frontend/public
