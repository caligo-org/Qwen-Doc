"""listing.py <doc> [page]  -- every element of the doc (or page): id, category, gold heading depth/parent, text.
Gold headings are marked  H L<depth> ^<parent>. Use the ids in your tree file."""
import sys
from common import gold, gold_tree, reading_key
doc=sys.argv[1]; pg=int(sys.argv[2]) if len(sys.argv)>2 else None
r=gold()[doc]; gt=gold_tree(r); pi=r['page_info']
print(f"{doc}  {len(pi['height'])} pages  source={pi['page_attribute']['data_source']} lang={pi['page_attribute']['language']}")
for e in sorted(r['layout_dets'],key=reading_key(r)):
    if pg and e['page_id']!=pg: continue
    t=(e.get('text') or '').replace('\n',' / ')
    h=''
    if e['anno_id'] in gt:
        d,p=gt[e['anno_id']]; h=f"H L{d} ^{p}" if d else 'H (no relation)'
    print(f"p{e['page_id']:<3}#{e['anno_id']:<5}{e['category_type'][:14]:14} {h:16} {t[:110]}")
