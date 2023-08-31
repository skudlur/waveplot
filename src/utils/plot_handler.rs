use vcd::{Command, Parser, ScopeItem, Value};

use std::{collections::HashMap, env, error::Error, fs, fs::File, io, io::BufReader, rc::Rc, vec};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, widgets::*};

struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub state: TableState,
    pub items_length: usize,
    pub scroll_parser_tab: u16,
    pub scroll_vcd_tab: u16,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["Plot", "Parser", "Header", "VCD Code"],
            index: 0,
            state: TableState::default(),
            items_length: 0,
            scroll_parser_tab: 0,
            scroll_vcd_tab: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn next_header_tab(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items_length - 1 {
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
                    self.items_length - 1
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

    pub fn scroll_parser_down(&mut self) {
        self.scroll_parser_tab += 1;
    }

    pub fn scroll_parser_up(&mut self) {
        if self.scroll_parser_tab > 0 {
            self.scroll_parser_tab -= 1;
        }
    }

    pub fn scroll_vcd_down(&mut self) {
        self.scroll_vcd_tab += 1;
    }

    pub fn scroll_vcd_up(&mut self) {
        if self.scroll_vcd_tab > 0 {
            self.scroll_vcd_tab -= 1;
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

// takes control of terminal and key events
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // Handle keyboard events
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                } else if key.code == KeyCode::Char('d') || key.code == KeyCode::Right {
                    app.next();
                } else if key.code == KeyCode::Char('a') || key.code == KeyCode::Left {
                    app.previous();
                }

                // VCD Code Tab (index 3)
                if app.index == 3 {
                    if key.code == KeyCode::Char('w') {
                        app.scroll_vcd_up();
                    } else if key.code == KeyCode::Char('s') {
                        app.scroll_vcd_down();
                    } else if key.code == KeyCode::Up {
                        app.scroll_vcd_up();
                    } else if key.code == KeyCode::Down {
                        app.scroll_vcd_down();
                    }
                } else if app.index == 2 {
                    // Header Tab (index 2)
                    if key.code == KeyCode::Char('w') {
                        app.previous_header_tab()
                    } else if key.code == KeyCode::Char('s') {
                        app.next_header_tab();
                    } else if key.code == KeyCode::Up {
                        app.previous_header_tab();
                    } else if key.code == KeyCode::Down {
                        app.next_header_tab();
                    }
                } else if app.index == 1 {
                    // Parser Tab (index 1)
                    if key.code == KeyCode::Char('w') {
                        app.scroll_parser_up()
                    } else if key.code == KeyCode::Char('s') {
                        app.scroll_parser_down();
                    } else if key.code == KeyCode::Up {
                        app.scroll_parser_up()
                    } else if key.code == KeyCode::Down {
                        app.scroll_parser_down();
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let file_path = get_path();

    let file = File::open(file_path.clone()).unwrap();

    let mut parser = Parser::new(BufReader::new(file));

    let header = parser.parse_header().unwrap();

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
    let mut variable_codes = Vec::new();
    let mut variable_values = HashMap::new();
    let mut variable_value_types = HashMap::new();
    let mut variable_time_stamps = Vec::new();
    let mut variable_graph_coordinates = HashMap::new();

    let scope = header.items.iter().find_map(|f| {
        if let ScopeItem::Scope(scope) = f {
            Some(scope.clone())
        } else {
            None
        }
    });

    scope.clone().unwrap().items.iter().for_each(|x| match x {
        ScopeItem::Var(v) => {
            variable_types.push(v.var_type.to_string());
            variable_sizes.push(v.size.to_string());
            variable_references.push(v.reference.clone());
            variable_indexes_ref.push(v.index);
            variable_codes.push(v.code.to_string());
            variable_values.insert(v.code.to_string(), Vec::<String>::new());
        }
        _ => {}
    });

    variable_indexes_ref.iter().for_each(|x| {
        if x.is_some() {
            variable_indexes.push(x.unwrap().to_string());
        } else {
            variable_indexes.push("None".to_string());
        }
    });

    let mut parse_line_by_line = Vec::new();

    parser.for_each(|f| {
        match f.unwrap() {
            Command::Begin(id) => {
                parse_line_by_line.push(format!("Begin: {:?}", id));
            }
            Command::ChangeReal(id, value) => {
                parse_line_by_line.push(format!("{:?} changed to {:?}", id.to_string(), value));
            }
            Command::ChangeString(id, value) => {
                parse_line_by_line.push(format!("{:?} changed to {:?}", id.to_string(), value));
            }
            Command::Comment(comment) => {
                parse_line_by_line.push(format!("Comment: {:?}", comment));
            }
            Command::Date(date) => {
                parse_line_by_line.push(format!("Date: {:?}", date));
            }
            Command::End(id) => {
                parse_line_by_line.push(format!("End: {:?}", id));
            }
            Command::ChangeVector(id, value) => {
                parse_line_by_line.push(format!(
                    "{:?} changed to {:?}",
                    id.to_string(),
                    value.to_string()
                ));
            }
            Command::Enddefinitions => {
                parse_line_by_line.push(format!("End Definitions"));
            }
            Command::ScopeDef(scope_type, name) => {
                parse_line_by_line.push(format!("Scope Def: {:?} {:?}", scope_type, name));
            }
            Command::Timescale(time, unit) => {
                parse_line_by_line.push(format!("Timescale: {:?} {:?}", time, unit));
            }
            Command::Version(version) => {
                parse_line_by_line.push(format!("Version: {:?}", version));
            }
            Command::Upscope => {
                parse_line_by_line.push(format!("Upscope"));
            }
            Command::ChangeScalar(id, value) => {
                parse_line_by_line.push(format!("{:?} changed to {:?}", id.to_string(), value));
                if value == Value::V0 {
                    variable_value_types.insert(id.to_string(), "0".to_string());
                } else if value == Value::V1 {
                    variable_value_types.insert(id.to_string(), "1".to_string());
                } else if value == Value::X {
                    variable_value_types.insert(id.to_string(), "X".to_string());
                } else if value == Value::Z {
                    variable_value_types.insert(id.to_string(), "Z".to_string());
                } else {
                    variable_value_types.insert(id.to_string(), "U".to_string());
                }
                variable_values
                    .get_mut(&id.to_string())
                    .unwrap()
                    .push(value.to_string());
            }
            Command::Timestamp(time) => {
                parse_line_by_line.push(format!("Timestamp: {:?}", time));
                variable_time_stamps.push(time);
            }
            _ => {
                // println!("Something else");
            }
        }
    });

    // Check if the variable is a 0 or 1 and if it is, add it to the variable_graph_coordinates
    // And fill its values with respect to timestamps tuple of (timestamp, value)
    variable_value_types.iter().for_each(|v| {
        if v.1 == "0" || v.1 == "1" {
            // fetch its values from variable_values
            variable_graph_coordinates.insert(v.0, Vec::<(u64, u64)>::new());

            for (index, element) in variable_values.get(v.0).unwrap().iter().enumerate() {
                if element == "0" || element == "1" {
                    variable_graph_coordinates.get_mut(v.0).unwrap().push((
                        variable_time_stamps[index],
                        element.to_string().parse::<u64>().unwrap(),
                    ));
                }
            }
        }
    });

    // Making sure that values are filled for every timestamp
    // If not, fill it with the last value
    // Ex: (0, 1), (1, 1) will be converted to (0, 1), (1, 1), (2, 1) (3,1) etc. [3 is the last timestamp]
    variable_graph_coordinates.iter_mut().for_each(|v| {
        if variable_time_stamps.len() > v.1.len() {
            for index in v.1.len()..variable_time_stamps.len() {
                v.1.push((variable_time_stamps[index], v.1[v.1.len() - 1].1));
            }
        }
    });

    let size = f.size();

    // Make 2 chunks, one for the tabs and one for the content
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
            title.clone() + " (use 'a' and 'd' or left and right arrows to change tabs)",
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

    if app.index == 0 {
        // Plot Tab (index 0)
        let number_of_graphs = variable_graph_coordinates.len();

        let mut variable_graphs_converted_coordinates = Vec::new();

        // Convert the coordinates to f64 since the chart component only accepts f64
        variable_graph_coordinates.iter().for_each(|(key, value)| {
            let converted_data: Vec<(f64, f64)> =
                value.iter().map(|(a, b)| (*a as f64, *b as f64)).collect();

            variable_graphs_converted_coordinates.push(converted_data);
        });

        // Make 3 columns
        let outer_layout_constraints = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let mut inner_chunks_constraint = Vec::new();

        let multiple_of_three = number_of_graphs / 3;

        // If the number of graphs is less than 3, then make it 100% of the screen
        // If the number of graphs >2 <6, then make it 50% of the screen
        // If the number of graphs >5 <9, use the same logic as above
        match multiple_of_three {
            0 => inner_chunks_constraint.push(Constraint::Percentage(100)),
            1 => {
                inner_chunks_constraint.push(Constraint::Percentage(50));
                inner_chunks_constraint.push(Constraint::Percentage(50));
            }
            _ => {
                for _ in 1..multiple_of_three {
                    inner_chunks_constraint
                        .push(Constraint::Percentage(100 / multiple_of_three as u16));
                }
            }
        }

        // Three columns
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(inner_chunks_constraint.as_ref())
            .split(outer_layout_constraints[0]);

        let middle_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(inner_chunks_constraint.as_ref())
            .split(outer_layout_constraints[1]);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(inner_chunks_constraint.as_ref())
            .split(outer_layout_constraints[2]);

        for (index, value) in variable_graphs_converted_coordinates.iter().enumerate() {
            let datasets_one = vec![Dataset::default()
                .name("data")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(&variable_graphs_converted_coordinates[index])];

            let chart_one = Chart::new(datasets_one)
                .block(
                    Block::default()
                        .title(Span::styled(
                            variable_graph_coordinates
                                .keys()
                                .nth(index)
                                .unwrap()
                                .to_string(),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::ITALIC),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("Time Stamps")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([
                            variable_graphs_converted_coordinates[index][0].0,
                            variable_graphs_converted_coordinates[index]
                                [variable_graphs_converted_coordinates[index].len() - 1]
                                .0,
                        ])
                        .labels(vec![
                            Span::styled(
                                variable_graphs_converted_coordinates[index][0]
                                    .0
                                    .to_string(),
                                Style::default().bold(),
                            ),
                            Span::styled(
                                variable_graphs_converted_coordinates[index]
                                    [variable_graphs_converted_coordinates[index].len() - 1]
                                    .0
                                    .to_string(),
                                Style::default().bold(),
                            ),
                        ]),
                )
                .y_axis(
                    Axis::default()
                        .title(Span::styled(
                            variable_graph_coordinates
                                .keys()
                                .nth(index)
                                .unwrap()
                                .to_string(),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::ITALIC),
                        ))
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 1.0])
                        .labels(vec!["0".bold(), "1".bold()]),
                );

            let chunk_index = index / 3;
            let chunk_offset = index % 3;

            // Place the graphs in the respective chunks
            let target_chunk = match chunk_offset {
                0 => left_chunks.get(chunk_index),
                1 => middle_chunks.get(chunk_index),
                2 => right_chunks.get(chunk_index),
                _ => None,
            };

            if let Some(chunk) = target_chunk {
                f.render_widget(chart_one, *chunk);
            }
        }
    } else if app.index == 2 {
        // Header Tab (index 2)
        let inside_chunk = Layout::default()
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(chunks[1]);

        let horizontal_chunks_inside_chunk_zero = Layout::default()
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

        let header_scope_type = scope.clone().unwrap().scope_type.to_string();
        let header_scope_identifier = scope.clone().unwrap().identifier.to_string();

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

        f.render_widget(header_version_block, horizontal_chunks_inside_chunk_zero[0]);
        f.render_widget(header_date_block, horizontal_chunks_inside_chunk_zero[1]);
        f.render_widget(
            header_timescale_block,
            horizontal_chunks_inside_chunk_zero[2],
        );

        let header_scope_title = format!("{} {}", header_scope_type, header_scope_identifier);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);

        let header_cells = ["Type", "Size", "Reference", "Index", "Code"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));

        let header = Row::new(header_cells);

        let mut row_data = vec![vec![".", ".", ".", ".", "."]];

        let loop_length = variable_types.len();

        app.items_length = loop_length + 1;

        // Make table component acceptable data
        for i in 0..loop_length {
            row_data.push(vec![
                &variable_types[i],
                &variable_sizes[i],
                &variable_references[i],
                &variable_indexes[i],
                &variable_codes[i],
            ])
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

        let header_scope_block = Table::new(rows)
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
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]);

        f.render_stateful_widget(header_scope_block, inside_chunk[1], &mut app.state);
    } else if app.index == 3 {
        // VCD Code Tab (index 3)
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
            .scroll((app.scroll_vcd_tab, 0))
            .wrap(Wrap { trim: true });

        f.render_widget(vcd_code_tab, chunks[1]);
    } else {
        // Parser Tab (index 1)
        let mut parser_content = Vec::new();

        parse_line_by_line.iter().for_each(|f| {
            parser_content.push(Line::from(f.to_string()));
        });

        let parser_block = Paragraph::new(parser_content)
            .style(Style::default().fg(Color::Gray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Blue))
                    .title(Span::styled(
                        "Parser (use 'w' and 's' or up and down arrow keys to scroll)",
                        Style::default().add_modifier(Modifier::BOLD),
                    )),
            )
            .alignment(Alignment::Left)
            .scroll((app.scroll_parser_tab, 0))
            .wrap(Wrap { trim: true });

        f.render_widget(parser_block, chunks[1]);
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
