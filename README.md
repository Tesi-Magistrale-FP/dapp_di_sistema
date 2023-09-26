# dApp di sistema
La <strong>dApp di sistema</strong> nasce per offrire agli utenti un’<strong>applicazione per interagire facilmente con l’ecosistema</strong>. 

## Funzioni offerte
La dApp di sistema interagisce con gli smart contract e con i dati nei canali, offrendo le seguenti categorie di funzionalità:
- <strong>Utente:</strong> offerte ai generici utenti dell’ecosistema. Comprendono l’autenticazione e l’esplorazione delle applicazioni pubblicate con le relative operazioni sui dati. Nel primo caso, avviene un’interazione con lo ISC Autenticazione e viene sfruttato anche il framework IOTA Identity. Nel secondo caso, viene adoperato lo ISC GestioneApplicazioni per leggere le informazioni delle app pubblicate e delle operazioni annesse.
- <strong>Produttore:</strong> dedicate agli utenti che ricoprono il ruolo di produttore. Quest’ultimo deve poter gestire le autorizzazioni che ha concesso, accedere ai dati prodotti e controllare i log. Per far ciò, è necessario interagire con gli smart contract Autorizzazioni e GestoreApplicazioni sia per recuperare tutte le autorizzazioni concesse sia per ottenere i riferimenti ai canali dati e log usati. Tramite questi ultimi, sfruttando il framework IOTA Streams, diventa possibile accedere ai canali per leggere i dati prodotti e i log.
- <strong>Consumatore:</strong> specifiche per gli utenti che svolgono il ruolo di consumatore. Nell’ecosistema trattato, il consumatore è un semplice utente che possiede l’autorizzazione per accedere ai dati di un produttore. Più nello specifico, un consumatore spesso può essere anche lo sviluppatore di un’applicazione decentralizzata usata proprio dai produttori. Di conseguenza, tra le funzionalità offerte al consumatore deve esserci la possibilità di pubblicare nuove applicazioni, gestire quelle già pubblicate, aggiungere o rimuovere operazioni sui dati e accedere ai dati dei produttori, sempre se l’autorizzazione ricevuta lo contempla. Anche in questo caso, bisogna interagire con lo smart contract GestoreApplicazioni e con il canale dati.

## Moduli software
La dApp di sistema è composta da diversi moduli Rust:
- [autenticazione.rs](https://github.com/Tesi-Magistrale-FP/dapp_di_sistema/blob/main/src/moduli/autenticazione.rs): gestisce l'autenticazione degli utenti dell'ecosistema:
  - <strong>Registrazione:</strong> registra un utente nell'ecosistema.
  - <strong>Login:</strong> effettua il login di un utente nell'ecosistema.
  - <strong>Eliminazione:</strong> elimina l'account creato da un utente dell'ecosistema.
  - <strong>Registra credenziali verificabili:</strong> registra le credenziali verificabili usate da un utente per un'applicazione.
  - <strong>Verifica credenziali verificabili:</strong> verifica la correttezza delle credenziali verificabili di un utente.
- [applicazioni.rs](https://github.com/Tesi-Magistrale-FP/dapp_di_sistema/blob/main/src/moduli/applicazioni.rs): gestisce le applicazioni decentralizzate:
  - <strong>Pubblica applicazione:</strong> pubblicazione di un'applicazione sullo store.
  - <strong>Modifica applicazione:</strong> modifica un'applicazione pubblicata.
  - <strong>ID applicazione:</strong> restituisce l'ID di un'applicazione.
  - <strong>Dati applicazione:</strong> restituisce i dati di un'applicazione.
  - <strong>Elenco applicazioni:</strong> restituisce il numero di applicazioni pubblicate.
  - <strong>Elenco applicazioni utente:</strong> restituisce gli ID delle applicazioni pubblicate da un utente.
- [operazioni.rs](https://github.com/Tesi-Magistrale-FP/dapp_di_sistema/blob/main/src/moduli/operazioni.rs): gestisce le operazioni sui dati delle applicazioni decentralizzate:
  - <strong>Aggiungi operazione:</strong> aggiunge un'operazione sui dati a un'applicazione pubblicata.
  - <strong>Modifica stato operazione:</strong> modifica lo stato (attiva/disattiva) di un'operazione sui dati.
  - <strong>ID operazione:</strong> restituisce l'ID di un'operazione.
  - <strong>Dati operazione:</strong> restituisce i dati di un'operazione.
  - <strong>Elenco operazioni:</strong> restituisce gli ID delle operazioni sui dati associate a un'applicazione.
- [autorizzazioni.rs](https://github.com/Tesi-Magistrale-FP/dapp_di_sistema/blob/main/src/moduli/autorizzazioni.rs): gestisce le autorizzazioni concesse dai produttori ai consumatori:
  - <strong>Fornisci autorizzazione:</strong> produttore fornisce l'autorizzazione a un consumatore per un'operazione sui dati di un'applicazione.
  - <strong>Rimuovi autorizzazione:</strong> produttore rimuove un'autorizzazione precedentemente concessa.
  - <strong>Elenco autorizzazioni:</strong> restituisce i dettagli delle autorizzazioni fornite da un produttore.