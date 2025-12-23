use crate::{
    app::{ActiveFeature, App, AppState},
    features::AliasManagerState,
    ui_components::{input_dialog, menu_view},
};
use ratatui::Frame;

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
            let prompt = manager.get_prompt();
            let input = manager.get_input();
            let cursor_pos = input.len();

            input_dialog::render_input_dialog(
                frame,
                area,
                "Alias Manager",
                &prompt,
                &input,
                cursor_pos,
                false,
            );
        }
        AliasManagerState::Confirming { .. } => {
            let message = manager.get_prompt();

            input_dialog::render_message_dialog(
                frame,
                area,
                "Confirm Alias",
                &message,
                false,
            );
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
            let message = manager.get_prompt();

            input_dialog::render_message_dialog(
                frame,
                area,
                "Success",
                &message,
                false,
            );
        }
        AliasManagerState::Error { .. } => {
            let message = manager.get_prompt();

            input_dialog::render_message_dialog(
                frame,
                area,
                "Error",
                &message,
                true,
            );
        }
    }
}
