# Molecular formulas and fragment assignments

## CQ-0007

Which chemical entities have a reported molecular formula containing at least one fluorine atom?

**Match.** Use the formula assigned to the analyte itself, excluding counterions, solvents and measurement adducts. Sum the counts of atoms with atomic number 9 across all isotopes. A positive count qualifies wherever `F` appears in the parsed formula. `Fe` is iron and contributes zero. If the analyte formula is missing or its component cannot be separated, its fluorine content is unknown.

**Answer.** One row per entity and matching formula assignment, with the entity ID, reported formula, fluorine count, assignment status and source. Competing assignments remain distinguishable.

[Feng et al.](https://doi.org/10.1039/C9SC03374B) reported two glucosylated fluorometabolites from *Streptomyces calvus* and characterized them by high-resolution MS and fluorine NMR.

## CQ-0014

Which chemical entities have a reported molecular formula containing at least four nitrogen atoms and no sulfur atoms?

**Match.** Count atoms across isotopes in the same complete analyte formula, excluding counterions, solvents and measurement adducts. Require a nitrogen count of four or more and a sulfur count of zero. A missing or partial formula cannot establish the absence of sulfur.

**Answer.** One row per entity and matching formula assignment, with the entity ID, reported formula, nitrogen and sulfur counts, assignment status and source.

**Examples.** [Caffeine, `C8H10N4O2` (PubChem CID 2519)](https://pubchem.ncbi.nlm.nih.gov/compound/2519), qualifies at exactly four nitrogen atoms. As formula-only counterexamples, `C8H10N4OS` fails the sulfur condition and `C8H11N3O2` fails the nitrogen condition.

## CQ-0010

Which molecular formulas reported in this dataset could account for a measured monoisotopic precursor peak under the specified adduct hypotheses?

**Match.** For each distinct neutral formula and allowed adduct, calculate the monoisotopic ion mass using the declared isotope masses and charge carrier, then divide by the absolute charge. Keep every pair whose absolute error is within the supplied tolerance, inclusive. For a tolerance in ppm, use the signed error `10⁶ × (observed − calculated) / calculated`, with both values expressed as mass-to-charge ratios. A different adduct is a separate hypothesis.

**Answer.** One row per peak, formula and adduct, with charge, observed and calculated mass-to-charge ratios, signed error in ppm, and the IDs of all entities reporting that formula. Isomers remain separate entities even when they share a row.

[Kind and Fiehn](https://pmc.ncbi.nlm.nih.gov/articles/PMC1851972/) investigated ambiguity in formula assignments from accurate masses and described why the adduct matters when relating a peak to a neutral formula.

## CQ-0011

Before searching spectra, which entities have enough atoms to produce an ion with the composition of glucose and accommodate a neutral loss of water?

**Match.** Start with each entity’s neutral formula and a specified ion form, such as `[M+Na]+`. Let `P` be the resulting precursor ion composition, `G` the ion formed from `C6H12O6` with the same carrier and charge, and `W` neutral `H2O`. For this screen assume unlabeled analytes and retention of the carrier. Require `P − G` to have no negative atom counts and at least one positive count. Test the specified origin of the water loss separately.

- **From the precursor.** `P − W` must have no negative atom counts and retain the carrier and a nonempty analyte residue. If the glucose-composition ion must itself be the water-loss product, also require `P − W = G`.
- **From the glucose-composition ion.** `G − W` must be a nonempty ion composition retaining the carrier. The candidate sequence is `P → G → G − W`.

**Answer.** One row per entity, formula assignment, ion form and requested relationship, with the calculated product compositions and the precursor-to-`G` difference. For example, sodiated `C12H22O11` can supply sodiated `C6H12O6`, leaving `C6H10O5`. No spectrum is required. Passing the atom-count checks establishes neither a fragmentation pathway nor glucose identity.

[Böcker and Dührkop](https://doi.org/10.1186/s13321-016-0116-8) distinguish modeled fragmentation trees with formula-labeled losses from experimental trees recording which ion was isolated at each stage.

## CQ-0012

Which entities have a tandem mass spectrum where the same isolated precursor gives both a product assigned the composition of glucose and a product consistent with loss of water?

**Match.** Follow an entity’s precursor assignment to its product-ion scan. Require a measured product with an existing ion-formula assignment corresponding to neutral `C6H12O6`, and a measured product whose assigned ion formula is the precursor ion formula minus `H2O` with unchanged charge. Check the measured transition against a declared tolerance. For ordinary water, the expected neutral monoisotopic loss is 18.010565 Da, so the expected mass-to-charge separation is 18.010565 divided by the absolute charge. Count a single product for both conditions only when its assignment satisfies both, and flag that case. Exclude ambiguous co-isolation where the products cannot be assigned to the same precursor.

**Answer.** One row per entity, precursor and qualifying product pair, with spectrum and peak IDs, measured mass-to-charge ratios, ion formulas, charges, assignment sources and loss mass error in Da. Two products in the same scan provide no evidence that the glucose-composition product lost water itself.

**Measured example.** [The maltose spectra in Figure 1B–C](https://pmc.ncbi.nlm.nih.gov/articles/PMC5882935/#Fig1) show sodiated maltose, `[C12H22O11+Na]+` at nominal `m/z` 365, giving products at nominal `m/z` 203 and 347. The former fits `[C6H12O6+Na]+`. The latter fits `[C12H20O10+Na]+`, the precursor minus water. These nominal peak labels are distinct from high-resolution mass measurements.

## CQ-0013

Which entities produce a fragment assigned the composition of glucose that, when isolated in a later MS stage, itself gives a water-loss product?

**Match.** Follow the acquisition links from the entity’s precursor to the measured fragment and from that fragment’s isolation to its own product scan. Require an existing assignment corresponding to neutral `C6H12O6` for the isolated fragment. Require its measured product to retain the charge and carrier, differ in assigned composition by exactly `H2O`, and agree with the predicted mass-to-charge separation within the specified tolerance. A computed fragmentation tree or two peaks in one product spectrum do not establish which ion was isolated.

**Answer.** One row per entity and linked acquisition path, with the scan and peak IDs in order, isolations, observed mass-to-charge ratios, assigned ion formulas and charges, assignment sources, and loss mass error in Da. Report any independent sugar identification separately.

**Measured example.** In [Nguan et al., Figure 8b](https://pmc.ncbi.nlm.nih.gov/articles/PMC11613545/#fig8), a raffinose fragmentation sequence yields a sodiated hexose fragment at nominal `m/z` 203. Its product spectrum includes nominal `m/z` 185, consistent with `[C6H12O6+Na]+ → [C6H10O5+Na]+`. The authors assign the isolated hexose to **galactose**. It qualifies for the composition question even though it would be excluded from a glucose-specific structure search.

## CQ-0017

Which formulas contain at least one fluorine (F) atom ?

**Example.** PFAS are retruned, water isnt.

[Review question](https://github.com/earth-metabolome-initiative/competency-questions/issues/3)
