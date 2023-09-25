use std::process::Command;
use std::process::Stdio;

// Pubblicazione applicazione
pub fn pubblica_applicazione(nome: String, descrizione: String, url: String, did_proprietario: String) -> bool
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di pubblicazione
    
    println!("\n---------------------------------------");
    println!("\nPUBBLICAZIONE APPLICAZIONE\n");

    // Mostra i dati passati
    println!("- Nome: {}\n- Descrizione: {}\n- Url: {}\n- DID proprietario: {}", nome, descrizione, url, did_proprietario);

    // Esegue il comando della wasp-cli per eseguire la pubblicazione dell'applicazione sullo ISC GestioneApplicazioni
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "gestioneapplicazioni", "pubblicaApplicazione", "String", "nome", "String", &nome, "String", "descrizione", "String", &descrizione, "String", "url", "String", &url, "String", "didProprietario", "String", &did_proprietario, "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della pubblicazione
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                            // Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore pubblicazione applicazione -> {output_s}");
        esito = false;
    }
    else                                                                                                          	// Chiamata alla funzione avvenuta con successo
    {
        // Recupera l'indirizzo della transazione usata per chiamare la funzione
        let output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
        let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));
        
        // Recupera l'evento generato dalla funzione per comprendere l'esito della pubblicazione
        output = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "request", &ind_trans])
            .output()
            .unwrap();

        output_s = String::from_utf8(output.stdout).unwrap();

        if output_s.contains("gestioneapplicazioni.applicazionePubblicata")                                       	// Pubblicazione avvenuta con successo
        {
            println!("- Applicazione pubblicata!");
        }
        else                                                                                            			// Pubblicazione non avvenuta
        {
            println!("- Applicazione non pubblicata!");
            esito = false;
        }
    }

    return esito;
}

// Modifica applicazione pubblicata
pub fn modifica_applicazione(nome_attuale: String, nome: String, descrizione: String, url: String, did_proprietario: String, attiva: bool) -> bool										
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di modifica
    
    println!("\n---------------------------------------");
    println!("\nMODIFICA APPLICAZIONE\n");

    // Mostra i dati passati
    println!("- Nome attuale: {}\n- Nome nuovo: {}\n- Descrizione: {}\n- Url: {}\n- DID proprietario: {}\n- Stato: {}", nome_attuale, nome, descrizione, url, did_proprietario, attiva);

    // Esegue il comando della wasp-cli per eseguire la modifica dell'applicazione sullo ISC GestioneApplicazioni
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "gestioneapplicazioni", "modificaApplicazione", "String", "nomeAttuale", "String", &nome_attuale, "String", "nome", "String", &nome, "String", "descrizione", "String", &descrizione, "String", "url", "String", &url, "String", "didProprietario", "String", &did_proprietario, "String", "attiva", "Bool", &attiva.to_string(), "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della modifica
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                            // Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore modifica applicazione -> {output_s}");
        esito = false;
    }
    else                                                                                                          	// Chiamata alla funzione avvenuta con successo
    {
        // Recupera l'indirizzo della transazione usata per chiamare la funzione
        let output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
        let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));
        
        // Recupera l'evento generato dalla funzione per comprendere l'esito della modifica
        output = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "request", &ind_trans])
            .output()
            .unwrap();

        output_s = String::from_utf8(output.stdout).unwrap();

        if output_s.contains("gestioneapplicazioni.applicazioneModificata")                                         // Modifica avvenuta con successo
        {
            println!("- Applicazione modificata!");
        }
        else if output_s.contains("gestioneapplicazioni.applicazioneNonModificata")                               	// Modifica non avvenuta
        {
            println!("- Applicazione non modificata!");
            esito = false;
        }
        else if output_s.contains("gestioneapplicazioni.nomeAppUsato")                                    			// Modifica non avvenuta perché esiste già un'applicazione con questo nome
        {
            println!("- Nome nuovo dell'applicazione già usato!");
            esito = false;
        }
    }

    return esito;
}

// Restituisce l'ID associato all'applicazione
pub fn id_applicazione(nome: String, did_proprietario: String) -> i32											
{
    let mut esito: i32 = -1;

    println!("\n---------------------------------------");
    println!("\nID APPLICAZIONE\n");

    println!("- Nome {}\n- DID proprietario {}", nome, did_proprietario);

    // Esegue il comando della wasp-cli per recuperare l'ID dell'applicazione dello ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "idApplicazione", "String", "didProprietario", "String", &did_proprietario, "String", "nome", "String", &nome, "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "idApp", "int32"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per ottenere l'ID dell'applicazione
    let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("idApp: ", "").replace("\n", "");

    let id_app: i32 = output_s.parse::<i32>().unwrap();																// Trasformazione in intero

    if id_app == -1																									// Applicazione inesistente
    {
        println!("- Nessuna applicazione trovata!");
    }
    else																											// Applicazione esistente e ID trovato
    {
        println!("- ID applicazione: {}", output_s);
        esito = output_s.parse::<i32>().unwrap();
    }

    return esito;
}

// Restituisce i dati dell'applicazione
pub fn dati_applicazione(id_app: i32) -> String											
{
    let mut esito: String = String::from("");

    println!("\n---------------------------------------");
    println!("\nDATI APPLICAZIONE\n");

    // Esegue il comando della wasp-cli per eseguire ottenere i dati dell'applicazione dallo ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "datiApplicazione", "String", "idApp", "Int32", &id_app.to_string(), "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "datiApp", "string"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per ottenere i dati dell'applicazione
    let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("datiApp: ", "").replace("\"", "").replace("\n", "");

    if output_s.len() == 0 																							// Nessuna applicazione trovata
    {
        println!("- Nessuna applicazione trovata!");
    }
    else																											// Applicazione trovata e dati ottenuti
    {
        let valori: Vec<&str> = output_s.split("|").collect();														// Ottiene i singoli dati separati dal carattere |
        esito = output_s.clone();

        println!("- Dati applicazione:\n  - ID applicazione: {}\n  - Nome: {}\n  - Descrizione: {}\n  - Url: {}\n  - DID proprietario: {}", id_app.to_string(), valori[0], valori[1], valori[2], valori[3]);
        
    }

    return esito;
}

// Restituisce il numero di applicazioni pubblicate
pub fn elenco_applicazioni() -> i32											
{
    println!("\n---------------------------------------");
    println!("\nELENCO APPLICAZIONI\n");

    // Esegue il comando della wasp-cli per ottenere il numero di applicazioni pubblicate dallo ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "elencoApplicazioni", "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "numeroApps", "int32"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire il numero di applicazioni pubblicate
    let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("numeroApps: ", "").replace("\n", "");
    let esito = output_s.parse::<i32>().unwrap();

    if esito == 0																									// Nessuna applicazione pubblicata
    {
        println!("- Nessuna applicazione pubblicata!");
    }
    else																											// Esiste almeno un'applicazione pubblicata
    {
        println!("- {} applicazioni pubblicate!", esito);
    }

    return esito;
}

// Elenca le applicazioni sviluppate da un determinato utente
pub fn elenco_apps_utente(did_utente: String) -> Vec<String>											
{
    let mut esito: Vec<String> = Vec::new();

    println!("\n---------------------------------------");
    println!("\nELENCO APPLICAZIONI SVILUPPATE DA UN UTENTE\n");

    println!("- DID sviluppatore {}", did_utente);

    // Esegue il comando della wasp-cli per eseguire ottenere tutte le applicazioni pubblicate da un determinato utente dallo ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "elencoAppsUtente", "String", "didUtente", "String", &did_utente, "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "idApps", "string"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per ottenere gli ID delle applicazioni
    let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("idApps: ", "").replace("\"", "").replace("\n", "");

    if output_s.len() == 0 																							// Nessuna applicazione pubblicata dall'utente
    {
        println!("- Nessuna applicazione pubblicata!");
    }
    else																											// Utente ha pubblicato almeno un'applicazione
    {
        println!("- Elenco ID delle applicazioni pubblicate:");

        let id_apps = output_s.split('|');																			// Ottiene l'ID di ogni applicazione separato dal carattere |

        for id_app in id_apps
        {
            println!("  - ID applicazione: {}", id_app);
            esito.push(id_app.to_string());
        }
    }

    return esito;
}