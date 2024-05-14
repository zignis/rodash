use once_cell::sync::Lazy;
use regex::{
    Captures,
    Regex,
};

/// Used to match HTML entities and HTML characters.
static UNESCAPED_HTML: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[&<>"']"#).expect("unable to build the pattern"));

/// Converts the characters "&", "<", ">", '"', and "'" in `value` to their corresponding HTML
/// entities.
///
/// Though the ">" character is escaped for symmetry, characters like ">" and "/" don't need
/// escaping in HTML and have no special meaning unless they're part of a tag or unquoted attribute
/// value. See [Mathias Bynens's article](https://mathiasbynens.be/notes/ambiguous-ampersands)
/// (under "semi-related fun fact") for more details.
///
/// When working with HTML you should always [quote attribute values](http://wonko.com/post/html-escaping)
/// to reduce XSS vectors.
///
/// # Note
///
/// No other characters are escaped.
///
/// * `value` - The string to escape.
///
/// # Examples
///
/// ```
/// use rodash::escape;
///
/// assert_eq!(
///     escape("fred, barney, & pebbles"),
///     "fred, barney, &amp; pebbles".to_string()
/// );
/// ```
pub fn escape<S>(value: S) -> String
where
    S: AsRef<str>,
{
    let value = value.as_ref();

    if UNESCAPED_HTML.is_match(value) {
        UNESCAPED_HTML
            .replace_all(value, |cap: &Captures| {
                let item = &cap[0];

                match item {
                    "&" => "&amp;",
                    "<" => "&lt;",
                    ">" => "&gt;",
                    r#"""# => "&quot;",
                    "'" => "&#39;",
                    _ => item,
                }
                .to_owned()
            })
            .to_string()
    } else {
        value.to_string()
    }
}

/// A trait that implements the [Escape::escape] method on strings.
pub trait Escape<S>
where
    S: AsRef<str> + ?Sized,
{
    /// Converts the characters "&", "<", ">", '"', and "'" in this string to their corresponding
    /// HTML entities.
    ///
    /// Though the ">" character is escaped for symmetry, characters like ">" and "/" don't need
    /// escaping in HTML and have no special meaning unless they're part of a tag or unquoted
    /// attribute value. See [Mathias Bynens's article](https://mathiasbynens.be/notes/ambiguous-ampersands)
    /// (under "semi-related fun fact") for more details.
    ///
    /// When working with HTML you should always [quote attribute values](http://wonko.com/post/html-escaping)
    /// to reduce XSS vectors.
    ///
    /// # Note
    ///
    /// No other characters are escaped.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Escape;
    ///
    /// assert_eq!(
    ///     "fred, barney, & pebbles".escape(),
    ///     "fred, barney, &amp; pebbles".to_string()
    /// );
    /// ```
    fn escape(&self) -> String;
}

impl Escape<String> for String {
    fn escape(&self) -> String {
        escape(self)
    }
}

impl Escape<str> for str {
    fn escape(&self) -> String {
        escape(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        unescape,
        Unescape,
    };

    #[test]
    fn can_escape_values() {
        assert_eq!(
            escape(r#"&<>"'/&<>"'/"#),
            "&amp;&lt;&gt;&quot;&#39;/&amp;&lt;&gt;&quot;&#39;/".to_string()
        );
        assert_eq!(escape(r#"`"#), "`".to_string());
        assert_eq!(escape(r#"/"#), "/".to_string());
        assert_eq!(
            escape(unescape(
                r#"&amp;&lt;&gt;&quot;&#39;/&amp;&lt;&gt;&quot;&#39;/"#
            )),
            "&amp;&lt;&gt;&quot;&#39;/&amp;&lt;&gt;&quot;&#39;/".to_string()
        );

        assert_eq!(
            r#"&<>"'/&<>"'/"#.escape(),
            "&amp;&lt;&gt;&quot;&#39;/&amp;&lt;&gt;&quot;&#39;/".to_string()
        );
        assert_eq!(r#"`"#.escape(), "`".to_string());
        assert_eq!(r#"/"#.escape(), "/".to_string());
        assert_eq!(
            r#"&amp;&lt;&gt;&quot;&#39;/&amp;&lt;&gt;&quot;&#39;/"#
                .unescape()
                .escape(),
            "&amp;&lt;&gt;&quot;&#39;/&amp;&lt;&gt;&quot;&#39;/".to_string()
        );
    }

    #[test]
    fn can_handle_strings_with_nothing_to_escape() {
        assert_eq!(escape("abc"), "abc".to_string());
        assert_eq!("abc".escape(), "abc".to_string());
    }
}
