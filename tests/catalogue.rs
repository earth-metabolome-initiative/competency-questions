use competency_questions::{QUESTION_SETS, question};
use std::collections::BTreeSet;

#[test]
fn every_question_has_one_address_and_a_nonempty_answer_contract() {
    let mut identifiers = BTreeSet::new();
    let mut names = BTreeSet::new();
    for &(name, document) in QUESTION_SETS {
        assert!(names.insert(name), "duplicate set {name}");
        let mut sections = document.split("\n## ").skip(1).peekable();
        assert!(sections.peek().is_some(), "empty set {name}");
        for section in sections {
            let mut lines = section.lines();
            let title = lines.next().expect("expected a question title");
            assert!(!title.trim().is_empty());
            let identifier = lines
                .find(|line| !line.is_empty())
                .and_then(|line| line.strip_prefix("### "))
                .expect("expected an ID subtitle");
            let digits = identifier
                .strip_prefix("CQ-")
                .expect("expected CQ- heading");
            assert!(digits.len() >= 4 && digits.bytes().all(|digit| digit.is_ascii_digit()));
            assert!(
                identifiers.insert(identifier),
                "duplicate question {identifier}"
            );
            let body = question(identifier).expect("unresolvable question");
            assert_eq!(body.lines().next(), Some(title));
            assert!(!body.contains("\n## "), "{identifier}");
        }
    }
    assert_eq!(question("CQ-"), None);
    assert_eq!(question(""), None);
}
