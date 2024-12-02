mod keyboard;
mod audio;
mod settings;


use keyboard::*;

fn main() {
   let config = settings::read_settings("./src/settings.json");


   
    let state = keyboard::new_keyboard_state();
    handle_keys(&config, &state);
    

}
