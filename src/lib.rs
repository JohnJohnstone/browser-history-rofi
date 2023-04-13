use std::process::Command;

use log::info;

use browser_history::{get_history, History};


rofi_mode::export_mode!(HistoryMode);

struct HistoryMode {
    entries: Vec<History>,
}

impl HistoryMode {
    fn get_entries() -> Vec<History> {
        get_history()
    }

    fn entry_content_for_entry(entry: History) -> rofi_mode::String {
        rofi_mode::format!(
            "<span><b>{}</b> - <i>{}</i></span>",
            Self::escape_pango_markup(entry.title.unwrap_or_default()),
            Self::escape_pango_markup(entry.url)
        )
    }

    fn escape_pango_markup(s: String) -> String {
        s.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&apos;")
    }
}

impl rofi_mode::Mode<'_> for HistoryMode {
    const NAME: &'static str = "browser_history\0";

    fn init(_api: rofi_mode::Api<'_>) -> Result<Self, ()> {

        env_logger::init();

        info!("Initialising History Mode");

        let entries = HistoryMode::get_entries();

        Ok(HistoryMode {
            entries,
        })
    }

    fn entries(&mut self) -> usize {
         self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        let entry = &self.entries[line];
        HistoryMode::entry_content_for_entry(entry.clone())
    }

    fn react(
        &mut self,
        event: rofi_mode::Event,
        _input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {
        match event {
            rofi_mode::Event::Ok { alt:_, selected } => {
                let selected_entry = &self.entries[selected];
                open_in_browser(selected_entry.url.clone());
                rofi_mode::Action::Exit
            }
            _ => rofi_mode::Action::Exit
        }
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        let entry = &self.entries[line];
        let match_str = HistoryMode::entry_content_for_entry(entry.clone());
        matcher.matches(match_str.as_str())
    }

    fn entry_style(&self, _line: usize) -> rofi_mode::Style {
        rofi_mode::Style::MARKUP
    }

    fn entry_attributes(&self, _line: usize) -> rofi_mode::Attributes {
        rofi_mode::Attributes::new()
    }

    fn entry_icon(&mut self, _line: usize, _height: u32) -> Option<rofi_mode::cairo::Surface> {
        None
    }

    fn completed(&self, line: usize) -> rofi_mode::String {
        self.entry_content(line)
    }

    fn preprocess_input(&mut self, input: &str) -> rofi_mode::String {
        input.into()
    }

    fn message(&mut self) -> rofi_mode::String {
        rofi_mode::String::new()
    }
}

fn open_in_browser(url: String) {
    Command::new("firefox").arg(url).spawn().unwrap();
}
