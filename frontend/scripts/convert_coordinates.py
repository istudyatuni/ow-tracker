import json

# https://github.com/nottldr/outer-wilds-ventures/blob/main/src/data/assets/library.json
with open('library_orig.json') as f:
    entries = json.load(f)['entries']

coords = dict()
alt_sprites = dict()
for e in entries:
    i = e['id']

    pos = e['cardPosition']
    coords[i] = [pos['x'], pos['y']]

    if e.get('altSpritePath') != None:
        alt_sprites[i] = e['altSpritePath'].replace('.png', '')

with open('coordinates.json', 'w') as f:
    json.dump(coords, f, sort_keys=True, indent=2)

with open('alt_sprites.json', 'w') as f:
    json.dump(alt_sprites, f, sort_keys=True, indent=2)
