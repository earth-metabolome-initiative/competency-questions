# Chemical competency questions

![Review status](https://img.shields.io/badge/questions-draft-blue)

What should a natural-products researcher be able to retrieve from a dataset? Each entry gives a question, the evidence needed to answer it, and what one answer contains. These are proposed retrieval requirements for expert review. The crate distributes the questions as plain text without a chemistry library, database or network connection.

| Review set | Focus |
| --- | --- |
| [`smiles-rs`](questions/smiles-rs.md) | Identity and comparison of reported structures |
| [`smarts`](questions/smarts.md) | Finding structural motifs |
| [`molecular-formulas`](questions/molecular-formulas.md) | Composition, candidate formulas and spectral assignments |
| [`elements`](questions/elements.md) | Element and isotope evidence |

The questions suggest different indexes. [`CQ-0007`](questions/molecular-formulas.md#cq-0007) asks which analyte formulas contain fluorine. [`CQ-0014`](questions/molecular-formulas.md#cq-0014) also needs atom counts to retrieve formulas with at least four nitrogen atoms and no sulfur. [`CQ-0013`](questions/molecular-formulas.md#cq-0013) needs a link from an isolated product ion to the scan of its products. An index of peak masses alone cannot recover that acquisition history.

## Add a question

From the root of a writable checkout, choose a set from the table above and pass the question as one quoted argument. For example, run this **only if you want to submit this question**.

```sh
cargo run -- add molecular-formulas "Which chemical entities contain exactly two chlorine atoms?"
```

You need Rust and Cargo, GNU `timeout`, network access, and the GitHub CLI (`gh`) signed in to an account that can open issues in the [EMI catalogue](https://github.com/earth-metabolome-initiative/competency-questions/issues). The command immediately posts the question and set **publicly** to that tracker, including when run from a fork. It appends the question to the local set file and prints its permanent ID, file path and issue link. It does not commit or push the file.

Share the ID or issue link as soon as the command succeeds. In the printed file, add what counts as a match, what one result should contain, and any supporting example or citation. Submit that file through the usual review process. To clarify an existing question, edit its entry and keep its ID.

GitHub allocates issue numbers centrally across branches and clones. The question number is the issue number plus 14, preserving `CQ-0001` through `CQ-0014`. For example, issue 1 gives `CQ-0015`. Other issues and pull requests can leave gaps. Never fill gaps or reuse a retired ID. If a request is interrupted, check the tracker for the question before retrying. If the issue exists, use its number to complete the local entry with the reserved ID and issue link.

## Cite or amend a question

Use [`CQ-0007`](questions/molecular-formulas.md#cq-0007) to discuss the fluorine question. Use `questions/molecular-formulas.md` to assign the whole set, or list individual IDs across sets. A question has one home even when answering it requires several crates. Search the checkout with `git grep --untracked -n '^## CQ-0007$' -- questions`.

For an exact handoff, give the recipient the repository location and full commit hash alongside the IDs or set path. In that checkout, `git show <commit>:questions/molecular-formulas.md` retrieves that revision of the set. A bare ID follows the question through wording edits. For an uncommitted draft, share the working files with the recipient.

To comment, quote the ID and give an example that should match or should be excluded. A different retrieval task gets a new ID. Leave a forwarding note under a retired ID.

```rust
use competency_questions::{question, QUESTION_SETS};

let fluorine = question("CQ-0007").ok_or("unknown question")?;
assert!(fluorine.to_lowercase().contains("fluorine"));
assert!(QUESTION_SETS.iter().any(|(name, _)| *name == "elements"));
# Ok::<(), &'static str>(())
```

## Why capture the question first?

[Grüninger and Fox’s methodology](https://eil.mie.utoronto.ca/theory/enterprise-modelling/entmethod/) derives informal competency questions from motivating scenarios. [Wiśniewski et al.](https://arxiv.org/abs/1811.09529) found that a single question pattern can have several query formulations. [Cagle’s worked example](https://ontologist.substack.com/p/competency-questions-lemmas-and-shapes) separates the question from conditions on the input data. Here, expert review settles what counts as an answer before choosing a schema, an index or a query language. The scientific sources linked in each set motivate the questions. The proposed answer criteria are local decisions.
