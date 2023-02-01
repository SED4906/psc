mod eval;
mod parser;
fn main() {
    println!("{:#?}",crate::parser::program("%!
    %% Example 1
    
    newpath
    100 200 moveto
    200 250 lineto
    100 300 lineto
    2 setlinewidth
    stroke
    showpage
    "));
}
