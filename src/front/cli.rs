use std::collections::LinkedList;
use std::io::{self, Write};
use crossterm::{cursor, event, terminal, ExecutableCommand};
use crossterm::event::{KeyCode, KeyEvent};
use defer::defer;

pub fn cli_run() -> io::Result<()> {
    // Initialize the terminal
    terminal::enable_raw_mode()?;

    // Defer disabling raw mode to restore terminal when the function exits
    defer!({let _ = terminal::disable_raw_mode();});

    // Get stdout to write to the terminal
    let mut stdout = io::stdout();

    // Clear the terminal screen
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::EnableBlinking)?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    // Print a welcome message
    println!("Welcome to the CLI. Press 'q' to quit.");

    // Main input loop
    loop {
        let mut  token_stack : LinkedList<String> = LinkedList::new();

        // If there's an event available (like a key press), handle it
        if event::poll(std::time::Duration::from_millis(100))? {
            // Read the event
            if let event::Event::Key(KeyEvent { code, modifiers, kind, state }) = event::read()? {
                match code {
                    // If the user presses 'q' or 'Esc', quit the loop
                    KeyCode::Char('q') => {
                        println!("Quitting...");
                        break;
                    },
                    KeyCode::Esc => {
                        println!("Exiting...");
                        break;
                    },
                    // Handle other keys
                    KeyCode::Char(c) => {
                        print!("You pressed: '{}'", c);
                        stdout.flush()?;
                    },
                    KeyCode::Backspace => {
                        print!("")
                    }
                    _ => {}
                }
            }
        }
    }

    // Ensure the terminal is restored when we're done
    Ok(())
}
