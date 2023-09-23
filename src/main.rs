use anyhow::Result;

mod features;

#[tokio::main]
async fn main() -> Result<()> 
{
    let _registrazione: bool = features::autenticazione::registrazione(String::from("did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ"), String::from("Francy160498!*")).await?;
    let _login: bool = features::autenticazione::login(String::from("did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ"), String::from("Francy160498!*")).await?;
    let _eliminazione: bool = features::autenticazione::eliminazione(String::from("did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ"), String::from("Francy160498!*")).await?;

    Ok(())
}
