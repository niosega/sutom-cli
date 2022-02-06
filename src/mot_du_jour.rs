use reqwest;

pub fn mot_du_jour() -> String {
    let response = reqwest::blocking::get("https://sutom.nocle.fr/mots/30.txt")
        .unwrap()
        .text();
    return response.unwrap();
}