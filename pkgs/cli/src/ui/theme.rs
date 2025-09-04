use crate::prelude::*;

pub static UI_THEME: LazyLock<ColorfulTheme> = LazyLock::new(|| ColorfulTheme::default());

pub static UI_INFO_PREFIX: LazyLock<StyledObject<&str>> = LazyLock::new(|| style("i").blue());

pub static UI_WARN_PREFIX: LazyLock<StyledObject<&str>> = LazyLock::new(|| style("!").yellow());

pub struct UiTheme {}

impl UiTheme {
    pub fn for_dialoguer() -> &'static ColorfulTheme {
        &*UI_THEME
    }

    pub fn format_info(message: &str) -> String {
        format!(
            "{} {}",
            *UI_INFO_PREFIX,
            UI_THEME.prompt_style.apply_to(message)
        )
    }

    pub fn format_warn(message: &str) -> String {
        format!(
            "{} {}",
            *UI_WARN_PREFIX,
            UI_THEME.prompt_style.apply_to(message)
        )
    }

    pub fn format_success(message: &str) -> String {
        format!(
            "{} {}",
            UI_THEME.success_prefix,
            UI_THEME.prompt_style.apply_to(message)
        )
    }

    pub fn format_error(message: &str) -> String {
        format!(
            "{} {}",
            UI_THEME.error_prefix,
            UI_THEME.error_style.apply_to(message)
        )
    }

    pub fn start_spinner(message: &str) -> ProgressBar {
        ProgressBar::no_length().with_message(message.to_string())
    }
}
