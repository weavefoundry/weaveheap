import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);

let addon;
try {
  addon = require('../../target/debug/weaveheap_node.node');
} catch (e) {
  try {
    addon = require('weaveheap-node');
  } catch {
    console.log('weaveheap-node addon not found (scaffold).');
    process.exit(0);
  }
}

const fn = addon.version_major || addon.versionMajor;
console.log('versionMajor:', typeof fn === 'function' ? fn() : 'unavailable');
