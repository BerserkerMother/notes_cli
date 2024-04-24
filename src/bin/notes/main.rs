use std::{
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{self, Duration, Instant},
};

use crossterm::{
    event::{self, Event as CEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use notes_cli::{get_handler, CrossTerminal, Event};
use tui::{backend::CrosstermBackend, Terminal};

fn initialized_terminal() -> Result<CrossTerminal, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn cleanup_terminal(mut terminal: CrossTerminal) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    terminal.flush()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    let (tx2, rx2): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let tick_rate = Duration::from_millis(20);
    thread::spawn(move || {
        let mut last_tick = time::Instant::now();
        let mut input_enabled = true;
        loop {
            if let Ok(state) = rx2.try_recv() {
                input_enabled = !state; // Toggle input processing based on editor mode
            }

            if input_enabled {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                if event::poll(timeout).expect("poll doesn't work!") {
                    if let CEvent::Key(key) = event::read().expect("can't read event") {
                        tx.send(Event::Input(key)).expect("can't send the event!");
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    let mut terminal = initialized_terminal()?;
    let mut handler = get_handler(terminal.get_frame().size())?;
    loop {
        // let ref_active
        terminal.draw(|f| {
            handler.render(f).expect("unexpected error!");
        })?;
        let event = rx.recv()?;
        if handler.is_editor_mode() {
            cleanup_terminal(terminal)?;
            tx2.send(true)?;
            // io::stdout().flush().unwrap();
            handler.handle_edit()?;
            terminal = initialized_terminal()?;
            tx2.send(false)?;
        }
        handler.handle_event(event)?;
        if handler.should_exit() {
            cleanup_terminal(terminal)?;
            break;
        }
    }
    Ok(())
}
