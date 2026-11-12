"""Convert gold trees into MPDocBench.json form.
  to_mpdocbench.py [out.json]      (default MPDocBench.roletrees.json; reads tools/common.GOLD and trees/)
Docs without a tree are copied unchanged. For a doc with a tree (validated with tools/check.py first):
- every node's element becomes category 'title' (label kept if already a title, else 标题) and its text is the node text;
- a node with `merge` gets the union box of itself and the merged elements; merged elements become text_block (段落);
- not_headings become text_block (段落); repeats that are gold titles become text_block too (a repeat is not a node);
- parent_son relations into any node, merged element or not_heading are replaced by the tree's parents.
Parents are reachable in reading order (check.py), so the MPDocBench tree equals the gold tree. Levels and `continues`
links have no MPDocBench field; they live only in trees/. Where the tree's reading order differs from anno_id order
(the scorer's sibling order), heading ids are permuted among themselves in the copy so the two agree."""
import sys, os, json, glob, copy
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)),'tools'))
from common import gold, box, TREES, GOLD, docname
from check import check
def convert(r, t):
    r=copy.deepcopy(r); by={e['anno_id']:e for e in r['layout_dets']}
    touched=set()
    for n in t['nodes']:
        e=by[n['anno']]; touched.add(n['anno'])
        if e['category_type']!='title': e['category_type']='title'; e['label']='标题'
        e['text']=n['text']
        if n.get('merge'):
            bs=[box(by[a]['poly']) for a in [n['anno']]+n['merge'] if by[a]['page_id']==e['page_id']]
            x0,y0,x1,y1=min(b[0] for b in bs),min(b[1] for b in bs),max(b[2] for b in bs),max(b[3] for b in bs)
            e['poly']=[x0,y0,x1,y0,x1,y1,x0,y1]
            for a in n['merge']: by[a]['category_type']='text_block'; by[a]['label']='段落'; touched.add(a)
    for x in t.get('not_headings',[]): by[x['anno']]['category_type']='text_block'; by[x['anno']]['label']='段落'; touched.add(x['anno'])
    for x in t.get('repeats',[]):
        e=by[x['anno']]
        if e['category_type']=='title': e['category_type']='text_block'; e['label']='段落'
        touched.add(x['anno'])
    rel=[x for x in r['extra']['relation'] if not (x['relation_type']=='parent_son' and (x['target_anno_id'] in touched or x['source_anno_id'] in touched-{n['anno'] for n in t['nodes']}))]
    rel+=[{'source_anno_id':n['parent'],'target_anno_id':n['anno'],'relation_type':'parent_son'} for n in t['nodes']]
    r['extra']['relation']=rel
    # the official scorer orders sibling headings by anno_id; where the tree's reading order differs, permute the
    # node ids among themselves (all references follow) so anno_id order = tree order
    ids=[n['anno'] for n in t['nodes']]
    if ids!=sorted(ids):
        m=dict(zip(ids,sorted(ids)))
        for e in r['layout_dets']:
            if e['anno_id'] in m: e['anno_id']=m[e['anno_id']]
        for x in r['extra']['relation']:
            for k in ('source_anno_id','target_anno_id'):
                if x[k] in m: x[k]=m[x[k]]
    return r
if __name__=='__main__':
    out=sys.argv[1] if len(sys.argv)>1 else os.path.join(os.path.dirname(os.path.abspath(__file__)),'MPDocBench.roletrees.json')
    trees={os.path.basename(f)[:-5]:f for f in glob.glob(os.path.join(TREES,'*.json'))}
    bad=[d for d in trees if not check(d,quiet=True)]
    if bad: sys.exit(f'invalid trees: {bad}')
    G=json.load(open(GOLD)); n=0
    for i,r in enumerate(G):
        d=docname(r)
        if d in trees:
            for e in r['layout_dets']:
                if e['poly'] and isinstance(e['poly'][0],list): e['poly']=[v for pt in e['poly'] for v in pt]
            G[i]=convert(r,json.load(open(trees[d]))); n+=1
    json.dump(G,open(out,'w'),ensure_ascii=False)
    print(f'{n} docs converted -> {out}')
