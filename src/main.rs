mod dictionnaire;
mod mot_du_jour;
use std::io;
use std::io::Write;
use std::{thread, time};
use std::collections::HashMap;
use termion::{color, style};

fn uppercase(name: &mut String) {
    let length = name.len();
    for _n in 0..length {
        if let Some(letter) = name.chars().next() {
            name.remove(0);
            name.push(letter.to_uppercase().next().unwrap());
        }
    }
}

fn frequences_lettre(mot_joueur: &String) -> HashMap<char, i64> {
    let mut frequences = HashMap::new();
    for idx in 0x41..0x5B {
        frequences.insert(idx as u8 as char, 0);
    }

    for c in mot_joueur.chars() {
        let count = frequences.entry(c).or_insert(0);
        *count += 1;
    }

    return frequences;
}

fn verifie_mot(mot_joueur: &String, mot_du_jour: &String, frequences: &HashMap<char, i64>) -> Result<bool, &'static str> {
    let long_joueur = mot_joueur.chars().count();
    let long_jour = mot_du_jour.chars().count();

    if long_joueur != long_jour {
        return Err("Mauvaise longueur !");
    }

    if !dictionnaire::MOTS.contains(&&mot_joueur[..]) {
        return Err("Mot inconnu dans le dictionnaire !");
    }

    let mut error = 0;
    let mut local_frequences = frequences.clone();
    for idx in 0..long_jour {
        let c = mot_joueur.chars().nth(idx).unwrap();
        let nb_occurrence = local_frequences.get(&c).unwrap();
        if mot_du_jour.chars().nth(idx).unwrap() == c {
            print!("{}{}{}", color::Bg(color::Red), c, color::Bg(color::Reset));
            let count = local_frequences.entry(c).or_insert(0);
            *count -= 1;
        } else if nb_occurrence > &0 {
            print!("{}{}{}{}{}", color::Fg(color::Black), color::Bg(color::LightYellow), c, color::Bg(color::Reset), color::Fg(color::Reset));
            let count = local_frequences.entry(c).or_insert(0);
            *count -= 1;
            error += 1
        } else {
            error += 1;
            print!("{}{}{}", color::Bg(color::Blue), c, color::Bg(color::Reset));
        }
        print!(" ");
        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
        std::io::stdout().flush().unwrap();
    }
    print!("\n");

    Ok(error == 0)
}

fn main() {
    print!("{}", style::Bold);
    let mot_du_jour = mot_du_jour::mot_du_jour();
    let frequences = frequences_lettre(&mot_du_jour);

    println!("Mot de {} lettres !", mot_du_jour.chars().count());
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
        
        let res = verifie_mot(&buffer, &mot_du_jour, &frequences);
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
    print!("{}", style::NoBold);
}
