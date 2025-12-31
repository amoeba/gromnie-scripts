use gromnie_scripting_api as gromnie;

struct HelloWorld {
     timer_id: Option<u64>,
 }
 
 impl gromnie::Script for HelloWorld {
    fn new() -> Self {
        HelloWorld { timer_id: None }
    }

    fn id(&self) -> &str {
        "hello_world_wasm"
    }

    fn name(&self) -> &str {
        "Hello World (WASM)"
    }

    fn description(&self) -> &str {
        "WASM component that sends a greeting 5 seconds after an object is created"
    }

    fn on_load(&mut self) {
        gromnie::log("Hello World script loaded!");
    }

    fn on_unload(&mut self) {
        gromnie::log("Hello World script unloaded!");
    }

    fn subscribed_events(&self) -> Vec<u32> {
        vec![gromnie::events::EVENT_CREATE_OBJECT]
    }

    fn on_event(&mut self, event: gromnie::GameEvent) {
        match event {
            gromnie::GameEvent::CreateObject(obj) => {
                gromnie::log(&format!("Object created: {} (ID: {})", obj.name, obj.id));

                // Schedule a 5-second timer
                let timer_id = gromnie::schedule_timer(5, "greeting");
                self.timer_id = Some(timer_id);

                gromnie::log("Scheduled 5-second greeting timer");
            }
            _ => {}
        }
    }

    fn on_tick(&mut self, _delta_millis: u64) {
        // Check if our timer has fired
        if let Some(timer_id) = self.timer_id {
            if gromnie::check_timer(timer_id) {
                gromnie::log("Timer fired! Sending greeting...");
                gromnie::send_chat("Hello from WASM! 👋");
                self.timer_id = None;
            }
        }
    }
}

gromnie::register_script!(HelloWorld);
