use std::io::{self, Stdout};

use color_eyre::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Frame, Terminal as RatatuiTerminal};

/// Initialize panic hook to restore terminal on panic
///
/// CRITICAL: This MUST be called before any terminal setup to ensure
/// the terminal is always restored, even on panic.
fn init_panic_hook() {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Restore terminal FIRST before showing panic
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        default_panic(info);
    }));
}

/// Terminal wrapper with RAII cleanup
///
/// Manages terminal state (raw mode, alternate screen) and ensures
/// proper cleanup on drop, even if the application panics.
pub struct Terminal {
    inner: RatatuiTerminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    /// Create a new Terminal instance
    ///
    /// Sets up the panic hook, enables raw mode, enters alternate screen,
    /// and creates the ratatui terminal.
    pub fn new() -> Result<Self> {
        // CRITICAL: Install panic hook FIRST
        init_panic_hook();

        // Enable raw mode for keyboard input
        enable_raw_mode()?;

        // Enter alternate screen (preserves original terminal content)
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        // Create backend and terminal
        let backend = CrosstermBackend::new(stdout);
        let inner = RatatuiTerminal::new(backend)?;

        Ok(Self { inner })
    }

    /// Draw a frame using the provided closure
    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Frame),
    {
        self.inner.draw(f)?;
        Ok(())
    }

    /// Get current terminal size
    pub fn size(&self) -> Result<ratatui::layout::Size> {
        Ok(self.inner.size()?)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Restore terminal state
        // Use let _ = to ignore errors during cleanup
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}
