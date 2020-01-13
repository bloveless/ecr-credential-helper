use std::env;
use std::error::Error;

use base64;

use dotenv::dotenv;
use serde_json::json;
use rusoto_core::Region;
use rusoto_ecr::{EcrClient, Ecr, GetAuthorizationTokenRequest};

use kube::{
    api::{RawApi, PostParams},
    client::APIClient,
    config,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "info,kube=trace");

    let aws_account_id = env::var("AWS_ACCOUNT_ID").unwrap();
    let aws_secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").unwrap();
    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID").unwrap();
    let aws_default_region = env::var("AWS_DEFAULT_REGION").unwrap();
    let email = env::var("EMAIL").unwrap();
    let namespaces = env::var("NAMESPACES").unwrap();

    println!(
        "AWS_ACCOUNT_ID: {}\nAWS_SECRET_ACCESS_KEY: {}\nAWS_ACCESS_KEY_ID: {}\nAWS_DEFAULT_REGION: {}\nEMAIL: {}\nNAMESPACES: {}\n",
        aws_account_id,
        aws_secret_access_key,
        aws_access_key_id,
        aws_default_region,
        email,
        namespaces
    );

    let config = config::load_kube_config().await?;
    let client = APIClient::new(config);

    let authorization_token = get_aws_authorization_token(Region::UsWest2, &aws_account_id)?;
    println!("AUTHORIZATION TOKEN: {}\n", authorization_token);

    let ecr_url = format!("https://{}.dkr.ecr.{}.amazonaws.com", aws_account_id, aws_default_region);
    let secret_data = json!({
        "auths": {
            ecr_url: {
                "username": "AWS",
                "password": authorization_token,
                "email": email,
                "auth": base64::encode(format!("AWS:{}", authorization_token).as_bytes()),
            },
        }
    });

    let secret_yaml = json!({
        "apiVersion": "v1",
        "type": "kubernetes.io/dockerconfigjson",
        "metadata": {
            "name": "us-west-2-ecr-registry",
            "namespace": "mockrift-com",
        },
        "data": {
            ".dockerconfigjson": base64::encode(secret_data.to_string().as_bytes()),
        },
    });

    println!("Secret yaml: {}", secret_yaml);

    let sec = RawApi::v1Secret().within("mockrift-com");
    let req = sec.create(&PostParams::default(), secret_yaml.to_string().as_bytes().to_vec())?;
    let res = client.request_text(req).await?;
    println!("Response: {}", res);
    //let _ = client.request_status::<Secret>(req).await.map(|res| println!("{:?}", res));


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
