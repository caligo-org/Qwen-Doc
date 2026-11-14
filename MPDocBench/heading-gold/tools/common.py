import json, os, functools
GOLD=os.path.join(os.path.dirname(os.path.abspath(__file__)),'..','..','MPDocBench.json')
PAGES=os.environ.get('MPDOCBENCH_PAGES','')   # folder with <doc>/page_<n>.jpg (the dataset's images); needed by show.py only
ROOT=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TREES=os.path.join(ROOT,'trees')
def docname(r): return r['page_info']['image_path'].rsplit('.pdf',1)[0]
@functools.lru_cache(1)
def gold():
    G={}
    for r in json.load(open(GOLD)):
        for e in r['layout_dets']:
            if e['poly'] and isinstance(e['poly'][0],list): e['poly']=[v for pt in e['poly'] for v in pt]
        G[docname(r)]=r
    return G
def box(p): return [min(p[0::2]),min(p[1::2]),max(p[0::2]),max(p[1::2])]
def gold_tree(r):
    """{anno: (depth, parent)} for gold titles in the tree (parent 'root' or anno); titles with no relation -> (None, None)."""
    T={e['anno_id']:e for e in r['layout_dets'] if e['category_type']=='title'}
    par={x['target_anno_id']:x['source_anno_id'] for x in r['extra']['relation'] if x['relation_type']=='parent_son' and x['target_anno_id'] in T and (x['source_anno_id'] in T or x['source_anno_id']=='root')}
    inrel=set(par)|{p for p in par.values() if p!='root'}
    def dep(a,seen=()):
        p=par.get(a,'root'); return 1 if p=='root' or a in seen else 1+dep(p,seen+(a,))
    return {a:((dep(a),par.get(a,'root')) if a in inrel else (None,None)) for a in T}
def reading_key(r):
    """element order: page, then gold 'order' when set, else anno id."""
    return lambda e:(e['page_id'], e['order'] if isinstance(e.get('order'),int) else 10**6, e['anno_id'])
