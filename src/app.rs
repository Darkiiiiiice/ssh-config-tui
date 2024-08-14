use std::io;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect, Layout, Direction, Flex, Constraint};
use ratatui::prelude::{Color, Line, Modifier, Style, Stylize, Text, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, List, ListDirection, Paragraph};
use ratatui::widgets::block::{Position, Title};
use crate::tui::TUI;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut TUI) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handler_events()?;
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(100),
            ])
            .flex(Flex::Center)
            .split(frame.area());


        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ])
            .vertical_margin(1)
            .horizontal_margin(1)
            .flex(Flex::Start)
            .split(outer_layout[0]);

        let outer_title = Title::from("  SSH Config  ".green().bold());
        let outer_block = Block::bordered()
            .title(outer_title.alignment(Alignment::Center).position(Position::Top))
            .border_set(border::ROUNDED);

        let inner_left_title = Title::from(" Host ".green().bold());
        let inner_left_block = Block::bordered()
            .title(
                inner_left_title
                .alignment(Alignment::Center)
                    .position(Position::Top)
            )
            .border_set(border::ROUNDED);
        let inner_right_title = Title::from(" Editor ".green().bold());
        let inner_right_block = Block::bordered()
            .title(
                inner_right_title
                    .alignment(Alignment::Right)
                    .position(Position::Top)
            )
            .border_set(border::ROUNDED);


        let title = Title::from(" SSH Config ".green().bold());
        let instractions = Title::from(Line::from(vec![
            "Decrement".into(),
            "<Left>".blue().bold(),
            "Increment".into(),
            "<Right>".yellow().bold(),
            "Quit".into(),
            "<Q>".red().bold(),
        ]));

        frame.render_widget(
            Paragraph::new("outer 0")
                .centered()
                .block(outer_block)
            ,
            outer_layout[0]
        );

        let items = ["Item 1", "Item 2", "Item 3"];
        let list = List::new(items)
            .block(inner_left_block)
            .style(Style::default().fg(Color::Blue))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
        frame.render_widget(
            list,
            inner_layout[0]
        );
        frame.render_widget(
            Paragraph::new("inner 1")
                .centered()
                .block(inner_right_block),
            inner_layout[1]
        );
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
            _ => {}
        }
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
