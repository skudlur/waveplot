use std::{
    env,
    error::Error,
    io,
    time::{Duration, Instant},
    rc::Rc
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

// No idea what this does
struct App {
    scroll: u16,
}

// No idea what this does
impl App {
    fn new() -> App {
        App { scroll: 0 }
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 10;
    }
}

// Enums representing the different types of arguments
pub enum Arguments {
    Empty,
    Version,
    Help,
    Path,
}

pub fn get_args() -> Arguments {
    // Get the arguments passed to the program
    let envs = env::args();
    let mut args = Vec::new();

    for (i, env) in envs.enumerate() {
        // Skip the first argument, which is the program name
        if i == 0 {
            continue;
        } else {
            args.push(env);
        }
    }

    // Returns the type of argument passed
    for arg in args {
        if arg == "-v" || arg == "--version" {
            return Arguments::Version;
        } else if arg == "-h" || arg == "--help" {
            return Arguments::Help;
        } else {
            return Arguments::Path;
        }
    }

    // If no arguments are passed, returns empty
    return Arguments::Empty;
}

// Get the path passed to the program
fn get_path() -> String {
    let envs = env::args();

    for (i, env) in envs.enumerate() {
        // Skip the first argument, which is the program name
        if i == 0 {
            continue;
        } else {
            return env.to_string();
        }
    }

    return String::from("");
}

// Handles Version , Help and Empty arguments
pub fn usage_handler() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())

    // match argument_type {
    //     "Help" => {
    //         text = "\nUsage: check [options] [path]\n\nOptions:\n\t-h, --help\t\tDisplay this message\n\t-v, --version\t\tDisplay version\n\nArguments:\n\tpath\t\t\tPath to the file or directory to be checked";
    //     }
    //     "Empty" => {
    //         text = "\nUsage: check [options] [path]\n\nOptions:\n\t-h, --help\t\tDisplay this message\n\t-v, --version\t\tDisplay version\n\nArguments:\n\tpath\t\t\tPath to the file or directory to be checked";
    //     }
    //     "Version" => {
    //         text = env!("CARGO_PKG_VERSION");
    //     }
    //    "Path" => {
    //     let path = get_path();
    //     println!("{}", path);
    //    }
    //     _ => {
    //         text = "\nUsage: check [options] [path]\n\nOptions:\n\t-h, --help\t\tDisplay this message\n\t-v, --version\t\tDisplay version\n\nArguments:\n\tpath\t\t\tPath to the file or directory to be checked";
    //     }
    // }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let arguments = get_args();

    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let helpText = vec![
        Line::from(""),
        Line::from("Usage:".bold().fg(Color::Gray)),
        Line::from(".      waveplot [PATH] ".fg(Color::LightCyan)),
        Line::from(".      waveplot [OPTIONS]".fg(Color::LightGreen)),
        Line::from(""),
        Line::from("PATH:".bold().fg(Color::Gray)),
        Line::from(".      Path to the vcd file".fg(Color::LightCyan)),
        Line::from(""),
        Line::from("OPTIONS:".bold().fg(Color::Gray)),
        Line::from(".      -h, --help ".fg(Color::LightGreen)),
        Line::from(".      -v, --version".fg(Color::LightGreen)),
    ];

    let version = format!(".                 v{}", env!("CARGO_PKG_VERSION"));

    let versionText = vec![
        Line::from(""),
        Line::from("Waveplot Version:".bold().fg(Color::Blue)),
        Line::from(version.fg(Color::Cyan)),
    ];

    let mut text = vec![];

    match arguments {
        Arguments::Help => {
            text = helpText;
            renderParagraph(text, f, chunks)
        }
        Arguments::Empty => {
            text = vec![
                Line::from(""),
                Line::from("Please enter the path to the vcd file".fg(Color::Red)),
            ];
            text.extend(helpText);
            renderParagraph(text, f, chunks)
        }
        Arguments::Version => {
            text = versionText;
            renderParagraph(text, f, chunks)
        }
        Arguments::Path => {
            text = vec![
                Line::from("Path Handler"),
                Line::from("This is a line   ".red()),
                Line::from("This is a line".on_blue()),
                Line::from("This is a longer line".crossed_out()),
                Line::from("This is a line".green().italic()),
            ];
        }
    }
}

fn renderParagraph<B: Backend> (text: Vec<Line<'_>>, f: &mut Frame<'_, B>, chunks: Rc<[Rect]>) {

    let title = format!(
        "\t 🌊 Waveplot v{} (press 'q' to exit) ",
        env!("CARGO_PKG_VERSION")
    );


    let paragraph = Paragraph::new(text)
    .style(Style::default().fg(Color::Gray))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Blue))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            )),
    )
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });
f.render_widget(paragraph, chunks[0]);
}
