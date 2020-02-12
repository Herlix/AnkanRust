# rust_talk

Hi! 

This is the repo for my "Why Rust?" talk held @ FooCafe in Malmö Sweden in January 2020.

Wasm frontend: https://github.com/yewstack

My goal is to have this website as slides instead of powerpoint or something similar. 
Why make life easy? ;) 


## 🚴 Usage

### 🔬 Build & Serve locally with

```
yarn && yarn build && yarn start:dev
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
