mod apply;
mod config;
mod ui;

use apply::apply_profile;
use config::{load_config, save_config, Config};
use std::io::Write;
use ui::asusctl::AsusCtlBox;
use ui::footer::render_footer;
use ui::profiles::ProfilesBox;
use ui::ryzenadj::RyzenAdjBox;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use std::process::Command;

#[derive(Clone, Copy)]
pub enum Focus {
    Profiles,
    AsusCtl,
    RyzenAdj,
}

fn get_active_profile_from_asusctl() -> Option<String> {
    let output = Command::new("asusctl")
        .arg("profile")
        .arg("get")
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&output.stdout);

    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("Active profile:") {
            return Some(rest.trim().to_string());
        }
    }

    None
}

fn main() -> anyhow::Result<()> {
    let mut cfg: Config = load_config();

    let mut profiles = ProfilesBox::new(&cfg);

    if let Some(active) = get_active_profile_from_asusctl() {
        profiles.active = Some(active);
    }

    let mut asusctl = AsusCtlBox::new();
    let mut ryzenadj = RyzenAdjBox::new();

    let mut focus = Focus::Profiles;

    let mut save_message_until: Option<std::time::Instant> = None;
    let mut apply_at: Option<std::time::Instant> = None;

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, Clear(ClearType::All))?;

    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(35),
                    Constraint::Percentage(25),
                    Constraint::Percentage(10),
                ])
                .split(f.size());

            profiles.render(f, chunks[0], matches!(focus, Focus::Profiles));

            if let Some(active) = profiles.active.as_ref() {
                asusctl.render(f, chunks[1], &cfg, active, matches!(focus, Focus::AsusCtl));
                ryzenadj.render(f, chunks[2], &cfg, active, matches!(focus, Focus::RyzenAdj));
            }

            let saved_active = save_message_until
                .map(|until| until > std::time::Instant::now())
                .unwrap_or(false);

            render_footer(f, chunks[3], focus, saved_active);
        })?;

        // auto-apply timer
        if let Some(when) = apply_at {
            if when <= std::time::Instant::now() {
                let active = profiles.active.as_ref().unwrap().clone();

                disable_raw_mode()?;
                execute!(
                    std::io::stdout(),
                    LeaveAlternateScreen,
                    crossterm::cursor::Show,
                    Clear(ClearType::All)
                )?;
                std::io::stdout().flush()?;

                apply_profile(&active);

                drop(terminal);

                enable_raw_mode()?;
                execute!(
                    std::io::stdout(),
                    EnterAlternateScreen,
                    crossterm::cursor::Hide,
                    Clear(ClearType::All)
                )?;

                let backend = CrosstermBackend::new(&mut stdout);
                terminal = Terminal::new(backend)?;

                apply_at = None;
            }
        }

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        drop(terminal);
                        execute!(std::io::stdout(), LeaveAlternateScreen)?;
                        execute!(std::io::stdout(), Clear(ClearType::All))?;
                        return Ok(());
                    }

                    KeyCode::Tab | KeyCode::Char('l') => {
                        focus = match focus {
                            Focus::Profiles => Focus::AsusCtl,
                            Focus::AsusCtl => Focus::RyzenAdj,
                            Focus::RyzenAdj => Focus::Profiles,
                        };
                        continue;
                    }

                    KeyCode::Char('h') => {
                        focus = match focus {
                            Focus::Profiles => Focus::RyzenAdj,
                            Focus::AsusCtl => Focus::Profiles,
                            Focus::RyzenAdj => Focus::AsusCtl,
                        };
                        continue;
                    }

                    KeyCode::Char('s') => {
                        save_config(&cfg);
                        save_message_until =
                            Some(std::time::Instant::now() + std::time::Duration::from_secs(1));
                    }
                    KeyCode::Char('a') => {
                        apply_at = Some(std::time::Instant::now());
                    }

                    _ => {}
                }

                match focus {
                    Focus::Profiles => {
                        let switched = profiles.handle_key(key);
                        if switched {
                            apply_at = Some(std::time::Instant::now());
                        }
                    }

                    Focus::AsusCtl => {
                        let active = profiles.active.as_ref().unwrap();
                        asusctl.handle_key(key, &mut cfg, active);
                    }

                    Focus::RyzenAdj => {
                        let active = profiles.active.as_ref().unwrap();
                        ryzenadj.handle_key(key, &mut cfg, active);
                    }
                }
            }
        }
    }
}
