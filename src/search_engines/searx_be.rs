use std::collections::HashSet;

pub fn general(
    query: &str,
    amount: u32,
    force_amount: bool,
    url_set: &mut HashSet<String>,
) -> Result<(), String> {
    if amount == 0 {
        return Ok(());
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

        let html = super::get_html(url.as_str())?;
        //Get a list of all occurrences in html
        let occurrences: Vec<_> = html
            .match_indices("<h4 class=\"result_header\" id=\"result-")
            .collect();

        let for_count: usize = std::cmp::min(occurrences.len(), (amount - urls_found) as usize);
        if for_count < 1 {
            if force_amount {
                return Err(format!("Ran out of pages at {} urls found", urls_found));
            }
            return Ok(());
        }

        for i in 0..for_count {
            let string_search_offset = occurrences[i].0 + 42;
            let url_start_index =
                html[string_search_offset..].find("href").unwrap() + string_search_offset + 6;
            let url_end_index = html[url_start_index..].find('\"').unwrap() + url_start_index;
            let url: String = html[url_start_index..url_end_index].to_string();
            urls_found += url_set.insert(url) as u32;
        }
    }

    Ok(())
}

pub fn images(
    query: &str,
    amount: u32,
    force_amount: bool,
    url_set: &mut HashSet<String>,
) -> Result<(), String> {
    //Result<(image-url, url), error msg>

    println!("-----searx.be image");

    let url = format!(
        "https://searx.be/search?q={}&categories=images&language=en-US",
        query
    );
    println!("Query URL: {}", url);

    let html = super::get_html(url.as_str())?;
    //Get a list of all occurrences in html
    let occurrences: Vec<_> = html
        .match_indices("<div class=\"col-md-6\"><a href=\"")
        .collect();

    if force_amount && (occurrences.len() / 2) < (amount as usize) {
        return Err(format!("Ran out of pages at {}", occurrences.len() / 2));
    }

    let for_count: usize = std::cmp::min(occurrences.len() / 2, amount as usize);
    for i in 0..for_count {
        let url_start_index = occurrences[i * 2].0 + 31;
        let url_end_index = html[url_start_index..].find('\"').unwrap() + url_start_index;
        let img_url: String = html[url_start_index..url_end_index].to_string();

        let url_start_index = occurrences[i * 2 + 1].0 + 31;
        let url_end_index = html[url_start_index..].find('\"').unwrap() + url_start_index;
        let website_url: String = html[url_start_index..url_end_index].to_string();

        url_set.insert(img_url);
        url_set.insert(website_url);
    }
    Ok(())
}
