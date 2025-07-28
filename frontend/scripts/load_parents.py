import json

# https://github.com/nottldr/outer-wilds-ventures/blob/main/src/data/assets/entries.json
with open('entries_orig.json') as f:
    entries = json.load(f)

parents = dict()
for astro in entries:
    for e in astro['entries']:
        if "parentId" in e:
            parents[e['id']] = e['parentId']

with open('parents.json', 'w') as f:
    json.dump(parents, f, sort_keys=True, indent=2)
