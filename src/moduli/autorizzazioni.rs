use std::process::Command;
use std::process::Stdio;

// Produttore fornisce autorizzazione a un consumatore
pub fn fornisci_autorizzazione(did_produttore: String, did_consumatore: String, id_app: i32, id_op: i32) -> bool								
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di concessione
    
    println!("\n---------------------------------------");
    println!("\nCONCESSIONE AUTORIZZAZIONE\n");

    // Mostra i dati passati
    println!("- DID produttore: {}\n- DID consumatore: {}\n- ID applicazione: {}\n- ID operazione: {}", did_produttore, did_consumatore, id_app, id_op);

    // Esegue il comando della wasp-cli per fornire l'autorizzazione sullo ISC Autorizzazioni
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "autorizzazioni", "fornisciAutorizzazione", "String", "didProduttore", "String", &did_produttore, "String", "didConsumatore", "String", &did_consumatore, "String", "idApplicazione", "Int32", &(id_app.to_string()), "String", "idOperazione", "Int32", &(id_op.to_string()), "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della concessione
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                           	// Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore concessione autorizzazione -> {output_s}");
        esito = false;
    }
    else                                                                                                          	// Chiamata alla funzione avvenuta con successo
    {
        // Recupera l'indirizzo della transazione usata per chiamare la funzione
        let output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
        let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));
        
        // Recupera l'evento generato dalla funzione per comprendere l'esito della concessione
        output = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "request", &ind_trans])
            .output()
            .unwrap();

        output_s = String::from_utf8(output.stdout).unwrap();

        if output_s.contains("autorizzazioni.autorizzazioneFornita")                                            	// Autorizzazione fornita con successo
        {
            println!("- Autorizzazione concessa!");
        }
        else                                                                                            			// Autorizzazione non fornita
        {
            println!("- Autorizzazione non concessa!");
            esito = false;
        }
    }

    return esito;
}

// Produttore rimuove autorizzazione concessa precedentemente a consumatore
pub fn rimuovi_autorizzazione(did_produttore: String, did_consumatore: String, id_app: i32, id_op: i32) -> bool										
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di rimozione
    
    println!("\n---------------------------------------");
    println!("\nRIMOZIONE AUTORIZZAZIONE\n");

    // Mostra i dati passati
    println!("- DID produttore: {}\n- DID consumatore: {}\n- ID applicazione: {}\n- ID operazione: {}", did_produttore, did_consumatore, id_app, id_op);

    // Esegue il comando della wasp-cli per rimuovere l'autorizzazione dallo ISC Autorizzazioni
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "autorizzazioni", "rimuoviAutorizzazione", "String", "didProduttore", "String", &did_produttore, "String", "didConsumatore", "String", &did_consumatore, "String", "idApplicazione", "Int32", &(id_app.to_string()), "String", "idOperazione", "Int32", &(id_op.to_string()), "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della rimozione
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                            // Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore rimozione autorizzazione -> {output_s}");
        esito = false;
    }
    else                                                                                                          	// Chiamata alla funzione avvenuta con successo
    {
        // Recupera l'indirizzo della transazione usata per chiamare la funzione
        let output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
        let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));
        
        // Recupera l'evento generato dalla funzione per comprendere l'esito della rimozione
        output = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "request", &ind_trans])
            .output()
            .unwrap();

        output_s = String::from_utf8(output.stdout).unwrap();

        if output_s.contains("autorizzazioni.autorizzazioneRimossa")                                            	// Rimozione avvenuta con successo
        {
            println!("- Autorizzazione rimossa!");
        }
        else                                                                                            			// Rimozione non avvenuta
        {
            println!("- Autorizzazione non rimossa!");
            esito = false;
        }
    }

    return esito;
}

// Elenca le autorizzazioni di un determinato produttore
pub fn elenco_autorizzazioni(did_produttore: String) -> Vec<String>											
{
    let mut esito: Vec<String> = Vec::new();

    println!("\n---------------------------------------");
    println!("\nELENCO AUTORIZZAZIONI\n");

    println!("- DID produttore {}", did_produttore);

    // Esegue il comando della wasp-cli per ottenere le autorizzazioni concesse dallo ISC Autorizzazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "autorizzazioni", "elencoAutorizzazioni", "String", "didProduttore", "String", &did_produttore, "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "elenco", "string"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per ottenere le autorizzazioni
    let output_s: String = String::from_utf8(output_view.stdout).unwrap();

    if output_s.len() == 0 																							// Nessuna autorizzazione concessa
    {
        println!("- Nessuna autorizzazione concessa!");
    }
    else																											// Almeno un'autorizzazione concessa
    {
        println!("- Elenco autorizzazioni concesse:");

        let autorizzazioni = output_s.split("||");																	// Ottiene le singole autorizzazioni concesse separate dai caratteri ||

        for autorizzazione in autorizzazioni																		// Per ogni autorizzazione
        {
            let autorizzazione_s = autorizzazione.replace("elenco: ", "").replace("\"", "").replace("\n", "");
            let autorizzazione_split = autorizzazione_s.split('|');													// Ottiene i dati di una specifica autorizzazione separati dal carattere |
            let valori: Vec<&str> = autorizzazione_split.collect();
            println!("  - Stato: {} | ID applicazione: {} | ID operazione: {} | DID consumatore: {}", valori[0], valori[1], valori[2], valori[3]);
        
            esito.push(autorizzazione_s.to_string());
        }
    }

    return esito;
}