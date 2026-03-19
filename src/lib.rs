//! # TLAPYEN — The Last Program You'll Ever Need
//!
//! A proc macro that calls Claude Code at compile time to generate Rust code
//! from natural language descriptions.
//!
//! ## Requirements
//!
//! - The `claude` CLI (`npm install -g @anthropic-ai/claude-code`)
//! - An internet connection
//! - Faith
//!
//! ## Usage
//!
//! ```rust,ignore
//! use tlapyen::tlapyen;
//!
//! tlapyen!("A function called add that takes two i32s and returns their sum");
//!
//! fn main() {
//!     assert_eq!(add(2, 3), 5); // ...probably
//! }
//! ```

mod claude;
mod strip;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

/// Generates Rust code from a natural language description by calling Claude
/// Code at compile time.
///
/// Pass a string literal describing the code you want. Claude will generate it.
/// The result is cached in `target/tlapyen-cache/` so subsequent builds don't
/// re-invoke Claude (unless you change the prompt or run `cargo clean`).
///
/// # Panics
///
/// When Claude is feeling uncooperative, when you don't have WiFi, or when the
/// generated code doesn't parse. So, you know, sometimes.
#[proc_macro]
pub fn tlapyen(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let prompt = lit.value();

    // Check cache first — determinism is a spectrum
    if let Some(cached) = claude::check_cache(&prompt) {
        return match cached.parse::<TokenStream>() {
            Ok(ts) => ts,
            Err(_) => {
                // Cache is corrupted somehow. Nuke it and regenerate.
                let _ = std::fs::remove_dir_all("target/tlapyen-cache");
                generate_and_cache(&prompt, &lit)
            }
        };
    }

    generate_and_cache(&prompt, &lit)
}

fn generate_and_cache(prompt: &str, lit: &LitStr) -> TokenStream {
    let raw = match claude::invoke_claude(prompt) {
        Ok(code) => code,
        Err(e) => {
            return syn::Error::new(lit.span(), e)
                .to_compile_error()
                .into();
        }
    };

    let code = strip::strip_markdown_fences(&raw);

    // Try to parse what Claude gave us
    match code.parse::<TokenStream>() {
        Ok(ts) => {
            claude::write_cache(prompt, &code);
            ts
        }
        Err(e) => {
            let msg = format!(
                "Claude generated code that doesn't parse. Shocking, I know.\n\
                 \n\
                 Parse error: {}\n\
                 \n\
                 Claude's output:\n\
                 ────────────────\n\
                 {}\n\
                 ────────────────\n\
                 \n\
                 Try refining your prompt. Or don't. I'm a macro, not a cop.",
                e, code
            );
            syn::Error::new(lit.span(), msg)
                .to_compile_error()
                .into()
        }
    }
}
