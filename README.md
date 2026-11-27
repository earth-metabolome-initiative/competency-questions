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

## Cite or amend a question

Use [`CQ-0007`](questions/molecular-formulas.md#cq-0007) to discuss the fluorine question. Use `questions/molecular-formulas.md` to assign the whole set, or list individual IDs across sets. A question has one home even when answering it requires several crates. Search the checkout with `git grep --untracked -n '^## CQ-0007$' -- questions`.

For an exact handoff, give the recipient the repository location and full commit hash alongside the IDs or set path. In that checkout, `git show <commit>:questions/molecular-formulas.md` retrieves that revision of the set. A bare ID follows the question through wording edits. For an uncommitted draft, share the working files with the recipient.

To comment, quote the ID and give an example that should match or should be excluded. To add a question, use the next unused `CQ-` number with four digits as its own second-level heading. Keep the ID and set path through clarifications. A different retrieval task gets a new ID. Leave a forwarding note under a retired ID and never reuse it.

```rust
use competency_questions::{question, QUESTION_SETS};

let fluorine = question("CQ-0007").ok_or("unknown question")?;
assert!(fluorine.to_lowercase().contains("fluorine"));
assert!(QUESTION_SETS.iter().any(|(name, _)| *name == "elements"));
# Ok::<(), &'static str>(())
```

## Why capture the question first?

[Grüninger and Fox’s methodology](https://eil.mie.utoronto.ca/theory/enterprise-modelling/entmethod/) derives informal competency questions from motivating scenarios. [Wiśniewski et al.](https://arxiv.org/abs/1811.09529) found that a single question pattern can have several query formulations. [Cagle’s worked example](https://ontologist.substack.com/p/competency-questions-lemmas-and-shapes) separates the question from conditions on the input data. Here, expert review settles what counts as an answer before choosing a schema, an index or a query language. The scientific sources linked in each set motivate the questions. The proposed answer criteria are local decisions.
