"""coverage.py   Pages never rendered with show.py (review/seen.log) and trees missing or invalid, per category."""
import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import gold, ROOT, TREES
seen=set()
f=os.path.join(ROOT,'review','seen.log')
if os.path.exists(f):
    for l in open(f):
        d,p,_=l.rstrip('\n').split('\t'); seen.add((d,int(p)))
# renders made after the v3 brief but before seen.log existed: recover them from the image file names
import glob, re
t0=float('inf')   # (the relabel used an extra recovery from image files here)
for img in glob.glob(os.path.join(ROOT,'review','*.jpg')):
    m=re.match(r'(.+)_p([\d-]+)(?:_tree)?\.jpg$',os.path.basename(img))
    if m and os.path.getmtime(img)>=t0 and m.group(1) in gold():
        for p in m.group(2).split('-'): seen.add((m.group(1),int(p)))
miss=collections.defaultdict(list); tot=collections.Counter(); notree=[]
for d,r in gold().items():
    c=r['page_info']['page_attribute']['data_source']; n=len(r['page_info']['height']); tot[c]+=n
    if not os.path.exists(os.path.join(TREES,d+'.json')): notree.append(d)
    for p in range(1,n+1):
        if (d,p) not in seen: miss[c].append((d,p))
for c in sorted(tot): print(f"{c:16} pages {tot[c]:4}  unseen {len(miss[c]):4}")
print('docs without a tree:',len(notree))
if '-v' in sys.argv:
    for c in sorted(miss):
        for d,p in miss[c]: print(c,d,p)
