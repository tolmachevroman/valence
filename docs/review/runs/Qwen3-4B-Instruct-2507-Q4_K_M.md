# Review run: Qwen3-4B-Instruct-2507-Q4_K_M

50 molecules from `fixtures/review`, cards written as the app writes them. Times start when a molecule's snippets are in hand; fetching adds 1–3 s uncached. The Danger card is built from data and appears at once, so first-card times count model cards.

## Against the PRD targets

| Measure | Target | Result |
| --- | --- | --- |
| First model card | under 10 s | 28 of 48 molecules; 9.6 s median, 22.5 s p90, 42.2 s max |
| Streaming starts | under 3 s | 35 of 50; 2.5 s median, 6.8 s p90, 8.0 s max |
| Molecules with 3+ cards | 90% | 39 of 50 (78%) |
| Cards with a claim not in the sources | 0 | by review, below |
| Model load | under 10 s | 9.0 s |
| Model card, including pick, check and retry | | 10.3 s median, 15.5 s p90, 24.0 s max |

## Outcomes

| Card | written | trimmed | retried | skipped | dropped | no input | error |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Where you meet it | 27 | 7 | 12 | 0 | 4 | 0 | 0 |
| Origin story | 16 | 10 | 1 | 3 | 7 | 0 | 0 |
| Weird fact | 26 | 5 | 5 | 3 | 11 | 0 | 0 |

## Cards

Each claim should be stated by the cited snippets, folded under the card.

### DDT (1 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> 1-Chloro-4-(2,2,2-trichloro-1-(4-chlorophenyl)ethyl)benzene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · dropped · 11.7 s

Problems: not stated in the cited snippets: "Where you meet it: old pesticides"; not stated in the cited snippets: "This molecule, a chlorophenylethane, was formerly used as an insecticide and pesticide."; not stated in the cited snippets: "All agricultural uses were cancelled by the EPA in 1972."

**Weird fact** · skipped · 4.4 s


### acetaminophen (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Acetaminophen carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 3 more hazards, not listed here.

**Origin story** · dropped · 15.8 s

Problems: not stated in the cited snippets: "First found in 1886 with toxic side effects"; not stated in the cited snippets: "This synthesis was not immediately recognized for its safety or efficacy."

**Weird fact** · dropped · 14.4 s

Problems: not stated in the cited snippets: "Paracetamol synthesized in 1852?"; not stated in the cited snippets: "This is unusual because acetanilide was the first aniline derivative found to have analgesic properties, and paracetamol was later developed as a safer alternative."; not stated in the cited snippets: "The claim about 1852 synthesis is noted in some reports but is not confirmed."

**Where you meet it** · written · 12.0 s

> **In your medicine cabinet**  
> Acetaminophen, known as Tylenol or paracetamol, is found in over 600 over-the-counter cold, allergy, sleep, and pain medications. It's also available as a standalone tablet, syrup, or injection. Commonly used during pregnancy, it's recommended by the WHO, FDA, EMA, and MHRA for short-term fever and pain.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/1983#section=Record-Description): Acetaminophen (paracetamol), also commonly known as Tylenol, is the most commonly taken analgesic worldwide and is recommended as first-line therapy in pain conditions by the World Health Organization (WHO). It is also used for its antipyretic effects, helping to reduce fever. This drug was initially approved by the U.S. FDA in 1951 and is available in a variety of forms including syrup form, regular tablets, effervescent tablets, injection, suppository, and other forms. Acetaminophen is often found combined with other drugs in more than 600 over the counter (OTC) allergy medications, cold medications, sleep medications, pain relievers, and other products.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Paracetamol): Paracetamol, or acetaminophen, is an analgesic and antipyretic agent used to treat fever and mild to moderate pain. It is a widely available over-the-counter drug sold generically or under various brand names, including Tylenol and Panadol.
- [Wikipedia, Use in pregnancy and breastfeeding](https://en.wikipedia.org/wiki/Paracetamol#Use_in_pregnancy_and_breastfeeding): Paracetamol has long been established as a safe medication to treat short-term fever and significant pain in pregnant patients. Regulatory agencies, including the European Medicines Agency (EMA), the Medicines and Healthcare products Regulatory Agency (MHRA), the World Health Organization (WHO), and the U.S. Food and Drug Administration (FDA) recommend paracetamol as a first-line analgesic and antipyretic in pregnancy, with use limited to situations where necessary, at the lowest effective dose, and for the shortest duration.

</details>


### allicin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Allicin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · trimmed · 9.3 s

> **Discovered in Japan in the 1940s**  
> In 1944, Chester J. Cavallito and John Hays Bailey first isolated and studied allicin in the laboratory. Allicin became a model for developing other thiamine disulfides such as sulbutiamine, fursultiamine, and benfothiamine.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Allicin#History): It was first isolated and studied in the laboratory by Chester J. Cavallito and John Hays Bailey in 1944.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Allicin#History): Allicin was discovered as part of efforts to create thiamine derivatives in the 1940s, mainly in Japan. Allicin became a model for medicinal chemistry efforts to create other thiamine disulfides. The results included sulbutiamine, fursultiamine (thiamine tetrahydrofurfuryl disulfide) and benfothiamine. These compounds are hydrophobic, easily pass from the intestines to the bloodstream, and are reduced to thiamine by cysteine or glutathione.

</details>

**Where you meet it** · written · 6.9 s

> **In the smell of crushed garlic**  
> Allicin forms when fresh garlic or leeks are chopped or crushed, triggering a reaction that produces its sharp aroma. You can find it in the oily, slightly yellow liquid that gives garlic its distinctive smell. It also acts as a natural defense against pests in garlic plants.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/65036#section=Record-Description): antibacterial principle of Allium sativum (garlic); structure; TH - CIAF
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Allicin): Allicin is an organosulfur compound obtained from garlic and leeks. When fresh garlic is chopped or crushed, the enzyme alliinase converts alliin into allicin, which is responsible for the aroma of fresh garlic. Allicin is unstable and quickly changes into a series of other sulfur-containing compounds such as diallyl disulfide. Allicin is an antifeedant, i.e. the defense mechanism against attacks by pests on the garlic plant.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Allicin): Allicin is an oily, slightly yellow liquid that gives garlic its distinctive odor. It is a thioester of a sulfinic acid. It is also known as allyl thiosulfinate. Its biological activity can be attributed to both its antioxidant activity and its reaction with thiol-containing proteins.

</details>

**Weird fact** · trimmed · 7.6 s

> **Allicin led to thiamine derivatives**  
> Allicin was discovered in the 1940s in Japan during thiamine derivative research. It became a model for creating other thiamine disulfides like sulbutiamine, fursultiamine, and benfothiamine.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Allicin#History): Allicin was discovered as part of efforts to create thiamine derivatives in the 1940s, mainly in Japan. Allicin became a model for medicinal chemistry efforts to create other thiamine disulfides. The results included sulbutiamine, fursultiamine (thiamine tetrahydrofurfuryl disulfide) and benfothiamine. These compounds are hydrophobic, easily pass from the intestines to the bloodstream, and are reduced to thiamine by cysteine or glutathione.

</details>


### artemisinin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Artemisinin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 7.6 s

> **Discovery in 1972 from sweet wormwood**  
> In 1972, Tu Youyou discovered artemisinin in the leaves of Artemisia annua during research under Project 523, a secret military program ordered by Mao Zedong at the request of North Vietnamese leaders. The program aimed to find an effective malaria treatment. Tu's team isolated an extract from the plant, which later proved to be a key starting point for purified artemisinin.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Artemisinin#History): Artemisinin is an antimalarial lactone derived from Artemisia annua or sweet wormwood. In 1596, Li Shizhen recommended tea made from 'sweet wormwood specifically to treat malaria symptoms in his Compendium of Materia Medica. The genus name is derived from the Greek goddess Artemis and, more specifically, may have been named after Queen Artemisia II of Caria, a botanist and medical researcher in the fourth century BC.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Artemisinin#History): Artemisia annua is a common herb found in many parts of the world. In 1967, a plant screening research program, under a secret military program code-named "Project 523", was set up by the People's Liberation Army to find an adequate treatment for malaria; the program and early clinical work were ordered by Mao Zedong at the request of North Vietnamese leaders to provide assistance for their malaria-ridden army. In the course of this research in 1972, Tu Youyou discovered artemisinin in the leaves of Artemisia annua.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Artemisinin#History): Tu's team subsequently isolated an extract. Results were published in the Chinese Medical Journal in 1979. The extracted substance, once subject to purification, proved to be a useful starting point to obtain purified artemisinin. A 2012 review reported that artemisinin-based therapies were the most effective drugs for treatment of malaria at that time; it was also reported to clear malaria parasites from patients' bodies faster than other drugs. In addition to artemisinin, Project 523 developed a number of products that can be used in combination with artemisinin, including lumefantrine, piperaquine, and pyronaridine.

</details>

**Where you meet it** · written · 11.0 s

> **In traditional herbs and malaria treatment**  
> Artemisinin is found in Artemisia annua, known as sweet wormwood, used in traditional Chinese medicine. It appears in malaria treatments worldwide, especially as artesunate or artemether for severe or uncomplicated cases. The molecule is also noted in studies for use against cancer, viral, and bacterial infections.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Artemisinin): Artemisinin and its semisynthetic derivatives are a group of drugs used in the treatment of malaria due to Plasmodium falciparum. It was discovered in 1972 by Tu Youyou, who shared the 2015 Nobel Prize in Physiology or Medicine for her discovery. Artemisinin-based combination therapies (ACTs) have become standard treatment worldwide for P. falciparum malaria as well as malaria due to other species of Plasmodium. Artemisinin can be extracted from the herb Artemisia annua (sweet wormwood), which is used in traditional Chinese medicine.
- [Wikipedia, Medical use](https://en.wikipedia.org/wiki/Artemisinin#Medical_use): For severe malaria, the WHO recommends intravenous or intramuscular treatment with the artemisinin derivative artesunate for at least 24 hours. Artesunate treatment is continued until the treated person is well enough to take oral medication. They are then given a three-day course of an ACT, for uncomplicated malaria. Where artesunate is not available, the WHO recommends intramuscular injection of the less potent artemisinin derivative artemether. For children less than six years old, if injected artesunate is not available the WHO recommends rectal administration of artesunate, followed by referral to a facility with the resources for further care.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/68827#section=Record-Description): A 2023 review notes that Artemisinin can be isolated from Artemisia annua, also known as sweet wormwood, sweet annie, sweet sagewort, and annual wormwood (Chinese: qngho). It has been used to treat various conditions, e.g. malaria, cancer, viral and bacterial infections, and some cardiovasculreviear diseases.

</details>

**Weird fact** · retried · 15.1 s

> **Named after a queen who studied plants**  
> The genus name Artemisia is derived from the Greek goddess Artemis. It may have been named after Queen Artemisia II of Caria, who was a botanist and medical researcher in the fourth century BC.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Artemisinin#History): Artemisinin is an antimalarial lactone derived from Artemisia annua or sweet wormwood. In 1596, Li Shizhen recommended tea made from 'sweet wormwood specifically to treat malaria symptoms in his Compendium of Materia Medica. The genus name is derived from the Greek goddess Artemis and, more specifically, may have been named after Queen Artemisia II of Caria, a botanist and medical researcher in the fourth century BC.

</details>


### ascorbic acid (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> L-Ascorbic Acid is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · trimmed · 6.8 s

> **From scurvy prevention to synthesis**  
> Tadeus Reichstein synthesized it in bulk using the Reichstein process, enabling mass production. In 1934, Hoffmann–La Roche acquired the patent, trademarked it as Redoxon, and began marketing synthetic vitamin C as a dietary supplement.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Vitamin_C#History): Vitamin C was discovered in 1912, isolated in 1928 and synthesized in 1933, making it the first vitamin to be synthesized. Shortly thereafter Tadeus Reichstein succeeded in synthesizing the vitamin in bulk by what is now called the Reichstein process. This made possible the inexpensive mass-production of vitamin C. In 1934, Hoffmann–La Roche bought the Reichstein process patent, trademarked synthetic vitamin C under the brand name Redoxon, and began to market it as a dietary supplement.

</details>

**Weird fact** · written · 8.7 s

> **Humans lack the enzyme to make vitamin C**  
> Some mammals can produce vitamin C, but humans lack the enzyme l-gulonolactone oxidase. This means humans must get vitamin C from their diet. Irwin Stone proposed that humans have a mutated gene for this enzyme.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Vitamin_C#History): In 1957, J. J. Burns showed that some mammals are susceptible to scurvy as their liver does not produce the enzyme l-gulonolactone oxidase, the last of the chain of four enzymes that synthesize vitamin C. American biochemist Irwin Stone was the first to exploit vitamin C for its food preservative properties. He later developed the idea that humans possess a mutated form of the l-gulonolactone oxidase coding gene.

</details>

**Where you meet it** · written · 11.3 s

> **In your fruit and bread**  
> L-Ascorbic Acid is found naturally in citrus fruits and many vegetables. It's added to canned fruits to slow oxidation and browning, and may be used as a flour treatment in breadmaking. As a food additive, it carries E numbers under approval by the European Food Safety Authority.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/54670067#section=Record-Description): A six carbon compound related to glucose. It is found naturally in citrus fruits and many vegetables. Ascorbic acid is an essential nutrient in human diets, and necessary to maintain connective tissue and bone. Its biologically active form, vitamin C, functions as a reducing agent and coenzyme in several metabolic pathways. Vitamin C is considered an antioxidant.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/54670067#section=Record-Description): Ascorbic Acid is a natural water-soluble vitamin (Vitamin C). Ascorbic acid is a potent reducing and antioxidant agent that functions in fighting bacterial infections, in detoxifying reactions, and in the formation of collagen in fibrous tissue, teeth, bones, connective tissue, skin, and capillaries. Found in citrus and other fruits, and in vegetables, vitamin C cannot be produced or stored by humans and must be obtained in the diet. (NCI04)
- [Wikipedia, Food preservation additive](https://en.wikipedia.org/wiki/Vitamin_C#Food_preservation_additive): Ascorbic acid and some of its salts and esters are common additives added to foods such as canned fruits, mostly to slow oxidation and enzymatic browning. It may be used as a flour treatment agent used in breadmaking. As food additives, they are assigned E numbers, with safety assessment and approval the responsibility of the European Food Safety Authority. The relevant E numbers are:

</details>


### aspirin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Aspirin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 3 more hazards, not listed here.

**Origin story** · trimmed · 6.9 s

> **First pure form made at Bayer**  
> In 1897, Felix Hoffmann (or Arthur Eichengrün) of Bayer first produced acetylsalicylic acid in a pure, stable form. This was part of Bayer's investigation to find a less-irritating alternative to salicylate medicines.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Aspirin#History): In 1853, the chemist Charles Frédéric Gerhardt treated sodium salicylate with acetyl chloride to produce acetylsalicylic acid for the first time; in the second half of the 19th century, other academic chemists established the compound's chemical structure and devised more efficient methods of synthesis. In 1897, scientists at the drug and dye firm Bayer began investigating acetylsalicylic acid as a less-irritating replacement for standard common salicylate medicines, and identified a new way to synthesize it. That year, Felix Hoffmann (or Arthur Eichengrün) of Bayer was the first to produce acetylsalicylic acid in a pure, stable form.

</details>

**Where you meet it** · retried · 14.4 s

> **In pain relief and heart care**  
> Aspirin, known as acetylsalicylic acid (ASA), is used to treat pain, fever, and inflammation. It is also used long term to reduce the chance of heart attack, ischaemic stroke, and blood clots in high-risk people. Aspirin works similarly to other NSAIDs and suppresses platelet function.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/2244#section=Record-Description): Also known as Aspirin, acetylsalicylic acid (ASA) is a commonly used drug for the treatment of pain and fever due to various causes. Acetylsalicylic acid has both anti-inflammatory and antipyretic effects. This drug also inhibits platelet aggregation and is used in the prevention of blood clots stroke, and myocardial infarction (MI). Interestingly, the results of various studies have demonstrated that long-term use of acetylsalicylic acid may decrease the risk of various cancers, including colorectal, esophageal, breast, lung, prostate, liver and skin cancer.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Aspirin): Aspirin is the genericized trademark for acetylsalicylic acid (ASA), a nonsteroidal anti-inflammatory drug (NSAID) used to reduce pain, fever, and inflammation, and as an antithrombotic. Aspirin is used to treat inflammatory conditions including Kawasaki disease, pericarditis, and rheumatic fever.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Aspirin): Aspirin is also used long term to reduce the chance of heart attack, ischaemic stroke, and blood clots occurring in people considered to be at high risk for such events. For pain or fever, effects typically begin within 30 minutes. Aspirin works similarly to other NSAIDs but also suppresses the normal functioning of platelets.

</details>

**Weird fact** · retried · 12.4 s

> **Aspirin powder can be explosive**  
> Aspirin powder mixed with air can form an explosive mixture. Factories must control airborne powder levels to prevent explosions. NIOSH set a recommended exposure limit of 5 mg/m3 in the United States.

<details><summary>Sources</summary>

- [Wikipedia, Chemical properties](https://en.wikipedia.org/wiki/Aspirin#Chemical_properties): Like flour mills, factories producing aspirin tablets must control the amount of the powder that becomes airborne inside the building, because the powder-air mixture can be explosive. The National Institute for Occupational Safety and Health (NIOSH) has set a recommended exposure limit in the United States of 5 mg/m3 (time-weighted average). OSHA decision in 1993.

</details>


### batrachotoxin (3 cards)

**Origin story** · written · 6.4 s

> **Discovery of batrachotoxin in frogs**  
> In 1963, Fritz Märki and Bernhard Witkop isolated the toxic alkaloid fraction from the black-legged poison dart frog at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. They identified four major toxic steroidal alkaloids, including batrachotoxin, homobatrachotoxin, pseudobatrachotoxin, and batrachotoxinin A. The structure determination faced significant difficulties due to the toxin's potency and scarcity.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Batrachotoxin#History): Batrachotoxin was discovered by Fritz Märki and Bernhard Witkop, at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. Märki and Witkop isolated the toxic alkaloids fraction from the black-legged poison dart frog and determined its chemical properties in 1963. They isolated four major toxic steroidal alkaloids including batrachotoxin, homobatrachotoxin (isobatrachotoxin), pseudobatrachotoxin, and batrachotoxinin A. Due to the difficulty of handling such a potent toxin and the minuscule amount that could be collected, a comprehensive structure determination involved several difficulties.

</details>

**Where you meet it** · written · 11.9 s

> **In frogs, birds, and beetles**  
> Batrachotoxin is found in the skin of frogs from the genus Phyllobates, in birds of the genera Pitohui and Ifrita, and in beetles of the genus Choresine. Indigenous groups like the Noanamá Chocó and Emberá Chocó use it to poison blowgun darts. The toxin binds to sodium channels, causing paralysis and death.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/6324647#section=Record-Description): Amphibians have the ability to produce and secrete a mixture of bioactive substances in specialized skin glands for antibiotic self-protection and defense against predators. Some of these secretions contain the highly toxic batrachotoxin, tetrodotoxin, and samandarine. Batrachotoxin (BTX) is a steroidal alkaloid that binds to voltage-gated sodium channels and makes them unable to produce action potentials. It is found in frogs of the genus Phyllobates, birds in the genera Pitohui and Ifrita, and beetles in the genus Choresine. Batrachotoxin was discovered by the precolonial inhabitants of the Northwestern Andes (present day Colombia).
- [Wikipedia, Use](https://en.wikipedia.org/wiki/Batrachotoxin#Use): The most common use of this toxin is by the Noanamá Chocó and Emberá Chocó of the Embera-Wounaan of western Colombia for poisoning blowgun darts for use in hunting.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Batrachotoxin): Batrachotoxin (BTX) is an extremely potent cardiotoxic and neurotoxic steroidal alkaloid found in certain species of beetles, birds, and frogs. The name is from the Greek word βάτραχος, bátrachos, 'frog'. Structurally-related chemical compounds are often referred to collectively as batrachotoxins. In certain frogs, this alkaloid is present mostly on the skin. Such frogs are among those used for poisoning darts. Batrachotoxin binds to and irreversibly opens the sodium channels of nerve cells and prevents them from closing, resulting in paralysis and death. No antidote is known.

</details>

**Weird fact** · written · 10.1 s

> **Birds with deadly skin toxins**  
> Several bird species in New Guinea have batrachotoxin in their skin and feathers. The hooded pitohui (Pitohui dichrous) is the most toxic of these birds.

<details><summary>Sources</summary>

- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Batrachotoxin#Sources): Several species of bird endemic to New Guinea have the toxin in their skin and on their feathers: the blue-capped ifrit (Ifrita kowaldi), little shrikethrush (aka rufous shrike-thrush, Colluricincla megarhyncha), and the following pitohui species: the hooded pitohui (Pitohui dichrous, the most toxic of the birds), crested pitohui (Ornorectes cristatus), black pitohui (Melanorectes nigrescens), rusty pitohui (Pseudorectes ferrugineus), and the variable pitohui, which is now split into three species: the northern variable pitohui (Pitohui kirhocephalus), Raja Ampat pitohui (P. cerviniventris), and southern variable pitohui (P. uropygialis).

</details>


### benzene (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Benzene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · dropped · 15.5 s

Problems: name Europe is not in the cited snippets; body has 85 words, must be under 80

**Where you meet it** · retried · 19.3 s

> **In fuels, smoke, and products**  
> Benzene is found in gasoline, where concentrations can reach up to 30% in some countries, though the U.S. average is 1–3%. It is also present in cigarette smoke and crude oil. Benzene appears in products as a solvent for inks, paints, and rubber, and is used to make dyes, detergents, drugs, and plastics.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/241#section=Record-Description): Benzene is a colorless liquid with a sweet odor. It evaporates into the air very quickly and dissolves slightly in water. It is highly flammable and is formed from both natural processes and human activities. Benzene is widely used in the United States; it ranks in the top 20 chemicals for production volume. Some industries use benzene to make other chemicals which are used to make plastics, resins, and nylon and synthetic fibers. Benzene is also used to make some types of rubbers, lubricants, dyes, detergents, drugs, and pesticides. Natural sources of benzene include volcanoes and forest fires. Benzene is also a natural part of crude oil, gasoline, and cigarette smoke.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/241#section=Uses): Benzene is used as a constituent in motor fuels; as a solvent for fats, waxes, resins, oils, inks, paints, plastics, and rubber; in the extraction of oils from seeds and nuts; and in photogravure printing. It is also used as a chemical intermediate. Benzene is also used in the manufacture of detergents, explosives, pharmaceuticals, and dyestuffs.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/241#section=Uses): Benzene was used in the past as a solvent in inks, rubber, lacquers, and paint removers. Today, it is used mainly in closed processes to synthesize organic chemicals. Gasoline in some countries contains a high concentration of benzene (as high as 30%); the U.S. average is 1-3%. Workers who remove or clean underground storage tanks may be exposed to significant levels. [ACGIH] Gasoline in North America now contains about 1% benzene. [AIHAJ 2002;63(2):225-30] The European Union (EU) reduced in 2000 the maximum allowed benzene content in gasoline from 5% to 1% by volume. Mean exposures in the Swedish petroleum industry are well below the Swedish occupational exposure limits. [PMID 28578463]

</details>

**Weird fact** · retried · 15.2 s

> **Benzene's name comes from a Southeast Asian resin**  
> The word 'benzene' derives from 'gum benzoin', an aromatic resin known in Southeast Asia. European pharmacists and perfumers encountered it in the 16th century via trade routes. An acidic material from benzoin was called 'flowers of benzoin', leading to benzoic acid and eventually benzene.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Benzene#History): The word "benzene" derives from "gum benzoin" (benzoin resin), an aromatic resin known since ancient times in Southeast Asia, and later to European pharmacists and perfumers in the 16th century via trade routes. An acidic material was derived from benzoin by sublimation, and named "flowers of benzoin", or benzoic acid. The hydrocarbon derived from benzoic acid thus acquired the names benzin, benzol, and benzene. Michael Faraday first isolated and identified benzene in 1825 from the oily residue derived from the production of illuminating gas, giving it the name bicarburet of hydrogen.

</details>


### benzylpenicillin (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Penicillin G carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 2 more hazards, not listed here.

**Where you meet it** · written · 10.2 s

> **In injections for infections**  
> Penicillin G is found in injectable forms used to treat infections like pneumonia, syphilis, and strep throat. It is administered intravenously or intramuscularly because it is poorly absorbed when taken orally. Common forms include sodium, potassium, benzathine, or procaine salts.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5904#section=Record-Description): Penicillin G (potassium or sodium) is an antibacterial prescription medicine approved by the U.S. Food and Drug Administration (FDA) for the treatment of certain serious bacterialinfections, such as pneumonia, meningitis, gonorrhea, syphilis, and others. Pneumonia and syphilis can be opportunistic infections (OI) of HIV.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5904#section=Record-Description): Benzylpenicillin (Penicillin G) is narrow spectrum antibiotic used to treat infections caused by susceptible bacteria. It is a natural penicillin antibiotic that is administered intravenously or intramuscularly due to poor oral absorption. Penicillin G may also be used in some cases as prophylaxis against susceptible organisms. Natural penicillins are considered the drugs of choice for several infections caused by susceptible gram positive aerobic organisms, such as *Streptococcus pneumoniae*, groups A, B, C and G streptococci, nonenterococcal group D streptococci, viridans group streptococci, and non-penicillinase producing staphylococcus.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Benzylpenicillin): Benzylpenicillin, also known as penicillin G (PenG) or BENPEN, is an antibiotic used to treat a number of bacterial infections. This includes pneumonia, strep throat, syphilis, necrotizing enterocolitis, diphtheria, gas gangrene, leptospirosis, cellulitis, and tetanus. It is not a first-line agent for pneumococcal meningitis. Due to benzylpenicillin's limited bioavailability for oral medications, it is generally taken as an injection in the form of a sodium, potassium, benzathine, or procaine salt. Benzylpenicillin is given by injection into a vein or muscle. Two long-acting forms benzathine benzylpenicillin and procaine benzylpenicillin are available for use by injection into a muscle only.

</details>

**Weird fact** · skipped · 5.2 s


### buckminsterfullerene (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Fullerenes carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · written · 9.5 s

> **Laser vaporization and a cage-like structure**  
> In 1984, Eric Rohlfing, Donald Cox, and Andrew Kaldor first generated buckminsterfullerene using a laser to vaporize carbon in a supersonic helium beam, though they did not recognize the structure. In 1985, Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University repeated the experiment and identified C60 as a cage-like molecule, a regular truncated icosahedron. They named it after R. Buckminster Fuller due to its resemblance to his geodesic domes.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Buckminsterfullerene#History): Theoretical predictions of buckminsterfullerene molecules appeared in the late 1960s and early 1970s. It was first generated in 1984 by Eric Rohlfing, Donald Cox, and Andrew Kaldor, using a laser to vaporize carbon in a supersonic helium beam, although the group did not realize that buckminsterfullerene had been produced. In 1985 their work was repeated by Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University, who recognized the structure of C60 as buckminsterfullerene.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Buckminsterfullerene#History): Using laser evaporation of graphite the Smalley team found Cn clusters (where n > 20 and even) of which the most common were C60 and C70. A solid rotating graphite disk was used as the surface from which carbon was vaporized using a laser beam creating hot plasma that was then passed through a stream of high-density helium gas. The carbon species were subsequently cooled and ionized resulting in the formation of clusters. Clusters ranged in molecular masses, but Kroto and Smalley found predominance in a C60 cluster that could be enhanced further by allowing the plasma to react longer. They also discovered that C60 is a cage-like molecule, a regular truncated icosahedron.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Buckminsterfullerene#History): The discoverers of the allotrope named the newfound molecule after American architect R. Buckminster Fuller, who designed many geodesic dome structures that look similar to C60 and who had died in 1983, the year before discovery. Another common name for buckminsterfullerene is "buckyballs".

</details>

**Where you meet it** · written · 9.4 s

> **From soot to space**  
> Fullerenes like buckminsterfullerene appear in soot and are found in shungite, a rock with 99% carbon. They also exist in space, where neutral C60 is seen in planetary nebulae and stars, and C+60 causes diffuse interstellar bands in the near-infrared.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Buckminsterfullerene#Occurrence): Buckminsterfullerene is the most common naturally occurring fullerene. Small quantities of it can be found in soot.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/123591#section=Natural-Pollution-Sources): Fullerene was first discovered in shungite, a green-schist facies metamorphic rock containing 99% carbon(1).
- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Buckminsterfullerene#Occurrence): It also exists in space. Neutral C60 has been observed in planetary nebulae and several types of star. The ionised form, C+60, has been identified in the interstellar medium, where it is the cause of several absorption features known as diffuse interstellar bands in the near-infrared.

</details>

**Weird fact** · written · 9.8 s

> **Fullerenes in space, causing starlight absorption**  
> Neutral C60 has been observed in planetary nebulae and several types of star. The ionised form, C+60, has been identified in the interstellar medium, where it is the cause of several absorption features known as diffuse interstellar bands in the near-infrared.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Buckminsterfullerene#Occurrence): It also exists in space. Neutral C60 has been observed in planetary nebulae and several types of star. The ionised form, C+60, has been identified in the interstellar medium, where it is the cause of several absorption features known as diffuse interstellar bands in the near-infrared.

</details>


### caffeine (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Caffeine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · dropped · 13.8 s

Problems: not stated in the cited snippets: "First isolated from coffee in 1819"

**Weird fact** · trimmed · 10.0 s

> **Coffee's hidden discoverer**  
> Caffeine was discovered in coffee in 1821 by Mr. Robiquet. We do not know why Robiquet never published his analysis.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Caffeine#History): Caffeine, noun (feminine). Crystallizable substance discovered in coffee in 1821 by Mr. Robiquet. During the same period – while they were searching for quinine in coffee because coffee is considered by several doctors to be a medicine that reduces fevers and because coffee belongs to the same family as the cinchona [quinine] tree – on their part, Messrs. Pelletier and Caventou obtained caffeine; but because their research had a different goal and because their research had not been finished, they left priority on this subject to Mr. Robiquet. We do not know why Mr. Robiquet has not published the analysis of coffee which he read to the Pharmacy Society.

</details>

**Where you meet it** · trimmed · 18.1 s

> **In tea, coffee, and chocolate**  
> Caffeine occurs naturally in tea leaves, coffee beans, cocoa beans, mate leaves, guarana paste, and kola nuts. It is found in the seeds, fruits, nuts, or leaves of plants native to Africa, East Asia, and South America.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/2519#section=Record-Description): Caffeine is xanthine alkaloid that occurs naturally in seeds, leaves and fruit of several plants and trees that acts as a natural pesticide. Caffeine is a major component of coffee, tea and chocolate and in humans acts as a central nervous system (CNS) stimulant. Consumption of caffeine, even in high doses, has not been associated with elevations in serum enzyme elevations or instances of clinically apparent liver injury.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Caffeine): Caffeine is a bitter, white crystalline purine, a methylxanthine alkaloid, and is chemically related to the adenine and guanine bases of deoxyribonucleic acid (DNA) and ribonucleic acid (RNA). It is found in the seeds, fruits, nuts, or leaves of a number of plants native to Africa, East Asia, and South America and helps to protect them against herbivores and from competition by preventing the germination of nearby seeds, as well as encouraging consumption by select animals such as honey bees. The most common sources of caffeine for human consumption are the tea leaves of the Camellia sinensis plant and the coffee bean, the seed of the Coffea plant.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/2519#section=Uses): Occurs naturally in tea leaves, coffee beans, cocoa beans, mate leaves, guarana paste, and kola nuts; [Merck Index] Used in beverages, cardiac and respiratory stimulants, and diuretics; [HSDB] Also used in cosmetics; [eChemPortal: SIDSUNEP]

</details>


### capsaicin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Capsaicin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · retried · 9.7 s

> **First extracted and studied**  
> In 1816, Christian Friedrich Bucholz first extracted capsicin in impure form. In 1873, Rudolf Buchheim stated that 'capsicol' caused a burning feeling on mucous membranes. In 1878, Endre Hőgyes confirmed that 'capsicol' increased gastric acid secretion.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Capsaicin#History): The compound was first extracted in impure form in 1816 by Christian Friedrich Bucholz (1770–1818).
- [Wikipedia, History](https://en.wikipedia.org/wiki/Capsaicin#History): In 1873 German pharmacologist Rudolf Buchheim (1820–1879) and in 1878 the Hungarian doctor Endre Hőgyes stated that "capsicol" (partially purified capsaicin) caused the burning feeling when in contact with mucous membranes and increased secretion of gastric acid.

</details>

**Where you meet it** · written · 10.1 s

> **The heat in red peppers**  
> Capsaicin, the pungent principle in hot red peppers like Capsicum annuum and C. frutescens, gives them their spicy taste and burning sensation. It's found in chili powder and paprika, adding heat to food. The intensity of this heat is measured on the Scoville scale.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Capsaicin): Capsaicin (8-methyl-N-vanillyl-6-nonenamide) is a toxin that is the main active component of chili peppers and gives them their distinct pungent, "spicy" or "hot" taste. It is a potent irritant for mammals for which it produces a sensation of burning in any tissue with which it comes into contact. Capsaicin and several related amides (capsaicinoids) are produced as secondary metabolites by chili peppers, likely as deterrents against eating by mammals and against the growth of fungi. Pure capsaicin is a hydrophobic, highly pungent (i.e., spicy) crystalline solid.
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Capsaicin#Uses): Because of the burning sensation caused by capsaicin when it comes in contact with mucous membranes, it is commonly used in food products to provide added spiciness or "heat" (piquancy), usually in the form of spices such as chili powder and paprika. In high concentrations, capsaicin will also cause a burning effect on other sensitive areas, such as skin or eyes. The degree of heat found within a food is often measured on the Scoville scale.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/1548943#section=Natural-Pollution-Sources): Capsaicin is the pungent principle found in fruit of hot, red peppers such as Capsicum annuum and C. frutescens (Solanaceae)(1).

</details>

**Weird fact** · written · 9.5 s

> **Seeds don't make capsaicin**  
> Capsaicin is not produced in the seeds of Capsicum plants. Instead, the highest concentration is found in the white pith where seeds are attached, and the seeds themselves do not produce any capsaicin.

<details><summary>Sources</summary>

- [Wikipedia, Natural function](https://en.wikipedia.org/wiki/Capsaicin#Natural_function): Capsaicin is present in large quantities in the placental tissue (which holds the seeds), the internal membranes and, to a lesser extent, the other fleshy parts of the fruits of plants in the genus Capsicum. The seeds themselves do not produce any capsaicin, although the highest concentration of capsaicin can be found in the white pith of the inner wall, where the seeds are attached.

</details>


### carbon monoxide (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Carbon Monoxide carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 8.7 s

> **From fire to first synthesis**  
> Man likely first encountered carbon monoxide toxicity when introducing fire into dwellings circa 800,000 BC. Aristotle (384–322 BC) first recorded toxic fumes from burning coals, and Galen (AD 129–199) proposed a change in air composition caused harm. Joseph Priestley first synthesized carbon monoxide in 1772, followed by Carl Wilhelm Scheele in 1773 and Torbern Bergman in 1775.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Carbon_monoxide#History): Man has maintained a complex relationship with carbon monoxide since first learning to control fire circa 800,000 BC. Early man probably discovered the toxicity of carbon monoxide poisoning upon introducing fire into his dwellings. The early development of metallurgy and smelting technologies emerging circa 6,000 BC through the Bronze Age likewise plagued man from carbon monoxide exposure. Apart from the toxicity of carbon monoxide, Native Americans in the United States may have experienced the neuroactive properties of carbon monoxide through shamanistic fireside rituals.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Carbon_monoxide#History): Early civilizations developed mythological tales to explain the origin of fire, such as Prometheus from Greek mythology who shared fire with humans. Aristotle (384–322 BC) first recorded that burning coals produced toxic fumes. Greek physician Galen (AD 129–199) speculated that there was a change in the composition of the air that caused harm when inhaled, and many others of the era developed a basis of knowledge about carbon monoxide in the context of coal fume toxicity. Cleopatra may have died from carbon monoxide poisoning.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Carbon_monoxide#History): Joseph Priestley is considered to have first synthesized carbon monoxide in 1772. Carl Wilhelm Scheele similarly isolated carbon monoxide from charcoal in 1773 and thought it could be the carbonic entity making fumes toxic. Torbern Bergman isolated carbon monoxide from oxalic acid in 1775. In the presence of oxygen, including atmospheric concentrations, carbon monoxide burns with a blue flame, producing carbon dioxide. Antoine Lavoisier conducted similarly inconclusive experiments in 1777.

</details>

**Where you meet it** · retried · 20.0 s

> **In smoke, stoves, and fires**  
> Carbon monoxide may be emitted from tobacco smoke and malfunctioning fuel-burning stoves (wood, kerosene, natural gas, propane) and heating systems (wood, oil, natural gas). It is also generated from blocked flues connected to these appliances. Small amounts occur naturally in the atmosphere from volcanoes, forest and bushfires, and other combustion processes like burning biomass fuel for cooking and heating.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Carbon_monoxide): CO is one of the most acutely toxic contaminants affecting indoor air quality. CO may be emitted from tobacco smoke and generated from malfunctioning fuel-burning stoves (wood, kerosene, natural gas, propane) and fuel-burning heating systems (wood, oil, natural gas) and from blocked flues connected to these appliances. Carbon monoxide poisoning is the most common type of fatal air poisoning in many countries.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/281#section=Uses): Ceramics making [Category: Hobbies] Lost-wax casting [Category: Hobbies] Sculpturing plastics [Category: Hobbies] Glassblowing [Category: Hobbies] Smoking cigarettes [Category: Food & Drugs] Burning biomass fuel for cooking and heating [Category: Environments]
- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Carbon_monoxide#Occurrence): Carbon monoxide (CO) is present in small amounts (about 80 ppb) in the Earth's atmosphere. Most comes from chemical reactions with organic compounds emitted by human activities and natural origins due to photochemical reactions in the troposphere that generate about 5 × 1012 kilograms per year. Other natural sources of CO include volcanoes, forest and bushfires, and other miscellaneous forms of combustion such as fossil fuels. Small amounts are also emitted from the ocean, and from geological activity because carbon monoxide occurs dissolved in molten volcanic rock at high pressures in the Earth's mantle.

</details>

**Weird fact** · trimmed · 12.0 s

> **Stronger bond than nitrogen gas**  
> Carbon monoxide has a bond-dissociation energy of 1072 kJ/mol, stronger than nitrogen gas's 942 kJ/mol. The bond length of 112.8 pm is consistent with a triple bond, similar to nitrogen (109.76 pm).

<details><summary>Sources</summary>

- [Wikipedia, Physical and chemical properties](https://en.wikipedia.org/wiki/Carbon_monoxide#Physical_and_chemical_properties): The carbon and oxygen are connected by a triple bond that consists of a net two pi bonds and one sigma bond. The bond length between the carbon atom and the oxygen atom is 112.8 pm. This bond length is consistent with a triple bond, as in molecular nitrogen (N2), which has a similar bond length (109.76 pm) and nearly the same molecular mass. Carbon–oxygen double bonds are significantly longer, 120.8 pm in formaldehyde, for example. The boiling point (82 K) and melting point (68 K) are very similar to those of N2 (77 K and 63 K, respectively). The bond-dissociation energy of 1072 kJ/mol is stronger than that of N2 (942 kJ/mol) and represents the strongest chemical bond known.

</details>


### carvone (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Carvone, (+-)- carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · trimmed · 5.0 s

> **From caraway to carvone**  
> Goldschmidt and Zürrer identified it as a ketone related to limonene. The structure was finally elucidated by Georg Wagner in 1894.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Carvone#History): Caraway was used for medicinal purposes by the ancient Romans, but carvone was probably not isolated as a pure compound until Franz Varrentrapp (1815–1877) obtained it in 1849. It was originally called carvol by Schweizer. Goldschmidt and Zürrer identified it as a ketone related to limonene, and the structure was finally elucidated by Georg Wagner (1849–1903) in 1894.

</details>

**Where you meet it** · retried · 19.5 s

> **In caraway, spearmint, and orange peel**  
> Carvone is found naturally in caraway seeds (Carum carvi), where S-(+)-carvone makes up 60–70% of the oil. It also occurs in 40–60% of dill seed oil (Anethum graveolens) and in mandarin orange peel oil. R-(−)-carvone is the main component in spearmint oil (Mentha spicata), present in 50–80% of the oil.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Carvone): Carvone is a member of a family of chemicals called terpenoids. Carvone is found naturally in many essential oils, but is most abundant in the oils from seeds of caraway (Carum carvi), spearmint (Mentha spicata), and dill.
- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Carvone#Occurrence): S-(+)-Carvone is the principal constituent (60–70%) of the oil from caraway seeds (Carum carvi), which is produced on a scale of about 10 tonnes per year. It also occurs to the extent of about 40–60% in dill seed oil (from Anethum graveolens), and also in mandarin orange peel oil. R-(−)-Carvone is also the most abundant compound in the essential oil from several species of mint, particularly spearmint oil (Mentha spicata), which is composed of 50–80% R-(−)-carvone. Spearmint is a major source of naturally produced R-(−)-carvone. However, the majority of R-(−)-carvone used in commercial applications is synthesized from R-(+)-limonene. The R-(−)-carvone isomer also occurs in kuromoji oil.
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Carvone#Uses): Both carvones are used in the food and flavor industry. As the compound most responsible for the flavor of caraway, dill, and spearmint, carvone has been used for millennia in food. Food applications are mainly met by carvone made from limonene. R-(−)-Carvone is also used for air freshening products and, like many essential oils, oils containing carvones are used in aromatherapy and alternative medicine.

</details>

**Weird fact** · written · 9.1 s

> **Carvone stops potatoes from sprouting**  
> S-(+)-Carvone is used to prevent premature sprouting of potatoes during storage. It is marketed in the Netherlands under the name Talent.

<details><summary>Sources</summary>

- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Carvone#Uses): S-(+)-Carvone is also used to prevent premature sprouting of potatoes during storage, being marketed in the Netherlands for this purpose under the name Talent.

</details>


### cholesterol (2 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Cholesterol is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · skipped · 1.2 s

**Where you meet it** · written · 8.5 s

> **In meat, eggs, and milk**  
> Cholesterol is found in red meat, egg yolks, whole eggs, liver, kidney, fish oil, shellfish, butter, and human breast milk. It's also present in animal fats and oils, where it helps maintain membrane integrity and is essential for hormone synthesis.

<details><summary>Sources</summary>

- [Wikipedia, Dietary sources](https://en.wikipedia.org/wiki/Cholesterol#Dietary_sources): Animal fats are complex mixtures of triglycerides, with lesser amounts of both the phospholipids and cholesterol molecules from which all animal (and human) cell membranes are constructed. Since all animal cells manufacture cholesterol, all animal-based foods contain cholesterol in varying amounts. Major dietary sources of cholesterol include red meat, egg yolks and whole eggs, liver, kidney, giblets, fish oil, shellfish, and butter. Human breast milk also contains significant quantities of cholesterol.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Cholesterol): Cholesterol is the principal sterol of all animals, distributed in body tissues, especially the brain and spinal cord, and in animal fats and oils.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/5997#section=Uses): Found in many foods (meats, eggs, milk, cheese, fish, shellfish) derived from animal products. Essential for membrane integrity. Used in steroid hormone synthesis.

</details>

**Weird fact** · dropped · 11.0 s

Problems: not stated in the cited snippets: "Cholesterol changes color with temperature"


### cinnamaldehyde (1 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Cinnamaldehyde carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · dropped · 19.4 s

Problems: not stated in the cited snippets: "In cinnamon bark and snacks"

**Weird fact** · dropped · 13.2 s

Problems: title has 9 words, at most 8 allowed


### citric acid (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Citric Acid carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · dropped · 15.8 s

Problems: not stated in the cited snippets: "In your citrus and soda"

**Weird fact** · written · 9.8 s

> **Used in birth control and laxatives**  
> Citric acid is an ingredient in Phexxi, a non-hormonal contraceptive approved by the FDA on May 2020. It is also used with magnesium oxide to form magnesium citrate, an osmotic laxative.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/311#section=Record-Description): A key intermediate in metabolism. It is an acid compound found in citrus fruits. The salts of citric acid (citrates) can be used as anticoagulants due to their calcium-chelating ability. Citric acid is one of the active ingredients in Phexxi, a non-hormonal contraceptive agent that was approved by the FDA on May 2020. It is also used in combination with magnesium oxide to form magnesium citrate, an osmotic laxative.

</details>


### cubane (2 cards)

**Where you meet it** · retried · 10.2 s

> **In high-performance explosives**  
> Octanitrocubane and heptanitrocubane have been studied as high-performance explosives. These compounds have a very high density for hydrocarbon molecules and store a large amount of energy in a small space. Their geometry and stability make them suitable isosteres for benzene rings.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Cubane): Having high potential energy and kinetic stability makes cubane and its derivative compounds useful for controlled energy storage. For example, octanitrocubane and heptanitrocubane have been studied as high-performance explosives. These compounds also typically have a very high density for hydrocarbon molecules. The resulting high energy density means a large amount of energy can be stored in a comparably smaller amount of space, an important consideration for applications in fuel storage and energy transport. Furthermore, their geometry and stability make them suitable isosteres for benzene rings.

</details>

**Weird fact** · written · 4.7 s

> **Cubane refuses to break down**  
> Cubane is highly strained and cannot decompose because the resulting cubene molecules would be pyramidal alkenes, too high-energy for most elimination pathways. Certain metallic ions can catalyze σ-bond rearrangement to cuneane.

<details><summary>Sources</summary>

- [Wikipedia, Reactions and derivatives](https://en.wikipedia.org/wiki/Cubane#Reactions_and_derivatives): Cubane is highly strained, but cannot decompose: the resultant cubene molecules would be pyramidal alkenes, too high-energy for most elimination pathways. Certain metallic ions catalyze σ-bond rearrangement to cuneane:

</details>


### dopamine (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Dopamine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · dropped · 11.0 s

Problems: not stated in the cited snippets: "First synthesized and identified in the brain"

**Where you meet it** · written · 11.2 s

> **In medicines and your body**  
> Dopamine is found in Magnolia officinalis and Senegalia berlandieri. As a medication, it's sold under names like Intropin, Dopastat, and Revimine and is used to treat severe low blood pressure and cardiac arrest in newborns. It acts locally in blood vessels, kidneys, pancreas, digestive system, and immune system, where it regulates functions like blood flow and gut protection.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/681#section=Record-Description): dopamine has been reported in Magnolia officinalis, Senegalia berlandieri, and other organisms with data available.
- [Wikipedia, Medical uses](https://en.wikipedia.org/wiki/Dopamine#Medical_uses): Dopamine as a manufactured medication is sold under the trade names Intropin, Dopastat, and Revimine, among others. It is on the World Health Organization's List of Essential Medicines. It is most commonly used as a stimulant drug in the treatment of severe low blood pressure, slow heart rate, and cardiac arrest. It is especially important in treating these in newborn infants. It is given intravenously. Since the half-life of dopamine in plasma is very short—approximately one minute in adults, two minutes in newborn infants and up to five minutes in preterm infants—it is usually given in a continuous intravenous drip rather than a single injection.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Dopamine): Outside the central nervous system, dopamine functions primarily as a local paracrine messenger. In blood vessels, it inhibits norepinephrine release and acts as a vasodilator; in the kidneys, it increases sodium excretion and urine output; in the pancreas, it reduces insulin production; in the digestive system, it hinders gastrointestinal motility and protects intestinal mucosa; and in the immune system, it reduces the activity of lymphocytes. With the exception of the blood vessels, dopamine is synthesized locally within these systems and exerts its effects near the cells that release it.

</details>

**Weird fact** · written · 9.3 s

> **Animals may have borrowed dopamine from bacteria**  
> Dopamine-synthesizing enzymes in animals might have come from bacteria via horizontal gene transfer. This transfer could have happened late in evolution, possibly during the formation of mitochondria through bacterial symbiosis.

<details><summary>Sources</summary>

- [Wikipedia, Comparative biology and evolution](https://en.wikipedia.org/wiki/Dopamine#Comparative_biology_and_evolution): There are no reports of dopamine in archaea, but it has been detected in some types of bacteria and in the protozoan called Tetrahymena. Perhaps more importantly, there are types of bacteria that contain homologs of all the enzymes that animals use to synthesize dopamine. It has been proposed that animals derived their dopamine-synthesizing machinery from bacteria, via horizontal gene transfer that may have occurred relatively late in evolutionary time, perhaps as a result of the symbiotic incorporation of bacteria into eukaryotic cells that gave rise to mitochondria.

</details>


### epinephrine (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Epinephrine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 9.6 s

> **In your body and emergency kits**  
> Epinephrine is naturally released by the adrenal glands to regulate blood pressure and smooth muscle constriction. It appears in emergency auto-injectors like EpiPen, first available in the USA in 1987, with newer versions approved in 0.3 mg and 0.15 mg strengths by Teva Pharmaceuticals USA on August 16, 2018. It is the drug of choice for treating severe allergic reactions.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Record-Description): Epinephrine, also known as adrenaline, is a hormone and neurotransmitter and produced by the adrenal glands that can also be used as a drug due to its various important functions. Though it has long been used in the treatment of hypersensitivity reactions, epinephrine in the auto-injector form (EpiPen) has been available since 1987 in the USA. Many new products/biosimilars and dosage routes have been approved under various names over the last several decades,,. On August 16, 2018, Teva Pharmaceuticals USA gained approval to market its generic epinephrine auto-injector in 0.3 mg and 0.15 mg strengths.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Uses): Drug of choice for emergency treatment of severe allergic reactions to insect stings or bites, foods, drugs, and other allergens.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Natural-Pollution-Sources): Epinephrine is a naturally occurring hormone, which is released from the adrenal medulla and is involved in the regulation of blood pressure and smooth muscle constriction(1). Epinephrine naturally occurs at the ends of sympathetic nerve fibers where they serve as chemical mediators for carrying nerve impulses(2).

</details>

**Weird fact** · written · 7.0 s

> **Epinephrine can cause heart attacks from overuse**  
> Epinephrine overdose may lead to myocardial ischemia and infarction. This is especially dangerous in people with preexisting conditions or when high doses are given. The strong effect on heart tissue can result in fatal cardiac arrhythmias or ischemia.

<details><summary>Sources</summary>

- [PubChem, Toxicity Summary](https://pubchem.ncbi.nlm.nih.gov/compound/5816#section=Toxicity-Summary): Signs and Symptoms of Overdose Epinephrine overdose may lead to myocardial ischemia, infarction, renal insufficiency, and cardiomyopathy. Patients may also experience severe pallor, cold skin, and metabolic acidosis due to elevated blood lactic acid levels. These complications are more likely in individuals with preexisting conditions or when epinephrine is administered in high doses. Pulmonary edema may also occur as a result of peripheral vasoconstriction and myocardial stimulation. Due to its strong β1-adrenergic effects on cardiac tissue, epinephrine toxicity can result in potentially fatal cardiac arrhythmias or ischemia.

</details>


### ethanol (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Ethanol carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · trimmed · 8.3 s

> **Ethanol first made synthetically in 1825**  
> In 1825, Michael Faraday discovered that sulfuric acid could absorb large volumes of coal gas. In 1828, Hennell and Georges-Simon Serullas independently found that this acid could be decomposed into ethanol. Thus, Faraday had unknowingly discovered a method to produce ethanol from ethylene via acid-catalyzed hydration.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Ethanol#History): Ethanol was first prepared synthetically in 1825 by Michael Faraday. He found that sulfuric acid could absorb large volumes of coal gas. He gave the resulting solution to Henry Hennell, a British chemist, who found in 1826 that it contained "sulphovinic acid" (ethyl hydrogen sulfate). In 1828, Hennell and the French chemist Georges-Simon Serullas independently discovered that sulphovinic acid could be decomposed into ethanol. Thus, in 1825 Faraday had unwittingly discovered that ethanol could be produced from ethylene (a component of coal gas) by acid-catalyzed hydration, a process similar to current industrial ethanol synthesis.

</details>

**Weird fact** · dropped · 8.3 s

Problems: body has 1 sentences, needs 2 to 4

**Where you meet it** · retried · 19.3 s

> **In drinks, fruit, and fuel**  
> Ethanol is found in alcoholic beverages, overripe fruit, and bertam palm blossoms. It is naturally produced by yeast during fermentation and is present in yeast habitats. As of 2024, world ethanol fuel production reached 120 gigaliters, mostly from the U.S.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Ethanol): Ethanol (also called ethyl alcohol, grain alcohol, drinking alcohol, or simply alcohol) is an organic compound with the chemical formula CH3CH2OH. It is an alcohol, with its formula also written as C2H5OH, C2H6O, or EtOH, where Et is the pseudoelement symbol for ethyl. Ethanol is a volatile, flammable, colorless liquid with a pungent taste. As a central nervous system (CNS) depressant, it is the active ingredient in alcoholic beverages, and the second most consumed drug globally behind caffeine.
- [Wikipedia, Natural occurrence](https://en.wikipedia.org/wiki/Ethanol#Natural_occurrence): Ethanol is a byproduct of the metabolic process of yeast. As such, ethanol will be present in any yeast habitat. Ethanol can commonly be found in overripe fruit. Ethanol produced by symbiotic yeast can be found in bertam palm blossoms. Although some animal species, such as the pentailed treeshrew, exhibit ethanol-seeking behaviors, most show no interest or avoidance of food sources containing ethanol. Ethanol is also produced during the germination of many plants as a result of natural anaerobiosis.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Ethanol): Ethanol is naturally produced by the fermentation process of sugars by yeasts or via petrochemical processes such as ethylene hydration. Historically it was used as a general anesthetic, and has modern medical applications as an antiseptic, disinfectant, solvent for some medications, and antidote for methanol poisoning and ethylene glycol poisoning. It is used as a chemical solvent and in the synthesis of organic compounds, and as a fuel source for lamps, stoves, and internal combustion engines. Ethanol also can be dehydrated to make ethylene, an important chemical feedstock. As of 2024, world production of ethanol fuel was 120 gigaliters (3.2×1010 U.S. gallons), coming mostly from the U.S.

</details>


### ethylene (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Ethylene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · dropped · 11.6 s

Problems: not stated in the cited snippets: "Ethylene first seen in Amsterdam, 1777"; not stated in the cited snippets: "The method of synthesis was the same as that used by Mr. Enée."

**Where you meet it** · written · 10.9 s

> **Ripening fruit and plastic bags**  
> Ethylene is naturally released by fruits, flowers, and ripening avocados and apples, with plants producing 0.6 to 6 ug/kg per hour, peaking at up to 120 ug/kg per hour during ripening. It's also found in gaseous metabolites from germinating seeds and fading flowers. In industry, ethylene is used to make polyethylene plastic, with worldwide production exceeding 225 million tonnes in 2022.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Ethylene): Ethylene is also an important natural plant hormone and is used in agriculture to induce ripening of fruits. The hydrate of ethylene is ethanol.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/6325#section=Natural-Pollution-Sources): Ethylene is a natural product emitted by fruits, flowers, leaves, roots, and tubers(1). The release rate of ethylene from plants varies during the life cycle of the plant(1). Plants that normally produce 0.6 to 6 ug ethylene/kg (fresh weight) per hour may produce up to 120 ug/kg per hour during ripening of fruits and during senescence and loss of leaves(1). Ethylene has been found in the gaseous metabolites released by germinating bean, corn, cotton, and pea seeds, from fading morning glory flowers and from ripening avocados and apples(2). Ethylene is released to the atmosphere from biomass combustion and volcanos(3).
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Ethylene): Ethylene is widely used in the chemical industry, and its worldwide production (over 225 million tonnes in 2022) exceeds that of any other organic compound. Much of this production goes toward creating polyethylene, which is a widely used plastic containing polymer chains of ethylene units in various chain lengths.

</details>

**Weird fact** · dropped · 11.4 s

Problems: body has 1 sentences, needs 2 to 4


### formaldehyde (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Formaldehyde carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 13.7 s

> **From your kitchen to lab tables**  
> Formaldehyde is a colorless, flammable gas with a pungent smell, naturally produced in small amounts in our bodies. It appears in household products like antiseptics, medicines, and cosmetics. A 5% solution is used as a disinfectant in healthcare and agriculture, while a 37% solution preserves tissue samples in labs. Global production reached 26 million tons per year in 2024.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/712#section=Record-Description): At room temperature, formaldehyde is a colorless, flammable gas that has a distinct, pungent smell. It is also known as methanal, methylene oxide, oxymethyline, methylaldehyde, and oxomethane. Formaldehyde is naturally produced in small amounts in our bodies. It is used in the production of fertilizer, paper, plywood, and urea-formaldehyde resins. It is also used as a preservative in some foods and in many products used around the house, such as antiseptics, medicines, and cosmetics.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/712#section=Record-Description): Formaldehyde, solution, flammable appears as a colorless aqueous solution of formaldehyde, which is a gas at ordinary conditions. Has a pungent irritating odor. Flash point varies from 122 to 141 °F. Denser than water. The vapors are heavier than air and are highly irritating to the nose. Toxic if swallowed. Contact can cause severe injury to the skin accompanied by drying, cracking, and scaling. Used to make plastics, other chemicals, and fertilizers. Used as a preservative and a corrosion inhibitor. Rate of onset: Immediate Persistence: Hours Odor threshold: 1 ppm Source/use/other hazard: Disinfection/germicide; fungicide; textile; health care (tissue fixing).
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Formaldehyde): Formaldehyde ( for-MAL-di-hide, US also fər-) (systematic name methanal) is a pungent colorless gas mainly used in the production of industrial resins, such as for particle board, coatings, plastics, pulp, paper, synthetic fibers, and textiles. A ~5% solution in water of formaldehyde is used as a disinfectant and fumigant in industrial, agricultural, and healthcare settings, and a ~37% solution is used to preserve tissue samples in labs. In 2024, the global production of formaldehyde was estimated at 26 million tons per year, and is a precursor to many other materials and chemical compounds.

</details>

**Weird fact** · written · 9.8 s

> **Formaldehyde polymerizes on its own**  
> Aqueous formaldehyde spontaneously oligomerizes without special conditions. It forms cyclic structures like 1,3,5-trioxane and polymerizes into paraformaldehyde. This behavior is unusual among small aldehydes.

<details><summary>Sources</summary>

- [Wikipedia, Organic chemistry](https://en.wikipedia.org/wiki/Formaldehyde#Organic_chemistry): Monomeric CH2O is a gas and is rarely encountered in the laboratory. Aqueous formaldehyde, unlike some other small aldehydes (which need specific conditions to oligomerize through aldol condensation) oligomerizes spontaneously at a common state. The trimer 1,3,5-trioxane, (CH2O)3, is a typical oligomer. Many cyclic oligomers of other sizes have been isolated. Similarly, formaldehyde hydrates to give the geminal diol methanediol, which condenses further to form hydroxy-terminated oligomers HO(CH2O)nH. The polymer is called paraformaldehyde. The higher concentration of formaldehyde—the more equilibrium shifts towards polymerization.

</details>


### geosmin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Geosmin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · trimmed · 6.9 s

> **The smell after rain**  
> Geosmin gives soil its earthy scent, especially when rain falls after dry weather — known as petrichor. This molecule is produced by bacteria, especially cyanobacteria, in soil and water.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Geosmin): Geosmin ( jee-OZ-min) is a chemical with a distinct earthy or musty odor. It is easily smelled by most people, having a very low odor detection threshold, ranging from 0.006 to 0.01 micrograms per liter in water. Geosmin is responsible for the earthy taste of beetroots and a contributor to the strong scent, known as petrichor, that occurs when rain falls after a spell of dry weather or when soil is disturbed. It is produced by bacteria, especially cyanobacteria, often present in soil and water supplies. Geosmin, along with 2-methylisoborneol, are the major biological causes of muddy odor and flavor in drinking water worldwide and in farmed fish.

</details>

**Weird fact** · written · 7.0 s

> **Humans smell geosmin at 5 ppt**  
> The human nose can detect geosmin at as low as 5 parts per trillion. Naturally occurring (−)-geosmin is roughly eight times more potent than its mirror form, (+)-geosmin. This extreme sensitivity may have evolved to help humans find scarce water.

<details><summary>Sources</summary>

- [Wikipedia, Effects](https://en.wikipedia.org/wiki/Geosmin#Effects): The human olfactory system is extremely sensitive to geosmin and is able to detect it at concentrations as low as anywhere from 400 parts per trillion to 5 parts per trillion. Human sensitivity differs between the two enantiomers. In a study of 50 subjects, the mean odor detection threshold was 9.5 ng/L for naturally occurring (−)-geosmin and 78 ng/L for (+)-geosmin. This makes the natural form roughly eight times more potent. It is proposed that this extreme sensitivity to geosmin evolved in humans or their ancestors to aid them in their search for scarce water, but this is not conclusive.

</details>


### glucose (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> D-Glucose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · written · 6.4 s

> **Glucose first isolated from raisins**  
> In 1747, German chemist Andreas Marggraf first isolated glucose from raisins. Later, in 1792, Johann Tobias Lowitz discovered glucose in grapes and distinguished it from sucrose. The term 'glucose' was coined by Jean-Baptiste Dumas in 1838.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Glucose#History): Glucose was first isolated from raisins in 1747 by the German chemist Andreas Marggraf. Glucose was discovered in grapes by another German chemist, Johann Tobias Lowitz, in 1792, and distinguished as being different from cane sugar (sucrose). Glucose is the term coined by Jean-Baptiste Dumas in 1838, which has prevailed in the chemical literature. Friedrich August Kekulé proposed the term dextrose (from the Latin dexter, meaning "right"), because in aqueous solutions of glucose, the plane of linearly polarized light is turned to the right. In contrast, l-fructose (usually referred to as l-fructose) (a ketohexose) and l-glucose (l-glucose) turn linearly polarized light to the left.

</details>

**Weird fact** · retried · 13.4 s

> **Glucose's name comes from its light twist**  
> Friedrich August Kekulé proposed the term dextrose because glucose turns linearly polarized light to the right. The term 'dextrose' comes from the Latin 'dexter', meaning 'right'. This optical property led to the naming convention, distinguishing it from l-fructose and l-glucose, which turn light to the left.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Glucose#History): Glucose was first isolated from raisins in 1747 by the German chemist Andreas Marggraf. Glucose was discovered in grapes by another German chemist, Johann Tobias Lowitz, in 1792, and distinguished as being different from cane sugar (sucrose). Glucose is the term coined by Jean-Baptiste Dumas in 1838, which has prevailed in the chemical literature. Friedrich August Kekulé proposed the term dextrose (from the Latin dexter, meaning "right"), because in aqueous solutions of glucose, the plane of linearly polarized light is turned to the right. In contrast, l-fructose (usually referred to as l-fructose) (a ketohexose) and l-glucose (l-glucose) turn linearly polarized light to the left.

</details>

**Where you meet it** · written · 14.1 s

> **Sugars in your food and nature**  
> D-Glucose is found in honey, starch, and glycogen, and appears in foods like grape juice and malt used for beer. It's in soft drinks in the US, often via HFCS-55 or HFCS-42, and in Mexico, where cane sugar sweetens beverages. It's also found in roots of Ilex asprella in China and straws of rice in California, and in male cones of Wollemia nobilis in Rome.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5793#section=Record-Description): Glucose is a simple sugar (monosaccharide) generated during phosynthesis involving water, carbon and sunlight in plants. It is produced in humans via hepatic gluconeogenesis and breakdown of polymeric glucose forms (glycogenolysis). It circulates in human circulation as blood glucose and acts as an essential energy source for many organisms through aerobic or anaerobic respiration and fermentation. It is primarily stored as starch in plants and glycogen in animals to be used in various metabolic processes in the cellular level. Its aldohexose stereoisomer, dextrose or D-glucose, is the most commonly occurring isomer of glucose in nature.
- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Glucose#Sources): Most dietary carbohydrates contain glucose, either as their only building block (as in the polysaccharides starch and glycogen), or together with another monosaccharide (as in the hetero-polysaccharides sucrose and lactose). Unbound glucose is one of the main ingredients of honey. Glucose is extremely abundant and has been isolated from a variety of natural sources across the world, including male cones of the coniferous tree Wollemia nobilis in Rome, the roots of Ilex asprella plants in China, and straws from rice in California.
- [Wikipedia, Commercial usage](https://en.wikipedia.org/wiki/Glucose#Commercial_usage): Glucose is mainly used for the production of fructose and of glucose-containing foods. In foods, it is used as a sweetener, humectant, to increase the volume and to create a softer mouthfeel. Various sources of glucose, such as grape juice (for wine) or malt (for beer), are used for fermentation to ethanol during the production of alcoholic beverages. Most soft drinks in the US use HFCS-55 (with a fructose content of 55% in the dry mass), while most other HFCS-sweetened foods in the US use HFCS-42 (with a fructose content of 42% in the dry mass). In Mexico, on the other hand, soft drinks are sweetened by cane sugar, which has a higher sweetening power.

</details>


### glyphosate (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Glyphosate carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 6.4 s

> **First made in 1950, then again in 1970**  
> In 1950, Swiss chemist Henry Martin synthesized glyphosate while working at Cilag, but the work was never published. In 1970, at Monsanto in the United States, John E. Franz made glyphosate as the third analog of aminomethylphosphonic acid, testing compounds for stronger herbicidal activity. Two earlier analogs showed weak herbicidal activity.

<details><summary>Sources</summary>

- [Wikipedia, Discovery](https://en.wikipedia.org/wiki/Glyphosate#Discovery): Glyphosate was first synthesized in 1950 by Swiss chemist Henry Martin, who worked for the Swiss company Cilag. The work was never published. Early studies found it to be a weak chemical chelating agent.
- [Wikipedia, Discovery](https://en.wikipedia.org/wiki/Glyphosate#Discovery): Glyphosate was independently discovered in the United States at Monsanto in 1970. About 100 derivatives of aminomethylphosphonic acid had been prepared as potential water-softening agents. Two were found to have weak herbicidal activity, and John E. Franz, a chemist at Monsanto, was asked to try to make analogs with stronger herbicidal activity. Glyphosate was the third analog he made. Franz received the National Medal of Technology of the United States in 1987 and the Perkin Medal for Applied Chemistry in 1990 for his discoveries.

</details>

**Weird fact** · written · 8.5 s

> **AMPA lingers for two years in frozen soil**  
> The glyphosate metabolite AMPA has been found in Swedish forest soils up to two years after a glyphosate application. This persistence is attributed to the soil being frozen for most of the year.

<details><summary>Sources</summary>

- [Wikipedia, Environmental fate](https://en.wikipedia.org/wiki/Glyphosate#Environmental_fate): The reported half-life of glyphosate in soil varies from two to 197 days with a typical field half-life of 47 days being suggested. Soil and climate conditions affect glyphosate's persistence in soil. The median half-life of glyphosate in water varies from a few days to 91 days. At a site in Texas, half-life was as little as three days. A site in Iowa had a half-life of 141.9 days. The glyphosate metabolite AMPA has been found in Swedish forest soils up to two years after a glyphosate application. In this case, the persistence of AMPA was attributed to the soil being frozen for most of the year.

</details>

**Where you meet it** · retried · 16.1 s

> **In gardens, farms, and roadsides**  
> Glyphosate is found in weed killer products like RoundUp™, widely used in farms, home gardens, and lawns. It is commonly applied in agriculture, horticulture, viticulture, and silviculture. Glyphosate-based formulations are easily bought in most stores and are used for roadside and forest applications.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/3496#section=Record-Description): Glyphosate is the active ingredient in weed killer products such as RoundUp™. Glyphosate products are one of the most widely used weed killers worldwide in farms and in home gardens and lawns. These products typically contain glyphosate in combination with other ingredients that help improve the absorption of the glyphosate into the plant. Glyphosate-based formulations (GBFs) are easily bought in most stores. These products can have different combinations of other ingredients or different concentrations of glyphosate.
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Glyphosate#Uses): Glyphosate is effective in killing a wide variety of plants, including grasses and broadleaf and woody plants. By volume, it is one of the most widely used herbicides. In 2007, glyphosate was the most used herbicide in the United States agricultural sector, with 180 to 185 million pounds (82,000 to 84,000 tonnes) applied, the second-most used in home and garden with 5 to 8 million pounds (2,300 to 3,600 tonnes) and 13 to 15 million pounds (5,900 to 6,800 tonnes) in non-agricultural settings. It is commonly used for agriculture, horticulture, viticulture, and silviculture purposes, as well as garden maintenance (including home use).
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/3496#section=Uses): A systemic herbicide commonly used for roadside and forest applications; [EXTOXNET]

</details>


### hydrogen cyanide (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Hydrogen Cyanide carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 5.7 s

> **First isolated from Prussian blue**  
> In 1752, French chemist Pierre Macquer isolated hydrogen cyanide by converting Prussian blue to an iron oxide and a volatile component, which could reconstitute the original compound. This volatile component was later identified as hydrogen cyanide. The Swedish chemist Carl Wilhelm Scheele independently prepared it from Prussian blue in 1782.

<details><summary>Sources</summary>

- [Wikipedia, History of discovery and naming](https://en.wikipedia.org/wiki/Hydrogen_cyanide#History_of_discovery_and_naming): Hydrogen cyanide was first isolated in 1752 by French chemist Pierre Macquer who converted Prussian blue to an iron oxide plus a volatile component and found that these could be used to reconstitute it. The new component was what is now known as hydrogen cyanide. It was subsequently prepared from Prussian blue by the Swedish chemist Carl Wilhelm Scheele in 1782, and was eventually given the German name Blausäure (lit. 'Blue acid') because of its acidic nature in water and its derivation from Prussian blue. In English, it became known popularly as prussic acid.

</details>

**Where you meet it** · retried · 19.4 s

> **In pits and in fumigants**  
> Hydrogen cyanide is found in the pits of cherries, apricots, apples, and bitter almonds, where cyanohydrins like mandelonitrile and amygdalin slowly release it. One hundred grams of crushed apple seeds can yield about 70 mg of HCN. It is also used to fumigate enclosed spaces and food production facilities, and is present in plants such as cassava roots, lima beans, and almonds.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Hydrogen_cyanide#Occurrence): HCN is obtainable from fruits that have a pit, such as cherries, apricots, apples, and nuts such as bitter almonds, from which almond oil and extract is made. Many of these pits contain small amounts of cyanohydrins such as mandelonitrile and amygdalin, which slowly release hydrogen cyanide. One hundred grams of crushed apple seeds can yield about 70 mg of HCN.
- [Wikipedia, Applications](https://en.wikipedia.org/wiki/Hydrogen_cyanide#Applications): HCN is used globally as a fumigant against many species of pest insects that infest food production facilities. Both its efficacy and method of application lead to very small amounts of the fumigant being used compared to other toxic substances used for the same purpose. Using HCN as a fumigant also has less environmental impact, compared to some other fumigants such as sulfuryl fluoride, and methyl bromide.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/768#section=Uses): Used to manufacture other chemicals and to fumigate enclosed spaces; [ATSDR ToxProfiles] Used in the following processes: electroplating, metallurgy, and photo development; HCN is a byproduct of blast furnaces, coke ovens, photoengraving, petroleum refining, and some metal mining processes. HCN is released when burning nitrogen containing compounds, e.g., wool, silk, and plastics. HCN is present in plants such as cassava roots, lima beans, and almonds. [CHEMINFO]

</details>

**Weird fact** · retried · 12.8 s

> **Apple seeds release toxic hydrogen cyanide**  
> One hundred grams of crushed apple seeds can yield about 70 mg of HCN. These seeds contain cyanohydrins like amygdalin that slowly release hydrogen cyanide. HCN is found in fruits with pits such as apples, cherries, and apricots.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Hydrogen_cyanide#Occurrence): HCN is obtainable from fruits that have a pit, such as cherries, apricots, apples, and nuts such as bitter almonds, from which almond oil and extract is made. Many of these pits contain small amounts of cyanohydrins such as mandelonitrile and amygdalin, which slowly release hydrogen cyanide. One hundred grams of crushed apple seeds can yield about 70 mg of HCN.

</details>


### ibuprofen (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Ibuprofen, (+-)- carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · written · 6.2 s

> **From hangover cure to pain relief**  
> In the 1950s and 1960s, a team led by Stewart Adams at Boots Group researched safer alternatives to aspirin. Adams initially tested the compound as a hangover treatment. The molecule, derived from propionic acid, was discovered and synthesized during this period, with a patent application filed in 1961.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Ibuprofen#History): Ibuprofen was derived from propionic acid by the research arm of Boots Group during the 1960s. The name is derived from the 3 functional groups: isobutyl (ibu) propionic acid (pro) phenyl (fen). Its discovery was the result of research during the 1950s and 1960s to find a safer alternative to aspirin. The molecule was discovered and synthesized by a team led by Stewart Adams, with a patent application filed in 1961. Adams initially tested the drug as treatment for his hangover. In 1985, Boots's worldwide patent for ibuprofen expired and generic products were launched.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Ibuprofen#History): In recognition of the pioneering research work, here on Pennyfoot Street, by Dr Stewart Adams and Dr John Nicholson in the Research Department of Boots, which led to the discovery of ibuprofen used by millions worldwide for the relief of pain.

</details>

**Where you meet it** · dropped · 12.9 s

Problems: name U is not in the cited snippets

**Weird fact** · written · 10.9 s

> **Ibuprofen cuts Parkinson's risk by 38%**  
> Ibuprofen is linked to a 38% lower risk of developing Parkinson's disease. This neuroprotective effect was reported by Harvard Medical School researchers in March 2011. Other pain relievers like aspirin and paracetamol did not show the same benefit.

<details><summary>Sources</summary>

- [Wikipedia, Research](https://en.wikipedia.org/wiki/Ibuprofen#Research): Ibuprofen has been associated with a lower risk of Parkinson's disease and may delay or prevent it. Aspirin, other NSAIDs, and paracetamol (acetaminophen) did not affect the risk for Parkinson's. In March 2011, researchers at Harvard Medical School announced that ibuprofen had a neuroprotective effect against the risk of developing Parkinson's disease. People regularly consuming ibuprofen were reported to have a 38% lower risk of developing Parkinson's disease, but no such effect was found for other pain relievers, such as aspirin and paracetamol.

</details>


### indigo (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Indigo carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · dropped · 10.8 s

Problems: name Isaac is not in the cited snippets

**Where you meet it** · written · 8.5 s

> **In your blue jeans and plants**  
> Indigo is found in the leaves of Indigofera tinctoria plants, where it naturally forms a blue pigment. Today, most indigo is synthetic and used in denim production, with a pair of jeans requiring 3–12 grams of dye. It's also used in small amounts for wool and silk dyeing.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Indigo_dye): Indigo dye is an organic compound with a distinctive blue color. Indigo is a natural dye obtained from the leaves of some plants of the Indigofera genus, in particular Indigofera tinctoria. Dye-bearing Indigofera plants were once common throughout the world. Blue colorants are relatively rare. Since indigo is insoluble, it is also referred to as a pigment (C.I. Pigment Blue 66, C.I.).
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Indigo_dye): Most indigo dye produced today is synthetic, constituting around 80,000 tonnes each year, as of 2023. It is most commonly associated with the production of denim cloth and blue jeans, where its properties allow for effects such as stone washing and acid washing to be applied quickly.
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Indigo_dye#Uses): The primary use for indigo is as a dye for cotton yarn, mainly used in the production of denim cloth suitable for blue jeans; on average, a pair of blue jeans requires 3–12 grams (0.11–0.42 oz) of dye. Smaller quantities are used in the dyeing of wool and silk.

</details>

**Weird fact** · written · 10.7 s

> **Indigo turns white with bacteria's breath**  
> In Japan, thermophilic, anaerobic bacteria generate hydrogen as a metabolic product, which converts insoluble indigo into soluble indigo white. This process occurs in a heated vat and allows cloth to be dyed and decorated using traditional techniques like shibori and katazome.

<details><summary>Sources</summary>

- [Wikipedia, Dyeing technology](https://en.wikipedia.org/wiki/Indigo_dye#Dyeing_technology): A pre-industrial process for production of indigo white, used in Europe, was to dissolve the indigo in stale urine, which contains ammonia. A more convenient reductive agent is zinc. Another pre-industrial method, used in Japan, was to dissolve the indigo in a heated vat in which a culture of thermophilic, anaerobic bacteria was maintained. Some species of such bacteria generate hydrogen as a metabolic product, which convert insoluble indigo into soluble indigo white. Cloth dyed in such a vat was decorated with the techniques of shibori (tie-dye), kasuri, katazome, and tsutsugaki.

</details>


### limonene (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> 1-Methyl-4-Prop-1-En-2-Ylcyclohexene carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Where you meet it** · retried · 16.7 s

> **In citrus peels and cleaning products**  
> D-limonene, found in orange, lemon, grapefruit, and berry oils, is used as a flavoring agent in food and medicines to mask bitter tastes. It is also added to aftershave lotions, bath products, and hand cleansers for its lemon or orange fragrance. (+)-Limonene is used in organic herbicides and cleaning products where it dissolves oils.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Limonene): Limonene is a slightly yellow-green liquid aliphatic hydrocarbon classified as a cyclic monoterpene, and is the major component in the fragrance and essential oil of citrus fruit peels, taking its name from Italian limone 'lemon'.
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Limonene#Uses): As the main fragrance of citrus peels, D-limonene is used in food manufacturing and some medicines, such as a flavoring agent to mask the bitter taste of alkaloids, and as a fragrance in perfumery, aftershave lotions, bath products, and other personal care products. (+)-Limonene is also used as a botanical insecticide. (+)-Limonene is used in the organic herbicides. It is added to cleaning products, such as hand cleansers, to give a lemon or orange fragrance (see orange oil) and for its ability to dissolve oils. In contrast, (−)-limonene has a piny, turpentine-like odor.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/22311#section=Natural-Pollution-Sources): Limonene is found in many oils and fruits including orange, lemon, grapefruit, berry, leaf, caraway, dill, bergamot, peppermint and spearmint oils(1-4). Limonene emissions to the environment are associated with wax myrtle, sweet acacia, oranges, tomatoes, grasses, and California western sagebrush(5). Emissions of limonene are also associated with balsam poplar, European larche, European fir, Scots pine, Siberian pine, silver fir, common juniper, zeravshan juniper, pencil cedar, evergreen cypress, northern white cedar, chinese arbor vitae, marsh tea and deciduous moss(6).

</details>

**Weird fact** · skipped · 7.4 s


### lycopene (2 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Lycopene is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Where you meet it** · written · 9.6 s

> **The red in your food**  
> Lycopene gives red to pink hues in tomatoes, pink grapefruit, watermelon, red guavas, and baked beans. It's found in these foods and products like ketchup. As a carotenoid, it may help retard aging and degenerative diseases in animals.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/446925#section=Record-Description): Lycopene is a naturally occurring red carotenoid pigment that is responsible for red to pink colors seen in tomatoes, pink grapefruit, and other foods. Having a chemical formula of C40H56, lycopene is a tetraterpene assembled from eight isoprene units that are solely composed of carbon and hydrogen. Lycopene may undergo extensive isomerization that allows 1056 theoretical cis-trans configurations; however, the all-trans configuration of lycopene is the most predominant isomer found in foods that gives the red hue.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/446925#section=Record-Description): Lycopene is a linear, unsaturated hydrocarbon carotenoid, the major red pigment in fruits such as tomatoes, pink grapefruit, apricots, red oranges, watermelon, rosehips, and guava. As a class, carotenoids are pigment compounds found in photosynthetic organisms (plants, algae, and some types of fungus), and are chemically characterized by a large polyene chain containing 35-40 carbon atoms; some carotenoid polyene chains are terminated by two 6-carbon rings. In animals, carotenoids such as lycopene may possess antioxidant properties which may retard aging and many degenerative diseases. As an essential nutrient, lycopene is required in the animal diet. (NCI04)
- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Lycopene#Occurrence): Aside from tomatoes or tomato products like ketchup, it is found in watermelons, grapefruits, red guavas, and baked beans. It has no vitamin A activity.

</details>

**Weird fact** · dropped · 11.2 s

Problems: body has 1 sentences, needs 2 to 4


### melatonin (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Melatonin is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · trimmed · 7.9 s

> **From skin color to sleep aid**  
> In 1917, Carey Pratt McCord and Floyd P. Allen observed that cow pineal gland extracts caused tadpole skin to lighten, linking it to melanophore contraction. In 1958, Aaron B. Lerner and his team at Yale University isolated melatonin from bovine pineal gland extracts, motivated by its potential to treat skin diseases.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Melatonin#History): Melatonin's discovery is linked to the study of color changes in some amphibians and reptiles, a phenomenon initially observed through the administration of pineal gland extracts. In 1917, Carey Pratt McCord and Floyd P. Allen found that feeding extracts from the pineal glands of cows caused the skin of tadpoles to lighten by contracting the dark epidermal melanophores.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Melatonin#History): The hormone melatonin was isolated in 1958 by Aaron B. Lerner, a dermatology professor, and his team at Yale University. Motivated by the possibility that a substance from the pineal gland could be beneficial in treating skin diseases, they extracted and identified melatonin from bovine pineal gland extracts. Subsequent research in the mid-1970s by Lynch and others demonstrated that melatonin production follows a circadian rhythm in human pineal glands.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Melatonin#History): The etymology of melatonin stems from its skin-lightening properties. As detailed in their publication in the Journal of the American Chemical Society, Lerner and his colleagues proposed the name melatonin, derived from the Greek words melas, meaning 'black' or 'dark', and tonos, meaning 'labour', 'colour' or 'suppress'. This naming convention follows that of serotonin, another agent affecting skin color, discovered in 1948 as a modulator of vascular tone, which influenced its name based on its serum vasoconstrictor effect.

</details>

**Where you meet it** · written · 10.3 s

> **In your body and in plants**  
> Melatonin is naturally secreted by the pineal gland and retina during dark periods. It's found in plants like feverfew, St John's wort, rice, corn, and tomatoes. The medication Circadin, approved since 2007, treats insomnia in certain regions.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Melatonin): Melatonin is sometimes administered exogenously as a dietary supplement and medication. It is used in the treatment of sleep-related disorders such as insomnia and various circadian rhythm sleep disorders.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/896#section=Natural-Pollution-Sources): Melatonin is secreted from the pineal gland and retina during dark periods of the vertebrate circadian rhythm and is formed from serotonin(1). Melatonin, a hormone of the pineal gland, is also produced by extrapineal tissues that lighten skin color in amphibians(2). Melatonin has been identified in many plants including feverfew (Tanacetum parthenium), St John's wort (Hypericum perforatum), rice, corn, tomato and other edible fruits(3).
- [Wikipedia, Applications](https://en.wikipedia.org/wiki/Melatonin#Applications): An extended-release pharmaceutical formulation of melatonin is approved under the brand name Circadin for the treatment of insomnia in certain settings, such as in people aged 55 years of age or older. It is approved in the European Union, Israel, Australia, and countries in Asia and elsewhere in the world, but not in the United States (where it reached phase III clinical trials but was not approved). The medication has been licensed since 2007.

</details>

**Weird fact** · written · 8.6 s

> **Pineal extract lights up tadpole skin**  
> Melatonin's discovery began when pineal gland extracts caused tadpole skin to lighten by contracting dark melanophores. This color change was observed in 1917 by McCord and Allen.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Melatonin#History): Melatonin's discovery is linked to the study of color changes in some amphibians and reptiles, a phenomenon initially observed through the administration of pineal gland extracts. In 1917, Carey Pratt McCord and Floyd P. Allen found that feeding extracts from the pineal glands of cows caused the skin of tadpoles to lighten by contracting the dark epidermal melanophores.

</details>


### menthol (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Menthol carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · trimmed · 4.9 s

> **First isolated in 1771**  
> Early characterizations were conducted by Oppenheim, Beckett, Moriya, and Atkinson. The name 'menthol' was proposed by F. L. Alphonse Oppenheim in 1861.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Menthol#History): Menthol was first isolated in 1771, by the German, Hieronymus David Gaubius. Early characterizations were done by Oppenheim, Beckett, Moriya, and Atkinson. It was named by F. L. Alphonse Oppenheim (1833–1877) in 1861.

</details>

**Where you meet it** · written · 9.9 s

> **In minty snacks and medicines**  
> Menthol is found naturally in peppermint oil from Mentha × piperita, giving minty flavor and a cooling sensation. It appears in foods and pharmaceuticals as a flavor enhancer and local anesthetic. Japanese menthol also contains a small amount of neomenthol, the 1-epimer.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/1254#section=Record-Description): Menthol is an organic compound made synthetically or obtained from peppermint or mint oils with flavoring and local anesthetic properties. When added to pharmaceuticals and foods, menthol functions as a fortifier for peppermint flavors. It also has a counterirritant effect on skin and mucous membranes, thereby producing a local analgesic or anesthetic effect.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Menthol): Menthol is a monoterpenoid organic compound that occurs naturally in the oils of certain plants in the mint family, such as corn mint and peppermint. It is a white or clear waxy crystalline substance that is solid at room temperature and melts slightly above. The main form of menthol occurring in nature is (−)-menthol, which is assigned the (1R,2S,5R) configuration.
- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Menthol#Occurrence): Menthol occurs naturally in peppermint oil (along with a little menthone, the ester menthyl acetate and other compounds), obtained from Mentha × piperita (peppermint). Japanese menthol also contains a small percentage of the 1-epimer neomenthol.

</details>

**Weird fact** · written · 10.1 s

> **Only one form is naturally common**  
> Natural menthol exists as a single pure stereoisomer, the (1R,2S,5R) form. Out of eight possible stereoisomers, this is the one nearly always found in nature.

<details><summary>Sources</summary>

- [Wikipedia, Structure](https://en.wikipedia.org/wiki/Menthol#Structure): Natural menthol exists as one pure stereoisomer, nearly always the (1R,2S,5R) form (bottom left corner of the diagram below). The eight possible stereoisomers are:

</details>


### morphine (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Morphine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 5 more hazards, not listed here.

**Origin story** · written · 7.7 s

> **Discovered in Paderborn, 1804**  
> In December 1804 in Paderborn, German pharmacist Friedrich Sertürner discovered morphine as the first active alkaloid from the opium poppy plant. He noticed it induced drowsiness in rats and stray dogs and reported his findings in 1805. He named it morphium after the Greek god of dreams, Morpheus, because it causes sleep.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Morphine#History): Morphine was discovered as the first active alkaloid extracted from the opium poppy plant in December 1804 in Paderborn by German pharmacist Friedrich Sertürner. Sertürner noticed the compound induced drowsiness in rats and stray dogs and he reported his findings in 1805. In 1817, Sertürner reported experiments in which he administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died. Sertürner originally named the substance morphium after the Greek god of dreams, Morpheus, as it has a tendency to cause sleep. Sertürner's morphium was six times stronger than opium.

</details>

**Weird fact** · dropped · 12.4 s

Problems: not stated in the cited snippets: "Friedrich Sertürner administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died."; not stated in the cited snippets: "This shows how potent morphine was even in early experiments."

**Where you meet it** · written · 12.7 s

> **In poppies and pain meds**  
> Morphine is found naturally in the latex of the opium poppy (Papaver somniferum), making up 8–14% of dry opium. It appears in pain medications administered orally, sublingually, intravenously, or via injection. The molecule is also present in the sap of the opium poppy.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Morphine): Morphine, formerly known as morphium, is an opiate found naturally in opium, a dark brown resin produced by drying the latex of opium poppies (Papaver somniferum). It is mainly used as an analgesic (pain medication). There are multiple methods used to administer morphine: oral; sublingual; via inhalation; injection into a muscle, injection under the skin, or injection into the spinal cord area; transdermal; intravenously; or via rectal suppository. It acts directly on the central nervous system (CNS) to induce analgesia and alter perception and emotional response to pain. Physical and psychological dependence and tolerance may develop with repeated administration.
- [Wikipedia, Natural occurrence](https://en.wikipedia.org/wiki/Morphine#Natural_occurrence): Morphine is the most abundant opiate found in opium, the dried latex extracted by shallowly scoring the unripe seedpods of the Papaver somniferum poppy. Morphine is generally 8–14% of the dry weight of opium. Przemko and Norman cultivars of the opium poppy, are used to produce two other alkaloids, thebaine and oripavine, which are used in the manufacture of semi-synthetic and synthetic opioids like oxycodone and etorphine. P. bracteatum does not contain morphine or codeine, or other narcotic phenanthrene-type, alkaloids. This species is rather a source of thebaine.
- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/5288826#section=Natural-Pollution-Sources): Morphine occurs naturally in poppies(1), specifically in sap from the opium poppy (Papaver somniferum)(2).

</details>


### nicotine (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Nicotine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 6.6 s

> **From tobacco to lab isolation**  
> In 1828, Wilhelm Heinrich Posselt and Karl Ludwig Reimann isolated nicotine from the tobacco plant in Germany, believing it was a poison. Its chemical structure was discovered in 1893 by Adolf Pinner and Richard Wolffenstein. The first synthesis of nicotine occurred in 1904 by Amé Pictet and A. Rotschy.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Nicotine#History): Nicotine was originally isolated from the tobacco plant in 1828 by chemists Wilhelm Heinrich Posselt and Karl Ludwig Reimann from Germany, who believed it was a poison. Its chemical empirical formula was described by Melsens in 1843, its structure was discovered by Adolf Pinner and Richard Wolffenstein in 1893, and it was first synthesized by Amé Pictet and A. Rotschy in 1904.

</details>

**Where you meet it** · trimmed · 10.6 s

> **In tobacco and nightshade plants**  
> Nicotine is found in tobacco leaves, making up 0.6–3% of their dry weight. It also occurs in trace amounts in tomatoes, potatoes, and eggplants.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Nicotine): Nicotine is a stimulant and alkaloid found primarily in plants of the nightshade family, notably in tobacco; it is also synthesized. Nicotine is used recreationally for its stimulant and anxiolytic effects. In tobacco leaves, nicotine constitutes about 0.6–3% of the dry weight, and smaller, trace quantities occur in other Solanaceae crops such as tomatoes, potatoes, and eggplants. In pure form, nicotine is a colorless to yellowish, oily liquid that readily penetrates biological membranes and acts as a potent neurotoxin in insects, where it serves as an antiherbivore toxin.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Nicotine): In humans, nicotine acts primarily as a stimulant by binding to and activating nicotinic acetylcholine receptors (nAChRs) in the central nervous system and peripheral tissues. This results in the release of neurotransmitters such as dopamine, acetylcholine, and norepinephrine, producing effects including increased alertness, reduced anxiety, and mild euphoria. Nicotine is typically consumed through tobacco smoking, vaping, or other nicotine delivery systems.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Nicotine): Nicotine is highly addictive, and nicotine dependence is characterized by tolerance, physical dependence, psychological dependence, and nicotine withdrawal symptoms such as irritability, anxiety, and difficulty concentrating. Nicotine replacement therapy (NRT) products, including gums, patches, and lozenges, deliver the compound in slower, lower doses that are less addictive and are used medically to help people quit smoking. Synthetic derivatives of nicotine, such as varenicline, act as partial agonists at nicotinic receptors and are also used as smoking cessation aids.

</details>

**Weird fact** · dropped · 14.3 s

Problems: not stated in the cited snippets: "Nicotine saved Jamestown in 1612"


### nitroglycerin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Nitroglycerin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 6.6 s

> **First made in 1846 by Sobrero**  
> In 1846, Italian chemist Ascanio Sobrero synthesized nitroglycerin while working under Théophile-Jules Pelouze at the University of Turin. He initially named it 'pyroglycerin' and strongly warned against its use as an explosive. This discovery marked the first practical explosive stronger than black powder.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Nitroglycerin#History): Nitroglycerin was the first practical explosive produced that was stronger than black powder. It was synthesized by the Italian chemist Ascanio Sobrero in 1846, working under Théophile-Jules Pelouze at the University of Turin. Sobrero initially called his discovery "pyroglycerin" and warned vigorously against its use as an explosive.

</details>

**Where you meet it** · written · 12.6 s

> **In heart pills and dynamite**  
> Nitroglycerin is found in medications for angina pectoris and as an antihypertensive, where it dilates veins to reduce heart preload. It is also present in dynamite, patented in 1867, used in tunnel drilling, mining, and demolition. These formulations come in various doses and are applied in both medical and civil engineering contexts.

<details><summary>Sources</summary>

- [Wikipedia, Medical use](https://en.wikipedia.org/wiki/Nitroglycerin#Medical_use): Nitroglycerin belongs to a group of drugs called nitrates, which includes many other nitrates like isosorbide dinitrate (Isordil) and isosorbide mononitrate (Imdur, Ismo, Monoket). These agents all exert their effect by being converted to nitric oxide in the body by mitochondrial aldehyde dehydrogenase (ALDH2), and nitric oxide is a potent natural vasodilator.
- [Wikipedia, Medical use](https://en.wikipedia.org/wiki/Nitroglycerin#Medical_use): In medicine, nitroglycerin is probably most commonly prescribed for angina pectoris, a painful symptom of ischemic heart disease caused by inadequate flow of blood and oxygen to the heart, and as a potent antihypertensive agent. Nitroglycerin corrects the imbalance between the flow of oxygen and blood to the heart and the heart's energy demand. There are many formulations on the market at different doses. At low doses, nitroglycerin dilates veins more than arteries, thereby reducing preload (volume of blood in the heart after filling); this is thought to be its primary mechanism of action.
- [Wikipedia, Use as an explosive and a propellant](https://en.wikipedia.org/wiki/Nitroglycerin#Use_as_an_explosive_and_a_propellant): He named this explosive dynamite and patented it in 1867. It was supplied ready for use in the form of sticks, individually wrapped in greased waterproof paper. Dynamite and similar explosives were widely adopted for civil engineering tasks, such as in drilling highway and railroad tunnels, for mining, for clearing farmland of stumps, in quarrying, and in demolition work. Likewise, military engineers have used dynamite for construction and demolition work.

</details>

**Weird fact** · dropped · 20.8 s

Problems: not stated in the cited snippets: "Workers get 'Sunday heart attacks'"


### oxytocin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Oxytocin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.

**Origin story** · written · 8.5 s

> **From uterine contractions to synthesis**  
> In 1906, Henry Hallett Dale discovered the uterine-contracting properties of a substance later named oxytocin. In 1910 and 1911, Ott and Scott, and Schafer and Mackenzie, described its milk ejection property. In 1909, William Blair-Bell performed the first clinical use to induce childbirth. In 1953, Vincent du Vigneaud synthesized oxytocin, the first polypeptide hormone to be synthesized.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Oxytocin#History): The uterine-contracting properties of the principle that would later be named oxytocin were discovered by British pharmacologist Henry Hallett Dale in 1906, and its milk ejection property was described by Ott and Scott in 1910 and by Schafer and Mackenzie in 1911. In 1909 the first clinical use of oxytocin was performed by William Blair-Bell to induce childbirth in patients with complications.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Oxytocin#History): By the 1920s, oxytocin and vasopressin had been isolated from pituitary tissue and given their current names. Oxytocin's molecular structure was determined in 1952. In the early 1950s, American biochemist Vincent du Vigneaud found that oxytocin is made up of nine amino acids. He identified its amino acid sequence, the first polypeptide hormone to be sequenced. In 1953, du Vigneaud carried out the synthesis of oxytocin, the first polypeptide hormone to be synthesized. Du Vigneaud was awarded the Nobel Prize in Chemistry in 1955 for his work. Further work on different synthetic routes for oxytocin, as well as the preparation of analogues of the hormone (e.g.

</details>

**Where you meet it** · retried · 13.5 s

> **In birth, in love, in food**  
> Oxytocin is released during childbirth and sexual activity, stimulating uterine contractions and maternal bonding. It is also found in the bloodstream and in pharmaceutical form. In Oxystelma esculentum, oxytocin has been reported with data available.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Oxytocin): Oxytocin is a peptide hormone and neuropeptide normally produced in the hypothalamus and released by the posterior pituitary. Present in animals since early stages of evolution, in humans it plays roles in behavior that include social bonding, love, reproduction, childbirth, and the period after childbirth. Oxytocin is released into the bloodstream as a hormone in response to sexual activity and during childbirth. It is also available in pharmaceutical form. In either form, oxytocin stimulates uterine contractions to speed up the process of childbirth.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Oxytocin): In its natural form, it also plays a role in maternal bonding and milk production. Production and secretion of oxytocin is controlled by a positive feedback mechanism, where its initial release stimulates production and release of further oxytocin. For example, when oxytocin is released during a uterine contraction at the start of childbirth, this stimulates the production and release of more oxytocin and an increase in the intensity and frequency of contractions. This process intensifies in frequency and continues until the triggering activity ceases. A similar process takes place during lactation and during sexual activity.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/439302#section=Record-Description): oxytocin has been reported in Oxystelma esculentum with data available.

</details>

**Weird fact** · written · 11.0 s

> **Oxytocin has a different building block in primates**  
> A novel structure of oxytocin was reported in 2011 in marmosets, tamarins, and other New World primates. A single amino acid substitution occurs at the 8-position: proline replaces leucine. This variant, Pro8-OT, has been confirmed by two other laboratories and found in additional oxytocin structural variants in this primate group.

<details><summary>Sources</summary>

- [Wikipedia, Chemistry](https://en.wikipedia.org/wiki/Oxytocin#Chemistry): While the structure of oxytocin is highly conserved in placental mammals, a novel structure of oxytocin was reported in 2011 in marmosets, tamarins, and other New World primates. Genomic sequencing of the gene for oxytocin revealed a single in-frame mutation (thymine for cytosine) which results in a single amino acid substitution at the 8-position (proline for leucine). Since this original Lee et al. paper, two other laboratories have confirmed Pro8-OT and documented additional oxytocin structural variants in this primate taxon. Vargas-Pinilla et al.

</details>


### quinine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Quinine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 3 more hazards, not listed here.

**Origin story** · trimmed · 24.0 s

> **Jesuits bring bark to Europe**  
> Nicolás Monardes (1571) and Juan Fragoso (1572) described the tree whose bark was used for a drink to treat diarrhea. Jesuit Agostino Salumbrino observed the Quechua using the bark to treat shivering and sent a sample to Rome for testing as a malaria treatment.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Quinine#History): Spanish Jesuit missionaries were the first to bring cinchona to Europe. The Spanish had observed the Quechua's use of cinchona and were aware of its bark's medicinal properties by the 1570s: Nicolás Monardes (1571) and Juan Fragoso (1572) both described a tree, which was subsequently identified as the cinchona tree, whose bark was used to produce a drink to treat diarrhea. Quinine has been used in unextracted form by Europeans since at least the early 17th century.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Quinine#History): The Jesuit Agostino Salumbrino (1564–1642), an apothecary by training who lived in Lima (now in present-day Peru), observed the Quechua using the bark of the cinchona tree to treat such shivering. While its effect in treating malaria (and malaria-induced shivering) was unrelated to its effect in controlling shivering from rigors, it was a successful medicine against malaria. At the first opportunity, Salumbrino sent a small quantity to Rome for testing as a malaria treatment. In the years that followed, cinchona bark, known as Jesuit's bark or Peruvian bark, became one of the most valuable commodities shipped from Peru to Europe.

</details>

**Where you meet it** · written · 15.2 s

> **In tonic water and malaria treatment**  
> Quinine appears in tonic water and bitter lemon to give a bitter taste. It is also used medically to treat malaria, especially in areas with resistance to chloroquine, and for nocturnal leg cramps—though side effects are a concern. As an antimalarial, it is administered orally or intravenously.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Quinine): Quinine is an alkaloid used medically to treat malaria and babesiosis. This includes the treatment of malaria due to Plasmodium falciparum that is resistant to chloroquine when artesunate is not available. While sometimes used for nocturnal leg cramps, quinine is not recommended for this purpose due to the risk of serious side effects. It can be taken by mouth or intravenously. Malaria resistance to quinine occurs in certain areas of the world. Quinine is also used as an ingredient in tonic water and other beverages to impart a bitter taste.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/3034034#section=Uses): Medicine (antimalarial) as the alkaloid or as numerous salts and derivatives; flavoring in carbonated beverages
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/3034034#section=Uses): For the treatment of malaria and leg cramps. Used as an antipyretic (fever-reducing), antimalarial, analgesic (painkilling), and anti-inflammatory. It is also used as a flavor component of tonic water and bitter lemon.

</details>

**Weird fact** · written · 12.8 s

> **Quinine suppresses brain chemicals**  
> Quinine can act as a competitive inhibitor of monoamine oxidase (MAO), an enzyme that removes neurotransmitters from the brain. As an MAO inhibitor, it has potential to serve as a treatment for psychological disorders, similar to antidepressants.

<details><summary>Sources</summary>

- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Quinine#Uses): Quinine was frequently prescribed as an off-label treatment for leg cramps at night, but this has become less common since 2010 due to a warning from the US Food and Drug Administration (FDA) that such practice is associated with life-threatening side effects. Quinine can also act as a competitive inhibitor of monoamine oxidase (MAO), an enzyme that removes neurotransmitters from the brain. As an MAO inhibitor, it has potential to serve as a treatment for individuals with psychological disorders, similar to antidepressants that inhibit MAO.

</details>


### serotonin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Serotonin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 10.7 s

> **From enteramine to serotonin**  
> In 1935, Vittorio Erspamer in Pavia discovered a substance from enterochromaffin cells that caused intestinal contraction, which he named 'enteramine'. Two years later, he confirmed it was a new amine. In 1948, Maurice M. Rapport, Arda Green, and Irvine Page discovered a vasoconstrictor in blood serum and named it serotonin. In 1952, enteramine was shown to be the same as serotonin.

<details><summary>Sources</summary>

- [Wikipedia, History & etymology](https://en.wikipedia.org/wiki/Serotonin#History_&_etymology): It had been known to physiologists for over a century that a vasoconstrictor material appears in serum when blood was allowed to clot. In 1935, Italian Vittorio Erspamer, working in Pavia, showed an extract from enterochromaffin cells made intestines contract. Some believed it contained adrenaline, but two years later, Erspamer was able to show it was a previously unknown amine, which he named "enteramine". In 1948, Maurice M. Rapport, Arda Green, and Irvine Page of the Cleveland Clinic discovered a vasoconstrictor substance in blood serum, and since it was a serum agent affecting vascular tone, they named it serotonin.
- [Wikipedia, History & etymology](https://en.wikipedia.org/wiki/Serotonin#History_&_etymology): In 1952, enteramine was shown to be the same substance as serotonin, and as the broad range of physiological roles was elucidated, the abbreviation 5-HT of the proper chemical name 5-hydroxytryptamine became the preferred name in the pharmacological field. Synonyms of serotonin include: 5-hydroxytriptamine, enteramine, substance DS, and 3-(β-aminoethyl)-5-hydroxyindole. In 1953, Betty Twarog and Page discovered serotonin in the central nervous system.

</details>

**Where you meet it** · trimmed · 12.5 s

> **In your gut and brain**  
> Serotonin is mostly found in the gastrointestinal tract, where it regulates intestinal movements. In the central nervous system, it helps regulate mood, appetite, and sleep.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5202#section=Record-Description): A biochemical messenger and regulator, synthesized from the essential amino acid L-TRYPTOPHAN. In humans it is found primarily in the central nervous system, gastrointestinal tract, and blood platelets. Serotonin mediates several important physiological functions including neurotransmission, gastrointestinal motility, hemostasis, and cardiovascular integrity. Multiple receptor families (RECEPTORS, SEROTONIN) explain the broad physiological actions and distribution of this biochemical mediator.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Serotonin): Most of the body's serotonin—about 90%—is synthesized in the gastrointestinal tract by enterochromaffin cells, where it regulates intestinal movements. It is also produced in smaller amounts in the brainstem's raphe nuclei, the skin's Merkel cells, pulmonary neuroendocrine cells, and taste receptor cells of the tongue. Once secreted, serotonin is taken up by platelets in the blood, which release it during clotting to promote vasoconstriction and platelet aggregation. Around 8% of the body's serotonin is stored in platelets, and 1–2% is found in the CNS.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Serotonin): Serotonin, also known as 5-hydroxytryptamine (5-HT), is a monoamine neurotransmitter with a wide range of functions in both the central nervous system (CNS) and also peripheral tissues. It is involved in mood, cognition, reward, learning, memory, and physiological processes such as vomiting and vasoconstriction. In the CNS, serotonin regulates mood, appetite, and sleep.

</details>

**Weird fact** · written · 12.6 s

> **A parasite makes you feel sick with serotonin**  
> The parasite Entamoeba histolytica secretes serotonin, causing sustained secretory diarrhea. It also becomes more virulent when exposed to serotonin. Patients infected with it have highly elevated serotonin levels that return to normal after recovery.

<details><summary>Sources</summary>

- [Wikipedia, Comparative biology & evolution](https://en.wikipedia.org/wiki/Serotonin#Comparative_biology_&_evolution): Serotonin is used by a variety of single-cell organisms for various purposes. SSRIs have been found to be toxic to algae. The gastrointestinal parasite Entamoeba histolytica secretes serotonin, causing a sustained secretory diarrhea in some people. Patients infected with E. histolytica have been found to have highly elevated serum serotonin levels, which returned to normal following resolution of the infection. E. histolytica also responds to the presence of serotonin by becoming more virulent.

</details>


### sodium chloride (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Sodium Chloride carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · trimmed · 14.7 s

> **In your salt shaker and roads**  
> Sodium chloride is found in sea salt, obtained from evaporation of seawater, and in table salt, mined from underground deposits and often treated with an additive to prevent clumping. Sodium chloride is used in de-icing roads in sub-freezing weather. A 0.9% solution of sodium chloride is known as physiological saline.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5234#section=Record-Description): Sea salt is the salt obtained from the evaporation of seawater or water from saltwater lakes. Sea salt production is subject to little processing which leaves certain trace minerals and elements behind. In comparison, table salt is mined from underground sedimentary deposits and is more heavily processed to eliminate minerals. Unlike sea salt, table salt usually contains an additive to prevent clumping and involves addition of iodine. Sea salt is a food ingredient and is often colored by adding charcoal or red clay to sometimes be referred to as “Hawaiian Sea Salt.”
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Sodium_chloride): Sodium chloride , is an ionic compound with the chemical formula NaCl, representing a 1:1 ratio of sodium and chloride ions. It is transparent or translucent, brittle, hygroscopic, and occurs as the mineral halite. In its edible form, common table salt, it is commonly used as a condiment and food preservative. Large quantities of sodium chloride are used in many industrial processes, and it is a major source of sodium and chlorine compounds used as feedstocks for further chemical syntheses. Another major application of sodium chloride is de-icing of roadways in sub-freezing weather.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5234#section=Record-Description): Saline is a solution of salt and water. Saline usually refers to normal or physiological saline, which is an aqueous solution containing 0.9% sodium chloride.

</details>

**Weird fact** · written · 8.6 s

> **Salt forms clouds at sea**  
> Sodium chloride particles in the atmosphere act as cloud condensation nuclei far out at sea. These tiny salt particles enable cloud formation in otherwise non-polluted air.

<details><summary>Sources</summary>

- [Wikipedia, Occurrence](https://en.wikipedia.org/wiki/Sodium_chloride#Occurrence): Salt is found in the Earth's crust as the mineral halite (rock salt), and a tiny amount exists as suspended sea salt particles in the atmosphere. These particles are the dominant cloud condensation nuclei far out at sea, which allow the formation of clouds in otherwise non-polluted air.

</details>


### strychnine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Strychnine carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · written · 12.4 s

> **First discovered in a bean**  
> In 1818, French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier first discovered strychnine in Saint-Ignatius' bean (Strychnos ignatii). The compound was isolated from plants in the genus Strychnos, which had been known since ancient times for their toxic effects. Historic records suggest strychnine-containing preparations were used as early as 1640 to kill animals and humans.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Strychnine#History): Strychnine was first discovered by French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier in 1818 in the Saint-Ignatius' bean. In some Strychnos plants a 9,10-dimethoxy derivative of strychnine, the alkaloid brucine, is also present. Brucine is not as poisonous as strychnine. Historic records indicate that preparations containing strychnine (presumably) had been used to kill dogs, cats, and birds in Europe as far back as 1640. It was allegedly used by convicted murderer William Palmer to kill his final victim, John Cook. It was also used during World War II by Oskar Dirlewanger against civilians.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Strychnine#History): The toxic and medicinal effects of Strychnos nux-vomica have been well known from the times of ancient India, although the chemical compound itself was not identified and characterized until the 19th century. The inhabitants of these countries had historical knowledge of the species Strychnos nux-vomica and Saint-Ignatius' bean (Strychnos ignatii). Strychnos nux-vomica is a tree native to the tropical forests on the Malabar Coast in Southern India, Sri Lanka and Indonesia, which attains a height of about 12 metres (39 ft). The tree has a crooked, short, thick trunk and the wood is close grained and very durable.
- [Wikipedia, History](https://en.wikipedia.org/wiki/Strychnine#History): Strychnine was the first alkaloid to be identified in plants of the genus Strychnos, family Loganiaceae. Strychnos, named by Carl Linnaeus in 1753, is a genus of trees and climbing shrubs of the Gentianales order. The genus contains 196 various species and is distributed throughout the warm regions of Asia (58 species), America (64 species), and Africa (75 species). The seeds and bark of many plants in this genus contain strychnine.

</details>

**Where you meet it** · written · 11.5 s

> **In seeds of the Strychnos tree**  
> Strychnine is found in the seeds of the Strychnos nux-vomica tree and also in Strychnos ignatii and Strychnos wallichiana. It is used as a pesticide to kill birds and rodents. When inhaled, swallowed, or absorbed through the eyes or mouth, it causes muscular convulsions and death through asphyxia.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/441071#section=Record-Description): strychnine has been reported in Strychnos ignatii, Strychnos wallichiana, and other organisms with data available.
- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/441071#section=Record-Description): Strychnine is a plant toxin most commonly found in the seeds of the Strychnine tree (Strychnos nux-vomica L.). It is used as a pesticide, particularly for killing small vertebrates such as birds and rodents. Strychnine causes muscular convulsions and eventually death through asphyxia or sheer exhaustion. (L1230)
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Strychnine): Strychnine is a highly toxic, colorless, bitter, crystalline alkaloid used as a pesticide, particularly for killing small vertebrates such as birds and rodents. Strychnine, when inhaled, swallowed, or absorbed through the eyes or mouth, causes poisoning which results in muscular convulsions and eventually death through asphyxia. While it is no longer used medicinally, it was used historically in small doses to strengthen muscle contractions, such as a heart and bowel stimulant and performance-enhancing drug. The most common source is from the seeds of the Strychnos nux-vomica tree.

</details>

**Weird fact** · trimmed · 13.1 s

> **Used to kill civilians in WWII**  
> Strychnine was allegedly used by Oskar Dirlewanger during World War II against civilians. It was also reportedly used by William Palmer to kill John Cook.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Strychnine#History): Strychnine was first discovered by French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier in 1818 in the Saint-Ignatius' bean. In some Strychnos plants a 9,10-dimethoxy derivative of strychnine, the alkaloid brucine, is also present. Brucine is not as poisonous as strychnine. Historic records indicate that preparations containing strychnine (presumably) had been used to kill dogs, cats, and birds in Europe as far back as 1640. It was allegedly used by convicted murderer William Palmer to kill his final victim, John Cook. It was also used during World War II by Oskar Dirlewanger against civilians.

</details>


### sucralose (4 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Sucralose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · trimmed · 14.1 s

> **A sweet taste from a chlorinated sugar**  
> Phadnis was told to 'test' a chlorinated sugar compound and, according to an anecdotal account, thought Hough asked him to 'taste' it, so he did and found it exceptionally sweet. The substance was later patented by Tate & Lyle in 1976.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Sucralose#History): Sucralose was discovered in 1975 by scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College (now part of King's College London). While researching novel uses of sucrose and its synthetic derivatives, mixing sulfuryl chloride with sugar, Phadnis was told to "test" a chlorinated sugar compound. According to an anecdotal account, Phadnis thought Hough asked him to "taste" it, so he did and found the compound to be exceptionally sweet. Tate & Lyle patented the substance in 1976; as of 2008, the only remaining patents concerned specific manufacturing processes.

</details>

**Where you meet it** · written · 8.9 s

> **Sweetness in your coffee and candy**  
> Sucralose is found in coffee pods, breakfast bars, soft drinks, and canned fruits where it replaces corn syrup-based additives. It's also in Splenda, a powdered sweetener mixed with dextrose and maltodextrin, where sucralose makes up about 1.1% of the blend.

<details><summary>Sources</summary>

- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Sucralose#Uses): Sucralose is used as a replacement for (or in combination with) other artificial or natural sweeteners such as aspartame, acesulfame potassium or high-fructose corn syrup. It is used in products such as candy, breakfast bars, coffee pods, and soft drinks. It is also used in canned fruits wherein water and sucralose take the place of much higher-energy corn syrup-based additives. Sucralose mixed with dextrose (glucose) or maltodextrin (both made from corn) as bulking agents is sold internationally by McNeil Nutritionals under the Splenda brand name.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/71485#section=Uses): Add sweetness with or without the extra calories ... /in/ beverages, baked goods, confections, table-top sugar, substitutes, many processed foods
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Sucralose#Uses): Sucralose is used in many food and beverage products because it is a non-nutritive sweetener (14 kilojoules [3.3 kcal] per typical one-gram serving), does not promote dental cavities, is safe for consumption by diabetics and nondiabetics and does not affect insulin levels. The powdered form of the sucralose-based sweetener product Splenda contains two bulking agents: dextrose and maltodextrin. Sucralose content is about 1.1% and the remainder is bulking agents.

</details>

**Weird fact** · trimmed · 12.9 s

> **He tasted a sweet sugar compound**  
> Shashikant Phadnis was told to 'test' a chlorinated sugar compound. He thought it meant to taste it and found it exceptionally sweet.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Sucralose#History): Sucralose was discovered in 1975 by scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College (now part of King's College London). While researching novel uses of sucrose and its synthetic derivatives, mixing sulfuryl chloride with sugar, Phadnis was told to "test" a chlorinated sugar compound. According to an anecdotal account, Phadnis thought Hough asked him to "taste" it, so he did and found the compound to be exceptionally sweet. Tate & Lyle patented the substance in 1976; as of 2008, the only remaining patents concerned specific manufacturing processes.

</details>


### sucrose (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Sucrose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · skipped · 1.3 s

**Where you meet it** · retried · 15.8 s

> **In fruits, nectars, and sweets**  
> Sucrose is found in plant roots, fruits, and nectars, where it stores energy from photosynthesis. It is widely used in confectionery, desserts, and as a sweetener in biscuits, cookies, cakes, pies, candy, ice cream, and sorbets. It also acts as a food preservative in fruit preserves and is common in many processed and so-called 'junk foods'.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5988#section=Record-Description): A nonreducing disaccharide composed of glucose and fructose linked via their anomeric carbons. It is obtained commercially from sugarcane, sugar beet (beta vulgaris), and other plants and used extensively as a food and a sweetener.
- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Sucrose#Sources): In nature, sucrose is present in many plants, and in particular their roots, fruits and nectars, because it serves as a way to store energy, primarily from photosynthesis. Many mammals, birds, insects and bacteria accumulate and feed on the sucrose in plants and for some it is their main food source. Although honeybees consume sucrose, the honey they produce consists primarily of fructose and glucose, with only trace amounts of sucrose.
- [Wikipedia, Consumption](https://en.wikipedia.org/wiki/Sucrose#Consumption): Sucrose forms a major element in confectionery and desserts. Cooks use it for sweetening. It can also act as a food preservative when used in sufficient concentrations, and thus is an important ingredient in the production of fruit preserves. Sucrose is important to the structure of many foods, including biscuits and cookies, cakes and pies, candy, and ice cream and sorbets. It is a common ingredient in many processed and so-called "junk foods".

</details>

**Weird fact** · written · 11.2 s

> **Sucrose has no free reducing end**  
> Unlike most disaccharides, sucrose's glycosidic bond links the reducing ends of both glucose and fructose. This means sucrose has no free reducing end, making it non-reducing. Glucose is in the α form and fructose in the β-D-fructofuranose form in sucrose.

<details><summary>Sources</summary>

- [Wikipedia, Physical and chemical properties](https://en.wikipedia.org/wiki/Sucrose#Physical_and_chemical_properties): Sucrose's IUPAC name is β-D-fructofuranosyl-(2→1)-α-D-glucopyranoside. In this disaccharide, glucose and fructose are linked via a glycosidic linkage, i.e. an ether bond between C1 on the glucosyl subunit and C2 on the fructosyl unit. Glucose exists predominantly as a mixture of α and β "pyranose" anomers, but sucrose has only the α form. Fructose exists as a mixture of five tautomers but sucrose has only the β-D-fructofuranose form. Unlike most disaccharides, the glycosidic bond in sucrose is formed between the reducing ends of both glucose and fructose, and not between the reducing end of one and the non-reducing end of the other.

</details>


### testosterone (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Testosterone carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories. A minority of company reports add 5 more hazards, not listed here.

**Origin story** · trimmed · 6.4 s

> **From fowl to self-injection**  
> In 1889, Charles-Édouard Brown-Séquard self-injected an extract of dog and guinea pig testicles, reporting restored vigor but transient effects. His hopes for the compound were ultimately dashed.

<details><summary>Sources</summary>

- [Wikipedia, History and production](https://en.wikipedia.org/wiki/Testosterone#History_and_production): A testicular action was linked to circulating blood fractions – now understood to be a family of androgenic hormones – in the early work on castration and testicular transplantation in fowl by Arnold Adolph Berthold (1803–1861). Research on the action of testosterone received a brief boost in 1889, when the Harvard professor Charles-Édouard Brown-Séquard (1817–1894), then in Paris, self-injected subcutaneously a "rejuvenating elixir" consisting of an extract of dog and guinea pig testicle. He reported in The Lancet that his vigor and feeling of well-being were markedly restored but the effects were transient, and Brown-Séquard's hopes for the compound were dashed.
- [Wikipedia, History and production](https://en.wikipedia.org/wiki/Testosterone#History_and_production): In 1927, the University of Chicago's Professor of Physiologic Chemistry, Fred C. Koch, established easy access to a large source of bovine testicles – the Chicago stockyards – and recruited students willing to endure the tedious work of extracting their isolates.

</details>

**Where you meet it** · written · 9.3 s

> **Where testosterone shows up**  
> Testosterone is produced in the testes of males and, to a lesser extent, in the ovaries of females. It drives male reproductive tissue development and secondary traits like muscle mass and body hair. In both sexes, it influences mood, behavior, metabolism, and bone health.

<details><summary>Sources</summary>

- [PubChem, Natural Pollution Sources](https://pubchem.ncbi.nlm.nih.gov/compound/6013#section=Natural-Pollution-Sources): Testosterone is a principal hormone of the testes produced in interstitial cells and required for male sexual differentiation(1).
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Testosterone): Testosterone is the primary male sex hormone and androgen in males. In humans, testosterone drives the development of male reproductive tissues such as testicles and prostate, as well as promoting secondary sexual characteristics such as increased muscle and bone mass, and the growth of body hair. It is associated with increased aggression, sex drive, dominance, courtship display, and a wide range of behavioral characteristics. In addition, testosterone in both sexes is involved in health and well-being, where it has a significant effect on overall mood, cognition, social and sexual behavior, metabolism and energy output, the cardiovascular system, and in the prevention of osteoporosis.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Testosterone): Testosterone is a steroid hormone from the androstane class containing a ketone and a hydroxyl group at positions three and seventeen respectively. It is biosynthesized in several steps from cholesterol and is converted in the liver to inactive metabolites. It exerts its action through binding to and activation of the androgen receptor. In humans and most other vertebrates, testosterone is secreted primarily by the testicles of males and, to a lesser extent, the ovaries of females. On average, in adult males, levels of testosterone are about seven to eight times as great as in adult females.

</details>

**Weird fact** · dropped · 12.6 s

Problems: not stated in the cited snippets: "Jawless fish use a different hormone"; not stated in the cited snippets: "This shows that sex hormones evolved differently across species."


### tetrodotoxin (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Tetrodotoxin carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 12.5 s

> **In pufferfish, octopuses, and newts**  
> Tetrodotoxin is found in the liver and sex organs of pufferfish, globe fish, and toadfish. It also appears in blue-ringed octopuses, rough-skinned newts, and moon snails. The toxin is produced by bacteria like Pseudoalteromonas, Pseudomonas, and Vibrio in symbiotic relationships.

<details><summary>Sources</summary>

- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/11174599#section=Uses): Tetrodotoxin is an extremely potent poison (toxin) found mainly in the liver and sex organs (gonads) of some fish, such as puffer fish, globefish, and toadfish (order Tetraodontiformes) and in some amphibian, octopus, and shellfish species. Human poisonings occur when the flesh and/or organs of the fish are improperly prepared and eaten. Tetrodotoxin interferes with the transmission of signals from nerves to muscles and causes an increasing paralysis of the muscles of the body. Tetrodotoxin poisoning can be fatal.
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Tetrodotoxin): Tetrodotoxin (TTX) is a potent neurotoxin. Its name derives from Tetraodontiformes, an order that includes pufferfish, porcupinefish, ocean sunfish, and triggerfish; several of these species carry the toxin. Although tetrodotoxin was discovered in these fish, it is found in several other animals (e.g., in blue-ringed octopuses, rough-skinned newts, and moon snails). It is also produced by certain infectious or symbiotic bacteria like Pseudoalteromonas, Pseudomonas, and Vibrio as well as other species found in symbiotic relationships with animals and plants.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/11174599#section=Uses): Toxin from the ovaries and liver of Tetraodontidae species (especially the globe fish); [Merck Index] Used for physiological studies; [HSDB] Very potent toxin from the liver and sex organs of fish (i.e., puffer fish, globe fish, and toadfish); Also found in some amphibians, octopus, and shellfish; [Reference #1]

</details>

**Weird fact** · written · 10.2 s

> **Pufferfish poison made by bacteria**  
> Tetrodotoxin, found in pufferfish and other animals, is actually produced by bacteria like Vibrio alginolyticus and Pseudoalteromonas tetraodonis. There is no known antidote for this neurotoxin.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/11174599#section=Record-Description): Tetrodotoxin is a neurotoxin with potential analgesic activity. Tetrodotoxin binds to the pores of fast voltage-gated fast sodium channels in nerve cell membranes, inhibiting nerve action potentials and blocking nerve transmission. Although found in various species of fish (such as the pufferfish), newts, frogs, flatworms, and crabs, tetrodotoxin, for which there is no known antidote, is actually produced by bacteria such as Vibrio alginolyticus, Pseudoalteromonas tetraodonis, and other vibrio and pseudomonas bacterial species.

</details>


### thalidomide (3 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> Thalidomide carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Where you meet it** · written · 5.4 s

> **In medicines for skin and blood cancer**  
> Thalidomide is used to treat the skin symptoms of moderate to severe erythema nodosum leprosum (ENL) and for maintenance therapy to prevent ENL recurrence. It is also used to treat multiple myeloma.

<details><summary>Sources</summary>

- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/5426#section=Uses): For the acute treatment of the cutaneous manifestations of moderate to severe erythema nodosum leprosum (ENL). Also for use as maintenance therapy for prevention and suppression of the cutaneous manifestations of ENL recurrence. Used to treat multiple myeloma.

</details>

**Weird fact** · written · 6.9 s

> **Once a sleeping pill, now a cancer drug**  
> Thalidomide was originally introduced as a non-barbiturate hypnotic. It was withdrawn due to teratogenic effects but has since been reintroduced for treating inflammatory disorders and cancers. Its use requires patients to enroll in the THALIDOMID Risk Evaluation and Mitigation Strategy (REMS) program to ensure contraception adherence.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5426#section=Record-Description): A piperidinyl isoindole originally introduced as a non-barbiturate hypnotic, thalidomide was withdrawn from the market due to teratogenic effects. It has been reintroduced and used for a number of inflammatory disorders and cancers. Thalidomide displays immunosuppressive and anti-angiogenic activity through modulating the release of inflammatory mediators like tumor necrosis factor-alpha (TNF-a) and other cytokine action. Due to severe teratogenicity, pregnancy must be excluded before the start of treatment and patients must enrol in the THALIDOMID Risk Evaluation and Mitigation Strategy (REMS) program to ensure contraception adherence.

</details>


### theobromine (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Theobromine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 2 more hazards, not listed here.

**Origin story** · written · 4.6 s

> **Discovered in cacao beans**  
> In 1841, chemist A. Woskresensky discovered theobromine in cacao beans. This discovery was followed by the first synthesis of theobromine from xanthine in 1882 by Hermann Emil Fischer.

<details><summary>Sources</summary>

- [Wikipedia, History](https://en.wikipedia.org/wiki/Theobromine#History): Theobromine was discovered in 1841 in cacao beans by the chemist A. Woskresensky. Synthesis of theobromine from xanthine was first reported in 1882 by Hermann Emil Fischer.

</details>

**Where you meet it** · retried · 14.3 s

> **In chocolate and cocoa**  
> Theobromine is found in cocoa and chocolate, with higher concentrations in dark than milk chocolate. Cocoa beans naturally contain about 1% theobromine. A 28-gram (1 oz) serving of milk chocolate contains about 60 milligrams, while the same amount of dark chocolate contains about 200 milligrams.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/5429#section=Record-Description): Theobromine is a dimethylxanthine having the two methyl groups located at positions 3 and 7. A purine alkaloid derived from the cacao plant, it is found in chocolate, as well as in a number of other foods, and is a vasodilator, diuretic and heart stimulator. It has a role as a bronchodilator agent, a vasodilator agent, an adenosine receptor antagonist, a mouse metabolite, a plant metabolite, a food component and a human blood serum metabolite.
- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Theobromine#Sources): Theobromine is the primary alkaloid found in cocoa and chocolate. Cocoa butter only contains trace amounts of theobromine. There are usually higher concentrations in dark than in milk chocolate.
- [Wikipedia, Sources](https://en.wikipedia.org/wiki/Theobromine#Sources): There are approximately 60 milligrams (1 grain) of theobromine in 28 grams (1 oz) of milk chocolate, while the same amount of dark chocolate contains about 200 milligrams (3 grains). Cocoa beans naturally contain approximately 1% theobromine.

</details>

**Weird fact** · written · 9.9 s

> **No bromine, yet it's called theobromine**  
> Theobromine contains no bromine, despite its name. The name comes from Greek 'bromos' meaning 'stench', not the element bromine.

<details><summary>Sources</summary>

- [Wikipedia, Etymology](https://en.wikipedia.org/wiki/Theobromine#Etymology): Despite its name, the compound contains no bromine, which is based on Greek bromos ("stench").

</details>


### trinitrotoluene (2 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Danger**  
> 2,4,6-Trinitrotoluene carries "Danger", the stronger of the two GHS signal words, used for the more severe hazard categories.

**Origin story** · dropped · 10.2 s

Problems: number 2 is not in the cited snippets; number 4 is not in the cited snippets; number 6 is not in the cited snippets

**Where you meet it** · trimmed · 9.8 s

> **In shells, mines, and fracked wells**  
> TNT is found in military shells, grenades, and airborne demolition bombs. It is used in industrial and mining blasting, including underwater and deep-well applications.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/8376#section=Record-Description): 2,4,6-Trinitrotoluene is a yellow, odorless solid that does not occur naturally in the environment. It is commonly known as TNT and is an explosive used in military shells, bombs, and grenades, in industrial uses, and in underwater blasting. 2,4,6-Trinitrotoluene production in the United States occurs solely at military arsenals.
- [Wikipedia, Applications](https://en.wikipedia.org/wiki/TNT#Applications): TNT is one of the most commonly used explosives for military, industrial, and mining applications. TNT has been used in oil and gas fracking (hydraulic fracturing) in shale formations. The technique involves displacing and detonating nitroglycerin in hydraulically induced fractures followed by wellbore shots using pelletized TNT.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/8376#section=Uses): 2,4,6-Trinitrotoluene is used as a high explosive in military and industrial applications. It has been widely used for filling shells, grenades and airborne demolition bombs, as it is sufficiently insensitive to the shock of ejection from a gun barrel but can be exploded on impact by a detonator mechanism. It has been used either as the pure explosive or in binary mixtures ... Small amounts of 2,4,6-trinitrotoluene have been used for industrial explosive applications, such as deep-well and underwater blasting ...

</details>

**Weird fact** · dropped · 10.7 s

Problems: number 2 is not in the cited snippets; number 4 is not in the cited snippets; number 6 is not in the cited snippets


### vanillin (4 cards)

**Danger level** · from data · 0.0 s

> **Official signal word: Warning**  
> Vanillin carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories. A minority of company reports add 1 more hazard, not listed here.

**Origin story** · written · 8.0 s

> **From Mesoamerica to Germany**  
> Native Mesoamerican peoples, especially the Totonacs of modern-day Veracruz, Mexico, discovered and cultivated vanilla as a flavoring for beverages by the 15th century. The Aztecs used it in chocolate drinks called xocohotl. In 1874, Ferdinand Tiemann and Wilhelm Haarmann deduced vanillin's structure and began industrial production in Holzminden, Germany, using the Reimer–Tiemann reaction.

<details><summary>Sources</summary>

- [Wikipedia, Natural history](https://en.wikipedia.org/wiki/Vanillin#Natural_history): Although it is generally accepted that vanilla was domesticated in Mesoamerica and subsequently spread to the Old World in the 16th century, in 2019, researchers published a paper stating that vanillin residue had been discovered inside jars within a tomb in Canaan dating to the 2nd millennium BCE, suggesting the possible cultivation of an unidentified, Old World-endemic Vanilla species in Canaan since the Middle Bronze Age. Traces of vanillin were also found in wine jars in Jerusalem, which were used by the Judahite elite before the city was destroyed in 586 BCE.
- [Wikipedia, Natural history](https://en.wikipedia.org/wiki/Vanillin#Natural_history): Vanilla beans, called tlilxochitl, were discovered and cultivated as a flavoring for beverages by native Mesoamerican peoples, most famously the Totonacs of modern-day Veracruz, Mexico. Since at least the early 15th century, the Aztecs used vanilla as a flavoring for chocolate in drinks called xocohotl.
- [Wikipedia, Synthetic history](https://en.wikipedia.org/wiki/Vanillin#Synthetic_history): In 1874, the German scientists Ferdinand Tiemann and Wilhelm Haarmann deduced its chemical structure, at the same time finding a synthesis for vanillin from coniferin, a glucoside of coniferyl alcohol found in pine bark. Tiemann and Haarmann founded a company Haarmann and Reimer (now part of Symrise) and started the first industrial production of vanillin using their process (now known as the Reimer–Tiemann reaction) in Holzminden, Germany.

</details>

**Where you meet it** · trimmed · 10.3 s

> **Scent of chocolate and ice cream**  
> Vanillin is the main flavor in vanilla bean extract and is widely used in sweet foods. Synthetic vanillin is now more common than natural extract in foods, beverages, and pharmaceuticals.

<details><summary>Sources</summary>

- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Vanillin): Vanillin is an organic compound with the molecular formula C8H8O3. It is a phenolic aldehyde. Its functional groups include aldehyde, hydroxyl, and ether. It is the primary component of the ethanolic extract of the vanilla bean. Synthetic vanillin is now used more often than natural vanilla extract as a flavoring in foods, beverages, and pharmaceuticals.
- [Wikipedia, Uses](https://en.wikipedia.org/wiki/Vanillin#Uses): The largest use of vanillin is as a flavoring, usually in sweet foods. The ice cream and chocolate industries together comprise 75% of the market for vanillin as a flavoring, with smaller amounts being used in confections and baked goods.
- [Wikipedia, Natural history](https://en.wikipedia.org/wiki/Vanillin#Natural_history): Vanilla beans, called tlilxochitl, were discovered and cultivated as a flavoring for beverages by native Mesoamerican peoples, most famously the Totonacs of modern-day Veracruz, Mexico. Since at least the early 15th century, the Aztecs used vanilla as a flavoring for chocolate in drinks called xocohotl.

</details>

**Weird fact** · written · 10.2 s

> **Vanillin in 2nd millennium BCE jars**  
> Vanillin residue was found in jars within a tomb in Canaan dating to the 2nd millennium BCE, suggesting possible cultivation of an unidentified Vanilla species there since the Middle Bronze Age. Traces were also found in wine jars in Jerusalem used by the Judahite elite before 586 BCE.

<details><summary>Sources</summary>

- [Wikipedia, Natural history](https://en.wikipedia.org/wiki/Vanillin#Natural_history): Although it is generally accepted that vanilla was domesticated in Mesoamerica and subsequently spread to the Old World in the 16th century, in 2019, researchers published a paper stating that vanillin residue had been discovered inside jars within a tomb in Canaan dating to the 2nd millennium BCE, suggesting the possible cultivation of an unidentified, Old World-endemic Vanilla species in Canaan since the Middle Bronze Age. Traces of vanillin were also found in wine jars in Jerusalem, which were used by the Judahite elite before the city was destroyed in 586 BCE.

</details>


### water (3 cards)

**Danger level** · from data · 0.0 s

> **Not classified as hazardous**  
> Water is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

**Origin story** · skipped · 1.5 s

**Where you meet it** · written · 10.6 s

> **Where you find it everywhere**  
> Water is the main constituent of Earth's oceans, lakes, rivers, and rain. It is the fluid inside all living organisms and appears in daily activities like drinking, cooking, bathing, and gardening. It also powers industries, agriculture, and recreation such as fishing and boating.

<details><summary>Sources</summary>

- [PubChem, Record Description](https://pubchem.ncbi.nlm.nih.gov/compound/962#section=Record-Description): Water (chemical formula: H2O) is a transparent fluid which forms the world's streams, lakes, oceans and rain, and is the major constituent of the fluids of organisms. As a chemical compound, a water molecule contains one oxygen and two hydrogen atoms that are connected by covalent bonds. Water is a liquid at standard ambient temperature and pressure, but it often co-exists on Earth with its solid state, ice; and gaseous state, steam (water vapor).
- [Wikipedia, Introduction](https://en.wikipedia.org/wiki/Water): Water is an inorganic compound with the chemical formula H2O. It is a transparent, tasteless, odorless, and nearly colorless chemical substance. It is the main constituent of Earth's streams, lakes, and oceans. Water is also the fluid of all known living organisms, in which it acts as a solvent. Water, being a polar molecule, undergoes strong intermolecular hydrogen bonding which is a large contributor to its physical and chemical properties. It is vital for all known forms of life, despite not providing food energy or being an organic micronutrient.
- [PubChem, Uses](https://pubchem.ncbi.nlm.nih.gov/compound/962#section=Uses): Drinking, culinary purposes, bathing, washing, laundering, heating, air conditioning, agriculture, stock raising and gardens, for industrial processes and cooling, for water power and steam power, for fire protection, for disposal of wastes, for fishing, swimming, boating and other recreational purposes, for fish and wildlife propagation, and for navigation

</details>

**Weird fact** · written · 9.1 s

> **Water vapor in a distant quasar cloud**  
> A cloud of water vapor around a quasar 12 billion light years away contained 140 trillion times more water than all of Earth's oceans combined. This discovery shows water has been prevalent in the universe for nearly its entire existence.

<details><summary>Sources</summary>

- [Wikipedia, Distribution in nature](https://en.wikipedia.org/wiki/Water#Distribution_in_nature): On 22 July 2011, a report described the discovery of a gigantic cloud of water vapor containing "140 trillion times more water than all of Earth's oceans combined" around a quasar located 12 billion light years from Earth. According to the researchers, the "discovery shows that water has been prevalent in the universe for nearly its entire existence".

</details>

