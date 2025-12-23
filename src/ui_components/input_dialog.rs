use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::theme::Theme;

/// Renders an input dialog with prompt and text input
pub fn render_input_dialog(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    prompt: &str,
    input: &str,
    cursor_position: usize,
    is_error: bool,
) {
    // Create centered area for dialog
    let dialog_width = area.width.min(80);
    let dialog_height = area.height.min(20);

    let centered = centered_rect(dialog_width, dialog_height, area);

    // Create layout for prompt and input
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(centered);

    // Render prompt
    let prompt_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(if is_error {
            Theme::error()
        } else {
            Theme::border()
        });

    let prompt_text = Paragraph::new(prompt)
        .block(prompt_block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .style(if is_error {
            Theme::error()
        } else {
            Theme::text()
        });

    frame.render_widget(prompt_text, chunks[0]);

    // Render input field with cursor
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::input_focused());

    // Create input line with cursor
    let mut spans = vec![];
    for (i, c) in input.chars().enumerate() {
        if i == cursor_position {
            spans.push(Span::styled(c.to_string(), Theme::cursor()));
        } else {
            spans.push(Span::styled(c.to_string(), Theme::input()));
        }
    }

    // Add cursor at end if position is at/past end
    if cursor_position >= input.len() {
        spans.push(Span::styled(" ", Theme::cursor()));
    }

    let input_line = Line::from(spans);
    let input_paragraph = Paragraph::new(input_line)
        .block(input_block)
        .style(Theme::input());

    frame.render_widget(input_paragraph, chunks[1]);
}

/// Renders a message dialog (for success/error/confirmation)
pub fn render_message_dialog(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    message: &str,
    is_error: bool,
) {
    // Create centered area for dialog
    let dialog_width = area.width.min(80);
    let dialog_height = area.height.min(20);

    let centered = centered_rect(dialog_width, dialog_height, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(if is_error {
            Theme::error()
        } else {
            Theme::success()
        });

    let text = Paragraph::new(message)
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .style(if is_error {
            Theme::text()
        } else {
            Theme::text()
        });

    frame.render_widget(text, centered);
}

/// Helper to create a centered rectangle
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let horizontal_margin = area.width.saturating_sub(width) / 2;
    let vertical_margin = area.height.saturating_sub(height) / 2;

    Rect {
        x: area.x + horizontal_margin,
        y: area.y + vertical_margin,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
