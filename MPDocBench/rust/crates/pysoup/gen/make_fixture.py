"""Select a small regression fixture (tests/data/fixture.jsonl) from the JSONL
cases written by gen/dump_cases.py.

    python gen/make_fixture.py <cases dir> > tests/data/fixture.jsonl
"""
import json
import os
import random
import sys

d = sys.argv[1]
rng = random.Random(5)
plan = [  # (file, max cases, max input length)
    ('adv.jsonl', 220, 4000),
    ('fuzz.jsonl', 80, 1500),
    ('charfuzz.jsonl', 60, 400),
    ('gt.jsonl', 50, 1500),
    ('pred.jsonl', 70, 1500),
    ('merge.jsonl', 15, 4000),
    ('merge_pred.jsonl', 10, 4000),
]
for fn, k, maxlen in plan:
    p = os.path.join(d, fn)
    if not os.path.exists(p):
        continue
    recs = []
    for line in open(p):
        r = json.loads(line)
        if len(json.dumps(r['in'])) <= maxlen:
            recs.append(line.rstrip('\n'))
    # keep every exception case of adv (RecursionError emulation etc.)
    errs = [x for x in recs if '"err"' in x[-60:]][:20]
    pick = rng.sample(recs, min(k, len(recs)))
    for x in dict.fromkeys(errs + pick):
        print(x)
