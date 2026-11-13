"""show.py <doc> <page>[,<page>...] [--tree]  -> writes review/<doc>_p<pages>.jpg and prints its path.
Every element is boxed with its id. Gold headings: coloured, 'L<depth> #id'. Other elements: grey '#id <category>'.
--tree: label headings from trees/<doc>.json instead (your proposal): 'L<level> #id', merged boxes '+#id',
continuations 'cont #x', not-headings struck through in grey."""
import sys, os, json
from PIL import Image, ImageDraw, ImageFont
from common import gold, gold_tree, box, PAGES, ROOT, TREES
doc=sys.argv[1]; pages=[int(p) for p in sys.argv[2].split(',')]; use_tree='--tree' in sys.argv
HL={int(x) for x in sys.argv[sys.argv.index('--hl')+1].split(',')} if '--hl' in sys.argv else set()
OUT=sys.argv[sys.argv.index('--out')+1] if '--out' in sys.argv else None
r=gold()[doc]; gt=gold_tree(r)
if not PAGES: sys.exit('set MPDOCBENCH_PAGES to the folder holding <doc>/page_<n>.jpg')
FONT=os.environ.get('CJK_FONT','/System/Library/Fonts/Hiragino Sans GB.ttc')
F=ImageFont.truetype(FONT,16); FS=ImageFont.truetype(FONT,12)
COL={'rep':(120,120,120),1:(210,0,0),2:(0,100,230),3:(0,150,0),4:(180,0,180),5:(230,120,0),6:(0,150,150),7:(120,80,0),8:(80,80,200)}
lab={}
if use_tree:
    from tree import read, nid
    t=read(doc)
    for n in t['nodes']:
        a=nid(n); lab[a]=(n['level'],f"L{n['level']} #{a}"+(f" cont #{n['continues']}" if n.get('continues') is not None else '')+(f" ∪{len(n['boxes'])}" if len(n['boxes'])>1 else ''))
        for m in n['boxes'][1:]: lab[m]=(n['level'],f"∪ part of #{a}")
    for x in t['repeats']:
        for b in x['boxes']: lab[b]=('rep',f"REPEAT of #{x['of']} (not a node)")
    BRK={x['before']:x['closes'] for x in t.get('breaks',[])}
    STA={n['starts']:nid(n) for n in t['nodes'] if n.get('starts') is not None}
else:
    BRK={}; STA={}
    for a,(d,p) in gt.items(): lab[a]=(d or 0,f"L{d} #{a}" if d else f"no-rel #{a}")
ims=[]
for p in pages:
    im=Image.open(f"{PAGES}/{doc}/page_{p}.jpg").convert('RGB'); s=1400/im.height if im.height>1400 else 1.0
    if s!=1.0: im=im.resize((int(im.width*s),int(im.height*s)),Image.LANCZOS)
    dr=ImageDraw.Draw(im)
    for e in r['layout_dets']:
        if e['page_id']!=p or not e['poly']: continue
        b=[v*s for v in box(e['poly'])]; a=e['anno_id']
        if a in lab:
            c=COL.get(lab[a][0],(90,90,90)); txt=lab[a][1]; dr.rectangle(b,outline=c,width=3); f=F
        else:
            c=(150,150,150); txt=f"#{a} {e['category_type'][:6]}"; dr.rectangle(b,outline=c,width=1); f=FS
        if a in BRK:  # break: thick red line above the box, the section of #closes ends here
            dr.line([0,b[1]-6,im.width,b[1]-6],fill=(230,0,0),width=5); bt=f"BREAK closes #{BRK[a]}"
            dr.rectangle([im.width-dr.textlength(bt,font=F)-8,b[1]-28,im.width,b[1]-8],fill=(230,0,0)); dr.text((im.width-dr.textlength(bt,font=F)-4,b[1]-28),bt,fill='white',font=F)
        if a in STA:  # section start moved before the heading's box (layout reading order puts its content first)
            dr.line([b[0],b[1]-4,b[2],b[1]-4],fill=(0,90,230),width=5); st=f"SECTION #{STA[a]} STARTS"
            dr.rectangle([b[0],b[1]-26,b[0]+dr.textlength(st,font=F)+6,b[1]-6],fill=(0,90,230)); dr.text((b[0]+3,b[1]-26),st,fill='white',font=F)
        if a in HL: dr.rectangle([b[0]-4,b[1]-4,b[2]+4,b[3]+4],outline=(255,0,0),width=6)
        tw=dr.textlength(txt,font=f); h=f.size+3; ty=b[1]-h if b[1]>=h else b[1]
        dr.rectangle([b[0],ty,b[0]+tw+4,ty+h],fill=c); dr.text((b[0]+2,ty),txt,fill='white',font=f)
    ims.append(im)
W=sum(i.width for i in ims)+8*(len(ims)-1); H=max(i.height for i in ims)
S=Image.new('RGB',(W,H),'white'); x=0
for i in ims: S.paste(i,(x,0)); x+=i.width+8
out=OUT or os.path.join(ROOT,'review',f"{doc}_p{'-'.join(map(str,pages))}{'_tree' if use_tree else ''}.jpg")
os.makedirs(os.path.dirname(out),exist_ok=True); S.save(out,quality=85)
with open(os.path.join(ROOT,'review','seen.log'),'a') as lg:
    for p in pages: lg.write(f"{doc}\t{p}\t{'tree' if use_tree else 'gold'}\n")
print(out)
