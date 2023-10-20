use crate::{turbo_stream_target, turbo_stream_target_all};

// Removes the <tt>target</tt> from the dom. The target can either be a dom id string or an object that responds to
// <tt>to_key</tt>, which is then called and passed through <tt>ActionView::RecordIdentifier.dom_id</tt> (all Active Records
// do). Examples:
//
//   <%= turbo_stream.remove "clearance_5" %>
//   <%= turbo_stream.remove clearance %>
pub fn remove<S: AsRef<str>>(target: S) -> String {
    turbo_stream_target("remove", target.as_ref(), &mut Default::default(), "")
}

// Removes the <tt>targets</tt> from the dom. The targets can either be a CSS selector string or an object that responds to
// <tt>to_key</tt>, which is then called and passed through <tt>ActionView::RecordIdentifier.dom_id</tt> (all Active Records
// do). Examples:
//
//   <%= turbo_stream.remove_all ".clearance_item" %>
//   <%= turbo_stream.remove_all clearance %>
pub fn remove_all(targets: &str) -> String {
    turbo_stream_target_all("remove", targets, &mut Default::default(), "")
}

// Replace the <tt>target</tt> in the dom with either the <tt>content</tt> passed in, a rendering result determined
// by the <tt>rendering</tt> keyword arguments, the content in the block, or the rendering of the target as a record. Examples:
//
//   <%= turbo_stream.replace "clearance_5", "<div id='clearance_5'>Replace the dom target identified by clearance_5</div>" %>
//   <%= turbo_stream.replace clearance %>
//   <%= turbo_stream.replace clearance, partial: "clearances/clearance", locals: { title: "Hello" } %>
//   <%= turbo_stream.replace "clearance_5" do %>
//     <div id='clearance_5'>Replace the dom target identified by clearance_5</div>
//   <% end %>
pub fn replace<S: AsRef<str>>(target: &str, content: S) -> String {
    turbo_stream_target("replace", target, &mut Default::default(), content)
}

// Replace the <tt>targets</tt> in the dom with either the <tt>content</tt> passed in, a rendering result determined
// by the <tt>rendering</tt> keyword arguments, the content in the block, or the rendering of the target as a record. Examples:
//
//   <%= turbo_stream.replace_all ".clearance_item", "<div class='clearance_item'>Replace the dom target identified by the class clearance_item</div>" %>
//   <%= turbo_stream.replace_all clearance %>
//   <%= turbo_stream.replace_all clearance, partial: "clearances/clearance", locals: { title: "Hello" } %>
//   <%= turbo_stream.replace_all ".clearance_item" do %>
//     <div class='.clearance_item'>Replace the dom target identified by the class clearance_item</div>
//   <% end %>
pub fn replace_all<S: AsRef<str>>(targets: &str, content: S) -> String {
    turbo_stream_target_all("replace", targets, &mut Default::default(), content)
}

// Insert the <tt>content</tt> passed in, a rendering result determined by the <tt>rendering</tt> keyword arguments,
// the content in the block, or the rendering of the target as a record before the <tt>target</tt> in the dom. Examples:
//
//   <%= turbo_stream.before "clearance_5", "<div id='clearance_4'>Insert before the dom target identified by clearance_5</div>" %>
//   <%= turbo_stream.before clearance %>
//   <%= turbo_stream.before clearance, partial: "clearances/clearance", locals: { title: "Hello" } %>
//   <%= turbo_stream.before "clearance_5" do %>
//     <div id='clearance_4'>Insert before the dom target identified by clearance_5</div>
//   <% end %>
pub fn before<S: AsRef<str>>(target: &str, content: S) -> String {
    turbo_stream_target("before", target, &mut Default::default(), content)
}

// Insert the <tt>content</tt> passed in, a rendering result determined by the <tt>rendering</tt> keyword arguments,
// the content in the block, or the rendering of the target as a record before the <tt>targets</tt> in the dom. Examples:
//
//   <%= turbo_stream.before_all ".clearance_item", "<div class='clearance_item'>Insert before the dom target identified by the class clearance_item</div>" %>
//   <%= turbo_stream.before_all clearance %>
//   <%= turbo_stream.before_all clearance, partial: "clearances/clearance", locals: { title: "Hello" } %>
//   <%= turbo_stream.before_all ".clearance_item" do %>
//     <div class='clearance_item'>Insert before the dom target identified by clearance_item</div>
//   <% end %>
pub fn before_all<S: AsRef<str>>(targets: &str, content: S) -> String {
    turbo_stream_target_all("before", targets, &mut Default::default(), content)
}

// Insert the <tt>content</tt> passed in, a rendering result determined by the <tt>rendering</tt> keyword arguments,
// the content in the block, or the rendering of the target as a record after the <tt>target</tt> in the dom. Examples:
//
//   <%= turbo_stream.after "clearance_5", "<div id='clearance_6'>Insert after the dom target identified by clearance_5</div>" %>
//   <%= turbo_stream.after clearance %>
//   <%= turbo_stream.after clearance, partial: "clearances/clearance", locals: { title: "Hello" } %>
//   <%= turbo_stream.after "clearance_5" do %>
//     <div id='clearance_6'>Insert after the dom target identified by clearance_5</div>
//   <% end %>
pub fn after<S: AsRef<str>>(target: &str, content: S) -> String {
    turbo_stream_target("after", target, &mut Default::default(), content)
}

// Insert the <tt>content</tt> passed in, a rendering result determined by the <tt>rendering</tt> keyword arguments,
// the content in the block, or the rendering of the target as a record after the <tt>targets</tt> in the dom. Examples:
//
//   <%= turbo_stream.after_all ".clearance_item", "<div class='clearance_item'>Insert after the dom target identified by the class clearance_item</div>" %>
//   <%= turbo_stream.after_all clearance %>
//   <%= turbo_stream.after_all clearance, partial: "clearances/clearance", locals: { title: "Hello" } %>
//   <%= turbo_stream.after_all "clearance_item" do %>
//     <div class='clearance_item'>Insert after the dom target identified by the class clearance_item</div>
//   <% end %>
pub fn after_all<S: AsRef<str>>(targets: &str, content: S) -> String {
    turbo_stream_target_all("after", targets, &mut Default::default(), content)
}

// Update the <tt>target</tt> in the dom with either the <tt>content</tt> passed in or a rendering result determined
// by the <tt>rendering</tt> keyword arguments, the content in the block, or the rendering of the target as a record. Examples:
//
//   <%= turbo_stream.update "clearance_5", "Update the content of the dom target identified by clearance_5" %>
//   <%= turbo_stream.update clearance %>
//   <%= turbo_stream.update clearance, partial: "clearances/unique_clearance", locals: { title: "Hello" } %>
//   <%= turbo_stream.update "clearance_5" do %>
//     Update the content of the dom target identified by clearance_5
//   <% end %>
pub fn update<S: AsRef<str>>(target: &str, content: S) -> String {
    turbo_stream_target("update", target, &mut Default::default(), content)
}

// Update the <tt>targets</tt> in the dom with either the <tt>content</tt> passed in or a rendering result determined
// by the <tt>rendering</tt> keyword arguments, the content in the block, or the rendering of the targets as a record. Examples:
//
//   <%= turbo_stream.update_all "clearance_item", "Update the content of the dom target identified by the class clearance_item" %>
//   <%= turbo_stream.update_all clearance %>
//   <%= turbo_stream.update_all clearance, partial: "clearances/new_clearance", locals: { title: "Hello" } %>
//   <%= turbo_stream.update_all "clearance_item" do %>
//     Update the content of the dom target identified by the class clearance_item
//   <% end %>
pub fn update_all<S: AsRef<str>>(targets: &str, content: S) -> String {
    turbo_stream_target_all("update", targets, &mut Default::default(), content)
}

// Append to the target in the dom identified with <tt>target</tt> either the <tt>content</tt> passed in or a
// rendering result determined by the <tt>rendering</tt> keyword arguments, the content in the block,
// or the rendering of the content as a record. Examples:
//
//   <%= turbo_stream.append "clearances", "<div id='clearance_5'>Append this to .clearances</div>" %>
//   <%= turbo_stream.append "clearances", clearance %>
//   <%= turbo_stream.append "clearances", partial: "clearances/unique_clearance", locals: { clearance: clearance } %>
//   <%= turbo_stream.append "clearances" do %>
//     <div id='clearance_5'>Append this to .clearances</div>
//   <% end %>
pub fn append<S: AsRef<str>>(target: &str, content: S) -> String {
    turbo_stream_target("append", target, &mut Default::default(), content)
}

// Append to the targets in the dom identified with <tt>targets</tt> either the <tt>content</tt> passed in or a
// rendering result determined by the <tt>rendering</tt> keyword arguments, the content in the block,
// or the rendering of the content as a record. Examples:
//
//   <%= turbo_stream.append_all ".clearances", "<div class='clearance_item'>Append this to .clearance_group</div>" %>
//   <%= turbo_stream.append_all ".clearances", clearance %>
//   <%= turbo_stream.append_all ".clearances", partial: "clearances/new_clearance", locals: { clearance: clearance } %>
//   <%= turbo_stream.append_all ".clearances" do %>
//     <div id='clearance_item'>Append this to .clearances</div>
//   <% end %>
pub fn append_all<S: AsRef<str>>(targets: &str, content: S) -> String {
    turbo_stream_target_all("append", targets, &mut Default::default(), content)
}

// Prepend to the target in the dom identified with <tt>target</tt> either the <tt>content</tt> passed in or a
// rendering result determined by the <tt>rendering</tt> keyword arguments or the content in the block,
// or the rendering of the content as a record. Examples:
//
//   <%= turbo_stream.prepend "clearances", "<div id='clearance_5'>Prepend this to .clearances</div>" %>
//   <%= turbo_stream.prepend "clearances", clearance %>
//   <%= turbo_stream.prepend "clearances", partial: "clearances/unique_clearance", locals: { clearance: clearance } %>
//   <%= turbo_stream.prepend "clearances" do %>
//     <div id='clearance_5'>Prepend this to .clearances</div>
//   <% end %>
pub fn prepend<S: AsRef<str>>(target: &str, content: S) -> String {
    turbo_stream_target("prepend", target, &mut Default::default(), content)
}

// Prepend to the targets in the dom identified with <tt>targets</tt> either the <tt>content</tt> passed in or a
// rendering result determined by the <tt>rendering</tt> keyword arguments or the content in the block,
// or the rendering of the content as a record. Examples:
//
//   <%= turbo_stream.prepend_all ".clearances", "<div class='clearance_item'>Prepend this to .clearances</div>" %>
//   <%= turbo_stream.prepend_all ".clearances", clearance %>
//   <%= turbo_stream.prepend_all ".clearances", partial: "clearances/new_clearance", locals: { clearance: clearance } %>
//   <%= turbo_stream.prepend_all ".clearances" do %>
//     <div class='clearance_item'>Prepend this to .clearances</div>
//   <% end %>
pub fn prepend_all<S: AsRef<str>>(targets: &str, content: S) -> String {
    turbo_stream_target_all("prepend", targets, &mut Default::default(), content)
}

#[cfg(test)]
mod tests {

    #[test]
    fn empty_content() {
        let expected = r#"<turbo-stream action="append" target="message_1"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::append("message_1", ""));
    }

    #[test]
    fn remove() {
        let expected = r#"<turbo-stream action="remove" target="message_1"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::remove("message_1"));
    }

    #[test]
    fn remove_all() {
        let expected = r#"<turbo-stream action="remove" targets=".message"><template></template></turbo-stream>"#;
        assert_eq!(expected, super::remove_all(".message"));
    }

    #[test]
    fn replace() {
        let expected = r#"<turbo-stream action="replace" target="message_1"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::replace("message_1", "Test"));
    }

    #[test]
    fn replace_all() {
        let expected = r#"<turbo-stream action="replace" targets=".message"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::replace_all(".message", "Test"));
    }

    #[test]
    fn before() {
        let expected = r#"<turbo-stream action="before" target="message_1"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::before("message_1", "Test"));
    }

    #[test]
    fn before_all() {
        let expected = r#"<turbo-stream action="before" targets=".message"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::before_all(".message", "Test"));
    }

    #[test]
    fn after() {
        let expected = r#"<turbo-stream action="after" target="message_1"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::after("message_1", "Test"));
    }

    #[test]
    fn after_all() {
        let expected = r#"<turbo-stream action="after" targets=".message"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::after_all(".message", "Test"));
    }

    #[test]
    fn update() {
        let expected = r#"<turbo-stream action="update" target="message_1"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::update("message_1", "Test"));
    }

    #[test]
    fn update_all() {
        let expected = r#"<turbo-stream action="update" targets=".message"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::update_all(".message", "Test"));
    }

    #[test]
    fn append() {
        let expected = r#"<turbo-stream action="append" target="message_1"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::append("message_1", "Test"));
    }

    #[test]
    fn append_all() {
        let expected = r#"<turbo-stream action="append" targets=".message"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::append_all(".message", "Test"));
    }

    #[test]
    fn prepend() {
        let expected = r#"<turbo-stream action="prepend" target="message_1"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::prepend("message_1", "Test"));
    }

    #[test]
    fn prepend_all() {
        let expected = r#"<turbo-stream action="prepend" targets=".message"><template>Test</template></turbo-stream>"#;
        assert_eq!(expected, super::prepend_all(".message", "Test"));
    }
}
