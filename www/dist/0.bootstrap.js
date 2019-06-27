(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/sassruist.js":
/*!***************************!*\
  !*** ../pkg/sassruist.js ***!
  \***************************/
/*! exports provided: return_string */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"return_string\", function() { return return_string; });\n/* harmony import */ var _sassruist_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./sassruist_bg */ \"../pkg/sassruist_bg.wasm\");\n\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachedTextEncoder = new TextEncoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nlet passStringToWasm;\nif (typeof cachedTextEncoder.encodeInto === 'function') {\n    passStringToWasm = function(arg) {\n\n\n        let size = arg.length;\n        let ptr = _sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](size);\n        let offset = 0;\n        {\n            const mem = getUint8Memory();\n            for (; offset < arg.length; offset++) {\n                const code = arg.charCodeAt(offset);\n                if (code > 0x7F) break;\n                mem[ptr + offset] = code;\n            }\n        }\n\n        if (offset !== arg.length) {\n            arg = arg.slice(offset);\n            ptr = _sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"](ptr, size, size = offset + arg.length * 3);\n            const view = getUint8Memory().subarray(ptr + offset, ptr + size);\n            const ret = cachedTextEncoder.encodeInto(arg, view);\n\n            offset += ret.written;\n        }\n        WASM_VECTOR_LEN = offset;\n        return ptr;\n    };\n} else {\n    passStringToWasm = function(arg) {\n\n\n        let size = arg.length;\n        let ptr = _sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](size);\n        let offset = 0;\n        {\n            const mem = getUint8Memory();\n            for (; offset < arg.length; offset++) {\n                const code = arg.charCodeAt(offset);\n                if (code > 0x7F) break;\n                mem[ptr + offset] = code;\n            }\n        }\n\n        if (offset !== arg.length) {\n            const buf = cachedTextEncoder.encode(arg.slice(offset));\n            ptr = _sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"](ptr, size, size = offset + buf.length);\n            getUint8Memory().set(buf, ptr + offset);\n            offset += buf.length;\n        }\n        WASM_VECTOR_LEN = offset;\n        return ptr;\n    };\n}\n\nlet cachedTextDecoder = new TextDecoder('utf-8');\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nlet cachedGlobalArgumentPtr = null;\nfunction globalArgumentPtr() {\n    if (cachedGlobalArgumentPtr === null) {\n        cachedGlobalArgumentPtr = _sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_global_argument_ptr\"]();\n    }\n    return cachedGlobalArgumentPtr;\n}\n\nlet cachegetUint32Memory = null;\nfunction getUint32Memory() {\n    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== _sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory = new Uint32Array(_sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory;\n}\n/**\n* @param {string} text\n* @returns {string}\n*/\nfunction return_string(text) {\n    const ptr0 = passStringToWasm(text);\n    const len0 = WASM_VECTOR_LEN;\n    const retptr = globalArgumentPtr();\n    _sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"return_string\"](retptr, ptr0, len0);\n    const mem = getUint32Memory();\n    const rustptr = mem[retptr / 4];\n    const rustlen = mem[retptr / 4 + 1];\n\n    const realRet = getStringFromWasm(rustptr, rustlen).slice();\n    _sassruist_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](rustptr, rustlen * 1);\n    return realRet;\n\n}\n\n\n\n//# sourceURL=webpack:///../pkg/sassruist.js?");

/***/ }),

/***/ "../pkg/sassruist_bg.wasm":
/*!********************************!*\
  !*** ../pkg/sassruist_bg.wasm ***!
  \********************************/
/*! exports provided: memory, return_string, __wbindgen_global_argument_ptr, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/sassruist_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _pkg_sassruist__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../pkg/sassruist */ \"../pkg/sassruist.js\");\n\n\ndocument.getElementById('convert').addEventListener('click', convert)\n\nfunction convert () {\n\tconst text = document.getElementById('src').value\n\tconst result = _pkg_sassruist__WEBPACK_IMPORTED_MODULE_0__[\"return_string\"](text)\n\tdocument.getElementById('result').value = result\n}\n\nconst text = [\n\t'a {',\n\t'  &_b {',\n\t'    &_c {',\n\t'      color: red;',\n\t'    }',\n\t'  }',\n\t'}',\n].join('\\n')\ndocument.getElementById('src').value = text\nconvert()\n\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);