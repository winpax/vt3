use vt3::VtClient;

fn main() {
    let api_key = match std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| "Please provide the api key!")
    {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };
    let domain = "google.com";

    let res = VtClient::new(&api_key)
        .domain_info(domain);
    match res {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
