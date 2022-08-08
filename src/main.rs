mod search_engines;

use crate::search_engines::UrlRequest;
use crate::search_engines::UrlRequestSite;

fn main() {
    let amount: u32 = 30;
    let query_vec = vec!["dog"];
    let site_vec = vec![
        UrlRequestSite::General_ddg,
        //UrlRequestSite::General_searx_be,
        //UrlRequestSite::Images_searx_be,
    ];
    let force_amount = false;

    let mut request_vec: Vec<UrlRequest> = vec![];

    for query in query_vec.iter() {
        for site in site_vec.iter() {
            request_vec.push(UrlRequest {
                query,
                amount,
                force_amount,
                site: *site,
            });
        }
    }

    let hosts: String = search_engines::get_hosts(&request_vec).unwrap_or_else(|error| {
        panic!("Error! {}", error);
    });

    println!(
        "Hosts: \n{}\n\nAmount: {}",
        hosts,
        hosts.split("\n").count()
    );

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
