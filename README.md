# Outer Wilds progress tracker

This site allows you to view and share your progress in game

Example (**SPOILERS!!**): [ship log after completing tutorial](https://istudyatuni.github.io/ow-tracker/#save=AAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPxAAAAA=)

### Features

- Full in-game translations for cards and rumors
- Upload your save and view the same Ship Log as in the game
	- *There are still some bugs lurking here*
- Share a save via the link
- View full map
- Hide categories
- Hide spoilers
- Categories not yet found in the save are hidden
- Category names are marked as spoilers if certain facts are not yet found in the save
- View all unexplored facts highlighted

### Known problems

- Sometimes "more to explore" are hidden
- "Spoiler" font doesn't work for Chinese, Japanese and Korean
- Arrowheads are shifted from center in some places
- Map doesn't center itself correctly when some categories are hidden

## Develop

### Extract translations and entries data

You need Rust [installed](https://rustup.rs)

```sh
just extract-translations
```

### Run frontend

```sh
# optional, to get yarn and nodejs
nix develop 

cd frontend && yarn install
just dev
```

## Contributing

If you notice that your save is not rendered correctly, please leave a comment in [this issue](https://github.com/istudyatuni/ow-tracker/issues/1)

### UI translation

You can help translate the project's UI into your language. English file is located in [`frontend/public/translations/ui/english.ftl`](frontend/public/translations/ui/english.ftl). For more info about the file format see [projectfluent.org](https://projectfluent.org)

1. Copy `english.ftl` to the file with id of your language. Currently the same id's are used as in the game. See list of ids in [`frontend/src/lib/language.js`](./frontend/src/lib/language.js)
2. After translation, comment out the line with your language in the `frontend/src/lib/language.js` in the `language_to_code()` function

## Acknowledgements

- Images and cards positions are taken from [outerwilds.ventures](https://outerwilds.ventures)
