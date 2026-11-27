# Comparing reported structures with `SMILES`

## CQ-0001

Which organism reports describe the same molecule as my quercetin aglycone reference, even when two catalogues draw its rings differently?

**Answer.** One row per organism and original publication, with the reported compound name and all contributing catalogue record IDs. Two catalogues citing the same occurrence contribute one row. An independent publication on the same organism contributes another.

**Match rule.** Supply the reference `SMILES`, each reported structure and its organism–publication link. Compare the whole molecular graph under one declared aromaticity convention. Ignore atom order, ring numbering and equivalent ways of writing attached hydrogens. Preserve isotopes, charges, bond orders outside aromatic systems, specified stereochemistry and all disconnected components. A quercetin glycoside, a different tautomer and a name-only report do not qualify. A structure match alone cannot establish an organism occurrence.

**Sources.** [Daylight’s `SMILES` specification](https://www.daylight.com/dayhtml/doc/theory/theory.smiles.html) illustrates equivalent ring and hydrogen spellings and separately encoded tautomers. [Rutz et al.](https://pmc.ncbi.nlm.nih.gov/articles/PMC9135406/) link structures to organisms through cited reports in LOTUS.

## CQ-0002

Among cacao records with the connectivity of (+)-catechin, which assign that stereoisomer, which assign a different one, and which leave the assignment open?

**Answer.** One row per source record with the sample or processing stage, citation, assignment at C-2 and C-3, and overall result. Keep the source’s stereochemical evidence alongside the drawn structure so a reviewer can distinguish a chiral separation from a name assigned to an unresolved peak.

**Match rule.** Use a fully assigned (+)-catechin reference and align atoms by connectivity before comparing the two centres. Classify as different if any assigned centre conflicts, as matching if both agree, and as unresolved otherwise. If the source explicitly identifies a mixture, report it separately. An unmarked stereocentre establishes neither a racemate nor a specific enantiomer. Flag conflicts between the drawing and the source assignment for review.

**Sources.** [Hurst et al.](https://doi.org/10.1186/1752-153X-5-53) resolved (−)-epicatechin and (+)-catechin in ripe cacao beans and reported (−)-catechin after processing. [Daylight’s isomeric `SMILES` rules](https://www.daylight.com/dayhtml/doc/theory/theory.smiles.html) explain how local atom order determines the meaning of `@` and `@@`.

## CQ-0003

Which reference materials contain the same berberine cation as my standard, and which counterions or solvates distinguish the materials I could compare?

**Answer.** Group records by the matched berberine cation. Within that group, give one row per material with its original record ID and citation, reported form, counterion, solvent and stoichiometry where known. Mark an unreported counterion or hydration state as unknown. A cation-only drawing cannot establish that the material was anhydrous.

**Match rule.** Supply the cation reference, the full structure as deposited and any material description. Require an entire connected component to match the reference, retaining the positive charge. Account for other components using the source’s assignments. Flag unidentified components or a second organic constituent of uncertain role rather than silently discarding them. A covalent berberine derivative fails the component match. Sharing a cation does not establish identical salt forms or crystal structures.

**Source.** [Kornilova et al.](https://pubmed.ncbi.nlm.nih.gov/35547800/) determined the crystal structure of berberine chloride methanol monosolvate and compared it with reported hydrates and an ethanol solvate hemihydrate.
