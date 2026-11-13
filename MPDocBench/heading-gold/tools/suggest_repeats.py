"""suggest_repeats.py <doc>   Lines that repeat an open heading's text (running heads, "(continued)", 续) after your
tree is written: prints candidates as {"boxes": [id], "of": node, "why": ...} for you to check on the page and add
to `repeats` (a running head naming the document or the current section, a "(continued)" heading with nothing in
between). It does not edit the tree."""
import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import gold
from tree import read, nid
FLAG=re.compile(r'[\(（]\s*(continued|cont\.?|cont\'d|续|續|接上页|接上頁)\s*[\)）]|\b(continued|cont\'d)\b|续表|續表',re.I)
key=lambda s:re.sub(r'[\s\W_]+','',FLAG.sub('',s or '')).lower()
doc=sys.argv[1]; r=gold()[doc]; t=read(doc); by={e['anno_id']:e for e in r['layout_dets']}
N=t['nodes']; used={b for n in N for b in n['boxes']}|{b for x in t['repeats'] for b in x['boxes']}|{x['box'] for x in t['not_headings']+t['considered']}
pos=lambda a:(by[a]['page_id'],a)
for e in sorted(r['layout_dets'],key=lambda e:(e['page_id'],e['anno_id'])):
    a=e['anno_id']; k=key(e.get('text'))
    if a in used or len(k)<2: continue
    st=[]
    for n in N:
        if pos(nid(n))>pos(a): break
        while st and st[-1]['level']>=n['level']: st.pop()
        st.append(n)
    for n in st:
        nk=key(n['text']); parts=[key(by[b].get('text')) for b in n['boxes']]
        if (k==nk or k in parts) and by[nid(n)]['page_id']<e['page_id']:
            print(json.dumps({'boxes':[a],'of':nid(n),'why':f"p{e['page_id']} {e['category_type']} repeats '{n['text'][:40]}' while its section is open"},ensure_ascii=False)); break
