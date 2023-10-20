use crate::{turbo_stream_action, turbo_stream_target, turbo_stream_target_all};
use std::collections::BTreeMap;

pub fn add_css_class(targets: &str, classes: &str) -> String {
    let mut attributes = BTreeMap::from([("classes", classes)]);
    turbo_stream_target_all("add_css_class", targets, &mut attributes, "")
}

pub fn toggle_css_class(targets: &str, classes: &str) -> String {
    let mut attributes = BTreeMap::from([("classes", classes)]);
    turbo_stream_target_all("toggle_css_class", targets, &mut attributes, "")
}

pub fn clear_local_storage() -> String {
    clear_storage("local")
}

pub fn clear_session_storage() -> String {
    clear_storage("session")
}

pub fn clear_storage(storage_type: &str) -> String {
    let mut attributes = BTreeMap::from([("type", storage_type)]);
    turbo_stream_action("clear_storage", &mut attributes, "")
}

pub fn console_log(level: &str, message: &str) -> String {
    let mut attributes = BTreeMap::from([("level", level), ("message", message)]);
    turbo_stream_action("console_log", &mut attributes, "")
}

pub fn console_table(data: &str, columns: &str) -> String {
    let mut attributes = BTreeMap::from([("data", data), ("columns", columns)]);
    turbo_stream_action("console_table", &mut attributes, "")
}

pub fn dispatch_event<S: AsRef<str>>(targets: &str, name: &str, detail: S) -> String {
    let mut attributes = BTreeMap::from([("name", name)]);
    turbo_stream_target_all("dispatch_event", targets, &mut attributes, detail)
}

pub fn graft(targets: &str, parent: &str) -> String {
    let mut attributes = BTreeMap::from([("parent", parent)]);
    turbo_stream_target_all("graft", targets, &mut attributes, "")
}

pub fn history_back() -> String {
    turbo_stream_action("history_back", &mut Default::default(), "")
}

pub fn history_forward() -> String {
    turbo_stream_action("history_forward", &mut Default::default(), "")
}

pub fn history_go(delta: usize) -> String {
    let delta_str = delta.to_string();
    let mut attributes: BTreeMap<&str, &str> = BTreeMap::from([("delta", delta_str.as_str())]);
    turbo_stream_action("history_go", &mut attributes, "")
}

pub fn inner_html<S: AsRef<str>>(targets: &str, html: S) -> String {
    turbo_stream_target_all("inner_html", targets, &mut Default::default(), html)
}

pub fn insert_adjacent_html<S: AsRef<str>>(targets: &str, position: &str, html: S) -> String {
    let mut attributes = BTreeMap::from([("position", position)]);
    turbo_stream_target_all("insert_adjacent_html", targets, &mut attributes, html)
}

pub fn insert_adjacent_text(targets: &str, position: &str, text: &str) -> String {
    let mut attributes = BTreeMap::from([("position", position), ("text", text)]);
    turbo_stream_target_all("insert_adjacent_text", targets, &mut attributes, "")
}

pub fn morph<S: AsRef<str>>(targets: &str, html: S) -> String {
    turbo_stream_target_all("morph", targets, &mut Default::default(), html)
}

pub fn notification<'a, S: AsRef<str>>(
    title: &'a str,
    options: &mut BTreeMap<&str, &'a str>,
    body: S,
) -> String {
    options.insert("title", title);
    turbo_stream_action("notification", options, body)
}

pub fn outer_html<S: AsRef<str>>(targets: &str, html: S) -> String {
    turbo_stream_target_all("outer_html", targets, &mut Default::default(), html)
}

pub fn push_state<S: AsRef<str>>(url: &str, title: &str, state: S) -> String {
    let state = state.as_ref();
    let mut attributes = BTreeMap::from([("url", url), ("title", title), ("state", state)]);
    turbo_stream_action("push_state", &mut attributes, "")
}

pub fn redirect_to(url: &str, turbo_action: &str, turbo_frame: Option<&str>) -> String {
    let mut attributes = BTreeMap::from([("url", url), ("turbo-action", turbo_action)]);
    if let Some(value) = turbo_frame {
        attributes.insert("turbo-frame", value);
    }
    turbo_stream_action("redirect_to", &mut attributes, "")
}

pub fn reload() -> String {
    turbo_stream_action("reload", &mut Default::default(), "")
}

pub fn remove_attribute(targets: &str, attribute: &str) -> String {
    let mut attributes = BTreeMap::from([("attribute", attribute)]);
    turbo_stream_target_all("remove_attribute", targets, &mut attributes, "")
}

pub fn remove_css_class(targets: &str, classes: &str) -> String {
    let mut attributes = BTreeMap::from([("classes", classes)]);
    turbo_stream_target_all("remove_css_class", targets, &mut attributes, "")
}

pub fn remove_local_storage_item(key: &str) -> String {
    remove_storage_item(key, "local")
}

pub fn remove_session_storage_item(key: &str) -> String {
    remove_storage_item(key, "session")
}

pub fn remove_storage_item(key: &str, storage_type: &str) -> String {
    let mut attributes = BTreeMap::from([("key", key), ("type", storage_type)]);
    turbo_stream_action("remove_storage_item", &mut attributes, "")
}

pub fn replace_state<S: AsRef<str>>(url: &str, title: &str, state: S) -> String {
    let state = state.as_ref();
    let mut attributes = BTreeMap::from([("url", url), ("title", title), ("state", state)]);
    turbo_stream_action("replace_state", &mut attributes, "")
}

pub fn reset_form(targets: &str) -> String {
    turbo_stream_target_all("reset_form", targets, &mut Default::default(), "")
}

pub fn scroll_into_view(targets: &str) -> String {
    turbo_stream_target_all("scroll_into_view", targets, &mut Default::default(), "")
}

pub fn set_attribute(targets: &str, attribute: &str, value: &str) -> String {
    let mut attributes = BTreeMap::from([("attribute", attribute), ("value", value)]);
    turbo_stream_target_all("set_attribute", targets, &mut attributes, "")
}

pub fn set_cookie(cookie: &str) -> String {
    let mut attributes = BTreeMap::from([("cookie", cookie)]);
    turbo_stream_action("set_cookie", &mut attributes, "")
}

pub fn set_cookie_item(key: &str, value: &str) -> String {
    let mut attributes = BTreeMap::from([("key", key), ("value", value)]);
    turbo_stream_action("set_cookie_item", &mut attributes, "")
}

pub fn set_dataset_attribute(targets: &str, attribute: &str, value: &str) -> String {
    let mut attributes = BTreeMap::from([("attribute", attribute), ("value", value)]);
    turbo_stream_target_all("set_dataset_attribute", targets, &mut attributes, "")
}

pub fn set_focus(targets: &str) -> String {
    turbo_stream_target_all("set_focus", targets, &mut Default::default(), "")
}

pub fn set_storage_local_item(key: &str, value: &str) -> String {
    set_storage_item(key, value, "local")
}

pub fn set_meta(name: &str, content: &str) -> String {
    let mut attributes = BTreeMap::from([("name", name), ("content", content)]);
    turbo_stream_action("set_meta", &mut attributes, "")
}

pub fn set_property(targets: &str, name: &str, value: &str) -> String {
    let mut attributes = BTreeMap::from([("name", name), ("value", value)]);
    turbo_stream_target_all("set_property", targets, &mut attributes, "")
}

pub fn set_storage_session_item(key: &str, value: &str) -> String {
    set_storage_item(key, value, "session")
}

pub fn set_storage_item(key: &str, value: &str, storage_type: &str) -> String {
    let mut attributes = BTreeMap::from([("key", key), ("value", value), ("type", storage_type)]);
    turbo_stream_action("set_storage_item", &mut attributes, "")
}

pub fn set_style(targets: &str, name: &str, value: &str) -> String {
    let mut attributes = BTreeMap::from([("name", name), ("value", value)]);
    turbo_stream_target_all("set_style", targets, &mut attributes, "")
}

pub fn set_styles(targets: &str, styles: &str) -> String {
    let mut attributes = BTreeMap::from([("styles", styles)]);
    turbo_stream_target_all("set_styles", targets, &mut attributes, "")
}

pub fn set_title(title: &str) -> String {
    let mut attributes = BTreeMap::from([("title", title)]);
    turbo_stream_action("set_title", &mut attributes, "")
}

pub fn set_value(targets: &str, value: &str) -> String {
    let mut attributes = BTreeMap::from([("value", value)]);
    turbo_stream_target_all("set_value", targets, &mut attributes, "")
}

pub fn text_content(targets: &str, text: &str) -> String {
    let mut attributes = BTreeMap::from([("text", text)]);
    turbo_stream_target_all("text_content", targets, &mut attributes, "")
}

pub fn turbo_clear_cache() -> String {
    turbo_stream_action("turbo_clear_cache", &mut Default::default(), "")
}

pub fn turbo_frame_reload(target: &str) -> String {
    turbo_stream_target("turbo_frame_reload", target, &mut Default::default(), "")
}

pub fn turbo_frame_set_src(target: &str, src: &str) -> String {
    let mut attributes = BTreeMap::from([("src", src)]);
    turbo_stream_target("turbo_frame_set_src", target, &mut attributes, "")
}

pub fn turbo_progress_bar_hide() -> String {
    turbo_stream_action("turbo_progress_bar_hide", &mut Default::default(), "")
}

pub fn turbo_progress_bar_set_value(value: &str) -> String {
    let mut attributes = BTreeMap::from([("value", value)]);
    turbo_stream_action("turbo_progress_bar_set_value", &mut attributes, "")
}

pub fn turbo_progress_bar_show() -> String {
    turbo_stream_action("turbo_progress_bar_show", &mut Default::default(), "")
}

#[cfg(test)]
mod tests {

    #[test]
    fn add_css_class() {
        let expected = r##"<turbo-stream action="add_css_class" classes="container text-center" targets="#element"><template></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::add_css_class("#element", "container text-center")
        );
    }

    #[test]
    fn toggle_css_class() {
        let expected = r##"<turbo-stream action="toggle_css_class" classes="container text-center" targets="#element"><template></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::toggle_css_class("#element", "container text-center")
        );
    }

    #[test]
    fn clear_local_storage() {
        let expected = r#"<turbo-stream action="clear_storage" type="local"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::clear_local_storage());
    }

    #[test]
    fn clear_session_storage() {
        let expected = r#"<turbo-stream action="clear_storage" type="session"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::clear_session_storage());
    }

    #[test]
    fn clear_storage() {
        let expected = r#"<turbo-stream action="clear_storage" type="local"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::clear_storage("local"));
    }

    #[test]
    fn console_log() {
        let expected = r##"<turbo-stream action="console_log" level="info" message="#element"><template></template></turbo-stream>"##;
        assert_eq!(expected, super::console_log("info", "#element"));
    }

    #[test]
    fn console_table() {
        let expected = r#"<turbo-stream action="console_table" columns="[&quot;fruits&quot;]" data="[&quot;apples&quot;,&quot;oranges&quot;,&quot;bananas&quot;]"><template></template></turbo-stream>"#;
        assert_eq!(
            expected,
            super::console_table(r#"["apples","oranges","bananas"]"#, r#"["fruits"]"#)
        );
    }

    #[test]
    fn dispatch_event() {
        let expected = r##"<turbo-stream action="dispatch_event" name="custom-event" targets="#element"><template>{"foo":"bar"}</template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::dispatch_event("#element", "custom-event", r#"{"foo":"bar"}"#)
        );
    }

    #[test]
    fn graft() {
        let expected = r##"<turbo-stream action="graft" parent="#parent" targets="#input"><template></template></turbo-stream>"##;
        assert_eq!(expected, super::graft("#input", "#parent"));
    }

    #[test]
    fn history_back() {
        let expected =
            r#"<turbo-stream action="history_back"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::history_back());
    }

    #[test]
    fn history_forward() {
        let expected =
            r#"<turbo-stream action="history_forward"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::history_forward());
    }

    #[test]
    fn history_go() {
        let expected =
            r#"<turbo-stream action="history_go" delta="1"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::history_go(1));
    }

    #[test]
    fn inner_html() {
        let expected = r##"<turbo-stream action="inner_html" targets="#elements"><template><p>Content</p></template></turbo-stream>"##;
        assert_eq!(expected, super::inner_html("#elements", "<p>Content</p>"));
    }

    #[test]
    fn insert_adjacent_html() {
        let expected = r##"<turbo-stream action="insert_adjacent_html" position="beforeend" targets="#element"><template><p>Content</p></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::insert_adjacent_html("#element", "beforeend", "<p>Content</p>")
        );
    }

    #[test]
    fn insert_adjacent_text() {
        let expected = r##"<turbo-stream action="insert_adjacent_text" position="beforeend" targets="#element" text="mytext"><template></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::insert_adjacent_text("#element", "beforeend", "mytext")
        );
    }

    #[test]
    fn morph() {
        let expected = r##"<turbo-stream action="morph" targets="#input"><template><p>Morph</p></template></turbo-stream>"##;
        assert_eq!(expected, super::morph("#input", "<p>Morph</p>"));
    }

    #[test]
    fn notification() {
        let expected = r#"<turbo-stream action="notification" title="mytitle"><template><p>Content</p></template></turbo-stream>"#;
        assert_eq!(
            expected,
            super::notification("mytitle", &mut Default::default(), "<p>Content</p>")
        );
    }

    #[test]
    fn outer_html() {
        let expected = r##"<turbo-stream action="outer_html" targets="#element"><template><p>Outer HTML</p></template></turbo-stream>"##;
        assert_eq!(expected, super::outer_html("#element", "<p>Outer HTML</p>"));
    }

    #[test]
    fn push_state() {
        let expected = r#"<turbo-stream action="push_state" state="{}" title="title-1" url="/users/1"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::push_state("/users/1", "title-1", "{}"));
    }

    #[test]
    fn redirect_to_without_frame() {
        let expected = r#"<turbo-stream action="redirect_to" turbo-action="advance" url="/users/1"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::redirect_to("/users/1", "advance", None));
    }

    #[test]
    fn redirect_to_with_frame() {
        let expected = r#"<turbo-stream action="redirect_to" turbo-action="advance" turbo-frame="my_frame" url="/users/1"><template></template></turbo-stream>"#;
        assert_eq!(
            expected,
            super::redirect_to("/users/1", "advance", Some("my_frame"))
        );
    }

    #[test]
    fn reload() {
        let expected = r#"<turbo-stream action="reload"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::reload());
    }

    #[test]
    fn remove_attribute() {
        let expected = r##"<turbo-stream action="remove_attribute" attribute="data-controller" targets="#input"><template></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::remove_attribute("#input", "data-controller")
        );
    }

    #[test]
    fn remove_css_class() {
        let expected = r##"<turbo-stream action="remove_css_class" classes="container text-center" targets="#element"><template></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::remove_css_class("#element", "container text-center")
        );
    }

    #[test]
    fn remove_session_storage_item() {
        let expected = r#"<turbo-stream action="remove_storage_item" key="key" type="session"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::remove_session_storage_item("key"));
    }

    #[test]
    fn remove_local_storage_item() {
        let expected = r#"<turbo-stream action="remove_storage_item" key="key" type="local"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::remove_local_storage_item("key"));
    }

    #[test]
    fn remove_storage_item() {
        let expected = r#"<turbo-stream action="remove_storage_item" key="key" type="local"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::remove_storage_item("key", "local"));
    }

    #[test]
    fn replace_state() {
        let expected = r#"<turbo-stream action="replace_state" state="{}" title="title-1" url="/users/1"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::replace_state("/users/1", "title-1", "{}"));
    }

    #[test]
    fn reset_form() {
        let expected = r##"<turbo-stream action="reset_form" targets="#form"><template></template></turbo-stream>"##;
        assert_eq!(expected, super::reset_form("#form"));
    }

    #[test]
    fn scroll_into_view() {
        let expected = r##"<turbo-stream action="scroll_into_view" targets="#element"><template></template></turbo-stream>"##;
        assert_eq!(expected, super::scroll_into_view("#element"));
    }

    #[test]
    fn set_attribute() {
        let expected = r##"<turbo-stream action="set_attribute" attribute="data-controller" targets="#element" value="example"><template></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::set_attribute("#element", "data-controller", "example")
        );
    }

    #[test]
    fn set_cookie() {
        let expected = r#"<turbo-stream action="set_cookie" cookie="name=turbo_power; SameSite=None; Secure"><template></template></turbo-stream>"#;
        assert_eq!(
            expected,
            super::set_cookie("name=turbo_power; SameSite=None; Secure")
        );
    }

    #[test]
    fn set_cookie_item() {
        let expected = r#"<turbo-stream action="set_cookie_item" key="my-key" value="my-value"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::set_cookie_item("my-key", "my-value"));
    }

    #[test]
    fn set_dataset_attribute() {
        let expected = r##"<turbo-stream action="set_dataset_attribute" attribute="data-controller" targets="#element" value="example"><template></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::set_dataset_attribute("#element", "data-controller", "example")
        );
    }

    #[test]
    fn set_focus() {
        let expected = r##"<turbo-stream action="set_focus" targets="#input"><template></template></turbo-stream>"##;
        assert_eq!(expected, super::set_focus("#input"));
    }

    #[test]
    fn set_storage_local_item() {
        let expected = r#"<turbo-stream action="set_storage_item" key="my-key" type="local" value="my-value"><template></template></turbo-stream>"#;
        assert_eq!(
            expected,
            super::set_storage_local_item("my-key", "my-value")
        );
    }

    #[test]
    fn set_meta() {
        let expected = r#"<turbo-stream action="set_meta" content="initial-scale=1.0" name="viewport"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::set_meta("viewport", "initial-scale=1.0"));
    }

    #[test]
    fn set_property() {
        let expected = r##"<turbo-stream action="set_property" name="ariaDisabled" targets="#element" value="true"><template></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::set_property("#element", "ariaDisabled", "true")
        );
    }

    #[test]
    fn set_storage_session_item() {
        let expected = r#"<turbo-stream action="set_storage_item" key="my-key" type="session" value="my-value"><template></template></turbo-stream>"#;
        assert_eq!(
            expected,
            super::set_storage_session_item("my-key", "my-value")
        );
    }

    #[test]
    fn set_storage_item() {
        let expected = r#"<turbo-stream action="set_storage_item" key="my-key" type="local" value="my-value"><template></template></turbo-stream>"#;
        assert_eq!(
            expected,
            super::set_storage_item("my-key", "my-value", "local")
        );
    }

    #[test]
    fn set_style() {
        let expected = r##"<turbo-stream action="set_style" name="background" targets="#element" value="black"><template></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::set_style("#element", "background", "black")
        );
    }

    #[test]
    fn set_styles() {
        let expected = r##"<turbo-stream action="set_styles" styles="background: black; color: white" targets="#element"><template></template></turbo-stream>"##;
        assert_eq!(
            expected,
            super::set_styles("#element", "background: black; color: white")
        );
    }

    #[test]
    fn set_title() {
        let expected = r#"<turbo-stream action="set_title" title="My Title"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::set_title("My Title"));
    }

    #[test]
    fn set_value() {
        let expected = r##"<turbo-stream action="set_value" targets="#input" value="Value"><template></template></turbo-stream>"##;
        assert_eq!(expected, super::set_value("#input", "Value"));
    }

    #[test]
    fn text_content() {
        let expected = r##"<turbo-stream action="text_content" targets="#element" text="Text Content"><template></template></turbo-stream>"##;
        assert_eq!(expected, super::text_content("#element", "Text Content"));
    }

    #[test]
    fn turbo_clear_cache() {
        let expected =
            r#"<turbo-stream action="turbo_clear_cache"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::turbo_clear_cache());
    }

    #[test]
    fn turbo_frame_reload() {
        let expected = r#"<turbo-stream action="turbo_frame_reload" target="user_1"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::turbo_frame_reload("user_1"));
    }

    #[test]
    fn turbo_frame_set_src() {
        let expected = r#"<turbo-stream action="turbo_frame_set_src" src="/users" target="user_1"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::turbo_frame_set_src("user_1", "/users"));
    }

    #[test]
    fn turbo_progress_bar_hide() {
        let expected = r#"<turbo-stream action="turbo_progress_bar_hide"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::turbo_progress_bar_hide());
    }

    #[test]
    fn turbo_progress_bar_set_value() {
        let expected = r#"<turbo-stream action="turbo_progress_bar_set_value" value="0"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::turbo_progress_bar_set_value("0"));
    }

    #[test]
    fn turbo_progress_bar_show() {
        let expected = r#"<turbo-stream action="turbo_progress_bar_show"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::turbo_progress_bar_show());
    }
}
