# Sample executable to render DOT to SVG

Needs this in your `Cargo.toml`

```toml
[dependencies]
layout-rs = "0.1.1"
```

This renders a simple SVG:

```rust
fn main() {
    let contents = "strict graph {\n    365677982[label=\"c2\"]\n    589337185[label=\"c\"]\n    1032268661[label=\"b2\"]\n    1499841983[label=\"g\"]\n    1589197455[label=\"a\"]\n    2122408781[label=\"d\"]\n    2689846215[label=\"a2\"]\n    2702065653[label=\"b\"]\n    2973971830[label=\"unauthorized_to_see_this_one\"]\n    3033988996[label=\"h\"]\n    3090001605[label=\"e\"]\n    3944162213[label=\"f\"]\n    589337185 -- 2122408781 [weight=1]\n    1032268661 -- 365677982 [weight=1]\n    1032268661 -- 2973971830 [weight=1]\n    1032268661 -- 3944162213 [weight=1]\n    1499841983 -- 3033988996 [weight=1]\n    1589197455 -- 2702065653 [weight=1]\n    2122408781 -- 3090001605 [weight=1]\n    2689846215 -- 1032268661 [weight=1]\n    2702065653 -- 365677982 [weight=1]\n    2702065653 -- 589337185 [weight=1]\n    3090001605 -- 3944162213 [weight=1]\n    3944162213 -- 1499841983 [weight=1]\n}\n";
    let mut parser = DotParser::new(&contents);
    match parser.process() {
        Result::Err(err) => {
            println!("Error: {:?}", err);
        }
        Result::Ok(g) => {
            let mut gb = GraphBuilder::new();
            gb.visit_graph(&g);
            let mut graph = gb.get();
            let mut svg = SVGWriter::new();
            let debug_mode = false;
            let disable_opt = false; // graph optimizations
            let disable_layout = false; // node layout pass
            graph.do_it(debug_mode, disable_opt, disable_layout, &mut svg);
            let content = svg.finalize();
            println!("{}", content);
        }
    }
}
```