mod search_engines;

use crate::search_engines::ddg;
use crate::search_engines::searx_be;

fn main() {
    /*println!(
        "{}",
        search_engines::get_html("https://searx.be/search?q=a&categories=images&language=en-US")
    );
    return;*/
    //let vec_ddg = ddg::general("dog", 5u32);
    // print_vector(&vec_ddg);

    //let vec_searxbe = searx_be::general("dog", 5u32);
    //print_vector(&vec_searxbe);

    let vec_img_searxbe = searx_be::image("dog", 300u32, true).unwrap_or_else(|error_msg| {
        println!("ERROR: {}", error_msg);
    });
    print_vector_tuple(&vec_img_searxbe);

    println!("Program finished.");
}

fn print_vector(vec: &Vec<String>) {
    let iter = vec.iter();
    for url in iter {
        println!("\t{}", url);
    }
}

fn print_vector_tuple(vec: &Vec<(String, String)>) {
    let iter = vec.iter();
    for tuple in iter {
        println!("\tIMG: {}\n\tURL: {}\n", tuple.0, tuple.1);
    }
}
