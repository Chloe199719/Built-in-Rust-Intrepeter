use interpeter::repl;

fn main() {
    println!("Feel Free to type in commands");
    repl::start(&mut std::io::stdin().lock(), &mut std::io::stdout());
}
