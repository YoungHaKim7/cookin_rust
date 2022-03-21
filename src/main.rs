// background on error handling in Rust, read this page of the Rust book and this blog post.
use std::result::Result;
use url::{Position, Url};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
} 
