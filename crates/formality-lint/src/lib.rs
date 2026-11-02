//! Regex-based checks inside `judgment_fn!` invocations.
//!
//! Unlike the coverage scraper, this checks test judgments too and reports
//! malformed source. No Rust or judgment DSL parser is involved.

use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
use regex::Regex;
use std::ops::Range;
use std::sync::LazyLock;

static MACRO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bjudgment_fn\s*!\s*([({\[])").unwrap());
static RAW_STRING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^(?:r|br|cr)(#*)""##).unwrap());
static CHAR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|[^\n])|[^'\\\n])'").unwrap()
});
static CLONE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:\.|::)\s*(?<name>clone)\s*\(").unwrap());

/// A source diagnostic with one-based line and character-column numbers.
#[derive(Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub line: usize,
    pub column: usize,
    pub rule: &'static str,
    pub message: &'static str,
    /// UTF-8 byte range in the original source, for source annotations.
    pub span: Range<usize>,
}

impl Diagnostic {
    /// Render a source annotation without terminal escape codes.
    ///
    /// `source` must be the same text that was passed to [`lint`].
    pub fn render(&self, path: &str, source: &str) -> String {
        let mut group = Level::ERROR
            .primary_title(self.message)
            .id(self.rule)
            .element(
                Snippet::source(source)
                    .path(path)
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(self.span.clone())),
            );
        if self.rule == "explicit-clone" {
            group = group.element(
                Level::HELP.message("use term constructors or implicit borrowing/casting"),
            );
        }
        Renderer::plain().render(&[group])
    }
}

fn diagnostic(
    source: &str,
    span: Range<usize>,
    rule: &'static str,
    message: &'static str,
) -> Diagnostic {
    let before = &source[..span.start];
    Diagnostic {
        line: before.bytes().filter(|b| *b == b'\n').count() + 1,
        column: before.rsplit('\n').next().unwrap().chars().count() + 1,
        rule,
        message,
        span,
    }
}

type ScanResult<T> = Result<T, (usize, &'static str)>;

/// Hide comments and literals without changing byte offsets or newlines.
fn mask_non_code(source: &str) -> ScanResult<String> {
    let bytes = source.as_bytes();
    let mut masked = bytes.to_vec();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        let rest = &source[i..];
        if rest.starts_with("//") {
            i += rest.find('\n').unwrap_or(rest.len());
        } else if rest.starts_with("/*") {
            i += 2;
            let mut depth = 1;
            while i < bytes.len() && depth > 0 {
                if bytes[i..].starts_with(b"/*") {
                    depth += 1;
                    i += 2;
                } else if bytes[i..].starts_with(b"*/") {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if depth > 0 {
                return Err((start, "unterminated block comment"));
            }
        } else if let Some(raw) = RAW_STRING.captures(rest).filter(|_| {
            !source[..i]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_')
        }) {
            let content = i + raw.get(0).unwrap().end();
            let terminator = format!("\"{}", &raw[1]);
            let end = source[content..]
                .find(&terminator)
                .ok_or((start, "unterminated raw string"))?;
            i = content + end + terminator.len();
        } else if bytes[i] == b'"' {
            i += 1;
            loop {
                match bytes.get(i) {
                    Some(b'\\') => i += 2,
                    Some(b'"') => {
                        i += 1;
                        break;
                    }
                    Some(_) => i += 1,
                    None => return Err((start, "unterminated string")),
                }
            }
        } else if let Some(character) = CHAR.find(rest) {
            // A lifetime such as 'a has no closing quote and is left alone.
            i += character.end();
        } else {
            i += rest.chars().next().unwrap().len_utf8();
            continue;
        }
        for byte in &mut masked[start..i] {
            if *byte != b'\n' {
                *byte = b' ';
            }
        }
    }
    // We replace complete comments/literals with ASCII and retain other UTF-8.
    Ok(String::from_utf8(masked).expect("masking preserves UTF-8"))
}

fn close_for(open: u8) -> Option<u8> {
    match open {
        b'(' => Some(b')'),
        b'[' => Some(b']'),
        b'{' => Some(b'}'),
        _ => None,
    }
}

fn body_end(code: &str, opening: usize) -> ScanResult<usize> {
    let mut stack = vec![close_for(code.as_bytes()[opening]).unwrap()];
    for (i, byte) in code.bytes().enumerate().skip(opening + 1) {
        if let Some(close) = close_for(byte) {
            stack.push(close);
        } else if matches!(byte, b')' | b']' | b'}') {
            if stack.pop() != Some(byte) {
                return Err((i, "mismatched delimiter in judgment_fn!"));
            }
            if stack.is_empty() {
                return Ok(i);
            }
        }
    }
    Err((opening, "unterminated judgment_fn! invocation"))
}

/// Check literal macro invocations, including those in test code.
pub fn lint(source: &str) -> Vec<Diagnostic> {
    let mut findings = Vec::new();
    let scan = || -> ScanResult<()> {
        let code = mask_non_code(source)?;
        let mut cursor = 0;
        while let Some(invocation) = MACRO.find_at(&code, cursor) {
            let opening = invocation.end() - 1;
            let end = body_end(&code, opening)?;
            for occurrence in CLONE.captures_iter(&code[opening + 1..end]) {
                let start = opening + 1 + occurrence.get(0).unwrap().start();
                let end = opening + 1 + occurrence.name("name").unwrap().end();
                findings.push(diagnostic(
                    source,
                    start..end,
                    "explicit-clone",
                    "explicit clone call in judgment",
                ));
            }
            cursor = end + 1;
        }
        Ok(())
    }();
    if let Err((offset, message)) = scan {
        let end = offset + source[offset..].chars().next().map_or(0, char::len_utf8);
        findings.push(diagnostic(source, offset..end, "scan-error", message));
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_clone_with_source_and_help() {
        let source = "judgment_fn! {\n    (let expr = value.clone())\n}\n";
        let finding = lint(source).remove(0);
        assert_eq!(&source[finding.span.clone()], ".clone");
        expect_test::expect![[r#"
            error[explicit-clone]: explicit clone call in judgment
             --> example.rs:2:22
              |
            2 |     (let expr = value.clone())
              |                      ^^^^^^
              |
              = help: use term constructors or implicit borrowing/casting"#]]
        .assert_eq(&finding.render("example.rs", source));
    }

    #[test]
    fn render_scan_error_at_end_of_file() {
        let source = "judgment_fn! {";
        let finding = lint(source).remove(0);
        expect_test::expect![[r#"
            error[scan-error]: unterminated judgment_fn! invocation
             --> example.rs:1:14
              |
            1 | judgment_fn! {
              |              ^"#]]
        .assert_eq(&finding.render("example.rs", source));
    }

    #[test]
    fn render_unicode_and_multiline_call() {
        let source = "judgment_fn! {\n    (let λ = value.\n        /* λ */ clone ())\n}\n";
        let finding = lint(source).remove(0);
        assert_eq!(&source[finding.span.clone()], ".\n        /* λ */ clone");
        expect_test::expect![[r#"
            error[explicit-clone]: explicit clone call in judgment
             --> unicode.rs:2:19
              |
            2 |       (let λ = value.
              |  ___________________^
            3 | |         /* λ */ clone ())
              | |_____________________^
              |
              = help: use term constructors or implicit borrowing/casting"#]]
        .assert_eq(&finding.render("unicode.rs", source));
    }

    #[test]
    fn render_multiple_findings_independently() {
        let source = "judgment_fn! {\n    (let a = left.clone())\n    (let b = right.clone())\n}";
        let findings = lint(source);
        assert_eq!(findings.len(), 2);
        let output = findings
            .iter()
            .map(|d| d.render("example.rs", source))
            .collect::<Vec<_>>()
            .join("\n\n");
        expect_test::expect![[r#"
            error[explicit-clone]: explicit clone call in judgment
             --> example.rs:2:18
              |
            2 |     (let a = left.clone())
              |                  ^^^^^^
              |
              = help: use term constructors or implicit borrowing/casting

            error[explicit-clone]: explicit clone call in judgment
             --> example.rs:3:19
              |
            3 |     (let b = right.clone())
              |                   ^^^^^^
              |
              = help: use term constructors or implicit borrowing/casting"#]]
        .assert_eq(&output);
    }

    #[test]
    fn only_judgment_bodies() {
        let source = "fn helper() { x.clone(); }\njudgment_fn! {\n    (let x = value.clone())\n}\nfn other() { y.clone(); }\njudgment_fn! { (let y = value.clone()) }\n";
        let found = lint(source);
        assert_eq!(
            found
                .iter()
                .map(|d| (d.line, d.column, d.rule))
                .collect::<Vec<_>>(),
            [(3, 19, "explicit-clone"), (6, 30, "explicit-clone")]
        );
    }

    #[test]
    fn delimiters_in_comments_and_literals() {
        let source = r###"// judgment_fn! { fake.clone() }
const EXAMPLE: &str = "judgment_fn! { fake.clone() }";
judgment_fn! {
    // } fake.clone()
    /* } /* nested */ fake.clone() */
    (let s = "\" } fake.clone()")
    (let r = r##"}"# fake.clone()"##)
    (let b = br#"} fake.clone()"#)
    (let c = c"} fake.clone()")
    (let ch = '}')
    (let ch = '\'')
    (let ch = '\u{7d}')
    (let byte = b'}')
    (let xs = [nested({ () })])
    (check(&'a value) => result)
    (let x = real.clone())
}
outside.clone();
"###;
        let findings = lint(source);
        assert_eq!(findings.len(), 1);
        assert_eq!((findings[0].line, findings[0].rule), (16, "explicit-clone"));
    }

    #[test]
    fn whitespace_and_macro_delimiters() {
        for (open, close) in [('(', ')'), ('[', ']'), ('{', '}')] {
            let source = format!("path::judgment_fn /*comment*/ ! {open}(let x = value . /* note */ clone\n ()){close}");
            assert_eq!(lint(&source).len(), 1);
            assert_eq!(lint(&source)[0].rule, "explicit-clone");
        }
    }

    #[test]
    fn constructors_and_other_macros_are_clean() {
        assert!(lint("judgment_fn! { (let x = Expr::bin_op(lhs, op, rhs)) }").is_empty());
        assert!(lint("other_judgment_fn! { x.clone() }").is_empty());
    }

    #[test]
    fn qualified_clone() {
        for expr in ["Clone::clone(value)", "<T as Clone>::clone(value)"] {
            let found = lint(&format!("judgment_fn! {{ {expr} }}"));
            assert_eq!(found.len(), 1);
            assert_eq!(found[0].rule, "explicit-clone");
        }
    }

    #[test]
    fn malformed_source_is_reported() {
        for source in [
            "judgment_fn! {",
            "judgment_fn! { (] }",
            "judgment_fn! { \"unterminated",
            "r##\"unterminated",
            "/*",
        ] {
            assert_eq!(lint(source).last().unwrap().rule, "scan-error", "{source}");
        }
    }

    #[test]
    fn unicode_offsets_are_preserved() {
        let source = "judgment_fn! { (let s = \"λ\") (let λ = value.clone()) }";
        let found = lint(source);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0].column,
            source[..source.find(".clone").unwrap()].chars().count() + 1
        );
    }
}
