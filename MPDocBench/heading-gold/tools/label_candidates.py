"""Candidates for the label rule (NOTATIONS 'Labels'). Writes label_candidates.json (written next to the trees):
 A  node that looks like a generic label: same normalised text as a node under a different parent elsewhere in the
    doc, unnumbered, short                                                          -> probably text
 B  node that looks like a lead-in sentence: ends with a colon and is long (> 6 words / > 14 CJK chars) -> probably text
 C  node that is a short label ending with a colon (not A/B)                         -> check specific vs generic
 D  text element (not a node) that is a short own-line label ending with a colon and is followed by content on the same
    page, unnumbered                                                                -> check: specific label = heading"""
import sys, os, json, glob, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import gold, TREES, ROOT
NUM=re.compile(r'^\s*(?:[\(（]?[0-9一二三四五六七八九十ⅠⅡⅢIVX]+[\)）.、．:]|[A-Za-z][.)]\s|第.{1,3}[章节条部])')
COLON=re.compile(r'[:：]\s*$')
def size(t): cjk=len(re.findall(r'[一-鿿]',t)); return cjk if cjk>len(t)/3 else len(t.split())
norm=lambda t:re.sub(r'[\s:：]+','',t or '').lower()
out=[]
for f in sorted(glob.glob(os.path.join(TREES,'*.json'))):
    t=json.load(open(f)); d=t['doc']; r=gold()[d]; by={e['anno_id']:e for e in r['layout_dets']}
    src=r['page_info']['page_attribute']['data_source']
    N=t['nodes']; cnt=collections.defaultdict(set)
    for n in N: cnt[norm(n['text'])].add(n['parent'])
    for n in N:
        txt=n['text']; k=norm(txt)
        if n['level']==1: continue
        kind=None
        if len(cnt[k])>1 and not NUM.match(txt) and size(txt)<=4: kind='A'
        elif COLON.search(txt) and size(txt)>(14 if re.search(r'[一-鿿]',txt) else 6): kind='B'
        elif COLON.search(txt): kind='C'
        if kind: out.append(dict(doc=d,src=src,kind=kind,anno=n['anno'],page=by[n['anno']]['page_id'],level=n['level'],text=txt[:80]))
    taken={n['anno'] for n in N}|{m for n in N for m in n.get('merge',[])}|{x['anno'] for x in t.get('repeats',[])}
    els=sorted([e for e in r['layout_dets'] if e['category_type']=='text_block'],key=lambda e:(e['page_id'],e['anno_id']))
    for i,e in enumerate(els):
        txt=(e.get('text') or '').strip()
        if e['anno_id'] in taken or '\n' in txt or not COLON.search(txt) or NUM.match(txt): continue
        if size(txt)>(10 if re.search(r'[一-鿿]',txt) else 5): continue
        nxt=els[i+1] if i+1<len(els) else None
        if nxt and nxt['page_id']==e['page_id']:
            out.append(dict(doc=d,src=src,kind='D',anno=e['anno_id'],page=e['page_id'],level=None,text=txt[:80]))
json.dump(out,open(os.path.join(ROOT,'label_candidates.json'),'w'),ensure_ascii=False,indent=0)
c=collections.Counter((x['src'],x['kind']) for x in out)
srcs=sorted({x['src'] for x in out})
print('kind A=generic repeated node, B=lead-in node, C=colon label node, D=colon label in text')
for s_ in srcs: print(f"{s_:16}",' '.join(f"{k}:{c[(s_,k)]:3}" for k in 'ABCD'))
print('total',len(out),{k:sum(v for (s,kk),v in c.items() if kk==k) for k in 'ABCD'})
