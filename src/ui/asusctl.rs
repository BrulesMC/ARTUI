use crate::config::{Config, Profile};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};
use serde_json::Value;

pub struct AsusCtlBox {
    pub index: usize,
    pub buffer: String,
    pub editing_gpu: bool,
}

impl AsusCtlBox {
    pub fn new() -> Self {
        Self {
            index: 0,
            buffer: String::new(),
            editing_gpu: false,
        }
    }
    // Parse the fan curve string into a vector(temperature, fan speed)
    fn parse_curve(s: &str) -> Vec<(i64, i64)> {
        s.split(',')
            .filter_map(|pair| {
                let (t, f) = pair.split_once(':')?;
                Some((
                    t.trim_end_matches('c').parse().ok()?,
                    f.trim_end_matches('%').parse().ok()?,
                ))
            })
            .collect()
    }
    // Format the curve back into a string
    fn format_curve(rows: &[(i64, i64)]) -> String {
        rows.iter()
            .map(|(t, f)| format!("{t}c:{f}%"))
            .collect::<Vec<_>>()
            .join(",")
    }
    // Determine whether we're editing CPU or GPU
    fn key(&self) -> &'static str {
        if self.editing_gpu {
            "gpu"
        } else {
            "cpu"
        }
    }
    // Update the preview of the fan curve with the current buffer
    fn update_preview(rows: &mut [(i64, i64)], idx: usize, buf: &str) {
        let mut p = buf.to_string();
        while p.len() < 4 {
            p.push('0');
        }
        // Parse temperature and fan speed, default to existing if parse fails
        let t = p[0..2].parse().unwrap_or(rows[idx].0);
        let f = p[2..4].parse().unwrap_or(rows[idx].1);
        rows[idx] = (t, f);
    }
    // Set the current rows back to the profile
    fn commit(map: &mut Profile, key: &str, rows: &[(i64, i64)]) {
        map.insert(key.into(), Value::String(Self::format_curve(rows)));
    }
    //navigation
    pub fn handle_key(&mut self, key: KeyEvent, cfg: &mut Config, profile: &str) {
        if let Some(map) = cfg.get_mut(profile) {
            let key_name = self.key();
            let raw = map.get(key_name).and_then(|v| v.as_str()).unwrap_or("");
            let mut rows = Self::parse_curve(raw);

            if rows.is_empty() {
                return;
            }

            match key.code {
                KeyCode::Char('j') => self.index = (self.index + 1) % rows.len(),
                KeyCode::Char('k') => self.index = (self.index + rows.len() - 1) % rows.len(),
                KeyCode::Char('c') => {
                    self.editing_gpu = false;
                    self.buffer.clear();
                }
                KeyCode::Char('g') => {
                    self.editing_gpu = true;
                    self.buffer.clear();
                }
                //limit to 4 characters
                KeyCode::Char(c) if c.is_ascii_digit() && self.buffer.len() < 4 => {
                    self.buffer.push(c);
                    Self::update_preview(&mut rows, self.index, &self.buffer);
                    Self::commit(map, key_name, &rows);
                }
                KeyCode::Backspace | KeyCode::Delete => {
                    if !self.buffer.is_empty() {
                        self.buffer.pop();
                        Self::update_preview(&mut rows, self.index, &self.buffer);
                        Self::commit(map, key_name, &rows);
                    }
                }

                _ => {}
            }
        }
    }
    //render the block
    pub fn render(&self, f: &mut Frame, area: Rect, cfg: &Config, profile: &str, focused: bool) {
        let key_name = self.key();

        let raw = cfg
            .get(profile)
            .and_then(|p| p.get(key_name))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let rows = Self::parse_curve(raw);
        let rows: Vec<Row> = rows
            .iter()
            .enumerate()
            .map(|(i, (t, f))| {
                let mut row = Row::new(vec![format!("{t}°C → {f}%")]);
                if focused && i == self.index {
                    row = row.style(Style::default().add_modifier(Modifier::REVERSED));
                }
                row
            })
            .collect();

        let title = if self.editing_gpu {
            "ASUSCTL GPU Fan Curve"
        } else {
            "ASUSCTL CPU Fan Curve"
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(if focused {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });

        f.render_widget(
            Table::new(rows, [Constraint::Percentage(100)]).block(block),
            area,
        );
    }
}
