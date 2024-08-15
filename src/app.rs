use crate::tui::TUI;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Direction, Flex, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Style, Stylize, Widget};
use ratatui::symbols::border;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{
    Block, List, ListDirection, ListItem, ListState, Paragraph, StatefulWidget,
};
use ratatui::Frame;
use std::io;

#[derive(Debug, Default)]
struct HostItem<'a> {
    name: &'a str,
    domain: &'a str,
}

impl<'a> HostItem<'a> {
    fn new(name: &'a str, domain: &'a str) -> Self {
        Self { name, domain }
    }

    fn to_list_item(&self, index: usize) -> ListItem {
        ListItem::new(Line::styled(format!("{}", self.name), Color::Red))
    }
}

impl<'a> From<&(&'a str, &'a str)> for HostItem<'a> {
    fn from((name, domain): &(&'a str, &'a str)) -> Self {
        Self { name, domain }
    }
}

#[derive(Debug, Default)]
pub struct App<'a> {
    counter: u8,
    state: ListState,
    hosts: Vec<HostItem<'a>>,
    last_selected: Option<usize>,
    exit: bool,
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut TUI) -> io::Result<()> {
        let items = vec![
            HostItem::new("Github", "github.com"),
            HostItem::new("DevCloud", "github.com"),
            HostItem::new("Staging", "github.com"),
            HostItem::new("QaCloud", "github.com"),
            HostItem::new("TestBM", "github.com"),
            HostItem::new("Github", "github.com"),
            HostItem::new("Github", "github.com"),
            HostItem::new("Github", "github.com"),
            HostItem::new("Github", "github.com"),
        ];

        self.hosts.extend(items);
        self.state.select_first();

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handler_events()?;
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handler_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            KeyCode::Char('q') => self.exit(),
            // KeyCode::Char('h') | Left => self.items.unselect(),
            KeyCode::Char('j') => self.next(),
            KeyCode::Char('k') => self.prev(),
            // KeyCode::Char('l') | Right | Enter => self.change_status(),
            KeyCode::Char('g') => self.go_top(),
            KeyCode::Char('G') => self.go_bottom(),
            _ => {}
        }
    }

    fn go_top(&mut self) {
        self.state.select_first()
    }

    fn go_bottom(&mut self) {
        self.state.select_last()
    }

    fn next(&mut self) {

        match self.state.selected() {
            Some(i) => {
                if i >= self.hosts.len() - 1 {
                    self.state.select_first();
                } else {
                    self.state.select_next();
                }
            }
            None => self.state.select_first(),
        };
    }

    fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i <= 0 {
                    self.hosts.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.hosts.len() - 1,
        };
        self.state.select(Some(i))

    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized,
    {
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(100)])
            .flex(Flex::Center)
            .split(area);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .vertical_margin(1)
            .horizontal_margin(1)
            .flex(Flex::Start)
            .split(outer_layout[0]);

        self.render_container(outer_layout[0], buf);
        self.render_left_container(inner_layout[0], buf);
        self.render_right_container(inner_layout[1], buf);
    }
}

impl App<'_> {
    fn render_left_container(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Host ".green().bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center).position(Position::Top))
            .border_set(border::ROUNDED);

        let items: Vec<ListItem> = self
            .hosts
            .iter()
            .enumerate()
            .map(|(i, item)| item.to_list_item(i))
            .collect();

        let list = List::new(items)
            .block(block)
            .style(Style::default().fg(Color::Blue))
            .highlight_style(Style::default().add_modifier(Modifier::SLOW_BLINK))
            .highlight_symbol(">>> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    fn render_right_container(&self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Editor ".green().bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Right).position(Position::Top))
            .border_set(border::ROUNDED);

        Paragraph::new("inner 1")
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_container(&self, area: Rect, buf: &mut Buffer) {
        let outer_title = Title::from("  SSH Config  ".green().bold());
        let outer_block = Block::bordered()
            .title(
                outer_title
                    .alignment(Alignment::Center)
                    .position(Position::Top),
            )
            .border_set(border::ROUNDED);

        Paragraph::new("outer 0")
            .centered()
            .block(outer_block)
            .render(area, buf);
    }
}
