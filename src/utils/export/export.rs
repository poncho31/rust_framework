use mysql::*;
use mysql::prelude::*;
use dotenv::from_path;
use serde_json::{Value as JsonValue, json};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use csv::WriterBuilder;
use std::path::Path;

fn run() -> Result<(), Box<dyn Error>> {
    // Récupérer les arguments du programme
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <requête SQL> ou --file <chemin_du_fichier_JSON>");
        return Ok(());
    }

    // Déterminer si l'entrée est un fichier JSON ou une requête brute
    let (sql_query, output_path, filename) = if args[1] == "--file" {
        // Charger la requête et les paramètres depuis un fichier JSON
        if args.len() < 3 {
            eprintln!("Erreur: Spécifiez un chemin de fichier après --file");
            return Ok(());
        }
        let file_path = &args[2];
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let json: JsonValue = serde_json::from_str(&content)?;

        // Extraire les paramètres
        let sql_query = match json["query"].as_str() {
            Some(query) => query.to_string(),
            None => {
                eprintln!("Erreur: La clé 'query' est absente ou invalide dans le fichier JSON");
                return Ok(());
            }
        };

        let output_path = match json["output_path"].as_str() {
            Some(path) => path.to_string(),
            None => {
                eprintln!("Erreur: La clé 'output_path' est absente ou invalide dans le fichier JSON");
                return Ok(());
            }
        };

        let filename = match json["filename"].as_str() {
            Some(name) => name.to_string(),
            None => {
                eprintln!("Erreur: La clé 'filename' est absente ou invalide dans le fichier JSON");
                return Ok(());
            }
        };

        (sql_query, output_path, filename)
    } else {
        // La requête brute est le deuxième argument, les autres paramètres ont des valeurs par défaut
        let sql_query   = args[1].clone();
        let output_path = "".to_string();
        let filename    = "output.csv".to_string();
        (sql_query, output_path, filename)
    };

    let env_path = Path::new("../../../../../rust.env");
    from_path(env_path)?;

    // Récupérer les informations de connexion depuis les variables d'environnement
    let username = env::var("DB_USERNAME")?;
    let password = env::var("DB_PASSWORD")?;
    let host     = env::var("DB_HOST")?;
    let port     = env::var("DB_PORT")?;
    let database = env::var("DB_DATABASE")?;

    // Configurer la connexion MySQL
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(host))
        .tcp_port(port.parse()?)
        .user(Some(username))
        .pass(Some(password))
        .db_name(Some(database));

    let pool = Pool::new(opts)?;
    let mut conn = pool.get_conn()?;

    // Configurer la session MySQL pour UTF-8
    conn.query_drop("SET NAMES utf8mb4")?;
    conn.query_drop("SET CHARACTER SET utf8mb4")?;
    conn.query_drop("SET character_set_results = utf8mb4")?;

    // Exécuter la requête SQL
    let result_set: Vec<Row> = conn.query(sql_query)?;

    // Création fichier CSV
    let mut file = File::create(format!("{}\\{}", output_path, filename))?;
    file.write_all(b"\xEF\xBB\xBF")?; // Ajout du BOM pour UTF-8

    let mut wtr = WriterBuilder::new()
        .delimiter(b';')
        .quote_style(csv::QuoteStyle::Always)
        .from_writer(file);

    // Écrire les en-têtes
    if let Some(first_row) = result_set.first() {
        let headers: Vec<String> = first_row
            .columns_ref()
            .iter()
            .map(|col| col.name_str().to_string())
            .collect();
        wtr.write_record(&headers)?;
    }

    // Écrire les lignes
    for row in result_set {
        let values: Vec<String> = row
            .unwrap()
            .into_iter()
            .map(|val| match val {
                Value::Bytes(bytes) => String::from_utf8(bytes).unwrap_or_else(|_| "[INVALID UTF-8]".to_string()),
                Value::NULL => "".to_string(),
                _ => val.as_sql(false).to_string(),
            })
            .collect();
        wtr.write_record(values)?;
    }

    wtr.flush()?;


    let data = json!(
        {
            "filename" : filename,
            "filepath" : output_path,

        }
    );

    println!("{}", data.to_string());

    Ok(())
}
