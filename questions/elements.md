# Elements and isotope evidence

<a id="cq-0007"></a>
For fluorine in an analyte formula, see [`CQ-0007` in the molecular-formulas set](molecular-formulas.md#cq-0007).

## CQ-0008

Which LC–MS features have a resolved isotope doublet consistent with one bromine atom?

**Scope.** `mixed` — element isotope data and LC–MS measurements. Coelution, peak spacing and intensity ratio require measured peaks.

**Match.** Within one feature of known charge, use coeluting peaks assigned to the same ion species that differ by one substitution of bromine-81 for bromine-79. For natural isotope abundance, require the charge-corrected spacing to fall within a specified mass tolerance of 1.997952 Da and the heavy-to-light intensity ratio within a specified absolute tolerance of 0.973. Use integrated intensities from the same peak boundaries. Exclude unresolved contributions and adducts that can supply bromine when screening for bromine in the analyte.

**Answer.** One candidate per feature and doublet, with feature and peak IDs, adduct, charge, both measured mass-to-charge ratios, observed spacing and intensity ratio, and the tolerances used. This screen alone cannot establish a bromination site or a molecular formula.

The [NIST bromine isotope table](https://physics.nist.gov/cgi-bin/Compositions/stand_alone.pl?ele=Br) supplies the isotope masses and abundances. [A fungal natural-products study](https://doi.org/10.1021/jo2009593) used isotope patterns to select halogenated metabolites for isolation and structural analysis.

## CQ-0009

Which metabolites have isotope-labeling evidence for exactly six carbon atoms?

**Scope.** `mixed` — element isotope data and paired isotope-labeling experiments. The expected mass shift is calculable, while the observed endpoints and their pairing require feature records.

**Match.** In a paired native and uniformly carbon-13-labeled experiment, require coeluting all-light and fully labeled signals for the same monomeric ion and charge, with a carbon-free adduct. Six substitutions predict an ion mass increase of 6 × 1.003355 Da. Compare the measured mass-to-charge separation multiplied by the absolute charge to that increase within the supplied mass tolerance. Require evidence that the heavy peak is the fully labeled endpoint. A partially labeled six-carbon shift can also come from a larger molecule.

**Answer.** One row per qualifying feature pair, with sample and feature IDs, charge and adduct, both measured mass-to-charge ratios, inferred carbon count, mass residual in Da, tolerance, and provenance for the pairing and label assignment. Include the metabolite ID when assigned.

[Bueschl et al.](https://pmc.ncbi.nlm.nih.gov/articles/PMC4098048/) used native and uniformly labeled fungal and plant material to extract feature pairs and infer the carbon count of metabolite ions.
