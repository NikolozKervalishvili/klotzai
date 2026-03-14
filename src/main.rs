use klotzai::lexer::Lexer;

fn main() {
    let source = std::fs::read_to_string("input.txt").expect("file not found");
    let lexer = Lexer::new(&source);
    for token in lexer {
        dbg!(token);
    }
}
