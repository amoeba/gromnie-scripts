use gromnie_scripting_api as gromnie;

struct Echo;

impl gromnie::Script for Echo {
    fn new() -> Self {
        Echo
    }

    fn id(&self) -> &str {
        "echo"
    }

    fn name(&self) -> &str {
        "Echo Script (WASM)"
    }

    fn description(&self) -> &str {
        "Echoes all events received by the scripting system for debugging purposes"
    }

    fn on_load(&mut self) {
        gromnie::log("Echo Script loaded!");
    }

    fn on_unload(&mut self) {
        gromnie::log("Echo Script unloaded!");
    }

    fn subscribed_events(&self) -> Vec<u32> {
        // Subscribe to all events
        vec![
            gromnie::events::EVENT_ALL,
        ]
    }

    fn on_event(&mut self, event: gromnie::ScriptEvent) {
        gromnie::log(&format!("{:?}", event));
    }

    fn on_tick(&mut self, _delta_millis: u64) {
        // No-op for event logger
    }
}

gromnie::register_script!(Echo);
