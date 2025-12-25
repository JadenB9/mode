use crate::{
    app::{ActiveFeature, App, AppState},
    features::{AliasManagerState, BookmarkManagerState, PortState, ProcessAction, ProcessManagerState, ScanOption, ScanType, ScannerState, UsageViewerState},
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
        ActiveFeature::Scanner(scanner) => {
            render_scanner(frame, scanner);
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

/// Renders the scanner based on its state
fn render_scanner(frame: &mut Frame, scanner: &crate::features::Scanner) {
    let area = frame.area();

    match &scanner.state {
        ScannerState::SelectingScanType { selected } => {
            let scan_types = ScanType::all();
            let selected_idx = *selected;

            // Create layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3), // Prompt
                    Constraint::Min(10),   // Scan type list
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            // Render prompt
            let prompt = scanner.get_prompt();
            let prompt_paragraph = Paragraph::new(prompt)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Port Scanner")
                        .title_style(Theme::title())
                        .border_style(Theme::border()),
                );
            frame.render_widget(prompt_paragraph, chunks[0]);

            // Render scan type list
            let scan_items: Vec<ListItem> = scan_types
                .iter()
                .enumerate()
                .map(|(i, scan_type)| {
                    let is_selected = i == selected_idx;
                    let prefix = if is_selected { "▸ " } else { "  " };

                    let title_style = if is_selected {
                        Theme::menu_item_selected()
                    } else {
                        Theme::menu_item_active()
                    };

                    let title_line = Line::from(Span::styled(
                        format!("{}{}", prefix, scan_type.name()),
                        title_style
                    ));

                    let desc_line = Line::from(Span::styled(
                        format!("    {}", scan_type.description()),
                        Theme::dim()
                    ));

                    ListItem::new(vec![title_line, desc_line])
                })
                .collect();

            let scan_list = List::new(scan_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Select Scan Type")
                    .title_style(Theme::title())
                    .border_style(Theme::border()),
            );
            frame.render_widget(scan_list, chunks[1]);

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
        ScannerState::EnteringTarget { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let prompt = scanner.get_prompt();
            let input = scanner.get_input();
            let cursor_pos = input.len();

            input_dialog::render_input_dialog(
                frame,
                chunks[0],
                "Port Scanner",
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
                Span::styled(": Back", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
        ScannerState::SelectingOptions { selected, .. } => {
            let options = ScanOption::all();
            let selected_idx = *selected;
            let (service_detection, save_to_file) = scanner.get_options_state().unwrap();

            // Create layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3), // Prompt
                    Constraint::Min(10),   // Options list
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            // Render prompt
            let prompt = scanner.get_prompt();
            let prompt_paragraph = Paragraph::new(prompt)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Scan Options")
                        .title_style(Theme::title())
                        .border_style(Theme::border()),
                );
            frame.render_widget(prompt_paragraph, chunks[0]);

            // Render options list
            let option_items: Vec<ListItem> = options
                .iter()
                .enumerate()
                .map(|(i, option)| {
                    let is_selected = i == selected_idx;
                    let prefix = if is_selected { "▸ " } else { "  " };

                    let enabled = match option {
                        ScanOption::ServiceDetection => service_detection,
                        ScanOption::SaveToFile => save_to_file,
                    };

                    let title_style = if is_selected {
                        Theme::menu_item_selected()
                    } else {
                        Theme::menu_item_active()
                    };

                    let title_line = Line::from(Span::styled(
                        format!("{}{}", prefix, option.name()),
                        title_style
                    ));

                    let desc_line = Line::from(Span::styled(
                        format!("    {}", option.description(enabled)),
                        Theme::dim()
                    ));

                    ListItem::new(vec![title_line, desc_line])
                })
                .collect();

            let options_list = List::new(option_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Configure Options")
                    .title_style(Theme::title())
                    .border_style(Theme::border()),
            );
            frame.render_widget(options_list, chunks[1]);

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("↑/↓", Theme::accent()),
                Span::styled(": Navigate  ", Theme::help()),
                Span::styled("Space", Theme::accent()),
                Span::styled(": Toggle  ", Theme::help()),
                Span::styled("Enter", Theme::accent()),
                Span::styled(": Continue  ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Back", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[2]);
        }
        ScannerState::Confirming { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            if let Some(data) = scanner.get_confirmation_data() {
                let key_info: Vec<(&str, &str)> = data
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();

                input_dialog::render_confirmation_dialog(
                    frame,
                    chunks[0],
                    "Confirm Port Scan",
                    "Start Scan",
                    key_info,
                    "Begin port scan with these settings?",
                );
            }

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("Y", Theme::accent()),
                Span::styled(": Start Scan  ", Theme::help()),
                Span::styled("N", Theme::accent()),
                Span::styled(": Cancel", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[1]);
        }
        ScannerState::Scanning { .. } => {
            let message = scanner.get_prompt();
            input_dialog::render_message_dialog(frame, area, "Port Scanner", &message, false);
        }
        ScannerState::ViewingResults { target, open_ports, scroll } => {
            // Create layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Min(10),   // Results list
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            // Render header
            let header = format!(
                "Scan Results for {} - {} open port{}",
                target,
                open_ports.len(),
                if open_ports.len() == 1 { "" } else { "s" }
            );
            let header_paragraph = Paragraph::new(header)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Port Scanner")
                        .title_style(Theme::title())
                        .border_style(Theme::border()),
                );
            frame.render_widget(header_paragraph, chunks[0]);

            // Render results list
            let result_items: Vec<ListItem> = open_ports
                .iter()
                .enumerate()
                .map(|(i, port_info)| {
                    let is_highlighted = i == *scroll;
                    let service = port_info.service.as_deref().unwrap_or("unknown");

                    let state_str = match port_info.state {
                        PortState::Open => "OPEN",
                        PortState::Closed => "CLOSED",
                        PortState::Filtered => "FILTERED",
                    };

                    let line = if is_highlighted {
                        Line::from(vec![
                            Span::styled("▸ ", Theme::accent()),
                            Span::styled(format!("Port {:5}", port_info.port), Theme::menu_item_selected()),
                            Span::styled(" | ", Theme::dim()),
                            Span::styled(format!("{:8}", state_str), Theme::success()),
                            Span::styled(" | ", Theme::dim()),
                            Span::styled(service, Theme::menu_item_selected()),
                        ])
                    } else {
                        Line::from(vec![
                            Span::styled("  ", Theme::text()),
                            Span::styled(format!("Port {:5}", port_info.port), Theme::text()),
                            Span::styled(" | ", Theme::dim()),
                            Span::styled(format!("{:8}", state_str), Theme::success()),
                            Span::styled(" | ", Theme::dim()),
                            Span::styled(service, Theme::dim()),
                        ])
                    };

                    ListItem::new(line)
                })
                .collect();

            let results_list = List::new(result_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Open Ports")
                    .title_style(Theme::title())
                    .border_style(Theme::border()),
            );
            frame.render_widget(results_list, chunks[1]);

            // Render help text
            let help_text = Line::from(vec![
                Span::styled("↑/↓", Theme::accent()),
                Span::styled(": Scroll  ", Theme::help()),
                Span::styled("Enter", Theme::accent()),
                Span::styled(" or ", Theme::help()),
                Span::styled("ESC", Theme::accent()),
                Span::styled(": Return to Menu", Theme::help()),
            ]);
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(help_paragraph, chunks[2]);
        }
        ScannerState::Success { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let message = scanner.get_prompt();
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
        ScannerState::Error { .. } => {
            // Create layout with dialog and help text
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Dialog
                    Constraint::Length(3), // Help text
                ])
                .split(area);

            let message = scanner.get_prompt();
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
