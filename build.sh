#! /usr/bin/env bash

set -ex

wasm-pack build --no-typescript --out-dir "pkg" --target nodejs