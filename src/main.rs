use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use mode::{App, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let result = run(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Return result or error
    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
    // Create application
    let mut app = App::new();

    // Create event handler with 250ms tick rate
    let event_handler = EventHandler::new(Duration::from_millis(250));

    // Main event loop
    loop {
        // Render UI
        terminal.draw(|frame| mode::ui::render(frame, &app))?;

        // Handle events
        if let Some(event) = event_handler.next() {
            app.handle_event(event)?;
        }

        // Check if should quit
        if app.should_quit() {
            break;
        }
    }

    // If there's an exit command, write it to a file for shell integration
    if let Some(exit_cmd) = &app.exit_command {
        use std::io::Write;
        if let Ok(home) = std::env::var("HOME") {
            let cmd_file = format!("{}/.mode_exit_cmd", home);
            if let Ok(mut file) = std::fs::File::create(&cmd_file) {
                let _ = writeln!(file, "{}", exit_cmd);
            }
        }
    }

    Ok(())
}
