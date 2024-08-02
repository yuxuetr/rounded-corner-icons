# Rust & WASM 图标直角变圆角

## 创建 Rust 库

```shell
# /path/to/rounded-corner-icons
cargo new make-rounded-corner-icons-wasm --lib
```

## Rust Wasm 安装依赖

```toml
# /path/to/rounded-corner-icons/Cargo.toml
[package]
edition = "2021"
license = "MIT"
name = "wasm_rounded_icon"
version = "0.1.0"

[dependencies]
console_error_panic_hook = {version = "0.1.7", optional = true}
image = "0.25.2"
imageproc = "0.25.0"
wasm-bindgen = "0.2.92"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = ["console_error_panic_hook"]

[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

## 创建 React 项目

```shell
# /path/to/rounded-corner-icons
pnpm create vite@latest wasm-rounded-icon --template react
```

## 已编写完成的 Rust WASM 库构建 WASM 库

```shell
# /path/to/rounded-corner-icons/make-rounded-corner-icons-wasm
wasm-pack build --out-dir ../wasm-rounded-icon/src/pkg --target web --release
```

## 配置`vite.config.ts`

```ts
// /path/to/rounded-corner-icons/wasm-rounded-icon/vite.config.ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  optimizeDeps: {
    exclude: ["src/pkg/wasm_rounded_icon"],
  },
});
```

## 运行程序

```shell
# /path/to/rounded-corner-icons/wasm-rounded-icon
pnpm run dev
```
