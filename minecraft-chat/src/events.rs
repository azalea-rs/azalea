enum ClickAction {
    OPEN_URL = Action::new("open_url", true),
    OPEN_FILE = Action::new("open_file", false),
    RUN_COMMAND = Action::new("run_command", true),
    SUGGEST_COMMAND = Action::new("suggest_command", true),
    CHANGE_PAGE = Action::new("change_page", true),
    COPY_TO_CLIPBOARD = Action::new("copy_to_clipboard", true),
}

struct ClickAction {
    pub name: String,
    pub allow_from_server: bool,
}

impl ClickAction {
    fn new(name: &str, allow_from_server: bool) -> Self {
        Self {
            name: name.to_string(),
            allow_from_server,
        }
    }
}

struct ClickEvent {
    action: ClickAction,
}
