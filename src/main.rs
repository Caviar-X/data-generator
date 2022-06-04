use data_generator::parser::Parser;
fn main() {
    let p = Parser::new_with_context("{int from 0 to 1e5}".into());
    println!("{:?}",p.next_token());
}
