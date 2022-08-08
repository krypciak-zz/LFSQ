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
    let pages: u32 = (amount - 1) / 30 + 1;
    println!(
        "-----duckduckgo.com\nQuerying {} page{}...",
        pages,
        if pages == 1 { "" } else { "s" }
    );
    let mut urls_found: u32 = 0;

    for current_page in 0..pages {
        //format html duckduckgo search query
        let url = format!(
            "https://html.duckduckgo.com/html/?q={}&s={}",
            query,
            current_page * 30
        );
        println!("Query URL: {}", url);
        let html = super::get_html(url.as_str());
        if force_amount {
            if let Err(x) = html {
                return Err(x);
            }
        }
        let html = html.unwrap();
        //println!("HTML: {}", html);
        let mut iterator = html.split("\n");

        loop {
            let line = iterator.next();
            let line: &str = match line {
                None => {
                    break;
                }
                Some(x) => x,
            };

            if line.starts_with("                  <a class=\"result__url\" href=\"") {
                let link: String = String::from(iterator.next().unwrap().trim_start());
                url_set.insert(link);
                urls_found += 1;
                if urls_found >= amount {
                    break;
                }
            } else {
                continue;
            }
        }
    }
    Ok(())
}
