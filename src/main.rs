use dotenv::dotenv;
use std::env;
use std::error::Error;
use rusoto_core::Region;
use rusoto_ecr::{EcrClient, Ecr, GetAuthorizationTokenRequest};

use kube::{
    api::Api,
    client::APIClient,
    config,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "info,kube=trace");

    let config = config::load_kube_config().await?;
    let client = APIClient::new(config);

    // Manage pods
    let pods = Api::v1Pod(client).within("fritzandandre");

    let fna = pods.get("fritzandandre-php-0").await?;

    println!("Got blog pod with containers: {:?}", fna.spec.containers);

    let aws_account_id = env::var("AWS_ACCOUNT_ID").unwrap();
    let aws_secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").unwrap();
    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID").unwrap();
    let aws_default_region = env::var("AWS_DEFAULT_REGION").unwrap();
    let email = env::var("EMAIL").unwrap();
    let namespaces = env::var("NAMESPACES").unwrap();

    let authorization_token = get_aws_authorization_token(Region::UsWest2, &aws_account_id)?;

    println!(
        "AWS_ACCOUNT_ID: {}\nAWS_SECRET_ACCESS_KEY: {}\nAWS_ACCESS_KEY_ID: {}\nAWS_DEFAULT_REGION: {}\nEMAIL: {}\nNAMESPACES: {}\n",
        aws_account_id,
        aws_secret_access_key,
        aws_access_key_id,
        aws_default_region,
        email,
        namespaces
    );

    println!("AUTHORIZATION TOKEN: {}\n", authorization_token);

    Ok(())
}

fn get_aws_authorization_token(region: Region, aws_account_id: &String) -> Result<String, Box<dyn Error>> {
    let client = EcrClient::new(region);
    let token_request = GetAuthorizationTokenRequest {
        registry_ids: Some(vec![aws_account_id.to_string()])
    };

    let auth_token_response = client
        .get_authorization_token(token_request)
        .sync()?;

    let auth_data = auth_token_response
        .authorization_data
        .ok_or("Unable to get authorization data")?;

    let token = auth_data.first().ok_or("Response didn't contain a token")?;

    Ok(token.authorization_token.clone().unwrap())
}
