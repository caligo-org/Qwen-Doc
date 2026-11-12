"""check.py <doc> | --all   Validate trees/<doc>.json and print its outline.
Checks: ids exist; nodes in reading order (page order); parent is 'root' or an EARLIER node with a smaller level and is
the parent that levels give in reading order (nearest earlier node with a smaller level); merge ids exist and are not
nodes; continues points to an earlier node whose section was closed by something in between;
every repeat points to a node whose section is still open at the repeat's position; every gold title is a node, a merge or a not_heading. Exit 1 on errors."""
import sys, os, json, glob
from common import gold, gold_tree, TREES
def check(doc, quiet=False):
    r=gold()[doc]; by={e['anno_id']:e for e in r['layout_dets']}; errs=[]
    t=json.load(open(os.path.join(TREES,doc+'.json')))
    if t.get('doc')!=doc: errs.append('doc field mismatch')
    nodes=t.get('nodes',[]); ids=[n['anno'] for n in nodes]
    if len(ids)!=len(set(ids)): errs.append('duplicate node ids')
    merged=[m for n in nodes for m in n.get('merge',[])]
    nh=[x['anno'] for x in t.get('not_headings',[])]
    for a in ids+merged+nh:
        if a not in by: errs.append(f'#{a} does not exist')
    if set(merged)&set(ids): errs.append(f'merged ids are also nodes: {sorted(set(merged)&set(ids))}')
    lv={}; stack=[(0,'root')]; lastpage=0
    for n in nodes:
        a=n['anno']; L=n.get('level')
        if not isinstance(L,int) or L<1: errs.append(f'#{a} bad level {L}'); continue
        pg=by[a]['page_id'] if a in by else 0
        if pg<lastpage: errs.append(f'#{a} (p{pg}) is listed after a node on p{lastpage}: nodes must be in reading order')
        lastpage=max(lastpage,pg)
        while stack[-1][0]>=L: stack.pop()
        want=stack[-1][1]
        if n.get('parent')!=want: errs.append(f"#{a} parent {n.get('parent')} but levels in reading order give {want}")
        if n.get('continues') is not None:
            c=n['continues']
            if c not in lv: errs.append(f"#{a} continues #{c}, which is not an earlier node")
            elif any(x[1]==c for x in stack): errs.append(f"#{a} continues #{c} but #{c}'s section is still open (nothing in between): make it a repeat, not a node")
        if not (n.get('text') or '').strip(): errs.append(f'#{a} empty text')
        lv[a]=L; stack.append((L,a))
    reps=t.get('repeats',[]); ra=[x['anno'] for x in reps]
    for x in reps:
        if x['anno'] not in by: errs.append(f"repeat #{x['anno']} does not exist"); continue
        if x['anno'] in ids or x['anno'] in merged: errs.append(f"repeat #{x['anno']} is also a node or merge")
        if x['of'] not in ids: errs.append(f"repeat #{x['anno']} of #{x['of']}, which is not a node"); continue
        pos=(by[x['anno']]['page_id'],x['anno']); st=[]
        for n in nodes:
            if (by[n['anno']]['page_id'],n['anno'])>pos: break
            while st and st[-1][0]>=n['level']: st.pop()
            st.append((n['level'],n['anno']))
        if x['of'] not in [s_[1] for s_ in st]: errs.append(f"repeat #{x['anno']} of #{x['of']}, but that section is not open there (something came in between): make it a node with continues")
    if set(ra)&set(nh): errs.append(f'ids both in repeats and not_headings: {sorted(set(ra)&set(nh))}')
    gt=gold_tree(r); cov=set(ids)|set(merged)|set(nh)|set(ra)
    miss=[a for a in gt if a not in cov]
    if miss: errs.append(f'gold titles not accounted for (add as node, merge or not_heading): {miss}')
    extra=[a for a in nh if a not in gt]
    if extra: errs.append(f'not_headings that are not gold titles (only gold titles go there): {extra}')
    if not quiet:
        for n in nodes:
            a=n['anno']; pg=by[a]['page_id'] if a in by else '?'
            ex=(f" +{n['merge']}" if n.get('merge') else '')+(f" cont#{n['continues']}" if n.get('continues') else '')
            print('  '*(n.get('level',1)-1)+f"L{n.get('level')} #{a} p{pg}{ex} {n.get('text','')[:70]!r}")
        for x in reps: print(f"  repeat #{x['anno']} (p{by[x['anno']]['page_id']}) of #{x['of']}")
        print(f"{len(nodes)} nodes, {len(merged)} merged, {len(reps)} repeats, {len(nh)} not_headings, gold titles {len(gt)}")
    for e in errs: print('ERROR', doc, e)
    return not errs
if __name__=='__main__':
    if sys.argv[1]=='--all':
        fs=sorted(glob.glob(os.path.join(TREES,'*.json'))); ok=sum(check(os.path.basename(f)[:-5],quiet=True) for f in fs)
        print(f'{ok}/{len(fs)} trees valid'); sys.exit(0 if ok==len(fs) else 1)
    sys.exit(0 if check(sys.argv[1]) else 1)
