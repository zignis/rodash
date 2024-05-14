use once_cell::sync::Lazy;
use regex::{
    Captures,
    Regex,
};

/// Used to match HTML entities and HTML characters.
static ESCAPED_HTML: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"&(?:amp|lt|gt|quot|#(0+)?39);"#).expect("unable to build the pattern")
});

/// The inverse of [escape]. This method converts the HTML entities `&amp;`, `&lt;`, `&gt;`,
/// `&quot;` and `&#39;` in `value` to their corresponding characters.
///
/// # Note
///
/// No other characters are unescaped.
///
/// * `value` - The string to unescape.
///
/// # Examples
///
/// ```
/// use rodash::unescape;
///
/// assert_eq!(
///     unescape("fred, barney, &amp; pebbles"),
///     "fred, barney, & pebbles".to_string()
/// );
/// ```
///
/// [escape]: crate::escape
pub fn unescape<S>(value: S) -> String
where
    S: AsRef<str>,
{
    let value = value.as_ref();

    if ESCAPED_HTML.is_match(value) {
        ESCAPED_HTML
            .replace_all(value, |cap: &Captures| {
                let item = &cap[0];

                match item {
                    "&amp;" => "&",
                    "&lt;" => "<",
                    "&gt;" => ">",
                    "&quot;" => r#"""#,
                    _ => "'",
                }
                .to_owned()
            })
            .to_string()
    } else {
        value.to_string()
    }
}

/// A trait that implements the [Unescape::unescape] method on strings.
pub trait Unescape<S>
where
    S: AsRef<str> + ?Sized,
{
    /// The inverse of [escape]. This method converts the HTML entities `&amp;`, `&lt;`, `&gt;`,
    /// `&quot;` and `&#39;` in this string to their corresponding characters.
    ///
    /// # Note
    ///
    /// No other characters are unescaped.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Unescape;
    ///
    /// assert_eq!(
    ///     "fred, barney, &amp; pebbles".unescape(),
    ///     "fred, barney, & pebbles".to_string()
    /// );
    /// ```
    ///
    /// [escape]: crate::Escape::escape
    fn unescape(&self) -> String;
}

impl Unescape<String> for String {
    fn unescape(&self) -> String {
        unescape(self)
    }
}

impl Unescape<str> for str {
    fn unescape(&self) -> String {
        unescape(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        escape,
        Escape,
    };

    #[test]
    fn can_unescape_values_in_order() {
        assert_eq!(unescape(r#"&amp;lt;"#), "&lt;".to_string());
        assert_eq!(r#"&amp;lt;"#.unescape(), "&lt;".to_string());
    }

    #[test]
    fn can_unescape_values() {
        assert_eq!(
            unescape(r#"&amp;&lt;&gt;&quot;&#39;/&amp;&lt;&gt;&quot;&#39;/"#),
            r#"&<>"'/&<>"'/"#.to_string()
        );
        assert_eq!(
            unescape(escape(r#"&<>"'/&<>"'/"#)),
            r#"&<>"'/&<>"'/"#.to_string()
        );
        assert_eq!(unescape(r#"&#96;"#), r#"&#96;"#.to_string());
        assert_eq!(unescape(r#"&#x2F;"#), r#"&#x2F;"#.to_string());

        assert_eq!(
            r#"&amp;&lt;&gt;&quot;&#39;/&amp;&lt;&gt;&quot;&#39;/"#.unescape(),
            r#"&<>"'/&<>"'/"#.to_string()
        );
        assert_eq!(
            r#"&<>"'/&<>"'/"#.escape().unescape(),
            r#"&<>"'/&<>"'/"#.to_string()
        );
        assert_eq!(r#"&#96;"#.unescape(), r#"&#96;"#.to_string());
        assert_eq!(r#"&#x2F;"#.unescape(), r#"&#x2F;"#.to_string());
    }

    #[test]
    fn can_handle_strings_with_nothing_to_unescape() {
        assert_eq!(unescape("abc"), "abc".to_string());
        assert_eq!("abc".unescape(), "abc".to_string());
    }

    #[test]
    fn can_handle_leading_zeros_in_html_entities() {
        assert_eq!(unescape("&#39;"), "'".to_string());
        assert_eq!(unescape("&#039;"), "'".to_string());
        assert_eq!(unescape("&#000039;"), "'".to_string());

        assert_eq!("&#39;".unescape(), "'".to_string());
        assert_eq!("&#039;".unescape(), "'".to_string());
        assert_eq!("&#000039;".unescape(), "'".to_string());
    }
}
