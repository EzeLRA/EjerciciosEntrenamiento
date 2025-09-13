use std::io;

fn abreviar(cad: &str) -> String {
    if cad.len() > 10 {
        let first_char = cad.chars().next().unwrap_or_default();
        let last_char = cad.chars().last().unwrap_or_default();
        let middle_length = cad.len() - 2;
        format!("{}{}{}", first_char, middle_length, last_char)
    } else {
        cad.to_string()
    }
}

fn main() {
    
    let mut entrada = String::new(); 

    if let Ok(_) = io::stdin().read_line(&mut entrada) {
        let cad = entrada.trim();
        
        if !cad.chars().all(|c| c.is_ascii_digit()) {
            println!("{}", abreviar(cad));
        }
    }

}
