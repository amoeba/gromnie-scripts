use gromnie_scripting_api as gromnie;

struct Counter {
    count: i32,
    timer_id: Option<u64>,
}

impl gromnie::Script for Counter {
    fn new() -> Self {
        Counter {
            count: 0,
            timer_id: None,
        }
    }

    fn id(&self) -> &str {
        "counter"
    }

    fn name(&self) -> &str {
        "Counter"
    }

    fn description(&self) -> &str {
        "Increments a counter every second and logs the value"
    }

    fn on_load(&mut self) {
        gromnie::log("Counter script loaded!");
        // Schedule a 1-second repeating timer
        self.timer_id = Some(gromnie::schedule_timer(1000, "increment"));
        gromnie::log("Counter timer started");
    }

    fn on_unload(&mut self) {
        gromnie::log(&format!(
            "Counter script unloaded! Final count: {}",
            self.count
        ));
    }

    fn subscribed_events(&self) -> Vec<u32> {
        vec![]
    }

    fn on_event(&mut self, _event: gromnie::ScriptEvent) {
        // No events to handle
    }

    fn on_tick(&mut self, _delta_millis: u64) {
        // Check if our timer has fired
        if let Some(timer_id) = self.timer_id {
            if gromnie::check_timer(timer_id) {
                self.count += 1;
                gromnie::log(&format!("Counter: {}", self.count));
                // Reschedule the timer for the next second
                self.timer_id = Some(gromnie::schedule_timer(1000, "increment"));
            }
        }
    }
}

gromnie::register_script!(Counter);
