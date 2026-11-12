// Regenerates src/katex/tables.rs from the vendored KaTeX (development aid, not run by the scorer):
//   node tools/gen_katex_tables.js ../../../metrics/cdm/modules/tokenize_latex/third_party/katex > src/katex/tables.rs

const K = require("path").resolve(process.argv[2]);
const symbols = require(K + "/src/symbols.js");
const functions = require(K + "/src/functions.js");
const envs = require(K + "/src/environments.js");
const handlers = new Map(); 
const out = {symbols: {math: {}, text: {}}, functions: {}, envs: {}};
for (const m of ["math", "text"]) for (const k of Object.keys(symbols[m])) out.symbols[m][k] = symbols[m][k].group;
function hid(h, name) { if (h === null) return "null"; if (!handlers.has(h)) handlers.set(h, name); return handlers.get(h); }
for (const k of Object.keys(functions)) { const f = functions[k]; out.functions[k] = {numArgs: f.numArgs, argTypes: f.argTypes, greediness: f.greediness, allowedInText: f.allowedInText, numOptionalArgs: f.numOptionalArgs, handler: hid(f.handler, k)}; }
for (const k of Object.keys(envs)) { const f = envs[k]; out.envs[k] = {numArgs: f.numArgs, argTypes: f.argTypes, greediness: f.greediness, numOptionalArgs: f.numOptionalArgs, handler: hid(f.handler, k)}; }

const t = out;
function lit(s) { let o = '"'; for (const ch of s) { const c = ch.codePointAt(0); if (c >= 0x20 && c < 0x7f && ch !== '"' && ch !== '\\') o += ch; else o += "\\u{" + c.toString(16) + "}"; } return o + '"'; }
const G = {accent:"Accent",bin:"Bin",close:"Close",inner:"Inner",mathord:"Mathord",op:"Op",open:"Open",punct:"Punct",rel:"Rel",spacing:"Spacing",textord:"Textord"};
const H = {"\\sqrt":"Sqrt","\\text":"Text","\\color":"Color","\\overline":"Overline","\\underline":"Underline","\\rule":"Rule","\\KaTeX":"Katex","\\phantom":"Phantom","\\blue":"ColorFn","\\arcsin":"OpNoLimitsNoSymbol","\\det":"OpLimitsNoSymbol","\\int":"OpNoLimitsSymbol","\\coprod":"OpLimitsSymbol","\\dfrac":"Genfrac","\\llap":"Lap","\\bigl":"Delim","null":"Null","\\mathrm":"Font","\\acute":"Accent","\\over":"Infix","\\\\":"Cr","\\begin":"Environment"};
const E = {array:"Array",tabular:"Tabular",matrix:"Matrix",picture:"Cases",cases:"Cases",aligned:"Aligned"};
const A = {text:"Text",color:"Color",size:"Size",original:"Original"};
let o = "// @generated from the vendored KaTeX (src/symbols.js, src/functions.js, src/environments.js)\n// of metrics/cdm/modules/tokenize_latex/third_party/katex. Do not edit.\n\nuse super::{ArgType, EnvHandler, FuncSpec, Group, Handler};\n\n";
for (const m of ["math", "text"]) {
  const e = Object.entries(t.symbols[m]);
  o += `pub(super) static ${m.toUpperCase()}_SYMBOLS: [(&str, Group); ${e.length}] = [\n`;
  for (const [k, v] of e) o += `    (${lit(k)}, Group::${G[v]}),\n`;
  o += "];\n\n";
}
const f = Object.entries(t.functions);
o += `pub(super) static FUNCTIONS: [(&str, FuncSpec); ${f.length}] = [\n`;
for (const [k, v] of f) {
  if (v.picture) throw 1;
  const at = v.argTypes ? "Some(&[" + v.argTypes.map(a => "ArgType::" + A[a]).join(", ") + "])" : "None";
  if (!(v.handler in H)) throw new Error(v.handler);
  o += `    (${lit(k)}, FuncSpec { num_args: ${v.numArgs}, num_optional: ${v.numOptionalArgs}, greediness: ${v.greediness}, arg_types: ${at}, handler: Handler::${H[v.handler]} }),\n`;
}
o += "];\n\n";
const en = Object.entries(t.envs);
o += `pub(super) static ENVIRONMENTS: [(&str, EnvHandler, u32); ${en.length}] = [\n`;
for (const [k, v] of en) { if (v.argTypes || v.numOptionalArgs || v.greediness !== 1) throw new Error(k); o += `    (${lit(k)}, EnvHandler::${E[v.handler]}, ${v.numArgs}),\n`; }
o += "];\n";
process.stdout.write(o);
