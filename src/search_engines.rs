pub mod ddg;
pub mod searx_be;

pub fn get_html(url: &str) -> String {
    //Get Response instance from url
    ureq::get(url)
        .call()
        .unwrap_or_else(|error| {
            panic!(
                "HTML request failed at {}\n(Probably ran out of pages)\n error: {:?}",
                url, error
            );
        })
        //Convert Response into string
        .into_string()
        .unwrap_or_else(|error| {
            panic!("error response->string {}", error);
        })
}
