use crate::{
    app::{ActiveFeature, App, AppState},
    features::{AliasManagerState, BookmarkManagerState, ProcessAction, ProcessManagerState, UsageViewerState},
    ui_components::{input_dialog, menu_view, theme::Theme},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Renders the UI based on application state
pub fn render(frame: &mut Frame, app: &App) {
    match &app.state {
        AppState::MainMenu => {
            menu_view::render_menu(frame, frame.area(), &app.menu_state);
        }
        AppState::FeatureActive(feature) => {
            render_feature(frame, feature);
        }
        AppState::Exiting => {
            // Could show exit message, but app will close immediately
        }
    }
}

/// Renders the active feature
fn render_feature(frame: &mut Frame, feature: &ActiveFeature) {
    match feature {
        ActiveFeature::AliasManager(manager) => {
            render_alias_manager(frame, manager);
        }
        ActiveFeature::ProcessManager(manager) => {
            render_process_manager(frame, manager);
        }
        ActiveFeature::BookmarkManager(manager) => {
            render_bookmark_manager(frame, manager);
        }
        ActiveFeature::UsageViewer(viewer) => {
            render_usage_viewer(frame, viewer);
        }
        ActiveFeature::Placeholder(placeholder) => {
            input_dialog::render_message_dialog(
                frame,
                frame.area(),
                "Feature Not Available",
                &placeholder.get_message(),
                false,
            );
        }
    }
}

/// Renders the alias manager based on its state
fn render_alias_manager(frame: &mut Frame, manager: &crate::features::AliasManager) {
    let area = frame.area();

    match &manager.state {
        AliasManagerState::EnteringName { .. } | AliasManagerState::EnteringCommand { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let prompt = manager.get_prompt();
            let input = manager.get_input();
            let cursor_pos = input.len();

            input_dialog::render_input_dialog(
                frame,
                chunks[0],
                "Alias Manager",
                &prompt,
                &input,
                cursor_pos,
                false,
            );

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Enter", Theme::accent()),
                Span::styled(": Continue  ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Cancel", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
        AliasManagerState::Confirming { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            if let Some((name, command)) = manager.get_confirmation_data() {
                let key_info = vec![
                    ("Alias Name", name.as_str()),
                    ("Command", command.as_str()),
                ];

                input_dialog::render_confirmation_dialog(
                    frame,
                    chunks[0],
                    "Confirm Alias Creation",
                    "Create New Alias",
                    key_info,
                    "Do you want to create this alias?",
                );
            }

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Y", Theme::accent()),
                Span::styled(": Confirm  ", Theme::help()),
                Span::styled("N", Theme::accent()),
                Span::styled(": Cancel", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
        AliasManagerState::Processing => {
            input_dialog::render_message_dialog(
                frame,
                area,
                "Alias Manager",
                "Creating alias...",
                false,
            );
        }
        AliasManagerState::Success { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let message = manager.get_prompt();

            input_dialog::render_message_dialog(
                frame,
                chunks[0],
                "Success",
                &message,
                false,
            );

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Enter", Theme::accent()),
                Span::styled(" or ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Continue", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
        AliasManagerState::Error { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let message = manager.get_prompt();

            input_dialog::render_message_dialog(
                frame,
                chunks[0],
                "Error",
                &message,
                true,
            );

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Enter", Theme::accent()),
                Span::styled(" or ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Continue", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
    }
}

/// Renders the process manager based on its state
fn render_process_manager(frame: &mut Frame, manager: &crate::features::ProcessManager) {
    let area = frame.area();

    match &manager.state {
        ProcessManagerState::SelectingAction { selected } => {
            let actions = ProcessAction::all();
            let selected_idx = *selected;

            // Create layout with prompt at top, menu in middle, help at bottom
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3), // Prompt
                    Constraint::Min(10),   // Action list
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            // Render prompt
            let prompt = manager.get_prompt();
            let prompt_paragraph = Paragraph::new(prompt)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Process Manager")
                        .title_style(Theme::title())
                        .border_style(Theme::border()),
                );
            frame.render_widget(prompt_paragraph, chunks[0]);

            // Render action list
            let action_items: Vec<ListItem> = actions
                .iter()
                .enumerate()
                .map(|(i, action)| {
                    let is_selected = i == selected_idx;
                    let prefix = if is_selected { "▸ " } else { "  " };

                    let title_style = if is_selected {
                        Theme::menu_item_selected()
                    } else {
                        Theme::menu_item_active()
                    };

                    let title_line = Line::from(Span::styled(
                        format!("{}{}", prefix, action.name()),
                        title_style
                    ));

                    let desc_line = Line::from(Span::styled(
                        format!("    {}", action.description()),
                        Theme::dim()
                    ));

                    ListItem::new(vec![title_line, desc_line])
                })
                .collect();

            let action_list = List::new(action_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Select Action")
                    .title_style(Theme::title())
                    .border_style(Theme::border()),
            );
            frame.render_widget(action_list, chunks[1]);

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("↑/↓", Theme::accent()),
                Span::styled(": Navigate  ", Theme::help()),
                Span::styled("Enter", Theme::accent()),
                Span::styled(": Select  ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Cancel", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[2]);
        }
        ProcessManagerState::Confirming { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            if let Some((action_name, description)) = manager.get_confirmation_data() {
                let key_info = vec![
                    ("Action", action_name.as_str()),
                    ("Target", description.as_str()),
                ];

                input_dialog::render_confirmation_dialog(
                    frame,
                    chunks[0],
                    "Confirm Process Action",
                    "Kill Processes",
                    key_info,
                    "Are you sure you want to proceed?",
                );
            }

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Y", Theme::accent()),
                Span::styled(": Confirm  ", Theme::help()),
                Span::styled("N", Theme::accent()),
                Span::styled(": Cancel", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
        ProcessManagerState::Processing { .. } => {
            let message = manager.get_prompt();
            input_dialog::render_message_dialog(frame, area, "Process Manager", &message, false);
        }
        ProcessManagerState::Success { .. } => {
            let message = manager.get_prompt();
            input_dialog::render_message_dialog(frame, area, "Success", &message, false);
        }
        ProcessManagerState::Error { .. } => {
            let message = manager.get_prompt();
            input_dialog::render_message_dialog(frame, area, "Error", &message, true);
        }
    }
}

/// Renders the bookmark manager based on its state
fn render_bookmark_manager(frame: &mut Frame, manager: &crate::features::BookmarkManager) {
    let area = frame.area();

    match &manager.state {
        BookmarkManagerState::Confirming { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            if let Some(directory) = manager.get_confirmation_data() {
                let key_info = vec![
                    ("Directory", directory.as_str()),
                    ("Alias Name", "temp"),
                    ("Action", "Create/Overwrite"),
                ];

                input_dialog::render_confirmation_dialog(
                    frame,
                    chunks[0],
                    "Confirm Bookmark",
                    "Temporary Directory Bookmark",
                    key_info,
                    "Save this location as 'temp'?",
                );
            }

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Enter", Theme::accent()),
                Span::styled(": Confirm  ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Cancel", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
        BookmarkManagerState::Processing => {
            let message = manager.get_prompt();
            input_dialog::render_message_dialog(frame, area, "Bookmark Manager", &message, false);
        }
        BookmarkManagerState::Success { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let message = manager.get_prompt();
            input_dialog::render_message_dialog(frame, chunks[0], "Success", &message, false);

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Enter", Theme::accent()),
                Span::styled(" or ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Exit", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
        BookmarkManagerState::Error { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let message = manager.get_prompt();
            input_dialog::render_message_dialog(frame, chunks[0], "Error", &message, true);

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Enter", Theme::accent()),
                Span::styled(" or ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Continue", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
    }
}

/// Renders the usage viewer based on its state
fn render_usage_viewer(frame: &mut Frame, viewer: &crate::features::UsageViewer) {
    let area = frame.area();

    match &viewer.state {
        UsageViewerState::Ready => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let message = viewer.get_prompt();
            input_dialog::render_message_dialog(frame, chunks[0], "Usage Viewer", &message, false);

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Enter", Theme::accent()),
                Span::styled(": Open Browser  ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Cancel", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
        UsageViewerState::Opening => {
            let message = viewer.get_prompt();
            input_dialog::render_message_dialog(frame, area, "Usage Viewer", &message, false);
        }
        UsageViewerState::Success { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let message = viewer.get_prompt();
            input_dialog::render_message_dialog(frame, chunks[0], "Success", &message, false);

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Enter", Theme::accent()),
                Span::styled(" or ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Continue", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
        UsageViewerState::Error { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let message = viewer.get_prompt();
            input_dialog::render_message_dialog(frame, chunks[0], "Error", &message, true);

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Enter", Theme::accent()),
                Span::styled(" or ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Continue", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
    }
}
