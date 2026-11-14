# Other JS engines available for Rust

## Bindings

- [`rusty_v8`](https://github.com/denoland/rusty_v8) too heavy
- [`mozjs`](https://github.com/servo/mozjs) too heavy
- `bun`? no sure where their rusty jsc is published
- [`rquickjs`](https://github.com/DelSkayn/rquickjs) missing [native class extend](https://github.com/DelSkayn/rquickjs/pull/587)
  - also longbridge jit fork [`quickjs-jit`](https://github.com/longbridge/quickjs-jit), vibed & no feature improvement

## Impls

- [`boa`](https://github.com/boa-dev/boa) progress very slow, seem to be a college project (quality uncertainty)
- [`nova`](https://github.com/trynova/nova) native class completely missing, the andromeda runtime is using a global store, which is infavorable
- [`brimstone`](https://github.com/Hans-Halverson/brimstone) <!-- un investigated yet -->
