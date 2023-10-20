use std::collections::BTreeMap;
pub mod power;
pub mod stream;
const STATIC_TAGS: &str = "<turbo-stream><template></template></turbo-stream>";
#[inline]
pub(crate) fn turbo_stream_action<'a, S: AsRef<str>>(
    action: &'a str,
    attributes: &mut BTreeMap<&str, &'a str>,
    content: S,
) -> String {
    attributes.insert("action", action);
    turbo_stream_tag(attributes, content)
}

#[inline]
pub(crate) fn turbo_stream_target<'a, S: AsRef<str>>(
    action: &'a str,
    target: &'a str,
    attributes: &mut BTreeMap<&str, &'a str>,
    content: S,
) -> String {
    attributes.insert("target", target);
    turbo_stream_action(action, attributes, content)
}

#[inline]
pub(crate) fn turbo_stream_target_all<'a, S: AsRef<str>>(
    action: &'a str,
    targets: &'a str,
    attributes: &mut BTreeMap<&str, &'a str>,
    content: S,
) -> String {
    attributes.insert("targets", targets);
    turbo_stream_action(action, attributes, content)
}

#[inline]
pub(crate) fn turbo_stream_tag<S: AsRef<str>>(
    attributes: &mut BTreeMap<&str, &str>,
    content: S,
) -> String {
    let content = content.as_ref();
    let attributes: BTreeMap<&str, String> = attributes
        .iter_mut()
        .map(|(key, value)| {
            let value = html_escape::encode_double_quoted_attribute(*value);
            (*key, value.to_string())
        })
        .collect();
    let attr_len: usize = attributes
        .iter()
        .map(|(key, value)| 4 + key.len() + value.len())
        .sum();
    let capacity = STATIC_TAGS.len() + attr_len + content.len();
    let mut output = String::with_capacity(capacity);
    output.push_str(r#"<turbo-stream"#);
    if attributes.is_empty() {
        output.push('>');
    } else {
        for (key, value) in attributes {
            output.push(' ');
            output.push_str(key);
            output.push_str(r#"=""#);
            output.push_str(value.as_str());
            output.push('"');
        }
        output.push('>');
    }
    output.push_str("<template>");
    output.push_str(content);
    output.push_str("</template></turbo-stream>");
    output
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn custom() {
        let expected =
            r#"<turbo-stream action="custom" target="target"><template></template></turbo-stream>"#;
        assert_eq!(
            expected,
            super::turbo_stream_target("custom", "target", &mut Default::default(), "")
        );
    }

    #[test]
    fn additional_attributes() {
        let expected = r#"<turbo-stream action="custom" foo="bar" hello="world" target="target"><template></template></turbo-stream>"#;
        let mut attributes = BTreeMap::from([("hello", "world"), ("foo", "bar")]);
        assert_eq!(
            expected,
            super::turbo_stream_target("custom", "target", &mut attributes, "")
        );
    }
}
