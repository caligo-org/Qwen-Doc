"""Write the gold trees into MPDocBench.json form (the unchanged Heading TEDS scorer reads it).
  to_mpdocbench.py [out.json]      (default MPDocBench.roletrees.json; reads tools/common.GOLD and trees/)
Docs without a tree are copied unchanged. For a doc with a tree (validated with tools/check.py first):
- each heading node is a union of boxes: its first box becomes the title element, with the node text and the union's
  bounding box (boxes on the first box's page); the union's other boxes that are gold titles become text_block (others, e.g. a logo image, keep their category);
- repeats, not_headings and considered entries that are gold titles become text_block;
- parent_son relations into any touched box are replaced by the tree's parents (node id = first box);
- where the tree's reading order differs from anno_id order (the scorer's sibling order), heading ids are permuted
  among themselves so the two agree.
Levels, `continues`, repeats, unions and reasons have no MPDocBench field; they live only in trees/."""
import sys, os, json, glob, copy
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)),'tools'))
from common import box, TREES, GOLD, docname
from check import check
from tree import read, nid
def convert(r, t):
    r=copy.deepcopy(r); by={e['anno_id']:e for e in r['layout_dets']}; touched=set(); heads=set()
    for n in t['nodes']:
        a=nid(n); e=by[a]; touched|=set(n['boxes']); heads.add(a)
        e['category_type']='title'
        if e.get('label') not in ('标题','目录标题','段落标题','文档标题'): e['label']='标题'
        e['text']=n['text'].replace('\n',' ')
        if len(n['boxes'])>1:
            bs=[box(by[b]['poly']) for b in n['boxes'] if by[b]['page_id']==e['page_id']]
            x0,y0,x1,y1=min(b[0] for b in bs),min(b[1] for b in bs),max(b[2] for b in bs),max(b[3] for b in bs)
            e['poly']=[x0,y0,x1,y0,x1,y1,x0,y1]
            for b in n['boxes'][1:]:
                if by[b]['category_type']=='title': by[b]['category_type']='text_block'; by[b]['label']='段落'
    for b in [b for x in t['repeats'] for b in x['boxes']]+[x['box'] for x in t['not_headings']]+[x['box'] for x in t.get('considered',[])]:
        touched.add(b)
        if by[b]['category_type']=='title': by[b]['category_type']='text_block'; by[b]['label']='段落'
    rel=[x for x in r['extra']['relation'] if not (x['relation_type']=='parent_son' and (x['target_anno_id'] in touched or x['source_anno_id'] in touched-heads))]
    rel+=[{'source_anno_id':n['parent'],'target_anno_id':nid(n),'relation_type':'parent_son'} for n in t['nodes']]
    r['extra']['relation']=rel
    ids=[nid(n) for n in t['nodes']]
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
    docs=[os.path.basename(f)[:-5] for f in glob.glob(os.path.join(TREES,'*.json'))]
    bad=[d for d in docs if not check(d,quiet=True)]
    if bad: sys.exit(f'invalid trees: {bad}')
    G=json.load(open(GOLD)); n=0
    for i,r in enumerate(G):
        d=docname(r)
        if d in docs:
            for e in r['layout_dets']:
                if e['poly'] and isinstance(e['poly'][0],list): e['poly']=[v for pt in e['poly'] for v in pt]
            G[i]=convert(r,read(d)); n+=1
    json.dump(G,open(out,'w'),ensure_ascii=False)
    print(f'{n} docs converted -> {out}')
