use anyhow::Result;

mod moduli;

#[tokio::main]
async fn main() -> Result<()> 
{
    // DID produttore -> did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ
    // DID consumatore -> did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy

	// Test modulo Autenticazione
    /*
    let _registrazione: bool = moduli::autenticazione::registrazione(String::from("did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ"), String::from("Francy160498!*")).await?;
    let _login: bool = moduli::autenticazione::login(String::from("did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ"), String::from("Francy160498!*")).await?;
    let _eliminazione: bool = moduli::autenticazione::eliminazione(String::from("did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ"), String::from("Francy160498!*")).await?;
    */

	// Test modulo Autorizzazioni
    /*
    let _fornisci_autorizzazione: bool = moduli::autorizzazioni::fornisci_autorizzazione(String::from("did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ"), String::from("did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy"), 0, 0);
    let _rimuovi_autorizzazzione: bool = moduli::autorizzazioni::rimuovi_autorizzazione(String::from("did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ"), String::from("did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy"), 0, 0);
    let _elenco_autorizzazioni: Vec<String> = moduli::autorizzazioni::elenco_autorizzazioni(String::from("did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ"));
    */

	// Test modulo Applicazioni
    /*
    let _pubblica_applicazione: bool = moduli::applicazioni::pubblica_applicazione(String::from("GPlay0"), String::from("Google Play"), String::from("https://playgoogle.com"), String::from("did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy"));
    let _modifica_applicazione: bool = moduli::applicazioni::modifica_applicazione(String::from("GPlay0"), String::from("GPlay"), String::from("Google Play"), String::from("https://playgoogle.com"), String::from("did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy"), true);
    let id_app: i32 = moduli::applicazioni::id_applicazione(String::from("GPlay"), String::from("did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy"));
    let _dati_app: String = moduli::applicazioni::dati_applicazione(id_app);
    let _elenco_app: i32 = moduli::applicazioni::elenco_applicazioni();
    let _elenco_apps_utente: Vec<String> = moduli::applicazioni::elenco_apps_utente(String::from("did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy"));
    */

	// Test modulo Operazioni
    /*
    let _aggiungi_operazione: bool = moduli::operazioni::aggiungi_operazione(0, String::from("Raccolta"), String::from("Raccoglie i dati degli utenti"), String::from("process: dati"), 0);
    let _modifica_stato_operazione_f: bool = moduli::operazioni::modifica_stato_operazione(0, String::from("Raccolta"), false);
    let _modifica_stato_operazione_t: bool = moduli::operazioni::modifica_stato_operazione(0, String::from("Raccolta"), true);
    let id_op: i32 = moduli::operazioni::id_operazione(0, String::from("Raccolta"));
    let _dati_app: String = moduli::operazioni::dati_operazione(id_op);
    let _elenco_ops: Vec<String> = moduli::operazioni::elenco_operazioni(0);
    */
    
	// Test modulo Canali
	
    Ok(())
}
