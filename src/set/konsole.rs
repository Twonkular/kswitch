use crate::theme::Theme;

fn set_default_theme(theme: &Theme) {
    todo!()
}

fn set_session_theme(session_id: String, theme: &Theme) {
    todo!()
}

fn get_session_ids() -> Vec<String> {
    todo!()
}

pub fn set(theme: &Theme) {
    let session_ids = get_session_ids();

    for id in session_ids.iter() {
        println!("{}", id);
    }
}
