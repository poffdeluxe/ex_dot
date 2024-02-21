use gv::parser::DotParser;
use gv::GraphBuilder;
use layout::backends::svg::SVGWriter;
use layout::gv;
use rustler::types::OwnedBinary;
use rustler::Binary;
use rustler::Env;
use rustler::Error;
use rustler::Term;
use std::io::Write;

mod atoms {
    rustler::atoms! {
        ok,
        dot_error
    }
}

fn load(_env: Env, _: Term) -> bool {
    true
}

#[rustler::nif]
fn nif_dot_to_svg<'a>(env: Env<'a>, input: Binary) -> Result<Term<'a>, Error> {
    let contents: &[u8] = input.as_slice();
    let contents_str = std::str::from_utf8(contents).expect("Invalid UTF-8");
    let mut parser = DotParser::new(contents_str);

    match parser.process() {
        Result::Err(_err) => Ok(atoms::dot_error().to_term(env)),
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

            let mut binary = OwnedBinary::new(content.len()).unwrap();
            let _ = binary.as_mut_slice().write_all(content.as_bytes());
            Ok(binary.release(env).to_term(env))
        }
    }
}

rustler::init!("Elixir.Dot.Native", [nif_dot_to_svg], load = load);
