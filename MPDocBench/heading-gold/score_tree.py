"""Heading TEDS on explicit trees (no Markdown levels).

  score_tree.py <pred.json> [--gold-trees trees/] [--mpdocbench MPDocBench.json] [--docs a,b] [--out scores.json]

pred.json: {doc: [{"id": <any>, "text": str, "parent": <id> | "root", "continues": <id>?}, ...]}  (headings in reading order)
           items with "repeat_of": <id> instead of "parent" are repeats (same heading again while its section is open);
           they are not tree nodes. Optional "boxes": [anno ids] on a heading: the boxes its text is printed in.
Unions: for predictions that carry "boxes", precision/recall of multi-box headings (a predicted union counts when its
box set equals a gold union's box set).
Continues links: a predicted link counts when the two linked predicted headings match (same text after normalisation) the
two linked gold headings; reported as precision/recall over all docs that have gold links.
Gold per doc: trees/<doc>.json when present (semantic parents), else the MPDocBench relations as annotated
(no Markdown rewrite; titles that no relation mentions are attached to root, as the paper's Algorithm S1 says).
Both trees are built the same way (children in reading order) and compared with the toolkit's APTED config:
TEDS = 1 - dist / max(|pred|, |gold|), node counts include the root. Text is normalised the same on both sides
(LaTeX style unwrapped, whitespace runs -> one space, stripped)."""
import sys, os, json, re, collections, statistics as st
TK=os.path.join(os.path.dirname(os.path.abspath(__file__)),'..')
sys.path.insert(0, TK)
from apted import APTED
from metrics.head_metric import TedsHeadNode, TedsHeadConfig
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)),'tools'))
from tree import read, nid
def remove_latex_style(c):
    # the same unwrapping md_tex_filter applies to predicted Markdown (utils/extract.py)
    c=re.sub(r'\\title\{(.*?)\}', r'\1', c)
    c=re.sub(r'\\title\s*\{\s*(.*?)\s*\}', r'\1', c, flags=re.DOTALL)
    c=re.sub(r'\\text\s*\{\s*(.*?)\s*\}', r'\1', c, flags=re.DOTALL)
    c=re.sub(r'\\section\*?\{(.*?)\}', r'\1', c)
    c=re.sub(r'\\section\*?\{\s*(.*?)\s*\}', r'\1', c, flags=re.DOTALL)
    return c
def norm(t): return re.sub(r'\s+', ' ', remove_latex_style(t or '')).strip()
def build(items):
    """items: [(id, text, parent)] in reading order -> root TedsHeadNode."""
    root=TedsHeadNode('root',0,0); nodes={'root':root}
    for k,(i,t,_) in enumerate(items): nodes[i]=TedsHeadNode(norm(t),0,k+1)
    for i,_,p in items:
        par=nodes.get(p,root) if p!=i else root
        nodes[i].parent=par; par.children.append(nodes[i])
    for n in nodes.values(): n.children.sort(key=lambda c:c.order)
    return root, len(nodes)
def teds(pred, gold):
    (a,na),(b,nb)=build(pred),build(gold)
    return 1.0-APTED(a,b,TedsHeadConfig()).compute_edit_distance()/max(na,nb)
def docname(r): return r['page_info']['image_path'].rsplit('.pdf',1)[0]
def gold_items(r, tdir):
    f=os.path.join(tdir, docname(r)+'.json')
    if os.path.exists(f):
        return [(nid(n),n['text'],n['parent']) for n in read(docname(r),tdir)['nodes']]
    T={e['anno_id']:e for e in r['layout_dets'] if e['category_type']=='title'}
    par={x['target_anno_id']:x['source_anno_id'] for x in r['extra']['relation']
         if x['relation_type']=='parent_son' and x['target_anno_id'] in T and (x['source_anno_id'] in T or x['source_anno_id']=='root')}
    return [(a,T[a].get('text',''),par.get(a,'root')) for a in sorted(T)]   # anno order = reading order for titles
def arg(k,d=None): return sys.argv[sys.argv.index(k)+1] if k in sys.argv else d
if __name__=='__main__':
    P=json.load(open(sys.argv[1])); here=os.path.dirname(os.path.abspath(__file__))
    tdir=arg('--gold-trees',os.path.join(here,'trees')); G=json.load(open(arg('--mpdocbench',f'{TK}/MPDocBench.json')))
    docs=set(arg('--docs').split(',')) if arg('--docs') else None
    res={}; cont=[0,0,0]; reps=[0,0,0]; uni=[0,0,0]
    for r in G:
        d=docname(r)
        if docs and d not in docs: continue
        pred=[(x['id'],x['text'],x['parent']) for x in P.get(d,[]) if 'repeat_of' not in x]
        res[d]=teds(pred,gold_items(r,tdir))
        f=os.path.join(tdir,d+'.json')
        if os.path.exists(f):
            t=read(d,tdir); gt={nid(n):norm(n['text']) for n in t['nodes']}
            gl=collections.Counter((gt[nid(n)],gt[n['continues']]) for n in t['nodes'] if n.get('continues') is not None)
            gu={frozenset(n['boxes']) for n in t['nodes'] if len(n['boxes'])>1}
            pu={frozenset(x['boxes']) for x in P.get(d,[]) if len(x.get('boxes') or [])>1}
            if any('boxes' in x for x in P.get(d,[])): uni[0]+=len(gu&pu); uni[1]+=len(pu); uni[2]+=len(gu)
            pt={x['id']:norm(x['text']) for x in P.get(d,[])}
            pl=collections.Counter((pt[x['id']],pt.get(x['continues'])) for x in P.get(d,[]) if x.get('continues') is not None)
            cont[0]+=sum((gl&pl).values()); cont[1]+=sum(pl.values()); cont[2]+=sum(gl.values())
            by={e['anno_id']:e for e in r['layout_dets']}
            gr=collections.Counter(gt[x['of']] for x in t['repeats'])
            prr=collections.Counter(pt.get(x['repeat_of']) for x in P.get(d,[]) if 'repeat_of' in x)
            reps[0]+=sum((gr&prr).values()); reps[1]+=sum(prr.values()); reps[2]+=sum(gr.values())
    if arg('--out'): json.dump(res,open(arg('--out'),'w'),indent=0)
    print(f'{len(res)} docs, mean TEDS {100*st.mean(res.values()):.2f}')
    if uni[2] or uni[1]: print(f'unions: {uni[0]} correct, precision {uni[0]/max(uni[1],1):.2f}, recall {uni[0]/max(uni[2],1):.2f} ({uni[2]} gold multi-box headings)')
    if reps[2]: print(f'repeats: {reps[0]} correct, precision {reps[0]/max(reps[1],1):.2f}, recall {reps[0]/reps[2]:.2f} ({reps[2]} gold; matched by the text of the repeated heading)')
    if cont[2]: print(f'continues links: {cont[0]} correct, precision {cont[0]/max(cont[1],1):.2f}, recall {cont[0]/cont[2]:.2f} ({cont[2]} gold)')
    if len(res)<=10: [print(f'  {d} {v:.4f}') for d,v in res.items()]
