parcelRequire=function(e,r,t,n){var i,o="function"==typeof parcelRequire&&parcelRequire,u="function"==typeof require&&require;function f(t,n){if(!r[t]){if(!e[t]){var i="function"==typeof parcelRequire&&parcelRequire;if(!n&&i)return i(t,!0);if(o)return o(t,!0);if(u&&"string"==typeof t)return u(t);var c=new Error("Cannot find module '"+t+"'");throw c.code="MODULE_NOT_FOUND",c}p.resolve=function(r){return e[t][1][r]||r},p.cache={};var l=r[t]=new f.Module(t);e[t][0].call(l.exports,p,l,l.exports,this)}return r[t].exports;function p(e){return f(p.resolve(e))}}f.isParcelRequire=!0,f.Module=function(e){this.id=e,this.bundle=f,this.exports={}},f.modules=e,f.cache=r,f.parent=o,f.register=function(r,t){e[r]=[function(e,r){r.exports=t},{}]};for(var c=0;c<t.length;c++)try{f(t[c])}catch(e){i||(i=e)}if(t.length){var l=f(t[t.length-1]);"object"==typeof exports&&"undefined"!=typeof module?module.exports=l:"function"==typeof define&&define.amd?define(function(){return l}):n&&(this[n]=l)}if(parcelRequire=f,i)throw i;return f}({"PfKt":[function(require,module,exports) {
"use strict";Object.defineProperty(exports,"__esModule",{value:!0}),exports.Canvas=exports.__wbindgen_throw=exports.__wbindgen_object_drop_ref=exports.__wbg_error_4bb6c2a97407129a=exports.__wbg_stack_558ba5917b466edd=exports.__wbg_new_59cb74e423758ede=exports.default=void 0;var e=_(require("./pkg/wave_equation_bg.wasm"));function _(e){return e&&e.__esModule?e:{default:e}}var r=e.default;exports.default=r;var t=e.default.__wbg_new_59cb74e423758ede;exports.__wbg_new_59cb74e423758ede=t;var a=e.default.__wbg_stack_558ba5917b466edd;exports.__wbg_stack_558ba5917b466edd=a;var b=e.default.__wbg_error_4bb6c2a97407129a;exports.__wbg_error_4bb6c2a97407129a=b;var o=e.default.__wbindgen_object_drop_ref;exports.__wbindgen_object_drop_ref=o;var d=e.default.__wbindgen_throw;exports.__wbindgen_throw=d;var s=e.default.Canvas;exports.Canvas=s;
},{"./pkg/wave_equation_bg.wasm":"EX1K"}],"QvaY":[function(require,module,exports) {
"use strict";var e=n(require("../crate/Cargo.toml"));function t(){if("function"!=typeof WeakMap)return null;var e=new WeakMap;return t=function(){return e},e}function n(e){if(e&&e.__esModule)return e;var n=t();if(n&&n.has(e))return n.get(e);var r={};if(null!=e){var a=Object.defineProperty&&Object.getOwnPropertyDescriptor;for(var o in e)if(Object.prototype.hasOwnProperty.call(e,o)){var i=a?Object.getOwnPropertyDescriptor(e,o):null;i&&(i.get||i.set)?Object.defineProperty(r,o,i):r[o]=e[o]}}return r.default=e,n&&n.set(e,r),r}var r=document.getElementById("canvas"),a=e.Canvas.new(r.width,r.height),o=a.width(),i=a.height(),u=r.getContext("2d"),c=document.getElementById("step-btn");c.addEventListener("click",f);var l=document.getElementById("fps"),d=document.getElementById("frameCount");function f(){a.step(0,!1),g++,d.innerText=g;var t=a.image(),n=new Uint8ClampedArray(e.default.wasm.memory.buffer,t,4*i*o),r=new ImageData(n,o,i);u.putImageData(r,0,0)}var m=0,s=1,p=m-s,g=0,y=null;function v(){y=null,m=performance.now(),f(),s=performance.now(),p=s-m,w()}function w(){g%10==0&&(l.innerText=Math.floor(1e3/p)),y||(y=requestAnimationFrame(v))}var b=document.getElementById("start"),h=!1;b.addEventListener("click",function(){(h=!h)?(w(),b.innerHTML="Stop",b.style.backgroundColor="red"):(cancelAnimationFrame(y),y=null,b.innerHTML="Start",b.style.backgroundColor="greenyellow")});
},{"../crate/Cargo.toml":"PfKt"}],"Bh1I":[function(require,module,exports) {
var t=null;function e(){return t||(t=n()),t}function n(){try{throw new Error}catch(e){var t=(""+e.stack).match(/(https?|file|ftp|chrome-extension|moz-extension):\/\/[^)\n]+/g);if(t)return r(t[0])}return"/"}function r(t){return(""+t).replace(/^((?:https?|file|ftp|chrome-extension|moz-extension):\/\/.+)\/[^\/]+$/,"$1")+"/"}exports.getBundleURL=e,exports.getBaseURL=r;
},{}],"z1Am":[function(require,module,exports) {
var r=require("./bundle-url").getBundleURL;function e(r){Array.isArray(r)||(r=[r]);var e=r[r.length-1];try{return Promise.resolve(require(e))}catch(n){if("MODULE_NOT_FOUND"===n.code)return new s(function(n,i){t(r.slice(0,-1)).then(function(){return require(e)}).then(n,i)});throw n}}function t(r){return Promise.all(r.map(u))}var n={};function i(r,e){n[r]=e}module.exports=exports=e,exports.load=t,exports.register=i;var o={};function u(e){var t;if(Array.isArray(e)&&(t=e[1],e=e[0]),o[e])return o[e];var i=(e.substring(e.lastIndexOf(".")+1,e.length)||e).toLowerCase(),u=n[i];return u?o[e]=u(r()+e).then(function(r){return r&&module.bundle.register(t,r),r}).catch(function(r){throw delete o[e],r}):void 0}function s(r){this.executor=r,this.promise=null}s.prototype.then=function(r,e){return null===this.promise&&(this.promise=new Promise(this.executor)),this.promise.then(r,e)},s.prototype.catch=function(r){return null===this.promise&&(this.promise=new Promise(this.executor)),this.promise.catch(r)};
},{"./bundle-url":"Bh1I"}],"sC8V":[function(require,module,exports) {

},{}],"ocK6":[function(require,module,exports) {
var __dirname = "/home/trent/trent/dev/rust-waves/node_modules/parcel-plugin-wasm.rs";
var e,t="/home/trent/trent/dev/rust-waves/node_modules/parcel-plugin-wasm.rs";const n={},r=new Array(32).fill(void 0);function o(e){return r[e]}r.push(void 0,null,!0,!1);let s=r.length;function i(e){e<36||(r[e]=s,s=e)}function a(e){const t=o(e);return i(e),t}const u="undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder;let c=new u("utf-8",{ignoreBOM:!0,fatal:!0});c.decode();let f=null;function _(){return null!==f&&f.buffer===e.memory.buffer||(f=new Uint8Array(e.memory.buffer)),f}function l(e,t){return c.decode(_().subarray(e,e+t))}function d(e){s===r.length&&r.push(r.length+1);const t=s;return s=r[t],r[t]=e,t}let b=0;const w="undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder;let h=new w("utf-8");const g="function"==typeof h.encodeInto?function(e,t){return h.encodeInto(e,t)}:function(e,t){const n=h.encode(e);return t.set(n),{read:e.length,written:n.length}};function m(e,t,n){if(void 0===n){const n=h.encode(e),r=t(n.length);return _().subarray(r,r+n.length).set(n),b=n.length,r}let r=e.length,o=t(r);const s=_();let i=0;for(;i<r;i++){const t=e.charCodeAt(i);if(t>127)break;s[o+i]=t}if(i!==r){0!==i&&(e=e.slice(i)),o=n(o,r,r=i+3*e.length);const t=_().subarray(o+i,o+r);i+=g(e,t).written}return b=i,o}let p=null;function y(){return null!==p&&p.buffer===e.memory.buffer||(p=new Int32Array(e.memory.buffer)),p}class v{static __wrap(e){const t=Object.create(v.prototype);return t.ptr=e,t}__destroy_into_raw(){const e=this.ptr;return this.ptr=0,e}free(){const t=this.__destroy_into_raw();e.__wbg_canvas_free(t)}static new(t,n){var r=e.canvas_new(t,n);return v.__wrap(r)}step(t,n){e.canvas_step(this.ptr,t,n)}image(){return e.canvas_image(this.ptr)}test(){return e.canvas_test(this.ptr)}width(){return e.canvas_width(this.ptr)>>>0}height(){return e.canvas_height(this.ptr)>>>0}}function x(t){const r=fetch(t);let o;return(o="function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(r,{"./wave_equation_bg.js":n}):r.then(e=>e.arrayBuffer()).then(e=>WebAssembly.instantiate(e,{"./wave_equation_bg.js":n}))).then(({instance:t})=>{e=x.wasm=t.exports,n.wasm=e})}function A(r){const o=require("fs");return new Promise(function(e,n){o.readFile(t+r,function(t,r){t?n(t):e(r.buffer)})}).then(e=>WebAssembly.instantiate(e,{"./wave_equation_bg":n})).then(({instance:t})=>{e=x.wasm=t.exports,n.wasm=e})}n.__wbg_new_59cb74e423758ede=function(){return d(new Error)},n.__wbg_stack_558ba5917b466edd=function(t,n){var r=m(o(n).stack,e.__wbindgen_malloc,e.__wbindgen_realloc),s=b;y()[t/4+1]=s,y()[t/4+0]=r},n.__wbg_error_4bb6c2a97407129a=function(t,n){try{console.error(l(t,n))}finally{e.__wbindgen_free(t,n)}},n.__wbindgen_object_drop_ref=function(e){a(e)},n.__wbindgen_throw=function(e,t){throw new Error(l(e,t))},n.Canvas=v;const q=Object.assign(x,n);module.exports=function(e){return q(e).then(()=>n)};
},{"fs":"sC8V"}],0:[function(require,module,exports) {
var b=require("z1Am");b.register("wasm",require("ocK6"));b.load([["wave_equation_bg.7a488b96.wasm","EX1K"]]).then(function(){require("QvaY");});
},{}]},{},[0], null)
//# sourceMappingURL=js.07054247.js.map