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

rustler::init!("Elixir.RustDotToSvg", [nif_dot_to_svg], load = load);

// fn main() {
//     let contents = "strict graph {\n    365677982[label=\"c2\"]\n    589337185[label=\"c\"]\n    1032268661[label=\"b2\"]\n    1499841983[label=\"g\"]\n    1589197455[label=\"a\"]\n    2122408781[label=\"d\"]\n    2689846215[label=\"a2\"]\n    2702065653[label=\"b\"]\n    2973971830[label=\"unauthorized_to_see_this_one\"]\n    3033988996[label=\"h\"]\n    3090001605[label=\"e\"]\n    3944162213[label=\"f\"]\n    589337185 -- 2122408781 [weight=1]\n    1032268661 -- 365677982 [weight=1]\n    1032268661 -- 2973971830 [weight=1]\n    1032268661 -- 3944162213 [weight=1]\n    1499841983 -- 3033988996 [weight=1]\n    1589197455 -- 2702065653 [weight=1]\n    2122408781 -- 3090001605 [weight=1]\n    2689846215 -- 1032268661 [weight=1]\n    2702065653 -- 365677982 [weight=1]\n    2702065653 -- 589337185 [weight=1]\n    3090001605 -- 3944162213 [weight=1]\n    3944162213 -- 1499841983 [weight=1]\n}\n";
//     let mut parser = DotParser::new(&contents);
//     match parser.process() {
//         Result::Err(err) => {
//             println!("Error: {:?}", err);
//         }
//         Result::Ok(g) => {
//             let mut gb = GraphBuilder::new();
//             gb.visit_graph(&g);
//             let mut graph = gb.get();
//             let mut svg = SVGWriter::new();
//             let debug_mode = false;
//             let disable_opt = false; // graph optimizations
//             let disable_layout = false; // node layout pass
//             graph.do_it(debug_mode, disable_opt, disable_layout, &mut svg);
//             let content = svg.finalize();
//             println!("{}", content);
//         }
//     }
// }
