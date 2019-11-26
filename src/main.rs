extern crate dotenv;

use dotenv::dotenv;
use std::env;
use rusoto_core::Region;
use rusoto_ecr::{
    EcrClient,
    Ecr,
    GetAuthorizationTokenRequest
};

use kube::{
    api::Api,
    client::APIClient,
    config,
};

fn main() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "info,kube=trace");

    let config = config::load_kube_config().expect("failed to load kubeconfig");
    let client = APIClient::new(config);

    // Manage pods
    let pods = Api::v1Pod(client).within("fritzandandre");

    let fna = pods.get("fritzandandre-php-0").expect("Failed to get fna pod");
    println!("Got blog pod with containers: {:?}", fna.spec.containers);

    let aws_account_id = env::var("AWS_ACCOUNT_ID").unwrap_or_default();
    let aws_secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_default();
    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID").unwrap_or_default();
    let aws_default_region = env::var("AWS_DEFAULT_REGION").unwrap_or_default();
    let email = env::var("EMAIL").unwrap_or_default();
    let namespaces = env::var("NAMESPACES").unwrap_or_default();

    let client = EcrClient::new(Region::UsWest2);
    let mut registry_ids: Vec<String> = Vec::new();
    registry_ids.push(aws_account_id.to_string());

    let token_request = GetAuthorizationTokenRequest { registry_ids: Some(registry_ids) };
    let token = match client.get_authorization_token(token_request).sync() {
        Ok(token) => token,
        Err(e) => panic!("e: {}", e),
    };

    let data = token.authorization_data.unwrap();
    let data = data.first().unwrap();

    let token = match data.authorization_token {
        Some(token) => token,
        None => 0,
    };

    println!("data {:?}", data.authorization_token);

    println!(
        "AWS_ACCOUNT_ID: {}\nAWS_SECRET_ACCESS_KEY: {}\nAWS_ACCESS_KEY_ID: {}\nAWS_DEFAULT_REGION: {}\nEMAIL: {}\nNAMESPACES: {}\n",
        aws_account_id,
        aws_secret_access_key,
        aws_access_key_id,
        aws_default_region,
        email,
        namespaces
    );

    println!("Hello, world!");
}
