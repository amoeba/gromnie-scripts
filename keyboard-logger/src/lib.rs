use gromnie_scripting_api as gromnie;

struct KeyboardLogger;

impl gromnie::Script for KeyboardLogger {
    fn new() -> Self {
        KeyboardLogger
    }

    fn id(&self) -> &str {
        "keyboard-logger"
    }

    fn name(&self) -> &str {
        "Keyboard Logger (WASM)"
    }

    fn description(&self) -> &str {
        "Logs all chat messages received (placeholder for keyboard input logging)"
    }

    fn on_load(&mut self) {
        gromnie::log("Keyboard Logger loaded!");
    }

    fn on_unload(&mut self) {
        gromnie::log("Keyboard Logger unloaded!");
    }

    fn subscribed_events(&self) -> Vec<u32> {
        vec![gromnie::events::EVENT_CHAT_MESSAGE_RECEIVED]
    }

    fn on_event(&mut self, event: gromnie::ScriptEvent) {
        match event {
            gromnie::ScriptEvent::Game(game_event) => match game_event {
                gromnie::GameEvent::ChatMessageReceived(chat_message) => {
                    gromnie::log(&format!(
                        "Chat [{}]: {}",
                        chat_message.channel, chat_message.message
                    ));
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn on_tick(&mut self, _delta_millis: u64) {
        // No-op for event logger
    }
}

gromnie::register_script!(KeyboardLogger);
