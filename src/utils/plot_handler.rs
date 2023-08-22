use vcd::{Command, Parser, ScopeItem};

use std::{env, error::Error, fs, fs::File, io, io::BufReader, vec};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, widgets::*};

struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub scroll: u16,
    pub state: TableState,
    pub items: Vec<Vec<&'a str>>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["Plot", "Header", "VCD Code", "Tab3"],
            index: 0,
            scroll: 0,
            state: TableState::default(),
            items: vec![
                vec!["Row11", "Row12", "Row13"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row31", "Row32", "Row33"],
                vec!["Row41", "Row42", "Row43"],
                vec!["Row51", "Row52", "Row53"],
                vec!["Row61", "Row62\nTest", "Row63"],
                vec!["Row71", "Row72", "Row73"],
                vec!["Row81", "Row82", "Row83"],
                vec!["Row91", "Row92", "Row93"],
                vec!["Row101", "Row102", "Row103"],
                vec!["Row111", "Row112", "Row113"],
                vec!["Row121", "Row122", "Row123"],
                vec!["Row131", "Row132", "Row133"],
                vec!["Row141", "Row142", "Row143"],
                vec!["Row151", "Row152", "Row153"],
                vec!["Row161", "Row162", "Row163"],
                vec!["Row171", "Row172", "Row173"],
                vec!["Row181", "Row182", "Row183"],
                vec!["Row191", "Row192", "Row193"],
            ],
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn next_header_tab(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_header_tab(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub fn plot_handler() -> Result<(), Box<dyn Error>> {
    // scroll handler for header and vcd code tabs
    let mut scroll_header_tab: u16 = 0;
    let mut scroll_vcd_code_tab: u16 = 0;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app, scroll_header_tab, scroll_vcd_code_tab);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    mut scroll_header_tab: u16,
    mut scroll_vcd_code_tab: u16,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app, scroll_header_tab, scroll_vcd_code_tab))?;

        // Handle keyboard events
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                // match key.code {
                //     KeyCode::Char('w') => app.scroll_down(),
                //     KeyCode::Char('s') => app.scroll_up(),
                //     KeyCode::Up => app.scroll_down(),
                //     KeyCode::Down => app.scroll_up(),
                //     _ => {}
                // }
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                } else if key.code == KeyCode::Char('d') || key.code == KeyCode::Right {
                    app.next();
                } else if key.code == KeyCode::Char('a') || key.code == KeyCode::Left {
                    app.previous();
                }

                if app.index == 2 {
                    if key.code == KeyCode::Char('w') {
                        if scroll_vcd_code_tab > 0 {
                            scroll_vcd_code_tab -= 1;
                        }
                    } else if key.code == KeyCode::Char('s') {
                        scroll_vcd_code_tab += 1;
                    } else if key.code == KeyCode::Up {
                        if scroll_vcd_code_tab > 0 {
                            scroll_vcd_code_tab -= 1;
                        }
                    } else if key.code == KeyCode::Down {
                        scroll_vcd_code_tab += 1;
                    }
                } else if app.index == 1 {
                    if key.code == KeyCode::Char('w') {
                        app.previous_header_tab()
                    } else if key.code == KeyCode::Char('s') {
                        app.next_header_tab();
                    } else if key.code == KeyCode::Up {
                        app.previous_header_tab();
                    } else if key.code == KeyCode::Down {
                        app.next_header_tab();
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, scroll_header_tab: u16, scroll_vcd_code_tab: u16) {
    let file_path = get_path();

    let file = File::open(file_path.clone()).unwrap();

    let mut parser = Parser::new(BufReader::new(file));

    let header = parser.parse_header().unwrap();

    // Extract the file content into a vector of strings
    let file_content = match fs::read_to_string(file_path.clone()) {
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
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(size);

    let block = Block::default();
    f.render_widget(block, size);

    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![first.cyan().bold(), rest.green()])
        })
        .collect();

    let title = format!(
        "\t 🌊 Waveplot v{} (press 'q' to exit) ",
        env!("CARGO_PKG_VERSION")
    );

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(Span::styled(
            title.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        )))
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
                    "Code (use 'w' and 's' or up and down arrow keys to scroll)",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
        )
        .alignment(Alignment::Left)
        .scroll((scroll_vcd_code_tab, 0))
        .wrap(Wrap { trim: true });

    let inside_chunk = Layout::default()
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);

    let horizontal_chunks_inside_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(inside_chunk[0]);

    if app.index == 1 {
        let header_version = header.version.unwrap();
        let header_date = header.date.unwrap();
        let header_timescale = header.timescale.unwrap().0.to_string();
        let header_timescale_unit = header.timescale.unwrap().1.to_string();

        let mut variable_types = Vec::new();
        let mut variable_sizes = Vec::new();
        let mut variable_references = Vec::new();
        // for temporarily holding variable indexes
        let mut variable_indexes_ref = Vec::new();
        let mut variable_indexes = Vec::new();

        let scope = match &header.items[0] {
            ScopeItem::Scope(sc) => sc,
            x => panic!("Expected Scope, found {:?}", x),
        };
        scope.items.iter().for_each(|x| match x {
            ScopeItem::Var(v) => {
                variable_types.push(v.var_type.to_string());
                variable_sizes.push(v.size.to_string());
                variable_references.push(v.reference.clone());
                variable_indexes_ref.push(v.index);
            }
            x => panic!("Expected Var, found {:?}", x),
        });

        variable_indexes_ref.iter().for_each(|x| {
            if x.is_some() {
                variable_indexes.push(x.unwrap().to_string());
            } else {
                variable_indexes.push("None".to_string());
            }
        });

        let header_scope_type = scope.scope_type.to_string();
        let header_scope_identifier = scope.identifier.to_string();

        let header_version_block = Paragraph::new(vec![Line::from(vec![Span::styled(
            header_version,
            Style::default().fg(Color::LightCyan),
        )])])
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Blue))
                .title(Span::styled(
                    "Version",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

        let header_date_block = Paragraph::new(vec![Line::from(vec![Span::styled(
            header_date,
            Style::default().fg(Color::LightCyan),
        )])])
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Blue))
                .title(Span::styled(
                    "Date",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

        let header_timescale_block = Paragraph::new(vec![Line::from(vec![
            Span::styled(header_timescale, Style::default().fg(Color::LightCyan)),
            Span::styled(" ", Style::default().fg(Color::LightCyan)),
            Span::styled(
                header_timescale_unit,
                Style::default().fg(Color::LightGreen),
            ),
        ])])
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Blue))
                .title(Span::styled(
                    "Timescale",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

        f.render_widget(header_version_block, horizontal_chunks_inside_chunk[0]);
        f.render_widget(header_date_block, horizontal_chunks_inside_chunk[1]);
        f.render_widget(header_timescale_block, horizontal_chunks_inside_chunk[2]);

        let header_scope_title = format!("{} {}", header_scope_type, header_scope_identifier);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);

        let header_cells = ["Type", "Size", "Reference", "Index"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
        
        let header = Row::new(header_cells);

        let mut row_data = vec![
            vec![".", ".", ".", "."]
        ];
        
        let loopLength = variable_types.len();
        
        for i in 0..loopLength {
            row_data.push(vec![&variable_types[i], &variable_sizes[i], &variable_references[i], &variable_indexes[i]])
        } 

        let rows = row_data.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(*c));
            Row::new(cells).height(height as u16)
        });

        let header_scope_block = Table::new(
            rows
        )
            .header(header)
            .style(Style::default().fg(Color::Gray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Blue))
                    .title(vec![
                        Span::styled("Scope:", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(" ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(
                            header_scope_title,
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            " (use 'w' and 's' or up and down arrow keys to scroll)",
                            Style::default(),
                        ),
                    ]),
            )
            .highlight_style(selected_style)
            .widths(&[
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]);

        f.render_stateful_widget(header_scope_block, inside_chunk[1], &mut app.state);
    } else if app.index == 2 {
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
