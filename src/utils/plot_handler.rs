use vcd::{
    Parser,
    Command
};

use std::{
    error::Error, 
    io,
    io::BufReader,
    env,
    fs,
    fs::File 
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, widgets::*};

struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub scroll: u16
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["Plot", "Header", "VCD Code", "Tab3"],
            index: 0,
            scroll: 0
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll += 1;
    }

    pub fn scroll_down(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        } 
    }
}

pub fn plot_handler() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

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
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        // Handle keyboard events
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('d') => app.next(),
                    KeyCode::Char('a') => app.previous(),
                    KeyCode::Char('w') => app.scroll_down(),
                    KeyCode::Char('s') => app.scroll_up(),
                    KeyCode::Up => app.scroll_down(),
                    KeyCode::Down => app.scroll_up(),
                    KeyCode::Right => app.next(),
                    KeyCode::Left => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {

    let file_path = get_path();

    // Extract the file content into a vector of strings
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => String::from(""),
    };

    // Extract the VCD code into a renderable format
    let vcd_code_content = file_content
        .lines()
        .map(|l| Line::from(l))
        .collect::<Vec<_>>();


    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().black();
    f.render_widget(block, size);
    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![first.cyan().bold(),rest.green()])
        })
        .collect();

    let title = format!(
        "\t 🌊 Waveplot v{} (press 'q' to exit) ",
        env!("CARGO_PKG_VERSION")
    );

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL)
        .title(
            Span::styled(
                title.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            )
        ))
        .select(app.index)
        .style(Style::default().fg(Color::Blue))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::LightCyan),
        );
    f.render_widget(tabs, chunks[0]);

    let vcd_code_tab = Paragraph::new(vcd_code_content)
    .style(Style::default().fg(Color::Gray))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Blue))
            .title(Span::styled(
                "Code (use 'w' and 's' or up and down arrows to scroll)",
                Style::default().add_modifier(Modifier::BOLD),
            )),
    )
    .alignment(Alignment::Left)
    .scroll((app.scroll, 0))
    .wrap(Wrap { trim: true });

    // let inner = match app.index {
    //     0 => paragraph,
    //     1 => Block::default().title("Inner 1").borders(Borders::ALL),
    //     2 => Block::default().title("Inner 2").borders(Borders::ALL),
    //     3 => Block::default().title("Inner 3").borders(Borders::ALL),
    //     _ => unreachable!(),
    // };

    if app.index == 2 {
        f.render_widget(vcd_code_tab, chunks[1]);
    } else {
        let inner = Block::default().title("Inner 1").borders(Borders::ALL);
        f.render_widget(inner, chunks[1]);
    }
}

// Get the path passed to the program
pub fn get_path() -> String {
    let envs = env::args();

    for (i, env) in envs.enumerate() {
        // Skip the first argument, which is the program name (waveplot or ./waveplot) in this case
        if i == 0 {
            continue;
        } else {
            return env.to_string();
        }
    }

    return String::from("");
}
