use std::process::Command;
use std::process::Stdio;
use sha256::digest;
use identity_iota::account::Account;
use identity_iota::iota_core::IotaDID;
use identity_iota::account::IdentitySetup;
use identity_iota::account::Result;
use identity_iota::client::ExplorerUrl;

pub async fn registrazione(mut user_did_s: String, password: String) -> Result<bool>									// Registrazione dell'utente nell'ecosistema
{
    let mut esito = true;                                                                                 	// Esito dell'operazione di registrazione
    
    println!("\n---------------------------------------");
    println!("\nREGISTRAZIONE NELL'ECOSISTEMA\n");

    if user_did_s.len() == 0                                                                                            // Se l'identità deve essere creata
    {
        // Crea l'identità decentralizzata
        let user: Account = Account::builder().create_identity(IdentitySetup::default()).await?;

        // Restituisce il valore del DID associato all'identità appena creata
        let user_did: &IotaDID = user.did();
        user_did_s = user_did.to_string();

        // Mostra lo stato locale del Documento DID
        println!("- Documento associato alla DID {} \n{:#?}", user_did, user.document());

        // Mostra l'URL dell'Identity Resolver Explorer che consente di vedere il Documento DID online e tutta la sua storia passata
        let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
        println!("- Documento DID disponibile al seguente link -> {}", explorer.resolver_url(user_did)?);
    }
    else {
        println!("- DID dell'utente {}", user_did_s);
    }

    // Hash SHA256 della password
    let hash_pwd: String = digest(password.clone());
    println!("- Password: {} | Hash password: {}", password.clone(), hash_pwd);

    // Esegue il comando della wasp-cli per eseguire la registrazione dell'utente sullo ISC Autenticazione
    let output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "autenticazione", "registrazione", "String", "did", "String", &user_did_s, "String", "password", "String", &hash_pwd, "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della registrazione
    let output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...") {
        println!("- Errore registrazione -> {output_s}");
        esito = false;
    }
    else {
        println!("- Registrazione eseguita!");
    }

    Ok(esito)																								// Restituisce l'esito dell'operazione
}

pub async fn login(user_did_s: String, password: String) -> Result<bool>											// Login dell'utente nell'ecosistema
{
    let mut esito = true;                                                                                 	// Esito dell'operazione di login
    
    println!("\n---------------------------------------");
    println!("\nLOGIN NELL'ECOSISTEMA\n");

    println!("- DID dell'utente {}", user_did_s);

    // Hash SHA256 della password
    let hash_pwd: String = digest(password.clone());
    println!("- Password: {} | Hash password: {}", password.clone(), hash_pwd);

    // Esegue il comando della wasp-cli per eseguire il login dell'utente sullo ISC Autenticazione
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "autenticazione", "login", "String", "did", "String", &user_did_s, "String", "password", "String", &hash_pwd, "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "esitoL", "bool"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della registrazione
    let output_s: String = String::from_utf8(output_view.stdout).unwrap();

    if ! output_s.contains("true") && ! output_s.contains("false") {
        println!("- Errore login -> {output_s}");
        esito = false;
    }
    else if output_s.contains("false") {
        println!("- Login non eseguito!");
    }
    else if output_s.contains("true") {
        println!("- Login eseguito!");
    }

    Ok(esito)																								// Restituisce l'esito dell'operazione
}

pub async fn eliminazione(user_did_s: String, password: String) -> Result<bool>												// Eliminazione dell'utente dall'ecosistema
{
    let mut esito = true;                                                                                 	// Esito dell'operazione di eliminazione
    
    println!("\n---------------------------------------");
    println!("\nELIMINAZIONE DALL'ECOSISTEMA\n");

    println!("- DID dell'utente {}", user_did_s);

    // Hash SHA256 della password
    let hash_pwd: String = digest(password.clone());
    println!("- Password: {} | Hash password: {}", password.clone(), hash_pwd);

    // Esegue il comando della wasp-cli per eseguire l'eliminazione dell'utente sullo ISC Autenticazione
    let output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "autenticazione", "eliminazione", "String", "did", "String", &user_did_s, "String", "password", "String", &hash_pwd, "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito dell'eliminazione
    let output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...") {
        println!("- Errore eliminazione -> {output_s}");
        esito = false;
    }
    else {
        println!("- Eliminazione eseguita!");
    }

    Ok(esito)																								// Restituisce l'esito dell'operazione
}