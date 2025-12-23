use crate::{
    event::Event,
    features::{AliasManager, AliasManagerState, BookmarkManager, BookmarkManagerState, PlaceholderFeature, ProcessManager, ProcessManagerState, UsageViewer, UsageViewerState},
    menu::{MenuItem, MenuState},
    utils::Result,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Active feature types
#[derive(Debug)]
pub enum ActiveFeature {
    AliasManager(AliasManager),
    ProcessManager(ProcessManager),
    BookmarkManager(BookmarkManager),
    UsageViewer(UsageViewer),
    Placeholder(PlaceholderFeature),
}

/// Application state
#[derive(Debug)]
pub enum AppState {
    /// Main menu is displayed
    MainMenu,
    /// A feature is active
    FeatureActive(ActiveFeature),
    /// Application is exiting
    Exiting,
}

/// Main application
pub struct App {
    /// Current state
    pub state: AppState,
    /// Menu navigation state
    pub menu_state: MenuState,
    /// Should quit flag
    pub should_quit: bool,
    /// Optional error message
    pub error_message: Option<String>,
    /// Command to execute on exit (for shell integration)
    pub exit_command: Option<String>,
}

impl App {
    /// Creates a new application
    pub fn new() -> Self {
        Self {
            state: AppState::MainMenu,
            menu_state: MenuState::new(),
            should_quit: false,
            error_message: None,
            exit_command: None,
        }
    }

    /// Handles an event
    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key) => self.handle_key(key)?,
            Event::Tick => {
                // Handle periodic updates if needed
            }
            Event::Resize(_, _) => {
                // Terminal resize handled by ratatui
            }
            Event::Mouse(_) => {
                // Mouse events not implemented yet
            }
        }

        Ok(())
    }

    /// Handles keyboard input
    fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        // Check state first without borrowing
        let is_main_menu = matches!(self.state, AppState::MainMenu);
        let is_exiting = matches!(self.state, AppState::Exiting);

        if is_main_menu {
            self.handle_main_menu_key(key)
        } else if is_exiting {
            Ok(())
        } else {
            // Must be FeatureActive - handle inline to avoid borrow issues
            let mut should_return_to_menu = false;

            if let AppState::FeatureActive(feature) = &mut self.state {
                match feature {
                    ActiveFeature::AliasManager(manager) => {
                        should_return_to_menu = Self::handle_alias_manager_key_static(key, manager)?;

                        // If alias was successfully created, set exit command to source RC file
                        if manager.is_done() && matches!(manager.state, crate::features::AliasManagerState::Success { .. }) {
                            if let Some(rc_file) = manager.get_rc_file() {
                                self.exit_command = Some(format!("source {}", rc_file.display()));
                            }
                        }
                    }
                    ActiveFeature::ProcessManager(manager) => {
                        should_return_to_menu = Self::handle_process_manager_key_static(key, manager)?;
                    }
                    ActiveFeature::BookmarkManager(manager) => {
                        let result = Self::handle_bookmark_manager_key_static(key, manager)?;
                        should_return_to_menu = result.0;

                        // If bookmark was successful, exit the app and set exit command
                        if result.1 {
                            self.should_quit = true;
                            if let Some(rc_file) = manager.get_rc_file() {
                                self.exit_command = Some(format!("source {}", rc_file.display()));
                            }
                        }
                    }
                    ActiveFeature::UsageViewer(viewer) => {
                        should_return_to_menu = Self::handle_usage_viewer_key_static(key, viewer)?;
                    }
                    ActiveFeature::Placeholder(_) => {
                        // Just ESC to go back
                        if matches!(key.code, KeyCode::Esc) {
                            should_return_to_menu = true;
                        }
                    }
                }
            }

            if should_return_to_menu {
                self.state = AppState::MainMenu;
            }

            Ok(())
        }
    }

    /// Handles keyboard input in main menu
    fn handle_main_menu_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.menu_state.previous();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.menu_state.next();
            }
            KeyCode::Enter => {
                self.activate_menu_item()?;
            }
            KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }

        Ok(())
    }

    /// Activates the currently selected menu item
    fn activate_menu_item(&mut self) -> Result<()> {
        let item = self.menu_state.selected_item();

        if item.is_active() {
            match item {
                MenuItem::AliasManager => {
                    let mut manager = AliasManager::new();
                    if let Err(e) = manager.initialize() {
                        self.error_message = Some(format!("Failed to initialize: {}", e));
                    } else {
                        self.state = AppState::FeatureActive(ActiveFeature::AliasManager(manager));
                    }
                }
                MenuItem::ProcessManager => {
                    let manager = ProcessManager::new();
                    self.state = AppState::FeatureActive(ActiveFeature::ProcessManager(manager));
                }
                MenuItem::Bookmark => {
                    let mut manager = BookmarkManager::new();
                    if let Err(e) = manager.initialize() {
                        self.error_message = Some(format!("Failed to initialize: {}", e));
                    } else {
                        self.state = AppState::FeatureActive(ActiveFeature::BookmarkManager(manager));
                    }
                }
                MenuItem::UsageViewer => {
                    let mut viewer = UsageViewer::new();
                    // Immediately open the browser
                    if let Err(e) = viewer.open_browser() {
                        self.error_message = Some(format!("Failed to open browser: {}", e));
                    }
                    self.state = AppState::FeatureActive(ActiveFeature::UsageViewer(viewer));
                }
                _ => {
                    // Should not happen as we check is_active()
                }
            }
        } else {
            // Show placeholder
            self.state = AppState::FeatureActive(ActiveFeature::Placeholder(
                PlaceholderFeature::new(item.name().to_string()),
            ));
        }

        Ok(())
    }

    /// Handles keyboard input in alias manager (static method to avoid borrow issues)
    /// Returns true if should return to main menu
    fn handle_alias_manager_key_static(key: KeyEvent, manager: &mut AliasManager) -> Result<bool> {
        let mut return_to_menu = false;

        match &manager.state {
            AliasManagerState::EnteringName { .. } | AliasManagerState::EnteringCommand { .. } => {
                match key.code {
                    KeyCode::Char(c) => {
                        manager.handle_char(c);
                    }
                    KeyCode::Backspace => {
                        manager.handle_backspace();
                    }
                    KeyCode::Enter => {
                        manager.advance()?;
                    }
                    KeyCode::Esc => {
                        return_to_menu = true;
                    }
                    _ => {}
                }
            }
            AliasManagerState::Confirming { .. } => {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        manager.advance()?;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        manager.go_back();
                    }
                    _ => {}
                }
            }
            AliasManagerState::Success { .. } | AliasManagerState::Error { .. } => {
                // Any key returns to main menu
                if matches!(key.code, KeyCode::Enter | KeyCode::Esc) {
                    return_to_menu = true;
                }
            }
            _ => {}
        }

        Ok(return_to_menu)
    }

    /// Handles keyboard input in process manager (static method to avoid borrow issues)
    /// Returns true if should return to main menu
    fn handle_process_manager_key_static(key: KeyEvent, manager: &mut ProcessManager) -> Result<bool> {
        let mut return_to_menu = false;

        match &manager.state {
            ProcessManagerState::SelectingAction { .. } => {
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        manager.previous();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        manager.next();
                    }
                    KeyCode::Enter => {
                        manager.confirm_selection();
                    }
                    KeyCode::Esc => {
                        return_to_menu = true;
                    }
                    _ => {}
                }
            }
            ProcessManagerState::Confirming { .. } => {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        manager.execute_action();
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        manager.go_back();
                    }
                    _ => {}
                }
            }
            ProcessManagerState::Success { .. } | ProcessManagerState::Error { .. } => {
                // Any key returns to main menu
                if matches!(key.code, KeyCode::Enter | KeyCode::Esc) {
                    return_to_menu = true;
                }
            }
            _ => {}
        }

        Ok(return_to_menu)
    }

    /// Handles keyboard input in bookmark manager (static method to avoid borrow issues)
    /// Returns (should_return_to_menu, should_exit_app) tuple
    fn handle_bookmark_manager_key_static(key: KeyEvent, manager: &mut BookmarkManager) -> Result<(bool, bool)> {
        let mut return_to_menu = false;
        let mut exit_app = false;

        match &manager.state {
            BookmarkManagerState::Confirming { .. } => {
                match key.code {
                    KeyCode::Enter => {
                        manager.confirm_bookmark()?;
                        // Check if we should exit after confirming
                        if manager.should_exit_app() {
                            exit_app = true;
                        }
                    }
                    KeyCode::Esc => {
                        return_to_menu = true;
                    }
                    _ => {}
                }
            }
            BookmarkManagerState::Success { .. } => {
                // Any key exits the app
                if matches!(key.code, KeyCode::Enter | KeyCode::Esc) {
                    exit_app = true;
                }
            }
            BookmarkManagerState::Error { .. } => {
                // Any key returns to main menu
                if matches!(key.code, KeyCode::Enter | KeyCode::Esc) {
                    return_to_menu = true;
                }
            }
            _ => {}
        }

        Ok((return_to_menu, exit_app))
    }

    /// Handles keyboard input in usage viewer (static method to avoid borrow issues)
    /// Returns true if should return to main menu
    fn handle_usage_viewer_key_static(key: KeyEvent, viewer: &mut UsageViewer) -> Result<bool> {
        let mut return_to_menu = false;

        match &viewer.state {
            UsageViewerState::Ready => {
                // This shouldn't happen as we open immediately, but handle it
                if matches!(key.code, KeyCode::Enter) {
                    viewer.open_browser()?;
                } else if matches!(key.code, KeyCode::Esc) {
                    return_to_menu = true;
                }
            }
            UsageViewerState::Opening => {
                // Just wait
            }
            UsageViewerState::Success { .. } | UsageViewerState::Error { .. } => {
                // Any key returns to main menu
                if matches!(key.code, KeyCode::Enter | KeyCode::Esc) {
                    return_to_menu = true;
                }
            }
        }

        Ok(return_to_menu)
    }

    /// Returns whether the app should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
