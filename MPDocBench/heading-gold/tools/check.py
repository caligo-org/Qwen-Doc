"""check.py <doc> | --all   Validate trees/<doc>.json (schema v3, see tools/tree.py) and print its outline.
Checks: boxes exist and appear only once in the file; nodes in reading order (page of the first box); every parent is
'root' or an EARLIER node with a smaller level and is the parent the levels give in reading order (nearest earlier node
with a smaller level); a union's boxes are distinct; `continues` points to an earlier node whose section was closed by
something in between; every repeat points to a node whose section is still open at the repeat; every gold title is in
a node, a repeat or not_headings; every node, repeat, not_heading and considered entry has a non-empty `why`.
Breaks: `before` is an existing box that is not a heading box; `closes` is a node whose section is open there; the
next node after a break is not deeper than the closed node (else the levels would reopen it); a repeat after a break
must point to a section the break left open. `starts` (optional on a node): an existing non-heading box at or before
the node's own box, after the previous node's section start; section starts must be distinct (content goes to the greatest section start at or before it). `places`: a non-heading box assigned to a node.
Exit 1 on errors."""
import sys, os, json, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import gold, gold_tree, TREES
from tree import read, nid, start, spos
def check(doc, quiet=False):
    r=gold()[doc]; by={e['anno_id']:e for e in r['layout_dets']}; errs=[]
    raw=json.load(open(os.path.join(TREES,doc+'.json')))
    if raw.get('schema')!=3: errs.append('schema must be 3')
    t=read(doc); nodes=t['nodes']
    seen={}
    def take(b,where):
        if b not in by: errs.append(f'#{b} does not exist ({where})')
        elif b in seen: errs.append(f'#{b} used twice ({seen[b]} and {where})')
        seen[b]=where
    for n in nodes:
        if not n.get('boxes'): errs.append('node without boxes'); continue
        for b in n['boxes']: take(b,f'node #{nid(n)}')
    for x in t['repeats']:
        for b in x['boxes']: take(b,f'repeat of #{x["of"]}')
    for x in t['not_headings']: take(x['box'],'not_headings')
    for x in t['considered']: take(x['box'],'considered')
    lv={}; stack=[(0,'root')]; lastpage=0; ids=set()
    for n in nodes:
        if not n.get('boxes') or any(b not in by for b in n['boxes']): continue
        a=nid(n); L=n.get('level'); ids.add(a)
        if not isinstance(L,int) or L<1: errs.append(f'#{a} bad level {L}'); continue
        pg=by[a]['page_id']
        if pg<lastpage: errs.append(f'#{a} (p{pg}) listed after a node on p{lastpage}: nodes must be in reading order')
        lastpage=max(lastpage,pg)
        while stack[-1][0]>=L: stack.pop()
        if n.get('parent')!=stack[-1][1]: errs.append(f"#{a} parent {n.get('parent')} but levels in reading order give {stack[-1][1]}")
        c=n.get('continues')
        if c is not None:
            if c not in lv: errs.append(f'#{a} continues #{c}, which is not an earlier node')
            elif any(s[1]==c for s in stack): errs.append(f'#{a} continues #{c} but its section is still open: make it a repeat')
        if not (n.get('text') or '').strip(): errs.append(f'#{a} empty text')
        if not (n.get('why') or '').strip(): errs.append(f'#{a} missing why')
        lv[a]=L; stack.append((L,a))
    for x in t['repeats']:
        if x['of'] not in ids: errs.append(f"repeat {x['boxes']} of #{x['of']}, which is not a node"); continue
        if not x['boxes'] or x['boxes'][0] not in by: continue
        pos=(by[x['boxes'][0]]['page_id'],x['boxes'][0]); st=[]
        for n in nodes:
            a=nid(n)
            if a not in by or (by[a]['page_id'],a)>pos: break
            while st and st[-1][0]>=n['level']: st.pop()
            st.append((n['level'],a))
        if x['of'] not in [s[1] for s in st]: errs.append(f"repeat {x['boxes']} of #{x['of']}: that section is not open there (make it a node with continues)")
        if not (x.get('why') or '').strip(): errs.append(f"repeat {x['boxes']} missing why")
    for x in t['not_headings']+t['considered']:
        if not (x.get('why') or '').strip(): errs.append(f"#{x['box']} missing why")
    def pos(b): return (by[b]['page_id'],b)
    # Content placement: a content box belongs to the node whose section start (page, box id) is the greatest one at
    # or before it, minus what breaks close, plus `places`. Section starts must be distinct; they need not follow the
    # node order (a box read before its own section's heading can be listed in the tree where it belongs).
    okn=[n for n in nodes if all(b in by for b in n['boxes'])]
    for n in okn:
        st0=n.get('starts')
        if st0 is not None:
            if st0 not in by: errs.append(f'#{nid(n)} starts #{st0}: no such box'); continue
            if pos(st0)>pos(nid(n)): errs.append(f'#{nid(n)} starts #{st0}, which comes after the heading')
    okn=[n for n in okn if n.get('starts') is None or n['starts'] in by]
    S=sorted((spos(n,by),n) for n in okn) if okn else []
    S=[(sp,n) for sp,n in S]
    for i in range(1,len(S)):
        if S[i][0]==S[i-1][0]: errs.append(f'#{nid(S[i][1])} and #{nid(S[i-1][1])} start their sections at the same box')
    heads={b for n in nodes for b in n['boxes']}
    par={nid(n):n.get('parent') for n in nodes}
    def owner(p):
        o=None
        for sp,n in S:
            if sp<=p: o=n
            else: break
        return o
    def chain(n):
        out=[]; a=nid(n) if n else None
        while a not in (None,'root'): out.append(a); a=par.get(a)
        return out[::-1]
    BR=sorted([x for x in t['breaks'] if x.get('before') in by], key=lambda x: pos(x['before']))
    def open_at(p, upto=None):
        o=owner(p); ch=chain(o)
        for y in BR:
            if y is upto or pos(y['before'])>p: break
            if owner(pos(y['before'])) is o and y['closes'] in ch: ch=ch[:ch.index(y['closes'])]
        return ch
    for x in t['breaks']:
        b=x.get('before'); c=x.get('closes')
        if b not in by: errs.append(f'break before #{b}: no such box'); continue
        if b in heads: errs.append(f'break before #{b}: that box is a heading (a heading already closes by its level)')
        if not (x.get('why') or '').strip(): errs.append(f'break before #{b} missing why')
        ch=open_at(pos(b), upto=x)
        if c not in ch: errs.append(f'break before #{b} closes #{c}, which is not open there'); continue
        nxt=next((n for sp,n in S if sp>pos(b)), None)
        if nxt is not None and nxt['level']>lv.get(c,0) and owner(pos(b)) is not None and nxt.get('parent') in ch[ch.index(c):]:
            errs.append(f'break before #{b} closes #{c} (L{lv.get(c)}) but the next node #{nid(nxt)} is L{nxt["level"]} inside it, so its levels reopen #{c}')
    ids_all={nid(n) for n in nodes}
    for x in t['places']:
        b=x.get('box')
        if b not in by: errs.append(f'place #{b}: no such box'); continue
        if b in heads: errs.append(f'place #{b}: that box is a heading')
        if x.get('in') not in ids_all: errs.append(f"place #{b} in #{x.get('in')}: not a node")
        if not (x.get('why') or '').strip(): errs.append(f'place #{b} missing why')
    gt=gold_tree(r)
    miss=[a for a in gt if a not in seen]
    if miss: errs.append(f'gold titles not accounted for (node box, repeat or not_headings): {miss}')
    if not quiet:
        for n in nodes:
            a=nid(n); ex=(f" ∪{n['boxes'][1:]}" if len(n['boxes'])>1 else '')+(f" cont#{n['continues']}" if n.get('continues') is not None else '')
            print('  '*(n.get('level',1)-1)+f"L{n.get('level')} #{a} p{by[a]['page_id'] if a in by else '?'}{ex} {n.get('text','')[:70]!r}")
        for x in t['repeats']: print(f"  repeat {x['boxes']} of #{x['of']}")
        for x in t['breaks']: print(f"  break before #{x['before']} closes #{x['closes']}")
        for n in nodes:
            if n.get('starts') is not None: print(f"  #{nid(n)} section starts at #{n['starts']}")
        print(f"{len(nodes)} nodes, {sum(len(n['boxes'])>1 for n in nodes)} unions, {len(t['repeats'])} repeats, {len(t['not_headings'])} not_headings, {len(t['considered'])} considered, {len(t['breaks'])} breaks, gold titles {len(gt)}")
    for e in errs: print('ERROR', doc, e)
    return not errs
if __name__=='__main__':
    if sys.argv[1]=='--all':
        fs=sorted(glob.glob(os.path.join(TREES,'*.json'))); ok=sum(check(os.path.basename(f)[:-5],quiet=True) for f in fs)
        print(f'{ok}/{len(fs)} trees valid'); sys.exit(0 if ok==len(fs) else 1)
    sys.exit(0 if check(sys.argv[1]) else 1)
