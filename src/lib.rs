#![no_std]
#![doc = include_str!("../README.md")]

pub const QUESTION_SETS: &[(&str, &str)] = &[
    ("smiles-rs", include_str!("../questions/smiles-rs.md")),
    ("smarts", include_str!("../questions/smarts.md")),
    (
        "molecular-formulas",
        include_str!("../questions/molecular-formulas.md"),
    ),
    ("elements", include_str!("../questions/elements.md")),
];

/// Returns the Markdown body for an exact, case-sensitive question ID.
pub fn question(identifier: &str) -> Option<&'static str> {
    QUESTION_SETS
        .iter()
        .flat_map(|(_, document)| document.split("\n## ").skip(1))
        .find_map(|section| {
            let subtitle = section.lines().skip(1).find(|line| !line.is_empty())?;
            (subtitle.strip_prefix("### ") == Some(identifier)).then(|| section.trim_end())
        })
}
