pub fn general(query: &str, amount: u32) -> Vec<String> {
    let mut url_list: Vec<String> = vec![];
    if amount == 0 {
        return url_list;
    }
    println!("-----searx.be general");

    let mut urls_found: u32 = 0;
    let mut current_page: u32 = 0;
    while urls_found < amount {
        current_page += 1;
        let url = format!(
            "https://searx.be/search?q={}&categories=general&pageno={}&language=en-US",
            query, current_page
        );
        println!("Query URL: {}", url);

        let html = super::get_html(url.as_str());
        //Get a list of all occurrences in html
        let occurrences: Vec<_> = html
            .match_indices("<h4 class=\"result_header\" id=\"result-")
            .collect();

        let for_count: usize = std::cmp::min(occurrences.len(), (amount - urls_found) as usize);
        for i in 0..for_count {
            let string_search_offset = occurrences[i].0 + 42;
            let url_start_index =
                html[string_search_offset..].find("href").unwrap() + string_search_offset + 6;
            let url_end_index = html[url_start_index..].find('\"').unwrap() + url_start_index;
            let url: String = html[url_start_index..url_end_index].to_string();
            url_list.push(url);
            urls_found += 1;
        }
    }

    url_list
}

pub fn image(query: &str, amount: u32, force_amount: bool) -> Result<Vec<(String, String)>, &str> {
    let mut url_list: Vec<(String, String)> = vec![];

    println!("-----searx.be image");

    let url = format!(
        "https://searx.be/search?q={}&categories=images&language=en-US",
        query
    );
    println!("Query URL: {}", url);

    let html = super::get_html(url.as_str());
    //Get a list of all occurrences in html
    let occurrences: Vec<_> = html
        .match_indices("<div class=\"col-md-6\"><a href=\"")
        .collect();

    if force_amount && (occurrences.len() / 2) < (amount as usize) {
        return Err("");
    }

    let for_count: usize = std::cmp::min(occurrences.len() / 2, amount as usize);
    for i in 0..for_count {
        let url_start_index = occurrences[i * 2].0 + 31;
        let url_end_index = html[url_start_index..].find('\"').unwrap() + url_start_index;
        let img_url: String = html[url_start_index..url_end_index].to_string();

        let url_start_index = occurrences[i * 2 + 1].0 + 31;
        let url_end_index = html[url_start_index..].find('\"').unwrap() + url_start_index;
        let website_url: String = html[url_start_index..url_end_index].to_string();

        url_list.push((img_url, website_url));
    }
    Ok(url_list)
}
