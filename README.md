# Dot for Elixir

> **This library renders graphs in [DOT Language](https://graphviz.org/doc/info/lang.html) to SVG.**

It is a tiny ([Rustler](https://github.com/rusterlium/rustler)-based) wrapper around the [`layout-rs`](https://crates.io/crates/layout-rs). I initially used a local copy of `dot.exe` from the [Graphviz](https://graphviz.org/) package, and launched `dot` using [rambo](https://github.com/jayjun/rambo). However, I didn't want to spin up a console executable for each conversion, so looked for alternatives. Luckily, I found [`nadavrot/layout`](ttps://github.com/nadavrot/layout), which seems to do what I wanted. *I haven't checked whether that Rust crate can do *everything* that the full dot executable can do.*


## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `ex_dot` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    #{:ex_dot, github: "chgeuer/ex_dot"} # If you do this, you must set the environment variable EX_DOT_RUST_BUILD=true
    {:ex_dot, "~> 0.1.0"}
  ]
end
```
Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc) and published on [HexDocs](https://hexdocs.pm). Once published, the docs can be found at <https://hexdocs.pm/ex_dot>.

## Local build 

You must set the environment variable `EX_DOT_RUST_BUILD` to `true`, otherwise it will try to pull the [`rustler_precompiled`](https://github.com/philss/rustler_precompiled) bits.

## Using it in LiveBook

