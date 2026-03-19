/// Strips markdown code fences from Claude's output.
///
/// Claude loves wrapping things in ```rust ... ``` even when explicitly told
/// not to. This is our safety net.
pub fn strip_markdown_fences(input: &str) -> String {
    let trimmed = input.trim();

    // Try to strip ```rust or ``` opening fence and ``` closing fence
    let without_open = trimmed
        .strip_prefix("```rust")
        .or_else(|| trimmed.strip_prefix("```"))
        .unwrap_or(trimmed);

    let without_both = without_open
        .strip_suffix("```")
        .unwrap_or(without_open);

    without_both.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_rust_fences() {
        let input = "```rust\nfn hello() {}\n```";
        assert_eq!(strip_markdown_fences(input), "fn hello() {}");
    }

    #[test]
    fn strips_plain_fences() {
        let input = "```\nfn hello() {}\n```";
        assert_eq!(strip_markdown_fences(input), "fn hello() {}");
    }

    #[test]
    fn leaves_clean_code_alone() {
        let input = "fn hello() {}";
        assert_eq!(strip_markdown_fences(input), "fn hello() {}");
    }

    #[test]
    fn handles_whitespace() {
        let input = "  ```rust\n  fn hello() {}\n  ```  ";
        assert_eq!(strip_markdown_fences(input), "fn hello() {}");
    }
}
