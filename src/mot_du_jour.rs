use reqwest;
use chrono::{DateTime, Utc};

pub fn mot_du_jour() -> String {

    let aujourdhui: DateTime<Utc> = Utc::now();
    let origine = DateTime::parse_from_rfc3339("2022-01-08T00:00:00-08:00").unwrap().with_timezone(&Utc);
    let numero_grille = (aujourdhui - origine).num_days() + 1;
    let response = reqwest::blocking::get(format!("https://sutom.nocle.fr/mots/{}.txt", numero_grille))
        .unwrap()
        .text();
    return response.unwrap();
}