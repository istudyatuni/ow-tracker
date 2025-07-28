import json

# game's save file
with open('test-save.json') as f:
    facts = json.load(f)['shipLogFactSaves']

with open('save_keys.json', 'w') as f:
    json.dump(sorted(list(facts.keys())), f, indent=2)
