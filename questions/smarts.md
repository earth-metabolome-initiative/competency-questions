# Finding structural motifs with `SMARTS`

## Which reported apigenin glycosides carry a sugar directly at C-6 or C-8, which carry one through the oxygen at C-7, and which carry both types of linkage?

### CQ-0004

**Scope.** `mixed` — SMARTS matching on molecular graphs and source reports. The unresolved list includes reports with no drawn attachment.

**Answer.** One row per qualifying attachment with the record ID, source citation, numbered apigenin position and linkage type. Include the sugar’s anomeric carbon and the atoms connecting it to the scaffold, identified on the deposited structure. A compound with both types appears under each attachment.

**Match rule.** Supply the numbered apigenin scaffold and the structures to search. Match the anomeric carbon of a cyclic sugar either directly to apigenin C-6 or C-8, or through one oxygen to C-7. Allow other substituents and either anomer. A bond between two sugars does not qualify as a second attachment to apigenin. Keep a report that says only “apigenin hexoside” without a resolved linkage in a separate unresolved list.

**Source.** [Olennikov et al.](https://pmc.ncbi.nlm.nih.gov/articles/PMC6918269/) report apigenin glycosides in *Gentiana*, including apigenin 7-O-glucoside, isovitexin and isovitexin 7-O-glucoside. The last compound gives a case where both linkage types matter.

## Which phenolics in my collection contain free catechol groups on at least two distinct rings, as in rosmarinic acid?

### CQ-0005

**Scope.** `pure` — SMARTS matching on supplied molecular graphs. Distinct rings and oxygen pairs are identifiable from the matched atoms.

**Answer.** One row per qualifying structure, with its input record ID, count of qualifying rings and atom IDs for each pair of adjacent phenolic oxygens on each ring.

**Match rule.** Require two adjacent carbons on the same six-carbon aromatic ring each to bear a neutral hydroxyl. For example, `c1([OX2H1+0])c([OX2H1+0])cccc1` searches the drawn neutral form under a declared aromaticity model. Count rings within one connected molecule. Deduplicate reversed and rotated matches by the set of six ring atoms. A ring with three adjacent hydroxyls can supply two oxygen pairs but counts as one ring. Methyl ethers and phenolates fail this free-hydroxyl query.

**Sources.** [Gil et al.](https://pubmed.ncbi.nlm.nih.gov/23092164/) studied the two catechol moieties of rosmarinic acid and verbascoside. [Daylight’s `SMARTS` specification](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html) defines aromatic atoms, hydrogen counts and charge constraints.

## Which structures in my natural-products collection contain both an epoxide and an α-methylene-γ-lactone in the same molecule, as parthenolide does?

### CQ-0006

**Scope.** `pure` — SMARTS matching on supplied molecular graphs. Both motifs and their connected component are determined from the graph.

**Answer.** One row per matching component, with its input record ID and the atom IDs of the epoxide, the lactone ring and the terminal methylene. Multiple matches within that component can be inspected in the same row.

**Match rule.** Search for a three-membered ring of one oxygen and two saturated carbons. On that same connected component, require a five-membered cyclic ester with an exocyclic terminal `=CH2` on the ring carbon adjacent to the carbonyl carbon. Leave stereochemistry unconstrained. An epoxide and a lactone drawn as separate mixture components fail, as does an otherwise similar lactone whose exocyclic carbon carries an aryl substituent.

**Sources.** [Bommagani et al.](https://pmc.ncbi.nlm.nih.gov/articles/PMC4719832/) describe parthenolide and determine the structure of a pyrimidinyl derivative that retains the epoxide and lactone but has a substituted exocyclic alkene. [Daylight’s `SMARTS` specification](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html) distinguishes a same-component condition from an unconstrained search for disconnected patterns.
