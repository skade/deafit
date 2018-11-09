fn main() {
    let get_page = get_page()
                   .and_then(|response| {
                       println!("{}", response.body);
                   });

    tokio::run(get_page);
}

// exemplarisch: dieses HTTP-Interface gibt es so nicht
async fn get_page() -> HttpResponse {
    let http_request = http::get("https://deafit.org");

    let response = await http_request;

    response
}