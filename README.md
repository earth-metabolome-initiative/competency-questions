# Chemical competency questions

![Review status](https://img.shields.io/badge/questions-draft-blue)

What should a natural-products researcher be able to retrieve from a dataset? Each entry gives a question, the evidence needed to answer it, and what one answer contains. These are proposed retrieval requirements for expert review. The crate distributes the questions as plain text without a chemistry library, database or network connection.

| Topic and question set | Related crate | Focus |
| --- | --- | --- |
| [SMILES](questions/smiles-rs.md) | [`smiles-rs`](https://github.com/earth-metabolome-initiative/smiles-rs) | Identity and comparison of reported structures |
| [SMARTS](questions/smarts.md) | [`smarts-rs`](https://github.com/earth-metabolome-initiative/smarts-rs) | Finding structural motifs |
| [Elements](questions/elements.md) | [`elements_rs`](https://github.com/earth-metabolome-initiative/elements-rs) | Element and isotope evidence |
| [Molecular formulas (MF)](questions/molecular-formulas.md) | [`molecular-formulas`](https://github.com/earth-metabolome-initiative/molecular-formulas) | Composition, candidate formulas and spectral assignments |

Each reviewed question has one **Scope** tag. `pure` means the home crate's data and the supplied query parameters determine every match and answer field. Input record IDs may pass through unchanged. For SMARTS, the normal inputs are a pattern and target molecular graphs. A paper cited to motivate a question does not count as extra data.

`mixed` means the answer also needs another kind of data. The tag names it. For example, a question that uses a SMILES structure **and** an observed fragment needs molecular graphs and mass spectrometry. Keep it in one topic file under its original ID. Add or check the scope line when reviewing a new draft, before merging it.

For a formula-only indexing workload, compare [`CQ-0017`](questions/molecular-formulas.md#cq-0017) (fluorine present), [`CQ-0014`](questions/molecular-formulas.md#cq-0014) (atom counts across isotopes) and [`CQ-0011`](questions/molecular-formulas.md#cq-0011) (composition feasibility). [`CQ-0013`](questions/molecular-formulas.md#cq-0013) also needs the experimental isolation history. To list the formula-only IDs and their questions, run

```sh
git grep -B 4 -F '**Scope.** `pure`' -- questions/molecular-formulas.md
```

## Add a question

1. Open **[Add a question](https://github.com/earth-metabolome-initiative/competency-questions/issues/new?template=question.yml)** on GitHub. Write your question in the title box. Try “Which formulas contain sodium?”
2. Pick a topic. For this question, choose **Molecular formulas (MF)**.
3. Give one example. You could write “Table salt (NaCl) should match. Water (H2O) should not.” Click **Create** to send it.

GitHub gives your question a number and opens a **pull request**, a proposed change to the catalogue. Follow the link on your issue to see the question and its example. You can suggest changes there. When a reviewer accepts and merges the change, the question joins the catalogue and GitHub closes the issue. You do not need to install anything or choose an ID.

<details>
<summary>Prefer the terminal?</summary>

Run the following from the root of a writable checkout only if you want to submit the example.

```sh
cargo run -- add molecular-formulas "Which chemical entities contain exactly two chlorine atoms?"
```

The command requires Rust and Cargo, GNU `timeout`, and a signed-in GitHub CLI (`gh`). It opens a public tracking issue and adds the question to a file in your checkout. This terminal route leaves the pull request to you. Add your example to the printed file, then commit it, push your branch and open a pull request.

</details>

Both routes use the issue number plus 14 as the permanent question number, keeping the original fourteen IDs. Gaps are fine. If submission is interrupted, look for your issue before trying again.

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
