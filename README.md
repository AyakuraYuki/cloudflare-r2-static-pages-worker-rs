# cloudflare-r2-static-pages-worker-rs

A Cloudflare Worker that hosts static pages with R2, built by Rust.

There are some rules that I customized:

1. Define the `Content-Type` of Apple Universal Links config `apple-app-site-association` to `application/json`.
2. Define the `index.html` file as the default website entrance in R2.

## Deploy

I don't have a plan to make a deploy template.

To deploy this worker, consider cloning this repository and modifying the `wrangler.toml`, edit the value of `name` and `r2_buckets` to whatever you want, and if you want to change the entrance of the page, edit the value of `vars.ENTRANCE`.

After that, run `wrangler deploy` and follow the Wrangler Deployment Instructions to deploy this worker into your Cloudflare account.

## Configurations

### R2 Bucket

I assign an array with a single element binding as `R2` to the `r2_buckets` key, it is used by my worker with `env.bucket("R2")`.

So please don't change the `binding` value in `wrangler.toml`, also in Cloudflare Dashboard. Just keep it as `R2`.

What you can edit is the value of binding `R2`, change it to your R2 bucket's name.

### Variables

I assign a variable named `ENTRANCE` to define which file will become the entrance of the page. You can change it to your website entrance file name.

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
