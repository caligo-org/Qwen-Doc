"""roundtrip.py -> every tree written as Markdown levels, scored with the unchanged MPDocBench HeadTEDS against
MPDocBench.roletrees.json (built by to_mpdocbench.py). Expect 1.0 on every document."""
import sys, os, json
TK=os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),'..')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import types; _m=types.ModuleType('metrics'); _m.__path__=[os.path.join(TK,'metrics')]; sys.modules['metrics']=_m  # skip the package __init__ (pulls every metric's deps)
from metrics.head_metric import HeadTEDS
from tree import read
ROOT=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
data=json.load(open(os.path.join(ROOT,'MPDocBench.roletrees.json')))
def name(r): return r['page_info']['images_list'][0].split('/')[1]
h=HeadTEDS(); bad=[]; n=0
for r in data:
    d=name(r)
    if not os.path.exists(os.path.join(ROOT,'trees',d+'.json')): continue
    heads={x['anno_id']:x for x in r['layout_dets'] if x['category_type']=='title'}
    rel=[x for x in r['extra']['relation'] if x['relation_type']=='parent_son' and (x['source_anno_id'] in heads or x['source_anno_id']=='root') and x['target_anno_id'] in heads]
    md=['#'*nd['level']+' '+nd['text'].replace('\n',' ') for nd in read(d)['nodes']]
    s,_=h.evaluate(md,[rel,heads]); n+=1
    if s<0.99999: bad.append((d,round(s,4)))
print(n,'docs | not 1.0:',len(bad)); [print(*b) for b in bad]
