use std::process::Command;
use std::process::Stdio;

// Aggiunge un'operazione
pub fn aggiungi_operazione(id_app: i32, titolo: String, descrizione: String, software: String, tipo: i32) -> bool
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di aggiunta
    
    println!("\n---------------------------------------");
    println!("\nAGGIUNTA OPERAZIONE\n");

    if tipo != 0 && tipo != 1 && tipo != 2
    {
        return false;
    }

    // Mostra i dati passati
    println!("- ID applicazione: {}\n- Titolo: {}\n- Descrizione: {}\n- Software: {}\n- Tipo: {}", id_app, titolo, descrizione, software, tipo);

    // Esegue il comando della wasp-cli per eseguire l'aggiunta dell'operazione sullo ISC GestioneApplicazioni
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "gestioneapplicazioni", "aggiungiOperazione", "String", "idApp", "Int32", &id_app.to_string(), "String", "titolo", "String", &titolo, "String", "descrizione", "String", &descrizione, "String", "software", "String", &software, "String", "tipo", "Int32", &tipo.to_string(), "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito dell'aggiunta
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                            // Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore aggiunta operazione -> {output_s}");
        esito = false;
    }
    else                                                                                                          	// Chiamata alla funzione avvenuta con successo
    {
        // Recupera l'indirizzo della transazione usata per chiamare la funzione
        let output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
        let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));
        
        // Recupera l'evento generato dalla funzione per comprendere l'esito dell'aggiunta
        output = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "request", &ind_trans])
            .output()
            .unwrap();

        output_s = String::from_utf8(output.stdout).unwrap();

        if output_s.contains("gestioneapplicazioni.operazioneAggiunta")                                            	// Aggiunta avvenuta con successo
        {
            println!("- Operazione aggiunta!");
        }
        else                                                                                            			// Aggiunta non avvenuta
        {
            println!("- Operazione non aggiunta!");
            esito = false;
        }
    }

    return esito;
}

// Modifica lo stato di un'operazione
pub fn modifica_stato_operazione(id_app: i32, titolo: String, nuovo_stato: bool) -> bool										
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di modifica
    
    println!("\n---------------------------------------");
    println!("\nMODIFICA APPLICAZIONE\n");

    // Mostra i dati passati
    println!("- ID applicazione: {}\n- Titolo: {}\n- Nuovo stato: {}", id_app, titolo, nuovo_stato);

    // Esegue il comando della wasp-cli per modificare lo stato di un'operazione sullo ISC GestioneApplicazioni
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "gestioneapplicazioni", "modificaOperazione", "String", "idApp", "Int32", &id_app.to_string(), "String", "titolo", "String", &titolo, "String", "nuovoStato", "Bool", &nuovo_stato.to_string(), "--chain=mychain", "-s"])
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

        if output_s.contains("gestioneapplicazioni.operazioneAttivata")                                            	// Modifica avvenuta con successo
        {
            println!("- Operazione attivata!");
        }
        else if output_s.contains("gestioneapplicazioni.operazioneDisattivata")                                    	// Modifica non avvenuta
        {
            println!("- Operazione disattivata!");
        }
        else if output_s.contains("gestioneapplicazioni.operazioneInesistente")                        				// Operazione non esistente
        {
            println!("- Operazione inesistente!");
            esito = false;
        }
    }

    return esito;
}

// Restituisce l'ID associato all'operazione
pub fn id_operazione(id_app: i32, titolo: String) -> i32											
{
    let mut esito: i32 = -1;

    println!("\n---------------------------------------");
    println!("\nID OPERAZIONE\n");

    // Mostra i dati passati
    println!("- ID applicazione: {}\n- Titolo: {}", id_app, titolo);

    // Esegue il comando della wasp-cli per eseguire ottenere l'ID dell'operazione dallo ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "idOperazione", "String", "titolo", "String", &titolo, "String", "idApp", "Int32", &id_app.to_string(), "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "idOp", "int32"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per ottenere l'ID dell'operazione
    let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("idOp: ", "").replace("\n", "");

    let id_op: i32 = output_s.parse::<i32>().unwrap();

    if id_op == -1																									// Operazione inesistente
    {
        println!("- Nessuna operazione trovata!");
    }
    else																											// Operazione esiste e restituisce l'ID
    {
        println!("- ID operazione: {}", output_s);
        esito = output_s.parse::<i32>().unwrap();
    }

    return esito;
}

// Restituisce i dati dell'operazione
pub fn dati_operazione(id_op: i32) -> String											
{
    let mut esito: String = String::from("");

    println!("\n---------------------------------------");
    println!("\nDATI OPERAZIONE\n");

    // Esegue il comando della wasp-cli per ottenere i dati dell'operazione dallo ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "datiOperazione", "String", "idOp", "Int32", &id_op.to_string(), "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "datiOp", "string"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per ottenere i dati dell'operazione
    let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("datiOp: ", "").replace("\"", "").replace("\n", "");

    if output_s.len() == 0																							// Nessuna operazione trovata
    {
        println!("- Nessuna operazione trovata!");
    }
    else																											// Operazione trovata e restituisce i suoi dati
    {
        let valori: Vec<&str> = output_s.split("|").collect();														// Ottiene i singoli dati che sono separati dal carattere |
        esito = output_s.clone();

        println!("- Dati operazione:\n  - ID applicazione: {}\n  - ID operazione: {}\n  - Titolo: {}\n  - Descrizione: {}\n  - Software: {}\n  - Tipo: {}", valori[0], id_op, valori[1], valori[2], valori[3], valori[4]);
    }

    return esito;
}

// Restituisce gli ID delle operazioni di una specifica applicazione
pub fn elenco_operazioni(id_app: i32) -> Vec<String>											
{
    let mut esito: Vec<String> = Vec::new();

    println!("\n---------------------------------------");
    println!("\nELENCO OPERAZIONI DI UNA CERTA APPLICAZIONE\n");

    println!("- ID applicazione {}", id_app);

    // Esegue il comando della wasp-cli per eseguire gli ID delle operazioni associate a un'applicazione dallo ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "elencoOperazioni", "String", "idApp", "Int32", &id_app.to_string(), "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "idOps", "string"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per ottenere gli ID delle operazione
    let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("idOps: ", "").replace("\"", "").replace("\n", "");

    if output_s.len() == 0 																							// Nessuna operazione presente
    {
        println!("- Nessuna operazione presente!");
    }
    else																											// Operazione presente
    {
        println!("- Elenco ID delle operazioni associate:");

        let id_ops = output_s.split('|');																			// Ottiene gli ID delle operazioni separati dal carattere |

        for id_op in id_ops
        {
            println!("  - ID operazione: {}", id_op);
            esito.push(id_op.to_string());
        }
    }

    return esito;
}