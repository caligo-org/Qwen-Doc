# MPDocBench GT: merge display formulas split across boxes

Branch `fix-gt-formula-splits` (base `origin/main` 4e5aee3). Applied with `apply_splits.py edits.json MPDocBench/MPDocBench.json --doc <doc>` (one commit per document).
Evidence crops: `./` (page-image crop, every candidate box outlined and labelled with its anno_id).

**46 groups merged** (48 boxes removed) in 4 documents; **151 candidate groups rejected**. In every merge the first block keeps its anno_id, order and other fields, its poly becomes the union box, and the LaTeX is joined (`\quad` between parts ≥ 20 px apart, `\ ` otherwise; split `cases` become one `cases`). `extra.relation` entries whose target was removed are dropped (all were `parent_son` links from the same parent as the kept block, plus one link 363→372 in paper_eng_20a93115).

## Merged

### paper_chn_fdabe9a2.pdf

- **p1**, keep 24, remove [25] (qualifier) — evidence `./H_paper_chn_fdabe9a2_p1_24-25.png`
  - before 24: `\[I_{\mathrm{b w}}{=}I_{\mathrm{s w}}\times(29{-}N)/28\]`
  - before 25: `\[(1<N\leqslant15);\]`
  - after 24: `\[I_{\mathrm{b w}}{=}I_{\mathrm{s w}}\times(29{-}N)/28 \quad (1<N\leqslant15);\]`
- **p1**, keep 27, remove [28] (qualifier) — evidence `./H_paper_chn_fdabe9a2_p1_27-28.png`
  - before 27: `\[I_{\mathrm{b w}}{=}0.5I_{\mathrm{s w}}\]`
  - before 28: `\[(N>15);\]`
  - after 27: `\[I_{\mathrm{b w}}{=}0.5I_{\mathrm{s w}} \quad (N>15);\]`

### paper_eng_20a93115.pdf

- **p2**, keep 20, remove [21] (qualifier) — evidence `./H_paper_eng_20a93115_p2_20-21.png`
  - before 20: `\[p_{a,b}(f)=\sup_{x\in\mathbb{R}^{n}}\left|x^{a}D^{b}f(x)\right|,\]`
  - before 21: `\[a,b\in\mathbb{N}^{n},\]`
  - after 20: `\[p_{a,b}(f)=\sup_{x\in\mathbb{R}^{n}}\left|x^{a}D^{b}f(x)\right|, \quad a,b\in\mathbb{N}^{n},\]`
- **p3**, keep 27, remove [28] (qualifier) — evidence `./H_paper_eng_20a93115_p3_27-28.png`
  - before 27: `\[p_{K,m}(f)=\operatorname*{m a x}_{|\alpha|\leq m}\operatorname*{s u p}_{x\in K}|D^{\alpha}f(x)|,\]`
  - before 28: `\[m\in\mathbb{N}.\]`
  - after 27: `\[p_{K,m}(f)=\operatorname*{m a x}_{|\alpha|\leq m}\operatorname*{s u p}_{x\in K}|D^{\alpha}f(x)|, \quad m\in\mathbb{N}.\]`
- **p3**, keep 33, remove [34] (qualifier) — evidence `./H_paper_eng_20a93115_p3_33-34.png`
  - before 33: `\[\varphi_{K}(f)=\max_{x\in K}|f(x)|,\]`
  - before 34: `\[K\subset X{\mathrm{~c o m p a c t}},\]`
  - after 33: `\[\varphi_{K}(f)=\max_{x\in K}|f(x)|, \quad K\subset X{\mathrm{~c o m p a c t}},\]`
- **p4**, keep 54, remove [55] (qualifier) — evidence `./H_paper_eng_20a93115_p4_53-54-55.png`
  - before 54: `\[z_{\ell}(x)=\sigma\big(A_{\ell}z_{\ell-1}(x)-b_{\ell}\big),\]`
  - before 55: `\[\ell=2,\dots,L-1.\]`
  - after 54: `\[z_{\ell}(x)=\sigma\big(A_{\ell}z_{\ell-1}(x)-b_{\ell}\big),\ \ell=2,\dots,L-1.\]`
- **p4**, keep 60, remove [61] (qualifier) — evidence `./H_paper_eng_20a93115_p4_60-61.png`
  - before 60: `\[f(x)=w\cdot x,\]`
  - before 61: `\[x\in\mathbb{R}^{d},\]`
  - after 60: `\[f(x)=w\cdot x, \quad x\in\mathbb{R}^{d},\]`
- **p6**, keep 104, remove [105] (qualifier) — evidence `./H_paper_eng_20a93115_p6_104-105.png`
  - before 104: `\[e^{\ell_{1}(x)}e^{\ell_{2}(x)}=e^{(\ell_{1}+\ell_{2})(x)},\]`
  - before 105: `\[\ell_{1}+\ell_{2}\in X^{*},\]`
  - after 104: `\[e^{\ell_{1}(x)}e^{\ell_{2}(x)}=e^{(\ell_{1}+\ell_{2})(x)}, \quad \ell_{1}+\ell_{2}\in X^{*},\]`
- **p7**, keep 122, remove [123] (qualifier) — evidence `./H_paper_eng_20a93115_p7_122-123.png`
  - before 122: `\[H_{r}(x)=\sum_{i=1}^{M}\alpha_{i}\sum_{j=1}^{N_{i}}c_{i,j}\sigma\big(w_{i,j}\ell_{i}(x)-\theta_{i,j}\big),\]`
  - before 123: `\[x\in X.\]`
  - after 122: `\[H_{r}(x)=\sum_{i=1}^{M}\alpha_{i}\sum_{j=1}^{N_{i}}c_{i,j}\sigma\big(w_{i,j}\ell_{i}(x)-\theta_{i,j}\big), \quad x\in X.\]`
- **p7**, keep 130, remove [131] (qualifier) — evidence `./H_paper_eng_20a93115_p7_130-131.png`
  - before 130: `\[H(x)=(H_{1}(x),\dots,H_{m}(x)),\]`
  - before 131: `\[x\in X.\]`
  - after 130: `\[H(x)=(H_{1}(x),\dots,H_{m}(x)), \quad x\in X.\]`
- **p8**, keep 148, remove [149] (qualifier) — evidence `./H_paper_eng_20a93115_p8_148-149.png`
  - before 148: `\[\operatorname*{s u p}_{u\in V}\left|\ell_{i}(u)-\sum_{j=1}^{k}\xi_{i j}u(x_{j})\right|<\delta,\]`
  - before 149: `\[i=1,\dots,N.\]`
  - after 148: `\[\operatorname*{s u p}_{u\in V}\left|\ell_{i}(u)-\sum_{j=1}^{k}\xi_{i j}u(x_{j})\right|<\delta, \quad i=1,\dots,N.\]`
- **p9**, keep 159, remove [160] (qualifier) — evidence `./H_paper_eng_20a93115_p9_159-160.png`
  - before 159: `\[(u,y)\longmapsto G(u)(y),\]`
  - before 160: `\[(u,y)\in V\times K,\]`
  - after 159: `\[(u,y)\longmapsto G(u)(y), \quad (u,y)\in V\times K,\]`
- **p9**, keep 176, remove [177, 178] (qualifier) — evidence `./H_paper_eng_20a93115_p9_176-177-178.png`
  - before 176: `\[(\widehat{G}(u))(y)=\mathcal{B}(u)\mathcal{T}(y),\]`
  - before 177: `\[u\in X,\]`
  - before 178: `\[y\in\mathbb{R}^{d}.\]`
  - after 176: `\[(\widehat{G}(u))(y)=\mathcal{B}(u)\mathcal{T}(y), \quad u\in X,\ y\in\mathbb{R}^{d}.\]`
- **p11**, keep 194, remove [195] (qualifier) — evidence `./H_paper_eng_20a93115_p11_194-195.png`
  - before 194: `\[y\longmapsto\sigma(\omega\cdot y+\zeta),\]`
  - before 195: `\[y,\omega\in\mathbb{R}^{d},\zeta\in\mathbb{R},\]`
  - after 194: `\[y\longmapsto\sigma(\omega\cdot y+\zeta), \quad y,\omega\in\mathbb{R}^{d},\zeta\in\mathbb{R},\]`
- **p11**, keep 200, remove [201] (qualifier) — evidence `./H_paper_eng_20a93115_p11_200-201.png`
  - before 200: `\[\phi_{k}(y)=\sigma(\omega_{k}\cdot y+\zeta_{k}),\]`
  - before 201: `\[k=1,\ldots,N,\]`
  - after 200: `\[\phi_{k}(y)=\sigma(\omega_{k}\cdot y+\zeta_{k}), \quad k=1,\ldots,N,\]`
- **p11**, keep 203, remove [204] (qualifier) — evidence `./H_paper_eng_20a93115_p11_203-204.png`
  - before 203: `\[a_{k}:X\to\mathbb{R}^{m},\]`
  - before 204: `\[k=1,\dots,N,\]`
  - after 203: `\[a_{k}:X\to\mathbb{R}^{m}, \quad k=1,\dots,N,\]`
- **p12**, keep 217, remove [218] (qualifier) — evidence `./H_paper_eng_20a93115_p12_217-218.png`
  - before 217: `\[R_{h,r}(y)=\sum_{j=1}^{N(h,r)}\alpha_{h,r,j}\sigma(\omega_{h,r,j}\cdot y+\zeta_{h,r,j}),\]`
  - before 218: `\[y\in K,\]`
  - after 217: `\[R_{h,r}(y)=\sum_{j=1}^{N(h,r)}\alpha_{h,r,j}\sigma(\omega_{h,r,j}\cdot y+\zeta_{h,r,j}), \quad y\in K,\]`
- **p12**, keep 221, remove [222] (qualifier) — evidence `./H_paper_eng_20a93115_p12_221-222.png`
  - before 221: `\[R_{h}(y):=(R_{h,1}(y),\ldots,R_{h,m}(y)),\]`
  - before 222: `\[y\in K.\]`
  - after 221: `\[R_{h}(y):=(R_{h,1}(y),\ldots,R_{h,m}(y)), \quad y\in K.\]`
- **p12**, keep 234, remove [235] (qualifier) — evidence `./H_paper_eng_20a93115_p12_234-235.png`
  - before 234: `\[\phi_{k}(y)=\sigma(\omega_{k}\cdot y+\zeta_{k}),\]`
  - before 235: `\[k=1,\dots,N.\]`
  - after 234: `\[\phi_{k}(y)=\sigma(\omega_{k}\cdot y+\zeta_{k}), \quad k=1,\dots,N.\]`
- **p12**, keep 237, remove [238] (qualifier) — evidence `./H_paper_eng_20a93115_p12_237-238.png`
  - before 237: `\[R_{h^{(j)}}(y)=\sum_{k=1}^{N}A_{j,k}\phi_{k}(y),\]`
  - before 238: `\[y\in K,\]`
  - after 237: `\[R_{h^{(j)}}(y)=\sum_{k=1}^{N}A_{j,k}\phi_{k}(y), \quad y\in K,\]`
- **p13**, keep 243, remove [244] (qualifier) — evidence `./H_paper_eng_20a93115_p13_243-244.png`
  - before 243: `\[\eta_{j}:W\rightarrow[0,1],\]`
  - before 244: `\[j=1,\dots,M,\]`
  - after 243: `\[\eta_{j}:W\rightarrow[0,1], \quad j=1,\dots,M,\]`
- **p13**, keep 249, remove [250] (qualifier) — evidence `./H_paper_eng_20a93115_p13_249-250.png`
  - before 249: `\[c_{k}(h):=\sum\limits_{j=1}^{M}\eta_{j}(h)A_{j,k},\]`
  - before 250: `\[k=1,\ldots,N.\]`
  - after 249: `\[c_{k}(h):=\sum\limits_{j=1}^{M}\eta_{j}(h)A_{j,k}, \quad k=1,\ldots,N.\]`
- **p13**, keep 265, remove [266] (qualifier) — evidence `./H_paper_eng_20a93115_p13_265-266.png`
  - before 265: `\[b_{k,r}(u):=\left(c_{k}(G(u))\right)_{r},\]`
  - before 266: `\[u\in V.\]`
  - after 265: `\[b_{k,r}(u):=\left(c_{k}(G(u))\right)_{r}, \quad u\in V.\]`
- **p14**, keep 275, remove [276] (qualifier) — evidence `./H_paper_eng_20a93115_p14_275-276.png`
  - before 275: `\[a_{k}(u)=(a_{k,1}(u),\dots,a_{k,m}(u)),\]`
  - before 276: `\[k=1,\dots,N,\]`
  - after 275: `\[a_{k}(u)=(a_{k,1}(u),\dots,a_{k,m}(u)), \quad k=1,\dots,N,\]`
- **p14**, keep 290, remove [291] (qualifier) — evidence `./H_paper_eng_20a93115_p14_290-291.png`
  - before 290: `\[\mathcal{T}(y)=\big(t_{1}(y),\dots,t_{p}(y)\big)^{T}\in\mathbb{R}^{p},\]`
  - before 291: `\[y\in K,\]`
  - after 290: `\[\mathcal{T}(y)=\big(t_{1}(y),\dots,t_{p}(y)\big)^{T}\in\mathbb{R}^{p}, \quad y\in K,\]`
- **p18**, keep 363, remove [364, 365] (qualifier) — evidence `./H_paper_eng_20a93115_p18_363-364-365.png`
  - before 363: `\[G(u)(y)\approx\sum_{k=1}^{N}a_{k}(u)\sigma(\omega_{k}\cdot y+\zeta_{k}),\]`
  - before 364: `\[u\in V,\]`
  - before 365: `\[y\in K,\]`
  - after 363: `\[G(u)(y)\approx\sum_{k=1}^{N}a_{k}(u)\sigma(\omega_{k}\cdot y+\zeta_{k}), \quad u\in V,\ y\in K,\]`
- **p18**, keep 371, remove [372] (qualifier) — evidence `./H_paper_eng_20a93115_p18_371-372.png`
  - before 371: `\[A\longmapsto\mathrm{t r a c e}(W^{T}A),\]`
  - before 372: `\[A\in X,\]`
  - after 371: `\[A\longmapsto\mathrm{t r a c e}(W^{T}A), \quad A\in X,\]`
- **p18**, keep 375, remove [376] (qualifier) — evidence `./H_paper_eng_20a93115_p18_375-376.png`
  - before 375: `\[a_{k}(A)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma\big(\mathrm{t r a c e}(W_{k,i}^{T}A)-\theta_{k,i}\big),\]`
  - before 376: `\[A\in X.\]`
  - after 375: `\[a_{k}(A)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma\big(\mathrm{t r a c e}(W_{k,i}^{T}A)-\theta_{k,i}\big), \quad A\in X.\]`
- **p18**, keep 381, remove [382] (qualifier) — evidence `./H_paper_eng_20a93115_p18_381-382.png`
  - before 381: `\[x\longmapsto\sum_{n=1}^{\infty}w_{n}x_{n},\]`
  - before 382: `\[w=(w_{1},w_{2},\dots)\in\ell_{q}.\]`
  - after 381: `\[x\longmapsto\sum_{n=1}^{\infty}w_{n}x_{n}, \quad w=(w_{1},w_{2},\dots)\in\ell_{q}.\]`
- **p19**, keep 388, remove [389] (qualifier) — evidence `./H_paper_eng_20a93115_p19_388-389.png`
  - before 388: `\[a_{k}(x)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma\left(\sum_{n=1}^{\infty}w_{n}^{(k,i)}x_{n}-\theta_{k,i}\right),\]`
  - before 389: `\[x\in X.\]`
  - after 388: `\[a_{k}(x)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma\left(\sum_{n=1}^{\infty}w_{n}^{(k,i)}x_{n}-\theta_{k,i}\right), \quad x\in X.\]`
- **p19**, keep 391, remove [392] (qualifier) — evidence `./H_paper_eng_20a93115_p19_391-392.png`
  - before 391: `\[x\longmapsto\sum_{n=1}^{\infty}w_{n}x_{n},\]`
  - before 392: `\[w=(w_{1},w_{2},\dots)\in\ell_{1}.\]`
  - after 391: `\[x\longmapsto\sum_{n=1}^{\infty}w_{n}x_{n}, \quad w=(w_{1},w_{2},\dots)\in\ell_{1}.\]`
- **p19**, keep 396, remove [397] (qualifier) — evidence `./H_paper_eng_20a93115_p19_396-397.png`
  - before 396: `\[a_{k}(x)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma\left(\sum_{n=1}^{\infty}w_{n}^{(k,i)}x_{n}-\theta_{k,i}\right),\]`
  - before 397: `\[x\in X.\]`
  - after 396: `\[a_{k}(x)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma\left(\sum_{n=1}^{\infty}w_{n}^{(k,i)}x_{n}-\theta_{k,i}\right), \quad x\in X.\]`
- **p19**, keep 399, remove [400] (qualifier) — evidence `./H_paper_eng_20a93115_p19_399-400.png`
  - before 399: `\[f\longmapsto\int_{\Omega}f(x)g(x)d\mu(x),\]`
  - before 400: `\[g\in L_{q}(\Omega,\mu).\]`
  - after 399: `\[f\longmapsto\int_{\Omega}f(x)g(x)d\mu(x), \quad g\in L_{q}(\Omega,\mu).\]`
- **p19**, keep 402, remove [403] (qualifier) — evidence `./H_paper_eng_20a93115_p19_402-403.png`
  - before 402: `\[a_{k}(f)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma\biggl(\int_{\Omega}f(x)g_{k,i}(x)d\mu(x)-\theta_{k,i}\biggr),\]`
  - before 403: `\[f\in X.\]`
  - after 402: `\[a_{k}(f)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma\biggl(\int_{\Omega}f(x)g_{k,i}(x)d\mu(x)-\theta_{k,i}\biggr), \quad f\in X.\]`
- **p19**, keep 405, remove [406] (qualifier) — evidence `./H_paper_eng_20a93115_p19_405-406.png`
  - before 405: `\[f\longmapsto\langle T,f\rangle,\]`
  - before 406: `\[T\in\mathcal{S}^{\prime}(\mathbb{R}^{n}).\]`
  - after 405: `\[f\longmapsto\langle T,f\rangle, \quad T\in\mathcal{S}^{\prime}(\mathbb{R}^{n}).\]`
- **p20**, keep 410, remove [411] (qualifier) — evidence `./H_paper_eng_20a93115_p20_410-411.png`
  - before 410: `\[a_{k}(f)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma(\langle T_{k,i},f\rangle-\theta_{k,i}),\]`
  - before 411: `\[f\in X.\]`
  - after 410: `\[a_{k}(f)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma(\langle T_{k,i},f\rangle-\theta_{k,i}), \quad f\in X.\]`
- **p20**, keep 413, remove [414] (qualifier) — evidence `./H_paper_eng_20a93115_p20_413-414.png`
  - before 413: `\[f\longmapsto\langle T,f\rangle,\]`
  - before 414: `\[T\in\mathcal{D}^{\prime}(U).\]`
  - after 413: `\[f\longmapsto\langle T,f\rangle, \quad T\in\mathcal{D}^{\prime}(U).\]`
- **p20**, keep 416, remove [417] (qualifier) — evidence `./H_paper_eng_20a93115_p20_416-417.png`
  - before 416: `\[a_{k}(f)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma(\langle T_{k,i},f\rangle-\theta_{k,i}),\]`
  - before 417: `\[f\in X.\]`
  - after 416: `\[a_{k}(f)=\sum_{i=1}^{r_{k}}c_{k,i}\sigma(\langle T_{k,i},f\rangle-\theta_{k,i}), \quad f\in X.\]`

### paper_eng_b8670bda.pdf

- **p4**, keep 56, remove [57] (qualifier) — evidence `./H_paper_eng_b8670bda_p4_56-57.png`
  - before 56: `\[p(0)=1,\]`
  - before 57: `\[\dot{p}=-(1-m c)^{+}I p,\]`
  - after 56: `\[p(0)=1, \quad \dot{p}=-(1-m c)^{+}I p,\]`
- **p4**, keep 59, remove [60] (qualifier) — evidence `./H_paper_eng_b8670bda_p4_59-60.png`
  - before 59: `\[I(0)=I_{0},\]`
  - before 60: `\[\dot{I}=(1-m\overline{{{c}}})^{+}I(1-I),\]`
  - after 59: `\[I(0)=I_{0}, \quad \dot{I}=(1-m\overline{{{c}}})^{+}I(1-I),\]`
- **p6**, keep 96, remove [97] (cases) — evidence `./H_paper_eng_b8670bda_p6_96-97.png`
  - before 96: `\[x^{*}(m,I_{0},t_{f})=\begin{cases}0\\ t_{f}\\ m-1-\mathrm{W}\left(\left(\frac{1}{I_{0}}-1\right)e^{m-1-t_{f}}\right)\end{cases}\]`
  - before 97: `\[\begin{aligned}&if I_{0}(1+(m-1)e^{t_{f}})<1,\\&if I_{0}(m-t_{f})>1,\\&otherwise.\end{aligned}\]`
  - after 96: `\[x^{*}(m,I_{0},t_{f})=\begin{cases}0&\mathrm{if}~I_{0}(1+(m-1)e^{t_{f}})<1,\\ t_{f}&\mathrm{if}~I_{0}(m-t_{f})>1,\\ m-1-\mathrm{W}\left(\left(\frac{1}{I_{0}}-1\right)e^{m-1-t_{f}}\right)&\mathrm{otherwise.}\end{cases}\]`
- **p8**, keep 126, remove [127] (cases) — evidence `./H_paper_eng_b8670bda_p8_126-127.png`
  - before 126: `\[\mathcal{E}(x^{*})=\begin{cases}{(1-I(t_{f}))/{(1-I_{0})},}\\ {(1+x^{*}-m I_{0})/m(1-I_{0}),}\\ {t_{f}/m,}\\ \end{cases}\]`
  - before 127: `\[\begin{aligned}&if x^{*}=0,\\&if x^{*}\in(0,t_{f}),\\&if x^{*}=t_{f}.\end{aligned}\]`
  - after 126: `\[\mathcal{E}(x^{*})=\begin{cases}{(1-I(t_{f}))/{(1-I_{0})},}&\mathrm{if}~x^{*}=0,\\ {(1+x^{*}-m I_{0})/m(1-I_{0}),}&\mathrm{if}~x^{*}\in(0,t_{f}),\\ {t_{f}/m,}&\mathrm{if}~x^{*}=t_{f}.\end{cases}\]`
- **p12**, keep 210, remove [211] (cases) — evidence `./H_paper_eng_b8670bda_p12_210-211.png`
  - before 210: `\[c^{*}(\Phi)\in\begin{cases}\{0\}&\\ [0,\frac{1}{m}]&\\ \{\frac{1}{m}\}&\end{cases}\]`
  - before 211: `\[\begin{array}{r l}&{\mathrm{i f~}\Phi<\frac{1}{m},}\\ &{\mathrm{i f~}\Phi=\frac{1}{m},}\\ &{\mathrm{e l s e.}}\end{array}\]`
  - after 210: `\[c^{*}(\Phi)\in\begin{cases}\{0\}&\mathrm{if}~\Phi<\frac{1}{m},\\ {[0,\frac{1}{m}]}&\mathrm{if}~\Phi=\frac{1}{m},\\ \{\frac{1}{m}\}&\mathrm{else.}\end{cases}\]`

### paper_eng_c1aa81bd.pdf

- **p2**, keep 20, remove [21] (qualifier) — evidence `./H_paper_eng_c1aa81bd_p2_20-21.png`
  - before 20: `\[E_{N}\big(\sigma\big)=\frac{\beta}{\sqrt{N}}\sum_{i,j=1}^{N}W_{i j}\sigma_{i}\sigma_{j}+\left(\frac{1}{\sqrt{N}}\sum_{i=1}^{N}\chi_{i}\sigma_{i}\right)^{2},\]`
  - before 21: `\[\forall\sigma\in\{-1,1\}^{N},\]`
  - after 20: `\[E_{N}\big(\sigma\big)=\frac{\beta}{\sqrt{N}}\sum_{i,j=1}^{N}W_{i j}\sigma_{i}\sigma_{j}+\left(\frac{1}{\sqrt{N}}\sum_{i=1}^{N}\chi_{i}\sigma_{i}\right)^{2}, \quad \forall\sigma\in\{-1,1\}^{N},\]`
- **p5**, keep 63, remove [64] (qualifier) — evidence `./H_paper_eng_c1aa81bd_p5_63-64.png`
  - before 63: `\[\mathbb{E}\left[H_{N}(\sigma)H_{N}(\sigma^{\prime})\right]=N\xi\left(\frac{\sigma\sigma^{\prime\top}}{N}\right),\]`
  - before 64: `\[\forall\sigma,\sigma^{\prime}\in\mathbb{R}^{D\times N}.\]`
  - after 63: `\[\mathbb{E}\left[H_{N}(\sigma)H_{N}(\sigma^{\prime})\right]=N\xi\left(\frac{\sigma\sigma^{\prime\top}}{N}\right), \quad \forall\sigma,\sigma^{\prime}\in\mathbb{R}^{D\times N}.\]`
- **p5**, keep 67, remove [68] (qualifier) — evidence `./H_paper_eng_c1aa81bd_p5_67-68.png`
  - before 67: `\[\theta(a)=a\cdot\nabla\xi(a)-\xi(a),\]`
  - before 68: `\[\forall a\in\mathbb{R}^{D\times D}.\]`
  - after 67: `\[\theta(a)=a\cdot\nabla\xi(a)-\xi(a), \quad \forall a\in\mathbb{R}^{D\times D}.\]`

## Rejected candidates

Candidates = equation_isolated boxes on one page that share a line (vertical overlap ≥ 30 % of the smaller box) or are stacked with a gap < half a line. Kinds: H = same line, X = same line but another block between, V = stacked.

| kind | doc | page | anno_ids | reason | evidence |
|---|---|---|---|---|---|
| H | examPaper_eng_0b2fd078.pdf | 2 | [30, 31] | two independent formulas side by side (A1) | `./H_examPaper_eng_0b2fd078_p2_30-31.png` |
| H | handwritten_chn_1ae487be.pdf | 1 | [6, 30] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p1_6-30.png` |
| H | handwritten_chn_1ae487be.pdf | 1 | [8, 32] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p1_8-32.png` |
| H | handwritten_chn_1ae487be.pdf | 1 | [10, 34] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p1_10-34.png` |
| H | handwritten_chn_1ae487be.pdf | 1 | [12, 36] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p1_12-36.png` |
| H | handwritten_chn_1ae487be.pdf | 1 | [16, 42] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p1_16-42.png` |
| H | handwritten_chn_1ae487be.pdf | 1 | [18, 44] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p1_18-44.png` |
| H | handwritten_chn_1ae487be.pdf | 1 | [21, 46] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p1_21-46.png` |
| H | handwritten_chn_1ae487be.pdf | 1 | [23, 48] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p1_23-48.png` |
| H | handwritten_chn_1ae487be.pdf | 2 | [53, 87] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p2_53-87.png` |
| H | handwritten_chn_1ae487be.pdf | 2 | [55, 89] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p2_55-89.png` |
| H | handwritten_chn_1ae487be.pdf | 2 | [57, 91] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p2_57-91.png` |
| H | handwritten_chn_1ae487be.pdf | 2 | [61, 95] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p2_61-95.png` |
| H | handwritten_chn_1ae487be.pdf | 2 | [63, 97] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p2_63-97.png` |
| H | handwritten_chn_1ae487be.pdf | 2 | [66, 101] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p2_66-101.png` |
| H | handwritten_chn_1ae487be.pdf | 2 | [80, 116] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p2_80-116.png` |
| H | handwritten_chn_1ae487be.pdf | 2 | [84, 119] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p2_84-119.png` |
| H | handwritten_chn_1ae487be.pdf | 3 | [122, 126] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p3_122-126.png` |
| H | handwritten_chn_1ae487be.pdf | 3 | [124, 128] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p3_124-128.png` |
| H | handwritten_chn_1ae487be.pdf | 3 | [139, 162] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p3_139-162.png` |
| H | handwritten_chn_1ae487be.pdf | 3 | [142, 141, 164, 165] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p3_142-141-164-165.png` |
| H | handwritten_chn_1ae487be.pdf | 3 | [144, 168] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p3_144-168.png` |
| H | handwritten_chn_1ae487be.pdf | 3 | [146, 170] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p3_146-170.png` |
| H | handwritten_chn_1ae487be.pdf | 3 | [150, 174] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p3_150-174.png` |
| H | handwritten_chn_1ae487be.pdf | 3 | [152, 176] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p3_152-176.png` |
| H | handwritten_chn_1ae487be.pdf | 4 | [184, 201] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p4_184-201.png` |
| H | handwritten_chn_1ae487be.pdf | 4 | [190, 207] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p4_190-207.png` |
| H | handwritten_chn_1ae487be.pdf | 4 | [211, 222] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p4_211-222.png` |
| H | handwritten_chn_1ae487be.pdf | 4 | [213, 226] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p4_213-226.png` |
| H | handwritten_chn_1ae487be.pdf | 4 | [214, 227] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p4_214-227.png` |
| H | handwritten_chn_1ae487be.pdf | 4 | [216, 229] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p4_216-229.png` |
| H | handwritten_chn_1ae487be.pdf | 4 | [218, 231] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p4_218-231.png` |
| H | handwritten_chn_1ae487be.pdf | 4 | [219, 233] | two columns of separate chemical equations | `./H_handwritten_chn_1ae487be_p4_219-233.png` |
| H | handwritten_chn_50d71ae2.pdf | 1 | [18, 20] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p1_18-20.png` |
| H | handwritten_chn_50d71ae2.pdf | 3 | [39, 40] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p3_39-40.png` |
| H | handwritten_chn_50d71ae2.pdf | 3 | [41, 42] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p3_41-42.png` |
| H | handwritten_chn_50d71ae2.pdf | 3 | [43, 44] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p3_43-44.png` |
| H | handwritten_chn_50d71ae2.pdf | 3 | [45, 46] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p3_45-46.png` |
| H | handwritten_chn_50d71ae2.pdf | 3 | [61, 62, 63] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p3_61-62-63.png` |
| H | handwritten_chn_50d71ae2.pdf | 5 | [87, 88] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p5_87-88.png` |
| H | handwritten_chn_50d71ae2.pdf | 5 | [89, 90] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p5_89-90.png` |
| H | handwritten_chn_50d71ae2.pdf | 6 | [106, 111] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p6_106-111.png` |
| H | handwritten_chn_50d71ae2.pdf | 6 | [107, 112] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p6_107-112.png` |
| H | handwritten_chn_50d71ae2.pdf | 6 | [109, 115] | independent formulas laid out side by side (table/slide layout) | `./H_handwritten_chn_50d71ae2_p6_109-115.png` |
| H | paper_eng_20a93115.pdf | 13 | [252, 253] | two complete formulas side by side on one display line, no shared equation number (A1-like, left alone) | `./H_paper_eng_20a93115_p13_252-253.png` |
| H | paper_eng_20a93115.pdf | 14 | [296, 297] | two complete formulas side by side on one display line, no shared equation number (A1-like, left alone) | `./H_paper_eng_20a93115_p14_296-297.png` |
| H | paper_eng_20a93115.pdf | 16 | [320, 321] | two complete formulas side by side on one display line, no shared equation number (A1-like, left alone) | `./H_paper_eng_20a93115_p16_320-321.png` |
| H | paper_eng_20a93115.pdf | 16 | [334, 335] | two complete formulas side by side on one display line, no shared equation number (A1-like, left alone) | `./H_paper_eng_20a93115_p16_334-335.png` |
| H | paper_eng_4b1e42ed.pdf | 4 | [38, 39] | two complete formulas side by side on one display line, no shared equation number (A1-like, left alone) | `./H_paper_eng_4b1e42ed_p4_38-39.png` |
| H | paper_eng_64b5e1f4.pdf | 7 | [101, 109] | boxes are in different text columns | `./H_paper_eng_64b5e1f4_p7_101-109.png` |
| H | paper_eng_d76efc79.pdf | 4 | [85, 101] | boxes are in different text columns | `./H_paper_eng_d76efc79_p4_85-101.png` |
| H | pptSlides_chn_60917576.pdf | 2 | [12, 14] | independent formulas laid out side by side (table/slide layout) | `./H_pptSlides_chn_60917576_p2_12-14.png` |
| H | pptSlides_chn_60917576.pdf | 3 | [28, 30] | independent formulas laid out side by side (table/slide layout) | `./H_pptSlides_chn_60917576_p3_28-30.png` |
| V | examPaper_eng_0b2fd078.pdf | 2 | [33, 35] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_0b2fd078_p2_33-35.png` |
| V | examPaper_eng_0b2fd078.pdf | 3 | [41, 42] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_0b2fd078_p3_41-42.png` |
| V | examPaper_eng_23110a64.pdf | 2 | [22, 23] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_23110a64_p2_22-23.png` |
| V | examPaper_eng_23110a64.pdf | 9 | [125, 126] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_23110a64_p9_125-126.png` |
| V | examPaper_eng_23110a64.pdf | 13 | [176, 179] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_23110a64_p13_176-179.png` |
| V | examPaper_eng_2a2d3def.pdf | 5 | [48, 49, 50, 51, 52, 53] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_2a2d3def_p5_48-49-50-51-52-53.png` |
| V | examPaper_eng_66b20269.pdf | 3 | [42, 43, 44, 45, 46, 47, 48] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_66b20269_p3_42-43-44-45-46-47-48.png` |
| V | examPaper_eng_6c2631df.pdf | 2 | [25, 26] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_6c2631df_p2_25-26.png` |
| V | examPaper_eng_d1508a80.pdf | 1 | [13, 15, 17] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_d1508a80_p1_13-15-17.png` |
| V | examPaper_eng_d1508a80.pdf | 1 | [25, 26, 27] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_d1508a80_p1_25-26-27.png` |
| V | examPaper_eng_d1508a80.pdf | 2 | [28, 29, 30] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_d1508a80_p2_28-29-30.png` |
| V | examPaper_eng_dd12c9cc.pdf | 1 | [9, 10] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_dd12c9cc_p1_9-10.png` |
| V | examPaper_eng_dd12c9cc.pdf | 2 | [18, 19] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_dd12c9cc_p2_18-19.png` |
| V | examPaper_eng_dd12c9cc.pdf | 4 | [38, 39] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_dd12c9cc_p4_38-39.png` |
| V | examPaper_eng_dd12c9cc.pdf | 8 | [66, 67] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_dd12c9cc_p8_66-67.png` |
| V | examPaper_eng_f46d7359.pdf | 2 | [26, 27, 28, 29, 30] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_examPaper_eng_f46d7359_p2_26-27-28-29-30.png` |
| V | handwritten_chn_1ae487be.pdf | 1 | [44, 46, 48] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p1_44-46-48.png` |
| V | handwritten_chn_1ae487be.pdf | 2 | [55, 57] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p2_55-57.png` |
| V | handwritten_chn_1ae487be.pdf | 2 | [63, 64] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p2_63-64.png` |
| V | handwritten_chn_1ae487be.pdf | 3 | [126, 128] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p3_126-128.png` |
| V | handwritten_chn_1ae487be.pdf | 3 | [141, 142] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p3_141-142.png` |
| V | handwritten_chn_1ae487be.pdf | 3 | [150, 151, 152] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p3_150-151-152.png` |
| V | handwritten_chn_1ae487be.pdf | 3 | [164, 165] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p3_164-165.png` |
| V | handwritten_chn_1ae487be.pdf | 4 | [181, 182] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p4_181-182.png` |
| V | handwritten_chn_1ae487be.pdf | 4 | [184, 185, 186] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p4_184-185-186.png` |
| V | handwritten_chn_1ae487be.pdf | 4 | [204, 205] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p4_204-205.png` |
| V | handwritten_chn_1ae487be.pdf | 4 | [213, 214] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p4_213-214.png` |
| V | handwritten_chn_1ae487be.pdf | 4 | [218, 219] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p4_218-219.png` |
| V | handwritten_chn_1ae487be.pdf | 4 | [221, 222] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p4_221-222.png` |
| V | handwritten_chn_1ae487be.pdf | 4 | [226, 227] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p4_226-227.png` |
| V | handwritten_chn_1ae487be.pdf | 4 | [233, 234] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_1ae487be_p4_233-234.png` |
| V | handwritten_chn_50d71ae2.pdf | 1 | [5, 6] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p1_5-6.png` |
| V | handwritten_chn_50d71ae2.pdf | 1 | [9, 10] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p1_9-10.png` |
| V | handwritten_chn_50d71ae2.pdf | 1 | [15, 16, 17, 18, 20, 22] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p1_15-16-17-18-20-22.png` |
| V | handwritten_chn_50d71ae2.pdf | 2 | [29, 31, 32, 35] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p2_29-31-32-35.png` |
| V | handwritten_chn_50d71ae2.pdf | 3 | [39, 41, 43, 45] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p3_39-41-43-45.png` |
| V | handwritten_chn_50d71ae2.pdf | 3 | [40, 42, 44, 46] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p3_40-42-44-46.png` |
| V | handwritten_chn_50d71ae2.pdf | 4 | [66, 67] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p4_66-67.png` |
| V | handwritten_chn_50d71ae2.pdf | 4 | [74, 75] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p4_74-75.png` |
| V | handwritten_chn_50d71ae2.pdf | 5 | [80, 81] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p5_80-81.png` |
| V | handwritten_chn_50d71ae2.pdf | 5 | [86, 88, 87, 90, 89, 91, 92] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p5_86-88-87-90-89-91-92.png` |
| V | handwritten_chn_50d71ae2.pdf | 6 | [103, 104] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p6_103-104.png` |
| V | handwritten_chn_50d71ae2.pdf | 6 | [106, 107, 108, 109] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p6_106-107-108-109.png` |
| V | handwritten_chn_50d71ae2.pdf | 6 | [111, 112] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p6_111-112.png` |
| V | handwritten_chn_50d71ae2.pdf | 6 | [115, 116] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_50d71ae2_p6_115-116.png` |
| V | handwritten_chn_71864fb7.pdf | 1 | [13, 14] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_71864fb7_p1_13-14.png` |
| V | handwritten_chn_71864fb7.pdf | 3 | [52, 54] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_71864fb7_p3_52-54.png` |
| V | handwritten_chn_71864fb7.pdf | 4 | [70, 72, 74] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_71864fb7_p4_70-72-74.png` |
| V | handwritten_chn_a6f48a98.pdf | 4 | [56, 57, 58] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_a6f48a98_p4_56-57-58.png` |
| V | handwritten_chn_a6f48a98.pdf | 7 | [99, 100, 101, 102] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_a6f48a98_p7_99-100-101-102.png` |
| V | handwritten_chn_d1ec88bb.pdf | 3 | [29, 30] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_d1ec88bb_p3_29-30.png` |
| V | handwritten_chn_eac2d8c2.pdf | 1 | [8, 9, 10, 11, 12, 13] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_eac2d8c2_p1_8-9-10-11-12-13.png` |
| V | handwritten_chn_eac2d8c2.pdf | 3 | [49, 50] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_eac2d8c2_p3_49-50.png` |
| V | handwritten_chn_edc60213.pdf | 2 | [27, 28] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_handwritten_chn_edc60213_p2_27-28.png` |
| V | paper_chn_3567e55d.pdf | 2 | [46, 48] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_chn_3567e55d_p2_46-48.png` |
| V | paper_chn_5650c4e6.pdf | 2 | [44, 46] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_chn_5650c4e6_p2_44-46.png` |
| V | paper_chn_5650c4e6.pdf | 2 | [52, 54] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_chn_5650c4e6_p2_52-54.png` |
| V | paper_chn_5650c4e6.pdf | 2 | [57, 58] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_chn_5650c4e6_p2_57-58.png` |
| V | paper_chn_5650c4e6.pdf | 3 | [100, 102] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_chn_5650c4e6_p3_100-102.png` |
| V | paper_chn_be9986ff.pdf | 3 | [46, 47] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_chn_be9986ff_p3_46-47.png` |
| V | paper_chn_be9986ff.pdf | 3 | [50, 51] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_chn_be9986ff_p3_50-51.png` |
| V | paper_chn_fdabe9a2.pdf | 1 | [24, 27] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_chn_fdabe9a2_p1_24-27.png` |
| V | paper_chn_fdabe9a2.pdf | 1 | [25, 28] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_chn_fdabe9a2_p1_25-28.png` |
| V | paper_eng_01d0081c.pdf | 2 | [21, 23] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_01d0081c_p2_21-23.png` |
| V | paper_eng_20a93115.pdf | 16 | [317, 320] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_20a93115_p16_317-320.png` |
| V | paper_eng_b8670bda.pdf | 2 | [20, 22] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_b8670bda_p2_20-22.png` |
| V | paper_eng_b8670bda.pdf | 4 | [54, 56, 57, 60, 59] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_b8670bda_p4_54-56-57-60-59.png` |
| V | paper_eng_b8670bda.pdf | 5 | [75, 77, 79, 81, 83] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_b8670bda_p5_75-77-79-81-83.png` |
| V | paper_eng_b8670bda.pdf | 8 | [130, 132] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_b8670bda_p8_130-132.png` |
| V | paper_eng_b8670bda.pdf | 10 | [177, 179, 181] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_b8670bda_p10_177-179-181.png` |
| V | paper_eng_b8670bda.pdf | 12 | [214, 216] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_b8670bda_p12_214-216.png` |
| V | paper_eng_b8670bda.pdf | 14 | [262, 265] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_b8670bda_p14_262-265.png` |
| V | paper_eng_b8670bda.pdf | 15 | [296, 297] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_b8670bda_p15_296-297.png` |
| V | paper_eng_b8670bda.pdf | 17 | [341, 343, 346, 348] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_b8670bda_p17_341-343-346-348.png` |
| V | paper_eng_b8670bda.pdf | 18 | [374, 376] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_b8670bda_p18_374-376.png` |
| V | paper_eng_bba70704.pdf | 3 | [38, 39] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_bba70704_p3_38-39.png` |
| V | paper_eng_bba70704.pdf | 3 | [41, 42] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_bba70704_p3_41-42.png` |
| V | paper_eng_bba70704.pdf | 3 | [46, 47, 48, 49] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_bba70704_p3_46-47-48-49.png` |
| V | paper_eng_c1aa81bd.pdf | 13 | [193, 194] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_c1aa81bd_p13_193-194.png` |
| V | paper_eng_d76efc79.pdf | 3 | [51, 53] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_d76efc79_p3_51-53.png` |
| V | paper_eng_ff3a3862.pdf | 6 | [59, 61] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_paper_eng_ff3a3862_p6_59-61.png` |
| V | pptSlides_chn_60917576.pdf | 2 | [11, 12] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_pptSlides_chn_60917576_p2_11-12.png` |
| V | pptSlides_chn_60917576.pdf | 3 | [27, 28] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_pptSlides_chn_60917576_p3_27-28.png` |
| V | pptSlides_chn_60917576.pdf | 3 | [30, 31] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_pptSlides_chn_60917576_p3_30-31.png` |
| V | pptSlides_chn_60917576.pdf | 4 | [41, 43, 45] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_pptSlides_chn_60917576_p4_41-43-45.png` |
| V | pptSlides_chn_60917576.pdf | 5 | [54, 56] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_pptSlides_chn_60917576_p5_54-56.png` |
| V | pptSlides_chn_7c522d1e.pdf | 5 | [31, 33, 35, 37] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_pptSlides_chn_7c522d1e_p5_31-33-35-37.png` |
| V | pptSlides_chn_7c522d1e.pdf | 6 | [41, 42] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_pptSlides_chn_7c522d1e_p6_41-42.png` |
| V | pptSlides_chn_7c522d1e.pdf | 9 | [65, 66] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_pptSlides_chn_7c522d1e_p9_65-66.png` |
| V | pptSlides_chn_d81f00a8.pdf | 8 | [56, 58] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_pptSlides_chn_d81f00a8_p8_56-58.png` |
| V | textBook_chn_12b0d2b4.pdf | 10 | [101, 102] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_textBook_chn_12b0d2b4_p10_101-102.png` |
| V | textBook_eng_a237ffc8.pdf | 12 | [120, 121, 122, 123] | separate display lines (own baselines; numbered separately or stacked list), no construct spanning the boxes | `./V_textBook_eng_a237ffc8_p12_120-121-122-123.png` |
| X | paper_chn_3567e55d.pdf | 2 | [32, 43] | boxes are in different text columns / slide regions with other content between | `./X_paper_chn_3567e55d_p2_32-43.png` |
| X | paper_chn_3567e55d.pdf | 2 | [35, 48] | boxes are in different text columns / slide regions with other content between | `./X_paper_chn_3567e55d_p2_35-48.png` |
| X | paper_chn_5650c4e6.pdf | 3 | [82, 115] | boxes are in different text columns / slide regions with other content between | `./X_paper_chn_5650c4e6_p3_82-115.png` |
| X | paper_eng_d76efc79.pdf | 3 | [48, 74] | boxes are in different text columns / slide regions with other content between | `./X_paper_eng_d76efc79_p3_48-74.png` |
| X | paper_eng_d76efc79.pdf | 4 | [88, 105] | boxes are in different text columns / slide regions with other content between | `./X_paper_eng_d76efc79_p4_88-105.png` |
| X | pptSlides_chn_60917576.pdf | 3 | [27, 30] | boxes are in different text columns / slide regions with other content between | `./X_pptSlides_chn_60917576_p3_27-30.png` |

Partial: paper_eng_20a93115 p4 [53, 54, 55] merged only 54+55 (formula + its qualifier `ℓ=2,…,L−1`); 53 is a separate complete formula and stays.
