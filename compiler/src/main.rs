mod compiler;
mod parse;
mod util;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        panic!("i need the file and output dir");
    }

    let file = std::fs::read_to_string(args[1].clone()).unwrap();
    let config = parse::parse_config(
        std::fs::read_to_string(
            "/home/tritranduc/dev/code/vietnamese-program-language/config/keyword.config"
                .to_string(),
        )
        .unwrap(),
    );
    let ast = parse::parse_string_to_ast(file.clone(), &config);
    let context = inkwell::context::Context::create();
    let mut code_compiler = compiler::Compiler::new(
        &context,
        context.create_module("app"),
        context.create_builder(),
        &ast,
        &config,
    );
    code_compiler.parse_ast_to_module();
    code_compiler
        .build_to_file(
            &args[2],
            if args.len() >= 4 {
                Some(&args[3])
            } else {
                None
            },
        )
        .unwrap();
}
