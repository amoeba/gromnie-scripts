use gromnie_scripting_api as gromnie;
use std::fs::OpenOptions;
use std::io::Write;

struct FileLoggerScript {
    log_file: Option<std::fs::File>,
}

impl gromnie::Script for FileLoggerScript {
    fn new() -> Self {
        FileLoggerScript { log_file: None }
    }

    fn id(&self) -> &str {
        "file-logger"
    }

    fn name(&self) -> &str {
        "File Logger"
    }

    fn description(&self) -> &str {
        "Logs chat messages to /script_data/chat.log"
    }

    fn on_load(&mut self) {
        // Open log file in append mode
        // WASI preopens /script_data to ~/.config/gromnie/script_data/
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("/script_data/chat.log")
        {
            Ok(file) => {
                self.log_file = Some(file);

                // Write a startup message
                if let Some(ref mut f) = self.log_file {
                    let _ = writeln!(f, "\n=== File Logger WASM started ===");
                    let _ = f.flush();
                }

                gromnie::log("File logger started - logging to /script_data/chat.log");
            }
            Err(e) => {
                // Can't log to file
                let error_msg = format!("Failed to open log file: {}", e);
                gromnie::log(&error_msg);
                gromnie::send_chat(&error_msg);
            }
        }
    }

    fn on_unload(&mut self) {
        // Write shutdown message and close file
        if let Some(ref mut f) = self.log_file {
            let _ = writeln!(f, "=== File Logger WASM stopped ===\n");
            let _ = f.flush();
        }
        self.log_file = None;
        gromnie::log("File logger stopped");
    }

    fn subscribed_events(&self) -> Vec<u32> {
        // Subscribe to chat messages
        vec![gromnie::events::EVENT_CHAT_MESSAGE_RECEIVED]
    }

    fn on_event(&mut self, event: gromnie::ScriptEvent) {
        match event {
            gromnie::ScriptEvent::Game(game_event) => match game_event {
                gromnie::GameEvent::ChatMessageReceived(chat_info) => {
                    if let Some(ref mut file) = self.log_file {
                        // Get current timestamp
                        let timestamp = gromnie::get_event_time_millis();

                        // Log the chat message
                        let log_entry = format!(
                            "[{}] [channel:{}] {}\n",
                            timestamp, chat_info.channel, chat_info.message
                        );

                        if let Err(e) = file.write_all(log_entry.as_bytes()) {
                            // If write fails, log the error
                            let error_msg = format!("Log write failed: {}", e);
                            gromnie::log(&error_msg);
                            gromnie::send_chat(&error_msg);
                        } else {
                            // Flush to ensure it's written immediately
                            let _ = file.flush();
                        }
                    }
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

gromnie::register_script!(FileLoggerScript);
