use std::process::Command;
use std::process::Stdio;
use sha256::digest;
use identity_iota::account::Account;
use identity_iota::iota_core::IotaDID;
use identity_iota::account::IdentitySetup;
use identity_iota::account::Result;
use identity_iota::client::ExplorerUrl;
use identity_iota::account::MethodContent;
use identity_iota::client::CredentialValidationOptions;
use identity_iota::client::CredentialValidator;
use identity_iota::client::FailFast;
use identity_iota::core::FromJson;
use identity_iota::core::ToJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::Subject;
use identity_iota::crypto::ProofOptions;
use identity_iota::did::DID;
use identity_iota::account_storage::Stronghold;
use std::path::PathBuf;

static mut ISSUER: Option<Account> = None;

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

    Ok(esito)
}

// Eliminazione dell'utente dall'ecosistema
pub fn eliminazione(user_did_s: String, password: String) -> bool									
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

    return esito;
}

// Registrazione delle credenziali verificabili di un utente
pub async fn registra_utente_vc(id_app: String, did_utente: String, mut vc_utente: String, dati_utente: serde_json::Value) -> Result<bool>										
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di registrazione
    
    println!("\n---------------------------------------");
    println!("\nREGISTRA LE CREDENZIALI VERIFICABILI\n");

    let rif_utente: String = format!("{id_app}|{did_utente}");                                                      // Crea il riferimento dato da id_app|did_utente che sarà usato per accedere alle credenziali verificabili di un utente per un'applicazione

    println!("- Riferimento utente {}", rif_utente);
    
    if vc_utente.len() == 0                                                                                         // Credenziali verificabili da creare con i dati_utente
    {
        println!("\n- Creazione credenziali verificabili dai dati utente {}", dati_utente);

        // Crea una credenziale, da associare al Subject, che contiene delle informazioni utili per la piattaforma BlocksShare
        let subject: Subject = Subject::from_json_value(dati_utente)?;

        unsafe
        {
            // Ottiene delle credenziali a partire dal Subject e dall'Issuer
            let mut credenziali: Credential = CredentialBuilder::default()
                .id(Url::parse(format!("https://blocksshare.com/credentials/{did_utente}"))?)
                .issuer(Url::parse(ISSUER.as_mut().unwrap().did().as_str())?)
                .type_("BlocksShareCredential")
                .subject(subject)
                .build()?;
            
            // Firma le credenziali create con il metodo di verifica dell'Issuer
            ISSUER.as_mut().unwrap()
                .sign("#auth_vc", &mut credenziali, ProofOptions::default())
                .await?;

            // Prima di inviare le credenziali all'Holder, l'Issuer deve controllare che alcune proprietà delle credenziali rispettino le aspettative.
            // Questo controllo avviene tramite la seguente validazione.
            
            // Il controllo si concentra su:
            // - La verifica della firma delle credenziali usando il Documento DID dell'Issuer
            // - La correttezza della struttura semantica delle credenziali
            // - La validità della data di emissione (non deve essere una data futura)
            // - La validità della data di scadenza (non deve essere una data passata)
            CredentialValidator::validate(
                &credenziali,
                &ISSUER.as_mut().unwrap().document(),
                &CredentialValidationOptions::default(),
                FailFast::FirstError,
            )
            .unwrap();
            
            println!("- Credenziali validate con successo");

            // L'Issuer ora è sicure che le credenziali soddisfano le aspettative.
            // Le credenziali ora possono essere serializzate in formato JSON e trasmesse all'utente in maniera sicura.
            // NB: Le credenziali non vengono pubblicate sul Tangle di IOTA, ma inviate e salvate off-chain
            let credenziali_json: String = credenziali.to_json()?;                                                  // Credenziali verificabili in formato json
        
            println!("\n- Credenziali verificabili JSON {}", credenziali_json);

            vc_utente = digest(credenziali_json.clone());                                                   		// Hash SHA256 delle credenziali verificabili
        }
    }

    println!("\n- Hash redenziali verificabili {}", vc_utente);

    // Esegue il comando della wasp-cli per registrare le credenziali verificabili sullo ISC GestioneApplicazioni
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "gestioneapplicazioni", "registraUtenteVC", "String", "rifUtente", "String", &rif_utente, "String", "vcUtente", "String", &vc_utente, "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della registrazione
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                            // Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore registrazione credenziali verificabili -> {output_s}");
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

        if output_s.contains("gestioneapplicazioni.utenteRegistrato")                                               // Registrazione avvenuta con successo
        {
            println!("- Registrazione eseguita!");
        }
        else                                                                                            			// Registrazione non avvenuta
        {
            println!("- Registrazione non eseguita!");
            esito = false;
        }
    }

    Ok(esito)
}

// Login dell'utente nell'ecosistema
pub fn login(user_did_s: String, password: String) -> bool										
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

    return esito;
}

// Verifica la correttezza delle credenziali verificabili di un utente 
pub fn verifica_utente_vc(rif_utente: String, vc_utente: String) -> bool										
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di verifica
    
    println!("\n---------------------------------------");
    println!("\nVERIFICA CORRETTEZZA CREDENZIALI VERIFICABILI\n");

    println!("- Riferimento utente {} | - Credenziali verificabili {}", rif_utente, vc_utente);

    // Esegue il comando della wasp-cli per eseguire la verifica della credenziali verificabili sullo ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "verificaUtenteVC", "String", "rifUtente", "String", &rif_utente, "String", "vcUtente", "String", &vc_utente, "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "esitoC", "bool"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della verifica
    let output_s: String = String::from_utf8(output_view.stdout).unwrap();

    if ! output_s.contains("true") && ! output_s.contains("false") {
        println!("- Errore verifica credenziali verificabili -> {output_s}");
        esito = false;
    }
    else if output_s.contains("false") {
        println!("- Credenziali verificabili errate!");
        esito = false;
    }
    else if output_s.contains("true") {
        println!("- Credenziali verificabili corrette!");
    }

    return esito;
}

// Funzione per creare l'Issuer, ovvero l'entità che asserisce affermazioni su un soggetto
pub async fn crea_issuer() -> Result<()>
{
    println!("\n---------------------------------------");
    println!("\nCREAZIONE ISSUER PER RILASCIO DELLE CREDENZIALI VERIFICABILI\n");

    unsafe
    {
        // Stronghold è un file cifrato che gestisce le chiavi private. Viene implementato usando le migliori pratiche di sicurezza ed è raccomandato per gestire le chiavi private
        let stronghold_path: PathBuf = ".stronghold/issuer-strong.hodl".into();                        				// Path in cui verrà salvato il file Stronghold
        let password: String = "issuer_pwd".to_owned();                                                 			// Password dell'Issuer per sbloccare e accedere ai dati del suo file Stronghold
        let stronghold: Stronghold = Stronghold::new(&stronghold_path, password, None).await?;         				// Creazione Stronghold

        // Creazione dell'identità decentralizzata dell'Issuer, usando il file Stronghold come storage locale
        // La creazione si basa sulla generazione di una coppia di chiavi, sulla costruzione di un'identità e sulla sua pubblicazione nella Mainnet IOTA
        ISSUER = Some(Account::builder()
            .storage(stronghold)
            .create_identity(IdentitySetup::default())
            .await?);

        // Aggiunge all'Issuer il metodo per effettuare la verifica delle Verifiable Credentials
        ISSUER.as_mut().unwrap()
            .update_identity()
            .create_method()
            .content(MethodContent::GenerateEd25519)
            .fragment("auth_vc")
            .apply()
            .await?;

        // Restituisce il valore del DID associato all'identità appena creata 
        let iota_did: &IotaDID = ISSUER.as_mut().unwrap().did();

        println!("- Crezione Issuer avvenuta con successo");

        // Mostra lo stato locale del Documento DID
        println!("- Documento generato associato alla DID {} \n{:#?}", iota_did, ISSUER.as_mut().unwrap().document());

        // Mostra l'URL dell'Identity Resolver Explorer che consente di vedere il Documento DID online e tutta la sua storia passata
        let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
        println!("\n- Documento DID disponibile al seguente link -> {}", explorer.resolver_url(iota_did)?);
    }

    Ok(())
} 