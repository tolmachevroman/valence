# Review run: Qwen3-4B-Instruct-2507-Q4_K_M

50 molecules from `fixtures/review`, cards written as the app writes them. Times start when a molecule's snippets are in hand; fetching adds 1–3 s uncached. The Danger card is built from data and appears at once, so first-card times count model cards.

## Against the PRD targets

| Measure | Target | Result |
| --- | --- | --- |
| First model card | under 10 s | 49 of 50 molecules; 2.3 s median, 6.0 s p90, 11.2 s max |
| Streaming starts | under 3 s | 37 of 50; 1.9 s median, 4.6 s p90, 6.1 s max |
| Molecules with 3+ cards | 90% | 44 of 50 (88%) |
| Cards with a claim not in the sources | 0 | by review, below |
| Model load | under 10 s | 6.3 s |
| Model card, including retry | | 4.7 s median, 6.3 s p90, 13.4 s max |

## Outcomes

| Card | written | retried | skipped | dropped | no input | error |
| --- | --- | --- | --- | --- | --- | --- |
| Where you meet it | 49 | 0 | 0 | 1 | 0 | 0 |
| Origin story | 29 | 1 | 3 | 3 | 1 | 0 |
| Weird fact | 42 | 2 | 2 | 4 | 0 | 0 |

## Cards

Each claim should be stated by the cited snippets, folded under the card.

### DDT (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> 1-Chloro-4-(2,2,2-trichloro-1-(4-chlorophenyl)ethyl)benzene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 3.3 s

> DDT is a chlorophenylethane that is 1,1,1-trichloro-2,2-diphenylethane substituted by additional chloro substituents at positions 4 of the phenyl substituents. It is a commonly used organochlorine insecticide. Formerly used as an insecticide and pesticide; [HSDB] All agricultural uses cancelled in 1972 by EPA; [Sullivan, p.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/3036#section=Record-Description): DDT is a chlorophenylethane that is 1,1,1-trichloro-2,2-diphenylethane substituted by additional chloro substituents at positions 4 of the phenyl substituents. It is a commonly used organochlorine insecticide. It has a role as an endocrine disruptor, a bridged diphenyl acaricide, a carcinogenic agent and a persistent organic pollutant. It is a chlorophenylethane, an organochlorine insecticide, a benzenoid aromatic compound and a member of monochlorobenzenes. It is functionally related to a 4,4'-dichlorodiphenylmethane and a 1,1,1-trichloro-2,2-diphenylethane.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/3036#section=Uses): Formerly used as an insecticide and pesticide; [HSDB]
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/3036#section=Uses): All agricultural uses cancelled in 1972 by EPA; [Sullivan, p. 660]

</details>

**Weird fact** · skipped · 3.2 s


### acetaminophen (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Acetaminophen carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 3 more hazards, not listed here.

**Origin story** · dropped · 6.2 s

Problems: no chosen sentence names Acetaminophen

**Weird fact** · written · 5.0 s

> But its unacceptable toxic effects—the most alarming being cyanosis due to methemoglobinemia, an increase of hemoglobin in its ferric [Fe3+] state, called methemoglobin, which cannot bind oxygen, and thus decreases overall carriage of oxygen to tissue—prompted the search for less toxic aniline derivatives. In 1948, Bernard Brodie, Julius Axelrod and Frederick Flinn confirmed that paracetamol was the major metabolite of acetanilide in humans, and established that it was just as efficacious an analgesic as its precursor.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Paracetamol#History): Acetanilide was the first aniline derivative serendipitously found to possess analgesic as well as antipyretic properties, and was quickly introduced into medical practice under the name of Antifebrin by Cahn & Hepp in 1886. But its unacceptable toxic effects—the most alarming being cyanosis due to methemoglobinemia, an increase of hemoglobin in its ferric [Fe3+] state, called methemoglobin, which cannot bind oxygen, and thus decreases overall carriage of oxygen to tissue—prompted the search for less toxic aniline derivatives. Some reports state that Cahn & Hepp or a French chemist called Charles Gerhardt first synthesized paracetamol in 1852.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Paracetamol#History): Von Mering's claims remained essentially unchallenged for half a century until two teams of researchers from the United States analysed the metabolism of acetanilide and phenacetin. In 1947, David Lester and Leon Greenberg found strong evidence that paracetamol was a major metabolite of acetanilide in human blood, and in a subsequent study, they reported that large doses of paracetamol given to albino rats did not cause methemoglobinemia. In 1948, Bernard Brodie, Julius Axelrod and Frederick Flinn confirmed that paracetamol was the major metabolite of acetanilide in humans, and established that it was just as efficacious an analgesic as its precursor.

</details>

**Where you meet it** · written · 5.7 s

> Acetaminophen is often found combined with other drugs in more than 600 over the counter (OTC) allergy medications, cold medications, sleep medications, pain relievers, and other products. Paracetamol, or acetaminophen, is an analgesic and antipyretic agent used to treat fever and mild to moderate pain. It is a widely available over-the-counter drug sold generically or under various brand names, including Tylenol and Panadol.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/1983#section=Record-Description): Acetaminophen (paracetamol), also commonly known as Tylenol, is the most commonly taken analgesic worldwide and is recommended as first-line therapy in pain conditions by the World Health Organization (WHO). It is also used for its antipyretic effects, helping to reduce fever. This drug was initially approved by the U.S. FDA in 1951 and is available in a variety of forms including syrup form, regular tablets, effervescent tablets, injection, suppository, and other forms. Acetaminophen is often found combined with other drugs in more than 600 over the counter (OTC) allergy medications, cold medications, sleep medications, pain relievers, and other products.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Paracetamol): Paracetamol, or acetaminophen, is an analgesic and antipyretic agent used to treat fever and mild to moderate pain. It is a widely available over-the-counter drug sold generically or under various brand names, including Tylenol and Panadol.

</details>


### allicin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Allicin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 1.8 s

> Allicin was discovered as part of efforts to create thiamine derivatives in the 1940s, mainly in Japan.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Allicin#History): Allicin was discovered as part of efforts to create thiamine derivatives in the 1940s, mainly in Japan. Allicin became a model for medicinal chemistry efforts to create other thiamine disulfides. The results included sulbutiamine, fursultiamine (thiamine tetrahydrofurfuryl disulfide) and benfothiamine. These compounds are hydrophobic, easily pass from the intestines to the bloodstream, and are reduced to thiamine by cysteine or glutathione.

</details>

**Where you meet it** · written · 2.4 s

> Allicin is an organosulfur compound obtained from garlic and leeks. When fresh garlic is chopped or crushed, the enzyme alliinase converts alliin into allicin, which is responsible for the aroma of fresh garlic.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Allicin): Allicin is an organosulfur compound obtained from garlic and leeks. When fresh garlic is chopped or crushed, the enzyme alliinase converts alliin into allicin, which is responsible for the aroma of fresh garlic. Allicin is unstable and quickly changes into a series of other sulfur-containing compounds such as diallyl disulfide. Allicin is an antifeedant, i.e. the defense mechanism against attacks by pests on the garlic plant.

</details>

**Weird fact** · written · 3.2 s

> Allicin is chiral but occurs naturally only as a racemate. Allicin is an antifeedant, i.e. the defense mechanism against attacks by pests on the garlic plant.

<details><summary>Sources</summary>

- [Wikipedia, Structure and occurrence](https://en.wikipedia.org/wiki/Allicin#Structure_and_occurrence): Allicin features the thiosulfinate functional group, R-S(O)-S-R. The compound is not present in garlic unless tissue damage occurs, and is formed by the action of the enzyme alliinase on alliin. Allicin is chiral but occurs naturally only as a racemate. The racemic form can also be generated by oxidation of diallyl disulfide:
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Allicin): Allicin is an organosulfur compound obtained from garlic and leeks. When fresh garlic is chopped or crushed, the enzyme alliinase converts alliin into allicin, which is responsible for the aroma of fresh garlic. Allicin is unstable and quickly changes into a series of other sulfur-containing compounds such as diallyl disulfide. Allicin is an antifeedant, i.e. the defense mechanism against attacks by pests on the garlic plant.

</details>


### artemisinin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Artemisinin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · retried · 4.3 s

> Artemisinin is an antimalarial lactone derived from Artemisia annua or sweet wormwood. In 1596, Li Shizhen recommended tea made from 'sweet wormwood specifically to treat malaria symptoms in his Compendium of Materia Medica. Tu's team subsequently isolated an extract.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Artemisinin#History): Artemisinin is an antimalarial lactone derived from Artemisia annua or sweet wormwood. In 1596, Li Shizhen recommended tea made from 'sweet wormwood specifically to treat malaria symptoms in his Compendium of Materia Medica. The genus name is derived from the Greek goddess Artemis and, more specifically, may have been named after Queen Artemisia II of Caria, a botanist and medical researcher in the fourth century BC.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Artemisinin#History): Tu's team subsequently isolated an extract. Results were published in the Chinese Medical Journal in 1979. The extracted substance, once subject to purification, proved to be a useful starting point to obtain purified artemisinin. A 2012 review reported that artemisinin-based therapies were the most effective drugs for treatment of malaria at that time; it was also reported to clear malaria parasites from patients' bodies faster than other drugs. In addition to artemisinin, Project 523 developed a number of products that can be used in combination with artemisinin, including lumefantrine, piperaquine, and pyronaridine.

</details>

**Where you meet it** · written · 3.5 s

> (+)-artemisinin is a sesquiterpene lactone obtained from sweet wormwood, Artemisia annua, which is used as an antimalarial for the treatment of multi-drug resistant strains of falciparum malaria. Artemisinin can be extracted from the herb Artemisia annua (sweet wormwood), which is used in traditional Chinese medicine.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/68827#section=Record-Description): (+)-artemisinin is a sesquiterpene lactone obtained from sweet wormwood, Artemisia annua, which is used as an antimalarial for the treatment of multi-drug resistant strains of falciparum malaria. It has a role as an antimalarial and a plant metabolite. It is an organic peroxide and a sesquiterpene lactone.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Artemisinin): Artemisinin and its semisynthetic derivatives are a group of drugs used in the treatment of malaria due to Plasmodium falciparum. It was discovered in 1972 by Tu Youyou, who shared the 2015 Nobel Prize in Physiology or Medicine for her discovery. Artemisinin-based combination therapies (ACTs) have become standard treatment worldwide for P. falciparum malaria as well as malaria due to other species of Plasmodium. Artemisinin can be extracted from the herb Artemisia annua (sweet wormwood), which is used in traditional Chinese medicine.

</details>

**Weird fact** · written · 5.5 s

> The genus name is derived from the Greek goddess Artemis and, more specifically, may have been named after Queen Artemisia II of Caria, a botanist and medical researcher in the fourth century BC. An unusual component of the artemisinin molecules is an endoperoxide 1,2,4-trioxane ring.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Artemisinin#History): Artemisinin is an antimalarial lactone derived from Artemisia annua or sweet wormwood. In 1596, Li Shizhen recommended tea made from 'sweet wormwood specifically to treat malaria symptoms in his Compendium of Materia Medica. The genus name is derived from the Greek goddess Artemis and, more specifically, may have been named after Queen Artemisia II of Caria, a botanist and medical researcher in the fourth century BC.
- [Wikipedia, Chemistry](https://en.wikipedia.org/wiki/Artemisinin#Chemistry): An unusual component of the artemisinin molecules is an endoperoxide 1,2,4-trioxane ring. This is the main antimalarial centre of the molecule. Modifications at carbon 10 position give rise to a variety of derivatives which are more powerful than the original compound. Because the physical properties of artemisinin itself, such as poor bioavailability, limit its effectiveness, semisynthetic derivatives of artemisinin have been developed. Derivatives of dihydroartemisinin were made since 1976. Artesunate, arteether, and artemether were first synthesized in 1986.

</details>


### ascorbic acid (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> L-Ascorbic Acid is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · written · 1.7 s

> Vitamin C was discovered in 1912, isolated in 1928 and synthesized in 1933, making it the first vitamin to be synthesized. Shortly thereafter Tadeus Reichstein succeeded in synthesizing the vitamin in bulk by what is now called the Reichstein process. In 1912, the Polish biochemist Casimir Funk developed the concept of vitamins. One of these was thought to be the anti-scorbutic factor.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Vitamin_C#History): Vitamin C was discovered in 1912, isolated in 1928 and synthesized in 1933, making it the first vitamin to be synthesized. Shortly thereafter Tadeus Reichstein succeeded in synthesizing the vitamin in bulk by what is now called the Reichstein process. This made possible the inexpensive mass-production of vitamin C. In 1934, Hoffmann–La Roche bought the Reichstein process patent, trademarked synthetic vitamin C under the brand name Redoxon, and began to market it as a dietary supplement.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Vitamin_C#History): In 1907, a laboratory animal model which would help to identify the antiscorbutic factor was serendipitously discovered by the Norwegian physicians Axel Holst and Theodor Frølich, who when studying shipboard beriberi, fed guinea pigs their test diet of grains and flour and were surprised when scurvy resulted instead of beriberi. Unknown at that time, this species did not make its own vitamin C (being a caviomorph), whereas mice and rats do. In 1912, the Polish biochemist Casimir Funk developed the concept of vitamins. One of these was thought to be the anti-scorbutic factor. In 1928, this was referred to as "water-soluble C", although its chemical structure had not been determined.

</details>

**Weird fact** · written · 5.0 s

> In 1928, the Canadian Arctic anthropologist Vilhjalmur Stefansson showed that the Inuit avoided scurvy on a diet largely of raw meat. Vitamin C was discovered in 1912, isolated in 1928 and synthesized in 1933, making it the first vitamin to be synthesized.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Vitamin_C#History): The name antiscorbutic was used in the eighteenth and nineteenth centuries for foods known to prevent scurvy. These foods included lemons, limes, oranges, sauerkraut, cabbage, malt, and portable soup. In 1928, the Canadian Arctic anthropologist Vilhjalmur Stefansson showed that the Inuit avoided scurvy on a diet largely of raw meat.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Vitamin_C#History): Vitamin C was discovered in 1912, isolated in 1928 and synthesized in 1933, making it the first vitamin to be synthesized. Shortly thereafter Tadeus Reichstein succeeded in synthesizing the vitamin in bulk by what is now called the Reichstein process. This made possible the inexpensive mass-production of vitamin C. In 1934, Hoffmann–La Roche bought the Reichstein process patent, trademarked synthetic vitamin C under the brand name Redoxon, and began to market it as a dietary supplement.

</details>

**Where you meet it** · written · 6.0 s

> A six carbon compound related to glucose. It is found naturally in citrus fruits and many vegetables. Food sources of L-ascorbic acid include acerola (West Indian cherry); citrus fruits, tomatoes; potatoes, and green leafy vegetables.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/54670067#section=Record-Description): A six carbon compound related to glucose. It is found naturally in citrus fruits and many vegetables. Ascorbic acid is an essential nutrient in human diets, and necessary to maintain connective tissue and bone. Its biologically active form, vitamin C, functions as a reducing agent and coenzyme in several metabolic pathways. Vitamin C is considered an antioxidant.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/54670067#section=Natural-Pollution-Sources): L-Ascorbic acid is widely distributed in the plant and animal kingdom. Food sources of L-ascorbic acid include acerola (West Indian cherry); citrus fruits, tomatoes; potatoes, and green leafy vegetables. It has been isolated from the adrenal cortex of ox and later from lemons and paprika.

</details>


### aspirin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Aspirin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 3 more hazards, not listed here.

**Origin story** · written · 2.2 s

> In 1853, the chemist Charles Frédéric Gerhardt treated sodium salicylate with acetyl chloride to produce acetylsalicylic acid for the first time; in the second half of the 19th century, other academic chemists established the compound's chemical structure and devised more efficient methods of synthesis.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Aspirin#History): In 1853, the chemist Charles Frédéric Gerhardt treated sodium salicylate with acetyl chloride to produce acetylsalicylic acid for the first time; in the second half of the 19th century, other academic chemists established the compound's chemical structure and devised more efficient methods of synthesis. In 1897, scientists at the drug and dye firm Bayer began investigating acetylsalicylic acid as a less-irritating replacement for standard common salicylate medicines, and identified a new way to synthesize it. That year, Felix Hoffmann (or Arthur Eichengrün) of Bayer was the first to produce acetylsalicylic acid in a pure, stable form.

</details>

**Where you meet it** · written · 3.3 s

> Aspirin is the genericized trademark for acetylsalicylic acid (ASA), a nonsteroidal anti-inflammatory drug (NSAID) used to reduce pain, fever, and inflammation, and as an antithrombotic. Aspirin is used in the treatment of a number of conditions, including fever, pain, rheumatic fever, and inflammatory conditions, such as rheumatoid arthritis, pericarditis, and Kawasaki disease.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Aspirin): Aspirin is the genericized trademark for acetylsalicylic acid (ASA), a nonsteroidal anti-inflammatory drug (NSAID) used to reduce pain, fever, and inflammation, and as an antithrombotic. Aspirin is used to treat inflammatory conditions including Kawasaki disease, pericarditis, and rheumatic fever.
- [Wikipedia, Medical use](https://en.wikipedia.org/wiki/Aspirin#Medical_use): Aspirin is used in the treatment of a number of conditions, including fever, pain, rheumatic fever, and inflammatory conditions, such as rheumatoid arthritis, pericarditis, and Kawasaki disease. Lower doses of aspirin have also been shown to reduce the risk of death from a myocardial infarction (heart attack), or the risk of stroke in people who are at high risk or who have cardiovascular disease, but not in older adults who are otherwise healthy. There is evidence that aspirin is effective for preventing colorectal cancer, though the mechanisms of this effect are unclear. There is also evidence that aspirin can treat some cases of vasculogenic ED.

</details>

**Weird fact** · written · 5.7 s

> In 1897, scientists at the Bayer company began studying acetylsalicylic acid as a less-irritating replacement medication for common salicylate medicines. It developed heroin in 1898 as a cough suppressant, and the less potent aspirin the following year. By 1899, Bayer had named it "Aspirin" and was selling it around the world.

<details><summary>Sources</summary>

- [Wikipedia, Brand vs. generic name](https://en.wikipedia.org/wiki/Aspirin#Brand_vs._generic_name): In 1897, scientists at the Bayer company began studying acetylsalicylic acid as a less-irritating replacement medication for common salicylate medicines. It developed heroin in 1898 as a cough suppressant, and the less potent aspirin the following year. By 1899, Bayer had named it "Aspirin" and was selling it around the world.

</details>


### batrachotoxin (3 cards)

**Origin story** · written · 1.1 s

> Batrachotoxin was discovered by Fritz Märki and Bernhard Witkop, at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. Märki and Witkop isolated the toxic alkaloids fraction from the black-legged poison dart frog and determined its chemical properties in 1963.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Batrachotoxin#History): Batrachotoxin was discovered by Fritz Märki and Bernhard Witkop, at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. Märki and Witkop isolated the toxic alkaloids fraction from the black-legged poison dart frog and determined its chemical properties in 1963. They isolated four major toxic steroidal alkaloids including batrachotoxin, homobatrachotoxin (isobatrachotoxin), pseudobatrachotoxin, and batrachotoxinin A. Due to the difficulty of handling such a potent toxin and the minuscule amount that could be collected, a comprehensive structure determination involved several difficulties.

</details>

**Where you meet it** · written · 4.9 s

> Batrachotoxins (BTX) are extremely potent cardiotoxic and neurotoxic steroidal alkaloids found in poison dart frogs (genus Dendrobates and Phyllobates), Melyridae beetles, and certain birds (genus Pitohui and Ifrita). Batrachotoxin (BTX) is an extremely potent cardiotoxic and neurotoxic steroidal alkaloid found in certain species of beetles, birds, and frogs. Batrachotoxins are found in poison dart frogs (genus Dendrobates and Phyllobates), Melyridae beetles, and certain birds (genus Pitohui and Ifrita).

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/6324647#section=Record-Description): Batrachotoxins (BTX) are extremely potent cardiotoxic and neurotoxic steroidal alkaloids found in poison dart frogs (genus Dendrobates and Phyllobates), Melyridae beetles, and certain birds (genus Pitohui and Ifrita). They act directly on sodium channels.. It is the most potent non-peptidal neurotoxin known.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Batrachotoxin): Batrachotoxin (BTX) is an extremely potent cardiotoxic and neurotoxic steroidal alkaloid found in certain species of beetles, birds, and frogs. The name is from the Greek word βάτραχος, bátrachos, 'frog'. Structurally-related chemical compounds are often referred to collectively as batrachotoxins. In certain frogs, this alkaloid is present mostly on the skin. Such frogs are among those used for poisoning darts. Batrachotoxin binds to and irreversibly opens the sodium channels of nerve cells and prevents them from closing, resulting in paralysis and death. No antidote is known.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/6324647#section=Uses): Batrachotoxins are found in poison dart frogs (genus Dendrobates and Phyllobates), Melyridae beetles, and certain birds (genus Pitohui and Ifrita).

</details>

**Weird fact** · written · 5.8 s

> Batrachotoxin binds to and irreversibly opens the sodium channels of nerve cells and prevents them from closing, resulting in paralysis and death. Batrachotoxins (BTX) are extremely potent cardiotoxic and neurotoxic steroidal alkaloids found in poison dart frogs (genus Dendrobates and Phyllobates), Melyridae beetles, and certain birds (genus Pitohui and Ifrita). They act directly on sodium channels.. It is the most potent non-peptidal neurotoxin known.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Batrachotoxin): Batrachotoxin (BTX) is an extremely potent cardiotoxic and neurotoxic steroidal alkaloid found in certain species of beetles, birds, and frogs. The name is from the Greek word βάτραχος, bátrachos, 'frog'. Structurally-related chemical compounds are often referred to collectively as batrachotoxins. In certain frogs, this alkaloid is present mostly on the skin. Such frogs are among those used for poisoning darts. Batrachotoxin binds to and irreversibly opens the sodium channels of nerve cells and prevents them from closing, resulting in paralysis and death. No antidote is known.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/6324647#section=Record-Description): Batrachotoxins (BTX) are extremely potent cardiotoxic and neurotoxic steroidal alkaloids found in poison dart frogs (genus Dendrobates and Phyllobates), Melyridae beetles, and certain birds (genus Pitohui and Ifrita). They act directly on sodium channels.. It is the most potent non-peptidal neurotoxin known.

</details>


### benzene (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Benzene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 2.3 s

> Michael Faraday first isolated and identified benzene in 1825 from the oily residue derived from the production of illuminating gas, giving it the name bicarburet of hydrogen. At Benzolfest, Kekulé discussed the theory's creation, having discovered the ring shape of the benzene molecule after having a reverie (or day-dream) of a snake biting its own tail (a symbol in ancient cultures known as the ouroboros).

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Benzene#History): The word "benzene" derives from "gum benzoin" (benzoin resin), an aromatic resin known since ancient times in Southeast Asia, and later to European pharmacists and perfumers in the 16th century via trade routes. An acidic material was derived from benzoin by sublimation, and named "flowers of benzoin", or benzoic acid. The hydrocarbon derived from benzoic acid thus acquired the names benzin, benzol, and benzene. Michael Faraday first isolated and identified benzene in 1825 from the oily residue derived from the production of illuminating gas, giving it the name bicarburet of hydrogen.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Benzene#History): The new understanding of benzene, and hence of all aromatic compounds, proved to be notable enough for both pure and applied chemistry that, in 1890, the German Chemical Society organized a celebration in Kekulé's honor (named as the "Benzolfest"), celebrating the twenty-fifth anniversary of his first paper on benzene. At Benzolfest, Kekulé discussed the theory's creation, having discovered the ring shape of the benzene molecule after having a reverie (or day-dream) of a snake biting its own tail (a symbol in ancient cultures known as the ouroboros).

</details>

**Where you meet it** · written · 6.0 s

> Benzene is a colorless liquid with a sweet odor. Natural sources of benzene include volcanoes and forest fires. Benzene is also a natural part of crude oil, gasoline, and cigarette smoke. Benzene is found in the air from emissions from burning coal and oil, gasoline service stations, and motor vehicle exhaust.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/241#section=Record-Description): Benzene is a colorless liquid with a sweet odor. It evaporates into the air very quickly and dissolves slightly in water. It is highly flammable and is formed from both natural processes and human activities. Benzene is widely used in the United States; it ranks in the top 20 chemicals for production volume. Some industries use benzene to make other chemicals which are used to make plastics, resins, and nylon and synthetic fibers. Benzene is also used to make some types of rubbers, lubricants, dyes, detergents, drugs, and pesticides. Natural sources of benzene include volcanoes and forest fires. Benzene is also a natural part of crude oil, gasoline, and cigarette smoke.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/241#section=Record-Description): Benzene is found in the air from emissions from burning coal and oil, gasoline service stations, and motor vehicle exhaust. Acute (short-term) inhalation exposure of humans to benzene may cause drowsiness, dizziness, headaches, as well as eye, skin, and respiratory tract irritation, and, at high levels, unconsciousness. Chronic (long-term) inhalation exposure has caused various disorders in the blood, including reduced numbers of red blood cells and aplastic anemia, in occupational settings. Reproductive effects have been reported for women exposed by inhalation to high levels, and adverse effects on the developing fetus have been observed in animal tests.

</details>

**Weird fact** · written · 6.0 s

> X-ray diffraction reveals that benzene is a planar molecule with six-fold rotational symmetry, which is in the symmetry group, D6h. A valence bond description involves a superposition of resonance structures. Its considerable stability and distinctive chemical properties are manifested in the phenomenon of aromaticity.

<details><summary>Sources</summary>

- [Wikipedia, Structure](https://en.wikipedia.org/wiki/Benzene#Structure): X-ray diffraction reveals that benzene is a planar molecule with six-fold rotational symmetry, which is in the symmetry group, D6h. The carbon-carbon bond lengths of benzene are 140 picometers (pm) long, which are greater than a C=C double bond (135 pm) but shorter than a C-C single bond (147 pm).
- [Wikipedia, Structure](https://en.wikipedia.org/wiki/Benzene#Structure): In terms of bonding, the molecular orbital (MO) description involves three bonding pi-MOs. A valence bond description involves a superposition of resonance structures. Its considerable stability and distinctive chemical properties are manifested in the phenomenon of aromaticity. To reflect the delocalized nature of the bonding, benzene is often depicted with a circle inside a hexagonal arrangement of carbon atoms.

</details>


### benzylpenicillin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Penicillin G carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 2 more hazards, not listed here.

**Where you meet it** · written · 3.6 s

> Penicillin G (potassium or sodium) is an antibacterial prescription medicine approved by the U.S. Food and Drug Administration (FDA) for the treatment of certain serious bacterialinfections, such as pneumonia, meningitis, gonorrhea, syphilis, and others. Benzylpenicillin (Penicillin G) is narrow spectrum antibiotic used to treat infections caused by susceptible bacteria.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5904#section=Record-Description): Penicillin G (potassium or sodium) is an antibacterial prescription medicine approved by the U.S. Food and Drug Administration (FDA) for the treatment of certain serious bacterialinfections, such as pneumonia, meningitis, gonorrhea, syphilis, and others. Pneumonia and syphilis can be opportunistic infections (OI) of HIV.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5904#section=Record-Description): Benzylpenicillin (Penicillin G) is narrow spectrum antibiotic used to treat infections caused by susceptible bacteria. It is a natural penicillin antibiotic that is administered intravenously or intramuscularly due to poor oral absorption. Penicillin G may also be used in some cases as prophylaxis against susceptible organisms. Natural penicillins are considered the drugs of choice for several infections caused by susceptible gram positive aerobic organisms, such as *Streptococcus pneumoniae*, groups A, B, C and G streptococci, nonenterococcal group D streptococci, viridans group streptococci, and non-penicillinase producing staphylococcus.

</details>

**Weird fact** · retried · 9.3 s

> Penicillin G (potassium or sodium) is an antibacterial prescription medicine approved by the U.S. Food and Drug Administration (FDA) for the treatment of certain serious bacterialinfections, such as pneumonia, meningitis, gonorrhea, syphilis, and others. Benzylpenicillin (Penicillin G) is narrow spectrum antibiotic used to treat infections caused by susceptible bacteria.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5904#section=Record-Description): Penicillin G (potassium or sodium) is an antibacterial prescription medicine approved by the U.S. Food and Drug Administration (FDA) for the treatment of certain serious bacterialinfections, such as pneumonia, meningitis, gonorrhea, syphilis, and others. Pneumonia and syphilis can be opportunistic infections (OI) of HIV.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5904#section=Record-Description): Benzylpenicillin (Penicillin G) is narrow spectrum antibiotic used to treat infections caused by susceptible bacteria. It is a natural penicillin antibiotic that is administered intravenously or intramuscularly due to poor oral absorption. Penicillin G may also be used in some cases as prophylaxis against susceptible organisms. Natural penicillins are considered the drugs of choice for several infections caused by susceptible gram positive aerobic organisms, such as *Streptococcus pneumoniae*, groups A, B, C and G streptococci, nonenterococcal group D streptococci, viridans group streptococci, and non-penicillinase producing staphylococcus.

</details>


### buckminsterfullerene (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Fullerenes carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · written · 2.7 s

> Theoretical predictions of buckminsterfullerene molecules appeared in the late 1960s and early 1970s. It was first generated in 1984 by Eric Rohlfing, Donald Cox, and Andrew Kaldor, using a laser to vaporize carbon in a supersonic helium beam, although the group did not realize that buckminsterfullerene had been produced. The discoverers of the allotrope named the newfound molecule after American architect R. Buckminster Fuller, who designed many geodesic dome structures that look similar to C60 and who had died in 1983, the year before discovery.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Buckminsterfullerene#History): Theoretical predictions of buckminsterfullerene molecules appeared in the late 1960s and early 1970s. It was first generated in 1984 by Eric Rohlfing, Donald Cox, and Andrew Kaldor, using a laser to vaporize carbon in a supersonic helium beam, although the group did not realize that buckminsterfullerene had been produced. In 1985 their work was repeated by Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University, who recognized the structure of C60 as buckminsterfullerene.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Buckminsterfullerene#History): The discoverers of the allotrope named the newfound molecule after American architect R. Buckminster Fuller, who designed many geodesic dome structures that look similar to C60 and who had died in 1983, the year before discovery. Another common name for buckminsterfullerene is "buckyballs".

</details>

**Where you meet it** · written · 4.5 s

> Buckminsterfullerene is the most common naturally occurring fullerene. Neutral C60 has been observed in planetary nebulae and several types of star.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Buckminsterfullerene#Occurrence): Buckminsterfullerene is the most common naturally occurring fullerene. Small quantities of it can be found in soot.
- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Buckminsterfullerene#Occurrence): It also exists in space. Neutral C60 has been observed in planetary nebulae and several types of star. The ionised form, C+60, has been identified in the interstellar medium, where it is the cause of several absorption features known as diffuse interstellar bands in the near-infrared.

</details>

**Weird fact** · dropped · 13.2 s

Problems: no chosen sentence names Fullerenes


### caffeine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Caffeine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 2.1 s

> In 1819, the German chemist Friedlieb Ferdinand Runge isolated caffeine for the first time; he called it "Kaffebase" (i.e., a base that exists in coffee). In 1821, caffeine was isolated both by the French chemist Pierre Jean Robiquet and by another pair of French chemists, Pierre-Joseph Pelletier and Joseph Bienaimé Caventou, according to Swedish chemist Jöns Jacob Berzelius in his yearly journal. In 1827, M. Oudry isolated "théine" from tea, but in 1838 it was proved by Mulder and by Carl Jobst that theine was actually the same as caffeine.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Caffeine#History): In 1819, the German chemist Friedlieb Ferdinand Runge isolated caffeine for the first time; he called it "Kaffebase" (i.e., a base that exists in coffee). In 1821, caffeine was isolated both by the French chemist Pierre Jean Robiquet and by another pair of French chemists, Pierre-Joseph Pelletier and Joseph Bienaimé Caventou, according to Swedish chemist Jöns Jacob Berzelius in his yearly journal. Furthermore, Berzelius stated that the French chemists had made their discoveries independently of any knowledge of Runge's or each other's work.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Caffeine#History): In 1827, M. Oudry isolated "théine" from tea, but in 1838 it was proved by Mulder and by Carl Jobst that theine was actually the same as caffeine.

</details>

**Weird fact** · written · 4.8 s

> In 1821, caffeine was isolated both by the French chemist Pierre Jean Robiquet and by another pair of French chemists, Pierre-Joseph Pelletier and Joseph Bienaimé Caventou, according to Swedish chemist Jöns Jacob Berzelius in his yearly journal. Furthermore, Berzelius stated that the French chemists had made their discoveries independently of any knowledge of Runge's or each other's work.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Caffeine#History): In 1819, the German chemist Friedlieb Ferdinand Runge isolated caffeine for the first time; he called it "Kaffebase" (i.e., a base that exists in coffee). In 1821, caffeine was isolated both by the French chemist Pierre Jean Robiquet and by another pair of French chemists, Pierre-Joseph Pelletier and Joseph Bienaimé Caventou, according to Swedish chemist Jöns Jacob Berzelius in his yearly journal. Furthermore, Berzelius stated that the French chemists had made their discoveries independently of any knowledge of Runge's or each other's work.

</details>

**Where you meet it** · written · 6.0 s

> Caffeine is a major component of coffee, tea and chocolate and in humans acts as a central nervous system (CNS) stimulant. The most common sources of caffeine for human consumption are the tea leaves of the Camellia sinensis plant and the coffee bean, the seed of the Coffea plant. A smaller part is used in cosmetics (3%) or in technical applications (1%). Furthermore caffeine is a naturally occurring substance in various plant species (e.g. 0.9 to 2.6% in green coffee beans). It is a component in coffee, tea and cocoa.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/2519#section=Record-Description): Caffeine is xanthine alkaloid that occurs naturally in seeds, leaves and fruit of several plants and trees that acts as a natural pesticide. Caffeine is a major component of coffee, tea and chocolate and in humans acts as a central nervous system (CNS) stimulant. Consumption of caffeine, even in high doses, has not been associated with elevations in serum enzyme elevations or instances of clinically apparent liver injury.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Caffeine): Caffeine is a bitter, white crystalline purine, a methylxanthine alkaloid, and is chemically related to the adenine and guanine bases of deoxyribonucleic acid (DNA) and ribonucleic acid (RNA). It is found in the seeds, fruits, nuts, or leaves of a number of plants native to Africa, East Asia, and South America and helps to protect them against herbivores and from competition by preventing the germination of nearby seeds, as well as encouraging consumption by select animals such as honey bees. The most common sources of caffeine for human consumption are the tea leaves of the Camellia sinensis plant and the coffee bean, the seed of the Coffea plant.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/2519#section=Uses): Caffeine is a substance in wide use. It is predominantly used in the food sector and pharma sector (80% and 16% rspectively). A smaller part is used in cosmetics (3%) or in technical applications (1%). Furthermore caffeine is a naturally occurring substance in various plant species (e.g. 0.9 to 2.6% in green coffee beans). It is a component in coffee, tea and cocoa.

</details>


### capsaicin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Capsaicin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · written · 1.0 s

> The compound was first extracted in impure form in 1816 by Christian Friedrich Bucholz (1770–1818). In 1873 German pharmacologist Rudolf Buchheim (1820–1879) and in 1878 the Hungarian doctor Endre Hőgyes stated that "capsicol" (partially purified capsaicin) caused the burning feeling when in contact with mucous membranes and increased secretion of gastric acid.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Capsaicin#History): The compound was first extracted in impure form in 1816 by Christian Friedrich Bucholz (1770–1818).
- [Wikipedia, History](https://en.wikipedia.org/wiki/Capsaicin#History): In 1873 German pharmacologist Rudolf Buchheim (1820–1879) and in 1878 the Hungarian doctor Endre Hőgyes stated that "capsicol" (partially purified capsaicin) caused the burning feeling when in contact with mucous membranes and increased secretion of gastric acid.

</details>

**Where you meet it** · written · 4.8 s

> Capsaicin (8-methyl-N-vanillyl-6-nonenamide) is a toxin that is the main active component of chili peppers and gives them their distinct pungent, "spicy" or "hot" taste. Because of the burning sensation caused by capsaicin when it comes in contact with mucous membranes, it is commonly used in food products to provide added spiciness or "heat" (piquancy), usually in the form of spices such as chili powder and paprika. There has long been a demand for capsaicin-spiced products like chili pepper, and hot sauces such as Tabasco sauce and Mexican salsa.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Capsaicin): Capsaicin (8-methyl-N-vanillyl-6-nonenamide) is a toxin that is the main active component of chili peppers and gives them their distinct pungent, "spicy" or "hot" taste. It is a potent irritant for mammals for which it produces a sensation of burning in any tissue with which it comes into contact. Capsaicin and several related amides (capsaicinoids) are produced as secondary metabolites by chili peppers, likely as deterrents against eating by mammals and against the growth of fungi. Pure capsaicin is a hydrophobic, highly pungent (i.e., spicy) crystalline solid.
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Capsaicin#Uses): Because of the burning sensation caused by capsaicin when it comes in contact with mucous membranes, it is commonly used in food products to provide added spiciness or "heat" (piquancy), usually in the form of spices such as chili powder and paprika. In high concentrations, capsaicin will also cause a burning effect on other sensitive areas, such as skin or eyes. The degree of heat found within a food is often measured on the Scoville scale.
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Capsaicin#Uses): There has long been a demand for capsaicin-spiced products like chili pepper, and hot sauces such as Tabasco sauce and Mexican salsa. It is common for people to experience pleasurable and even euphoric effects from ingesting capsaicin. Folklore among self-described "chiliheads" attribute this to pain-stimulated release of endorphins, a different mechanism from the local receptor overload that makes capsaicin effective as a topical analgesic. Because capsaicin sublimes at 115 °C, some precaution is needed when cooking dry or with fats to preserve capsaicin in pungent food.

</details>

**Weird fact** · written · 6.7 s

> In birds, the TRPV1 channel does not respond to capsaicin or related chemicals, but mammalian TRPV1 is very sensitive to it. This is advantageous to the plant, as chili pepper seeds consumed by birds pass through the digestive tract and can germinate later, whereas mammals have molar teeth that destroy such seeds and prevent them from germinating.

<details><summary>Sources</summary>

- [Wikipedia, Natural function](https://en.wikipedia.org/wiki/Capsaicin#Natural_function): The seeds of Capsicum plants are dispersed predominantly by birds. In birds, the TRPV1 channel does not respond to capsaicin or related chemicals, but mammalian TRPV1 is very sensitive to it. This is advantageous to the plant, as chili pepper seeds consumed by birds pass through the digestive tract and can germinate later, whereas mammals have molar teeth that destroy such seeds and prevent them from germinating. Thus, natural selection may have led to increasing capsaicin production because it makes the plant less likely to be eaten by animals that do not help it disperse. There is also evidence that capsaicin may have evolved as an anti-fungal agent.

</details>


### carbon monoxide (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Carbon Monoxide carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 2.9 s

> Man has maintained a complex relationship with carbon monoxide since first learning to control fire circa 800,000 BC. Early man probably discovered the toxicity of carbon monoxide poisoning upon introducing fire into his dwellings. Aristotle (384–322 BC) first recorded that burning coals produced toxic fumes.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Carbon_monoxide#History): Man has maintained a complex relationship with carbon monoxide since first learning to control fire circa 800,000 BC. Early man probably discovered the toxicity of carbon monoxide poisoning upon introducing fire into his dwellings. The early development of metallurgy and smelting technologies emerging circa 6,000 BC through the Bronze Age likewise plagued man from carbon monoxide exposure. Apart from the toxicity of carbon monoxide, Native Americans in the United States may have experienced the neuroactive properties of carbon monoxide through shamanistic fireside rituals.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Carbon_monoxide#History): Early civilizations developed mythological tales to explain the origin of fire, such as Prometheus from Greek mythology who shared fire with humans. Aristotle (384–322 BC) first recorded that burning coals produced toxic fumes. Greek physician Galen (AD 129–199) speculated that there was a change in the composition of the air that caused harm when inhaled, and many others of the era developed a basis of knowledge about carbon monoxide in the context of coal fume toxicity. Cleopatra may have died from carbon monoxide poisoning.

</details>

**Where you meet it** · written · 5.9 s

> The most common source of carbon monoxide is the partial combustion of carbon-containing compounds. CO is one of the most acutely toxic contaminants affecting indoor air quality. CO may be emitted from tobacco smoke and generated from malfunctioning fuel-burning stoves (wood, kerosene, natural gas, propane) and fuel-burning heating systems (wood, oil, natural gas) and from blocked flues connected to these appliances.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Carbon_monoxide): The most common source of carbon monoxide is the partial combustion of carbon-containing compounds. Numerous environmental and biological sources generate carbon monoxide. In industry, carbon monoxide is important in the production of many compounds, including drugs, fragrances, and fuels.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Carbon_monoxide): CO is one of the most acutely toxic contaminants affecting indoor air quality. CO may be emitted from tobacco smoke and generated from malfunctioning fuel-burning stoves (wood, kerosene, natural gas, propane) and fuel-burning heating systems (wood, oil, natural gas) and from blocked flues connected to these appliances. Carbon monoxide poisoning is the most common type of fatal air poisoning in many countries.

</details>

**Weird fact** · written · 6.3 s

> Early man probably discovered the toxicity of carbon monoxide poisoning upon introducing fire into his dwellings. In the presence of oxygen, including atmospheric concentrations, carbon monoxide burns with a blue flame, producing carbon dioxide. Following the first report that carbon monoxide is a normal neurotransmitter in 1993, carbon monoxide has received significant clinical attention as a biological regulator.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Carbon_monoxide#History): Man has maintained a complex relationship with carbon monoxide since first learning to control fire circa 800,000 BC. Early man probably discovered the toxicity of carbon monoxide poisoning upon introducing fire into his dwellings. The early development of metallurgy and smelting technologies emerging circa 6,000 BC through the Bronze Age likewise plagued man from carbon monoxide exposure. Apart from the toxicity of carbon monoxide, Native Americans in the United States may have experienced the neuroactive properties of carbon monoxide through shamanistic fireside rituals.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Carbon_monoxide#History): Joseph Priestley is considered to have first synthesized carbon monoxide in 1772. Carl Wilhelm Scheele similarly isolated carbon monoxide from charcoal in 1773 and thought it could be the carbonic entity making fumes toxic. Torbern Bergman isolated carbon monoxide from oxalic acid in 1775. In the presence of oxygen, including atmospheric concentrations, carbon monoxide burns with a blue flame, producing carbon dioxide. Antoine Lavoisier conducted similarly inconclusive experiments in 1777.
- [Wikipedia, Biological and physiological properties](https://en.wikipedia.org/wiki/Carbon_monoxide#Biological_and_physiological_properties): Carbon monoxide is a bioactive molecule which acts as a gaseous signaling molecule. It is naturally produced by many enzymatic and non-enzymatic pathways, the best understood of which is the catabolic action of heme oxygenase on the heme derived from hemoproteins such as hemoglobin. Following the first report that carbon monoxide is a normal neurotransmitter in 1993, carbon monoxide has received significant clinical attention as a biological regulator.

</details>


### carvone (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Carvone, (+-)- carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 1.0 s

> Caraway was used for medicinal purposes by the ancient Romans, but carvone was probably not isolated as a pure compound until Franz Varrentrapp (1815–1877) obtained it in 1849. Goldschmidt and Zürrer identified it as a ketone related to limonene, and the structure was finally elucidated by Georg Wagner (1849–1903) in 1894.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Carvone#History): Caraway was used for medicinal purposes by the ancient Romans, but carvone was probably not isolated as a pure compound until Franz Varrentrapp (1815–1877) obtained it in 1849. It was originally called carvol by Schweizer. Goldschmidt and Zürrer identified it as a ketone related to limonene, and the structure was finally elucidated by Georg Wagner (1849–1903) in 1894.

</details>

**Where you meet it** · written · 5.0 s

> S-(+)-Carvone is the principal constituent (60–70%) of the oil from caraway seeds (Carum carvi), which is produced on a scale of about 10 tonnes per year. It also occurs to the extent of about 40–60% in dill seed oil (from Anethum graveolens), and also in mandarin orange peel oil. R-(−)-Carvone is also the most abundant compound in the essential oil from several species of mint, particularly spearmint oil (Mentha spicata), which is composed of 50–80% R-(−)-carvone.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Carvone#Occurrence): S-(+)-Carvone is the principal constituent (60–70%) of the oil from caraway seeds (Carum carvi), which is produced on a scale of about 10 tonnes per year. It also occurs to the extent of about 40–60% in dill seed oil (from Anethum graveolens), and also in mandarin orange peel oil. R-(−)-Carvone is also the most abundant compound in the essential oil from several species of mint, particularly spearmint oil (Mentha spicata), which is composed of 50–80% R-(−)-carvone. Spearmint is a major source of naturally produced R-(−)-carvone. However, the majority of R-(−)-carvone used in commercial applications is synthesized from R-(+)-limonene. The R-(−)-carvone isomer also occurs in kuromoji oil.

</details>

**Weird fact** · written · 6.5 s

> Carvone has two mirror image forms, or enantiomers: R-(−)-carvone, the sweetish minty smell of spearmint leaves. Its mirror image, S-(+)-carvone, has a spicy aroma with notes of rye, and gives caraway seeds their smell. The fact that the two enantiomers are perceived as smelling different is evidence that olfactory receptors must respond more strongly to one enantiomer than to the other.

<details><summary>Sources</summary>

- [Wikipedia, Stereoisomerism and odor](https://en.wikipedia.org/wiki/Carvone#Stereoisomerism_and_odor): Carvone has two mirror image forms, or enantiomers: R-(−)-carvone, the sweetish minty smell of spearmint leaves. Its mirror image, S-(+)-carvone, has a spicy aroma with notes of rye, and gives caraway seeds their smell.
- [Wikipedia, Stereoisomerism and odor](https://en.wikipedia.org/wiki/Carvone#Stereoisomerism_and_odor): The fact that the two enantiomers are perceived as smelling different is evidence that olfactory receptors must respond more strongly to one enantiomer than to the other. Not all enantiomers have distinguishable odors. Squirrel monkeys have also been found to be able to discriminate between carvone enantiomers.

</details>


### cholesterol (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Cholesterol is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · skipped · 0.5 s

**Where you meet it** · written · 4.2 s

> Major dietary sources of cholesterol include red meat, egg yolks and whole eggs, liver, kidney, giblets, fish oil, shellfish, and butter. Found in many foods (meats, eggs, milk, cheese, fish, shellfish) derived from animal products.

<details><summary>Sources</summary>

- [Wikipedia, Dietary sources](https://en.wikipedia.org/wiki/Cholesterol#Dietary_sources): Animal fats are complex mixtures of triglycerides, with lesser amounts of both the phospholipids and cholesterol molecules from which all animal (and human) cell membranes are constructed. Since all animal cells manufacture cholesterol, all animal-based foods contain cholesterol in varying amounts. Major dietary sources of cholesterol include red meat, egg yolks and whole eggs, liver, kidney, giblets, fish oil, shellfish, and butter. Human breast milk also contains significant quantities of cholesterol.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/5997#section=Uses): Found in many foods (meats, eggs, milk, cheese, fish, shellfish) derived from animal products. Essential for membrane integrity. Used in steroid hormone synthesis.

</details>

**Weird fact** · written · 6.2 s

> Cholesterol has 256 stereoisomers that arise from its eight stereocenters. In the brain, astrocytes produce cholesterol and transport it to neurons. It is absent among prokaryotes (bacteria and archaea), although there are some exceptions, such as Mycoplasma, which require cholesterol for growth.

<details><summary>Sources</summary>

- [Wikipedia, Stereoisomers](https://en.wikipedia.org/wiki/Cholesterol#Stereoisomers): Cholesterol has 256 stereoisomers that arise from its eight stereocenters. Only two of the stereoisomers have biochemical significance: nat-cholesterol and ent-cholesterol (for natural and enantiomer, respectively). The only cholesterol stereoisomer to occur naturally is nat-cholesterol.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Cholesterol): Cholesterol is biosynthesized by all animal cells and is an essential structural and signaling component of animal cell membranes. In vertebrates, hepatic cells typically produce the greatest amounts of it. In the brain, astrocytes produce cholesterol and transport it to neurons. It is absent among prokaryotes (bacteria and archaea), although there are some exceptions, such as Mycoplasma, which require cholesterol for growth. Cholesterol also serves as a precursor for the biosynthesis of steroid hormones, bile acid, and vitamin D.

</details>


### cinnamaldehyde (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Cinnamaldehyde carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · written · 5.3 s

> Cinnamaldehyde is a naturally occurring flavonoid that gives the spice cinnamon its flavour and odour. It occurs naturally in the bark of cinnamon trees and other species of the genus Cinnamomum such as camphor and cassia. Cinnamaldehyde occurs naturally in the bark of cinnamon trees and other species of the genus Cinnamomum like camphor and cassia. These trees are the natural source of cinnamon, and the essential oil of cinnamon bark is about 90% cinnamaldehyde.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/637511#section=Record-Description): Cinnamaldehyde is a naturally occurring flavonoid that gives the spice cinnamon its flavour and odour. It occurs naturally in the bark of cinnamon trees and other species of the genus Cinnamomum such as camphor and cassia. Sensitivity to cinnamaldehyde may be identified with a clinical patch test.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/637511#section=Record-Description): Cinnamaldehyde is the aldehyde that gives cinnamon its flavor and odor. Cinnamaldehyde occurs naturally in the bark of cinnamon trees and other species of the genus Cinnamomum like camphor and cassia. These trees are the natural source of cinnamon, and the essential oil of cinnamon bark is about 90% cinnamaldehyde. Cinnamaldehyde is also used as a fungicide. Proven effective on over 40 different crops, cinnamaldehyde is typically applied to the root systems of plants. Its low toxicity and well-known properties make it ideal for agriculture. To a lesser extent, cinnamaldehyde is an effective insecticide, and its scent is also known to repel animals like cats and dogs.

</details>

**Weird fact** · written · 7.0 s

> A concentration of 29 ppm of cinnamaldehyde kills half of Aedes aegypti mosquito larvae in 24 hours. Cinnamaldehyde is a bioactive electrophile that activates the transient receptor potential ankyrin 1 (TRPA1) ion channel, a chemosensory receptor expressed in sensory neurons and in the gastrointestinal tract.

<details><summary>Sources</summary>

- [Wikipedia, Applications](https://en.wikipedia.org/wiki/Cinnamaldehyde#Applications): Cinnamaldehyde has been tested as a safe and effective insecticide against mosquito larvae. A concentration of 29 ppm of cinnamaldehyde kills half of Aedes aegypti mosquito larvae in 24 hours. Trans-cinnamaldehyde works as a potent fumigant and practical repellant for adult mosquitos. It also has antibacterial and antifungal properties.
- [Wikipedia, Biological effects](https://en.wikipedia.org/wiki/Cinnamaldehyde#Biological_effects): Cinnamaldehyde is a bioactive electrophile that activates the transient receptor potential ankyrin 1 (TRPA1) ion channel, a chemosensory receptor expressed in sensory neurons and in the gastrointestinal tract. TRPA1 detects pungent or irritant compounds such as those found in cinnamon, mustard oil, and clove, producing the characteristic warming or burning sensation associated with these spices. In the gastrointestinal tract, TRPA1 activation by cinnamaldehyde influences the release of serotonin from enterochromaffin cells, linking chemical irritation with gut motility and sensory signaling.

</details>


### citric acid (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Citric Acid carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · written · 6.6 s

> A key intermediate in metabolism. It is an acid compound found in citrus fruits. Because it is one of the stronger edible acids, the dominant use of citric acid is as a flavoring and preservative in food and beverages, especially soft drinks and candies. Citric acid is used with sodium bicarbonate in a wide range of effervescent formulae, both for ingestion (e.g., powders and tablets) and for personal care (e.g., bath salts, bath bombs, and cleaning of grease).

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/311#section=Record-Description): A key intermediate in metabolism. It is an acid compound found in citrus fruits. The salts of citric acid (citrates) can be used as anticoagulants due to their calcium-chelating ability. Citric acid is one of the active ingredients in Phexxi, a non-hormonal contraceptive agent that was approved by the FDA on May 2020. It is also used in combination with magnesium oxide to form magnesium citrate, an osmotic laxative.
- [Wikipedia, Applications](https://en.wikipedia.org/wiki/Citric_acid#Applications): Because it is one of the stronger edible acids, the dominant use of citric acid is as a flavoring and preservative in food and beverages, especially soft drinks and candies. Within the European Union it is denoted by E number E330. Citrate salts of various metals are used to deliver those minerals in a biologically available form in many dietary supplements. Citric acid has 247 kcal per 100 g. In the United States the purity requirements for citric acid as a food additive are defined by the Food Chemicals Codex, which is published by the United States Pharmacopoeia (USP).
- [Wikipedia, Applications](https://en.wikipedia.org/wiki/Citric_acid#Applications): Citric acid can be added to ice cream as an emulsifying agent to keep fats from separating, to caramel to prevent sucrose crystallization, or in recipes in place of fresh lemon juice. Citric acid is used with sodium bicarbonate in a wide range of effervescent formulae, both for ingestion (e.g., powders and tablets) and for personal care (e.g., bath salts, bath bombs, and cleaning of grease). Citric acid sold in a dry powdered form is commonly sold in markets and groceries as "sour salt", due to its physical resemblance to table salt. It has use in culinary applications, as an alternative to vinegar or lemon juice, where a pure acid is needed.

</details>

**Weird fact** · dropped · 13.1 s

Problems: no chosen sentence names Citric Acid


### cubane (2 cards)

**Where you meet it** · written · 1.6 s

> Cubane is a synthetic hydrocarbon compound with the formula C8H8. It consists of eight carbon atoms arranged at the corners of a cube, with one hydrogen atom attached to each carbon atom.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Cubane): Cubane is a synthetic hydrocarbon compound with the formula C8H8. It consists of eight carbon atoms arranged at the corners of a cube, with one hydrogen atom attached to each carbon atom. A solid crystalline substance, cubane is one of the Platonic hydrocarbons and a member of the prismanes. It was first synthesized in 1964 by Philip Eaton and Thomas Cole. Before this work, Eaton believed that cubane would be impossible to synthesize due to the "required 90 degree bond angles". The cubic shape requires the carbon atoms to adopt an unusually sharp 90° bonding angle, which would be highly strained as compared to the 109.45° angle of a tetrahedral carbon.

</details>

**Weird fact** · written · 1.9 s

> Cubane is highly strained, but cannot decompose: the resultant cubene molecules would be pyramidal alkenes, too high-energy for most elimination pathways. The cubic shape requires the carbon atoms to adopt an unusually sharp 90° bonding angle, which would be highly strained as compared to the 109.45° angle of a tetrahedral carbon.

<details><summary>Sources</summary>

- [Wikipedia, Reactions and derivatives](https://en.wikipedia.org/wiki/Cubane#Reactions_and_derivatives): Cubane is highly strained, but cannot decompose: the resultant cubene molecules would be pyramidal alkenes, too high-energy for most elimination pathways. Certain metallic ions catalyze σ-bond rearrangement to cuneane:
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Cubane): Cubane is a synthetic hydrocarbon compound with the formula C8H8. It consists of eight carbon atoms arranged at the corners of a cube, with one hydrogen atom attached to each carbon atom. A solid crystalline substance, cubane is one of the Platonic hydrocarbons and a member of the prismanes. It was first synthesized in 1964 by Philip Eaton and Thomas Cole. Before this work, Eaton believed that cubane would be impossible to synthesize due to the "required 90 degree bond angles". The cubic shape requires the carbon atoms to adopt an unusually sharp 90° bonding angle, which would be highly strained as compared to the 109.45° angle of a tetrahedral carbon.

</details>


### dopamine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Dopamine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 2.0 s

> Dopamine was first synthesized in 1910 by George Barger and James Ewens at Wellcome Laboratories in London, England and first identified in the human brain by Katharine Montagu in 1957. It was named dopamine because it is a monoamine whose precursor in the Barger-Ewens synthesis is 3,4-dihydroxyphenylalanine (levodopa or L-DOPA).

<details><summary>Sources</summary>

- [Wikipedia, History and development](https://en.wikipedia.org/wiki/Dopamine#History_and_development): Dopamine was first synthesized in 1910 by George Barger and James Ewens at Wellcome Laboratories in London, England and first identified in the human brain by Katharine Montagu in 1957. It was named dopamine because it is a monoamine whose precursor in the Barger-Ewens synthesis is 3,4-dihydroxyphenylalanine (levodopa or L-DOPA). Dopamine's function as a neurotransmitter was first recognized in 1958 by Arvid Carlsson and Nils-Åke Hillarp at the Laboratory for Chemical Pharmacology of the National Heart Institute of Sweden.

</details>

**Where you meet it** · written · 4.4 s

> Dopamine is a naturally occurring catecholamine formed by decarboxylation of dehydroxyphenylalanine and a precursor of norepinephrine and epinephrine. Dopamine (DA, a contraction of 3,4-dihydroxyphenethylamine) is a neuromodulatory molecule that plays several important roles in cells. It is an organic chemical of the catecholamine and phenethylamine families. It is an amine synthesized by removing a carboxyl group through decarboxylation from a molecule of its precursor chemical, L-DOPA, which is synthesized in the brain and kidneys. Dopamine is also synthesized in plants and most animals.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/681#section=Record-Description): Dopamine is a monoamine compound with positive inotropic activity. Dopamine is a naturally occurring catecholamine formed by decarboxylation of dehydroxyphenylalanine and a precursor of norepinephrine and epinephrine. Dopamine binds to alpha-1 and beta-1 adrenergic receptors. Mediated through myocardial beta-1 adrenergic receptors, dopamine increase heart rate and force, thereby increasing cardiac output. Alpha-1 adrenergic receptor stimulation on vascular smooth muscle, leads to vasoconstriction and results in an increase in systemic vascular resistance.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Dopamine): Dopamine (DA, a contraction of 3,4-dihydroxyphenethylamine) is a neuromodulatory molecule that plays several important roles in cells. It is an organic chemical of the catecholamine and phenethylamine families. It is an amine synthesized by removing a carboxyl group through decarboxylation from a molecule of its precursor chemical, L-DOPA, which is synthesized in the brain and kidneys. Dopamine is also synthesized in plants and most animals. In the brain, dopamine functions as a neurotransmitter—a chemical released by neurons (nerve cells) to send signals to other nerve cells.

</details>

**Weird fact** · written · 6.0 s

> There are no reports of dopamine in archaea, but it has been detected in some types of bacteria and in the protozoan called Tetrahymena. Perhaps more importantly, there are types of bacteria that contain homologs of all the enzymes that animals use to synthesize dopamine. It has been proposed that animals derived their dopamine-synthesizing machinery from bacteria, via horizontal gene transfer that may have occurred relatively late in evolutionary time, perhaps as a result of the symbiotic incorporation of bacteria into eukaryotic cells that gave rise to mitochondria.

<details><summary>Sources</summary>

- [Wikipedia, Comparative biology and evolution](https://en.wikipedia.org/wiki/Dopamine#Comparative_biology_and_evolution): There are no reports of dopamine in archaea, but it has been detected in some types of bacteria and in the protozoan called Tetrahymena. Perhaps more importantly, there are types of bacteria that contain homologs of all the enzymes that animals use to synthesize dopamine. It has been proposed that animals derived their dopamine-synthesizing machinery from bacteria, via horizontal gene transfer that may have occurred relatively late in evolutionary time, perhaps as a result of the symbiotic incorporation of bacteria into eukaryotic cells that gave rise to mitochondria.

</details>


### epinephrine (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Epinephrine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 3.3 s

> Drug of choice for emergency treatment of severe allergic reactions to insect stings or bites, foods, drugs, and other allergens. Epinephrine is a naturally occurring hormone, which is released from the adrenal medulla and is involved in the regulation of blood pressure and smooth muscle constriction.

<details><summary>Sources</summary>

- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Uses): Drug of choice for emergency treatment of severe allergic reactions to insect stings or bites, foods, drugs, and other allergens.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Natural-Pollution-Sources): Epinephrine is a naturally occurring hormone, which is released from the adrenal medulla and is involved in the regulation of blood pressure and smooth muscle constriction. Epinephrine naturally occurs at the ends of sympathetic nerve fibers where they serve as chemical mediators for carrying nerve impulses.

</details>

**Weird fact** · written · 3.8 s

> Epinephrine naturally occurs at the ends of sympathetic nerve fibers where they serve as chemical mediators for carrying nerve impulses. Due to its strong β1-adrenergic effects on cardiac tissue, epinephrine toxicity can result in potentially fatal cardiac arrhythmias or ischemia.

<details><summary>Sources</summary>

- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Natural-Pollution-Sources): Epinephrine is a naturally occurring hormone, which is released from the adrenal medulla and is involved in the regulation of blood pressure and smooth muscle constriction. Epinephrine naturally occurs at the ends of sympathetic nerve fibers where they serve as chemical mediators for carrying nerve impulses.
- [PubChem, Toxicity Summary](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Toxicity-Summary): Signs and Symptoms of Overdose Epinephrine overdose may lead to myocardial ischemia, infarction, renal insufficiency, and cardiomyopathy. Patients may also experience severe pallor, cold skin, and metabolic acidosis due to elevated blood lactic acid levels. These complications are more likely in individuals with preexisting conditions or when epinephrine is administered in high doses. Pulmonary edema may also occur as a result of peripheral vasoconstriction and myocardial stimulation. Due to its strong β1-adrenergic effects on cardiac tissue, epinephrine toxicity can result in potentially fatal cardiac arrhythmias or ischemia.

</details>


### ethanol (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Ethanol carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 3.1 s

> The fermentation of sugar into ethanol is one of the earliest biotechnologies employed by humans. Dried residue on 9,000-year-old pottery found in China suggests that Neolithic people consumed alcoholic beverages. In 1828, Hennell and the French chemist Georges-Simon Serullas independently discovered that sulphovinic acid could be decomposed into ethanol.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Ethanol#History): The fermentation of sugar into ethanol is one of the earliest biotechnologies employed by humans. Ethanol has historically been identified variously as spirit of wine or ardent spirits, and as aqua vitae (Latin for "water of life") or aqua vita. The intoxicating effects of its consumption have been known since ancient times. Ethanol has been used by humans since prehistory as the intoxicating ingredient of alcoholic beverages. Dried residue on 9,000-year-old pottery found in China suggests that Neolithic people consumed alcoholic beverages.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Ethanol#History): Ethanol was first prepared synthetically in 1825 by Michael Faraday. He found that sulfuric acid could absorb large volumes of coal gas. He gave the resulting solution to Henry Hennell, a British chemist, who found in 1826 that it contained "sulphovinic acid" (ethyl hydrogen sulfate). In 1828, Hennell and the French chemist Georges-Simon Serullas independently discovered that sulphovinic acid could be decomposed into ethanol. Thus, in 1825 Faraday had unwittingly discovered that ethanol could be produced from ethylene (a component of coal gas) by acid-catalyzed hydration, a process similar to current industrial ethanol synthesis.

</details>

**Weird fact** · written · 5.8 s

> Ethanol has been detected in outer space, forming an icy coating around dust grains in interstellar clouds.

<details><summary>Sources</summary>

- [Wikipedia, Natural occurrence](https://en.wikipedia.org/wiki/Ethanol#Natural_occurrence): Ethanol has been detected in outer space, forming an icy coating around dust grains in interstellar clouds.

</details>

**Where you meet it** · written · 5.4 s

> Ethanol can commonly be found in overripe fruit. Industrial and laboratory organic solvent.

<details><summary>Sources</summary>

- [Wikipedia, Natural occurrence](https://en.wikipedia.org/wiki/Ethanol#Natural_occurrence): Ethanol is a byproduct of the metabolic process of yeast. As such, ethanol will be present in any yeast habitat. Ethanol can commonly be found in overripe fruit. Ethanol produced by symbiotic yeast can be found in bertam palm blossoms. Although some animal species, such as the pentailed treeshrew, exhibit ethanol-seeking behaviors, most show no interest or avoidance of food sources containing ethanol. Ethanol is also produced during the germination of many plants as a result of natural anaerobiosis.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/702#section=Uses): Most ethyl alcohol is used in alcoholic beverages in suitable dilutions. Reagent in synthetic organic chemistry and chromatography. Industrial and laboratory organic solvent. Other uses are in manufacture of denatured alcohol, pharmaceuticals (rubbing compounds, lotions, tonics, colognes), in perfumery. Octane booster in gasoline. Pharmaceutic aid (solvent).

</details>


### ethylene (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Ethylene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 1.9 s

> In the mid-19th century, the suffix -ene (an Ancient Greek root added to the end of female names meaning "daughter of") was widely used to refer to a molecule or part thereof that contained one fewer hydrogen atoms than the molecule being modified. Thus, ethylene (C2H4) was the "daughter of ethyl" (C2H5). The name ethylene was used in this sense as early as 1852.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Ethylene#History): In the mid-19th century, the suffix -ene (an Ancient Greek root added to the end of female names meaning "daughter of") was widely used to refer to a molecule or part thereof that contained one fewer hydrogen atoms than the molecule being modified. Thus, ethylene (C2H4) was the "daughter of ethyl" (C2H5). The name ethylene was used in this sense as early as 1852.

</details>

**Where you meet it** · written · 4.3 s

> Ethylene is widely used in the chemical industry, and its worldwide production (over 225 million tonnes in 2022) exceeds that of any other organic compound. Much of this production goes toward creating polyethylene, which is a widely used plastic containing polymer chains of ethylene units in various chain lengths. Ethylene has been found in the gaseous metabolites released by germinating bean, corn, cotton, and pea seeds, from fading morning glory flowers and from ripening avocados and apples.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Ethylene): Ethylene is widely used in the chemical industry, and its worldwide production (over 225 million tonnes in 2022) exceeds that of any other organic compound. Much of this production goes toward creating polyethylene, which is a widely used plastic containing polymer chains of ethylene units in various chain lengths.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/6325#section=Natural-Pollution-Sources): Ethylene is a natural product emitted by fruits, flowers, leaves, roots, and tubers. The release rate of ethylene from plants varies during the life cycle of the plant. Plants that normally produce 0.6 to 6 ug ethylene/kg (fresh weight) per hour may produce up to 120 ug/kg per hour during ripening of fruits and during senescence and loss of leaves. Ethylene has been found in the gaseous metabolites released by germinating bean, corn, cotton, and pea seeds, from fading morning glory flowers and from ripening avocados and apples. Ethylene is released to the atmosphere from biomass combustion and volcanos.

</details>

**Weird fact** · written · 5.6 s

> In the mid-19th century, the suffix -ene (an Ancient Greek root added to the end of female names meaning "daughter of") was widely used to refer to a molecule or part thereof that contained one fewer hydrogen atoms than the molecule being modified. Thus, ethylene (C2H4) was the "daughter of ethyl" (C2H5).

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Ethylene#History): In the mid-19th century, the suffix -ene (an Ancient Greek root added to the end of female names meaning "daughter of") was widely used to refer to a molecule or part thereof that contained one fewer hydrogen atoms than the molecule being modified. Thus, ethylene (C2H4) was the "daughter of ethyl" (C2H5). The name ethylene was used in this sense as early as 1852.

</details>


### formaldehyde (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Formaldehyde carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 5.9 s

> A ~5% solution in water of formaldehyde is used as a disinfectant and fumigant in industrial, agricultural, and healthcare settings, and a ~37% solution is used to preserve tissue samples in labs. Formaldehyde in food can be present naturally, added as an inadvertent contaminant, or intentionally added as a preservative, disinfectant, or bacteriostatic agent. Foods that the U.S. National Toxicology Program has reported to have higher levels compared to other foods are fish, seafood, and smoked ham.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Formaldehyde): Formaldehyde ( for-MAL-di-hide, US also fər-) (systematic name methanal) is a pungent colorless gas mainly used in the production of industrial resins, such as for particle board, coatings, plastics, pulp, paper, synthetic fibers, and textiles. A ~5% solution in water of formaldehyde is used as a disinfectant and fumigant in industrial, agricultural, and healthcare settings, and a ~37% solution is used to preserve tissue samples in labs. In 2024, the global production of formaldehyde was estimated at 26 million tons per year, and is a precursor to many other materials and chemical compounds.
- [Wikipedia, Contaminant in food](https://en.wikipedia.org/wiki/Formaldehyde#Contaminant_in_food): Formaldehyde in food can be present naturally, added as an inadvertent contaminant, or intentionally added as a preservative, disinfectant, or bacteriostatic agent. Cooking and smoking food can also result in formaldehyde being produced in food. Foods that the U.S. National Toxicology Program has reported to have higher levels compared to other foods are fish, seafood, and smoked ham. It also notes that formaldehyde in food generally occurs in a bound form and that formaldehyde is unstable in an aqueous solution.

</details>

**Weird fact** · written · 5.7 s

> Monomeric CH2O is a gas and is rarely encountered in the laboratory. Many cyclic oligomers of other sizes have been isolated. Similarly, formaldehyde hydrates to give the geminal diol methanediol, which condenses further to form hydroxy-terminated oligomers HO(CH2O)nH. The polymer is called paraformaldehyde.

<details><summary>Sources</summary>

- [Wikipedia, Organic chemistry](https://en.wikipedia.org/wiki/Formaldehyde#Organic_chemistry): Monomeric CH2O is a gas and is rarely encountered in the laboratory. Aqueous formaldehyde, unlike some other small aldehydes (which need specific conditions to oligomerize through aldol condensation) oligomerizes spontaneously at a common state. The trimer 1,3,5-trioxane, (CH2O)3, is a typical oligomer. Many cyclic oligomers of other sizes have been isolated. Similarly, formaldehyde hydrates to give the geminal diol methanediol, which condenses further to form hydroxy-terminated oligomers HO(CH2O)nH. The polymer is called paraformaldehyde. The higher concentration of formaldehyde—the more equilibrium shifts towards polymerization.

</details>


### geosmin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Geosmin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · written · 2.3 s

> Geosmin is responsible for the earthy taste of beetroots and a contributor to the strong scent, known as petrichor, that occurs when rain falls after a spell of dry weather or when soil is disturbed. It is produced by bacteria, especially cyanobacteria, often present in soil and water supplies.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Geosmin): Geosmin ( jee-OZ-min) is a chemical with a distinct earthy or musty odor. It is easily smelled by most people, having a very low odor detection threshold, ranging from 0.006 to 0.01 micrograms per liter in water. Geosmin is responsible for the earthy taste of beetroots and a contributor to the strong scent, known as petrichor, that occurs when rain falls after a spell of dry weather or when soil is disturbed. It is produced by bacteria, especially cyanobacteria, often present in soil and water supplies. Geosmin, along with 2-methylisoborneol, are the major biological causes of muddy odor and flavor in drinking water worldwide and in farmed fish.

</details>

**Weird fact** · written · 3.9 s

> The human olfactory system is extremely sensitive to geosmin and is able to detect it at concentrations as low as anywhere from 400 parts per trillion to 5 parts per trillion. In a study of 50 subjects, the mean odor detection threshold was 9.5 ng/L for naturally occurring (−)-geosmin and 78 ng/L for (+)-geosmin. This makes the natural form roughly eight times more potent.

<details><summary>Sources</summary>

- [Wikipedia, Effects](https://en.wikipedia.org/wiki/Geosmin#Effects): The human olfactory system is extremely sensitive to geosmin and is able to detect it at concentrations as low as anywhere from 400 parts per trillion to 5 parts per trillion. Human sensitivity differs between the two enantiomers. In a study of 50 subjects, the mean odor detection threshold was 9.5 ng/L for naturally occurring (−)-geosmin and 78 ng/L for (+)-geosmin. This makes the natural form roughly eight times more potent. It is proposed that this extreme sensitivity to geosmin evolved in humans or their ancestors to aid them in their search for scarce water, but this is not conclusive.

</details>


### glucose (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> D-Glucose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · written · 2.4 s

> Glucose was first isolated from raisins in 1747 by the German chemist Andreas Marggraf. Glucose was discovered in grapes by another German chemist, Johann Tobias Lowitz, in 1792, and distinguished as being different from cane sugar (sucrose).

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Glucose#History): Glucose was first isolated from raisins in 1747 by the German chemist Andreas Marggraf. Glucose was discovered in grapes by another German chemist, Johann Tobias Lowitz, in 1792, and distinguished as being different from cane sugar (sucrose). Glucose is the term coined by Jean-Baptiste Dumas in 1838, which has prevailed in the chemical literature. Friedrich August Kekulé proposed the term dextrose (from the Latin dexter, meaning "right"), because in aqueous solutions of glucose, the plane of linearly polarized light is turned to the right. In contrast, l-fructose (usually referred to as l-fructose) (a ketohexose) and l-glucose (l-glucose) turn linearly polarized light to the left.

</details>

**Weird fact** · written · 5.7 s

> Friedrich August Kekulé proposed the term dextrose (from the Latin dexter, meaning "right"), because in aqueous solutions of glucose, the plane of linearly polarized light is turned to the right. With six carbon atoms, it is classed as a hexose, a subcategory of the monosaccharides. d-Glucose is one of the sixteen aldohexose stereoisomers. The d-isomer, d-glucose, also known as dextrose, occurs widely in nature, but the l-isomer, l-glucose, does not.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Glucose#History): Glucose was first isolated from raisins in 1747 by the German chemist Andreas Marggraf. Glucose was discovered in grapes by another German chemist, Johann Tobias Lowitz, in 1792, and distinguished as being different from cane sugar (sucrose). Glucose is the term coined by Jean-Baptiste Dumas in 1838, which has prevailed in the chemical literature. Friedrich August Kekulé proposed the term dextrose (from the Latin dexter, meaning "right"), because in aqueous solutions of glucose, the plane of linearly polarized light is turned to the right. In contrast, l-fructose (usually referred to as l-fructose) (a ketohexose) and l-glucose (l-glucose) turn linearly polarized light to the left.
- [Wikipedia, Chemical and physical properties](https://en.wikipedia.org/wiki/Glucose#Chemical_and_physical_properties): With six carbon atoms, it is classed as a hexose, a subcategory of the monosaccharides. d-Glucose is one of the sixteen aldohexose stereoisomers. The d-isomer, d-glucose, also known as dextrose, occurs widely in nature, but the l-isomer, l-glucose, does not. Glucose can be obtained by hydrolysis of carbohydrates such as milk sugar (lactose), cane sugar (sucrose), maltose, cellulose, glycogen, etc. Dextrose is commonly commercially manufactured from starches, such as corn starch in the US and Japan, from potato and wheat starch in Europe, and from tapioca starch in tropical areas.

</details>

**Where you meet it** · written · 5.9 s

> Unbound glucose is one of the main ingredients of honey. Glucose is extremely abundant and has been isolated from a variety of natural sources across the world, including male cones of the coniferous tree Wollemia nobilis in Rome, the roots of Ilex asprella plants in China, and straws from rice in California.

<details><summary>Sources</summary>

- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Glucose#Sources): Most dietary carbohydrates contain glucose, either as their only building block (as in the polysaccharides starch and glycogen), or together with another monosaccharide (as in the hetero-polysaccharides sucrose and lactose). Unbound glucose is one of the main ingredients of honey. Glucose is extremely abundant and has been isolated from a variety of natural sources across the world, including male cones of the coniferous tree Wollemia nobilis in Rome, the roots of Ilex asprella plants in China, and straws from rice in California.

</details>


### glyphosate (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Glyphosate carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 1.4 s

> Glyphosate was first synthesized in 1950 by Swiss chemist Henry Martin, who worked for the Swiss company Cilag. About 100 derivatives of aminomethylphosphonic acid had been prepared as potential water-softening agents. Two were found to have weak herbicidal activity, and John E. Franz, a chemist at Monsanto, was asked to try to make analogs with stronger herbicidal activity. Glyphosate was the third analog he made.

<details><summary>Sources</summary>

- [Wikipedia, Discovery](https://en.wikipedia.org/wiki/Glyphosate#Discovery): Glyphosate was first synthesized in 1950 by Swiss chemist Henry Martin, who worked for the Swiss company Cilag. The work was never published. Early studies found it to be a weak chemical chelating agent.
- [Wikipedia, Discovery](https://en.wikipedia.org/wiki/Glyphosate#Discovery): Glyphosate was independently discovered in the United States at Monsanto in 1970. About 100 derivatives of aminomethylphosphonic acid had been prepared as potential water-softening agents. Two were found to have weak herbicidal activity, and John E. Franz, a chemist at Monsanto, was asked to try to make analogs with stronger herbicidal activity. Glyphosate was the third analog he made. Franz received the National Medal of Technology of the United States in 1987 and the Perkin Medal for Applied Chemistry in 1990 for his discoveries.

</details>

**Weird fact** · written · 5.4 s

> Glyphosate interferes with the shikimate pathway, which produces the aromatic amino acids phenylalanine, tyrosine and tryptophan in plants and microorganisms – but does not exist in the genome of animals, including humans.

<details><summary>Sources</summary>

- [Wikipedia, Mode of action](https://en.wikipedia.org/wiki/Glyphosate#Mode_of_action): Glyphosate interferes with the shikimate pathway, which produces the aromatic amino acids phenylalanine, tyrosine and tryptophan in plants and microorganisms – but does not exist in the genome of animals, including humans. It blocks this pathway by inhibiting the enzyme 5-enolpyruvylshikimate-3-phosphate synthase (EPSPS), which catalyzes the reaction of shikimate-3-phosphate (S3P) and phosphoenolpyruvate to form 5-enolpyruvyl-shikimate-3-phosphate (EPSP). Glyphosate is absorbed through foliage and minimally through roots, meaning that it is only effective on actively growing plants and cannot prevent seeds from germinating.

</details>

**Where you meet it** · written · 6.3 s

> Glyphosate is the active ingredient in weed killer products such as RoundUp™. Glyphosate products are one of the most widely used weed killers worldwide in farms and in home gardens and lawns. In 2007, glyphosate was the most used herbicide in the United States' agricultural sector and the second-most used (after 2,4-D) in home and garden, government and industry, and commercial applications.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/3496#section=Record-Description): Glyphosate is the active ingredient in weed killer products such as RoundUp™. Glyphosate products are one of the most widely used weed killers worldwide in farms and in home gardens and lawns. These products typically contain glyphosate in combination with other ingredients that help improve the absorption of the glyphosate into the plant. Glyphosate-based formulations (GBFs) are easily bought in most stores. These products can have different combinations of other ingredients or different concentrations of glyphosate.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Glyphosate): Farmers quickly adopted glyphosate for agricultural weed control, especially after Monsanto introduced glyphosate-resistant Roundup Ready crops, enabling farmers to kill weeds without killing their crops. In 2007, glyphosate was the most used herbicide in the United States' agricultural sector and the second-most used (after 2,4-D) in home and garden, government and industry, and commercial applications. From the late 1970s to 2016, there was a 100-fold increase in the frequency and volume of application of GBHs worldwide, with further increases expected in the future.

</details>


### hydrogen cyanide (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Hydrogen Cyanide carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 1.4 s

> The new component was what is now known as hydrogen cyanide. It was subsequently prepared from Prussian blue by the Swedish chemist Carl Wilhelm Scheele in 1782, and was eventually given the German name Blausäure (lit. 'Blue acid') because of its acidic nature in water and its derivation from Prussian blue. In English, it became known popularly as prussic acid.

<details><summary>Sources</summary>

- [Wikipedia, History of discovery and naming](https://en.wikipedia.org/wiki/Hydrogen_cyanide#History_of_discovery_and_naming): Hydrogen cyanide was first isolated in 1752 by French chemist Pierre Macquer who converted Prussian blue to an iron oxide plus a volatile component and found that these could be used to reconstitute it. The new component was what is now known as hydrogen cyanide. It was subsequently prepared from Prussian blue by the Swedish chemist Carl Wilhelm Scheele in 1782, and was eventually given the German name Blausäure (lit. 'Blue acid') because of its acidic nature in water and its derivation from Prussian blue. In English, it became known popularly as prussic acid.

</details>

**Where you meet it** · written · 6.2 s

> HCN is obtainable from fruits that have a pit, such as cherries, apricots, apples, and nuts such as bitter almonds, from which almond oil and extract is made. Many of these pits contain small amounts of cyanohydrins such as mandelonitrile and amygdalin, which slowly release hydrogen cyanide. Used to manufacture other chemicals and to fumigate enclosed spaces; [ATSDR ToxProfiles] Used in the following processes: electroplating, metallurgy, and photo development; HCN is a byproduct of blast furnaces, coke ovens, photoengraving, petroleum refining, and some metal mining processes.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Hydrogen_cyanide#Occurrence): HCN is obtainable from fruits that have a pit, such as cherries, apricots, apples, and nuts such as bitter almonds, from which almond oil and extract is made. Many of these pits contain small amounts of cyanohydrins such as mandelonitrile and amygdalin, which slowly release hydrogen cyanide. One hundred grams of crushed apple seeds can yield about 70 mg of HCN.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/768#section=Uses): Used to manufacture other chemicals and to fumigate enclosed spaces; [ATSDR ToxProfiles] Used in the following processes: electroplating, metallurgy, and photo development; HCN is a byproduct of blast furnaces, coke ovens, photoengraving, petroleum refining, and some metal mining processes. HCN is released when burning nitrogen containing compounds, e.g., wool, silk, and plastics. HCN is present in plants such as cassava roots, lima beans, and almonds. [CHEMINFO]

</details>

**Weird fact** · written · 5.7 s

> About half of people are unable to detect the odor of hydrogen cyanide owing to a recessive genetic trait. Whether hydrogen cyanide is an organic compound or not is a topic of debate among chemists.

<details><summary>Sources</summary>

- [Wikipedia, Structure and general properties](https://en.wikipedia.org/wiki/Hydrogen_cyanide#Structure_and_general_properties): About half of people are unable to detect the odor of hydrogen cyanide owing to a recessive genetic trait. For those that can detect it, the odor has been described as bitter almond-like.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Hydrogen_cyanide): Whether hydrogen cyanide is an organic compound or not is a topic of debate among chemists. It is traditionally considered inorganic, but can also be considered a nitrile, giving rise to its alternative names of methanenitrile and formonitrile.

</details>


### ibuprofen (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Ibuprofen, (+-)- carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · written · 2.1 s

> Ibuprofen was derived from propionic acid by the research arm of Boots Group during the 1960s. The molecule was discovered and synthesized by a team led by Stewart Adams, with a patent application filed in 1961. In recognition of the pioneering research work, here on Pennyfoot Street, by Dr Stewart Adams and Dr John Nicholson in the Research Department of Boots, which led to the discovery of ibuprofen used by millions worldwide for the relief of pain.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Ibuprofen#History): Ibuprofen was derived from propionic acid by the research arm of Boots Group during the 1960s. The name is derived from the 3 functional groups: isobutyl (ibu) propionic acid (pro) phenyl (fen). Its discovery was the result of research during the 1950s and 1960s to find a safer alternative to aspirin. The molecule was discovered and synthesized by a team led by Stewart Adams, with a patent application filed in 1961. Adams initially tested the drug as treatment for his hangover. In 1985, Boots's worldwide patent for ibuprofen expired and generic products were launched.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Ibuprofen#History): In recognition of the pioneering research work, here on Pennyfoot Street, by Dr Stewart Adams and Dr John Nicholson in the Research Department of Boots, which led to the discovery of ibuprofen used by millions worldwide for the relief of pain.

</details>

**Where you meet it** · written · 5.2 s

> Ibuprofen is used primarily to treat fever (including post-vaccination fever), mild to moderate pain (including pain relief after surgery), painful menstruation, osteoarthritis, dental pain, headaches, and pain from kidney stones. Ingredient in over-the-counter pain relievers

<details><summary>Sources</summary>

- [Wikipedia, Medical uses](https://en.wikipedia.org/wiki/Ibuprofen#Medical_uses): Ibuprofen is used primarily to treat fever (including post-vaccination fever), mild to moderate pain (including pain relief after surgery), painful menstruation, osteoarthritis, dental pain, headaches, and pain from kidney stones. About 60% of people respond to any NSAID; those who do not respond well to a particular one may respond to another. A Cochrane medical review of 51 trials of NSAIDs for the treatment of lower back pain found that "NSAIDs are effective for short-term symptomatic relief in patients with acute low back pain".
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/3672#section=Uses): Ingredient in over-the-counter pain relievers

</details>

**Weird fact** · written · 7.2 s

> Ibuprofen has been associated with a lower risk of Parkinson's disease and may delay or prevent it. Aspirin, other NSAIDs, and paracetamol (acetaminophen) did not affect the risk for Parkinson's. In March 2011, researchers at Harvard Medical School announced that ibuprofen had a neuroprotective effect against the risk of developing Parkinson's disease.

<details><summary>Sources</summary>

- [Wikipedia, Research](https://en.wikipedia.org/wiki/Ibuprofen#Research): Ibuprofen has been associated with a lower risk of Parkinson's disease and may delay or prevent it. Aspirin, other NSAIDs, and paracetamol (acetaminophen) did not affect the risk for Parkinson's. In March 2011, researchers at Harvard Medical School announced that ibuprofen had a neuroprotective effect against the risk of developing Parkinson's disease. People regularly consuming ibuprofen were reported to have a 38% lower risk of developing Parkinson's disease, but no such effect was found for other pain relievers, such as aspirin and paracetamol.

</details>


### indigo (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Indigo carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · skipped · 2.6 s

**Where you meet it** · written · 2.2 s

> Indigo is a natural dye obtained from the leaves of some plants of the Indigofera genus, in particular Indigofera tinctoria. Most indigo dye produced today is synthetic, constituting around 80,000 tonnes each year, as of 2023. It is most commonly associated with the production of denim cloth and blue jeans, where its properties allow for effects such as stone washing and acid washing to be applied quickly.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Indigo_dye): Indigo dye is an organic compound with a distinctive blue color. Indigo is a natural dye obtained from the leaves of some plants of the Indigofera genus, in particular Indigofera tinctoria. Dye-bearing Indigofera plants were once common throughout the world. Blue colorants are relatively rare. Since indigo is insoluble, it is also referred to as a pigment (C.I. Pigment Blue 66, C.I.).
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Indigo_dye): Most indigo dye produced today is synthetic, constituting around 80,000 tonnes each year, as of 2023. It is most commonly associated with the production of denim cloth and blue jeans, where its properties allow for effects such as stone washing and acid washing to be applied quickly.

</details>

**Weird fact** · written · 6.4 s

> Another pre-industrial method, used in Japan, was to dissolve the indigo in a heated vat in which a culture of thermophilic, anaerobic bacteria was maintained. Some species of such bacteria generate hydrogen as a metabolic product, which convert insoluble indigo into soluble indigo white.

<details><summary>Sources</summary>

- [Wikipedia, Dyeing technology](https://en.wikipedia.org/wiki/Indigo_dye#Dyeing_technology): A pre-industrial process for production of indigo white, used in Europe, was to dissolve the indigo in stale urine, which contains ammonia. A more convenient reductive agent is zinc. Another pre-industrial method, used in Japan, was to dissolve the indigo in a heated vat in which a culture of thermophilic, anaerobic bacteria was maintained. Some species of such bacteria generate hydrogen as a metabolic product, which convert insoluble indigo into soluble indigo white. Cloth dyed in such a vat was decorated with the techniques of shibori (tie-dye), kasuri, katazome, and tsutsugaki.

</details>


### limonene (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> 1-Methyl-4-Prop-1-En-2-Ylcyclohexene carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · written · 4.0 s

> Limonene is used as a solvent for cleaning purposes, such as adhesive remover, or the removal of oil from machine parts, as it is produced from a renewable source (citrus essential oil, as a byproduct of orange juice manufacture). Limonene is found in many oils and fruits including orange, lemon, grapefruit, berry, leaf, caraway, dill, bergamot, peppermint and spearmint oils.

<details><summary>Sources</summary>

- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Limonene#Uses): Limonene is used as a solvent for cleaning purposes, such as adhesive remover, or the removal of oil from machine parts, as it is produced from a renewable source (citrus essential oil, as a byproduct of orange juice manufacture). It is used as a paint stripper and is also useful as a fragrant alternative to turpentine. Limonene is also used as a solvent in some model airplane glues and as a constituent in some paints. Commercial air fresheners, with air propellants, containing limonene are used by stamp collectors to remove self-adhesive postage stamps from envelope paper.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/22311#section=Natural-Pollution-Sources): Limonene is found in many oils and fruits including orange, lemon, grapefruit, berry, leaf, caraway, dill, bergamot, peppermint and spearmint oils. Limonene emissions to the environment are associated with wax myrtle, sweet acacia, oranges, tomatoes, grasses, and California western sagebrush. Emissions of limonene are also associated with balsam poplar, European larche, European fir, Scots pine, Siberian pine, silver fir, common juniper, zeravshan juniper, pencil cedar, evergreen cypress, northern white cedar, chinese arbor vitae, marsh tea and deciduous moss.

</details>

**Weird fact** · skipped · 4.2 s


### lycopene (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Lycopene is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Where you meet it** · written · 3.2 s

> Lycopene is an acyclic carotene commonly obtained from tomatoes and other red fruits. Aside from tomatoes or tomato products like ketchup, it is found in watermelons, grapefruits, red guavas, and baked beans.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/446925#section=Record-Description): Lycopene is an acyclic carotene commonly obtained from tomatoes and other red fruits. It has a role as an antioxidant and a plant metabolite. It contains a carotenoid psi-end derivative.
- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Lycopene#Occurrence): Aside from tomatoes or tomato products like ketchup, it is found in watermelons, grapefruits, red guavas, and baked beans. It has no vitamin A activity.

</details>

**Weird fact** · written · 6.1 s

> Lycopene may undergo extensive isomerization that allows 1056 theoretical cis-trans configurations; however, the all-trans configuration of lycopene is the most predominant isomer found in foods that gives the red hue. Lycopene is a linear, unsaturated hydrocarbon carotenoid, the major red pigment in fruits such as tomatoes, pink grapefruit, apricots, red oranges, watermelon, rosehips, and guava.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/446925#section=Record-Description): Lycopene is a naturally occurring red carotenoid pigment that is responsible for red to pink colors seen in tomatoes, pink grapefruit, and other foods. Having a chemical formula of C40H56, lycopene is a tetraterpene assembled from eight isoprene units that are solely composed of carbon and hydrogen. Lycopene may undergo extensive isomerization that allows 1056 theoretical cis-trans configurations; however, the all-trans configuration of lycopene is the most predominant isomer found in foods that gives the red hue.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/446925#section=Record-Description): Lycopene is a linear, unsaturated hydrocarbon carotenoid, the major red pigment in fruits such as tomatoes, pink grapefruit, apricots, red oranges, watermelon, rosehips, and guava. As a class, carotenoids are pigment compounds found in photosynthetic organisms (plants, algae, and some types of fungus), and are chemically characterized by a large polyene chain containing 35-40 carbon atoms; some carotenoid polyene chains are terminated by two 6-carbon rings. In animals, carotenoids such as lycopene may possess antioxidant properties which may retard aging and many degenerative diseases. As an essential nutrient, lycopene is required in the animal diet. (NCI04)

</details>


### melatonin (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Melatonin is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · written · 2.6 s

> Melatonin's discovery is linked to the study of color changes in some amphibians and reptiles, a phenomenon initially observed through the administration of pineal gland extracts. The hormone melatonin was isolated in 1958 by Aaron B. Lerner, a dermatology professor, and his team at Yale University. Motivated by the possibility that a substance from the pineal gland could be beneficial in treating skin diseases, they extracted and identified melatonin from bovine pineal gland extracts.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Melatonin#History): Melatonin's discovery is linked to the study of color changes in some amphibians and reptiles, a phenomenon initially observed through the administration of pineal gland extracts. In 1917, Carey Pratt McCord and Floyd P. Allen found that feeding extracts from the pineal glands of cows caused the skin of tadpoles to lighten by contracting the dark epidermal melanophores.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Melatonin#History): The hormone melatonin was isolated in 1958 by Aaron B. Lerner, a dermatology professor, and his team at Yale University. Motivated by the possibility that a substance from the pineal gland could be beneficial in treating skin diseases, they extracted and identified melatonin from bovine pineal gland extracts. Subsequent research in the mid-1970s by Lynch and others demonstrated that melatonin production follows a circadian rhythm in human pineal glands.

</details>

**Where you meet it** · written · 6.0 s

> Melatonin is a hormone produced by the pineal gland that has multiple effects including somnolence, and is believed to play a role in regulation of the sleep-wake cycle. Melatonin is sometimes administered exogenously as a dietary supplement and medication. Melatonin has been identified in many plants including feverfew (Tanacetum parthenium), St John's wort (Hypericum perforatum), rice, corn, tomato and other edible fruits.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/896#section=Record-Description): Melatonin is a hormone produced by the pineal gland that has multiple effects including somnolence, and is believed to play a role in regulation of the sleep-wake cycle. Melatonin is available over-the-counter and is reported to have beneficial effects on wellbeing and sleep. Melatonin has not been implicated in causing serum enzyme elevations or clinically apparent liver injury.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Melatonin): Melatonin is sometimes administered exogenously as a dietary supplement and medication. It is used in the treatment of sleep-related disorders such as insomnia and various circadian rhythm sleep disorders.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/896#section=Natural-Pollution-Sources): Melatonin is secreted from the pineal gland and retina during dark periods of the vertebrate circadian rhythm and is formed from serotonin. Melatonin, a hormone of the pineal gland, is also produced by extrapineal tissues that lighten skin color in amphibians. Melatonin has been identified in many plants including feverfew (Tanacetum parthenium), St John's wort (Hypericum perforatum), rice, corn, tomato and other edible fruits.

</details>

**Weird fact** · written · 6.1 s

> As detailed in their publication in the Journal of the American Chemical Society, Lerner and his colleagues proposed the name melatonin, derived from the Greek words melas, meaning 'black' or 'dark', and tonos, meaning 'labour', 'colour' or 'suppress'. This naming convention follows that of serotonin, another agent affecting skin color, discovered in 1948 as a modulator of vascular tone, which influenced its name based on its serum vasoconstrictor effect.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Melatonin#History): The etymology of melatonin stems from its skin-lightening properties. As detailed in their publication in the Journal of the American Chemical Society, Lerner and his colleagues proposed the name melatonin, derived from the Greek words melas, meaning 'black' or 'dark', and tonos, meaning 'labour', 'colour' or 'suppress'. This naming convention follows that of serotonin, another agent affecting skin color, discovered in 1948 as a modulator of vascular tone, which influenced its name based on its serum vasoconstrictor effect.

</details>


### menthol (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Menthol carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 1.4 s

> Menthol was first isolated in 1771, by the German, Hieronymus David Gaubius. Early characterizations were done by Oppenheim, Beckett, Moriya, and Atkinson. It was named by F. L. Alphonse Oppenheim (1833–1877) in 1861.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Menthol#History): Menthol was first isolated in 1771, by the German, Hieronymus David Gaubius. Early characterizations were done by Oppenheim, Beckett, Moriya, and Atkinson. It was named by F. L. Alphonse Oppenheim (1833–1877) in 1861.

</details>

**Where you meet it** · written · 3.5 s

> Menthol is a monoterpenoid organic compound that occurs naturally in the oils of certain plants in the mint family, such as corn mint and peppermint. For many people, menthol produces a cooling sensation when inhaled, eaten, or applied to the skin, and mint plants have been used for centuries for topical pain relief and as a food flavoring. Menthol occurs naturally in peppermint oil (along with a little menthone, the ester menthyl acetate and other compounds), obtained from Mentha × piperita (peppermint).

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Menthol): Menthol is a monoterpenoid organic compound that occurs naturally in the oils of certain plants in the mint family, such as corn mint and peppermint. It is a white or clear waxy crystalline substance that is solid at room temperature and melts slightly above. The main form of menthol occurring in nature is (−)-menthol, which is assigned the (1R,2S,5R) configuration.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Menthol): For many people, menthol produces a cooling sensation when inhaled, eaten, or applied to the skin, and mint plants have been used for centuries for topical pain relief and as a food flavoring. Menthol has local anesthetic and counterirritant qualities, and it is widely used to relieve minor throat irritation.
- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Menthol#Occurrence): Menthol occurs naturally in peppermint oil (along with a little menthone, the ester menthyl acetate and other compounds), obtained from Mentha × piperita (peppermint). Japanese menthol also contains a small percentage of the 1-epimer neomenthol.

</details>

**Weird fact** · written · 6.2 s

> In the natural compound, the isopropyl group is in the trans orientation to both the methyl and hydroxyl groups. The main form of menthol occurring in nature is (−)-menthol, which is assigned the (1R,2S,5R) configuration.

<details><summary>Sources</summary>

- [Wikipedia, Structure](https://en.wikipedia.org/wiki/Menthol#Structure): In the natural compound, the isopropyl group is in the trans orientation to both the methyl and hydroxyl groups. Thus, it can be drawn in any of the ways shown:
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Menthol): Menthol is a monoterpenoid organic compound that occurs naturally in the oils of certain plants in the mint family, such as corn mint and peppermint. It is a white or clear waxy crystalline substance that is solid at room temperature and melts slightly above. The main form of menthol occurring in nature is (−)-menthol, which is assigned the (1R,2S,5R) configuration.

</details>


### morphine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Morphine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 5 more hazards, not listed here.

**Origin story** · written · 2.1 s

> Morphine was discovered as the first active alkaloid extracted from the opium poppy plant in December 1804 in Paderborn by German pharmacist Friedrich Sertürner. Sertürner noticed the compound induced drowsiness in rats and stray dogs and he reported his findings in 1805.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Morphine#History): Morphine was discovered as the first active alkaloid extracted from the opium poppy plant in December 1804 in Paderborn by German pharmacist Friedrich Sertürner. Sertürner noticed the compound induced drowsiness in rats and stray dogs and he reported his findings in 1805. In 1817, Sertürner reported experiments in which he administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died. Sertürner originally named the substance morphium after the Greek god of dreams, Morpheus, as it has a tendency to cause sleep. Sertürner's morphium was six times stronger than opium.

</details>

**Weird fact** · written · 5.1 s

> In 1817, Sertürner reported experiments in which he administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died. Sertürner's morphium was six times stronger than opium.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Morphine#History): Morphine was discovered as the first active alkaloid extracted from the opium poppy plant in December 1804 in Paderborn by German pharmacist Friedrich Sertürner. Sertürner noticed the compound induced drowsiness in rats and stray dogs and he reported his findings in 1805. In 1817, Sertürner reported experiments in which he administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died. Sertürner originally named the substance morphium after the Greek god of dreams, Morpheus, as it has a tendency to cause sleep. Sertürner's morphium was six times stronger than opium.

</details>

**Where you meet it** · written · 6.3 s

> Morphine is the most abundant opiate found in opium, the dried latex extracted by shallowly scoring the unripe seedpods of the Papaver somniferum poppy. Morphine is obtained from opium, which itself is obtained from the milky exudate of incised unripe seed capsules of the poppy plant, Papaver somniferum, indigenous to Asia Minor.

<details><summary>Sources</summary>

- [Wikipedia, Natural occurrence](https://en.wikipedia.org/wiki/Morphine#Natural_occurrence): Morphine is the most abundant opiate found in opium, the dried latex extracted by shallowly scoring the unripe seedpods of the Papaver somniferum poppy. Morphine is generally 8–14% of the dry weight of opium. Przemko and Norman cultivars of the opium poppy, are used to produce two other alkaloids, thebaine and oripavine, which are used in the manufacture of semi-synthetic and synthetic opioids like oxycodone and etorphine. P. bracteatum does not contain morphine or codeine, or other narcotic phenanthrene-type, alkaloids. This species is rather a source of thebaine.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/5288826#section=Natural-Pollution-Sources): Morphine is obtained from opium, which itself is obtained from the milky exudate of incised unripe seed capsules of the poppy plant, Papaver somniferum, indigenous to Asia Minor. The alkaloids of opium include phenanthrenes (morphine, codeine, thebaine) and benzyl isoquinolines (papaverine, noscapine).

</details>


### nicotine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Nicotine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 1.7 s

> Nicotine was originally isolated from the tobacco plant in 1828 by chemists Wilhelm Heinrich Posselt and Karl Ludwig Reimann from Germany, who believed it was a poison. Nicotine is named after the tobacco plant Nicotiana tabacum, which in turn is named after the French ambassador in Portugal, Jean Nicot de Villemain, who sent tobacco and seeds to Paris in 1560, presented to the French King, and who promoted their medicinal use.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Nicotine#History): Nicotine was originally isolated from the tobacco plant in 1828 by chemists Wilhelm Heinrich Posselt and Karl Ludwig Reimann from Germany, who believed it was a poison. Its chemical empirical formula was described by Melsens in 1843, its structure was discovered by Adolf Pinner and Richard Wolffenstein in 1893, and it was first synthesized by Amé Pictet and A. Rotschy in 1904.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Nicotine#History): Nicotine is named after the tobacco plant Nicotiana tabacum, which in turn is named after the French ambassador in Portugal, Jean Nicot de Villemain, who sent tobacco and seeds to Paris in 1560, presented to the French King, and who promoted their medicinal use. Smoking was believed to protect against illness, particularly the plague. However, the "holy herb", tobacco, had first reached Europe by the early 1530s, brought by Spanish explorers.

</details>

**Where you meet it** · written · 4.7 s

> In tobacco leaves, nicotine constitutes about 0.6–3% of the dry weight, and smaller, trace quantities occur in other Solanaceae crops such as tomatoes, potatoes, and eggplants. SMOKING TOBACCO USUALLY CONTAINS 1-2% NICOTINE.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Nicotine): Nicotine is a stimulant and alkaloid found primarily in plants of the nightshade family, notably in tobacco; it is also synthesized. Nicotine is used recreationally for its stimulant and anxiolytic effects. In tobacco leaves, nicotine constitutes about 0.6–3% of the dry weight, and smaller, trace quantities occur in other Solanaceae crops such as tomatoes, potatoes, and eggplants. In pure form, nicotine is a colorless to yellowish, oily liquid that readily penetrates biological membranes and acts as a potent neurotoxin in insects, where it serves as an antiherbivore toxin.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/89594#section=Natural-Pollution-Sources): SMOKING TOBACCO USUALLY CONTAINS 1-2% NICOTINE.

</details>

**Weird fact** · written · 3.9 s

> Following its introduction, tobacco rapidly gained popularity in Europe for its stimulating effects, fueled by nicotine's addictive and pharmacological properties and tobacco's widespread embrace as a cure-all. The CDC says it is "toxic to developing fetuses and is a health danger for pregnant women." It can harm brain development up to age twenty-five, and early use of nicotine can predispose young people to smoking and drug use. However, at doses typically used by consumers, it presents little if any hazard to adult users.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Nicotine#History): Following its introduction, tobacco rapidly gained popularity in Europe for its stimulating effects, fueled by nicotine's addictive and pharmacological properties and tobacco's widespread embrace as a cure-all. By the early 17th century, this allure fueled its mercilessly laborious cultivation as a cash crop in the Virginia colonies, where John Rolfe's introduction in 1612 rescued Jamestown from economic collapse and famine, transforming it into a prosperous export hub with over 20,000 pounds shipped by 1619 and laying the groundwork for transatlantic trade.
- [Wikipedia, Adverse effects](https://en.wikipedia.org/wiki/Nicotine#Adverse_effects): Nicotine is classified as a poison, and it is "extremely hazardous". The CDC says it is "toxic to developing fetuses and is a health danger for pregnant women." It can harm brain development up to age twenty-five, and early use of nicotine can predispose young people to smoking and drug use. However, at doses typically used by consumers, it presents little if any hazard to adult users. Although at low amounts nicotine has a mild analgesic effect, at sufficiently high doses nicotine may result in nausea, vomiting, diarrhea, salivation, bradycardia, and possibly seizures, hypoventilation, and death.

</details>


### nitroglycerin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Nitroglycerin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 1.3 s

> Nitroglycerin was the first practical explosive produced that was stronger than black powder. It was synthesized by the Italian chemist Ascanio Sobrero in 1846, working under Théophile-Jules Pelouze at the University of Turin. Sobrero initially called his discovery "pyroglycerin" and warned vigorously against its use as an explosive.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Nitroglycerin#History): Nitroglycerin was the first practical explosive produced that was stronger than black powder. It was synthesized by the Italian chemist Ascanio Sobrero in 1846, working under Théophile-Jules Pelouze at the University of Turin. Sobrero initially called his discovery "pyroglycerin" and warned vigorously against its use as an explosive.

</details>

**Where you meet it** · written · 5.2 s

> Dynamite and similar explosives were widely adopted for civil engineering tasks, such as in drilling highway and railroad tunnels, for mining, for clearing farmland of stumps, in quarrying, and in demolition work. In medicine, nitroglycerin is probably most commonly prescribed for angina pectoris, a painful symptom of ischemic heart disease caused by inadequate flow of blood and oxygen to the heart, and as a potent antihypertensive agent.

<details><summary>Sources</summary>

- [Wikipedia, Use as an explosive and a propellant](https://en.wikipedia.org/wiki/Nitroglycerin#Use_as_an_explosive_and_a_propellant): He named this explosive dynamite and patented it in 1867. It was supplied ready for use in the form of sticks, individually wrapped in greased waterproof paper. Dynamite and similar explosives were widely adopted for civil engineering tasks, such as in drilling highway and railroad tunnels, for mining, for clearing farmland of stumps, in quarrying, and in demolition work. Likewise, military engineers have used dynamite for construction and demolition work.
- [Wikipedia, Medical use](https://en.wikipedia.org/wiki/Nitroglycerin#Medical_use): In medicine, nitroglycerin is probably most commonly prescribed for angina pectoris, a painful symptom of ischemic heart disease caused by inadequate flow of blood and oxygen to the heart, and as a potent antihypertensive agent. Nitroglycerin corrects the imbalance between the flow of oxygen and blood to the heart and the heart's energy demand. There are many formulations on the market at different doses. At low doses, nitroglycerin dilates veins more than arteries, thereby reducing preload (volume of blood in the heart after filling); this is thought to be its primary mechanism of action.

</details>

**Weird fact** · written · 4.8 s

> In its undiluted form, nitroglycerin is a contact explosive, meaning physical shock causes it to explode. Infrequent exposure to high doses of nitroglycerin can cause severe headaches known as "NG head" or "bang head".

<details><summary>Sources</summary>

- [Wikipedia, Instability and desensitization](https://en.wikipedia.org/wiki/Nitroglycerin#Instability_and_desensitization): In its undiluted form, nitroglycerin is a contact explosive, meaning physical shock causes it to explode. If it has not been adequately purified during manufacture, it can degrade over time to even more unstable forms. This makes nitroglycerin highly dangerous to transport or use. In its undiluted form, it is one of the world's most powerful explosives, comparable to the more recently developed RDX and PETN.
- [Wikipedia, Industrial exposure](https://en.wikipedia.org/wiki/Nitroglycerin#Industrial_exposure): Infrequent exposure to high doses of nitroglycerin can cause severe headaches known as "NG head" or "bang head". These headaches can be severe enough to incapacitate some people; however, humans develop a tolerance to and dependence on nitroglycerin after long-term exposure. Although rare, withdrawal can be fatal; symptoms include chest pain and other heart problems. These symptoms may be relieved with re-exposure to nitroglycerin or other suitable organic nitrates.

</details>


### oxytocin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Oxytocin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 2.1 s

> The uterine-contracting properties of the principle that would later be named oxytocin were discovered by British pharmacologist Henry Hallett Dale in 1906, and its milk ejection property was described by Ott and Scott in 1910 and by Schafer and Mackenzie in 1911. In 1909 the first clinical use of oxytocin was performed by William Blair-Bell to induce childbirth in patients with complications.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Oxytocin#History): The uterine-contracting properties of the principle that would later be named oxytocin were discovered by British pharmacologist Henry Hallett Dale in 1906, and its milk ejection property was described by Ott and Scott in 1910 and by Schafer and Mackenzie in 1911. In 1909 the first clinical use of oxytocin was performed by William Blair-Bell to induce childbirth in patients with complications.

</details>

**Where you meet it** · written · 3.0 s

> Oxytocin is released into the bloodstream as a hormone in response to sexual activity and during childbirth. It is also available in pharmaceutical form.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Oxytocin): Oxytocin is a peptide hormone and neuropeptide normally produced in the hypothalamus and released by the posterior pituitary. Present in animals since early stages of evolution, in humans it plays roles in behavior that include social bonding, love, reproduction, childbirth, and the period after childbirth. Oxytocin is released into the bloodstream as a hormone in response to sexual activity and during childbirth. It is also available in pharmaceutical form. In either form, oxytocin stimulates uterine contractions to speed up the process of childbirth.

</details>

**Weird fact** · written · 6.0 s

> While the structure of oxytocin is highly conserved in placental mammals, a novel structure of oxytocin was reported in 2011 in marmosets, tamarins, and other New World primates. Genomic sequencing of the gene for oxytocin revealed a single in-frame mutation (thymine for cytosine) which results in a single amino acid substitution at the 8-position (proline for leucine). Since this original Lee et al. paper, two other laboratories have confirmed Pro8-OT and documented additional oxytocin structural variants in this primate taxon.

<details><summary>Sources</summary>

- [Wikipedia, Chemistry](https://en.wikipedia.org/wiki/Oxytocin#Chemistry): While the structure of oxytocin is highly conserved in placental mammals, a novel structure of oxytocin was reported in 2011 in marmosets, tamarins, and other New World primates. Genomic sequencing of the gene for oxytocin revealed a single in-frame mutation (thymine for cytosine) which results in a single amino acid substitution at the 8-position (proline for leucine). Since this original Lee et al. paper, two other laboratories have confirmed Pro8-OT and documented additional oxytocin structural variants in this primate taxon. Vargas-Pinilla et al.

</details>


### quinine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Quinine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 3 more hazards, not listed here.

**Origin story** · written · 2.0 s

> The form of quinine most effective in treating malaria was found by Charles Marie de La Condamine in 1737. In 1820, French researchers Pierre Joseph Pelletier and Joseph Bienaimé Caventou first isolated quinine from the bark of a tree in the genus Cinchona – probably Cinchona pubescens – and subsequently named the substance.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Quinine#History): The form of quinine most effective in treating malaria was found by Charles Marie de La Condamine in 1737. In 1820, French researchers Pierre Joseph Pelletier and Joseph Bienaimé Caventou first isolated quinine from the bark of a tree in the genus Cinchona – probably Cinchona pubescens – and subsequently named the substance. The name was derived from the original Quechua (Inca) word for the cinchona tree bark, quina or quina-quina, which means "bark of bark" or "holy bark". Prior to 1820, the bark was dried, ground to a fine powder, and mixed into a liquid (commonly wine) in order to be drunk. Large-scale use of quinine as a malaria prophylaxis started around 1850.

</details>

**Where you meet it** · written · 5.9 s

> Malaria resistance to quinine occurs in certain areas of the world. Quinine is also used as an ingredient in tonic water and other beverages to impart a bitter taste. Used as an antipyretic (fever-reducing), antimalarial, analgesic (painkilling), and anti-inflammatory. It is also used as a flavor component of tonic water and bitter lemon.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Quinine): Quinine is an alkaloid used medically to treat malaria and babesiosis. This includes the treatment of malaria due to Plasmodium falciparum that is resistant to chloroquine when artesunate is not available. While sometimes used for nocturnal leg cramps, quinine is not recommended for this purpose due to the risk of serious side effects. It can be taken by mouth or intravenously. Malaria resistance to quinine occurs in certain areas of the world. Quinine is also used as an ingredient in tonic water and other beverages to impart a bitter taste.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/3034034#section=Uses): For the treatment of malaria and leg cramps. Used as an antipyretic (fever-reducing), antimalarial, analgesic (painkilling), and anti-inflammatory. It is also used as a flavor component of tonic water and bitter lemon.

</details>

**Weird fact** · written · 4.7 s

> The name was derived from the original Quechua (Inca) word for the cinchona tree bark, quina or quina-quina, which means "bark of bark" or "holy bark". Quinine can cause unpredictable serious and life-threatening blood and cardiovascular reactions including low platelet count and hemolytic–uremic syndrome/thrombotic thrombocytopenic purpura (HUS/TTP), long QT syndrome and other serious cardiac arrhythmias including torsades de pointes, blackwater fever, disseminated intravascular coagulation, leukopenia, and neutropenia.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Quinine#History): The form of quinine most effective in treating malaria was found by Charles Marie de La Condamine in 1737. In 1820, French researchers Pierre Joseph Pelletier and Joseph Bienaimé Caventou first isolated quinine from the bark of a tree in the genus Cinchona – probably Cinchona pubescens – and subsequently named the substance. The name was derived from the original Quechua (Inca) word for the cinchona tree bark, quina or quina-quina, which means "bark of bark" or "holy bark". Prior to 1820, the bark was dried, ground to a fine powder, and mixed into a liquid (commonly wine) in order to be drunk. Large-scale use of quinine as a malaria prophylaxis started around 1850.
- [Wikipedia, Adverse effects](https://en.wikipedia.org/wiki/Quinine#Adverse_effects): Quinine can cause unpredictable serious and life-threatening blood and cardiovascular reactions including low platelet count and hemolytic–uremic syndrome/thrombotic thrombocytopenic purpura (HUS/TTP), long QT syndrome and other serious cardiac arrhythmias including torsades de pointes, blackwater fever, disseminated intravascular coagulation, leukopenia, and neutropenia. Some people who have developed TTP due to quinine have gone on to develop kidney failure.

</details>


### serotonin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Serotonin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 1.6 s

> In 1935, Italian Vittorio Erspamer, working in Pavia, showed an extract from enterochromaffin cells made intestines contract. Some believed it contained adrenaline, but two years later, Erspamer was able to show it was a previously unknown amine, which he named "enteramine". In 1948, Maurice M. Rapport, Arda Green, and Irvine Page of the Cleveland Clinic discovered a vasoconstrictor substance in blood serum, and since it was a serum agent affecting vascular tone, they named it serotonin.

<details><summary>Sources</summary>

- [Wikipedia, History & etymology](https://en.wikipedia.org/wiki/Serotonin#History_&_etymology): It had been known to physiologists for over a century that a vasoconstrictor material appears in serum when blood was allowed to clot. In 1935, Italian Vittorio Erspamer, working in Pavia, showed an extract from enterochromaffin cells made intestines contract. Some believed it contained adrenaline, but two years later, Erspamer was able to show it was a previously unknown amine, which he named "enteramine". In 1948, Maurice M. Rapport, Arda Green, and Irvine Page of the Cleveland Clinic discovered a vasoconstrictor substance in blood serum, and since it was a serum agent affecting vascular tone, they named it serotonin.

</details>

**Where you meet it** · written · 3.1 s

> Most of the body's serotonin—about 90%—is synthesized in the gastrointestinal tract by enterochromaffin cells, where it regulates intestinal movements. It is also produced in smaller amounts in the brainstem's raphe nuclei, the skin's Merkel cells, pulmonary neuroendocrine cells, and taste receptor cells of the tongue. Once secreted, serotonin is taken up by platelets in the blood, which release it during clotting to promote vasoconstriction and platelet aggregation.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Serotonin): Most of the body's serotonin—about 90%—is synthesized in the gastrointestinal tract by enterochromaffin cells, where it regulates intestinal movements. It is also produced in smaller amounts in the brainstem's raphe nuclei, the skin's Merkel cells, pulmonary neuroendocrine cells, and taste receptor cells of the tongue. Once secreted, serotonin is taken up by platelets in the blood, which release it during clotting to promote vasoconstriction and platelet aggregation. Around 8% of the body's serotonin is stored in platelets, and 1–2% is found in the CNS.

</details>

**Weird fact** · written · 5.8 s

> The gastrointestinal parasite Entamoeba histolytica secretes serotonin, causing a sustained secretory diarrhea in some people. Patients infected with E. histolytica have been found to have highly elevated serum serotonin levels, which returned to normal following resolution of the infection.

<details><summary>Sources</summary>

- [Wikipedia, Comparative biology & evolution](https://en.wikipedia.org/wiki/Serotonin#Comparative_biology_&_evolution): Serotonin is used by a variety of single-cell organisms for various purposes. SSRIs have been found to be toxic to algae. The gastrointestinal parasite Entamoeba histolytica secretes serotonin, causing a sustained secretory diarrhea in some people. Patients infected with E. histolytica have been found to have highly elevated serum serotonin levels, which returned to normal following resolution of the infection. E. histolytica also responds to the presence of serotonin by becoming more virulent.

</details>


### sodium chloride (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Sodium Chloride carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 3.8 s

> Sodium chloride is the primary salt in seawater and in the extracellular fluid of many multicellular organisms. Saline usually refers to normal or physiological saline, which is an aqueous solution containing 0.9% sodium chloride. Another major application of sodium chloride is de-icing of roadways in sub-freezing weather.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5234#section=Record-Description): Sodium chloride, also known as salt, common salt, table salt or halite, is an ionic compound with the chemical formula NaCl, representing a 1:1 ratio of sodium and chloride ions. Sodium chloride is the primary salt in seawater and in the extracellular fluid of many multicellular organisms. It is listed on the World Health Organization Model List of Essential Medicines.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5234#section=Record-Description): Saline is a solution of salt and water. Saline usually refers to normal or physiological saline, which is an aqueous solution containing 0.9% sodium chloride.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Sodium_chloride): Sodium chloride , is an ionic compound with the chemical formula NaCl, representing a 1:1 ratio of sodium and chloride ions. It is transparent or translucent, brittle, hygroscopic, and occurs as the mineral halite. In its edible form, common table salt, it is commonly used as a condiment and food preservative. Large quantities of sodium chloride are used in many industrial processes, and it is a major source of sodium and chlorine compounds used as feedstocks for further chemical syntheses. Another major application of sodium chloride is de-icing of roadways in sub-freezing weather.

</details>

**Weird fact** · retried · 11.3 s

> In solid sodium chloride, each ion is surrounded by six ions of the opposite charge as expected on electrostatic grounds. The surrounding ions are located at the vertices of a regular octahedron.

<details><summary>Sources</summary>

- [Wikipedia, Chemistry](https://en.wikipedia.org/wiki/Sodium_chloride#Chemistry): In solid sodium chloride, each ion is surrounded by six ions of the opposite charge as expected on electrostatic grounds. The surrounding ions are located at the vertices of a regular octahedron. In the language of close-packing, the larger chloride ions (167 pm in size) are arranged in a cubic array whereas the smaller sodium ions (116 pm) fill all the cubic gaps (octahedral voids) between them. This same basic structure is found in many other compounds and is commonly known as the NaCl structure or rock salt crystal structure. It can be represented as a face-centered cubic (fcc) lattice with a two-atom basis or as two interpenetrating face centered cubic lattices.

</details>


### strychnine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Strychnine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 2.0 s

> Strychnine was first discovered by French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier in 1818 in the Saint-Ignatius' bean.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Strychnine#History): Strychnine was first discovered by French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier in 1818 in the Saint-Ignatius' bean. In some Strychnos plants a 9,10-dimethoxy derivative of strychnine, the alkaloid brucine, is also present. Brucine is not as poisonous as strychnine. Historic records indicate that preparations containing strychnine (presumably) had been used to kill dogs, cats, and birds in Europe as far back as 1640. It was allegedly used by convicted murderer William Palmer to kill his final victim, John Cook. It was also used during World War II by Oskar Dirlewanger against civilians.

</details>

**Where you meet it** · written · 3.8 s

> Strychnine is a plant toxin most commonly found in the seeds of the Strychnine tree (Strychnos nux-vomica L.). The most common source is from the seeds of the Strychnos nux-vomica tree. Strychnine is an alkaloid found in seeds of Strychnos nux vomica Loganiaceae and beans of Strychnos ignatii.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/441071#section=Record-Description): Strychnine is a plant toxin most commonly found in the seeds of the Strychnine tree (Strychnos nux-vomica L.). It is used as a pesticide, particularly for killing small vertebrates such as birds and rodents. Strychnine causes muscular convulsions and eventually death through asphyxia or sheer exhaustion.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Strychnine): Strychnine is a highly toxic, colorless, bitter, crystalline alkaloid used as a pesticide, particularly for killing small vertebrates such as birds and rodents. Strychnine, when inhaled, swallowed, or absorbed through the eyes or mouth, causes poisoning which results in muscular convulsions and eventually death through asphyxia. While it is no longer used medicinally, it was used historically in small doses to strengthen muscle contractions, such as a heart and bowel stimulant and performance-enhancing drug. The most common source is from the seeds of the Strychnos nux-vomica tree.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/441071#section=Natural-Pollution-Sources): Strychnine is an alkaloid found in seeds of Strychnos nux vomica Loganiaceae and beans of Strychnos ignatii.

</details>

**Weird fact** · written · 5.1 s

> Historic records indicate that preparations containing strychnine (presumably) had been used to kill dogs, cats, and birds in Europe as far back as 1640. It was allegedly used by convicted murderer William Palmer to kill his final victim, John Cook. It was also used during World War II by Oskar Dirlewanger against civilians.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Strychnine#History): Strychnine was first discovered by French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier in 1818 in the Saint-Ignatius' bean. In some Strychnos plants a 9,10-dimethoxy derivative of strychnine, the alkaloid brucine, is also present. Brucine is not as poisonous as strychnine. Historic records indicate that preparations containing strychnine (presumably) had been used to kill dogs, cats, and birds in Europe as far back as 1640. It was allegedly used by convicted murderer William Palmer to kill his final victim, John Cook. It was also used during World War II by Oskar Dirlewanger against civilians.

</details>


### sucralose (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Sucralose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · written · 2.1 s

> Sucralose was discovered in 1975 by scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College (now part of King's College London). While researching novel uses of sucrose and its synthetic derivatives, mixing sulfuryl chloride with sugar, Phadnis was told to "test" a chlorinated sugar compound. According to an anecdotal account, Phadnis thought Hough asked him to "taste" it, so he did and found the compound to be exceptionally sweet.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Sucralose#History): Sucralose was discovered in 1975 by scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College (now part of King's College London). While researching novel uses of sucrose and its synthetic derivatives, mixing sulfuryl chloride with sugar, Phadnis was told to "test" a chlorinated sugar compound. According to an anecdotal account, Phadnis thought Hough asked him to "taste" it, so he did and found the compound to be exceptionally sweet. Tate & Lyle patented the substance in 1976; as of 2008, the only remaining patents concerned specific manufacturing processes.

</details>

**Where you meet it** · written · 2.9 s

> Sucralose is used as a replacement for (or in combination with) other artificial or natural sweeteners such as aspartame, acesulfame potassium or high-fructose corn syrup. It is used in products such as candy, breakfast bars, coffee pods, and soft drinks. It is also used in canned fruits wherein water and sucralose take the place of much higher-energy corn syrup-based additives.

<details><summary>Sources</summary>

- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Sucralose#Uses): Sucralose is used as a replacement for (or in combination with) other artificial or natural sweeteners such as aspartame, acesulfame potassium or high-fructose corn syrup. It is used in products such as candy, breakfast bars, coffee pods, and soft drinks. It is also used in canned fruits wherein water and sucralose take the place of much higher-energy corn syrup-based additives. Sucralose mixed with dextrose (glucose) or maltodextrin (both made from corn) as bulking agents is sold internationally by McNeil Nutritionals under the Splenda brand name.

</details>

**Weird fact** · written · 5.4 s

> According to an anecdotal account, Phadnis thought Hough asked him to "taste" it, so he did and found the compound to be exceptionally sweet. Sucralose is a disaccharide derivative consisting of 4-chloro-4-deoxy-alpha-D-galactopyranose and 1,6-dichloro-1,6-dideoxy-beta-D-fructofuranose units linked by a glycosidic bond.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Sucralose#History): Sucralose was discovered in 1975 by scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College (now part of King's College London). While researching novel uses of sucrose and its synthetic derivatives, mixing sulfuryl chloride with sugar, Phadnis was told to "test" a chlorinated sugar compound. According to an anecdotal account, Phadnis thought Hough asked him to "taste" it, so he did and found the compound to be exceptionally sweet. Tate & Lyle patented the substance in 1976; as of 2008, the only remaining patents concerned specific manufacturing processes.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/71485#section=Record-Description): Sucralose is a disaccharide derivative consisting of 4-chloro-4-deoxy-alpha-D-galactopyranose and 1,6-dichloro-1,6-dideoxy-beta-D-fructofuranose units linked by a glycosidic bond. It has a role as a xenobiotic, a sweetening agent and an environmental contaminant. It is an organochlorine compound and a disaccharide derivative.

</details>


### sucrose (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Sucrose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · skipped · 0.7 s

**Where you meet it** · written · 5.3 s

> Sugar is often an added ingredient in food production and recipes. In nature, sucrose is present in many plants, and in particular their roots, fruits and nectars, because it serves as a way to store energy, primarily from photosynthesis. Sucrose forms a major element in confectionery and desserts.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Sucrose): Sugar is often an added ingredient in food production and recipes. About 185 million tonnes of sugar were produced worldwide in 2017.
- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Sucrose#Sources): In nature, sucrose is present in many plants, and in particular their roots, fruits and nectars, because it serves as a way to store energy, primarily from photosynthesis. Many mammals, birds, insects and bacteria accumulate and feed on the sucrose in plants and for some it is their main food source. Although honeybees consume sucrose, the honey they produce consists primarily of fructose and glucose, with only trace amounts of sucrose.
- [Wikipedia, Consumption](https://en.wikipedia.org/wiki/Sucrose#Consumption): Sucrose forms a major element in confectionery and desserts. Cooks use it for sweetening. It can also act as a food preservative when used in sufficient concentrations, and thus is an important ingredient in the production of fruit preserves. Sucrose is important to the structure of many foods, including biscuits and cookies, cakes and pies, candy, and ice cream and sorbets. It is a common ingredient in many processed and so-called "junk foods".

</details>

**Weird fact** · written · 6.0 s

> Unlike most disaccharides, the glycosidic bond in sucrose is formed between the reducing ends of both glucose and fructose, and not between the reducing end of one and the non-reducing end of the other.

<details><summary>Sources</summary>

- [Wikipedia, Physical and chemical properties](https://en.wikipedia.org/wiki/Sucrose#Physical_and_chemical_properties): Sucrose's IUPAC name is β-D-fructofuranosyl-(2→1)-α-D-glucopyranoside. In this disaccharide, glucose and fructose are linked via a glycosidic linkage, i.e. an ether bond between C1 on the glucosyl subunit and C2 on the fructosyl unit. Glucose exists predominantly as a mixture of α and β "pyranose" anomers, but sucrose has only the α form. Fructose exists as a mixture of five tautomers but sucrose has only the β-D-fructofuranose form. Unlike most disaccharides, the glycosidic bond in sucrose is formed between the reducing ends of both glucose and fructose, and not between the reducing end of one and the non-reducing end of the other.

</details>


### testosterone (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Testosterone is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · written · 1.5 s

> A testicular action was linked to circulating blood fractions – now understood to be a family of androgenic hormones – in the early work on castration and testicular transplantation in fowl by Arnold Adolph Berthold (1803–1861). Research on the action of testosterone received a brief boost in 1889, when the Harvard professor Charles-Édouard Brown-Séquard (1817–1894), then in Paris, self-injected subcutaneously a "rejuvenating elixir" consisting of an extract of dog and guinea pig testicle.

<details><summary>Sources</summary>

- [Wikipedia, History and production](https://en.wikipedia.org/wiki/Testosterone#History_and_production): A testicular action was linked to circulating blood fractions – now understood to be a family of androgenic hormones – in the early work on castration and testicular transplantation in fowl by Arnold Adolph Berthold (1803–1861). Research on the action of testosterone received a brief boost in 1889, when the Harvard professor Charles-Édouard Brown-Séquard (1817–1894), then in Paris, self-injected subcutaneously a "rejuvenating elixir" consisting of an extract of dog and guinea pig testicle. He reported in The Lancet that his vigor and feeling of well-being were markedly restored but the effects were transient, and Brown-Séquard's hopes for the compound were dashed.

</details>

**Where you meet it** · written · 4.0 s

> Testosterone is the primary male sex hormone and androgen in males. In humans, testosterone drives the development of male reproductive tissues such as testicles and prostate, as well as promoting secondary sexual characteristics such as increased muscle and bone mass, and the growth of body hair. In humans and most other vertebrates, testosterone is secreted primarily by the testicles of males and, to a lesser extent, the ovaries of females.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Testosterone): Testosterone is the primary male sex hormone and androgen in males. In humans, testosterone drives the development of male reproductive tissues such as testicles and prostate, as well as promoting secondary sexual characteristics such as increased muscle and bone mass, and the growth of body hair. It is associated with increased aggression, sex drive, dominance, courtship display, and a wide range of behavioral characteristics. In addition, testosterone in both sexes is involved in health and well-being, where it has a significant effect on overall mood, cognition, social and sexual behavior, metabolism and energy output, the cardiovascular system, and in the prevention of osteoporosis.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Testosterone): Testosterone is a steroid hormone from the androstane class containing a ketone and a hydroxyl group at positions three and seventeen respectively. It is biosynthesized in several steps from cholesterol and is converted in the liver to inactive metabolites. It exerts its action through binding to and activation of the androgen receptor. In humans and most other vertebrates, testosterone is secreted primarily by the testicles of males and, to a lesser extent, the ovaries of females. On average, in adult males, levels of testosterone are about seven to eight times as great as in adult females.

</details>

**Weird fact** · written · 5.6 s

> Agnathans (jawless vertebrates) such as lampreys do not produce testosterone but instead use androstenedione as a male sex hormone. Fish make a slightly different form called 11-ketotestosterone.

<details><summary>Sources</summary>

- [Wikipedia, Other species](https://en.wikipedia.org/wiki/Testosterone#Other_species): Testosterone is observed in most vertebrates. Testosterone and the classical nuclear androgen receptor first appeared in gnathostomes (jawed vertebrates). Agnathans (jawless vertebrates) such as lampreys do not produce testosterone but instead use androstenedione as a male sex hormone. Fish make a slightly different form called 11-ketotestosterone. Its counterpart in insects is ecdysone. The presence of these ubiquitous steroids in a wide range of animals suggest that sex hormones have an ancient evolutionary history.

</details>


### tetrodotoxin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Tetrodotoxin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 5.2 s

> Tetrodotoxin (TTX) is a potent neurotoxin. Its name derives from Tetraodontiformes, an order that includes pufferfish, porcupinefish, ocean sunfish, and triggerfish; several of these species carry the toxin. Although tetrodotoxin was discovered in these fish, it is found in several other animals (e.g., in blue-ringed octopuses, rough-skinned newts, and moon snails). Tetrodotoxin is an extremely potent poison (toxin) found mainly in the liver and sex organs (gonads) of some fish, such as puffer fish, globefish, and toadfish (order Tetraodontiformes) and in some amphibian, octopus, and shellfish species.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Tetrodotoxin): Tetrodotoxin (TTX) is a potent neurotoxin. Its name derives from Tetraodontiformes, an order that includes pufferfish, porcupinefish, ocean sunfish, and triggerfish; several of these species carry the toxin. Although tetrodotoxin was discovered in these fish, it is found in several other animals (e.g., in blue-ringed octopuses, rough-skinned newts, and moon snails). It is also produced by certain infectious or symbiotic bacteria like Pseudoalteromonas, Pseudomonas, and Vibrio as well as other species found in symbiotic relationships with animals and plants.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/11174599#section=Uses): Tetrodotoxin is an extremely potent poison (toxin) found mainly in the liver and sex organs (gonads) of some fish, such as puffer fish, globefish, and toadfish (order Tetraodontiformes) and in some amphibian, octopus, and shellfish species. Human poisonings occur when the flesh and/or organs of the fish are improperly prepared and eaten. Tetrodotoxin interferes with the transmission of signals from nerves to muscles and causes an increasing paralysis of the muscles of the body. Tetrodotoxin poisoning can be fatal.

</details>

**Weird fact** · written · 5.3 s

> Based on the presumption that tetrodotoxin is not always fatal, but at near-lethal doses can leave a person extremely unwell with the person remaining conscious, tetrodotoxin has been alleged to result in zombieism, and has been suggested as an ingredient in Haitian Vodou preparations. This idea first appeared in the 1938 non-fiction book Tell My Horse by Zora Neale Hurston in which there were multiple accounts of purported tetrodotoxin poisoning in Haiti by a voodoo sorcerer called the bokor.

<details><summary>Sources</summary>

- [Wikipedia, Popular culture](https://en.wikipedia.org/wiki/Tetrodotoxin#Popular_culture): Based on the presumption that tetrodotoxin is not always fatal, but at near-lethal doses can leave a person extremely unwell with the person remaining conscious, tetrodotoxin has been alleged to result in zombieism, and has been suggested as an ingredient in Haitian Vodou preparations. This idea first appeared in the 1938 non-fiction book Tell My Horse by Zora Neale Hurston in which there were multiple accounts of purported tetrodotoxin poisoning in Haiti by a voodoo sorcerer called the bokor. These stories were later popularized by Harvard-trained ethnobotanist Wade Davis in his 1985 book and Wes Craven's 1988 film, both titled The Serpent and the Rainbow.

</details>


### thalidomide (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Thalidomide carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · dropped · 4.7 s

Problems: no chosen sentence names Thalidomide

**Weird fact** · written · 3.2 s

> A piperidinyl isoindole originally introduced as a non-barbiturate hypnotic, thalidomide was withdrawn from the market due to teratogenic effects. It has been reintroduced and used for a number of inflammatory disorders and cancers.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5426#section=Record-Description): A piperidinyl isoindole originally introduced as a non-barbiturate hypnotic, thalidomide was withdrawn from the market due to teratogenic effects. It has been reintroduced and used for a number of inflammatory disorders and cancers. Thalidomide displays immunosuppressive and anti-angiogenic activity through modulating the release of inflammatory mediators like tumor necrosis factor-alpha (TNF-a) and other cytokine action. Due to severe teratogenicity, pregnancy must be excluded before the start of treatment and patients must enrol in the THALIDOMID Risk Evaluation and Mitigation Strategy (REMS) program to ensure contraception adherence.

</details>


### theobromine (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Theobromine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 2 more hazards, not listed here.

**Origin story** · written · 1.5 s

> Theobromine was discovered in 1841 in cacao beans by the chemist A. Woskresensky.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Theobromine#History): Theobromine was discovered in 1841 in cacao beans by the chemist A. Woskresensky. Synthesis of theobromine from xanthine was first reported in 1882 by Hermann Emil Fischer.

</details>

**Where you meet it** · written · 4.1 s

> In industry, theobromine is used as an additive and precursor to some cosmetics. It is found in chocolate and several other foods, including tea (Camellia sinensis), some American hollies (yaupon and guayusa) and the kola nut. Theobromine is the primary alkaloid found in cocoa and chocolate. There are approximately 60 milligrams (1 grain) of theobromine in 28 grams (1 oz) of milk chocolate, while the same amount of dark chocolate contains about 200 milligrams (3 grains).

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Theobromine): Theobromine, also known as xantheose, is the principal alkaloid of Theobroma cacao (cacao plant). Theobromine is slightly water-soluble (330 mg/L) with a bitter taste. In industry, theobromine is used as an additive and precursor to some cosmetics. It is found in chocolate and several other foods, including tea (Camellia sinensis), some American hollies (yaupon and guayusa) and the kola nut. It is a white or colourless solid, but commercial samples can appear yellowish.
- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Theobromine#Sources): Theobromine is the primary alkaloid found in cocoa and chocolate. Cocoa butter only contains trace amounts of theobromine. There are usually higher concentrations in dark than in milk chocolate.
- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Theobromine#Sources): There are approximately 60 milligrams (1 grain) of theobromine in 28 grams (1 oz) of milk chocolate, while the same amount of dark chocolate contains about 200 milligrams (3 grains). Cocoa beans naturally contain approximately 1% theobromine.

</details>

**Weird fact** · dropped · 13.4 s

Problems: no chosen sentence names Theobromine


### trinitrotoluene (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> 2,4,6-Trinitrotoluene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · dropped · 3.7 s

Problems: no chosen sentence names 2,4,6-Trinitrotoluene

**Where you meet it** · written · 4.2 s

> 2,4,6-Trinitrotoluene is a yellow, odorless solid that does not occur naturally in the environment. 2,4,6-Trinitrotoluene is an explosive used in military shells, bombs, and grenades, in industrial uses, and in underwater blasting.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/8376#section=Record-Description): 2,4,6-Trinitrotoluene is a yellow, odorless solid that does not occur naturally in the environment. It is commonly known as TNT and is an explosive used in military shells, bombs, and grenades, in industrial uses, and in underwater blasting. 2,4,6-Trinitrotoluene production in the United States occurs solely at military arsenals.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/8376#section=Uses): 2,4,6-Trinitrotoluene is an explosive used in military shells, bombs, and grenades, in industrial uses, and in underwater blasting. Exposure may results from drinking contaminated water that has migrated from chemical waste disposal sites, breathing contaminated air, eating contaminated foods such as fruits and vegetables, and/or eating contaminated soil.

</details>

**Weird fact** · dropped · 11.8 s

Problems: no chosen sentence names 2,4,6-Trinitrotoluene


### vanillin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Vanillin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · dropped · 3.4 s

Problems: no chosen sentence names Vanillin

**Where you meet it** · written · 4.7 s

> Vanillin is most prominent as the principal flavor and aroma compound in vanilla. Vanillin occurs in a wide variety of foods and plants such as orchids; major commercial source of natural vanillin is from vanilla bean extract.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Vanillin#Occurrence): Vanillin is most prominent as the principal flavor and aroma compound in vanilla. Cured vanilla pods contain about 2% by dry weight vanillin. Relatively pure vanillin may be visible as a white dust or "frost" on the exteriors of cured pods of high quality.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/1183#section=Natural-Pollution-Sources): Vanillin has been identified as a constituent in various tissues of several plant species. Vanillin occurs in a wide variety of foods and plants such as orchids; major commercial source of natural vanillin is from vanilla bean extract. Vanillin has been reported in the essential oil of Java citronella (Cymbopogon nardus Rendle), in benzoin, Peru balsam, clove bud oil and chiefly vanilla pods (Vanilla planifolia, Vanilla tahitensis). [

</details>

**Weird fact** · written · 5.5 s

> Scolytus multistriatus, one of the vectors of the Dutch elm disease, uses vanillin as a signal to find a host tree during oviposition.

<details><summary>Sources</summary>

- [Wikipedia, Ecology](https://en.wikipedia.org/wiki/Vanillin#Ecology): Scolytus multistriatus, one of the vectors of the Dutch elm disease, uses vanillin as a signal to find a host tree during oviposition.

</details>


### water (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Water is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · no input · 0.0 s

**Where you meet it** · written · 4.3 s

> Water (chemical formula: H2O) is a transparent fluid which forms the world's streams, lakes, oceans and rain, and is the major constituent of the fluids of organisms. Water is an inorganic compound with the chemical formula H2O. It is a transparent, tasteless, odorless, and nearly colorless chemical substance. It is the main constituent of Earth's streams, lakes, and oceans.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/962#section=Record-Description): Water (chemical formula: H2O) is a transparent fluid which forms the world's streams, lakes, oceans and rain, and is the major constituent of the fluids of organisms. As a chemical compound, a water molecule contains one oxygen and two hydrogen atoms that are connected by covalent bonds. Water is a liquid at standard ambient temperature and pressure, but it often co-exists on Earth with its solid state, ice; and gaseous state, steam (water vapor).
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Water): Water is an inorganic compound with the chemical formula H2O. It is a transparent, tasteless, odorless, and nearly colorless chemical substance. It is the main constituent of Earth's streams, lakes, and oceans. Water is also the fluid of all known living organisms, in which it acts as a solvent. Water, being a polar molecule, undergoes strong intermolecular hydrogen bonding which is a large contributor to its physical and chemical properties. It is vital for all known forms of life, despite not providing food energy or being an organic micronutrient.

</details>

**Weird fact** · written · 5.1 s

> Water is the only common substance to exist as a solid, liquid, and gas in normal terrestrial conditions.

<details><summary>Sources</summary>

- [Wikipedia, Properties](https://en.wikipedia.org/wiki/Water#Properties): Water (H2O) is a polar inorganic compound. At room temperature it is a tasteless and odorless liquid, nearly colorless with a hint of blue. The simplest hydrogen chalcogenide, it is by far the most studied chemical compound and is sometimes described as the "universal solvent" for its ability to dissolve more substances than any other liquid, though it is poor at dissolving nonpolar substances. This allows it to be the "solvent of life": indeed, water as found in nature almost always includes various dissolved substances, and special steps are required to obtain chemically pure water. Water is the only common substance to exist as a solid, liquid, and gas in normal terrestrial conditions.

</details>

