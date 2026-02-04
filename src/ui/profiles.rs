use crate::config::{Config, Profile};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub struct ProfilesBox {
    pub names: Vec<String>,
    pub index: usize,
    pub active: Option<String>,
}

impl ProfilesBox {
    pub fn new(cfg: &Config) -> Self {
        let names = cfg.keys().cloned().collect::<Vec<_>>();
        println!("Profiles loaded: {:?}", names);

        Self {
            names,
            index: 0,
            active: None,
        }
    }
    // Returns true if active profile changed
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        let mut changed = false;

        match key.code {
            KeyCode::Char('j') => {
                if self.index + 1 < self.names.len() {
                    self.index += 1;
                } else {
                    self.index = 0;
                }
            }

            KeyCode::Char('k') => {
                if self.index > 0 {
                    self.index -= 1;
                } else {
                    self.index = self.names.len() - 1;
                }
            }

            KeyCode::Char(' ') => {
                let new_active = self.names[self.index].clone();
                if self.active != Some(new_active.clone()) {
                    self.active = Some(new_active);
                }
            }
            KeyCode::Enter => {
                changed = true; // Mark that the active profile changed
            }
            _ => {}
        }

        changed
    }

    pub fn current<'a>(&self, cfg: &'a Config) -> &'a Profile {
        let name = &self.names[self.index];
        cfg.get(name).unwrap_or_else(|| {
            panic!("Profile '{}' missing in config", name);
        })
    }

    pub fn active_profile<'a>(&self, cfg: &'a Config) -> &'a Profile {
        if let Some(ref name) = self.active {
            cfg.get(name).unwrap_or_else(|| {
                panic!("Active profile '{}' missing in config", name);
            })
        } else {
            self.current(cfg)
        }
    }
    //render box
    pub fn render(&self, f: &mut Frame, area: Rect, focused: bool) {
        let items: Vec<ListItem> = self
            .names
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let mut display = name.clone();

                if Some(name.clone()) == self.active {
                    display = format!("**{}**", display);
                }

                let width = area.width as usize;
                let pad = width.saturating_sub(display.len()) / 2;
                let centered = format!("{:pad$}{}", "", display, pad = pad);

                let mut style = Style::default();

                if focused && i == self.index {
                    style = style.add_modifier(Modifier::REVERSED);
                }

                ListItem::new(centered).style(style)
            })
            .collect();

        let mut block = Block::default().title("Profiles").borders(Borders::ALL);

        if focused {
            block = block.border_style(Style::default().add_modifier(Modifier::BOLD));
        }

        let list = List::new(items).block(block);
        f.render_widget(list, area);
    }
}
