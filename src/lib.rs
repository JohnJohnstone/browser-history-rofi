use std::process::Command;

use log::info;

use browser_history::{get_history, History, Browser};

rofi_mode::export_mode!(HistoryMode);

struct HistoryMode<'rofi> {
    entries: Vec<History>,
    api: rofi_mode::Api<'rofi>,
}

impl<'rofi> HistoryMode<'rofi> {
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

impl<'rofi> rofi_mode::Mode<'rofi> for HistoryMode<'rofi> {
    const NAME: &'static str = "browser_history\0";

    fn init(api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {

        env_logger::init();

        info!("Initialising History Mode");

        let entries = HistoryMode::get_entries();

        Ok(HistoryMode {
            entries,
            api,
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

    fn entry_icon(&mut self, line: usize, height: u32) -> Option<rofi_mode::cairo::Surface> {
        let browser = &self.entries[line].browser;

        match browser {
            Browser::Firefox  => self.api.query_icon("firefox", height).wait(&mut self.api),
            Browser::Qutebrowser  => self.api.query_icon("qutebrowser", height).wait(&mut self.api),
            _ => None,
        }
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
