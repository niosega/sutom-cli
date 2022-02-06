mod dictionnaire;
mod mot_du_jour;
use std::io;
use std::io::Write;
use std::{thread, time};

fn uppercase(name: &mut String) {
    let length = name.len();
    for _n in 0..length {
        if let Some(letter) = name.chars().next() {
            name.remove(0);
            name.push(letter.to_uppercase().next().unwrap());
        }
    }
}

fn verifie_mot(mot_joueur: &String, mot_du_jour: &String) -> Result<bool, &'static str> {
    let long_joueur = mot_joueur.chars().count();
    let long_jour = mot_du_jour.chars().count();

    if long_joueur != long_jour {
        return Err("Mauvaise longueur !");
    }

    if !dictionnaire::MOTS.contains(&&mot_joueur[..]) {
        return Err("Mot inconnu dans le dictionnaire !");
    }

    let mut error = 0;
    for idx in 0..long_jour {
        if mot_du_jour.chars().nth(idx) == mot_joueur.chars().nth(idx) {
            print!("{}", "🟥");
            // 🟡
        } else {
            error += 1;
            print!("{}", '🟦');
        }
        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
        std::io::stdout().flush().unwrap();
    }
    print!("\n");

    Ok(error == 0)
}

fn main() {
    let mot_du_jour = mot_du_jour::mot_du_jour();

    for idx in 0..mot_du_jour.chars().count() {
        if idx == 0 {
            print!("{}", mot_du_jour.chars().nth(0).unwrap());
        } else {
            print!(" _ ");
        }
    }
    print!("\n");

    let mut idx = 0;
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        uppercase(& mut buffer);
        buffer = buffer[..buffer.chars().count() - 1].to_owned();
        
        let res = verifie_mot(&buffer, &mot_du_jour);
        match res {
            Ok(success) => {
                if success {
                    println!("");
                    println!("C'est gagné !");
                    break;
                }
            },
            Err(msg) => {
                println!("{}", msg);
                println!("");
                continue;
            }
        }
        
        println!("");
        idx += 1;
        if idx == 6 {
            println!("Perdu !");
            break;
        }
    }
}
