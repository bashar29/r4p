use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Deserialize)]
pub struct CompanyName(String);

impl CorrectName for CompanyName {}

impl AsRef<str> for CompanyName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CompanyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We just forward to the Display implementation of
        // the wrapped String.
        self.0.fmt(f)
    }
}

impl CompanyName {
    pub fn parse(s: String) -> Result<CompanyName, String> {
        let r = <CompanyName as CorrectName>::validate_name(s);
        match r {
            Ok(valid_name) => Ok(Self(valid_name)),
            Err(msg) => Err(msg),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PeopleName(String);

impl CorrectName for PeopleName {}

impl AsRef<str> for PeopleName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PeopleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We just forward to the Display implementation of
        // the wrapped String.
        self.0.fmt(f)
    }
}

impl PeopleName {
    pub fn parse(s: String) -> Result<PeopleName, String> {
        let r = <PeopleName as CorrectName>::validate_name(s);
        match r {
            Ok(valid_name) => Ok(Self(valid_name)),
            Err(msg) => Err(msg),
        }
    }
}

pub trait CorrectName {
    fn validate_name(s: String) -> Result<String, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(s)
        }
    }
}

#[derive(Debug)]
pub struct NewAccount {
    company_name: CompanyName,
    admin_lastname: PeopleName,
    admin_firstname: PeopleName,
    admin_email: String,
}
