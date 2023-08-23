# cloudflare-r2-static-pages-worker-rs

A Cloudflare Worker that host static pages with R2, building by Rust.

There are some rules that I customized:

1. Define the `Content-Type` of Apple Universal Links config `apple-app-site-association` to `application/json`.
2. Define the `index.html` file as the default pages entrance in R2.

## Deploy

[![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/AyakuraYuki/cloudflare-r2-static-pages-worker-rs/tree/main)

## Wrangler

Wrangler is used to develop, deploy, and configure your Worker via CLI.

Further documentation for Wrangler can be found [here](https://developers.cloudflare.com/workers/tooling/wrangler).

## Usage

With `wrangler`, you can build, test, and deploy your Worker with the following commands:

```sh
# run your Worker in an ideal development workflow (with a local server, file watcher & more)
$ npm run dev-remote

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
$ npm run deploy
```

Read the latest `worker` crate documentation here: https://docs.rs/worker

## WebAssembly

`cloudflare-r2-static-pages-worker-rs` (the Rust SDK for Cloudflare Workers used in this worker) is meant to be executed as compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple.

Read more about this on the [`cloudflare-r2-static-pages-worker-rs`](https://github.com/AyakuraYuki/cloudflare-r2-static-pages-worker-rs) project README.

## Issues

If you have any problems with the `worker` crate, please open an issue on the upstream project issue tracker on the [`cloudflare-r2-static-pages-worker-rs` repository](https://github.com/AyakuraYuki/cloudflare-r2-static-pages-worker-rs).
