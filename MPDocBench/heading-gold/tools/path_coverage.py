"""path_coverage.py -> every node of every tree has a verdict line in path_review/<cat>.tsv (round 4 full path review) or a RESTORED line in break_review/<cat>.tsv (round 5)."""
import json, glob, os, csv, collections, sys
sys.path.insert(0, os.path.dirname(__file__))
from tree import read, nid
ROOT=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
seen=collections.defaultdict(set); verd=collections.Counter(); bad=[]
for f in glob.glob(os.path.join(ROOT,'path_review/*.tsv')):
    for r in csv.DictReader(open(f), delimiter='\t'):
        v=(r.get('verdict') or '').strip(); verd[v]+=1
        if v not in ('OK','FIXED','ADDED','REMOVED'): bad.append((r.get('doc'),r.get('node'),v))
        if v!='REMOVED':
            if str(r.get('node')).strip()=='-': continue
            try: seen[r['doc']].add(int(str(r['node']).lstrip('#')))
            except: bad.append((r.get('doc'),r.get('node'),'node?'))
for f in glob.glob(os.path.join(ROOT,'break_review/*.tsv')):  # round 5: restored nodes are logged with the breaks
    for r in csv.DictReader(open(f), delimiter='\t'):
        if r.get('kind')=='RESTORED':
            try: seen[r['doc']].add(int(str(r['box']).lstrip('#'))); verd['RESTORED']+=1
            except: bad.append((r.get('doc'),r.get('box'),'box?'))
miss=[]; total=0
for f in sorted(glob.glob(os.path.join(ROOT,'trees/*.json'))):
    d=os.path.basename(f)[:-5]
    for n in read(d)['nodes']:
        total+=1
        if nid(n) not in seen[d]: miss.append((d,nid(n)))
print('nodes',total,'| with verdict',total-len(miss),'| missing',len(miss),'| verdicts',dict(verd),'| bad lines',len(bad))
for m in miss[:30]: print('MISSING',*m)
for b in bad[:30]: print('BAD',*b)
