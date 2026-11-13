"""Tree file access (schema v3). A heading node is a union of one or more boxes:
  {"boxes": [39, 40], "level": 3, "parent": "root" | <node id>, "text": "...", "why": "...", "continues": <node id>?}
  node id = boxes[0]. repeats: [{"boxes": [..], "of": <node id>, "why": ...}]. not_headings: [{"box": id, "why": ...}].
  considered (optional): [{"box": id, "why": ...}]  heading-looking lines that are text (not gold titles).
read() also accepts the v2 schema (anno + merge, repeats {anno, of}, not_headings {anno, note}) and returns v3."""
import os, json
from common import TREES
def nid(n): return n['boxes'][0]
def upgrade(t):
    if t.get('schema')==3: return t
    nodes=[]
    for n in t.get('nodes',[]):
        x={'boxes':[n['anno']]+list(n.get('merge',[])),'level':n['level'],'parent':n['parent'],'text':n['text'],'why':n.get('why') or n.get('note','')}
        if n.get('continues') is not None: x['continues']=n['continues']
        nodes.append(x)
    reps=[{'boxes':[x['anno']],'of':x['of'],'why':x.get('why','')} for x in t.get('repeats',[])]
    nh=[{'box':x['anno'],'why':x.get('why') or x.get('note','')} for x in t.get('not_headings',[])]
    return {'doc':t['doc'],'schema':3,'nodes':nodes,'repeats':reps,'not_headings':nh,'considered':t.get('considered',[])}
def read(doc, tdir=TREES): return upgrade(json.load(open(os.path.join(tdir,doc+'.json'))))
