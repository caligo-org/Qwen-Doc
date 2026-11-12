import sys
from bs4 import BeautifulSoup
def depth():
    f = sys._getframe(1); n = 0
    while f: n += 1; f = f.f_back
    return n
def at_depth(k, fn):
    if k > 0: return at_depth(k-1, fn)
    return fn(depth())
def maxn(make, base):
    lo, hi = 1, 1200
    while lo < hi:
        mid = (lo+hi+1)//2
        soup = BeautifulSoup(make(mid), 'html.parser')
        try:
            at_depth(base, lambda d: str(soup)); lo = mid
        except RecursionError:
            hi = mid-1
    d = at_depth(base, lambda d: d)
    return d, lo
cases = {
 'text': lambda n: '<b>'*n + 'x',
 'amp': lambda n: '<b>'*n + '&amp;',
 'empty': lambda n: '<b>'*n,
 'attr': lambda n: '<b>'*(n-1) + '<i a="1">',
 'attramp': lambda n: '<b>'*(n-1) + '<i a="&amp;">',
 'attr_text': lambda n: '<b>'*(n-1) + '<i a="1">x',
 'comment': lambda n: '<b>'*n + '<!--c-->',
 'br': lambda n: '<b>'*(n-1) + '<br>',
 'script': lambda n: '<b>'*(n-1) + '<script>x</script>',
 'outer_attr': lambda n: '<b a="1">'*n + 'x',
}
for base in (0, 7, 20):
    for k, mk in cases.items():
        print(base, k, maxn(mk, base))
