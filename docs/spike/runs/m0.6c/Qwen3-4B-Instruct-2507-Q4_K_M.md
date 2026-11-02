# Spike run: Qwen3-4B-Instruct-2507-Q4_K_M

30 molecules, 111 cards with input. Temperature 0.2, JSON schema constraint, one stricter retry, shortest card first, pick-then-write for Weird and Where, Danger from GHS data, KV cache: paged, fixed 4K tokens.

## Summary

| Measure | Value |
| --- | --- |
| From GHS data (no model) | 28 (25%) |
| Passed first try | 74 (67%) |
| Passed on retry | 4 (4%) |
| Skipped by model | 2 (2%) |
| Rejected twice (dropped) | 3 (3%) |
| No input (no card) | 9 |
| Molecules with 3+ cards | 28 of 30 |
| First text of a model card | 5.2 s median, 6.7 s p90, 7.5 s max |
| Model card, including pick and retry | 7.7 s median, 10.3 s p90, 14.7 s max |
| First card per molecule | 0.0 s median, 0.0 s p90, 5.0 s max |
| First model card per molecule | 5.6 s median, 9.9 s p90, 23.0 s max |
| Model load | 7.7 s |
| Peak memory | 4.06 GB |

### Problems found (all attempts)

- code not in sources: 1
- name not in sources: 2
- number not in sources: 1
- sentence count: 5
- title: 1
- too long: 1

## Cards

Fun score is for hand review in M0.6.

### allicin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Allicin carries the official GHS signal word "Warning". Its hazard statements: causes skin irritation, causes serious eye irritation, and may cause respiratory irritation.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 5.1 s

> **Discovered in Japan in the 1940s**  
> In 1944, Chester J. Cavallito and John Hays Bailey first isolated and studied allicin in the laboratory. It was discovered during efforts to create thiamine derivatives in Japan. Allicin became a model for developing other thiamine disulfides such as sulbutiamine, fursultiamine, and benfothiamine.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 6.2 s

> **In the smell of crushed garlic**  
> Allicin forms when fresh garlic or leeks are chopped or crushed, triggering a reaction in the enzyme alliinase. You meet it in the distinctive odor of fresh garlic, which quickly turns into other sulfur compounds. It’s also found in nature as a defense mechanism against pests.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 6.9 s

> **Allicin led to thiamine derivatives**  
> Allicin was discovered in the 1940s in Japan while trying to create thiamine derivatives. It became a model for designing compounds like sulbutiamine, fursultiamine, and benfothiamine. These are hydrophobic and pass easily from intestines to blood, then reduced to thiamine by cysteine or glutathione.

Sources: s1 (Wikipedia, History)  
Fun: _ /5


### artemisinin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Artemisinin carries the official GHS signal word "Danger". Its hazard statements: heating may cause a fire, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 13.6 s

> **From ancient remedy to antimalarial discovery**  
> In 1596, Li Shizhen recommended tea made from sweet wormwood to treat malaria. In 1967, a secret military program called Project 523 was launched under the People's Liberation Army at Mao Zedong's order to find a malaria treatment. In 1972, Tu Youyou discovered artemisinin in the leaves of Artemisia annua. She was influenced by a 340 CE text by Ge Hong recommending cold water steeping of the herb.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 81 words, must be under 80</summary>

```json
{"title": "From ancient herb to antimalarial discovery", "body": "In 1596, Li Shizhen recommended tea made from sweet wormwood to treat malaria symptoms in his Compendium of Materia Medica. In 1967, a secret military program called 'Project 523' was launched by the People's Liberation Army under Mao Zedong's order to find a malaria treatment. In 1972, Tu Youyou discovered artemisinin in the leaves of Artemisia annua, inspired by a 340 CE herbal text by Ge Hong. She also found that a low-temperature extraction process could isolate an effective antimalarial substance.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 8.5 s

> **In sweet wormwood and malaria treatment**  
> Artemisinin is found in Artemisia annua, also known as sweet wormwood, sweet annie, sweet sagewort, and annual wormwood. It is used in antimalarial drugs, especially for Plasmodium falciparum malaria, and appears in Artemisinin-based combination therapies (ACTs) as standard treatment worldwide. It has also been used in traditional medicine for malaria, cancer, viral and bacterial infections, and some cardiovasculreviear diseases.

Sources: s1 (Wikipedia, Introduction), s2 (PubChem, Record Description), s3 (PubChem, Record Description)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.0 s

> **Named after a queen botanist?**  
> Artemisinin's genus name may come from Queen Artemisia II of Caria, a fourth-century BC botanist and medical researcher. This is surprising because a plant used for malaria treatment is named after a historical queen who studied medicine.

Sources: s1 (Wikipedia, History)  
Fun: _ /5


### ascorbic acid

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Ascorbic acid is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.2 s

> **Lind's citrus experiment**  
> In May 1747, James Lind, a ship's surgeon in the Royal Navy, conducted one of the first controlled experiments by giving some crew members oranges and lemons while others received cider, vinegar, sulfuric acid, or seawater. He found citrus fruits prevented scurvy. Lind published his findings in 1753 in his Treatise on the Scurvy.

Sources: s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 9.1 s

> **In your citrus and bread**  
> Ascorbic acid is found naturally in citrus fruits and many vegetables. It's added to canned fruits to slow oxidation and browning, and may be used as a flour treatment in breadmaking. As a food additive, it carries E numbers under approval by the European Food Safety Authority.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Food preservation additive)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.1 s

> **Body stops absorbing more after 100 mg**  
> Ascorbic acid levels plateau at 65 μmol/L with intakes of 100 to 200 mg/day. Even higher doses don’t increase blood levels because absorption drops and excess is excreted in urine.

Sources: s1 (Wikipedia, Deficiency)  
Fun: _ /5


### aspirin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Aspirin carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed. A minority of reports list 3 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.4 s

> **First pure form in 1897**  
> In 1897, Felix Hoffmann (or Arthur Eichengrün) of Bayer first produced acetylsalicylic acid in a pure, stable form. This was part of Bayer's effort to create a less-irritating alternative to salicylate medicines. The compound was later branded as Aspirin, though the name was not the generic name.

Sources: s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.9 s

> **In your medicine cabinet and beyond**  
> Aspirin, known as acetylsalicylic acid (ASA), is found in medicines for pain, fever, and inflammation. It's used to treat conditions like Kawasaki disease and pericarditis. Long-term use helps reduce the risk of heart attacks, strokes, and blood clots in high-risk individuals.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.6 s

> **Aspirin dust can be explosive**  
> Aspirin powder in air can form an explosive mixture, like flour dust. Fact: factories must control airborne powder levels. NIOSH and OSHA both set a limit of 5 mg/m3, though OSHA's rule was later vacated in 1993.

Sources: s1 (Wikipedia, Chemical properties)  
Fun: _ /5


### batrachotoxin

**Danger level** · no input · 0 snippets · 0.0 s

**Origin story** · passed · 1 snippets · 5.0 s

> **Discovery of batrachotoxin in frogs**  
> In 1963, Fritz Märki and Bernhard Witkop isolated the toxic alkaloid fraction from the black-legged poison dart frog at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. They identified four major toxic steroidal alkaloids, including batrachotoxin, homobatrachotoxin, pseudobatrachotoxin, and batrachotoxinin A. The process was challenging due to the toxin's potency and the tiny amounts available.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 9.2 s

> **In frogs, birds, and darts**  
> Batrachotoxin is found in frogs of the genus Phyllobates, birds in Pitohui and Ifrita, and beetles in Choresine. It is used by the Noanamá Chocó and Emberá Chocó peoples of western Colombia to poison blowgun darts. The toxin forms in blisters when frogs are roasted or held over fire, and can be collected from the skin or fermented.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Use), s3 (Wikipedia, Use)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.2 s

> **Most potent non-peptidal neurotoxin**  
> Batrachotoxin is the most potent non-peptidal neurotoxin known. It acts directly on sodium channels and is found in poison dart frogs, beetles, and birds. Its extreme toxicity makes it one of the most surprising natural compounds.

Sources: s1 (PubChem, Record Description)  
Fun: _ /5


### benzylpenicillin

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Benzylpenicillin carries the official GHS signal word "Danger". Its hazard statements: may cause an allergic skin reaction, and may cause allergy or asthma symptoms or breathing difficulties if inhaled. A minority of reports list 2 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.4 s

> **In injections for infections**  
> Benzylpenicillin is found in injectable forms used to treat infections like pneumonia, strep throat, syphilis, and gas gangrene. It is administered intravenously or intramuscularly due to poor oral absorption. Common salts include sodium, potassium, benzathine, and procaine.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 6.4 s

> **Found in fungi, not just bacteria**  
> Benzylpenicillin has been reported in Sarocladium strictum and Hohenbuehelia grisea, which are fungi, not just bacteria. This is surprising because penicillin-like compounds are typically associated with bacterial sources.

Sources: s1 (PubChem, Record Description)  
Fun: _ /5


### buckminsterfullerene

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Buckminsterfullerene carries the official GHS signal word "Warning". Its hazard statements: causes serious eye irritation, and may cause respiratory irritation. A minority of reports list 1 other hazard.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 4 snippets · 14.7 s

<details><summary>Attempt 1 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "Laser vaporization sparks discovery", "body": "In 1984, Eric Rohlfing, Donald Cox, and Andrew Kaldor generated buckminsterfullerene using a laser to vaporize carbon in a supersonic helium beam, but did not recognize the structure. In 1985, Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University repeated the experiment and identified C60 as buckminsterfullerene. They found a strong peak at 720 daltons and concluded the structure was a spheroidal, cage-like molecule with icosahedral symmetry.", "sources": ["s1", "s2", "s3", "s4"]}
```
</details>

<details><summary>Attempt 2 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "First made in 1984 by laser vaporization", "body": "In 1984, Eric Rohlfing, Donald Cox, and Andrew Kaldor generated buckminsterfullerene using a laser to vaporize carbon in a supersonic helium beam, though they did not recognize the structure. In 1985, Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University repeated the experiment and identified the C60 structure as buckminsterfullerene. They found a strong peak at 720 daltons and concluded the molecule was a spheroidal, cage-like structure with icosahedral symmetry.", "sources": ["s1", "s2", "s3", "s4"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 8.3 s

> **In soot, space, and ancient rock**  
> Buckminsterfullerene is the most common naturally occurring fullerene, found in small quantities in soot. It also exists in space, where neutral C60 is seen in planetary nebulae and stars, and C+60 causes diffuse interstellar bands. It was first discovered in shungite, a rock containing 99% carbon.

Sources: s1 (Wikipedia, Occurrence), s2 (Wikipedia, Occurrence), s3 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · rejected · 1 snippets · 12.2 s

<details><summary>Attempt 1 rejected: number 60 is not in the cited snippets</summary>

```json
{"title": "C60 is a soccer ball-shaped cage", "body": "Buckminsterfullerene (C60) has a structure resembling a soccer ball — a regular truncated icosahedron. This cage-like shape was discovered by Kroto and Smalley using laser-evaporated graphite. The molecule is made of 60 carbon atoms arranged in a symmetric, hollow structure.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: 1980s is not in the cited snippets</summary>

```json
{"title": "C60 has a soccer ball shape", "body": "Buckminsterfullerene, C60, is a cage-like molecule with a structure resembling a soccer ball. It is a regular truncated icosahedron, a shape first recognized in the 1980s. The Smalley team found it to be the most common cluster among Cn species where n > 20 and even.", "sources": ["s1"]}
```
</details>


### caffeine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Caffeine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · skipped by model · 4 snippets · 1.8 s

**Weird fact** · passed · 1 snippets · 7.3 s

> **Caffeine helps premature babies gain weight**  
> Caffeine is used to treat bronchopulmonary dysplasia in premature infants and may improve weight gain and reduce risks of cerebral palsy and cognitive delay. This use is surprising because caffeine is commonly known as a stimulant for adults, not a medical treatment for babies.

Sources: s1 (Wikipedia, Uses)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 10.3 s

> **Sip it in coffee, tea, or chocolate**  
> Caffeine is found naturally in tea leaves, coffee beans, cocoa beans, mate leaves, guarana paste, and kola nuts. It occurs in plants from Africa, East Asia, and South America, where it acts as a natural pesticide and protects seeds. Humans encounter it in beverages, cosmetics, and as a stimulant in medicines.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (PubChem, Uses)  
Fun: _ /5


### capsaicin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Capsaicin carries the official GHS signal word "Danger". Its hazard statements: harmful if swallowed, causes skin irritation, and causes serious eye damage. A minority of reports list 1 other hazard.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 3.8 s

> **First extracted and studied**  
> In 1816, Christian Friedrich Bucholz first extracted capsaicin in impure form. In 1873, Rudolf Buchheim noted that 'capsicol' caused a burning sensation. In 1878, Endre Hőgyes confirmed that 'capsicol' increased gastric acid secretion.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 8.0 s

> **The heat in your chili**  
> Capsaicin gives hot peppers their spicy taste and is found in fruits like Capsicum annuum and C. frutescens. It's commonly added to chili powder and paprika for heat. The spiciness is measured on the Scoville scale.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Uses), s3 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.4 s

> **Seeds don't make capsaicin, but the pith does**  
> Capsaicin is not produced in the seeds themselves, even though they are the most visible part of the chili. The highest concentration is found in the white pith where seeds are attached, which is surprising because the seeds don’t make it at all.

Sources: s1 (Wikipedia, Natural function)  
Fun: _ /5


### carvone

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Carvone carries the official GHS signal word "Warning". Its hazard statements: may cause an allergic skin reaction.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 1 snippets · 3.8 s

> **Carvone isolated in 1849**  
> In 1849, Franz Varrentrapp obtained carvone from caraway, originally naming it carvol by Schweizer. Goldschmidt and Zürrer identified it as a ketone related to limonene. The structure was finally elucidated by Georg Wagner in 1894.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 10.6 s

> **Sour-sweet in caraway and mint**  
> Carvone is found naturally in caraway seeds (Carum carvi), where it makes up 60–70% of the oil, and in dill seeds (Anethum graveolens) at 40–60%. It is also abundant in spearmint oil (Mentha spicata), which contains 50–80% R-(−)-carvone. This molecule gives flavor to caraway, dill, and spearmint and appears in food, air fresheners, and aromatherapy products.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Occurrence), s3 (Wikipedia, Uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.2 s

> **Squirrel monkeys smell carvone enantiomers**  
> Carvone's two enantiomers smell different, and squirrel monkeys have been found to discriminate between them. This shows animals can detect subtle chemical differences that humans might miss.

Sources: s1 (Wikipedia, Stereoisomerism and odor)  
Fun: _ /5


### cinnamaldehyde

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Cinnamaldehyde carries the official GHS signal word "Warning". Its hazard statements: may cause an allergic skin reaction.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 9.8 s

> **Spicy sweetness in your snacks**  
> Cinnamaldehyde gives cinnamon its signature flavor and aroma, found naturally in the bark of Cinnamomum trees like camphor and cassia. It’s used in chewing gum, ice cream, candy, e-liquid, and beverages at levels from 9 to 4,900 ppm. Also found in perfumes, it can mimic sweet, spicy, and balsamic scents—though it’s sometimes misused to fake cinnamon in products like powdered beechnut husk.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Applications)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.7 s

> **29 ppm kills half the mosquito larvae**  
> Cinnamaldehyde kills half of Aedes aegypti mosquito larvae in just 24 hours at a concentration of 29 ppm. This low, precise dose makes it a surprisingly effective and targeted insecticide.

Sources: s1 (Wikipedia, Applications)  
Fun: _ /5


### cubane

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · no input · 0 snippets · 0.0 s

**Where you meet it** · passed · 1 snippets · 4.2 s

> **In high-energy research labs**  
> Cubane and its derivatives like octanitrocubane and heptanitrocubane are studied as high-performance explosives due to their high energy density and stability. These compounds are used in research for energy storage and as isosteres for benzene rings. Their geometry allows them to fit into molecular frameworks similar to benzene.

Sources: s1 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 3.8 s

> **Cubane refuses to break down**  
> Cubane is highly strained and cannot decompose into cubene because the resulting molecule would be a pyramidal alkene with too high energy. Only certain metallic ions can catalyze a σ-bond rearrangement to cuneane.

Sources: s1 (Wikipedia, Reactions and derivatives)  
Fun: _ /5


### dopamine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Dopamine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed, suspected of damaging fertility or the unborn child, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 4.8 s

> **First synthesized and identified**  
> In 1910, George Barger and James Ewens first synthesized dopamine at Wellcome Laboratories in London, England. It was first identified in the human brain by Katharine Montagu in 1957. The neurotransmitter function of dopamine was first recognized in 1958 by Arvid Carlsson and Nils-Åke Hillarp at the Laboratory for Chemical Pharmacology of the National Heart Institute of Sweden.

Sources: s1 (Wikipedia, History and development)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 8.9 s

> **Where dopamine works in your body**  
> Dopamine is a neurotransmitter in the brain, sending signals between nerve cells. Outside the brain, it acts as a local messenger in blood vessels, kidneys, pancreas, digestive system, and immune cells. It is also used medically as a drug under names like Intropin, Dopastat, and Revimine, especially in newborns with low blood pressure or cardiac arrest.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction), s3 (Wikipedia, Medical uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 7.4 s

> **Animals stole dopamine genes from bacteria**  
> Dopamine-synthesizing enzymes in animals may have come from bacteria via horizontal gene transfer. This suggests animals inherited their dopamine machinery from bacterial ancestors, possibly through early symbiotic relationships that led to mitochondria.

Sources: s1 (Wikipedia, Comparative biology and evolution)  
Fun: _ /5


### ethylene

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Ethylene carries the official GHS signal word "Danger". Its hazard statements: extremely flammable gas, and may cause drowsiness or dizziness.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.3 s

> **Ethylene discovered through heating ethanol**  
> Johann Joachim Becher discovered ethylene in 1669 by heating ethanol with sulfuric acid and mentioned it in his Physica Subterranea. Joseph Priestley reported in 1779 that Jan Ingenhousz saw ethylene synthesized in the same way by a Mr. Enée in Amsterdam in 1777. Ingenhousz later produced the gas himself.

Sources: s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.6 s

> **Ripening fruit and flowers**  
> Ethylene is naturally released by fruits, flowers, leaves, and tubers, especially during ripening or senescence. For example, ripening avocados and apples, and fading morning glory flowers, emit it. It's also used commercially to speed up fruit ripening.

Sources: s1 (Wikipedia, Introduction), s2 (PubChem, Natural Pollution Sources), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.4 s

> **Pythia's trance may have been ethylene**  
> Some geologists and scholars propose that the Pythia’s trance-like state at Delphi was caused by ethylene rising from ground faults. This is a speculative link, but it’s oddly fascinating — a gas from the earth possibly inspiring ancient prophecy.

Sources: s1 (Wikipedia, History)  
Fun: _ /5


### geosmin

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Geosmin carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 1 snippets · 4.5 s

> **The smell after rain**  
> You meet geosmin when it rains after dry weather, creating the scent known as petrichor. It's also in beetroots and made by bacteria in soil and water. Geosmin causes muddy taste and smell in drinking water and farmed fish.

Sources: s1 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 5.5 s

> **Humans smell geosmin at 5 ppt**  
> The human nose can detect geosmin at as low as 5 parts per trillion. This extreme sensitivity may have evolved to help our ancestors find scarce water, though this is not proven. Insects might use it to find food or avoid toxins.

Sources: s1 (Wikipedia, Effects)  
Fun: _ /5


### indigo

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Indigo carries the official GHS signal word "Warning". Its hazard statements: may causes damage to organs through prolonged or repeated exposure.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 5.2 s

> **In your blue jeans and plants**  
> Indigo is found in the leaves of Indigofera tinctoria plants, where it naturally forms a blue pigment. Today, most indigo is synthetic and used in denim production, with a pair of jeans requiring 3–12 grams of dye. It's also used in small amounts for wool and silk dyeing.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction), s3 (Wikipedia, Uses)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.3 s

> **Indigo's ancient roots in India**  
> The oldest known indigo-dyed fabric dates to 6,000 years ago and was found in Huaca Prieta, Peru. The Indigofera tinctoria species was domesticated in India, which became the earliest major center for its production and processing. India was also the source of indigo used by the Greeks and Romans, who valued it as a luxury product.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Weird fact** · passed on retry · 1 snippets · 11.1 s

> **Indigo turns soluble with bacteria's hydrogen**  
> In Japan, indigo was dissolved using thermophilic, anaerobic bacteria that generate hydrogen. This hydrogen converts insoluble indigo into soluble indigo white. The process relies on bacterial metabolism, not chemical reagents. Cloth was then decorated using shibori, kasuri, katazome, and tsutsugaki techniques.

Sources: s1 (Wikipedia, Dyeing technology)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Europeans is not in the cited snippets; name Japanese is not in the cited snippets</summary>

```json
{"title": "Indigo turns white in bacterial urine", "body": "Ancient Europeans dissolved indigo in stale urine, which contains ammonia, to make indigo white. Japanese methods used heat and bacteria that produce hydrogen, converting insoluble indigo into soluble indigo white. This bacterial process is both unusual and surprisingly effective.", "sources": ["s1"]}
```
</details>


### limonene

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Limonene carries the official GHS signal word "Warning". Its hazard statements: flammable liquid and vapor, causes skin irritation, may cause an allergic skin reaction, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 9.9 s

> **The zesty scent in your kitchen**  
> Limonene is the main fragrance in citrus fruit peels, especially lemons and oranges. It appears in food manufacturing as a flavoring agent and in medicines to mask bitter tastes. You’ll also find it in hand cleansers, bath products, and personal care items for its lemony scent and oil-dissolving power.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction), s3 (Wikipedia, Uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 9.5 s

> **Limonene's metabolites fight cancer**  
> Limonene's metabolites like perillic acid and limonene 1,2-diol are suggested to inhibit tumor growth by blocking p21-dependent signaling and inducing apoptosis. This chemo-preventive action is proposed through multiple pathways including G1 cell cycle arrest and gene expression changes.

Sources: s1 (PubChem, Record Description)  
Fun: _ /5


### lycopene

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Lycopene is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 6.7 s

> **The red in your food**  
> Lycopene gives red to pink hues in tomatoes, pink grapefruit, watermelon, red guavas, and baked beans. It's found naturally in these foods and is a major pigment in fruits like apricots and red oranges. As a carotenoid, it may help retard aging and degenerative diseases in animals.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Occurrence)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 7.8 s

> **Most in blood are twisted forms**  
> In human blood, twisted cis-isomers make up more than 60% of lycopene, even though the straight all-trans form is the most stable. This shift happens due to light or heat, and the biological effects of these isomers remain uninvestigated.

Sources: s1 (Wikipedia, Structure and physical properties)  
Fun: _ /5


### menthol

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Menthol is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 1 snippets · 6.3 s

> **First isolated in 1771**  
> In 1771, German chemist Hieronymus David Gaubius first isolated menthol. The molecule was later named in 1861 by F. L. Alphonse Oppenheim.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "First isolated in 1771", "body": "In 1771, German chemist Hieronymus David Gaubius first isolated menthol. Early characterizations were conducted by Oppenheim, Beckett, Moriya, and Atkinson. The name 'menthol' was proposed by F. L. Alphonse Oppenheim in 1861.", "sources": ["s1"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 8.7 s

> **In minty snacks and medicines**  
> Menthol is found naturally in peppermint oil from Mentha × piperita, giving minty flavor and a cooling sensation. It appears in foods and pharmaceuticals as a flavor enhancer and local anesthetic. Japanese menthol also contains a small amount of neomenthol, a 1-epimer.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Occurrence)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.8 s

> **Only one form is naturally found**  
> Natural menthol exists as a single pure stereoisomer: (1R,2S,5R). Out of eight possible stereoisomers, nature almost always selects just this one form.

Sources: s1 (Wikipedia, Structure)  
Fun: _ /5


### morphine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Morphine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed. A minority of reports list 5 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.7 s

> **From opium to morphine in 1804**  
> In December 1804, German pharmacist Friedrich Sertürner extracted morphine from the opium poppy plant in Paderborn. He noticed the compound induced drowsiness in rats and stray dogs and reported his findings in 1805. Sertürner named it morphium after the Greek god of dreams, Morpheus, due to its sleep-inducing effect.

Sources: s3 (Wikipedia, History)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 6.8 s

> **Morphine nearly killed people in early tests**  
> Friedrich Sertürner administered morphine to himself, three boys, three dogs, and a mouse; all four almost died. This shows how potent morphine was even in early experiments.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 10.3 s

> **In poppies and pain meds**  
> Morphine is found naturally in the latex of the opium poppy (Papaver somniferum), making up 8–14% of dry opium. It appears in pain medications administered orally, sublingually, intravenously, or via injection. The molecule is also present in the sap of the poppy plant.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Natural occurrence), s3 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### nicotine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Nicotine carries the official GHS signal word "Danger". Its hazard statements: fatal if swallowed, fatal in contact with skin, fatal if inhaled, and toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.6 s

> **From tobacco to isolation**  
> In 1828, Wilhelm Heinrich Posselt and Karl Ludwig Reimann isolated nicotine from the tobacco plant in Germany, believing it was a poison. They identified its empirical formula in 1843 by Melsens, and its structure was discovered in 1893 by Adolf Pinner and Richard Wolffenstein. Nicotine was first synthesized in 1904 by Amé Pictet and A. Rotschy.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 8.0 s

> **In tobacco and nightshade plants**  
> Nicotine is found in tobacco leaves, making up 0.6–3% of their dry weight. It also occurs in trace amounts in tomatoes, potatoes, and eggplants. People encounter it through smoking, vaping, or nicotine replacement therapies like patches and gums.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · skipped by model · 1 snippets · 5.8 s


### nitroglycerin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Nitroglycerin carries the official GHS signal word "Danger". Its hazard statements: explosive; mass explosion hazard, fatal if swallowed, fatal in contact with skin, fatal if inhaled, and may causes damage to organs through prolonged or repeated exposure. The official list has 1 more.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.0 s

> **First made in 1846 by Sobrero**  
> In 1846, Italian chemist Ascanio Sobrero synthesized nitroglycerin while working under Théophile-Jules Pelouze at the University of Turin. He named it 'pyroglycerin' and warned against its use as an explosive. The compound was later adopted by Alfred Nobel for commercial use after a deadly explosion in 1864.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 10.3 s

> **In medicine and in dynamite**  
> Nitroglycerin is prescribed for angina pectoris and as an antihypertensive, where it dilates veins to reduce heart preload. It is also found in formulations like Isordil and Imdur, converted to nitric oxide in the body. Historically, Alfred Nobel mixed it with diatomaceous earth to create dynamite, supplied as sticks wrapped in greased paper, used in tunnel drilling, mining, and demolition.

Sources: s1 (Wikipedia, Medical use), s2 (Wikipedia, Medical use), s3 (Wikipedia, Use as an explosive and a propellant)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 7.7 s

> **A shipment exploded in San Francisco**  
> In April 1866, a crate of nitroglycerin exploded in San Francisco, destroying a Wells Fargo office and killing 15 people. This incident led to a complete ban on transporting liquid nitroglycerin in California.

Sources: s1 (Wikipedia, History)  
Fun: _ /5


### quinine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Quinine carries the official GHS signal word "Danger". Its hazard statements: may cause an allergic skin reaction, and may cause allergy or asthma symptoms or breathing difficulties if inhaled. A minority of reports list 3 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.2 s

> **From Quechua bark to Jesuit testing**  
> The Quechua Indians of Peru, Bolivia, and Ecuador used ground cinchona bark mixed with sweetened water to treat shivering. Spanish Jesuit missionaries, including Agostino Salumbrino, observed this practice and sent cinchona bark to Rome for testing in the 17th century. Salumbrino, an apothecary in Lima, sent a small quantity to Rome to evaluate its effect on malaria-induced shivering.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 8.8 s

> **In tonic water and malaria treatment**  
> Quinine appears in tonic water and bitter lemon to give a bitter taste. It is used medically to treat malaria, especially in areas with resistance to chloroquine, and for nocturnal leg cramps—though side effects are a concern. It is also found in medicines as an alkaloid or salt.

Sources: s1 (Wikipedia, Introduction), s2 (PubChem, Uses), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 7.3 s

> **Quinine suppresses brain chemicals**  
> Quinine acts as a competitive inhibitor of monoamine oxidase (MAO), an enzyme that breaks down neurotransmitters. This means it could potentially treat psychological disorders, like antidepressants, despite being known for treating leg cramps.

Sources: s1 (Wikipedia, Uses)  
Fun: _ /5


### serotonin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Serotonin carries the official GHS signal word "Danger". Its hazard statements: toxic if swallowed, and suspected of damaging fertility or the unborn child.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 2 snippets · 10.1 s

<details><summary>Attempt 1 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "From enteramine to serotonin", "body": "In 1935, Vittorio Erspamer in Pavia discovered a substance from enterochromaffin cells that caused intestinal contraction, which he named 'enteramine'. Two years later, he confirmed it was a new amine. In 1948, Maurice M. Rapport, Arda Green, and Irvine Page discovered a vasoconstrictor in blood serum and named it serotonin. In 1952, enteramine was shown to be the same as serotonin.", "sources": ["s1", "s2"]}
```
</details>

<details><summary>Attempt 2 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "From enteramine to serotonin", "body": "In 1935, Vittorio Erspamer in Pavia discovered a substance from enterochromaffin cells that caused intestinal contraction and named it enteramine. Two years later, he showed it was a new amine. In 1948, Maurice M. Rapport, Arda Green, and Irvine Page discovered a vasoconstrictor in blood serum and named it serotonin. In 1952, enteramine was shown to be the same as serotonin.", "sources": ["s1", "s2"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 6.7 s

> **In your gut and brain**  
> Serotonin is mostly found in the gastrointestinal tract, where it regulates intestinal movements. A smaller amount is in the brainstem and skin, while 8% is stored in blood platelets, which release it during clotting. In the central nervous system, it helps regulate mood, appetite, and sleep.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 7.4 s

> **A parasite makes you feel sick with serotonin**  
> The parasite Entamoeba histolytica secretes serotonin, causing diarrhea in some people. It also becomes more virulent when exposed to serotonin. Patients infected with it have highly elevated serotonin levels that return to normal after recovery.

Sources: s1 (Wikipedia, Comparative biology & evolution)  
Fun: _ /5


### strychnine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Strychnine carries the official GHS signal word "Danger". Its hazard statements: fatal if swallowed, fatal in contact with skin, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.6 s

> **First discovered in a bean**  
> In 1818, French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier first discovered strychnine in Saint-Ignatius' bean (Strychnos ignatii). The compound was isolated from plants in the genus Strychnos, where it has long been known for its toxic properties. Historic records suggest its use in Europe as early as 1640 to kill animals and humans.

Sources: s3 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.5 s

> **In seeds of the Strychnos tree**  
> Strychnine is found in the seeds of the Strychnos nux-vomica tree and other plants like Strychnos ignatii and Strychnos wallichiana. It is used as a pesticide to kill birds and rodents. When inhaled, swallowed, or absorbed through the eyes or mouth, it causes muscular convulsions and death through asphyxia.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.2 s

> **Used to kill birds and humans in 1640**  
> Strychnine was allegedly used to kill dogs, cats, and birds as early as 1640 in Europe. It was also reportedly used by murderer William Palmer and by Oskar Dirlewanger during World War II to kill civilians. This shows its long history as a poison in both domestic and wartime contexts.

Sources: s1 (Wikipedia, History)  
Fun: _ /5


### sucralose

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Sucralose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.4 s

> **A chlorinated sugar test**  
> In 1975, scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College, mixed sulfuryl chloride with sugar. Phadnis was told to 'test' a chlorinated sugar compound and, believing Hough asked him to 'taste' it, did so and found it exceptionally sweet. The compound was sucralose, patented by Tate & Lyle in 1976.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.5 s

> **Sweetness in your coffee and candy**  
> Sucralose is found in coffee pods, breakfast bars, soft drinks, and canned fruits, where it replaces corn syrup-based additives. It's also in Splenda, which mixes sucralose with dextrose and maltodextrin, with sucralose making up only about 1.1% of the product. Used in baked goods, confections, and table-top sugar substitutes, it adds sweetness without calories or affecting insulin levels.

Sources: s1 (Wikipedia, Uses), s2 (Wikipedia, Uses), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 1 snippets · 11.4 s

> **He tasted a sweet sugar compound**  
> Shashikant Phadnis was told to 'test' a chlorinated sugar compound. He thought it meant to taste it and found it exceptionally sweet. This accidental taste led to the discovery of sucralose.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: title has 11 words, at most 8 allowed</summary>

```json
{"title": "He tasted a sweet compound thinking it was a taste test", "body": "Leslie Hough and Shashikant Phadnis were told to 'test' a chlorinated sugar compound. Phadnis thought 'taste' was what was meant and actually tasted it, finding it exceptionally sweet. This accidental tasting led to the discovery of sucralose.", "sources": ["s1"]}
```
</details>


### tetrodotoxin

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Tetrodotoxin carries the official GHS signal word "Danger". Its hazard statements: fatal if swallowed, fatal in contact with skin, and fatal if inhaled.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.4 s

> **TTX is deadlier than cyanide, even by mouth**  
> Tetrodotoxin is extremely toxic. Its oral LD50 for mice is 334 μg per kg, which is less than that of potassium cyanide (8,500 μg per kg), showing it's more poisonous orally. Intravenously, it's even deadlier—only 8 μg per kg is needed to be lethal in mice.

Sources: s1 (Wikipedia, Poisoning)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 11.0 s

> **In pufferfish and beyond**  
> Tetrodotoxin is found mainly in the liver and gonads of pufferfish, globefish, and toadfish, as well as in blue-ringed octopuses, rough-skinned newts, and moon snails. It can also be present in some shellfish and is produced by bacteria like Pseudoalteromonas, Pseudomonas, and Vibrio through symbiotic relationships. Human exposure occurs when consuming improperly prepared fish organs.

Sources: s1 (PubChem, Uses), s2 (Wikipedia, Introduction), s3 (PubChem, Uses)  
Fun: _ /5


### thalidomide

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Thalidomide carries the official GHS signal word "Danger". Its hazard statements: harmful if swallowed, and may damage fertility or the unborn child.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.7 s

> **From discarded by-product to a 'wonder drug'**  
> In 1952, Chemical Industry Basel synthesized thalidomide but discarded it because it had no effect on animals. In 1957, Chemie Grünenthal acquired it and Heinrich Mückter led the discovery program. While preparing reagents, Mückter's assistant Wilhelm Kunz isolated a by-product recognized by pharmacologist Herbert Keller as an analog of glutethimide.

Sources: s1 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 8.2 s

> **In cancer treatments and rare skin conditions**  
> Thalidomide is found in medicines like Thalomid and Distaval, used to treat multiple myeloma and leprosy-related skin lesions. It is also prescribed for HIV-related complications and erythema nodosum leprosum. Once widely sold for morning sickness, it was removed from the market in 1961 after causing severe birth defects in over 10,000 possibly 20,000 infants.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction), s3 (Wikipedia, Medical uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 9.0 s

> **Thalidomide causes specific deformities by day**  
> Thalidomide taken on day 20 caused brain damage, day 22 affected ears and face, day 24 led to arm deformities, and leg damage occurred if taken up to day 28. The timing of exposure directly determined the type of birth defect, making the impact highly predictable by pregnancy day.

Sources: s1 (Wikipedia, Society and culture)  
Fun: _ /5


### theobromine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Theobromine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed. A minority of reports list 2 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 3 snippets · 3.5 s

> **Discovered in cacao beans**  
> In 1841, chemist A. Woskresensky discovered theobromine in cacao beans. This discovery was followed by the first synthesis of theobromine from xanthine in 1882 by Hermann Emil Fischer.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.7 s

> **Sour chocolate and tea**  
> Theobromine is found in chocolate, tea (Camellia sinensis), American hollies (yaupon and guayusa), and the kola nut. It is also present in cacao beans and appears as a white or colourless solid, though commercial samples may be yellowish.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.4 s

> **No bromine, yet it's called theobromine**  
> Theobromine contains no bromine, despite its name. The name comes from Greek 'bromos' meaning 'stench', not the element bromine.

Sources: s1 (Wikipedia, Etymology)  
Fun: _ /5


### vanillin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Vanillin carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed, and toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.7 s

> **From Mesoamerica to synthetic labs**  
> Native Mesoamerican peoples, such as the Totonacs of modern-day Veracruz, Mexico, first used vanilla as a flavoring in beverages by the early 15th century. The Aztecs used it in chocolate drinks called xocohotl. In 1858, Théodore Nicolas Gobley isolated vanillin by evaporating vanilla extract and recrystallizing it. In 1874, Ferdinand Tiemann and Wilhelm Haarmann deduced its structure and developed a synthesis from coniferin, starting industrial production in Holzminden, Germany.

Sources: s1 (Wikipedia, Natural history), s2 (Wikipedia, Natural history), s3 (Wikipedia, Synthetic history), s4 (Wikipedia, Synthetic history)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 8.5 s

> **Scent of chocolate and ice cream**  
> Vanillin is the primary component of vanilla bean extract and is widely used in sweet foods. It appears in ice cream and chocolate, which together make up 75% of vanillin's market. Synthetic vanillin is now more common than natural extract in foods, beverages, and pharmaceuticals.

Sources: s1 (Wikipedia, Introduction), s3 (Wikipedia, Uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.8 s

> **Vanillin in 2nd millennium BCE jars**  
> Vanillin residue was found in jars from a tomb in Canaan dating to the 2nd millennium BCE, suggesting vanilla cultivation may have occurred there long before the 16th century. Traces were also found in wine jars in Jerusalem used by the Judahite elite before 586 BCE.

Sources: s1 (Wikipedia, Natural history)  
Fun: _ /5

