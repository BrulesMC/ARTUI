use crate::config::{Config, Profile};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};
use serde_json::Value;

pub struct RyzenAdjBox {
    pub index: usize,
}

impl RyzenAdjBox {
    pub fn new() -> Self {
        Self { index: 0 }
    }
    /// Build list of RyzenAdj fields
    fn fields<'a>(profile: &'a Profile) -> Vec<(&'a str, &'a Value)> {
        profile
            .iter()
            .filter(|(k, _)| *k != "cpu" && *k != "gpu") // skip ASUSCTL fields
            .map(|(k, v)| (k.as_str(), v))
            .collect()
    }

    pub fn handle_key(&mut self, key: KeyEvent, cfg: &mut Config, profile_name: &str) {
        let profile = match cfg.get_mut(profile_name) {
            Some(p) => p,
            None => return,
        };

        let fields = Self::fields(profile);
        if fields.is_empty() {
            return;
        }

        match key.code {
            KeyCode::Char('j') => {
                if self.index + 1 < fields.len() {
                    self.index += 1;
                } else {
                    self.index = 0;
                }
            }

            KeyCode::Char('k') => {
                if self.index > 0 {
                    self.index -= 1;
                } else {
                    self.index = fields.len() - 1;
                }
            }

            // toggle booleans
            KeyCode::Char(' ') | KeyCode::Enter => {
                let (key, val) = fields[self.index];
                if val.is_boolean() {
                    let old = val.as_bool().unwrap_or(false);
                    profile.insert(key.into(), Value::Bool(!old));
                }
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let (key, val) = fields[self.index];
                if val.is_number() {
                    let mut s = val.to_string();
                    if s == "null" {
                        s.clear();
                    }
                    s.push(c);

                    if let Ok(num) = s.parse::<i64>() {
                        profile.insert(key.into(), Value::Number(num.into()));
                    }
                }
            }
            KeyCode::Backspace => {
                let (key, val) = fields[self.index];
                if val.is_number() {
                    let mut s = val.to_string();
                    if s == "null" {
                        return;
                    }
                    s.pop();

                    if s.is_empty() {
                        profile.insert(key.into(), Value::Number(0.into()));
                    } else if let Ok(num) = s.parse::<i64>() {
                        profile.insert(key.into(), Value::Number(num.into()));
                    }
                }
            }

            _ => {}
        }
    }
    //render box
    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        cfg: &Config,
        profile_name: &str,
        focused: bool,
    ) {
        let profile = match cfg.get(profile_name) {
            Some(p) => p,
            None => return,
        };

        let fields = Self::fields(profile);

        let rows: Vec<Row> = fields
            .iter()
            .enumerate()
            .map(|(i, (key, val))| {
                let mut row = Row::new(vec![key.to_string(), val.to_string()]);

                if focused && i == self.index {
                    row = row.style(Style::default().add_modifier(Modifier::REVERSED));
                }

                row
            })
            .collect();

        let mut block = Block::default().title("RyzenAdj").borders(Borders::ALL);
        if focused {
            block = block.border_style(Style::default().add_modifier(Modifier::BOLD));
        }

        let table = Table::new(
            rows,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .block(block);

        f.render_widget(table, area);
    }
}
