(self['webpackChunkwebpack'] = self['webpackChunkwebpack'] || []).push([["main"], {
"./index.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
const _inner = __webpack_require__("./inner.js");
const _module = __webpack_require__("./module.js");
it("export should be unused when only unused functions use it", ()=>{
    (0, _module.f1)();
    expect(_module.pureUsed).toBe(42);
    expect((0, _module.fWithDefault)()).toBe(42);
    if (process.env.NODE_ENV === "production") {
        expect(_inner.exportUsed).toBe(false);
        expect(_inner.export2Used).toBe(true);
        expect(_inner.export3Used).toBe(true);
        expect(_inner.export4Used).toBe(true);
        expect(_inner.export5Used).toBe(true);
        expect(_inner.export6Used).toBe(true);
    }
    return __webpack_require__.e("chunk_js").then(__webpack_require__.bind(__webpack_require__, "./chunk.js")).then(__webpack_require__.interopRequire);
});
},
"./inner.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        enumerable: true,
        get: all[name]
    });
}
_export(exports, {
    EXPORT: ()=>EXPORT,
    EXPORT2: ()=>EXPORT2,
    EXPORT3: ()=>EXPORT3,
    EXPORT4: ()=>EXPORT4,
    EXPORT5: ()=>EXPORT5,
    EXPORT6: ()=>EXPORT6,
    exportUsed: ()=>exportUsed,
    export2Used: ()=>export2Used,
    export3Used: ()=>export3Used,
    export4Used: ()=>export4Used,
    export5Used: ()=>export5Used,
    export6Used: ()=>export6Used
});
const EXPORT = 42;
const EXPORT2 = 42;
const EXPORT3 = 42;
const EXPORT4 = 42;
const EXPORT5 = ()=>42;
const EXPORT6 = ()=>42;
const exportUsed = __webpack_exports_info__.EXPORT.used;
const export2Used = __webpack_exports_info__.EXPORT2.used;
const export3Used = __webpack_exports_info__.EXPORT3.used;
const export4Used = __webpack_exports_info__.EXPORT4.used;
const export5Used = __webpack_exports_info__.EXPORT5.used;
const export6Used = __webpack_exports_info__.EXPORT6.used;
},
"./module.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        enumerable: true,
        get: all[name]
    });
}
_export(exports, {
    f1: ()=>f1,
    pureUsed: ()=>pureUsed,
    fWithDefault: ()=>fWithDefault
});
const _inner = __webpack_require__("./inner.js");
function f1() {}
function f2() {
    return _inner.EXPORT;
}
const f5 = ()=>{
    return _inner.EXPORT;
};
let f6 = ()=>{
    return _inner.EXPORT;
};
const f7 = ()=>{
    return (0, _inner.EXPORT5)();
};
const f8 = ()=>{
    return (0, _inner.EXPORT6)();
};
let g5 = ()=>{
    return f5();
};
f6();
g5();
f7(f8());
f2("fwefe"), f2("efwefa");
f2(f2(), f2());
f2(class {
    f() {
        return _inner.EXPORT;
    }
});
f2(()=>_inner.EXPORT);
const pureUsed = _inner.EXPORT3;
function x1() {
    return _inner.EXPORT2;
}
const x2 = function x2() {
    return x1();
};
const x3 = ()=>{
    return x2();
};
x3();
function fWithDefault(r = _inner.EXPORT4) {
    return r;
}
},

},function(__webpack_require__) {
__webpack_require__("./index.js");
}
]);