"""breadcrumbs.py <doc> | --all [--flags-only]
Prints every breadcrumb (title › … › heading) of a tree and flags paths that look inconsistent:
  NUM   numbering does not nest: child "1.2" under a parent numbered "2", "（一）" under "（二）"'s sibling, "3.1.2" under "3.2"
  SEQ   numbered siblings out of sequence (1, 3) or restarting under the same parent
  DUP   a heading with the same text as its parent or an ancestor
  DEEP  numbered child shallower than its numbered parent (1.1 parent of 1)
  FRONT front/back matter (contents, preface, references, appendix…) nested under a body section
  ROOTS more than one L1 node (allowed only for untitled notes)
Writes review/breadcrumb_flags.json with --all."""
import sys, os, re, json, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import gold, TREES, ROOT
from tree import read, nid
CN='一二三四五六七八九十'
def cn2i(s):
    if not s: return None
    if s=='十': return 10
    if s.startswith('十'): return 10+CN.index(s[1])+1 if len(s)>1 and s[1] in CN else 10
    if '十' in s: a,b=s.split('十'); return (CN.index(a)+1)*10+((CN.index(b)+1) if b else 0)
    return CN.index(s)+1 if s in CN else None
def num(t):
    """(kind, tuple) of a heading's leading number, e.g. ('dot',(3,1)), ('cn',(2,)), ('paren',(1,)), ('ch',(3,))."""
    t=re.sub(r'^[\s"“”\'‘’「『【\[]+','',t)
    m=re.match(r'^(\d+(?:\.\d+)+)(?!\.?\d)',t)            # 3.2 / 3.2.1, followed by anything but more digits
    if m: return ('dot',tuple(int(x) for x in m.group(1).split('.')))
    m=re.match(r'^(\d+)\s*[.、．](?!\d)',t)                    # 3.  3、
    if m: return ('dot',(int(m.group(1)),))
    m=re.match(r'^(\d+)\s*(?=[\u4e00-\u9fff“"「])',t)                # "3 范围", "3研究方法"
    if m and len(m.group(1))<=2 and not re.match(r'^\d+\s*(年|月|日|%|个|家|项|亿|万)',t): return ('dot',(int(m.group(1)),))
    m=re.match(r'^(\d{1,2})\s+(?=[A-Z])',t)                         # "3 Background and Notation" (capitalised word after the number)
    if m and len(t)<90: return ('dot',(int(m.group(1)),))
    m=re.match(r'^([A-Z])\.(\d+(?:\.\d+)*)',t)
    if m: return ('dot',(m.group(1),)+tuple(int(x) for x in m.group(2).split('.')))
    m=re.match(r'^第([一二三四五六七八九十\d]+)([章节部分篇编条课单元讲])',t)
    if m: v=cn2i(m.group(1)) if not m.group(1).isdigit() else int(m.group(1)); return ('ch'+m.group(2),(v,))
    m=re.match(r'^([一二三四五六七八九十]+)[、．.]',t)
    if m: return ('cn',(cn2i(m.group(1)),))
    m=re.match(r'^[（(]([一二三四五六七八九十]+)[）)]',t)
    if m: return ('pcn',(cn2i(m.group(1)),))
    m=re.match(r'^[（(](\d+)[）)]',t)
    if m: return ('pnum',(int(m.group(1)),))
    m=re.match(r'^(chapter|part|section)\s+(\d+(?:\.\d+)*)',t,re.I)
    if m: return (m.group(1).lower(),tuple(int(x) for x in m.group(2).split('.')))
    return None
FRONT=re.compile(r'^(目\s*录|目\s*次|contents|table of contents|前\s*言|引\s*言|序\s*言|preface|foreword|参考文献|references|bibliography|附录|appendix|致谢|acknowledg|index$|索引)',re.I)
def check(doc):
    t=read(doc); N=t['nodes']; by={nid(n):n for n in N}; flags=[]
    roots=[n for n in N if n['parent']=='root']
    if len(roots)>1: flags.append(('ROOTS',nid(roots[1]),f"{len(roots)} top-level nodes: "+' | '.join(r['text'][:25] for r in roots[:4])))
    kids={}
    for n in N: kids.setdefault(n['parent'],[]).append(n)
    def path(n):
        out=[]; x=n
        while True:
            out.append(x)
            if x['parent']=='root' or x['parent'] not in by: break
            x=by[x['parent']]
        return out[::-1]
    for n in N:
        p=by.get(n['parent'])
        anc=path(n)[:-1]
        if any(re.sub(r'\W','',a['text']).lower()==re.sub(r'\W','',n['text']).lower() for a in anc):
            flags.append(('DUP',nid(n),f"same text as an ancestor: {n['text'][:40]}"))
        if p and p['level']>1:
            a,b=num(p['text']),num(n['text'])
            if a and b and a[0]=='dot' and b[0]=='dot':
                if len(b[1])<=len(a[1]): flags.append(('DEEP',nid(n),f"{n['text'][:30]} under {p['text'][:30]}"))
                elif b[1][:len(a[1])]!=a[1]: flags.append(('NUM',nid(n),f"{n['text'][:30]} under {p['text'][:30]}"))
            if a and b and a[0]==b[0] and a[0]!='dot': flags.append(('NUM',nid(n),f"same numbering kind as parent: {n['text'][:30]} under {p['text'][:30]}"))
            if FRONT.match(n['text']) and not FRONT.match(p['text']) and p['parent']!='root' and n['level']>2:
                flags.append(('FRONT',nid(n),f"{n['text'][:30]} nested under {p['text'][:30]} at L{n['level']}"))
    for par,ks in kids.items():
        seq={}
        for k in ks:
            v=num(k['text'])
            if v and len(v[1])>=1 and isinstance(v[1][-1],int): seq.setdefault((v[0],v[1][:-1]),[]).append((v[1][-1],k))
        for key,lst in seq.items():
            vals=[v for v,_ in lst]
            for (v1,k1),(v2,k2) in zip(lst,lst[1:]):
                if v2!=v1+1 and not k2.get('continues'): flags.append(('SEQ',nid(k2),f"{k1['text'][:25]} → {k2['text'][:25]} (siblings)"))
    return t,path,flags
if __name__=='__main__':
    if sys.argv[1]=='--all':
        allf={}
        for f in sorted(glob.glob(os.path.join(TREES,'*.json'))):
            d=os.path.basename(f)[:-5]; _,_,fl=check(d)
            if fl: allf[d]=[{'k':k,'node':i,'msg':m} for k,i,m in fl]
        os.makedirs(os.path.join(ROOT,'review'),exist_ok=True)
        json.dump(allf,open(os.path.join(ROOT,'review','breadcrumb_flags.json'),'w'),ensure_ascii=False,indent=0)
        import collections; c=collections.Counter(x['k'] for v in allf.values() for x in v)
        print(len(allf),'docs with flags',dict(c))
    else:
        t,path,fl=check(sys.argv[1])
        if '--flags-only' not in sys.argv:
            by={e['anno_id']:e for e in gold()[sys.argv[1]]['layout_dets']}
            for n in t['nodes']:
                kids=[m for m in t['nodes'] if m['parent']==nid(n)]
                print(f"#{nid(n)} p{by[nid(n)]['page_id']} L{n['level']}{' leaf' if not kids else ''}: "+' › '.join(x['text'][:45] for x in path(n)))
        for k,i,m in fl: print(f'FLAG {k} #{i}: {m}')
