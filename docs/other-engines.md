# Other JS engines available for Rust

> Written at 2026-11-14, using a `js engine language:Rust` search on GitHub.

## Bindings

- ~~[`rusty_v8`](https://github.com/denoland/rusty_v8)~~ too heavy
- ~~[`mozjs`](https://github.com/servo/mozjs)~~ too heavy
- ~~`bun`~~? no sure where their rusty jsc is published
- ~~[`rquickjs`](https://github.com/DelSkayn/rquickjs)~~ missing [native class extend](https://github.com/DelSkayn/rquickjs/pull/587)
  - also longbridge jit fork ~~[`quickjs-jit`](https://github.com/longbridge/quickjs-jit)~~, vibed & no feature improvement

## Impls

- ~~[`boa`](https://github.com/boa-dev/boa)~~ progress very slow, seem to be a college project (quality uncertainty)
- ~~[`nova`](https://github.com/trynova/nova)~~ native class completely missing, the andromeda runtime is using a global store, which is infavorable
- ~~[`brimstone`](https://github.com/Hans-Halverson/brimstone)~~ <!-- un investigated yet -->
- ~~[`yarashark`](https://github.com/Sharktheone/yavashark)~~ only 80% conformance
- ~~[`jsse`](https://github.com/pmatos/jsse)~~ purely vibed
- [`perry`](https://github.com/PerryTS/perry) a AOT compiler using LLVM.
- ~~[`katsu`](https://github.com/tamnd/katsu)~~ seem to be vibed
- ~~[`jsmoteur`](https://github.com/rocketlyz/jsmoteur)~~
- ~~[`v12`](https://github.com/DevYatsu/v12)~~ vibed

## Unmaintained

- ~~[`starlight`](https://github.com/Starlight-JS/starlight)~~

## Runtimes

These are not engine but we may need reference when we will make our own runtime

- ~~[`spiderfile`](https://github.com/redfire75369/spiderfire)~~
