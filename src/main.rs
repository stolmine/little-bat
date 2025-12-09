use battery::{units::ratio::percent, Manager, State};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};
use std::time::Duration;

#[derive(Parser)]
#[command(name = "little-bat", about = "A minimal TUI battery status display")]
struct Args {
    /// Show ASCII battery graphic instead of just percentage
    #[arg(short, long)]
    graphic: bool,

    /// Show label text (e.g., "Battery:", charging status)
    #[arg(short, long)]
    label: bool,
}

struct App {
    manager: Manager,
    graphic_mode: bool,
    show_label: bool,
}

impl App {
    fn new(graphic_mode: bool, show_label: bool) -> Result<Self, battery::Error> {
        Ok(Self {
            manager: Manager::new()?,
            graphic_mode,
            show_label,
        })
    }

    fn get_battery_info(&self) -> Option<(f32, State)> {
        self.manager
            .batteries()
            .ok()?
            .next()?
            .ok()
            .map(|b| (b.state_of_charge().get::<percent>(), b.state()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let terminal = ratatui::init();
    let app = App::new(args.graphic, args.label)?;
    let result = run(terminal, app);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app: App) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| render(frame, &app))?;

        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let content = match app.get_battery_info() {
        Some((charge, state)) => {
            if app.graphic_mode {
                render_graphic(charge, state, app.show_label)
            } else {
                render_percentage(charge, state, app.show_label)
            }
        }
        None => vec![Line::from("No battery found")],
    };

    let height = content.len() as u16;
    let width = content.iter().map(|l| l.width()).max().unwrap_or(20) as u16;
    let centered = centered_rect(area, width + 2, height);

    let widget = Paragraph::new(content).centered();
    frame.render_widget(widget, centered);
}

fn render_percentage(charge: f32, state: State, show_label: bool) -> Vec<Line<'static>> {
    let color = charge_color(charge);
    let mut lines = Vec::new();

    if show_label {
        lines.push(Line::from("Battery"));
    }

    lines.push(Line::from(Span::styled(
        format!("{:.0}%", charge),
        Style::default().fg(color),
    )));

    if show_label {
        lines.push(Line::from(Span::styled(
            state_text(state),
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines
}

fn render_graphic(charge: f32, state: State, show_label: bool) -> Vec<Line<'static>> {
    let color = charge_color(charge);
    let filled = (charge / 10.0).round() as usize;
    let empty = 10 - filled;

    let bar = format!(
        "[{}{}]",
        "█".repeat(filled),
        "░".repeat(empty)
    );

    let mut lines = Vec::new();

    if show_label {
        lines.push(Line::from("Battery"));
    }

    lines.push(Line::from(Span::styled(bar, Style::default().fg(color))));

    lines.push(Line::from(Span::styled(
        format!("{:.0}%", charge),
        Style::default().fg(color),
    )));

    if show_label {
        lines.push(Line::from(Span::styled(
            state_text(state),
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines
}

fn charge_color(charge: f32) -> Color {
    if charge > 50.0 {
        Color::Green
    } else if charge > 20.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn state_text(state: State) -> String {
    match state {
        State::Charging => "Charging".to_string(),
        State::Discharging => "Discharging".to_string(),
        State::Full => "Full".to_string(),
        State::Empty => "Empty".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
