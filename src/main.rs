
mod utils;

use utils::{get_args, Arguments, usage_handler};

use std::io;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() {
    let args_type = get_args();

    let _ = usage_handler();

    // match args_type {
    //     Arguments::Help => usage_handler("Help"),
    //     Arguments::Version => usage_handler("Version"),
    //     Arguments::Empty => usage_handler("Empty"),
    //     Arguments::Path => usage_handler("Path"),
    //     _ => usage_handler("Default")
    // }
}

// use std::{
//     error::Error,
//     io,
//     time::{Duration, Instant},
// };

// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use ratatui::{prelude::*, widgets::*};

// struct App {
//     scroll: u16,
// }

// impl App {
//     fn new() -> App {
//         App { scroll: 0 }
//     }

//     fn on_tick(&mut self) {
//         self.scroll += 1;
//         self.scroll %= 10;
//     }
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     // create app and run it
//     let tick_rate = Duration::from_millis(250);
//     let app = App::new();
//     let res = run_app(&mut terminal, app, tick_rate);

//     // restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     if let Err(err) = res {
//         println!("{err:?}");
//     }

//     Ok(())
// }

// fn run_app<B: Backend>(
//     terminal: &mut Terminal<B>,
//     mut app: App,
//     tick_rate: Duration,
// ) -> io::Result<()> {
//     let mut last_tick = Instant::now();
//     loop {
//         terminal.draw(|f| ui(f, &app))?;

//         let timeout = tick_rate
//             .checked_sub(last_tick.elapsed())
//             .unwrap_or_else(|| Duration::from_secs(0));
//         if crossterm::event::poll(timeout)? {
//             if let Event::Key(key) = event::read()? {
//                 if let KeyCode::Char('q') = key.code {
//                     return Ok(());
//                 }
//             }
//         }

//     }
// }

// fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
//     let size = f.size();

//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints(
//             [
//                 Constraint::Percentage(1),
//                 Constraint::Percentage(1),
//                 Constraint::Percentage(1),
//                 Constraint::Percentage(50),
//             ]
//             .as_ref(),
//         )
//         .split(size);

//     let text = vec![
//         Line::from("This is a line   ".red()),
//         Line::from("This is a line".on_blue()),
//     ];

    
//     let title = "Waveplot(press 'q' to quit)";

//     let paragraph = Paragraph::new(text)
//         .style(Style::default().fg(Color::Black).bg(Color::Black))
//         .block(  Block::default()
//         .borders(Borders::ALL)
//         .style(Style::default().fg(Color::Blue))
//         .title(Span::styled(
//             title,
//             Style::default().add_modifier(Modifier::BOLD),
//         )))
//         .alignment(Alignment::Left)
//         .wrap(Wrap { trim: true });
//     f.render_widget(paragraph, chunks[3]);
// }