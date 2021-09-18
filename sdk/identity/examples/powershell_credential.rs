use azure_identity::token_credentials::*;
use json::{parse, JsonValue};
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credential = AzurePowerShellCredential::new(None);
    let res = credential
        .get_token("https://graph.microsoft.com/.default")
        .await?;

    println!("Azure PowerShell response == {:?}", res);

    // Let's look at some information from the Microsoft Graph as an example. The following request will return the
    // authenticated user's profile, and we'll use that information to display the username of the Azure PowerShell
    // session (user principal name).

    let resp = reqwest::Client::new()
        .get(Url::parse("https://graph.microsoft.com/v1.0/me")?)
        .header("Authorization", format!("Bearer {}", res.token.secret()))
        .send()
        .await?
        .text()
        .await?;

    let response_data = parse(&resp).unwrap();

    let user_principal_name = if let JsonValue::Object(ref obj) = response_data {
        obj.get("userPrincipalName")
            .expect("Expected to receive a 'userPrincipalName' field.")
            .as_str()
            .unwrap()
    } else {
        panic!("Unexpected JSON response from Microsoft Graph: expected an object.");
    };

    println!("You are logged in as: {}", user_principal_name);

    Ok(())
}
