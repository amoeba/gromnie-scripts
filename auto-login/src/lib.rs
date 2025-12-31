use gromnie_scripting_api as gromnie;

struct AutoLoginScript {
    handled: bool,
}

impl gromnie::Script for AutoLoginScript {
    fn new() -> Self {
        AutoLoginScript { handled: false }
    }

    fn id(&self) -> &str {
        "auto_login_wasm"
    }

    fn name(&self) -> &str {
        "Auto Login (WASM)"
    }

    fn description(&self) -> &str {
        "Automatically logs in using the first available character. Errors if no characters exist."
    }

    fn on_load(&mut self) {
        gromnie::log("Auto Login script loaded");
    }

    fn on_unload(&mut self) {
        gromnie::log("Auto Login script unloaded");
    }

    fn subscribed_events(&self) -> Vec<u32> {
        // Subscribe to CharacterListReceived events
        vec![gromnie::events::EVENT_CHARACTER_LIST_RECEIVED]
    }

    fn on_event(&mut self, event: gromnie::GameEvent) {
        match event {
            gromnie::GameEvent::CharacterListReceived(account) => {
                // Only handle once
                if self.handled {
                    return;
                }
                self.handled = true;

                // Check if there are any characters
                if account.character_list.is_empty() {
                    gromnie::log(&format!(
                        "Auto Login: No characters found on account '{}'. Cannot auto-login.",
                        account.name
                    ));
                    return;
                }

                // Use the first character
                let first_char = &account.character_list[0];
                gromnie::log(&format!(
                    "Auto Login: Logging in as first character '{}' (ID: {})",
                    first_char.name, first_char.id
                ));

                // Send the login action
                gromnie::login_character(
                    &account.name,
                    first_char.id,
                    &first_char.name,
                );
            }
            _ => {}
        }
    }

    fn on_tick(&mut self, _delta_millis: u64) {
        // No periodic logic needed
    }
}

gromnie::register_script!(AutoLoginScript);
