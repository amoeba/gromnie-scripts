use gromnie_scripting_api as gromnie;

struct AutoLoginScript {
    handled: bool,
}

impl gromnie::Script for AutoLoginScript {
    fn new() -> Self {
        AutoLoginScript { handled: false }
    }

    fn id(&self) -> &str {
        "auto-login"
    }

    fn name(&self) -> &str {
        "Auto Login"
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

    fn on_event(&mut self, event: gromnie::ScriptEvent) {
        match event {
            gromnie::ScriptEvent::Game(game_event) => match game_event {
                gromnie::GameEvent::CharacterListReceived(account) => {
                    gromnie::log(&format!(
                        "Auto Login: CharacterListReceived event received, handled={}",
                        self.handled
                    ));

                    // Only handle once
                    if self.handled {
                        gromnie::log("Auto Login: Already handled, returning");
                        return;
                    }

                    // Only proceed if we're at character select screen
                    let client = gromnie::get_client_state();
                    gromnie::log(&format!(
                        "Auto Login: Client state = {:?}",
                        client.session.state
                    ));

                    if !matches!(client.scene, Scene::CharacterSelect(_)) {
                        gromnie::log(&format!(
                            "Auto Login: Not at character select, returning. State: {:?}",
                            client.session.state
                        ));
                        return;
                    }

                    self.handled = true;

                    // Check if there are any characters
                    if account.characters.is_empty() {
                        gromnie::log(&format!(
                            "Auto Login: No characters found on account. Cannot auto-login."
                        ));
                        return;
                    }

                    // Use the first character
                    let first_char = &account.characters[0];
                    gromnie::log(&format!(
                        "Auto Login: Logging in as first character '{}' (ID: {})",
                        first_char.name, first_char.character_id
                    ));

                    // Send the login action
                    gromnie::log("Auto Login: Calling login_character");
                    gromnie::login_character(
                        &account.account,
                        first_char.character_id,
                        &first_char.name,
                    );
                    gromnie::log("Auto Login: login_character called successfully");
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn on_tick(&mut self, _delta_millis: u64) {
        // No periodic logic needed
    }
}

gromnie::register_script!(AutoLoginScript);
