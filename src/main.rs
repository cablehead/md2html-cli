use std::io::{self, Read};
use pulldown_cmark::{Parser, Options, html};

fn main() -> io::Result<()> {
    // Read from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Enable all options
    let options = Options::all();

    // Create parser
    let parser = Parser::new_ext(&input, options);

    // Write to stdout
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    print!("{}", html_output);

    Ok(())
}