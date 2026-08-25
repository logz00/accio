use colorful::{Color, Colorful};


pub fn main_logo() 
{
    let logo = r#"    _            _         _             _                    
   / \   ___ ___(_) ___   | |__  _   _  | |    ___   __ _ ____
  / _ \ / __/ __| |/ _ \  | '_ \| | | | | |   / _ \ / _` |_  /
 / ___ \ (_| (__| | (_) | | |_) | |_| | | |__| (_) | (_| |/ / 
/_/   \_\___\___|_|\___/  |_.__/ \__, | |_____\___/ \__, /___|
                                 |___/              |___/     
Welcome to the Accio Recon Tool || Creator: github.com/logz00"#;

    println!("{}\n", logo.gradient(Color::SeaGreen3).bold());
}

