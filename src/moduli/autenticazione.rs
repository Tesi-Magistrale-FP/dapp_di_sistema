use std::process::Command;
use std::process::Stdio;
use sha256::digest;
use identity_iota::account::Account;
use identity_iota::iota_core::IotaDID;
use identity_iota::account::IdentitySetup;
use identity_iota::account::Result;
use identity_iota::client::ExplorerUrl;

// Registrazione dell'utente nell'ecosistema
pub async fn registrazione(mut user_did_s: String, password: String) -> Result<bool>								
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di registrazione
    
    println!("\n---------------------------------------");
    println!("\nREGISTRAZIONE NELL'ECOSISTEMA\n");

    if user_did_s.len() == 0                                                                                     	// Se l'identità deve essere creata
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
    else 																											// Identità passata come parametro
	{
        println!("- DID dell'utente {}", user_did_s);
    }

    // Hash SHA256 della password
    let hash_pwd: String = digest(password.clone());
    println!("- Password: {} | Hash password: {}", password.clone(), hash_pwd);

    // Esegue il comando della wasp-cli per eseguire la registrazione dell'utente sullo ISC Autenticazione
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "autenticazione", "registrazione", "String", "did", "String", &user_did_s, "String", "password", "String", &hash_pwd, "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della registrazione
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                            // Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore registrazione -> {output_s}");
        esito = false;
    }
    else                                                                                                          	// Chiamata alla funzione avvenuta con successo
    {
        // Recupera l'indirizzo della transazione usata per chiamare la funzione
        let output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
        let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));
        
        // Recupera l'evento generato dalla funzione per comprendere l'esito della registrazione
        output = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "request", &ind_trans])
            .output()
            .unwrap();

        output_s = String::from_utf8(output.stdout).unwrap();

        if output_s.contains("autenticazione.registrazioneSuccesso")                                            	// Registrazione avvenuta con successo
        {
            println!("- Registrazione eseguita!");
        }
        else                                                                                            			// Registrazione non avvenuta
        {
            println!("- Registrazione non eseguita!");
            esito = false;
        }
    }

    Ok(esito)																										// Restituisce l'esito dell'operazione
}

// Login dell'utente nell'ecosistema
pub async fn login(user_did_s: String, password: String) -> Result<bool>											
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di login
    
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

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito del login
    let output_s: String = String::from_utf8(output_view.stdout).unwrap();

    if ! output_s.contains("true") && ! output_s.contains("false") {
        println!("- Errore login -> {output_s}");
        esito = false;
    }
    else if output_s.contains("false") {
        println!("- Login non eseguito!");
        esito = false;
    }
    else if output_s.contains("true") {
        println!("- Login eseguito!");
    }

    Ok(esito)																										// Restituisce l'esito dell'operazione
}

// Eliminazione dell'utente dall'ecosistema
pub async fn eliminazione(user_did_s: String, password: String) -> Result<bool>										
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di eliminazione
    
    println!("\n---------------------------------------");
    println!("\nELIMINAZIONE DALL'ECOSISTEMA\n");

    println!("- DID dell'utente {}", user_did_s);

    // Hash SHA256 della password
    let hash_pwd: String = digest(password.clone());
    println!("- Password: {} | Hash password: {}", password.clone(), hash_pwd);

    // Esegue il comando della wasp-cli per eseguire l'eliminazione dell'utente sullo ISC Autenticazione
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "autenticazione", "eliminazione", "String", "did", "String", &user_did_s, "String", "password", "String", &hash_pwd, "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito dell'eliminazione
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                            // Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore eliminazione -> {output_s}");
        esito = false;
    }
    else                                                                                                          	// Chiamata alla funzione avvenuta con successo
    {
        // Recupera l'indirizzo della transazione usata per chiamare la funzione
        let output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
        let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));
        
        // Recupera l'evento generato dalla funzione per comprendere l'esito dell'eliminazione
        output = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "request", &ind_trans])
            .output()
            .unwrap();

        output_s = String::from_utf8(output.stdout).unwrap();

        if output_s.contains("autenticazione.eliminazioneSuccesso")                                            		// Eliminazione avvenuta con successo
        {
            println!("- Eliminazione eseguita!");
        }
        else                                                                                            			// Eliminazione non avvenuta
        {
            println!("- Eliminazione non eseguita!");
            esito = false;
        }
    }

    Ok(esito)																										// Restituisce l'esito dell'operazione
}