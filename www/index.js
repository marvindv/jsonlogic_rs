import { apply as wasmApply, Rule } from 'json-logic-wasm';

import { apply as jsApply } from 'json-logic-js';

const do_test = (txt, fn) => {
  console.time(txt);
  const res = fn();
  console.timeEnd(txt);
  console.warn(txt, res)
  return res;
}

const js_fizzbuzz = (index) => {
  const rule = {
    "if": [
      {"==": [ { "%": [ { "var": "i" }, 15 ] }, 0]},
      "fizzbuzz",
  
      {"==": [ { "%": [ { "var": "i" }, 3 ] }, 0]},
      "fizz",
  
      {"==": [ { "%": [ { "var": "i" }, 5 ] }, 0]},
      "buzz",
  
      { "var": "i" }
    ]
  };
  const data = { i: index };

  return jsApply(rule, data);
};

const wasm_fizzbuzz = (index) => {
  const rule = {
    "if": [
      {"==": [ { "%": [ { "var": "i" }, 15 ] }, 0]},
      "fizzbuzz",
  
      {"==": [ { "%": [ { "var": "i" }, 3 ] }, 0]},
      "fizz",
  
      {"==": [ { "%": [ { "var": "i" }, 5 ] }, 0]},
      "buzz",
  
      { "var": "i" }
    ]
  };
  const data = { i: index };

  return wasmApply(rule, data);
};

const wasm2_fizzbuzz = (index) => {
  return wasmFizzbuzz(index);
};

const n = 100;

do_test("wasm1 fizzbuzz", () => {
  for(var i=1; i<=n ; i++){
    wasm_fizzbuzz(i);
  }
});

do_test("wasm3 fizzbuzz", () => {
  const ast = Rule.compile({
    "if": [
      {"==": [ { "%": [ { "var": "i" }, 15 ] }, 0]},
      "fizzbuzz",
  
      {"==": [ { "%": [ { "var": "i" }, 3 ] }, 0]},
      "fizz",
  
      {"==": [ { "%": [ { "var": "i" }, 5 ] }, 0]},
      "buzz",
  
      { "var": "i" }
    ]
  });
  
  for (let i = 1; i <= n; i++) {
    ast.apply({ i });
  }
});

do_test("js1   fizzbuzz", () => {
  for(var i=1; i<=n ; i++){
    js_fizzbuzz(i);
  }
});

console.warn(wasmApply({ "!!": 1 }, undefined))
