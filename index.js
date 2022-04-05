#! /usr/bin/env node

// let wasm = require('./pkg');
const neonGiggle = require('.');

function f() {
  for (var i = 0; i < Math.floor(Math.random() * 373737); i++) {
    neonGiggle.submit_input("fish" + i);
  }

  console.log(`total: ${neonGiggle.get_results()}`);
}

f();
