# Nyko JS

WIP: A JavaScript engine.

## Why a new JS engine?

In many of my projects, I need JS engine to make the app configurable and extensible.
Though there are many other JS engine impls or bindings for Rust, none of them completely fullfill my needs
(e.g. rusty_v8 too heavy; rquickjs missing too many features like native class extend).

Do's:

- Investigate [other engines](docs/other-engines.md) first before crafting ours. Try even vibed ones to see if they can work as temporary solution.
- Learn and even "copy" designs from mainstream engines.
- Developer experience is the first consideration.
- We want to cover use cases ranging from lightweight cli (e.g. neovim) or browser (e.g. servo), by providing a flexible feature set.
- Formal verification is in consideration when basic features are mature.

Don'ts:

- Do not do aggressive architecture expreiments like `nova`.
- Do not do optimizations from scratch. We want to first be usable then consider our own innovations.
- Do not vibe the code.
  1. Downstream projects like servo forbid AI generated code.
  2. AI can be used for explorations, but the main branch must be human understandable.
  3. Vibe prevents us from forming an intuition on how code can be optimized.
