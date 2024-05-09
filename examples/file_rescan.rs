use async_vt3::VtClient;

#[tokio::main]
async fn main() {
    let api_key = match std::env::args().nth(1).ok_or("Please provide the api key!") {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };
    let file = "44d88612fea8a8f36de82e1278abb02f";

    let res = VtClient::new(&api_key).file_rescan(file).await;
    match res {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e),
    }
}
