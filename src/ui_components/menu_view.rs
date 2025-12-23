use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::{logo::Logo, theme::Theme};
use crate::menu::{MenuItem, MenuState};

/// Renders the main menu view
pub fn render_menu(frame: &mut Frame, area: Rect, menu_state: &MenuState) {
    // Create main layout: Logo, Menu, Help
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(Logo::height() + 2), // Logo + spacing
            Constraint::Min(10),                      // Menu
            Constraint::Length(3),                    // Help text
        ])
        .split(area);

    // Render logo
    render_logo(frame, chunks[0]);

    // Render menu items
    render_menu_items(frame, chunks[1], menu_state);

    // Render help text
    render_help(frame, chunks[2]);
}

/// Renders the logo at the top
fn render_logo(frame: &mut Frame, area: Rect) {
    let logo_lines: Vec<Line> = Logo::get()
        .iter()
        .map(|line| Line::from(Span::styled(*line, Theme::logo())))
        .collect();

    let tagline = Line::from(vec![
        Span::styled("─── ", Theme::dim()),
        Span::styled(Logo::tagline(), Theme::secondary()),
        Span::styled(" ───", Theme::dim()),
    ]);

    let mut all_lines = logo_lines;
    all_lines.push(Line::from(""));
    all_lines.push(tagline);

    let logo_paragraph = Paragraph::new(all_lines).alignment(Alignment::Center);

    frame.render_widget(logo_paragraph, area);
}

/// Renders the menu items
fn render_menu_items(frame: &mut Frame, area: Rect, menu_state: &MenuState) {
    let items = MenuItem::all();
    let selected = menu_state.selected();

    let menu_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = i == selected;
            let is_active = item.is_active();

            let prefix = if is_selected { "▸ " } else { "  " };

            let style = if is_selected {
                Theme::menu_item_selected()
            } else if is_active {
                Theme::menu_item_active()
            } else {
                Theme::menu_item_placeholder()
            };

            let description = if !is_active {
                " (Coming soon)"
            } else {
                ""
            };

            let content = format!("{}{}{}", prefix, item.name(), description);

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .title("Main Menu")
        .title_style(Theme::title())
        .border_style(Theme::border());

    let menu_list = List::new(menu_items).block(menu_block);

    frame.render_widget(menu_list, area);
}

/// Renders the help text at the bottom
fn render_help(frame: &mut Frame, area: Rect) {
    let help_text = Line::from(vec![
        Span::styled("↑/↓", Theme::accent()),
        Span::styled(" or ", Theme::help()),
        Span::styled("j/k", Theme::accent()),
        Span::styled(": Navigate  ", Theme::help()),
        Span::styled("Enter", Theme::accent()),
        Span::styled(": Select  ", Theme::help()),
        Span::styled("q", Theme::accent()),
        Span::styled(": Quit", Theme::help()),
    ]);

    let help_paragraph = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    frame.render_widget(help_paragraph, area);
}
