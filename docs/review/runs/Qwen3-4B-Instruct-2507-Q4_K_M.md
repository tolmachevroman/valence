# Review run: Qwen3-4B-Instruct-2507-Q4_K_M

50 molecules from `fixtures/review`, cards written as the app writes them. Times start when a molecule's snippets are in hand; fetching adds 1–3 s uncached. The Danger card is built from data and appears at once, so first-card times count model cards.

## Against the PRD targets

| Measure | Target | Result |
| --- | --- | --- |
| First model card | under 10 s | 44 of 50 molecules; 3.1 s median, 10.2 s p90, 14.1 s max |
| Streaming starts | under 3 s | 38 of 50; 2.1 s median, 5.2 s p90, 6.9 s max |
| Molecules with 3+ cards | 90% | 47 of 50 (94%) |
| Cards with a claim not in the sources | 0 | by review, below |
| Model load | under 10 s | 6.9 s |
| Model card, including retry | | 5.9 s median, 13.3 s p90, 15.4 s max |

## Outcomes

| Card | written | untitled | retried | skipped | dropped | no input | error |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Where you meet it | 29 | 10 | 8 | 0 | 3 | 0 | 0 |
| Origin story | 28 | 2 | 1 | 4 | 2 | 0 | 0 |
| Weird fact | 23 | 20 | 4 | 1 | 2 | 0 | 0 |

## Cards

Each claim should be stated by the cited snippets, folded under the card.

### DDT (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> 1-Chloro-4-(2,2,2-trichloro-1-(4-chlorophenyl)ethyl)benzene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · retried · 6.8 s

> **Used in agriculture and tanning**  
> Clofenotane was once widely used as a pesticide to control insects in agriculture and to control malaria, typhus and body lice, but its use is now limited. Farming (Pesticides) [Category: Industry] Leather Tanning and Processing [Category: Industry]

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/3036#section=Record-Description): Clofenotane is a white, odorless and tasteless, very stable, water-insoluble, synthetic, crystalline chlorinated hydrocarbon that is soluble in most organic solvents, fat and oils and tends to accumulate in ecosystems. Clofenotane was once widely used as a pesticide to control insects in agriculture and to control malaria, typhus and body lice, but its use is now limited. Exposure to clofenotane mainly affects the nervous system resulting in tremors, dizziness, nausea, incoordination, convulsions and seizures. This substance is reasonably anticipated to be a human carcinogen based on evidence of carcinogenicity in experimental animals.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/3036#section=Uses): Farming (Pesticides) [Category: Industry] Leather Tanning and Processing [Category: Industry]

</details>

**Weird fact** · skipped · 3.7 s


### acetaminophen (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Acetaminophen carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 3 more hazards, not listed here.

**Origin story** · written · 3.8 s

> **First found with analgesic properties**  
> Acetanilide was the first aniline derivative serendipitously found to possess analgesic as well as antipyretic properties, and was quickly introduced into medical practice under the name of Antifebrin by Cahn & Hepp in 1886. Some reports state that Cahn & Hepp or a French chemist called Charles Gerhardt first synthesized paracetamol in 1852.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Paracetamol#History): Acetanilide was the first aniline derivative serendipitously found to possess analgesic as well as antipyretic properties, and was quickly introduced into medical practice under the name of Antifebrin by Cahn & Hepp in 1886. But its unacceptable toxic effects—the most alarming being cyanosis due to methemoglobinemia, an increase of hemoglobin in its ferric [Fe3+] state, called methemoglobin, which cannot bind oxygen, and thus decreases overall carriage of oxygen to tissue—prompted the search for less toxic aniline derivatives. Some reports state that Cahn & Hepp or a French chemist called Charles Gerhardt first synthesized paracetamol in 1852.

</details>

**Weird fact** · untitled · 11.5 s

> ****  
> However, the abnormal liver function tests (meaning there was some inflammation or damage to the liver) were almost four times more likely in those on paracetamol, although the clinical importance of this effect is uncertain.

<details><summary>Sources</summary>

- [Wikipedia, Adverse effects](https://en.wikipedia.org/wiki/Paracetamol#Adverse_effects): In clinical trials for osteoarthritis, the number of participants reporting adverse effects was similar for those on paracetamol and on placebo. However, the abnormal liver function tests (meaning there was some inflammation or damage to the liver) were almost four times more likely in those on paracetamol, although the clinical importance of this effect is uncertain. After 13 weeks of paracetamol therapy for knee pain, a drop in hemoglobin level indicating gastrointestinal bleeding was observed in 20% of participants, this rate being similar to the ibuprofen group.

</details>

**Where you meet it** · retried · 13.5 s

> **Found in cold and pain medications**  
> Acetaminophen is often found combined with other drugs in more than 600 over the counter (OTC) allergy medications, cold medications, sleep medications, pain relievers, and other products. Paracetamol, or acetaminophen, is an analgesic and antipyretic agent used to treat fever and mild to moderate pain. It is a widely available over-the-counter drug sold generically or under various brand names, including Tylenol and Panadol.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/1983#section=Record-Description): Acetaminophen (paracetamol), also commonly known as Tylenol, is the most commonly taken analgesic worldwide and is recommended as first-line therapy in pain conditions by the World Health Organization (WHO). It is also used for its antipyretic effects, helping to reduce fever. This drug was initially approved by the U.S. FDA in 1951 and is available in a variety of forms including syrup form, regular tablets, effervescent tablets, injection, suppository, and other forms. Acetaminophen is often found combined with other drugs in more than 600 over the counter (OTC) allergy medications, cold medications, sleep medications, pain relievers, and other products.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Paracetamol): Paracetamol, or acetaminophen, is an analgesic and antipyretic agent used to treat fever and mild to moderate pain. It is a widely available over-the-counter drug sold generically or under various brand names, including Tylenol and Panadol.

</details>


### allicin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Allicin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · dropped · 4.2 s

Problems: s1.1 starts with "It" but the sentence before it was not chosen

**Where you meet it** · written · 2.7 s

> **Found in garlic and leeks**  
> Allicin is an organosulfur compound obtained from garlic and leeks. When fresh garlic is chopped or crushed, the enzyme alliinase converts alliin into allicin, which is responsible for the aroma of fresh garlic.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Allicin): Allicin is an organosulfur compound obtained from garlic and leeks. When fresh garlic is chopped or crushed, the enzyme alliinase converts alliin into allicin, which is responsible for the aroma of fresh garlic. Allicin is unstable and quickly changes into a series of other sulfur-containing compounds such as diallyl disulfide. Allicin is an antifeedant, i.e. the defense mechanism against attacks by pests on the garlic plant.

</details>

**Weird fact** · untitled · 6.9 s

> ****  
> The compound is not present in garlic unless tissue damage occurs, and is formed by the action of the enzyme alliinase on alliin. When fresh garlic is chopped or crushed, the enzyme alliinase converts alliin into allicin, which is responsible for the aroma of fresh garlic.

<details><summary>Sources</summary>

- [Wikipedia, Structure and occurrence](https://en.wikipedia.org/wiki/Allicin#Structure_and_occurrence): Allicin features the thiosulfinate functional group, R-S(O)-S-R. The compound is not present in garlic unless tissue damage occurs, and is formed by the action of the enzyme alliinase on alliin. Allicin is chiral but occurs naturally only as a racemate. The racemic form can also be generated by oxidation of diallyl disulfide:
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Allicin): Allicin is an organosulfur compound obtained from garlic and leeks. When fresh garlic is chopped or crushed, the enzyme alliinase converts alliin into allicin, which is responsible for the aroma of fresh garlic. Allicin is unstable and quickly changes into a series of other sulfur-containing compounds such as diallyl disulfide. Allicin is an antifeedant, i.e. the defense mechanism against attacks by pests on the garlic plant.

</details>


### artemisinin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Artemisinin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · untitled · 5.0 s

> ****  
> Results were published in the Chinese Medical Journal in 1979. The extracted substance, once subject to purification, proved to be a useful starting point to obtain purified artemisinin.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Artemisinin#History): Tu's team subsequently isolated an extract. Results were published in the Chinese Medical Journal in 1979. The extracted substance, once subject to purification, proved to be a useful starting point to obtain purified artemisinin. A 2012 review reported that artemisinin-based therapies were the most effective drugs for treatment of malaria at that time; it was also reported to clear malaria parasites from patients' bodies faster than other drugs. In addition to artemisinin, Project 523 developed a number of products that can be used in combination with artemisinin, including lumefantrine, piperaquine, and pyronaridine.

</details>

**Where you meet it** · written · 4.2 s

> **Found in sweet wormwood**  
> (+)-artemisinin is a sesquiterpene lactone obtained from sweet wormwood, Artemisia annua, which is used as an antimalarial for the treatment of multi-drug resistant strains of falciparum malaria. artemisinin has been reported in Artemisia lancea, Artemisia annua, and other organisms with data available.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/68827#section=Record-Description): (+)-artemisinin is a sesquiterpene lactone obtained from sweet wormwood, Artemisia annua, which is used as an antimalarial for the treatment of multi-drug resistant strains of falciparum malaria. It has a role as an antimalarial and a plant metabolite. It is an organic peroxide and a sesquiterpene lactone.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/68827#section=Record-Description): artemisinin has been reported in Artemisia lancea, Artemisia annua, and other organisms with data available.

</details>

**Weird fact** · written · 6.5 s

> **Endoperoxide ring breaks with iron**  
> An unusual component of the artemisinin molecules is an endoperoxide 1,2,4-trioxane ring. As the drug molecules come in contact with the haem (associated with the hemoglobin of the red blood cells), the iron(II) oxide breaks the endoperoxide ring. This process produces free radicals that in turn damage susceptible proteins, resulting in the death of the parasite.

<details><summary>Sources</summary>

- [Wikipedia, Chemistry](https://en.wikipedia.org/wiki/Artemisinin#Chemistry): An unusual component of the artemisinin molecules is an endoperoxide 1,2,4-trioxane ring. This is the main antimalarial centre of the molecule. Modifications at carbon 10 (C10) position give rise to a variety of derivatives which are more powerful than the original compound. Because the physical properties of artemisinin itself, such as poor bioavailability, limit its effectiveness, semisynthetic derivatives of artemisinin have been developed. Derivatives of dihydroartemisinin were made since 1976. Artesunate, arteether, and artemether were first synthesized in 1986.
- [Wikipedia, Mechanism of action](https://en.wikipedia.org/wiki/Artemisinin#Mechanism_of_action): As of 2018, the exact mechanism of action of artemisinins has not been fully elucidated. Artemisinin itself is a prodrug of the biologically active dihydroartemisinin. This metabolite undergoes cleavage of its endoperoxide ring inside the erythrocytes. As the drug molecules come in contact with the haem (associated with the hemoglobin of the red blood cells), the iron(II) oxide breaks the endoperoxide ring. This process produces free radicals that in turn damage susceptible proteins, resulting in the death of the parasite. In 2016 artemisinin was shown to bind to a large number of targets suggesting that it acts in a promiscuous manner.

</details>


### ascorbic acid (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> L-Ascorbic Acid is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · written · 2.6 s

> **Discovered in 1912, synthesized in 1928**  
> Vitamin C was discovered in 1912, isolated in 1928 and synthesized in 1933, making it the first vitamin to be synthesized. Shortly thereafter Tadeus Reichstein succeeded in synthesizing the vitamin in bulk by what is now called the Reichstein process. One of these was thought to be the anti-scorbutic factor.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Vitamin_C#History): Vitamin C was discovered in 1912, isolated in 1928 and synthesized in 1933, making it the first vitamin to be synthesized. Shortly thereafter Tadeus Reichstein succeeded in synthesizing the vitamin in bulk by what is now called the Reichstein process. This made possible the inexpensive mass-production of vitamin C. In 1934, Hoffmann–La Roche bought the Reichstein process patent, trademarked synthetic vitamin C under the brand name Redoxon, and began to market it as a dietary supplement.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Vitamin_C#History): In 1907, a laboratory animal model which would help to identify the antiscorbutic factor was serendipitously discovered by the Norwegian physicians Axel Holst and Theodor Frølich, who when studying shipboard beriberi, fed guinea pigs their test diet of grains and flour and were surprised when scurvy resulted instead of beriberi. Unknown at that time, this species did not make its own vitamin C (being a caviomorph), whereas mice and rats do. In 1912, the Polish biochemist Casimir Funk developed the concept of vitamins. One of these was thought to be the anti-scorbutic factor. In 1928, this was referred to as "water-soluble C", although its chemical structure had not been determined.

</details>

**Weird fact** · written · 5.8 s

> **Inuit avoided scurvy on raw meat diet**  
> In 1928, the Canadian Arctic anthropologist Vilhjalmur Stefansson showed that the Inuit avoided scurvy on a diet largely of raw meat.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Vitamin_C#History): The name antiscorbutic was used in the eighteenth and nineteenth centuries for foods known to prevent scurvy. These foods included lemons, limes, oranges, sauerkraut, cabbage, malt, and portable soup. In 1928, the Canadian Arctic anthropologist Vilhjalmur Stefansson showed that the Inuit avoided scurvy on a diet largely of raw meat.

</details>

**Where you meet it** · written · 7.0 s

> **Found in citrus and vegetables**  
> A six carbon compound related to glucose. It is found naturally in citrus fruits and many vegetables. Food sources of L-ascorbic acid include acerola (West Indian cherry); citrus fruits, tomatoes; potatoes, and green leafy vegetables(2).

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/54670067#section=Record-Description): A six carbon compound related to glucose. It is found naturally in citrus fruits and many vegetables. Ascorbic acid is an essential nutrient in human diets, and necessary to maintain connective tissue and bone. Its biologically active form, vitamin C, functions as a reducing agent and coenzyme in several metabolic pathways. Vitamin C is considered an antioxidant.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/54670067#section=Natural-Pollution-Sources): L-Ascorbic acid is widely distributed in the plant and animal kingdom(1). Food sources of L-ascorbic acid include acerola (West Indian cherry); citrus fruits, tomatoes; potatoes, and green leafy vegetables(2). It has been isolated from the adrenal cortex of ox and later from lemons and paprika(1).

</details>


### aspirin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Aspirin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 3 more hazards, not listed here.

**Origin story** · dropped · 5.2 s

Problems: the chosen sentences have 96 words; keep it under 90

**Where you meet it** · written · 3.7 s

> **Used in pharmaceutical products**  
> CHEMICAL PROFILE: Virtually all aspirin is used in pharmaceutical products in aspirin tablets or in concert with other ingredients (1981)

<details><summary>Sources</summary>

- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/2244#section=Uses): CHEMICAL PROFILE: Virtually all aspirin is used in pharmaceutical products in aspirin tablets or in concert with other ingredients (1981)

</details>

**Weird fact** · retried · 12.8 s

> **Bayer named it Aspirin in 1897**  
> In 1897, scientists at the drug and dye firm Bayer began investigating acetylsalicylic acid as a less-irritating replacement for standard common salicylate medicines, and identified a new way to synthesize it. That year, Felix Hoffmann (or Arthur Eichengrün) of Bayer was the first to produce acetylsalicylic acid in a pure, stable form. By 1899, Bayer had named it "Aspirin" and was selling it around the world.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Aspirin#History): In 1853, the chemist Charles Frédéric Gerhardt treated sodium salicylate with acetyl chloride to produce acetylsalicylic acid for the first time; in the second half of the 19th century, other academic chemists established the compound's chemical structure and devised more efficient methods of synthesis. In 1897, scientists at the drug and dye firm Bayer began investigating acetylsalicylic acid as a less-irritating replacement for standard common salicylate medicines, and identified a new way to synthesize it. That year, Felix Hoffmann (or Arthur Eichengrün) of Bayer was the first to produce acetylsalicylic acid in a pure, stable form.
- [Wikipedia, Brand vs. generic name](https://en.wikipedia.org/wiki/Aspirin#Brand_vs._generic_name): In 1897, scientists at the Bayer company began studying acetylsalicylic acid as a less-irritating replacement medication for common salicylate medicines. It developed heroin in 1898 as a cough suppressant, and the less potent aspirin the following year. By 1899, Bayer had named it "Aspirin" and was selling it around the world.

</details>


### batrachotoxin (3 cards)

**Origin story** · written · 2.0 s

> **Discovered in 1963**  
> Batrachotoxin was discovered by Fritz Märki and Bernhard Witkop, at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. Märki and Witkop isolated the toxic alkaloids fraction from the black-legged poison dart frog and determined its chemical properties in 1963.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Batrachotoxin#History): Batrachotoxin was discovered by Fritz Märki and Bernhard Witkop, at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. Märki and Witkop isolated the toxic alkaloids fraction from the black-legged poison dart frog and determined its chemical properties in 1963. They isolated four major toxic steroidal alkaloids including batrachotoxin, homobatrachotoxin (isobatrachotoxin), pseudobatrachotoxin, and batrachotoxinin A. Due to the difficulty of handling such a potent toxin and the minuscule amount that could be collected, a comprehensive structure determination involved several difficulties.

</details>

**Where you meet it** · written · 5.7 s

> **Found in frogs, beetles, and birds**  
> Batrachotoxins (BTX) are extremely potent cardiotoxic and neurotoxic steroidal alkaloids found in poison dart frogs (genus Dendrobates and Phyllobates), Melyridae beetles, and certain birds (genus Pitohui and Ifrita). Batrachotoxins are found in poison dart frogs (genus Dendrobates and Phyllobates), Melyridae beetles, and certain birds (genus Pitohui and Ifrita). (L1069)

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/6324647#section=Record-Description): Batrachotoxins (BTX) are extremely potent cardiotoxic and neurotoxic steroidal alkaloids found in poison dart frogs (genus Dendrobates and Phyllobates), Melyridae beetles, and certain birds (genus Pitohui and Ifrita). They act directly on sodium channels. (L1069). It is the most potent non-peptidal neurotoxin known.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/6324647#section=Uses): Batrachotoxins are found in poison dart frogs (genus Dendrobates and Phyllobates), Melyridae beetles, and certain birds (genus Pitohui and Ifrita). (L1069)

</details>

**Weird fact** · written · 6.2 s

> **Irreversibly opens sodium channels**  
> Batrachotoxin binds to and irreversibly opens the sodium channels of nerve cells and prevents them from closing, resulting in paralysis and death. Batrachotoxin (BTX) is a steroidal alkaloid that binds to voltage-gated sodium channels and makes them unable to produce action potentials.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Batrachotoxin): Batrachotoxin (BTX) is an extremely potent cardiotoxic and neurotoxic steroidal alkaloid found in certain species of beetles, birds, and frogs. The name is from the Greek word βάτραχος, bátrachos, 'frog'. Structurally-related chemical compounds are often referred to collectively as batrachotoxins. In certain frogs, this alkaloid is present mostly on the skin. Such frogs are among those used for poisoning darts. Batrachotoxin binds to and irreversibly opens the sodium channels of nerve cells and prevents them from closing, resulting in paralysis and death. No antidote is known.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/6324647#section=Record-Description): Amphibians have the ability to produce and secrete a mixture of bioactive substances in specialized skin glands for antibiotic self-protection and defense against predators. Some of these secretions contain the highly toxic batrachotoxin, tetrodotoxin, and samandarine. Batrachotoxin (BTX) is a steroidal alkaloid that binds to voltage-gated sodium channels and makes them unable to produce action potentials. It is found in frogs of the genus Phyllobates, birds in the genera Pitohui and Ifrita, and beetles in the genus Choresine. Batrachotoxin was discovered by the precolonial inhabitants of the Northwestern Andes (present day Colombia).

</details>


### benzene (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Benzene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · retried · 5.9 s

> **Discovered from oily residue**  
> Michael Faraday first isolated and identified benzene in 1825 from the oily residue derived from the production of illuminating gas, giving it the name bicarburet of hydrogen. At Benzolfest, Kekulé discussed the theory's creation, having discovered the ring shape of the benzene molecule after having a reverie (or day-dream) of a snake biting its own tail (a symbol in ancient cultures known as the ouroboros).

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Benzene#History): The word "benzene" derives from "gum benzoin" (benzoin resin), an aromatic resin known since ancient times in Southeast Asia, and later to European pharmacists and perfumers in the 16th century via trade routes. An acidic material was derived from benzoin by sublimation, and named "flowers of benzoin", or benzoic acid. The hydrocarbon derived from benzoic acid thus acquired the names benzin, benzol, and benzene. Michael Faraday first isolated and identified benzene in 1825 from the oily residue derived from the production of illuminating gas, giving it the name bicarburet of hydrogen.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Benzene#History): The new understanding of benzene, and hence of all aromatic compounds, proved to be notable enough for both pure and applied chemistry that, in 1890, the German Chemical Society organized a celebration in Kekulé's honor (named as the "Benzolfest"), celebrating the twenty-fifth anniversary of his first paper on benzene. At Benzolfest, Kekulé discussed the theory's creation, having discovered the ring shape of the benzene molecule after having a reverie (or day-dream) of a snake biting its own tail (a symbol in ancient cultures known as the ouroboros).

</details>

**Where you meet it** · written · 6.9 s

> **Found in gasoline and smoke**  
> Benzene is also a natural part of crude oil, gasoline, and cigarette smoke. Benzene is found in the air from emissions from burning coal and oil, gasoline service stations, and motor vehicle exhaust.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/241#section=Record-Description): Benzene is a colorless liquid with a sweet odor. It evaporates into the air very quickly and dissolves slightly in water. It is highly flammable and is formed from both natural processes and human activities. Benzene is widely used in the United States; it ranks in the top 20 chemicals for production volume. Some industries use benzene to make other chemicals which are used to make plastics, resins, and nylon and synthetic fibers. Benzene is also used to make some types of rubbers, lubricants, dyes, detergents, drugs, and pesticides. Natural sources of benzene include volcanoes and forest fires. Benzene is also a natural part of crude oil, gasoline, and cigarette smoke.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/241#section=Record-Description): Benzene is found in the air from emissions from burning coal and oil, gasoline service stations, and motor vehicle exhaust. Acute (short-term) inhalation exposure of humans to benzene may cause drowsiness, dizziness, headaches, as well as eye, skin, and respiratory tract irritation, and, at high levels, unconsciousness. Chronic (long-term) inhalation exposure has caused various disorders in the blood, including reduced numbers of red blood cells and aplastic anemia, in occupational settings. Reproductive effects have been reported for women exposed by inhalation to high levels, and adverse effects on the developing fetus have been observed in animal tests.

</details>

**Weird fact** · written · 6.5 s

> **Detected in deep space**  
> In 1997, benzene was detected in deep space.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Benzene#History): In 1845, Charles Blachford Mansfield, working under August Wilhelm von Hofmann, isolated benzene from coal tar. Four years later, Mansfield began the first industrial-scale production of benzene, based on the coal-tar method. Gradually, the sense developed among chemists that a number of substances were chemically related to benzene, comprising a diverse chemical family. In 1855, Hofmann was the first to apply the word "aromatic" to designate this family relationship, after a characteristic property of many of its members. In 1997, benzene was detected in deep space.

</details>


### benzylpenicillin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Penicillin G carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 2 more hazards, not listed here.

**Where you meet it** · written · 4.2 s

> **Treats bacterial infections**  
> Penicillin G (potassium or sodium) is an antibacterial prescription medicine approved by the U.S. Food and Drug Administration (FDA) for the treatment of certain serious bacterialinfections, such as pneumonia, meningitis, gonorrhea, syphilis, and others. Benzylpenicillin, also known as penicillin G (PenG) or BENPEN, is an antibiotic used to treat a number of bacterial infections. This includes pneumonia, strep throat, syphilis, necrotizing enterocolitis, diphtheria, gas gangrene, leptospirosis, cellulitis, and tetanus.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5904#section=Record-Description): Penicillin G (potassium or sodium) is an antibacterial prescription medicine approved by the U.S. Food and Drug Administration (FDA) for the treatment of certain serious bacterialinfections, such as pneumonia, meningitis, gonorrhea, syphilis, and others. Pneumonia and syphilis can be opportunistic infections (OI) of HIV.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Benzylpenicillin): Benzylpenicillin, also known as penicillin G (PenG) or BENPEN, is an antibiotic used to treat a number of bacterial infections. This includes pneumonia, strep throat, syphilis, necrotizing enterocolitis, diphtheria, gas gangrene, leptospirosis, cellulitis, and tetanus. It is not a first-line agent for pneumococcal meningitis. Due to benzylpenicillin's limited bioavailability for oral medications, it is generally taken as an injection in the form of a sodium, potassium, benzathine, or procaine salt. Benzylpenicillin is given by injection into a vein or muscle. Two long-acting forms benzathine benzylpenicillin and procaine benzylpenicillin are available for use by injection into a muscle only.

</details>

**Weird fact** · untitled · 10.6 s

> ****  
> When used to treat syphilis or Lyme disease a reaction known as Jarisch–Herxheimer may occur.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Benzylpenicillin): Side effects include diarrhea, seizures, and allergic reactions including anaphylaxis. When used to treat syphilis or Lyme disease a reaction known as Jarisch–Herxheimer may occur. It is not recommended in those with a history of penicillin allergy. Use during pregnancy is generally safe in the penicillin and β-lactam class of medications.

</details>


### buckminsterfullerene (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Fullerenes carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · written · 3.0 s

> **First generated in 1984**  
> Theoretical predictions of buckminsterfullerene molecules appeared in the late 1960s and early 1970s. It was first generated in 1984 by Eric Rohlfing, Donald Cox, and Andrew Kaldor, using a laser to vaporize carbon in a supersonic helium beam, although the group did not realize that buckminsterfullerene had been produced. Using laser evaporation of graphite the Smalley team found Cn clusters (where n > 20 and even) of which the most common were C60 and C70.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Buckminsterfullerene#History): Theoretical predictions of buckminsterfullerene molecules appeared in the late 1960s and early 1970s. It was first generated in 1984 by Eric Rohlfing, Donald Cox, and Andrew Kaldor, using a laser to vaporize carbon in a supersonic helium beam, although the group did not realize that buckminsterfullerene had been produced. In 1985 their work was repeated by Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University, who recognized the structure of C60 as buckminsterfullerene.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Buckminsterfullerene#History): Using laser evaporation of graphite the Smalley team found Cn clusters (where n > 20 and even) of which the most common were C60 and C70. A solid rotating graphite disk was used as the surface from which carbon was vaporized using a laser beam creating hot plasma that was then passed through a stream of high-density helium gas. The carbon species were subsequently cooled and ionized resulting in the formation of clusters. Clusters ranged in molecular masses, but Kroto and Smalley found predominance in a C60 cluster that could be enhanced further by allowing the plasma to react longer. They also discovered that C60 is a cage-like molecule, a regular truncated icosahedron.

</details>

**Where you meet it** · untitled · 9.9 s

> ****  
> Buckminsterfullerene is the most common naturally occurring fullerene. Fullerene was first discovered in shungite, a green-schist facies metamorphic rock containing 99% carbon(1).

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Buckminsterfullerene#Occurrence): Buckminsterfullerene is the most common naturally occurring fullerene. Small quantities of it can be found in soot.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/123591#section=Natural-Pollution-Sources): Fullerene was first discovered in shungite, a green-schist facies metamorphic rock containing 99% carbon(1).

</details>

**Weird fact** · untitled · 13.6 s

> ****  
> Neutral C60 has been observed in planetary nebulae and several types of star. The ionised form, C+60, has been identified in the interstellar medium, where it is the cause of several absorption features known as diffuse interstellar bands in the near-infrared.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Buckminsterfullerene#Occurrence): It also exists in space. Neutral C60 has been observed in planetary nebulae and several types of star. The ionised form, C+60, has been identified in the interstellar medium, where it is the cause of several absorption features known as diffuse interstellar bands in the near-infrared.

</details>


### caffeine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Caffeine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 3.2 s

> **Isolated from coffee in 1819**  
> In 1819, the German chemist Friedlieb Ferdinand Runge isolated caffeine for the first time; he called it "Kaffebase" (i.e., a base that exists in coffee). Crystallizable substance discovered in coffee in 1821 by Mr. Robiquet. Pelletier and Caventou obtained caffeine; but because their research had a different goal and because their research had not been finished, they left priority on this subject to Mr. Robiquet.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Caffeine#History): In 1819, the German chemist Friedlieb Ferdinand Runge isolated caffeine for the first time; he called it "Kaffebase" (i.e., a base that exists in coffee). In 1821, caffeine was isolated both by the French chemist Pierre Jean Robiquet and by another pair of French chemists, Pierre-Joseph Pelletier and Joseph Bienaimé Caventou, according to Swedish chemist Jöns Jacob Berzelius in his yearly journal. Furthermore, Berzelius stated that the French chemists had made their discoveries independently of any knowledge of Runge's or each other's work.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Caffeine#History): Caffeine, noun (feminine). Crystallizable substance discovered in coffee in 1821 by Mr. Robiquet. During the same period – while they were searching for quinine in coffee because coffee is considered by several doctors to be a medicine that reduces fevers and because coffee belongs to the same family as the cinchona [quinine] tree – on their part, Messrs. Pelletier and Caventou obtained caffeine; but because their research had a different goal and because their research had not been finished, they left priority on this subject to Mr. Robiquet. We do not know why Mr. Robiquet has not published the analysis of coffee which he read to the Pharmacy Society.

</details>

**Weird fact** · written · 6.3 s

> **French chemists discovered it independently**  
> Furthermore, Berzelius stated that the French chemists had made their discoveries independently of any knowledge of Runge's or each other's work.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Caffeine#History): In 1819, the German chemist Friedlieb Ferdinand Runge isolated caffeine for the first time; he called it "Kaffebase" (i.e., a base that exists in coffee). In 1821, caffeine was isolated both by the French chemist Pierre Jean Robiquet and by another pair of French chemists, Pierre-Joseph Pelletier and Joseph Bienaimé Caventou, according to Swedish chemist Jöns Jacob Berzelius in his yearly journal. Furthermore, Berzelius stated that the French chemists had made their discoveries independently of any knowledge of Runge's or each other's work.

</details>

**Where you meet it** · written · 6.5 s

> **Found in tea, coffee, and cocoa**  
> A purine alkaloid that occurs naturally in tea and coffee. Caffeine is a major component of coffee, tea and chocolate and in humans acts as a central nervous system (CNS) stimulant. 0.9 to 2.6% in green coffee beans). It is a component in coffee, tea and cocoa.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/2519#section=Record-Description): Caffeine is a trimethylxanthine in which the three methyl groups are located at positions 1, 3, and 7. A purine alkaloid that occurs naturally in tea and coffee. It has a role as a geroprotector, a mutagen, a central nervous system stimulant, a psychotropic drug, a diuretic, a xenobiotic, an EC 3.1.4.* (phosphoric diester hydrolase) inhibitor, an EC 2.7.11.1 (non-specific serine/threonine protein kinase) inhibitor, an adenosine A2A receptor antagonist, an adjuvant, a food additive, a ryanodine receptor agonist, an adenosine receptor antagonist, a mouse metabolite, a plant metabolite, a fungal metabolite, an environmental contaminant and a human blood serum metabolite.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/2519#section=Record-Description): Caffeine is xanthine alkaloid that occurs naturally in seeds, leaves and fruit of several plants and trees that acts as a natural pesticide. Caffeine is a major component of coffee, tea and chocolate and in humans acts as a central nervous system (CNS) stimulant. Consumption of caffeine, even in high doses, has not been associated with elevations in serum enzyme elevations or instances of clinically apparent liver injury.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/2519#section=Uses): Caffeine is a substance in wide use. It is predominantly used in the food sector and pharma sector (80% and 16% rspectively). A smaller part is used in cosmetics (3%) or in technical applications (1%). Furthermore caffeine is a naturally occurring substance in various plant species (e.g. 0.9 to 2.6% in green coffee beans). It is a component in coffee, tea and cocoa.

</details>


### capsaicin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Capsaicin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · written · 1.1 s

> **First extracted in 1816**  
> The compound was first extracted in impure form in 1816 by Christian Friedrich Bucholz (1770–1818).

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Capsaicin#History): The compound was first extracted in impure form in 1816 by Christian Friedrich Bucholz (1770–1818).

</details>

**Where you meet it** · written · 4.8 s

> **In chili peppers and foods**  
> Capsaicin (8-methyl-N-vanillyl-6-nonenamide) is a toxin that is the main active component of chili peppers and gives them their distinct pungent, "spicy" or "hot" taste. Because of the burning sensation caused by capsaicin when it comes in contact with mucous membranes, it is commonly used in food products to provide added spiciness or "heat" (piquancy), usually in the form of spices such as chili powder and paprika.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Capsaicin): Capsaicin (8-methyl-N-vanillyl-6-nonenamide) is a toxin that is the main active component of chili peppers and gives them their distinct pungent, "spicy" or "hot" taste. It is a potent irritant for mammals for which it produces a sensation of burning in any tissue with which it comes into contact. Capsaicin and several related amides (capsaicinoids) are produced as secondary metabolites by chili peppers, likely as deterrents against eating by mammals and against the growth of fungi. Pure capsaicin is a hydrophobic, highly pungent (i.e., spicy) crystalline solid.
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Capsaicin#Uses): Because of the burning sensation caused by capsaicin when it comes in contact with mucous membranes, it is commonly used in food products to provide added spiciness or "heat" (piquancy), usually in the form of spices such as chili powder and paprika. In high concentrations, capsaicin will also cause a burning effect on other sensitive areas, such as skin or eyes. The degree of heat found within a food is often measured on the Scoville scale.

</details>

**Weird fact** · untitled · 14.1 s

> ****  
> In birds, the TRPV1 channel does not respond to capsaicin or related chemicals, but mammalian TRPV1 is very sensitive to it. This is advantageous to the plant, as chili pepper seeds consumed by birds pass through the digestive tract and can germinate later, whereas mammals have molar teeth that destroy such seeds and prevent them from germinating.

<details><summary>Sources</summary>

- [Wikipedia, Natural function](https://en.wikipedia.org/wiki/Capsaicin#Natural_function): The seeds of Capsicum plants are dispersed predominantly by birds. In birds, the TRPV1 channel does not respond to capsaicin or related chemicals, but mammalian TRPV1 is very sensitive to it. This is advantageous to the plant, as chili pepper seeds consumed by birds pass through the digestive tract and can germinate later, whereas mammals have molar teeth that destroy such seeds and prevent them from germinating. Thus, natural selection may have led to increasing capsaicin production because it makes the plant less likely to be eaten by animals that do not help it disperse. There is also evidence that capsaicin may have evolved as an anti-fungal agent.

</details>


### carbon monoxide (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Carbon Monoxide carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 3.1 s

> **First learned to control fire**  
> Man has maintained a complex relationship with carbon monoxide since first learning to control fire circa 800,000 BC. Early man probably discovered the toxicity of carbon monoxide poisoning upon introducing fire into his dwellings.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Carbon_monoxide#History): Man has maintained a complex relationship with carbon monoxide since first learning to control fire circa 800,000 BC. Early man probably discovered the toxicity of carbon monoxide poisoning upon introducing fire into his dwellings. The early development of metallurgy and smelting technologies emerging circa 6,000 BC through the Bronze Age likewise plagued man from carbon monoxide exposure. Apart from the toxicity of carbon monoxide, Native Americans in the United States may have experienced the neuroactive properties of carbon monoxide through shamanistic fireside rituals.

</details>

**Where you meet it** · untitled · 12.7 s

> ****  
> The most common source of carbon monoxide is the partial combustion of carbon-containing compounds. CO may be emitted from tobacco smoke and generated from malfunctioning fuel-burning stoves (wood, kerosene, natural gas, propane) and fuel-burning heating systems (wood, oil, natural gas) and from blocked flues connected to these appliances.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Carbon_monoxide): The most common source of carbon monoxide is the partial combustion of carbon-containing compounds. Numerous environmental and biological sources generate carbon monoxide. In industry, carbon monoxide is important in the production of many compounds, including drugs, fragrances, and fuels.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Carbon_monoxide): CO is one of the most acutely toxic contaminants affecting indoor air quality. CO may be emitted from tobacco smoke and generated from malfunctioning fuel-burning stoves (wood, kerosene, natural gas, propane) and fuel-burning heating systems (wood, oil, natural gas) and from blocked flues connected to these appliances. Carbon monoxide poisoning is the most common type of fatal air poisoning in many countries.

</details>

**Weird fact** · written · 6.8 s

> **Cleopatra may have died from poisoning**  
> Cleopatra may have died from carbon monoxide poisoning.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Carbon_monoxide#History): Early civilizations developed mythological tales to explain the origin of fire, such as Prometheus from Greek mythology who shared fire with humans. Aristotle (384–322 BC) first recorded that burning coals produced toxic fumes. Greek physician Galen (AD 129–199) speculated that there was a change in the composition of the air that caused harm when inhaled, and many others of the era developed a basis of knowledge about carbon monoxide in the context of coal fume toxicity. Cleopatra may have died from carbon monoxide poisoning.

</details>


### carvone (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Carvone, (+-)- carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 2.0 s

> **Isolated in 1849 by Varrentrapp**  
> Caraway was used for medicinal purposes by the ancient Romans, but carvone was probably not isolated as a pure compound until Franz Varrentrapp (1815–1877) obtained it in 1849. Goldschmidt and Zürrer identified it as a ketone related to limonene, and the structure was finally elucidated by Georg Wagner (1849–1903) in 1894.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Carvone#History): Caraway was used for medicinal purposes by the ancient Romans, but carvone was probably not isolated as a pure compound until Franz Varrentrapp (1815–1877) obtained it in 1849. It was originally called carvol by Schweizer. Goldschmidt and Zürrer identified it as a ketone related to limonene, and the structure was finally elucidated by Georg Wagner (1849–1903) in 1894.

</details>

**Where you meet it** · written · 6.5 s

> **Found in caraway, dill, and spearmint**  
> S-(+)-Carvone is the principal constituent (60–70%) of the oil from caraway seeds (Carum carvi), which is produced on a scale of about 10 tonnes per year. It also occurs to the extent of about 40–60% in dill seed oil (from Anethum graveolens), and also in mandarin orange peel oil. R-(−)-Carvone is also the most abundant compound in the essential oil from several species of mint, particularly spearmint oil (Mentha spicata), which is composed of 50–80% R-(−)-carvone.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Carvone#Occurrence): S-(+)-Carvone is the principal constituent (60–70%) of the oil from caraway seeds (Carum carvi), which is produced on a scale of about 10 tonnes per year. It also occurs to the extent of about 40–60% in dill seed oil (from Anethum graveolens), and also in mandarin orange peel oil. R-(−)-Carvone is also the most abundant compound in the essential oil from several species of mint, particularly spearmint oil (Mentha spicata), which is composed of 50–80% R-(−)-carvone. Spearmint is a major source of naturally produced R-(−)-carvone. However, the majority of R-(−)-carvone used in commercial applications is synthesized from R-(+)-limonene. The R-(−)-carvone isomer also occurs in kuromoji oil.

</details>

**Weird fact** · untitled · 15.4 s

> ****  
> Carvone has two mirror image forms, or enantiomers: R-(−)-carvone, the sweetish minty smell of spearmint leaves. Its mirror image, S-(+)-carvone, has a spicy aroma with notes of rye, and gives caraway seeds their smell. The fact that the two enantiomers are perceived as smelling different is evidence that olfactory receptors must respond more strongly to one enantiomer than to the other.

<details><summary>Sources</summary>

- [Wikipedia, Stereoisomerism and odor](https://en.wikipedia.org/wiki/Carvone#Stereoisomerism_and_odor): Carvone has two mirror image forms, or enantiomers: R-(−)-carvone, the sweetish minty smell of spearmint leaves. Its mirror image, S-(+)-carvone, has a spicy aroma with notes of rye, and gives caraway seeds their smell.
- [Wikipedia, Stereoisomerism and odor](https://en.wikipedia.org/wiki/Carvone#Stereoisomerism_and_odor): The fact that the two enantiomers are perceived as smelling different is evidence that olfactory receptors must respond more strongly to one enantiomer than to the other. Not all enantiomers have distinguishable odors. Squirrel monkeys have also been found to be able to discriminate between carvone enantiomers.

</details>


### cholesterol (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Cholesterol is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · skipped · 1.0 s

**Where you meet it** · retried · 9.4 s

> **Found in animal fats and oils**  
> The principal sterol of all higher animals, distributed in body tissues, especially the brain and spinal cord, and in animal fats and oils. Animal fats are complex mixtures of triglycerides, with lesser amounts of both the phospholipids and cholesterol molecules from which all animal (and human) cell membranes are constructed.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5997#section=Record-Description): The principal sterol of all higher animals, distributed in body tissues, especially the brain and spinal cord, and in animal fats and oils.
- [Wikipedia, Dietary sources](https://en.wikipedia.org/wiki/Cholesterol#Dietary_sources): Animal fats are complex mixtures of triglycerides, with lesser amounts of both the phospholipids and cholesterol molecules from which all animal (and human) cell membranes are constructed. Since all animal cells manufacture cholesterol, all animal-based foods contain cholesterol in varying amounts. Major dietary sources of cholesterol include red meat, egg yolks and whole eggs, liver, kidney, giblets, fish oil, shellfish, and butter. Human breast milk also contains significant quantities of cholesterol.

</details>

**Weird fact** · untitled · 12.9 s

> ****  
> Synthesis within the body starts with the mevalonate pathway where two molecules of acetyl-CoA condense to form acetoacetyl-CoA. This is followed by a second condensation between acetyl-CoA and acetoacetyl-CoA to form 3-hydroxy-3-methylglutaryl CoA (HMG-CoA). Cholesterol also serves as a precursor for the biosynthesis of steroid hormones, bile acid, and vitamin D.

<details><summary>Sources</summary>

- [Wikipedia, Biosynthesis and regulation](https://en.wikipedia.org/wiki/Cholesterol#Biosynthesis_and_regulation): Synthesis within the body starts with the mevalonate pathway where two molecules of acetyl-CoA condense to form acetoacetyl-CoA. This is followed by a second condensation between acetyl-CoA and acetoacetyl-CoA to form 3-hydroxy-3-methylglutaryl CoA (HMG-CoA).
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Cholesterol): Cholesterol is biosynthesized by all animal cells and is an essential structural and signaling component of animal cell membranes. In vertebrates, hepatic cells typically produce the greatest amounts of it. In the brain, astrocytes produce cholesterol and transport it to neurons. It is absent among prokaryotes (bacteria and archaea), although there are some exceptions, such as Mycoplasma, which require cholesterol for growth. Cholesterol also serves as a precursor for the biosynthesis of steroid hormones, bile acid, and vitamin D.

</details>


### cinnamaldehyde (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Cinnamaldehyde carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · untitled · 12.0 s

> ****  
> Cinnamaldehyde is a naturally occurring flavonoid that gives the spice cinnamon its flavour and odour. It occurs naturally in the bark of cinnamon trees and other species of the genus Cinnamomum such as camphor and cassia. Cinnamaldehyde occurs naturally in the bark of cinnamon trees and other species of the genus Cinnamomum like camphor and cassia. These trees are the natural source of cinnamon, and the essential oil of cinnamon bark is about 90% cinnamaldehyde.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/637511#section=Record-Description): Cinnamaldehyde is a naturally occurring flavonoid that gives the spice cinnamon its flavour and odour. It occurs naturally in the bark of cinnamon trees and other species of the genus Cinnamomum such as camphor and cassia. Sensitivity to cinnamaldehyde may be identified with a clinical patch test.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/637511#section=Record-Description): Cinnamaldehyde is the aldehyde that gives cinnamon its flavor and odor. Cinnamaldehyde occurs naturally in the bark of cinnamon trees and other species of the genus Cinnamomum like camphor and cassia. These trees are the natural source of cinnamon, and the essential oil of cinnamon bark is about 90% cinnamaldehyde. Cinnamaldehyde is also used as a fungicide. Proven effective on over 40 different crops, cinnamaldehyde is typically applied to the root systems of plants. Its low toxicity and well-known properties make it ideal for agriculture. To a lesser extent, cinnamaldehyde is an effective insecticide, and its scent is also known to repel animals like cats and dogs.

</details>

**Weird fact** · written · 7.6 s

> **Kills half of mosquito larvae in 24 hours**  
> A concentration of 29 ppm of cinnamaldehyde kills half of Aedes aegypti mosquito larvae in 24 hours.

<details><summary>Sources</summary>

- [Wikipedia, Applications](https://en.wikipedia.org/wiki/Cinnamaldehyde#Applications): Cinnamaldehyde has been tested as a safe and effective insecticide against mosquito larvae. A concentration of 29 ppm of cinnamaldehyde kills half of Aedes aegypti mosquito larvae in 24 hours. Trans-cinnamaldehyde works as a potent fumigant and practical repellant for adult mosquitos. It also has antibacterial and antifungal properties.

</details>


### citric acid (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Citric Acid carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · written · 7.2 s

> **Found in citrus fruits**  
> A key intermediate in metabolism. It is an acid compound found in citrus fruits. Citric acid is an organic compound with the formula C6H8O7. It is a colorless weak organic acid. It occurs naturally in citrus fruits.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/311#section=Record-Description): A key intermediate in metabolism. It is an acid compound found in citrus fruits. The salts of citric acid (citrates) can be used as anticoagulants due to their calcium-chelating ability. Citric acid is one of the active ingredients in Phexxi, a non-hormonal contraceptive agent that was approved by the FDA on May 2020. It is also used in combination with magnesium oxide to form magnesium citrate, an osmotic laxative.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Citric_acid): Citric acid is an organic compound with the formula C6H8O7. It is a colorless weak organic acid. It occurs naturally in citrus fruits. In biochemistry, it is an intermediate in the citric acid cycle, which occurs in the metabolism of all aerobic organisms.

</details>

**Weird fact** · retried · 14.3 s

> **TCA cycle may have predated life**  
> Recent research indicates that the TCA cycle may have predated life on earth and may have provided the pathway for formation of amino acids(1).

<details><summary>Sources</summary>

- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/311#section=Natural-Pollution-Sources): In nature, the citric acid cycle (Krebs cycle, TCA cycle, tricarboxylic acid cycle) is a series of enzymatic reactions occurring in living cells of aerobic organisms, the net result of which is the conversion of pyruvic acid, formed by anaerobic metabolism of carbohydrates, into carbon dioxide and water. The metabolic intermediates are degraded by a combination of decarboxylation and dehydrogenation. It is the major terminal pathway of oxidation in animal, bacterial, and plant cells. Recent research indicates that the TCA cycle may have predated life on earth and may have provided the pathway for formation of amino acids(1).

</details>


### cubane (2 cards)

**Where you meet it** · retried · 4.0 s

> **Synthetic hydrocarbon compound**  
> Cubane is a synthetic hydrocarbon compound with the formula C8H8. It consists of eight carbon atoms arranged at the corners of a cube, with one hydrogen atom attached to each carbon atom.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Cubane): Cubane is a synthetic hydrocarbon compound with the formula C8H8. It consists of eight carbon atoms arranged at the corners of a cube, with one hydrogen atom attached to each carbon atom. A solid crystalline substance, cubane is one of the Platonic hydrocarbons and a member of the prismanes. It was first synthesized in 1964 by Philip Eaton and Thomas Cole. Before this work, Eaton believed that cubane would be impossible to synthesize due to the "required 90 degree bond angles". The cubic shape requires the carbon atoms to adopt an unusually sharp 90° bonding angle, which would be highly strained as compared to the 109.45° angle of a tetrahedral carbon.

</details>

**Weird fact** · written · 2.3 s

> **Cannot decompose into pyramidal alkenes**  
> Cubane is highly strained, but cannot decompose: the resultant cubene molecules would be pyramidal alkenes, too high-energy for most elimination pathways.

<details><summary>Sources</summary>

- [Wikipedia, Reactions and derivatives](https://en.wikipedia.org/wiki/Cubane#Reactions_and_derivatives): Cubane is highly strained, but cannot decompose: the resultant cubene molecules would be pyramidal alkenes, too high-energy for most elimination pathways. Certain metallic ions catalyze σ-bond rearrangement to cuneane:

</details>


### dopamine (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Dopamine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 2.2 s

> **First synthesized in 1910**  
> Dopamine was first synthesized in 1910 by George Barger and James Ewens at Wellcome Laboratories in London, England and first identified in the human brain by Katharine Montagu in 1957.

<details><summary>Sources</summary>

- [Wikipedia, History and development](https://en.wikipedia.org/wiki/Dopamine#History_and_development): Dopamine was first synthesized in 1910 by George Barger and James Ewens at Wellcome Laboratories in London, England and first identified in the human brain by Katharine Montagu in 1957. It was named dopamine because it is a monoamine whose precursor in the Barger-Ewens synthesis is 3,4-dihydroxyphenylalanine (levodopa or L-DOPA). Dopamine's function as a neurotransmitter was first recognized in 1958 by Arvid Carlsson and Nils-Åke Hillarp at the Laboratory for Chemical Pharmacology of the National Heart Institute of Sweden.

</details>

**Where you meet it** · written · 4.9 s

> **Found in brain and plants**  
> One of the catecholamine neurotransmitters in the brain. Dopamine is also synthesized in plants and most animals.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/681#section=Record-Description): One of the catecholamine neurotransmitters in the brain. It is derived from tyrosine and is the precursor to norepinephrine and epinephrine. Dopamine is a major transmitter in the extrapyramidal system of the brain, and important in regulating movement. A family of receptors (receptors, dopamine) mediate its action.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Dopamine): Dopamine (DA, a contraction of 3,4-dihydroxyphenethylamine) is a neuromodulatory molecule that plays several important roles in cells. It is an organic chemical of the catecholamine and phenethylamine families. It is an amine synthesized by removing a carboxyl group through decarboxylation from a molecule of its precursor chemical, L-DOPA, which is synthesized in the brain and kidneys. Dopamine is also synthesized in plants and most animals. In the brain, dopamine functions as a neurotransmitter—a chemical released by neurons (nerve cells) to send signals to other nerve cells.

</details>

**Weird fact** · dropped · 13.3 s

Problems: the chosen sentences have 114 words; keep it under 90


### epinephrine (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Epinephrine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 3.7 s

> **Found in adrenal glands and nature**  
> (R)-adrenaline is the R-enantiomer of adrenaline. It is a hormone secreted by the adrenal glands resulting in the 'fight-or-flight' response. Epinephrine is a naturally occurring hormone, which is released from the adrenal medulla and is involved in the regulation of blood pressure and smooth muscle constriction(1).

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Record-Description): (R)-adrenaline is the R-enantiomer of adrenaline. It is a hormone secreted by the adrenal glands resulting in the 'fight-or-flight' response. It has a role as a hormone, a beta-adrenergic agonist, a bronchodilator agent, a sympathomimetic agent, an alpha-adrenergic agonist, a vasodilator agent, an adrenergic agonist, a mydriatic agent, a vasoconstrictor agent and a mouse metabolite. It is a conjugate base of a (R)-adrenaline(1+). It is an enantiomer of a (S)-adrenaline.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Natural-Pollution-Sources): Epinephrine is a naturally occurring hormone, which is released from the adrenal medulla and is involved in the regulation of blood pressure and smooth muscle constriction(1). Epinephrine naturally occurs at the ends of sympathetic nerve fibers where they serve as chemical mediators for carrying nerve impulses(2).

</details>

**Weird fact** · untitled · 8.2 s

> ****  
> epinephrine has been reported in Vicia faba, Hemidesmus indicus, and other organisms with data available.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Record-Description): epinephrine has been reported in Vicia faba, Hemidesmus indicus, and other organisms with data available.

</details>


### ethanol (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Ethanol carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 2.9 s

> **First prepared synthetically in 1825**  
> Ethanol was first prepared synthetically in 1825 by Michael Faraday. In 1828, Hennell and the French chemist Georges-Simon Serullas independently discovered that sulphovinic acid could be decomposed into ethanol.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Ethanol#History): Ethanol was first prepared synthetically in 1825 by Michael Faraday. He found that sulfuric acid could absorb large volumes of coal gas. He gave the resulting solution to Henry Hennell, a British chemist, who found in 1826 that it contained "sulphovinic acid" (ethyl hydrogen sulfate). In 1828, Hennell and the French chemist Georges-Simon Serullas independently discovered that sulphovinic acid could be decomposed into ethanol. Thus, in 1825 Faraday had unwittingly discovered that ethanol could be produced from ethylene (a component of coal gas) by acid-catalyzed hydration, a process similar to current industrial ethanol synthesis.

</details>

**Weird fact** · written · 6.3 s

> **Found in outer space on dust grains**  
> Ethanol has been detected in outer space, forming an icy coating around dust grains in interstellar clouds.

<details><summary>Sources</summary>

- [Wikipedia, Natural occurrence](https://en.wikipedia.org/wiki/Ethanol#Natural_occurrence): Ethanol has been detected in outer space, forming an icy coating around dust grains in interstellar clouds.

</details>

**Where you meet it** · written · 6.7 s

> **Found in beverages and nature**  
> Ethanol can commonly be found in overripe fruit. Clear, colorless liquid rapidly absorbed from the gastrointestinal tract and distributed throughout the body. It has bactericidal activity and is used often as a topical disinfectant. It is widely used as a solvent and preservative in pharmaceutical preparations as well as serving as the primary ingredient in ALCOHOLIC BEVERAGES. [ChemIDplus]

<details><summary>Sources</summary>

- [Wikipedia, Natural occurrence](https://en.wikipedia.org/wiki/Ethanol#Natural_occurrence): Ethanol is a byproduct of the metabolic process of yeast. As such, ethanol will be present in any yeast habitat. Ethanol can commonly be found in overripe fruit. Ethanol produced by symbiotic yeast can be found in bertam palm blossoms. Although some animal species, such as the pentailed treeshrew, exhibit ethanol-seeking behaviors, most show no interest or avoidance of food sources containing ethanol. Ethanol is also produced during the germination of many plants as a result of natural anaerobiosis.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/702#section=Uses): Clear, colorless liquid rapidly absorbed from the gastrointestinal tract and distributed throughout the body. It has bactericidal activity and is used often as a topical disinfectant. It is widely used as a solvent and preservative in pharmaceutical preparations as well as serving as the primary ingredient in ALCOHOLIC BEVERAGES. [ChemIDplus]

</details>


### ethylene (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Ethylene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 2.9 s

> **Daughter of ethyl**  
> Thus, ethylene (C2H4) was the "daughter of ethyl" (C2H5). The name ethylene was used in this sense as early as 1852.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Ethylene#History): In the mid-19th century, the suffix -ene (an Ancient Greek root added to the end of female names meaning "daughter of") was widely used to refer to a molecule or part thereof that contained one fewer hydrogen atoms than the molecule being modified. Thus, ethylene (C2H4) was the "daughter of ethyl" (C2H5). The name ethylene was used in this sense as early as 1852.

</details>

**Where you meet it** · written · 5.3 s

> **Fruits, flowers, and nature**  
> Ethylene is a natural product emitted by fruits, flowers, leaves, roots, and tubers(1). Ethylene has been found in the gaseous metabolites released by germinating bean, corn, cotton, and pea seeds, from fading morning glory flowers and from ripening avocados and apples(2).

<details><summary>Sources</summary>

- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/6325#section=Natural-Pollution-Sources): Ethylene is a natural product emitted by fruits, flowers, leaves, roots, and tubers(1). The release rate of ethylene from plants varies during the life cycle of the plant(1). Plants that normally produce 0.6 to 6 ug ethylene/kg (fresh weight) per hour may produce up to 120 ug/kg per hour during ripening of fruits and during senescence and loss of leaves(1). Ethylene has been found in the gaseous metabolites released by germinating bean, corn, cotton, and pea seeds, from fading morning glory flowers and from ripening avocados and apples(2). Ethylene is released to the atmosphere from biomass combustion and volcanos(3).

</details>

**Weird fact** · untitled · 13.5 s

> ****  
> In the mid-19th century, the suffix -ene (an Ancient Greek root added to the end of female names meaning "daughter of") was widely used to refer to a molecule or part thereof that contained one fewer hydrogen atoms than the molecule being modified. Thus, ethylene (C2H4) was the "daughter of ethyl" (C2H5).

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Ethylene#History): In the mid-19th century, the suffix -ene (an Ancient Greek root added to the end of female names meaning "daughter of") was widely used to refer to a molecule or part thereof that contained one fewer hydrogen atoms than the molecule being modified. Thus, ethylene (C2H4) was the "daughter of ethyl" (C2H5). The name ethylene was used in this sense as early as 1852.

</details>


### formaldehyde (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Formaldehyde carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · untitled · 13.7 s

> ****  
> A ~5% solution in water of formaldehyde is used as a disinfectant and fumigant in industrial, agricultural, and healthcare settings, and a ~37% solution is used to preserve tissue samples in labs. Formaldehyde also occurs naturally. Formaldehyde in food can be present naturally, added as an inadvertent contaminant, or intentionally added as a preservative, disinfectant, or bacteriostatic agent.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Formaldehyde): Formaldehyde ( for-MAL-di-hide, US also fər-) (systematic name methanal) is a pungent colorless gas mainly used in the production of industrial resins, such as for particle board, coatings, plastics, pulp, paper, synthetic fibers, and textiles. A ~5% solution in water of formaldehyde is used as a disinfectant and fumigant in industrial, agricultural, and healthcare settings, and a ~37% solution is used to preserve tissue samples in labs. In 2024, the global production of formaldehyde was estimated at 26 million tons per year, and is a precursor to many other materials and chemical compounds.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Formaldehyde): Formaldehyde also occurs naturally. It is derived from the degradation of serine, dimethylglycine, and lipids. Demethylases act by converting N-methyl groups to formaldehyde.
- [Wikipedia, Contaminant in food](https://en.wikipedia.org/wiki/Formaldehyde#Contaminant_in_food): Formaldehyde in food can be present naturally, added as an inadvertent contaminant, or intentionally added as a preservative, disinfectant, or bacteriostatic agent. Cooking and smoking food can also result in formaldehyde being produced in food. Foods that the U.S. National Toxicology Program has reported to have higher levels compared to other foods are fish, seafood, and smoked ham. It also notes that formaldehyde in food generally occurs in a bound form and that formaldehyde is unstable in an aqueous solution.

</details>

**Weird fact** · untitled · 12.0 s

> ****  
> Monomeric CH2O is a gas and is rarely encountered in the laboratory. Aqueous formaldehyde, unlike some other small aldehydes (which need specific conditions to oligomerize through aldol condensation) oligomerizes spontaneously at a common state.

<details><summary>Sources</summary>

- [Wikipedia, Organic chemistry](https://en.wikipedia.org/wiki/Formaldehyde#Organic_chemistry): Monomeric CH2O is a gas and is rarely encountered in the laboratory. Aqueous formaldehyde, unlike some other small aldehydes (which need specific conditions to oligomerize through aldol condensation) oligomerizes spontaneously at a common state. The trimer 1,3,5-trioxane, (CH2O)3, is a typical oligomer. Many cyclic oligomers of other sizes have been isolated. Similarly, formaldehyde hydrates to give the geminal diol methanediol, which condenses further to form hydroxy-terminated oligomers HO(CH2O)nH. The polymer is called paraformaldehyde. The higher concentration of formaldehyde—the more equilibrium shifts towards polymerization.

</details>


### geosmin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Geosmin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · written · 2.8 s

> **Found in soil and water**  
> geosmin has been reported in Basella alba, Streptomyces, and other organisms with data available. Geosmin is responsible for the earthy taste of beetroots and a contributor to the strong scent, known as petrichor, that occurs when rain falls after a spell of dry weather or when soil is disturbed. It is produced by bacteria, especially cyanobacteria, often present in soil and water supplies.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/29746#section=Record-Description): geosmin has been reported in Basella alba, Streptomyces, and other organisms with data available.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Geosmin): Geosmin ( jee-OZ-min) is a chemical with a distinct earthy or musty odor. It is easily smelled by most people, having a very low odor detection threshold, ranging from 0.006 to 0.01 micrograms per liter in water. Geosmin is responsible for the earthy taste of beetroots and a contributor to the strong scent, known as petrichor, that occurs when rain falls after a spell of dry weather or when soil is disturbed. It is produced by bacteria, especially cyanobacteria, often present in soil and water supplies. Geosmin, along with 2-methylisoborneol, are the major biological causes of muddy odor and flavor in drinking water worldwide and in farmed fish.

</details>

**Weird fact** · untitled · 8.3 s

> ****  
> The human olfactory system is extremely sensitive to geosmin and is able to detect it at concentrations as low as anywhere from 400 parts per trillion to 5 parts per trillion. In a study of 50 subjects, the mean odor detection threshold was 9.5 ng/L for naturally occurring (−)-geosmin and 78 ng/L for (+)-geosmin.

<details><summary>Sources</summary>

- [Wikipedia, Effects](https://en.wikipedia.org/wiki/Geosmin#Effects): The human olfactory system is extremely sensitive to geosmin and is able to detect it at concentrations as low as anywhere from 400 parts per trillion to 5 parts per trillion. Human sensitivity differs between the two enantiomers. In a study of 50 subjects, the mean odor detection threshold was 9.5 ng/L for naturally occurring (−)-geosmin and 78 ng/L for (+)-geosmin. This makes the natural form roughly eight times more potent. It is proposed that this extreme sensitivity to geosmin evolved in humans or their ancestors to aid them in their search for scarce water, but this is not conclusive.

</details>


### glucose (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> D-Glucose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · written · 2.5 s

> **First isolated from raisins**  
> Glucose was first isolated from raisins in 1747 by the German chemist Andreas Marggraf.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Glucose#History): Glucose was first isolated from raisins in 1747 by the German chemist Andreas Marggraf. Glucose was discovered in grapes by another German chemist, Johann Tobias Lowitz, in 1792, and distinguished as being different from cane sugar (sucrose). Glucose is the term coined by Jean-Baptiste Dumas in 1838, which has prevailed in the chemical literature. Friedrich August Kekulé proposed the term dextrose (from the Latin dexter, meaning "right"), because in aqueous solutions of glucose, the plane of linearly polarized light is turned to the right. In contrast, l-fructose (usually referred to as l-fructose) (a ketohexose) and l-glucose (l-glucose) turn linearly polarized light to the left.

</details>

**Weird fact** · written · 6.1 s

> **d-Glucose turns light to the right**  
> Friedrich August Kekulé proposed the term dextrose (from the Latin dexter, meaning "right"), because in aqueous solutions of glucose, the plane of linearly polarized light is turned to the right. The d-isomer, d-glucose, also known as dextrose, occurs widely in nature, but the l-isomer, l-glucose, does not.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Glucose#History): Glucose was first isolated from raisins in 1747 by the German chemist Andreas Marggraf. Glucose was discovered in grapes by another German chemist, Johann Tobias Lowitz, in 1792, and distinguished as being different from cane sugar (sucrose). Glucose is the term coined by Jean-Baptiste Dumas in 1838, which has prevailed in the chemical literature. Friedrich August Kekulé proposed the term dextrose (from the Latin dexter, meaning "right"), because in aqueous solutions of glucose, the plane of linearly polarized light is turned to the right. In contrast, l-fructose (usually referred to as l-fructose) (a ketohexose) and l-glucose (l-glucose) turn linearly polarized light to the left.
- [Wikipedia, Chemical and physical properties](https://en.wikipedia.org/wiki/Glucose#Chemical_and_physical_properties): With six carbon atoms, it is classed as a hexose, a subcategory of the monosaccharides. d-Glucose is one of the sixteen aldohexose stereoisomers. The d-isomer, d-glucose, also known as dextrose, occurs widely in nature, but the l-isomer, l-glucose, does not. Glucose can be obtained by hydrolysis of carbohydrates such as milk sugar (lactose), cane sugar (sucrose), maltose, cellulose, glycogen, etc. Dextrose is commonly commercially manufactured from starches, such as corn starch in the US and Japan, from potato and wheat starch in Europe, and from tapioca starch in tropical areas.

</details>

**Where you meet it** · dropped · 12.0 s

Problems: the chosen sentences have 104 words; keep it under 90


### glyphosate (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Glyphosate carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 2.2 s

> **First synthesized in 1950**  
> Glyphosate was first synthesized in 1950 by Swiss chemist Henry Martin, who worked for the Swiss company Cilag. The work was never published.

<details><summary>Sources</summary>

- [Wikipedia, Discovery](https://en.wikipedia.org/wiki/Glyphosate#Discovery): Glyphosate was first synthesized in 1950 by Swiss chemist Henry Martin, who worked for the Swiss company Cilag. The work was never published. Early studies found it to be a weak chemical chelating agent.

</details>

**Weird fact** · written · 6.6 s

> **Blocks plants' amino acid pathway**  
> Glyphosate interferes with the shikimate pathway, which produces the aromatic amino acids phenylalanine, tyrosine and tryptophan in plants and microorganisms – but does not exist in the genome of animals, including humans. It blocks this pathway by inhibiting the enzyme 5-enolpyruvylshikimate-3-phosphate synthase (EPSPS), which catalyzes the reaction of shikimate-3-phosphate (S3P) and phosphoenolpyruvate to form 5-enolpyruvyl-shikimate-3-phosphate (EPSP).

<details><summary>Sources</summary>

- [Wikipedia, Mode of action](https://en.wikipedia.org/wiki/Glyphosate#Mode_of_action): Glyphosate interferes with the shikimate pathway, which produces the aromatic amino acids phenylalanine, tyrosine and tryptophan in plants and microorganisms – but does not exist in the genome of animals, including humans. It blocks this pathway by inhibiting the enzyme 5-enolpyruvylshikimate-3-phosphate synthase (EPSPS), which catalyzes the reaction of shikimate-3-phosphate (S3P) and phosphoenolpyruvate to form 5-enolpyruvyl-shikimate-3-phosphate (EPSP). Glyphosate is absorbed through foliage and minimally through roots, meaning that it is only effective on actively growing plants and cannot prevent seeds from germinating.

</details>

**Where you meet it** · dropped · 12.7 s

Problems: s9.4 starts with "It" but the sentence before it was not chosen


### hydrogen cyanide (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Hydrogen Cyanide carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 1.5 s

> **First isolated in 1752 by Pierre Macquer**  
> Hydrogen cyanide was first isolated in 1752 by French chemist Pierre Macquer who converted Prussian blue to an iron oxide plus a volatile component and found that these could be used to reconstitute it. The new component was what is now known as hydrogen cyanide.

<details><summary>Sources</summary>

- [Wikipedia, History of discovery and naming](https://en.wikipedia.org/wiki/Hydrogen_cyanide#History_of_discovery_and_naming): Hydrogen cyanide was first isolated in 1752 by French chemist Pierre Macquer who converted Prussian blue to an iron oxide plus a volatile component and found that these could be used to reconstitute it. The new component was what is now known as hydrogen cyanide. It was subsequently prepared from Prussian blue by the Swedish chemist Carl Wilhelm Scheele in 1782, and was eventually given the German name Blausäure (lit. 'Blue acid') because of its acidic nature in water and its derivation from Prussian blue. In English, it became known popularly as prussic acid.

</details>

**Where you meet it** · untitled · 14.3 s

> ****  
> HCN is obtainable from fruits that have a pit, such as cherries, apricots, apples, and nuts such as bitter almonds, from which almond oil and extract is made. Many of these pits contain small amounts of cyanohydrins such as mandelonitrile and amygdalin, which slowly release hydrogen cyanide.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Hydrogen_cyanide#Occurrence): HCN is obtainable from fruits that have a pit, such as cherries, apricots, apples, and nuts such as bitter almonds, from which almond oil and extract is made. Many of these pits contain small amounts of cyanohydrins such as mandelonitrile and amygdalin, which slowly release hydrogen cyanide. One hundred grams of crushed apple seeds can yield about 70 mg of HCN.

</details>

**Weird fact** · untitled · 14.4 s

> ****  
> About half of people are unable to detect the odor of hydrogen cyanide owing to a recessive genetic trait. Has a faint odor of almonds.

<details><summary>Sources</summary>

- [Wikipedia, Structure and general properties](https://en.wikipedia.org/wiki/Hydrogen_cyanide#Structure_and_general_properties): About half of people are unable to detect the odor of hydrogen cyanide owing to a recessive genetic trait. For those that can detect it, the odor has been described as bitter almond-like.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/768#section=Record-Description): Hydrocyanic acid, aqueous solution, with not more than 20% hydrogen cyanide is a clear colorless aqueous solution of a gas. Has a faint odor of almonds. Can evolve hydrogen cyanide gas, which is (barely) lighter than air. Flame can flash back to the source of a gas leak very easily. Lethal doses of gas may be inhaled. Lethal doses of cyanide can be absorbed from the solution through the skin.

</details>


### ibuprofen (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Ibuprofen, (+-)- carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · written · 3.1 s

> **Discovered in 1960s by Boots team**  
> Ibuprofen was derived from propionic acid by the research arm of Boots Group during the 1960s. The molecule was discovered and synthesized by a team led by Stewart Adams, with a patent application filed in 1961. Adams initially tested the drug as treatment for his hangover.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Ibuprofen#History): Ibuprofen was derived from propionic acid by the research arm of Boots Group during the 1960s. The name is derived from the 3 functional groups: isobutyl (ibu) propionic acid (pro) phenyl (fen). Its discovery was the result of research during the 1950s and 1960s to find a safer alternative to aspirin. The molecule was discovered and synthesized by a team led by Stewart Adams, with a patent application filed in 1961. Adams initially tested the drug as treatment for his hangover. In 1985, Boots's worldwide patent for ibuprofen expired and generic products were launched.

</details>

**Where you meet it** · retried · 11.4 s

> **Available in over-the-counter pain relievers**  
> Ibuprofen was discovered in 1961 in Nottingham by Stewart Adams and John Nicholson while working at Boots the Chemists, and initially sold as Brufen. It is available under a number of brand names including Advil, Brufen, Motrin, and Nurofen. Ingredient in over-the-counter pain relievers

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Ibuprofen): Ibuprofen was discovered in 1961 in Nottingham by Stewart Adams and John Nicholson while working at Boots the Chemists, and initially sold as Brufen. It is available under a number of brand names including Advil, Brufen, Motrin, and Nurofen. Ibuprofen was first sold in 1969 in the United Kingdom and in 1974 in the United States. It is on the World Health Organization's List of Essential Medicines. It is available as a generic medication. In 2023, it was the 32nd most commonly prescribed medication in the United States, with more than 17 million prescriptions.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/3672#section=Uses): Ingredient in over-the-counter pain relievers

</details>

**Weird fact** · retried · 15.0 s

> **Lowers Parkinson's disease risk**  
> Ibuprofen has been associated with a lower risk of Parkinson's disease and may delay or prevent it. People regularly consuming ibuprofen were reported to have a 38% lower risk of developing Parkinson's disease, but no such effect was found for other pain relievers, such as aspirin and paracetamol.

<details><summary>Sources</summary>

- [Wikipedia, Research](https://en.wikipedia.org/wiki/Ibuprofen#Research): Ibuprofen has been associated with a lower risk of Parkinson's disease and may delay or prevent it. Aspirin, other NSAIDs, and paracetamol (acetaminophen) did not affect the risk for Parkinson's. In March 2011, researchers at Harvard Medical School announced that ibuprofen had a neuroprotective effect against the risk of developing Parkinson's disease. People regularly consuming ibuprofen were reported to have a 38% lower risk of developing Parkinson's disease, but no such effect was found for other pain relievers, such as aspirin and paracetamol.

</details>


### indigo (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Indigo carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · skipped · 2.5 s

**Where you meet it** · written · 2.8 s

> **In denim cloth and nature**  
> Indigo is a natural dye obtained from the leaves of some plants of the Indigofera genus, in particular Indigofera tinctoria. The primary use for indigo is as a dye for cotton yarn, mainly used in the production of denim cloth suitable for blue jeans; on average, a pair of blue jeans requires 3–12 grams (0.11–0.42 oz) of dye.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Indigo_dye): Indigo dye is an organic compound with a distinctive blue color. Indigo is a natural dye obtained from the leaves of some plants of the Indigofera genus, in particular Indigofera tinctoria. Dye-bearing Indigofera plants were once common throughout the world. Blue colorants are relatively rare. Since indigo is insoluble, it is also referred to as a pigment (C.I. Pigment Blue 66, C.I.).
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Indigo_dye#Uses): The primary use for indigo is as a dye for cotton yarn, mainly used in the production of denim cloth suitable for blue jeans; on average, a pair of blue jeans requires 3–12 grams (0.11–0.42 oz) of dye. Smaller quantities are used in the dyeing of wool and silk.

</details>

**Weird fact** · untitled · 14.1 s

> ****  
> Reduction converts indigo into "white indigo" (leuco-indigo). When a submerged fabric is removed from the dyebath, the white indigo quickly combines with oxygen in the air and reverts to the insoluble, intensely colored indigo.

<details><summary>Sources</summary>

- [Wikipedia, Dyeing technology](https://en.wikipedia.org/wiki/Indigo_dye#Dyeing_technology): Indigo is a challenging dye because it is not soluble in water. To be dissolved, it must undergo a chemical change (reduction). Reduction converts indigo into "white indigo" (leuco-indigo). When a submerged fabric is removed from the dyebath, the white indigo quickly combines with oxygen in the air and reverts to the insoluble, intensely colored indigo. When it first became widely available in Europe in the 16th century, European dyers and printers struggled with indigo because of this distinctive property. It also required several chemical manipulations, some involving toxic materials, and presented many opportunities to injure workers.

</details>


### limonene (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> 1-Methyl-4-Prop-1-En-2-Ylcyclohexene carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · untitled · 10.2 s

> ****  
> Limonene is found in many oils and fruits including orange, lemon, grapefruit, berry, leaf, caraway, dill, bergamot, peppermint and spearmint oils(1-4). Limonene emissions to the environment are associated with wax myrtle, sweet acacia, oranges, tomatoes, grasses, and California western sagebrush(5).

<details><summary>Sources</summary>

- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/22311#section=Natural-Pollution-Sources): Limonene is found in many oils and fruits including orange, lemon, grapefruit, berry, leaf, caraway, dill, bergamot, peppermint and spearmint oils(1-4). Limonene emissions to the environment are associated with wax myrtle, sweet acacia, oranges, tomatoes, grasses, and California western sagebrush(5). Emissions of limonene are also associated with balsam poplar, European larche, European fir, Scots pine, Siberian pine, silver fir, common juniper, zeravshan juniper, pencil cedar, evergreen cypress, northern white cedar, chinese arbor vitae, marsh tea and deciduous moss(6).

</details>

**Weird fact** · written · 5.3 s

> **Racemizes at 300 °C**  
> Limonene occurs commonly as the (R)-enantiomer, but racemizes at 300 °C.

<details><summary>Sources</summary>

- [Wikipedia, Chemical reactions](https://en.wikipedia.org/wiki/Limonene#Chemical_reactions): Limonene occurs commonly as the (R)-enantiomer, but racemizes at 300 °C. When warmed with mineral acid, limonene isomerizes to the conjugated diene α-terpinene (which can also easily be converted to p-cymene). Evidence for this isomerization includes the formation of Diels–Alder adducts between α-terpinene adducts and maleic anhydride.

</details>


### lycopene (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Lycopene is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Where you meet it** · written · 3.6 s

> **Found in tomatoes and watermelons**  
> Lycopene is an acyclic carotene commonly obtained from tomatoes and other red fruits. Aside from tomatoes or tomato products like ketchup, it is found in watermelons, grapefruits, red guavas, and baked beans.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/446925#section=Record-Description): Lycopene is an acyclic carotene commonly obtained from tomatoes and other red fruits. It has a role as an antioxidant and a plant metabolite. It contains a carotenoid psi-end derivative.
- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Lycopene#Occurrence): Aside from tomatoes or tomato products like ketchup, it is found in watermelons, grapefruits, red guavas, and baked beans. It has no vitamin A activity.

</details>

**Weird fact** · retried · 12.6 s

> **Most predominant isomer in foods**  
> Lycopene may undergo extensive isomerization that allows 1056 theoretical cis-trans configurations; however, the all-trans configuration of lycopene is the most predominant isomer found in foods that gives the red hue. Lycopene is a linear, unsaturated hydrocarbon carotenoid, the major red pigment in fruits such as tomatoes, pink grapefruit, apricots, red oranges, watermelon, rosehips, and guava.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/446925#section=Record-Description): Lycopene is a naturally occurring red carotenoid pigment that is responsible for red to pink colors seen in tomatoes, pink grapefruit, and other foods. Having a chemical formula of C40H56, lycopene is a tetraterpene assembled from eight isoprene units that are solely composed of carbon and hydrogen. Lycopene may undergo extensive isomerization that allows 1056 theoretical cis-trans configurations; however, the all-trans configuration of lycopene is the most predominant isomer found in foods that gives the red hue.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/446925#section=Record-Description): Lycopene is a linear, unsaturated hydrocarbon carotenoid, the major red pigment in fruits such as tomatoes, pink grapefruit, apricots, red oranges, watermelon, rosehips, and guava. As a class, carotenoids are pigment compounds found in photosynthetic organisms (plants, algae, and some types of fungus), and are chemically characterized by a large polyene chain containing 35-40 carbon atoms; some carotenoid polyene chains are terminated by two 6-carbon rings. In animals, carotenoids such as lycopene may possess antioxidant properties which may retard aging and many degenerative diseases. As an essential nutrient, lycopene is required in the animal diet. (NCI04)

</details>


### melatonin (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Melatonin is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · written · 2.8 s

> **Discovered in pineal gland extracts**  
> Melatonin's discovery is linked to the study of color changes in some amphibians and reptiles, a phenomenon initially observed through the administration of pineal gland extracts. The hormone melatonin was isolated in 1958 by Aaron B. Lerner, a dermatology professor, and his team at Yale University. Motivated by the possibility that a substance from the pineal gland could be beneficial in treating skin diseases, they extracted and identified melatonin from bovine pineal gland extracts.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Melatonin#History): Melatonin's discovery is linked to the study of color changes in some amphibians and reptiles, a phenomenon initially observed through the administration of pineal gland extracts. In 1917, Carey Pratt McCord and Floyd P. Allen found that feeding extracts from the pineal glands of cows caused the skin of tadpoles to lighten by contracting the dark epidermal melanophores.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Melatonin#History): The hormone melatonin was isolated in 1958 by Aaron B. Lerner, a dermatology professor, and his team at Yale University. Motivated by the possibility that a substance from the pineal gland could be beneficial in treating skin diseases, they extracted and identified melatonin from bovine pineal gland extracts. Subsequent research in the mid-1970s by Lynch and others demonstrated that melatonin production follows a circadian rhythm in human pineal glands.

</details>

**Where you meet it** · retried · 11.9 s

> **Produced in pineal gland and plants**  
> In mammals, melatonin is produced by the pineal gland. Melatonin has been identified in many plants including feverfew (Tanacetum parthenium), St John's wort (Hypericum perforatum), rice, corn, tomato and other edible fruits(3).

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/896#section=Record-Description): Melatonin is a biogenic amine that is found in animals, plants and microbes. Aaron B. Lerner of Yale University is credited for naming the hormone and for defining its chemical structure in 1958. In mammals, melatonin is produced by the pineal gland. The pineal gland is small endocrine gland, about the size of a rice grain and shaped like a pine cone (hence the name), that is located in the center of the brain (rostro-dorsal to the superior colliculus) but outside the blood-brain barrier. The secretion of melatonin increases in darkness and decreases during exposure to light, thereby regulating the circadian rhythms of several biological functions, including the sleep-wake cycle.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/896#section=Natural-Pollution-Sources): Melatonin is secreted from the pineal gland and retina during dark periods of the vertebrate circadian rhythm and is formed from serotonin(1). Melatonin, a hormone of the pineal gland, is also produced by extrapineal tissues that lighten skin color in amphibians(2). Melatonin has been identified in many plants including feverfew (Tanacetum parthenium), St John's wort (Hypericum perforatum), rice, corn, tomato and other edible fruits(3).

</details>

**Weird fact** · untitled · 13.2 s

> ****  
> As detailed in their publication in the Journal of the American Chemical Society, Lerner and his colleagues proposed the name melatonin, derived from the Greek words melas, meaning 'black' or 'dark', and tonos, meaning 'labour', 'colour' or 'suppress'.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Melatonin#History): The etymology of melatonin stems from its skin-lightening properties. As detailed in their publication in the Journal of the American Chemical Society, Lerner and his colleagues proposed the name melatonin, derived from the Greek words melas, meaning 'black' or 'dark', and tonos, meaning 'labour', 'colour' or 'suppress'. This naming convention follows that of serotonin, another agent affecting skin color, discovered in 1948 as a modulator of vascular tone, which influenced its name based on its serum vasoconstrictor effect.

</details>


### menthol (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Menthol carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 1.6 s

> **First isolated in 1771**  
> Menthol was first isolated in 1771, by the German, Hieronymus David Gaubius.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Menthol#History): Menthol was first isolated in 1771, by the German, Hieronymus David Gaubius. Early characterizations were done by Oppenheim, Beckett, Moriya, and Atkinson. It was named by F. L. Alphonse Oppenheim (1833–1877) in 1861.

</details>

**Where you meet it** · written · 4.6 s

> **In peppermint and cornmint oils**  
> Menthol occurs naturally in peppermint oil (along with a little menthone, the ester menthyl acetate and other compounds), obtained from Mentha × piperita (peppermint). L-Menthol is the main component (other stereoisomers may be present) of peppermint oils and cornmint oils obtained from Mentha piperita and Mentha arvensis(1). /l-Menthol/

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Menthol#Occurrence): Menthol occurs naturally in peppermint oil (along with a little menthone, the ester menthyl acetate and other compounds), obtained from Mentha × piperita (peppermint). Japanese menthol also contains a small percentage of the 1-epimer neomenthol.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/1254#section=Natural-Pollution-Sources): L-Menthol is the main component (other stereoisomers may be present) of peppermint oils and cornmint oils obtained from Mentha piperita and Mentha arvensis(1). /l-Menthol/

</details>

**Weird fact** · untitled · 14.2 s

> ****  
> Menthol's ability to chemically trigger the cold-sensitive TRPM8 receptors in the skin is responsible for the well-known cooling sensation it provokes when inhaled, eaten, or applied to the skin. In this sense, it is similar to capsaicin, the chemical responsible for the spiciness of hot chilis (which stimulates heat sensors, also without causing an actual change in temperature).

<details><summary>Sources</summary>

- [Wikipedia, Biological properties](https://en.wikipedia.org/wiki/Menthol#Biological_properties): Menthol's ability to chemically trigger the cold-sensitive TRPM8 receptors in the skin is responsible for the well-known cooling sensation it provokes when inhaled, eaten, or applied to the skin. In this sense, it is similar to capsaicin, the chemical responsible for the spiciness of hot chilis (which stimulates heat sensors, also without causing an actual change in temperature).

</details>


### morphine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Morphine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 5 more hazards, not listed here.

**Origin story** · written · 3.3 s

> **Discovered in 1804 from opium poppy**  
> Morphine was discovered as the first active alkaloid extracted from the opium poppy plant in December 1804 in Paderborn by German pharmacist Friedrich Sertürner.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Morphine#History): Morphine was discovered as the first active alkaloid extracted from the opium poppy plant in December 1804 in Paderborn by German pharmacist Friedrich Sertürner. Sertürner noticed the compound induced drowsiness in rats and stray dogs and he reported his findings in 1805. In 1817, Sertürner reported experiments in which he administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died. Sertürner originally named the substance morphium after the Greek god of dreams, Morpheus, as it has a tendency to cause sleep. Sertürner's morphium was six times stronger than opium.

</details>

**Weird fact** · written · 5.9 s

> **All four almost died**  
> In 1817, Sertürner reported experiments in which he administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Morphine#History): Morphine was discovered as the first active alkaloid extracted from the opium poppy plant in December 1804 in Paderborn by German pharmacist Friedrich Sertürner. Sertürner noticed the compound induced drowsiness in rats and stray dogs and he reported his findings in 1805. In 1817, Sertürner reported experiments in which he administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died. Sertürner originally named the substance morphium after the Greek god of dreams, Morpheus, as it has a tendency to cause sleep. Sertürner's morphium was six times stronger than opium.

</details>

**Where you meet it** · retried · 13.4 s

> **Found in opium poppy sap**  
> Morphine occurs naturally in poppies(1), specifically in sap from the opium poppy (Papaver somniferum)(2). Morphine is obtained from opium, which itself is obtained from the milky exudate of incised unripe seed capsules of the poppy plant, Papaver somniferum, indigenous to Asia Minor.

<details><summary>Sources</summary>

- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/5288826#section=Natural-Pollution-Sources): Morphine occurs naturally in poppies(1), specifically in sap from the opium poppy (Papaver somniferum)(2).
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/5288826#section=Natural-Pollution-Sources): Morphine is obtained from opium, which itself is obtained from the milky exudate of incised unripe seed capsules of the poppy plant, Papaver somniferum, indigenous to Asia Minor. The alkaloids of opium include phenanthrenes (morphine, codeine, thebaine) and benzyl isoquinolines (papaverine, noscapine).

</details>


### nicotine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Nicotine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 2.3 s

> **Isolated from tobacco plant**  
> Nicotine was originally isolated from the tobacco plant in 1828 by chemists Wilhelm Heinrich Posselt and Karl Ludwig Reimann from Germany, who believed it was a poison.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Nicotine#History): Nicotine was originally isolated from the tobacco plant in 1828 by chemists Wilhelm Heinrich Posselt and Karl Ludwig Reimann from Germany, who believed it was a poison. Its chemical empirical formula was described by Melsens in 1843, its structure was discovered by Adolf Pinner and Richard Wolffenstein in 1893, and it was first synthesized by Amé Pictet and A. Rotschy in 1904.

</details>

**Where you meet it** · written · 5.0 s

> **Found in tobacco and other plants**  
> In tobacco leaves, nicotine constitutes about 0.6–3% of the dry weight, and smaller, trace quantities occur in other Solanaceae crops such as tomatoes, potatoes, and eggplants. Nicotine is an alkaloid contained in the leaves of the tobacco plants(1) Nicotiana tabacum and N. rustica(2).

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Nicotine): Nicotine is a stimulant and alkaloid found primarily in plants of the nightshade family, notably in tobacco; it is also synthesized. Nicotine is used recreationally for its stimulant and anxiolytic effects. In tobacco leaves, nicotine constitutes about 0.6–3% of the dry weight, and smaller, trace quantities occur in other Solanaceae crops such as tomatoes, potatoes, and eggplants. In pure form, nicotine is a colorless to yellowish, oily liquid that readily penetrates biological membranes and acts as a potent neurotoxin in insects, where it serves as an antiherbivore toxin.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/89594#section=Natural-Pollution-Sources): Nicotine is an alkaloid contained in the leaves of the tobacco plants(1) Nicotiana tabacum and N. rustica(2).

</details>

**Weird fact** · written · 4.8 s

> **Named after a French ambassador**  
> Nicotine is named after the tobacco plant Nicotiana tabacum, which in turn is named after the French ambassador in Portugal, Jean Nicot de Villemain, who sent tobacco and seeds to Paris in 1560, presented to the French King, and who promoted their medicinal use.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Nicotine#History): Nicotine is named after the tobacco plant Nicotiana tabacum, which in turn is named after the French ambassador in Portugal, Jean Nicot de Villemain, who sent tobacco and seeds to Paris in 1560, presented to the French King, and who promoted their medicinal use. Smoking was believed to protect against illness, particularly the plague. However, the "holy herb", tobacco, had first reached Europe by the early 1530s, brought by Spanish explorers.

</details>


### nitroglycerin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Nitroglycerin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 2.1 s

> **Discovered in 1846 by Sobrero**  
> Nitroglycerin was the first practical explosive produced that was stronger than black powder. It was synthesized by the Italian chemist Ascanio Sobrero in 1846, working under Théophile-Jules Pelouze at the University of Turin. Sobrero initially called his discovery "pyroglycerin" and warned vigorously against its use as an explosive.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Nitroglycerin#History): Nitroglycerin was the first practical explosive produced that was stronger than black powder. It was synthesized by the Italian chemist Ascanio Sobrero in 1846, working under Théophile-Jules Pelouze at the University of Turin. Sobrero initially called his discovery "pyroglycerin" and warned vigorously against its use as an explosive.

</details>

**Where you meet it** · untitled · 11.9 s

> ****  
> Nitroglycerin is an oily liquid that explodes when subjected to heat, shock, or flame. Nitroglycerin belongs to a group of drugs called nitrates, which includes many other nitrates like isosorbide dinitrate (Isordil) and isosorbide mononitrate (Imdur, Ismo, Monoket). In medicine, nitroglycerin is probably most commonly prescribed for angina pectoris, a painful symptom of ischemic heart disease caused by inadequate flow of blood and oxygen to the heart, and as a potent antihypertensive agent.

<details><summary>Sources</summary>

- [Wikipedia, Use as an explosive and a propellant](https://en.wikipedia.org/wiki/Nitroglycerin#Use_as_an_explosive_and_a_propellant): Nitroglycerin is an oily liquid that explodes when subjected to heat, shock, or flame. The main use of nitroglycerin, by tonnage, is in explosives such as dynamite and in propellants as an ingredient. However, its sensitivity has limited the usefulness of nitroglycerin as a military explosive; less sensitive explosives such as TNT, RDX, and HMX have largely replaced it in munitions.
- [Wikipedia, Medical use](https://en.wikipedia.org/wiki/Nitroglycerin#Medical_use): Nitroglycerin belongs to a group of drugs called nitrates, which includes many other nitrates like isosorbide dinitrate (Isordil) and isosorbide mononitrate (Imdur, Ismo, Monoket). These agents all exert their effect by being converted to nitric oxide in the body by mitochondrial aldehyde dehydrogenase (ALDH2), and nitric oxide is a potent natural vasodilator.
- [Wikipedia, Medical use](https://en.wikipedia.org/wiki/Nitroglycerin#Medical_use): In medicine, nitroglycerin is probably most commonly prescribed for angina pectoris, a painful symptom of ischemic heart disease caused by inadequate flow of blood and oxygen to the heart, and as a potent antihypertensive agent. Nitroglycerin corrects the imbalance between the flow of oxygen and blood to the heart and the heart's energy demand. There are many formulations on the market at different doses. At low doses, nitroglycerin dilates veins more than arteries, thereby reducing preload (volume of blood in the heart after filling); this is thought to be its primary mechanism of action.

</details>

**Weird fact** · written · 6.8 s

> **Frozen nitroglycerin must be thawed before use**  
> Early in its history, liquid nitroglycerin was found to be "desensitized" by freezing it at a temperature below 45 to 55 °F (7 to 13 °C), depending on its purity. Frozen nitroglycerin is much less energetic than liquid, and so must be thawed before use. Thawing it out can be extremely sensitizing, especially if impurities are present or the warming is too rapid.

<details><summary>Sources</summary>

- [Wikipedia, Instability and desensitization](https://en.wikipedia.org/wiki/Nitroglycerin#Instability_and_desensitization): Early in its history, liquid nitroglycerin was found to be "desensitized" by freezing it at a temperature below 45 to 55 °F (7 to 13 °C), depending on its purity. Its sensitivity to shock while frozen is somewhat unpredictable: "It is more insensitive to the shock from a fulminate cap or a rifle ball when in that condition but on the other hand it appears to be more liable to explode on breaking, crushing, tamping, etc." Frozen nitroglycerin is much less energetic than liquid, and so must be thawed before use. Thawing it out can be extremely sensitizing, especially if impurities are present or the warming is too rapid.

</details>


### oxytocin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Oxytocin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 3.0 s

> **Discovered in 1906 by Dale**  
> The uterine-contracting properties of the principle that would later be named oxytocin were discovered by British pharmacologist Henry Hallett Dale in 1906, and its milk ejection property was described by Ott and Scott in 1910 and by Schafer and Mackenzie in 1911.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Oxytocin#History): The uterine-contracting properties of the principle that would later be named oxytocin were discovered by British pharmacologist Henry Hallett Dale in 1906, and its milk ejection property was described by Ott and Scott in 1910 and by Schafer and Mackenzie in 1911. In 1909 the first clinical use of oxytocin was performed by William Blair-Bell to induce childbirth in patients with complications.

</details>

**Where you meet it** · written · 4.0 s

> **Present in animals and human behavior**  
> Oxytocin is a peptide hormone and neuropeptide normally produced in the hypothalamus and released by the posterior pituitary. Present in animals since early stages of evolution, in humans it plays roles in behavior that include social bonding, love, reproduction, childbirth, and the period after childbirth.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Oxytocin): Oxytocin is a peptide hormone and neuropeptide normally produced in the hypothalamus and released by the posterior pituitary. Present in animals since early stages of evolution, in humans it plays roles in behavior that include social bonding, love, reproduction, childbirth, and the period after childbirth. Oxytocin is released into the bloodstream as a hormone in response to sexual activity and during childbirth. It is also available in pharmaceutical form. In either form, oxytocin stimulates uterine contractions to speed up the process of childbirth.

</details>

**Weird fact** · untitled · 13.3 s

> ****  
> While the structure of oxytocin is highly conserved in placental mammals, a novel structure of oxytocin was reported in 2011 in marmosets, tamarins, and other New World primates. Genomic sequencing of the gene for oxytocin revealed a single in-frame mutation (thymine for cytosine) which results in a single amino acid substitution at the 8-position (proline for leucine).

<details><summary>Sources</summary>

- [Wikipedia, Chemistry](https://en.wikipedia.org/wiki/Oxytocin#Chemistry): While the structure of oxytocin is highly conserved in placental mammals, a novel structure of oxytocin was reported in 2011 in marmosets, tamarins, and other New World primates. Genomic sequencing of the gene for oxytocin revealed a single in-frame mutation (thymine for cytosine) which results in a single amino acid substitution at the 8-position (proline for leucine). Since this original Lee et al. paper, two other laboratories have confirmed Pro8-OT and documented additional oxytocin structural variants in this primate taxon. Vargas-Pinilla et al.

</details>


### quinine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Quinine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 3 more hazards, not listed here.

**Origin story** · untitled · 5.6 s

> ****  
> The Jesuit Agostino Salumbrino (1564–1642), an apothecary by training who lived in Lima (now in present-day Peru), observed the Quechua using the bark of the cinchona tree to treat such shivering. At the first opportunity, Salumbrino sent a small quantity to Rome for testing as a malaria treatment.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Quinine#History): The Jesuit Agostino Salumbrino (1564–1642), an apothecary by training who lived in Lima (now in present-day Peru), observed the Quechua using the bark of the cinchona tree to treat such shivering. While its effect in treating malaria (and malaria-induced shivering) was unrelated to its effect in controlling shivering from rigors, it was a successful medicine against malaria. At the first opportunity, Salumbrino sent a small quantity to Rome for testing as a malaria treatment. In the years that followed, cinchona bark, known as Jesuit's bark or Peruvian bark, became one of the most valuable commodities shipped from Peru to Europe.

</details>

**Where you meet it** · written · 6.8 s

> **Bark of cinchona tree and tonic water**  
> An alkaloid derived from the bark of the cinchona tree. It is used as an antimalarial drug, and is the active ingredient in extracts of the cinchona that have been used for that purpose since before 1633. Used as an antipyretic (fever-reducing), antimalarial, analgesic (painkilling), and anti-inflammatory. It is also used as a flavor component of tonic water and bitter lemon.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/3034034#section=Record-Description): An alkaloid derived from the bark of the cinchona tree. It is used as an antimalarial drug, and is the active ingredient in extracts of the cinchona that have been used for that purpose since before 1633. Quinine is also a mild antipyretic and analgesic and has been used in common cold preparations for that purpose. It was used commonly and as a bitter and flavoring agent, and is still useful for the treatment of babesiosis. Quinine is also useful in some muscular disorders, especially nocturnal leg cramps and myotonia congenita, because of its direct effects on muscle membrane and sodium channels. The mechanisms of its antimalarial effects are not well understood.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/3034034#section=Uses): For the treatment of malaria and leg cramps. Used as an antipyretic (fever-reducing), antimalarial, analgesic (painkilling), and anti-inflammatory. It is also used as a flavor component of tonic water and bitter lemon.

</details>

**Weird fact** · written · 5.9 s

> **Quinine can cause life-threatening blood reactions**  
> Because of the narrow difference between its therapeutic and toxic effects, quinine is a common cause of drug-induced disorders, including thrombocytopenia and thrombotic microangiopathy. Quinine can cause unpredictable serious and life-threatening blood and cardiovascular reactions including low platelet count and hemolytic–uremic syndrome/thrombotic thrombocytopenic purpura (HUS/TTP), long QT syndrome and other serious cardiac arrhythmias including torsades de pointes, blackwater fever, disseminated intravascular coagulation, leukopenia, and neutropenia.

<details><summary>Sources</summary>

- [Wikipedia, Contraindications](https://en.wikipedia.org/wiki/Quinine#Contraindications): Because of the narrow difference between its therapeutic and toxic effects, quinine is a common cause of drug-induced disorders, including thrombocytopenia and thrombotic microangiopathy. Even from minor levels occurring in common beverages, quinine can have severe adverse effects involving multiple organ systems, among which are immune system effects and fever, hypotension, hemolytic anemia, acute kidney injury, liver toxicity, and blindness. In people with atrial fibrillation, conduction defects, or heart block, quinine can cause heart arrhythmias, and should be avoided.
- [Wikipedia, Adverse effects](https://en.wikipedia.org/wiki/Quinine#Adverse_effects): Quinine can cause unpredictable serious and life-threatening blood and cardiovascular reactions including low platelet count and hemolytic–uremic syndrome/thrombotic thrombocytopenic purpura (HUS/TTP), long QT syndrome and other serious cardiac arrhythmias including torsades de pointes, blackwater fever, disseminated intravascular coagulation, leukopenia, and neutropenia. Some people who have developed TTP due to quinine have gone on to develop kidney failure.

</details>


### serotonin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Serotonin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 1.9 s

> **Discovered in blood serum**  
> In 1948, Maurice M. Rapport, Arda Green, and Irvine Page of the Cleveland Clinic discovered a vasoconstrictor substance in blood serum, and since it was a serum agent affecting vascular tone, they named it serotonin. In 1952, enteramine was shown to be the same substance as serotonin, and as the broad range of physiological roles was elucidated, the abbreviation 5-HT of the proper chemical name 5-hydroxytryptamine became the preferred name in the pharmacological field.

<details><summary>Sources</summary>

- [Wikipedia, History & etymology](https://en.wikipedia.org/wiki/Serotonin#History_&_etymology): It had been known to physiologists for over a century that a vasoconstrictor material appears in serum when blood was allowed to clot. In 1935, Italian Vittorio Erspamer, working in Pavia, showed an extract from enterochromaffin cells made intestines contract. Some believed it contained adrenaline, but two years later, Erspamer was able to show it was a previously unknown amine, which he named "enteramine". In 1948, Maurice M. Rapport, Arda Green, and Irvine Page of the Cleveland Clinic discovered a vasoconstrictor substance in blood serum, and since it was a serum agent affecting vascular tone, they named it serotonin.
- [Wikipedia, History & etymology](https://en.wikipedia.org/wiki/Serotonin#History_&_etymology): In 1952, enteramine was shown to be the same substance as serotonin, and as the broad range of physiological roles was elucidated, the abbreviation 5-HT of the proper chemical name 5-hydroxytryptamine became the preferred name in the pharmacological field. Synonyms of serotonin include: 5-hydroxytriptamine, enteramine, substance DS, and 3-(β-aminoethyl)-5-hydroxyindole. In 1953, Betty Twarog and Page discovered serotonin in the central nervous system.

</details>

**Where you meet it** · untitled · 7.1 s

> ****  
> Most of the body's serotonin—about 90%—is synthesized in the gastrointestinal tract by enterochromaffin cells, where it regulates intestinal movements. It is also produced in smaller amounts in the brainstem's raphe nuclei, the skin's Merkel cells, pulmonary neuroendocrine cells, and taste receptor cells of the tongue.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Serotonin): Most of the body's serotonin—about 90%—is synthesized in the gastrointestinal tract by enterochromaffin cells, where it regulates intestinal movements. It is also produced in smaller amounts in the brainstem's raphe nuclei, the skin's Merkel cells, pulmonary neuroendocrine cells, and taste receptor cells of the tongue. Once secreted, serotonin is taken up by platelets in the blood, which release it during clotting to promote vasoconstriction and platelet aggregation. Around 8% of the body's serotonin is stored in platelets, and 1–2% is found in the CNS.

</details>

**Weird fact** · untitled · 12.4 s

> ****  
> SSRIs have been found to be toxic to algae.

<details><summary>Sources</summary>

- [Wikipedia, Comparative biology & evolution](https://en.wikipedia.org/wiki/Serotonin#Comparative_biology_&_evolution): Serotonin is used by a variety of single-cell organisms for various purposes. SSRIs have been found to be toxic to algae. The gastrointestinal parasite Entamoeba histolytica secretes serotonin, causing a sustained secretory diarrhea in some people. Patients infected with E. histolytica have been found to have highly elevated serum serotonin levels, which returned to normal following resolution of the infection. E. histolytica also responds to the presence of serotonin by becoming more virulent.

</details>


### sodium chloride (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Sodium Chloride carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 4.5 s

> **Found in seawater and food**  
> Sodium chloride is the primary salt in seawater and in the extracellular fluid of many multicellular organisms. In its edible form, common table salt, it is commonly used as a condiment and food preservative.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5234#section=Record-Description): Sodium chloride, also known as salt, common salt, table salt or halite, is an ionic compound with the chemical formula NaCl, representing a 1:1 ratio of sodium and chloride ions. Sodium chloride is the primary salt in seawater and in the extracellular fluid of many multicellular organisms. It is listed on the World Health Organization Model List of Essential Medicines.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Sodium_chloride): Sodium chloride , is an ionic compound with the chemical formula NaCl, representing a 1:1 ratio of sodium and chloride ions. It is transparent or translucent, brittle, hygroscopic, and occurs as the mineral halite. In its edible form, common table salt, it is commonly used as a condiment and food preservative. Large quantities of sodium chloride are used in many industrial processes, and it is a major source of sodium and chlorine compounds used as feedstocks for further chemical syntheses. Another major application of sodium chloride is de-icing of roadways in sub-freezing weather.

</details>

**Weird fact** · written · 6.2 s

> **Each ion surrounded by six of opposite charge**  
> In solid sodium chloride, each ion is surrounded by six ions of the opposite charge as expected on electrostatic grounds. The surrounding ions are located at the vertices of a regular octahedron.

<details><summary>Sources</summary>

- [Wikipedia, Chemistry](https://en.wikipedia.org/wiki/Sodium_chloride#Chemistry): In solid sodium chloride, each ion is surrounded by six ions of the opposite charge as expected on electrostatic grounds. The surrounding ions are located at the vertices of a regular octahedron. In the language of close-packing, the larger chloride ions (167 pm in size) are arranged in a cubic array whereas the smaller sodium ions (116 pm) fill all the cubic gaps (octahedral voids) between them. This same basic structure is found in many other compounds and is commonly known as the NaCl structure or rock salt crystal structure. It can be represented as a face-centered cubic (fcc) lattice with a two-atom basis or as two interpenetrating face centered cubic lattices.

</details>


### strychnine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Strychnine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 3.2 s

> **First discovered in 1818**  
> Strychnine was first discovered by French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier in 1818 in the Saint-Ignatius' bean.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Strychnine#History): Strychnine was first discovered by French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier in 1818 in the Saint-Ignatius' bean. In some Strychnos plants a 9,10-dimethoxy derivative of strychnine, the alkaloid brucine, is also present. Brucine is not as poisonous as strychnine. Historic records indicate that preparations containing strychnine (presumably) had been used to kill dogs, cats, and birds in Europe as far back as 1640. It was allegedly used by convicted murderer William Palmer to kill his final victim, John Cook. It was also used during World War II by Oskar Dirlewanger against civilians.

</details>

**Where you meet it** · written · 4.4 s

> **Seeds of Strychnos nux-vomica tree**  
> Strychnine is a plant toxin most commonly found in the seeds of the Strychnine tree (Strychnos nux-vomica L.). The most common source is from the seeds of the Strychnos nux-vomica tree.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/441071#section=Record-Description): Strychnine is a plant toxin most commonly found in the seeds of the Strychnine tree (Strychnos nux-vomica L.). It is used as a pesticide, particularly for killing small vertebrates such as birds and rodents. Strychnine causes muscular convulsions and eventually death through asphyxia or sheer exhaustion. (L1230)
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Strychnine): Strychnine is a highly toxic, colorless, bitter, crystalline alkaloid used as a pesticide, particularly for killing small vertebrates such as birds and rodents. Strychnine, when inhaled, swallowed, or absorbed through the eyes or mouth, causes poisoning which results in muscular convulsions and eventually death through asphyxia. While it is no longer used medicinally, it was used historically in small doses to strengthen muscle contractions, such as a heart and bowel stimulant and performance-enhancing drug. The most common source is from the seeds of the Strychnos nux-vomica tree.

</details>

**Weird fact** · written · 5.7 s

> **Used to kill dogs, cats, and birds**  
> Historic records indicate that preparations containing strychnine (presumably) had been used to kill dogs, cats, and birds in Europe as far back as 1640. It was allegedly used by convicted murderer William Palmer to kill his final victim, John Cook.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Strychnine#History): Strychnine was first discovered by French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier in 1818 in the Saint-Ignatius' bean. In some Strychnos plants a 9,10-dimethoxy derivative of strychnine, the alkaloid brucine, is also present. Brucine is not as poisonous as strychnine. Historic records indicate that preparations containing strychnine (presumably) had been used to kill dogs, cats, and birds in Europe as far back as 1640. It was allegedly used by convicted murderer William Palmer to kill his final victim, John Cook. It was also used during World War II by Oskar Dirlewanger against civilians.

</details>


### sucralose (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Sucralose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · written · 2.4 s

> **Discovered by scientists at Tate & Lyle**  
> Sucralose was discovered in 1975 by scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College (now part of King's College London). While researching novel uses of sucrose and its synthetic derivatives, mixing sulfuryl chloride with sugar, Phadnis was told to "test" a chlorinated sugar compound. According to an anecdotal account, Phadnis thought Hough asked him to "taste" it, so he did and found the compound to be exceptionally sweet.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Sucralose#History): Sucralose was discovered in 1975 by scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College (now part of King's College London). While researching novel uses of sucrose and its synthetic derivatives, mixing sulfuryl chloride with sugar, Phadnis was told to "test" a chlorinated sugar compound. According to an anecdotal account, Phadnis thought Hough asked him to "taste" it, so he did and found the compound to be exceptionally sweet. Tate & Lyle patented the substance in 1976; as of 2008, the only remaining patents concerned specific manufacturing processes.

</details>

**Where you meet it** · written · 3.3 s

> **Used in soft drinks and candy**  
> Sucralose is used as a replacement for (or in combination with) other artificial or natural sweeteners such as aspartame, acesulfame potassium or high-fructose corn syrup. It is used in products such as candy, breakfast bars, coffee pods, and soft drinks. It is also used in canned fruits wherein water and sucralose take the place of much higher-energy corn syrup-based additives.

<details><summary>Sources</summary>

- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Sucralose#Uses): Sucralose is used as a replacement for (or in combination with) other artificial or natural sweeteners such as aspartame, acesulfame potassium or high-fructose corn syrup. It is used in products such as candy, breakfast bars, coffee pods, and soft drinks. It is also used in canned fruits wherein water and sucralose take the place of much higher-energy corn syrup-based additives. Sucralose mixed with dextrose (glucose) or maltodextrin (both made from corn) as bulking agents is sold internationally by McNeil Nutritionals under the Splenda brand name.

</details>

**Weird fact** · untitled · 13.3 s

> ****  
> While researching novel uses of sucrose and its synthetic derivatives, mixing sulfuryl chloride with sugar, Phadnis was told to "test" a chlorinated sugar compound. According to an anecdotal account, Phadnis thought Hough asked him to "taste" it, so he did and found the compound to be exceptionally sweet.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Sucralose#History): Sucralose was discovered in 1975 by scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College (now part of King's College London). While researching novel uses of sucrose and its synthetic derivatives, mixing sulfuryl chloride with sugar, Phadnis was told to "test" a chlorinated sugar compound. According to an anecdotal account, Phadnis thought Hough asked him to "taste" it, so he did and found the compound to be exceptionally sweet. Tate & Lyle patented the substance in 1976; as of 2008, the only remaining patents concerned specific manufacturing processes.

</details>


### sucrose (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Sucrose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · skipped · 1.2 s

**Where you meet it** · retried · 13.0 s

> **Found in fruits and nectars**  
> In nature, sucrose is present in many plants, and in particular their roots, fruits and nectars, because it serves as a way to store energy, primarily from photosynthesis. Many mammals, birds, insects and bacteria accumulate and feed on the sucrose in plants and for some it is their main food source.

<details><summary>Sources</summary>

- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Sucrose#Sources): In nature, sucrose is present in many plants, and in particular their roots, fruits and nectars, because it serves as a way to store energy, primarily from photosynthesis. Many mammals, birds, insects and bacteria accumulate and feed on the sucrose in plants and for some it is their main food source. Although honeybees consume sucrose, the honey they produce consists primarily of fructose and glucose, with only trace amounts of sucrose.

</details>

**Weird fact** · untitled · 15.4 s

> ****  
> Unlike most disaccharides, the glycosidic bond in sucrose is formed between the reducing ends of both glucose and fructose, and not between the reducing end of one and the non-reducing end of the other.

<details><summary>Sources</summary>

- [Wikipedia, Physical and chemical properties](https://en.wikipedia.org/wiki/Sucrose#Physical_and_chemical_properties): Sucrose's IUPAC name is β-D-fructofuranosyl-(2→1)-α-D-glucopyranoside. In this disaccharide, glucose and fructose are linked via a glycosidic linkage, i.e. an ether bond between C1 on the glucosyl subunit and C2 on the fructosyl unit. Glucose exists predominantly as a mixture of α and β "pyranose" anomers, but sucrose has only the α form. Fructose exists as a mixture of five tautomers but sucrose has only the β-D-fructofuranose form. Unlike most disaccharides, the glycosidic bond in sucrose is formed between the reducing ends of both glucose and fructose, and not between the reducing end of one and the non-reducing end of the other.

</details>


### testosterone (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Testosterone carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 5 more hazards, not listed here.

**Origin story** · written · 2.2 s

> **Self-injected rejuvenating elixir**  
> Research on the action of testosterone received a brief boost in 1889, when the Harvard professor Charles-Édouard Brown-Séquard (1817–1894), then in Paris, self-injected subcutaneously a "rejuvenating elixir" consisting of an extract of dog and guinea pig testicle. He reported in The Lancet that his vigor and feeling of well-being were markedly restored but the effects were transient, and Brown-Séquard's hopes for the compound were dashed.

<details><summary>Sources</summary>

- [Wikipedia, History and production](https://en.wikipedia.org/wiki/Testosterone#History_and_production): A testicular action was linked to circulating blood fractions – now understood to be a family of androgenic hormones – in the early work on castration and testicular transplantation in fowl by Arnold Adolph Berthold (1803–1861). Research on the action of testosterone received a brief boost in 1889, when the Harvard professor Charles-Édouard Brown-Séquard (1817–1894), then in Paris, self-injected subcutaneously a "rejuvenating elixir" consisting of an extract of dog and guinea pig testicle. He reported in The Lancet that his vigor and feeling of well-being were markedly restored but the effects were transient, and Brown-Séquard's hopes for the compound were dashed.

</details>

**Where you meet it** · untitled · 7.8 s

> ****  
> testosterone has been reported in Locusta migratoria, Botrytis cinerea, and other organisms with data available. In humans and most other vertebrates, testosterone is secreted primarily by the testicles of males and, to a lesser extent, the ovaries of females.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/6013#section=Record-Description): testosterone has been reported in Locusta migratoria, Botrytis cinerea, and other organisms with data available.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Testosterone): Testosterone is a steroid hormone from the androstane class containing a ketone and a hydroxyl group at positions three and seventeen respectively. It is biosynthesized in several steps from cholesterol and is converted in the liver to inactive metabolites. It exerts its action through binding to and activation of the androgen receptor. In humans and most other vertebrates, testosterone is secreted primarily by the testicles of males and, to a lesser extent, the ovaries of females. On average, in adult males, levels of testosterone are about seven to eight times as great as in adult females.

</details>

**Weird fact** · written · 5.6 s

> **Agnathans use androstenedione instead**  
> Agnathans (jawless vertebrates) such as lampreys do not produce testosterone but instead use androstenedione as a male sex hormone.

<details><summary>Sources</summary>

- [Wikipedia, Other species](https://en.wikipedia.org/wiki/Testosterone#Other_species): Testosterone is observed in most vertebrates. Testosterone and the classical nuclear androgen receptor first appeared in gnathostomes (jawed vertebrates). Agnathans (jawless vertebrates) such as lampreys do not produce testosterone but instead use androstenedione as a male sex hormone. Fish make a slightly different form called 11-ketotestosterone. Its counterpart in insects is ecdysone. The presence of these ubiquitous steroids in a wide range of animals suggest that sex hormones have an ancient evolutionary history.

</details>


### tetrodotoxin (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Tetrodotoxin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · untitled · 11.8 s

> ****  
> Although tetrodotoxin was discovered in these fish, it is found in several other animals (e.g., in blue-ringed octopuses, rough-skinned newts, and moon snails). Tetrodotoxin is an extremely potent poison (toxin) found mainly in the liver and sex organs (gonads) of some fish, such as puffer fish, globefish, and toadfish (order Tetraodontiformes) and in some amphibian, octopus, and shellfish species.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Tetrodotoxin): Tetrodotoxin (TTX) is a potent neurotoxin. Its name derives from Tetraodontiformes, an order that includes pufferfish, porcupinefish, ocean sunfish, and triggerfish; several of these species carry the toxin. Although tetrodotoxin was discovered in these fish, it is found in several other animals (e.g., in blue-ringed octopuses, rough-skinned newts, and moon snails). It is also produced by certain infectious or symbiotic bacteria like Pseudoalteromonas, Pseudomonas, and Vibrio as well as other species found in symbiotic relationships with animals and plants.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/11174599#section=Uses): Tetrodotoxin is an extremely potent poison (toxin) found mainly in the liver and sex organs (gonads) of some fish, such as puffer fish, globefish, and toadfish (order Tetraodontiformes) and in some amphibian, octopus, and shellfish species. Human poisonings occur when the flesh and/or organs of the fish are improperly prepared and eaten. Tetrodotoxin interferes with the transmission of signals from nerves to muscles and causes an increasing paralysis of the muscles of the body. Tetrodotoxin poisoning can be fatal.

</details>

**Weird fact** · dropped · 12.2 s

Problems: the chosen sentences have 93 words; keep it under 90


### thalidomide (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Thalidomide carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 2.3 s

> **Used in treating multiple myeloma**  
> Used to treat multiple myeloma.

<details><summary>Sources</summary>

- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/5426#section=Uses): For the acute treatment of the cutaneous manifestations of moderate to severe erythema nodosum leprosum (ENL). Also for use as maintenance therapy for prevention and suppression of the cutaneous manifestations of ENL recurrence. Used to treat multiple myeloma.

</details>

**Weird fact** · written · 3.5 s

> **Withdrawn due to teratogenic effects**  
> A piperidinyl isoindole originally introduced as a non-barbiturate hypnotic, thalidomide was withdrawn from the market due to teratogenic effects. It has been reintroduced and used for a number of inflammatory disorders and cancers.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5426#section=Record-Description): A piperidinyl isoindole originally introduced as a non-barbiturate hypnotic, thalidomide was withdrawn from the market due to teratogenic effects. It has been reintroduced and used for a number of inflammatory disorders and cancers. Thalidomide displays immunosuppressive and anti-angiogenic activity through modulating the release of inflammatory mediators like tumor necrosis factor-alpha (TNF-a) and other cytokine action. Due to severe teratogenicity, pregnancy must be excluded before the start of treatment and patients must enrol in the THALIDOMID Risk Evaluation and Mitigation Strategy (REMS) program to ensure contraception adherence.

</details>


### theobromine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Theobromine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 2 more hazards, not listed here.

**Origin story** · written · 1.8 s

> **Discovered in cacao beans**  
> Theobromine was discovered in 1841 in cacao beans by the chemist A. Woskresensky.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Theobromine#History): Theobromine was discovered in 1841 in cacao beans by the chemist A. Woskresensky. Synthesis of theobromine from xanthine was first reported in 1882 by Hermann Emil Fischer.

</details>

**Where you meet it** · written · 5.1 s

> **Found in chocolate and tea**  
> In industry, theobromine is used as an additive and precursor to some cosmetics. It is found in chocolate and several other foods, including tea (Camellia sinensis), some American hollies (yaupon and guayusa) and the kola nut. Theobromine is the primary alkaloid found in cocoa and chocolate.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Theobromine): Theobromine, also known as xantheose, is the principal alkaloid of Theobroma cacao (cacao plant). Theobromine is slightly water-soluble (330 mg/L) with a bitter taste. In industry, theobromine is used as an additive and precursor to some cosmetics. It is found in chocolate and several other foods, including tea (Camellia sinensis), some American hollies (yaupon and guayusa) and the kola nut. It is a white or colourless solid, but commercial samples can appear yellowish.
- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Theobromine#Sources): Theobromine is the primary alkaloid found in cocoa and chocolate. Cocoa butter only contains trace amounts of theobromine. There are usually higher concentrations in dark than in milk chocolate.

</details>

**Weird fact** · written · 7.9 s

> **Contains no bromine despite the name**  
> Despite its name, the compound contains no bromine, which is based on Greek bromos ("stench").

<details><summary>Sources</summary>

- [Wikipedia, Etymology](https://en.wikipedia.org/wiki/Theobromine#Etymology): Despite its name, the compound contains no bromine, which is based on Greek bromos ("stench").

</details>


### trinitrotoluene (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> 2,4,6-Trinitrotoluene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 2.5 s

> **First synthesized in 1863**  
> TNT was first synthesized in 1863 by German chemist Julius Wilbrand and was originally used as a yellow dye. Its potential as an explosive was not recognized for three decades, mainly because it was so much less sensitive than other explosives known at the time. Its explosive properties were discovered in 1891 by another German chemist, Carl Häussermann.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/TNT#History): TNT was first synthesized in 1863 by German chemist Julius Wilbrand and was originally used as a yellow dye. Its potential as an explosive was not recognized for three decades, mainly because it was so much less sensitive than other explosives known at the time. Its explosive properties were discovered in 1891 by another German chemist, Carl Häussermann. TNT can be safely poured when liquid into shell cases, and is so insensitive that in 1910 it was exempted from the UK's Explosives Act 1875 and was not considered an explosive for the purposes of manufacture and storage.

</details>

**Where you meet it** · written · 4.8 s

> **Used in military and industrial blasts**  
> TNT is one of the most commonly used explosives for military, industrial, and mining applications. 2,4,6-Trinitrotoluene is an explosive used in military shells, bombs, and grenades, in industrial uses, and in underwater blasting.

<details><summary>Sources</summary>

- [Wikipedia, Applications](https://en.wikipedia.org/wiki/TNT#Applications): TNT is one of the most commonly used explosives for military, industrial, and mining applications. TNT has been used in oil and gas fracking (hydraulic fracturing) in shale formations. The technique involves displacing and detonating nitroglycerin in hydraulically induced fractures followed by wellbore shots using pelletized TNT.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/8376#section=Uses): 2,4,6-Trinitrotoluene is an explosive used in military shells, bombs, and grenades, in industrial uses, and in underwater blasting. Exposure may results from drinking contaminated water that has migrated from chemical waste disposal sites, breathing contaminated air, eating contaminated foods such as fruits and vegetables, and/or eating contaminated soil. (L132)

</details>

**Weird fact** · written · 6.4 s

> **Skin turns bright yellow-orange**  
> TNT is poisonous, and skin contact can cause skin irritation, causing the skin to turn a bright yellow-orange color. During the First World War, female munition workers who handled the chemical found that their skin turned bright yellow, which resulted in their acquiring the nickname "canary girls" or simply "canaries".

<details><summary>Sources</summary>

- [Wikipedia, Safety and toxicity](https://en.wikipedia.org/wiki/TNT#Safety_and_toxicity): TNT is poisonous, and skin contact can cause skin irritation, causing the skin to turn a bright yellow-orange color. During the First World War, female munition workers who handled the chemical found that their skin turned bright yellow, which resulted in their acquiring the nickname "canary girls" or simply "canaries".

</details>


### vanillin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Vanillin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · written · 2.2 s

> **Native peoples discovered vanilla**  
> Vanilla beans, called tlilxochitl, were discovered and cultivated as a flavoring for beverages by native Mesoamerican peoples, most famously the Totonacs of modern-day Veracruz, Mexico. Since at least the early 15th century, the Aztecs used vanilla as a flavoring for chocolate in drinks called xocohotl.

<details><summary>Sources</summary>

- [Wikipedia, Natural history](https://en.wikipedia.org/wiki/Vanillin#Natural_history): Vanilla beans, called tlilxochitl, were discovered and cultivated as a flavoring for beverages by native Mesoamerican peoples, most famously the Totonacs of modern-day Veracruz, Mexico. Since at least the early 15th century, the Aztecs used vanilla as a flavoring for chocolate in drinks called xocohotl.

</details>

**Where you meet it** · dropped · 10.3 s

Problems: s12.1 starts with "It" but the sentence before it was not chosen

**Weird fact** · untitled · 12.8 s

> ****  
> Scolytus multistriatus, one of the vectors of the Dutch elm disease, uses vanillin as a signal to find a host tree during oviposition.

<details><summary>Sources</summary>

- [Wikipedia, Ecology](https://en.wikipedia.org/wiki/Vanillin#Ecology): Scolytus multistriatus, one of the vectors of the Dutch elm disease, uses vanillin as a signal to find a host tree during oviposition.

</details>


### water (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Water is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · skipped · 1.3 s

**Where you meet it** · written · 5.0 s

> **Streams, lakes, and oceans**  
> Water (chemical formula: H2O) is a transparent fluid which forms the world's streams, lakes, oceans and rain, and is the major constituent of the fluids of organisms. Water is an inorganic compound with the chemical formula H2O. It is a transparent, tasteless, odorless, and nearly colorless chemical substance. It is the main constituent of Earth's streams, lakes, and oceans.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/962#section=Record-Description): Water (chemical formula: H2O) is a transparent fluid which forms the world's streams, lakes, oceans and rain, and is the major constituent of the fluids of organisms. As a chemical compound, a water molecule contains one oxygen and two hydrogen atoms that are connected by covalent bonds. Water is a liquid at standard ambient temperature and pressure, but it often co-exists on Earth with its solid state, ice; and gaseous state, steam (water vapor).
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Water): Water is an inorganic compound with the chemical formula H2O. It is a transparent, tasteless, odorless, and nearly colorless chemical substance. It is the main constituent of Earth's streams, lakes, and oceans. Water is also the fluid of all known living organisms, in which it acts as a solvent. Water, being a polar molecule, undergoes strong intermolecular hydrogen bonding which is a large contributor to its physical and chemical properties. It is vital for all known forms of life, despite not providing food energy or being an organic micronutrient.

</details>

**Weird fact** · written · 6.3 s

> **Exists as solid, liquid, and gas**  
> Water is the only common substance to exist as a solid, liquid, and gas in normal terrestrial conditions.

<details><summary>Sources</summary>

- [Wikipedia, Properties](https://en.wikipedia.org/wiki/Water#Properties): Water (H2O) is a polar inorganic compound. At room temperature it is a tasteless and odorless liquid, nearly colorless with a hint of blue. The simplest hydrogen chalcogenide, it is by far the most studied chemical compound and is sometimes described as the "universal solvent" for its ability to dissolve more substances than any other liquid, though it is poor at dissolving nonpolar substances. This allows it to be the "solvent of life": indeed, water as found in nature almost always includes various dissolved substances, and special steps are required to obtain chemically pure water. Water is the only common substance to exist as a solid, liquid, and gas in normal terrestrial conditions.

</details>

