use competency_questions::QUESTION_SETS;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::{Command, ExitCode};

const USAGE: &str = "Usage: cargo run -- add <set> \"Question text?\"";
const REPOSITORY: &str = "earth-metabolome-initiative/competency-questions";
const LEGACY_QUESTIONS: u64 = 14;

fn reserve_issue(set_name: &str, question: &str) -> io::Result<u64> {
    let title: String = question.chars().take(200).collect();
    let output = Command::new("timeout")
        .args(["30s", "gh", "api"])
        .arg(format!("repos/{REPOSITORY}/issues"))
        .args(["--hostname", "github.com", "--method", "POST", "-f"])
        .arg(format!("title={title}"))
        .arg("-f")
        .arg(format!("body=Question set: `{set_name}`\n\n{question}"))
        .args(["--jq", ".number"])
        .output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "GitHub reservation failed ({}). Check the issue tracker before retrying. {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u64>()
        .ok()
        .filter(|number| *number > 0 && *number <= u64::MAX - LEGACY_QUESTIONS)
        .ok_or_else(|| io::Error::other("Invalid issue number. Check the tracker before retrying."))
}

fn add(set_name: &str, question: &str) -> io::Result<()> {
    if !QUESTION_SETS.iter().any(|(name, _)| *name == set_name) {
        return Err(io::Error::other(format!("Unknown question set {set_name}")));
    }
    let question = question.trim();
    if question.is_empty() || question.starts_with("## ") || question.chars().any(char::is_control)
    {
        return Err(io::Error::other("Supply the question as one line of text"));
    }
    let path = format!("questions/{set_name}.md");
    let mut file = OpenOptions::new()
        .append(true)
        .open(&path)
        .map_err(|error| {
            io::Error::other(format!(
                "Cannot append to {path}. Run from the catalogue root. {error}"
            ))
        })?;
    file.try_lock()
        .map_err(|error| io::Error::other(format!("Cannot lock {path}: {error}")))?;
    let number = reserve_issue(set_name, question)?;
    let identifier = format!("CQ-{:04}", number + LEGACY_QUESTIONS);
    let link = format!("https://github.com/{REPOSITORY}/issues/{number}");
    let entry = format!("\n## {identifier}\n\n{question}\n\n[Review question]({link})\n");
    file.write_all(entry.as_bytes())
        .and_then(|()| file.sync_data())
        .map_err(|error| {
            io::Error::other(format!(
                "{identifier} reserved at {link}. Complete the entry in {path} manually. {error}"
            ))
        })?;
    println!("{identifier}  {path}  {link}");
    Ok(())
}

fn main() -> ExitCode {
    let mut arguments = std::env::args().skip(1);
    match (
        arguments.next().as_deref(),
        arguments.next(),
        arguments.next(),
        arguments.next(),
    ) {
        (Some("add"), Some(set_name), Some(question), None) => match add(&set_name, &question) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        },
        (Some("--help" | "-h"), None, None, None) => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::FAILURE
        }
    }
}
