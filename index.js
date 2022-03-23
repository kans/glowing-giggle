#! /usr/bin/env node

let wasm = require('./pkg');

function f() {
  let mrf = wasm.get_mrf();

  for (var i = 0; i < 30; i++) {
    mrf.stuff("fish" + i);
  }

  mrf.what();
}

for (var i = 0; i < 30; i++) {
  f();
}