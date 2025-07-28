import glob
import json
import os

with open('coordinates.json') as f:
    entries = json.load(f)

ids = set(entries.keys())
used = set()
for path in glob.glob('sprites/*.jpg'):
    i = path.removeprefix('sprites/').removesuffix('.jpg')
    if i not in ids:
        os.remove(path)
