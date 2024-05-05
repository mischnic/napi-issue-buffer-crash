const native = require("./index.node");

// native.transform({ /*filename: "xyz", */ code: Buffer.from("abc") });
native.transform({ /*filename: "xyz", */ code: Buffer.alloc(0) });
