use vt3::VtClient;

#[tokio::main]
async fn main() {
    let api_key = match std::env::args().nth(1).ok_or("Please provide the api key!") {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };
    let url = "https://www.example.com";

    let vt_client = VtClient::new(&api_key);
    let resource_id = match vt_client.url_scan(url).await {
        Ok(report) => report,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1)
        }
    };

    if resource_id.data.id.is_empty() {
        println!("No resource id found")
    } else {
        match vt_client.url_rescan(&resource_id.data.id).await {
            Ok(report) => println!("{:#?}", report),
            Err(e) => println!("Error: {}", e),
        }
    }
}
