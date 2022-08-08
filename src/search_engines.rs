pub mod ddg;
pub mod searx_be;

use std::collections::HashSet;
use std::{thread, time};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum UrlRequestSite {
    General_ddg,
    General_searx_be,
    Images_searx_be,
}

pub struct UrlRequest<'a> {
    pub query: &'a str,
    pub amount: u32,
    pub force_amount: bool,
    pub site: UrlRequestSite,
}

pub fn create_request<'a>(
    query: &'a str,
    amount: u32,
    force_amount: bool,
    site: UrlRequestSite,
) -> UrlRequest {
    UrlRequest {
        query,
        amount,
        force_amount,
        site,
    }
}

pub fn get_hosts(vec: &Vec<UrlRequest>) -> Result<String, String> {
    let mut url_set = HashSet::<String>::new();

    let sleep = time::Duration::from_millis(2000);

    for request in vec.iter() {
        append_set_from_request(request, &mut url_set)?;
        thread::sleep(sleep);
    }

    Ok(get_hosts_from_set(&url_set))
}

fn get_hosts_from_set(url_set: &HashSet<String>) -> String {
    let mut hosts: String = String::from("# Hosts entries generated by LFSQ");
    url_set.iter().for_each(|url| {
        let host = get_host_from_url(&url);
        hosts.push_str(host.as_str());
    });
    hosts
}

pub fn append_set_from_request(
    request: &UrlRequest,
    url_set: &mut HashSet<String>,
) -> Result<(), String> {
    let query = request.query;
    let amount = request.amount;
    let force = request.force_amount;

    match &request.site {
        UrlRequestSite::General_ddg => ddg::general(query, amount, force, url_set),
        UrlRequestSite::General_searx_be => searx_be::general(query, amount, force, url_set),
        UrlRequestSite::Images_searx_be => searx_be::images(query, amount, force, url_set),
    }
}

fn get_host_from_url(url: &String) -> String {
    let mut host = String::from("\n0.0.0.0 ");

    let url = url.strip_prefix("https://").unwrap_or(url);
    let url = url.strip_prefix("http://").unwrap_or(url);

    let slash_index: usize = url.find("/").unwrap_or(url.len());
    let url = &url[..slash_index];

    host.push_str(url);
    host
}

pub fn get_html(url: &str) -> Result<String, String> {
    //Get Response instance from url

    let call = ureq::get(url).call();
    if let Err(error) = call {
        return Err(format!(
            "HTML request failed at {}\n(Probably ran out of pages)\n error: {:?}",
            url, error
        ));
    }

    //Convert Response into string
    Ok(call.unwrap().into_string().unwrap_or_else(|error| {
        panic!("error response->string {}", error);
    }))
}
