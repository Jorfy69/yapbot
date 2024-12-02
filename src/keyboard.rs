use device_query::{DeviceQuery, DeviceState};
    use std::collections::HashSet;
    
    use crate::audio;
    use crate::settings::Config;


    pub fn new_keyboard_state() -> DeviceState{
        return DeviceState::new();   
    }

    pub fn get_keyboard_event(state : &DeviceState) -> Vec<device_query::Keycode> {

        let key: Vec<device_query::Keycode>  = state.get_keys();
        
        return key; 
    }
    
    pub fn handle_keys(config: &Config, state: &DeviceState) {
        loop {
            // Get the current key event
            let keys = get_keyboard_event(state);
            
            let pressed_keys: HashSet<String> = keys.iter()
            .map(|key| format!("{:?}", key))  // Convert each KeyCode to its string representation
            .collect();
            
            let exit_key: HashSet<String> = vec![config.exit_key.clone()].into_iter().collect();

            if pressed_keys == exit_key {
                println!("Exiting");
                break;
            }
            // Check if the pressed keys match any key combo in the config
            for combo in &config.key_combos {
                let expected_keys: HashSet<_> = combo.keys.iter().cloned().collect();
                 if expected_keys == pressed_keys {
                    // Play the associated audio
                    println!("{}", combo.audio);
                    audio::play(&combo.audio);
                    
                    // Print the name of the key combo
                    println!("Key combo triggered: {}", combo.name);
    
                    
                }
            }
        }
    }
