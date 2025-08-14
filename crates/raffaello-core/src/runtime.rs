use std::{io::stdout, sync::Arc};

use crate::component::Component;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

#[derive(Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Raw,
    Alternative,
}

pub struct AppOptions {
    mode: Mode,
    tickrate: u32,
    framerate: u32,
    mouse: bool,
}

impl Default for AppOptions {
    fn default() -> Self {
        Self {
            mode: Mode::default(),
            tickrate: 60,
            framerate: 60,
            mouse: false,
        }
    }
}

pub struct App<C: Component, Ctx: Send + Sync + 'static> {
    context: Arc<Ctx>,
    root: C,
    options: AppOptions,
    // logger:
    // hooks:
    // errors
}

impl<C: Component, Ctx: Send + Sync + 'static> App<C, Ctx> {
    pub async fn run(mut self) -> color_eyre::Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        let guard = TerminalGuard::enter(self.options)?;

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<C::Message>();

        guard.exit()?;

        Ok(())
    }
}

pub struct AppBuilder<C: Component, Ctx: Send + Sync + 'static> {
    context: Arc<Ctx>,
    root: C,
    options: AppOptions,
    // logger:
    // hooks:
    // errors
}

impl<C: Component, Ctx: Send + Sync + 'static> AppBuilder<C, Ctx> {
    pub fn new(context: Arc<Ctx>, root: C) -> Self {
        Self {
            context,
            root,
            options: AppOptions::default(),
            // logger: None,
            // panic_hook: None,
            // errors: None,
        }
    }
    pub fn options(mut self, opts: AppOptions) -> Self {
        self.options = opts;
        self
    }

    pub fn build(self) -> App<C, Ctx> {
        App {
            context: self.context,
            root: self.root,
            options: self.options,
            // logger: self.logger,
            // panic_hook: self.panic_hook,
            // errors: self.errors,
        }
    }
}

pub struct TerminalGuard {
    mode: Mode,
    mouse: bool,
}

impl TerminalGuard {
    fn enter(opts: AppOptions) -> color_eyre::Result<Self> {
        enable_raw_mode()?;
        let mut out = stdout();
        if opts.mode == Mode::Alternative {
            execute!(out, EnterAlternateScreen)?;
        }
        if opts.mouse {
            execute!(out, EnableMouseCapture)?;
        }
        Ok(Self {
            mode: opts.mode,
            mouse: opts.mouse,
        })
    }

    fn exit(&self) -> color_eyre::Result<()> {
        let mut out = stdout();
        if self.mode == Mode::Alternative {
            execute!(out, LeaveAlternateScreen)?;
        }
        if self.mouse {
            execute!(out, DisableMouseCapture)?;
        }
        disable_raw_mode()?;
        Ok(())
    }
}
