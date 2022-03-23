#! /usr/bin/env node

let wasm = require('./pkg');

function f() {

  for (var i = 0; i < Math.floor(Math.random() * 373737); i++) {
    wasm.submit_input("fish" + i);
  }

  console.log("total: " + wasm.get_results());
}

while (true) {
  f();
  console.log(process.pid);
}