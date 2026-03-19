use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;

const SYSTEM_PROMPT: &str = "\
You are a Rust code generator embedded inside a procedural macro. \
Output ONLY valid Rust code. No markdown fences, no explanations, no comments \
unless the user's prompt explicitly asks for them. \
The output will be parsed directly by the Rust compiler. \
Do not include `fn main` unless asked.";

/// Returns the cache directory under `target/tlapyen-cache/`.
fn cache_dir() -> PathBuf {
    PathBuf::from("target").join("tlapyen-cache")
}

/// Hashes the prompt to produce a deterministic cache key.
/// (The word "deterministic" here is doing a lot of heavy lifting.)
fn hash_prompt(prompt: &str) -> String {
    let mut hasher = DefaultHasher::new();
    prompt.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Checks the cache for a previously generated result.
pub fn check_cache(prompt: &str) -> Option<String> {
    let path = cache_dir().join(hash_prompt(prompt));
    fs::read_to_string(path).ok()
}

/// Writes generated code to the cache.
pub fn write_cache(prompt: &str, code: &str) {
    let dir = cache_dir();
    let _ = fs::create_dir_all(&dir);
    let path = dir.join(hash_prompt(prompt));
    let _ = fs::write(path, code);
}

/// Invokes the `claude` CLI with the given prompt.
///
/// Returns the generated Rust code, or an error message that will
/// brighten your day (but not fix your build).
pub fn invoke_claude(prompt: &str) -> Result<String, String> {
    let result = Command::new("claude")
        .arg("-p")
        .arg(prompt)
        .arg("--system-prompt")
        .arg(SYSTEM_PROMPT)
        .arg("--allowedTools")
        .arg("")
        .arg("--output-format")
        .arg("text")
        .output();

    match result {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!(
                    "Claude CLI exited with {}: {}\n\
                     This is what happens when you let an AI write your code.",
                    output.status, stderr
                ));
            }

            let code = String::from_utf8_lossy(&output.stdout).to_string();
            let code = code.trim().to_string();

            if code.is_empty() {
                return Err(
                    "Claude returned nothing. The silent treatment. \
                     Try rephrasing your prompt, or consider that maybe \
                     Claude has opinions about your code."
                        .to_string(),
                );
            }

            Ok(code)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(
            "The `claude` CLI was not found on your PATH.\n\
             Install it: npm install -g @anthropic-ai/claude-code\n\
             Yes, your Rust build now depends on npm. You're welcome."
                .to_string(),
        ),
        Err(e) => Err(format!(
            "Failed to invoke `claude`: {}\n\
             Have you tried turning your compiler off and on again?",
            e
        )),
    }
}
