# Spike run: Qwen3-4B-Instruct-2507-Q4_K_M

30 molecules, 111 cards with input. Temperature 0.2, JSON schema constraint, one stricter retry, shortest card first, pick-then-write for Weird and Where, Danger from GHS data, dose and procedure sentences scrubbed, claim check after the validator, KV cache: paged, fixed 4K tokens.

## Summary

| Measure | Value |
| --- | --- |
| From GHS data (no model) | 28 (25%) |
| Passed first try | 44 (40%) |
| Passed on retry | 15 (14%) |
| Skipped by model | 3 (3%) |
| Rejected twice (dropped) | 21 (19%) |
| No input (no card) | 9 |
| Molecules with 3+ cards | 22 of 30 |
| First text of a model card | 5.2 s median, 6.6 s p90, 7.4 s max |
| Model card, including pick and retry | 10.2 s median, 15.4 s p90, 19.7 s max |
| First card per molecule | 0.0 s median, 0.0 s p90, 10.1 s max |
| First model card per molecule | 9.6 s median, 18.9 s p90, 38.9 s max |
| Model load | 7.4 s |
| Peak memory | 4.14 GB |

### Problems found (all attempts)

- claim check: 61
- name not in sources: 2
- sentence count: 1
- title: 2
- too long: 1

## Cards

Fun score is for hand review in M0.6.

### allicin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Allicin carries the official GHS signal word "Warning". Its hazard statements: causes skin irritation, causes serious eye irritation, and may cause respiratory irritation.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 2 snippets · 12.3 s

> **Discovered in Japan in the 1940s**  
> Allicin was discovered in the 1940s in Japan as part of efforts to create thiamine derivatives. Chester J. Cavallito and John Hays Bailey first isolated and studied it in 1944. Allicin became a model for developing thiamine disulfides such as sulbutiamine, fursultiamine, and benfothiamine.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "It was discovered during efforts to create thiamine derivatives in Japan."</summary>

```json
{"title": "Discovered in Japan in the 1940s", "body": "In 1944, Chester J. Cavallito and John Hays Bailey first isolated and studied allicin in the laboratory. It was discovered during efforts to create thiamine derivatives in Japan. Allicin became a model for developing other thiamine disulfides such as sulbutiamine, fursultiamine, and benfothiamine.", "sources": ["s1", "s2"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 7.1 s

> **In the smell of crushed garlic**  
> Allicin forms when fresh garlic or leeks are chopped or crushed, triggering a reaction in the enzyme alliinase. You meet it in the distinctive odor of fresh garlic, which quickly turns into other sulfur compounds. It’s also found in nature as a defense mechanism against pests.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · rejected · 1 snippets · 11.9 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "These compounds are hydrophobic and reduced to thiamine by cysteine or glutathione."</summary>

```json
{"title": "Allicin led to thiamine derivatives", "body": "Allicin was discovered in the 1940s in Japan during thiamine derivative research. It became a model for creating thiamine disulfides like sulbutiamine, fursultiamine, and benfothiamine. These compounds are hydrophobic and reduced to thiamine by cysteine or glutathione.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "These compounds are hydrophobic and reduced to thiamine by cysteine or glutathione."</summary>

```json
{"title": "Allicin was discovered for thiamine research", "body": "Allicin was discovered in the 1940s as part of efforts to create thiamine derivatives, mainly in Japan. It became a model for medicinal chemistry to develop other thiamine disulfides. These compounds are hydrophobic and reduced to thiamine by cysteine or glutathione.", "sources": ["s1"]}
```
</details>


### artemisinin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Artemisinin carries the official GHS signal word "Danger". Its hazard statements: heating may cause a fire, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 15.8 s

> **From ancient remedy to antimalarial discovery**  
> In 1596, Li Shizhen recommended tea made from sweet wormwood to treat malaria. In 1967, a secret military program called Project 523 was launched under the People's Liberation Army at Mao Zedong's order to find a malaria treatment. In 1972, Tu Youyou discovered artemisinin in the leaves of Artemisia annua. She was influenced by a 340 CE text by Ge Hong recommending cold water steeping of the herb.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 81 words, must be under 80</summary>

```json
{"title": "From ancient herb to antimalarial discovery", "body": "In 1596, Li Shizhen recommended tea made from sweet wormwood to treat malaria symptoms in his Compendium of Materia Medica. In 1967, a secret military program called 'Project 523' was launched by the People's Liberation Army under Mao Zedong's order to find a malaria treatment. In 1972, Tu Youyou discovered artemisinin in the leaves of Artemisia annua, inspired by a 340 CE herbal text by Ge Hong. She also found that a low-temperature extraction process could isolate an effective antimalarial substance.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Where you meet it** · passed on retry · 3 snippets · 16.1 s

> **In sweet wormwood and malaria treatment**  
> Artemisinin is found in Artemisia annua, also known as sweet wormwood. It is used in artemisinin-based combination therapies (ACTs) for malaria due to Plasmodium falciparum. The molecule is extracted from the herb and used as an antimalarial for multi-drug resistant strains of falciparum malaria.

Sources: s1 (Wikipedia, Introduction), s2 (PubChem, Record Description), s3 (PubChem, Record Description)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "It has also been used in traditional medicine for malaria, cancer, viral and bacterial infections, and some cardiovasculreviear diseases."</summary>

```json
{"title": "In sweet wormwood and malaria treatment", "body": "Artemisinin is found in Artemisia annua, also known as sweet wormwood, sweet annie, sweet sagewort, and annual wormwood. It is used in antimalarial drugs, especially for Plasmodium falciparum malaria, and appears in Artemisinin-based combination therapies (ACTs) as standard treatment worldwide. It has also been used in traditional medicine for malaria, cancer, viral and bacterial infections, and some cardiovasculreviear diseases.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · passed on retry · 1 snippets · 12.2 s

> **Named after a queen botanist**  
> The genus name Artemisia is derived from the Greek goddess Artemis. It may have been named after Queen Artemisia II of Caria, who was a botanist and medical researcher in the fourth century BC.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "This links a modern antimalarial drug to ancient medical practice and a historical female scientist."</summary>

```json
{"title": "Named after a queen botanist", "body": "Artemisinin's genus name may have been named after Queen Artemisia II of Caria, a botanist and medical researcher in the fourth century BC. This links a modern antimalarial drug to ancient medical practice and a historical female scientist.", "sources": ["s1"]}
```
</details>


### ascorbic acid

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Ascorbic acid is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.8 s

> **Lind's citrus experiment**  
> In May 1747, James Lind, a ship's surgeon in the Royal Navy, conducted one of the first controlled experiments by giving some crew members oranges and lemons while others received cider, vinegar, sulfuric acid, or seawater. He found citrus fruits prevented scurvy. Lind published his findings in 1753 in his Treatise on the Scurvy.

Sources: s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed on retry · 3 snippets · 15.7 s

> **In citrus and canned fruits**  
> Ascorbic acid is found naturally in citrus fruits and many vegetables. It is added to canned fruits to slow oxidation and enzymatic browning. It may also be used as a flour treatment agent in breadmaking. As food additives, it is assigned E numbers, with approval by the European Food Safety Authority.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Food preservation additive)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "In your citrus and bread"</summary>

```json
{"title": "In your citrus and bread", "body": "Ascorbic acid is found naturally in citrus fruits and many vegetables. It's added to canned fruits to slow oxidation and browning, and may be used as a flour treatment in breadmaking. As a food additive, it carries E numbers under approval by the European Food Safety Authority.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · rejected · 1 snippets · 12.1 s

<details><summary>Attempt 1 rejected: name C is not in the cited snippets</summary>

```json
{"title": "Sour krout saved Cook's crew", "body": "Captain James Cook used 'Sour krout' on his 1772–75 voyage, preventing scurvy in his crew. This early use of vitamin C-rich food predates the widespread adoption of lemon juice in the navy. The term 'Sour krout' may refer to a form of preserved, vitamin C-rich food.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: name C is not in the cited snippets</summary>

```json
{"title": "Sour krout stopped scurvy in Cook's voyage", "body": "Captain James Cook used 'Sour krout' on his 1772–75 voyage, which prevented scurvy in his crew. This early use of vitamin C-rich food predates the formal adoption of lemon juice by the British navy. The term 'Sour krout' may refer to a form of preserved, vitamin C-rich food.", "sources": ["s1"]}
```
</details>


### aspirin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Aspirin carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed. A minority of reports list 3 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 13.5 s

> **First pure form made in 1897**  
> In 1897, Felix Hoffmann (or Arthur Eichengrün) of Bayer first produced acetylsalicylic acid in a pure, stable form. This was part of Bayer's effort to create a less-irritating alternative to salicylate medicines. The compound was later branded as Aspirin, which was Bayer's brand name, not the generic name.

Sources: s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "The compound was later branded as Aspirin, though the name was not the generic name."</summary>

```json
{"title": "First pure form in 1897", "body": "In 1897, Felix Hoffmann (or Arthur Eichengrün) of Bayer first produced acetylsalicylic acid in a pure, stable form. This was part of Bayer's effort to create a less-irritating alternative to salicylate medicines. The compound was later branded as Aspirin, though the name was not the generic name.", "sources": ["s2"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 9.8 s

> **In your medicine cabinet and beyond**  
> Aspirin, known as acetylsalicylic acid (ASA), is found in medicines for pain, fever, and inflammation. It's used to treat conditions like Kawasaki disease and pericarditis. Long-term use helps reduce the risk of heart attacks, strokes, and blood clots in high-risk individuals.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed on retry · 1 snippets · 13.2 s

> **Aspirin has a hidden second form**  
> Until 2005, only one proven polymorph of aspirin (form I) existed. A 1981 report showed weak additional peaks in crystallized aspirin, which was dismissed as impurity at the time. In retrospect, this was identified as form II aspirin, a second crystal structure previously debated since the 1960s.

Sources: s1 (Wikipedia, Physical properties)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Aspirin has a hidden second form", "body": "Until 2005, only one polymorph of aspirin was proven, known as form I. However, a 1981 report showed weak additional peaks in crystallized aspirin, which was dismissed then but later recognized as form II aspirin.", "sources": ["s1"]}
```
</details>


### batrachotoxin

**Danger level** · no input · 0 snippets · 0.0 s

**Origin story** · passed · 1 snippets · 6.3 s

> **Discovery of batrachotoxin in frogs**  
> In 1963, Fritz Märki and Bernhard Witkop isolated the toxic alkaloid fraction from the black-legged poison dart frog at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. They identified four major toxic steroidal alkaloids, including batrachotoxin, homobatrachotoxin, pseudobatrachotoxin, and batrachotoxinin A. The process was challenging due to the toxin's potency and the tiny amounts available.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 11.0 s

> **In frogs, birds, and beetles**  
> Batrachotoxin is found in the skin of frogs from the genus Phyllobates, in birds of the genera Pitohui and Ifrita, and in beetles of the genus Choresine. Indigenous groups like the Noanamá Chocó and Emberá Chocó use it to poison blowgun darts. The toxin binds to sodium channels, causing paralysis and death, and no antidote is known.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Use), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed on retry · 1 snippets · 13.9 s

> **Toxin in frogs, birds, and beetles**  
> Batrachotoxin is found in frogs of the genus Phyllobates, birds in the genera Pitohui and Ifrita, and beetles in the genus Choresine. This shows the toxin exists across unrelated species. It was discovered by the precolonial inhabitants of the Northwestern Andes (present day Colombia).

Sources: s1 (PubChem, Record Description)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "This widespread presence across unrelated species is surprising and shows a shared evolutionary defense mechanism."</summary>

```json
{"title": "Frogs, birds, and beetles all have the toxin", "body": "Batrachotoxin is found in frogs of the genus Phyllobates, birds in the genera Pitohui and Ifrita, and beetles in the genus Choresine. This widespread presence across unrelated species is surprising and shows a shared evolutionary defense mechanism.", "sources": ["s1"]}
```
</details>


### benzylpenicillin

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Benzylpenicillin carries the official GHS signal word "Danger". Its hazard statements: may cause an allergic skin reaction, and may cause allergy or asthma symptoms or breathing difficulties if inhaled. A minority of reports list 2 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 10.2 s

> **In injections for infections**  
> Benzylpenicillin is found in injectable forms used to treat infections like pneumonia, strep throat, syphilis, and gas gangrene. It is administered intravenously or intramuscularly due to poor oral absorption. Common salts include sodium, potassium, benzathine, and procaine.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · skipped by model · 1 snippets · 5.1 s


### buckminsterfullerene

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Buckminsterfullerene carries the official GHS signal word "Warning". Its hazard statements: causes serious eye irritation, and may cause respiratory irritation. A minority of reports list 1 other hazard.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 17.5 s

> **Laser vaporization leads to discovery**  
> In 1984, Eric Rohlfing, Donald Cox, and Andrew Kaldor generated a carbon molecule using a laser to vaporize carbon in a supersonic helium beam, but did not recognize it as buckminsterfullerene. In 1985, Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University repeated the experiment and recognized the C60 structure as buckminsterfullerene. They used laser vaporization of graphite and found a strong peak at 720 daltons, indicating a C60 cluster.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "They found a strong peak at 720 daltons and concluded the structure was a spheroidal, cage-like molecule with icosahedral symmetry."</summary>

```json
{"title": "Laser vaporization sparks discovery", "body": "In 1984, Eric Rohlfing, Donald Cox, and Andrew Kaldor generated buckminsterfullerene using a laser to vaporize carbon in a supersonic helium beam, but did not recognize the structure. In 1985, Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University repeated the experiment and identified C60 as buckminsterfullerene. They found a strong peak at 720 daltons and concluded the structure was a spheroidal, cage-like molecule with icosahedral symmetry.", "sources": ["s1", "s2", "s3", "s4"]}
```
</details>

**Where you meet it** · rejected · 3 snippets · 14.4 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "It was first discovered in shungite, a rock containing 99% carbon."</summary>

```json
{"title": "In soot, space, and ancient rock", "body": "Buckminsterfullerene is the most common naturally occurring fullerene, found in small quantities in soot. It also exists in space, where neutral C60 is seen in planetary nebulae and stars, and C+60 causes diffuse interstellar bands. It was first discovered in shungite, a rock containing 99% carbon.", "sources": ["s1", "s2", "s3"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "It was first discovered in shungite, a rock containing 99% carbon."</summary>

```json
{"title": "In soot, space, and shungite", "body": "Buckminsterfullerene is the most common naturally occurring fullerene. Small quantities can be found in soot. It also exists in space, where neutral C60 is observed in planetary nebulae and stars, and C+60 is found in the interstellar medium. It was first discovered in shungite, a rock containing 99% carbon.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · passed · 1 snippets · 9.6 s

> **A buckyball was spotted in 2026**  
> The B80 buckyball was first observed in 2026 by a team led by Lai-Sheng Wang at Brown University. Its symmetric structure was confirmed by matching the experimental photoelectron spectrum with a simulated one.

Sources: s1 (Wikipedia, Boron fullerenes)  
Fun: _ /5


### caffeine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Caffeine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · skipped by model · 4 snippets · 1.7 s

**Weird fact** · passed · 1 snippets · 7.9 s

> **Caffeine helps premature babies gain weight**  
> Caffeine is used to treat bronchopulmonary dysplasia in premature infants. It may improve weight gain and reduce risks of cerebral palsy and cognitive delay. Subtle long-term side effects are possible.

Sources: s1 (Wikipedia, Uses)  
Fun: _ /5

**Where you meet it** · rejected · 3 snippets · 17.5 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "It occurs in plants from Africa, East Asia, and South America, where it acts as a natural pesticide and protects seeds."; not stated in the cited snippets: "Humans encounter it in beverages, cosmetics, and as a stimulant in medicines."</summary>

```json
{"title": "Sip it in coffee, tea, or chocolate", "body": "Caffeine is found naturally in tea leaves, coffee beans, cocoa beans, mate leaves, guarana paste, and kola nuts. It occurs in plants from Africa, East Asia, and South America, where it acts as a natural pesticide and protects seeds. Humans encounter it in beverages, cosmetics, and as a stimulant in medicines.", "sources": ["s1", "s2", "s3"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "Humans encounter it in beverages, cardiac and respiratory stimulants, and diuretics, and also in cosmetics."</summary>

```json
{"title": "In coffee, tea, and chocolate", "body": "Caffeine occurs naturally in the seeds, leaves, and fruit of plants like Camellia sinensis and Coffea. It is found in tea leaves, coffee beans, cocoa beans, mate leaves, guarana paste, and kola nuts. Humans encounter it in beverages, cardiac and respiratory stimulants, and diuretics, and also in cosmetics.", "sources": ["s1", "s2", "s3"]}
```
</details>


### capsaicin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Capsaicin carries the official GHS signal word "Danger". Its hazard statements: harmful if swallowed, causes skin irritation, and causes serious eye damage. A minority of reports list 1 other hazard.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 2 snippets · 9.4 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "First extracted and studied"</summary>

```json
{"title": "First extracted and studied", "body": "In 1816, Christian Friedrich Bucholz first extracted capsaicin in impure form. In 1873, Rudolf Buchheim noted that 'capsicol' caused a burning sensation. In 1878, Endre Hőgyes confirmed that 'capsicol' increased gastric acid secretion.", "sources": ["s1", "s2"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "First extracted and studied"; not stated in the cited snippets: "In 1878, Endre Hőgyes confirmed that 'capsicol' increased gastric acid secretion."</summary>

```json
{"title": "First extracted and studied", "body": "In 1816, Christian Friedrich Bucholz first extracted capsaicin in impure form. In 1873, Rudolf Buchheim stated that 'capsicol' caused a burning feeling. In 1878, Endre Hőgyes confirmed that 'capsicol' increased gastric acid secretion.", "sources": ["s1", "s2"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 9.6 s

> **The heat in your chili**  
> Capsaicin gives hot peppers their spicy taste and is found in fruits like Capsicum annuum and C. frutescens. It's commonly added to chili powder and paprika for heat. The spiciness is measured on the Scoville scale.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Uses), s3 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 9.0 s

> **Seeds don't make capsaicin**  
> Capsaicin is not produced in the seeds of Capsicum plants. Instead, the highest concentration is found in the white pith where seeds are attached, and the seeds themselves do not produce any capsaicin.

Sources: s1 (Wikipedia, Natural function)  
Fun: _ /5


### carvone

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Carvone carries the official GHS signal word "Warning". Its hazard statements: may cause an allergic skin reaction.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 1 snippets · 9.6 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "In 1849, Franz Varrentrapp obtained carvone from caraway, originally naming it carvol by Schweizer."</summary>

```json
{"title": "Carvone isolated in 1849", "body": "In 1849, Franz Varrentrapp obtained carvone from caraway, originally naming it carvol by Schweizer. Goldschmidt and Zürrer identified it as a ketone related to limonene. The structure was finally elucidated by Georg Wagner in 1894.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "In 1849, Franz Varrentrapp obtained carvone from caraway."; not stated in the cited snippets: "He originally named it carvol by Schweizer."</summary>

```json
{"title": "Carvone isolated in 1849", "body": "In 1849, Franz Varrentrapp obtained carvone from caraway. He originally named it carvol by Schweizer. The compound was later identified as a ketone related to limonene by Goldschmidt and Zürrer. Its structure was finally elucidated by Georg Wagner in 1894.", "sources": ["s1"]}
```
</details>

**Where you meet it** · rejected · 3 snippets · 19.7 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "Sour-sweet in caraway and mint"</summary>

```json
{"title": "Sour-sweet in caraway and mint", "body": "Carvone is found naturally in caraway seeds (Carum carvi), where it makes up 60–70% of the oil, and in dill seeds (Anethum graveolens) at 40–60%. It is also abundant in spearmint oil (Mentha spicata), which contains 50–80% R-(−)-carvone. This molecule gives flavor to caraway, dill, and spearmint and appears in food, air fresheners, and aromatherapy products.", "sources": ["s1", "s2", "s3"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "Carvone is found naturally in caraway seeds (Carum carvi), where it makes up 60–70% of the oil, and in spearmint (Mentha spicata), where it accounts for 50–80% of the oil."</summary>

```json
{"title": "In caraway, spearmint, and orange peel", "body": "Carvone is found naturally in caraway seeds (Carum carvi), where it makes up 60–70% of the oil, and in spearmint (Mentha spicata), where it accounts for 50–80% of the oil. It also occurs in dill seed oil (Anethum graveolens) and mandarin orange peel oil. In food, carvone is used for flavoring caraway, dill, and spearmint, and is often made from limonene.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · passed · 1 snippets · 9.6 s

> **Squirrel monkeys smell carvone enantiomers**  
> Carvone's two enantiomers are perceived as smelling different, showing olfactory receptors respond more strongly to one than the other. Squirrel monkeys have been found to discriminate between these enantiomers.

Sources: s1 (Wikipedia, Stereoisomerism and odor)  
Fun: _ /5


### cinnamaldehyde

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Cinnamaldehyde carries the official GHS signal word "Warning". Its hazard statements: may cause an allergic skin reaction.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · rejected · 3 snippets · 19.6 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "Spicy sweetness in your snacks"</summary>

```json
{"title": "Spicy sweetness in your snacks", "body": "Cinnamaldehyde gives cinnamon its signature flavor and aroma, found naturally in the bark of Cinnamomum trees like camphor and cassia. It’s used in chewing gum, ice cream, candy, e-liquid, and beverages at levels from 9 to 4,900 ppm. Also found in perfumes, it can mimic sweet, spicy, and balsamic scents—though it’s sometimes misused to fake cinnamon in products like powdered beechnut husk.", "sources": ["s1", "s2", "s3"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "In cinnamon bark and snacks"</summary>

```json
{"title": "In cinnamon bark and snacks", "body": "Cinnamaldehyde occurs naturally in the bark of cinnamon trees and other Cinnamomum species like camphor and cassia. The essential oil of cinnamon bark is about 90% cinnamaldehyde. It is used as flavoring in chewing gum, ice cream, candy, e-liquid, and beverages at levels from 9 to 4,900 ppm. It can also be found in perfumes with sweet, spicy, and balsamic scents.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · rejected · 1 snippets · 13.1 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "This precise, measurable effect makes it a potent and practical insecticide."</summary>

```json
{"title": "29 ppm kills half the mosquito larvae", "body": "A concentration of 29 ppm of cinnamaldehyde kills half of Aedes aegypti mosquito larvae in 24 hours. This precise, measurable effect makes it a potent and practical insecticide.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: title has 9 words, at most 8 allowed</summary>

```json
{"title": "29 ppm kills half the larvae in 24 hours", "body": "A concentration of 29 ppm of cinnamaldehyde kills half of Aedes aegypti mosquito larvae in 24 hours. This shows its effectiveness as a safe and effective insecticide. Trans-cinnamaldehyde also acts as a potent fumigant and repellant for adult mosquitos.", "sources": ["s1"]}
```
</details>


### cubane

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · no input · 0 snippets · 0.0 s

**Where you meet it** · passed on retry · 1 snippets · 10.1 s

> **In high-performance explosives**  
> Octanitrocubane and heptanitrocubane have been studied as high-performance explosives. These compounds have a very high density for hydrocarbon molecules and store a large amount of energy in a small space. Their geometry and stability make them suitable isosteres for benzene rings.

Sources: s1 (Wikipedia, Introduction)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "In high-energy research labs"; not stated in the cited snippets: "These compounds are used in research for energy storage and as isosteres for benzene rings."</summary>

```json
{"title": "In high-energy research labs", "body": "Cubane and its derivatives like octanitrocubane and heptanitrocubane are studied as high-performance explosives due to their high energy density and stability. These compounds are used in research for energy storage and as isosteres for benzene rings. Their geometry allows them to fit into molecular frameworks similar to benzene.", "sources": ["s1"]}
```
</details>

**Weird fact** · passed · 1 snippets · 4.7 s

> **Cubane refuses to break down**  
> Cubane is highly strained and cannot decompose because the resulting cubene molecules would be pyramidal alkenes, too high-energy for most elimination pathways. Certain metallic ions can catalyze σ-bond rearrangement to cuneane.

Sources: s1 (Wikipedia, Reactions and derivatives)  
Fun: _ /5


### dopamine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Dopamine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed, suspected of damaging fertility or the unborn child, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 2 snippets · 10.9 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "First synthesized and identified in the brain"</summary>

```json
{"title": "First synthesized and identified in the brain", "body": "In 1910, George Barger and James Ewens first synthesized dopamine at Wellcome Laboratories in London, England. It was first identified in the human brain in 1957 by Katharine Montagu. The neurotransmitter function of dopamine was first recognized in 1958 by Arvid Carlsson and Nils-Åke Hillarp.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "First synthesized and identified in the brain"</summary>

```json
{"title": "First synthesized and identified in the brain", "body": "In 1910, George Barger and James Ewens first synthesized dopamine at Wellcome Laboratories in London, England. In 1957, Katharine Montagu first identified dopamine in the human brain. The molecule was named because its precursor in the Barger-Ewens synthesis is 3,4-dihydroxyphenylalanine (levodopa or L-DOPA).", "sources": ["s1"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 11.5 s

> **Where dopamine works in your body**  
> Dopamine is a neurotransmitter in the brain, sending signals between nerve cells. Outside the brain, it acts as a local messenger in blood vessels, kidneys, pancreas, digestive system, and immune cells. It is also used medically as a drug under names like Intropin, Dopastat, and Revimine, especially in newborns with low blood pressure or cardiac arrest.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction), s3 (Wikipedia, Medical uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.6 s

> **Animals may have borrowed dopamine from bacteria**  
> Dopamine-synthesizing enzymes in animals might have come from bacteria via horizontal gene transfer. This transfer could have happened late in evolution, possibly during the formation of mitochondria through bacterial symbiosis.

Sources: s1 (Wikipedia, Comparative biology and evolution)  
Fun: _ /5


### ethylene

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Ethylene carries the official GHS signal word "Danger". Its hazard statements: extremely flammable gas, and may cause drowsiness or dizziness.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 4 snippets · 10.4 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "In 1777, a Mr."</summary>

```json
{"title": "Ethylene's first synthesis in 1777", "body": "In 1777, a Mr. Enée in Amsterdam synthesized ethylene, as reported by Jan Ingenhousz. Ingenhousz later produced the gas himself. Joseph Priestley documented this in his 1779 work.", "sources": ["s2"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "Ethylene first seen in 1777"</summary>

```json
{"title": "Ethylene first seen in 1777", "body": "In 1777, a Mr. Enée in Amsterdam synthesized ethylene, as reported by Jan Ingenhousz. Ingenhousz later produced the gas himself. Joseph Priestley mentioned this in his 1779 work.", "sources": ["s2"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 9.3 s

> **Ripening fruit and flowers**  
> Ethylene is naturally released by fruits, flowers, leaves, and tubers, especially during ripening or senescence. For example, ripening avocados and apples, and fading morning glory flowers, emit it. It's also used commercially to speed up fruit ripening.

Sources: s1 (Wikipedia, Introduction), s2 (PubChem, Natural Pollution Sources), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · rejected · 1 snippets · 12.0 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "This links a ancient ritual to a modern chemical compound in a surprising way."</summary>

```json
{"title": "Pythia's trance may have been ethylene", "body": "Some geologists and scholars propose that the Pythia’s trance-like state at Delphi was caused by ethylene rising from ground faults. This links a ancient ritual to a modern chemical compound in a surprising way.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "This proposed link connects an ancient ritual to a modern chemical compound."</summary>

```json
{"title": "Oracle's trance may be from ethylene", "body": "Some geologists and scholars think the Pythia's trance-like state at Delphi was caused by ethylene rising from ground faults. This proposed link connects an ancient ritual to a modern chemical compound.", "sources": ["s1"]}
```
</details>


### geosmin

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Geosmin carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 1 snippets · 6.1 s

> **The smell after rain**  
> You meet geosmin when it rains after dry weather, creating the scent known as petrichor. It's also in beetroots and made by bacteria in soil and water. Geosmin causes muddy taste and smell in drinking water and farmed fish.

Sources: s1 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 6.3 s

> **Humans smell geosmin at 5 ppt**  
> The human olfactory system can detect geosmin at as low as 5 parts per trillion. This extreme sensitivity is proposed to have evolved to help humans or their ancestors find scarce water, though this is not conclusive.

Sources: s1 (Wikipedia, Effects)  
Fun: _ /5


### indigo

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Indigo carries the official GHS signal word "Warning". Its hazard statements: may causes damage to organs through prolonged or repeated exposure.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 6.7 s

> **In your blue jeans and plants**  
> Indigo is found in the leaves of Indigofera tinctoria plants, where it naturally forms a blue pigment. Today, most indigo is synthetic and used in denim production, with a pair of jeans requiring 3–12 grams of dye. It's also used in small amounts for wool and silk dyeing.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction), s3 (Wikipedia, Uses)  
Fun: _ /5

**Origin story** · rejected · 4 snippets · 13.5 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "The oldest known indigo-dyed fabric dates to 6,000 years ago and was found in Huaca Prieta, Peru."</summary>

```json
{"title": "Indigo's ancient roots in India", "body": "The oldest known indigo-dyed fabric dates to 6,000 years ago and was found in Huaca Prieta, Peru. The Indigofera tinctoria species was domesticated in India, which became the earliest major center for its production and processing. India was also the source of indigo used by the Greeks and Romans, who valued it as a luxury product.", "sources": ["s1", "s2", "s3"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "First indigo dye found in Peru"</summary>

```json
{"title": "First indigo dye found in Peru", "body": "The oldest known fabric dyed with indigo dates to 6,000 years ago and was discovered in Huaca Prieta, Peru. This indicates early use of indigo in ancient civilizations, including those in Peru, Mesopotamia, Egypt, and Mesoamerica. The dye was cultivated in India, where the Indigofera tinctoria species was domesticated.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · passed · 1 snippets · 9.5 s

> **Indigo turns soluble with bacteria's breath**  
> In Japan, indigo is dissolved using thermophilic, anaerobic bacteria that generate hydrogen as a metabolic product. This hydrogen converts insoluble indigo into soluble indigo white. The process is part of traditional dyeing techniques like shibori and katazome.

Sources: s1 (Wikipedia, Dyeing technology)  
Fun: _ /5


### limonene

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Limonene carries the official GHS signal word "Warning". Its hazard statements: flammable liquid and vapor, causes skin irritation, may cause an allergic skin reaction, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 11.6 s

> **The zesty scent in your kitchen**  
> Limonene is the main fragrance in citrus peel essential oils like orange, lemon, and grapefruit. You’ll find it in food manufacturing as a flavoring agent and in personal care products like aftershave, bath items, and hand cleansers. It’s also present in cleaning products for its oil-dissolving power and lemon-like scent.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Uses), s3 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · skipped by model · 1 snippets · 6.8 s


### lycopene

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Lycopene is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 8.9 s

> **The red in your food**  
> Lycopene gives red to pink hues in tomatoes, pink grapefruit, watermelon, red guavas, and baked beans. It's found naturally in these foods and is a major pigment in fruits like apricots and red oranges. As a carotenoid, it may help retard aging and degenerative diseases in animals.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Occurrence)  
Fun: _ /5

**Weird fact** · passed on retry · 1 snippets · 12.1 s

> **Most in blood are twisted forms**  
> Plants and photosynthetic bacteria produce all-trans lycopene. When exposed to light or heat, it forms cis-isomers. In human blood, various cis-isomers make up more than 60% of total lycopene.

Sources: s1 (Wikipedia, Structure and physical properties)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "This is surprising because plants produce the straight all-trans form, and the twisted forms are less stable."</summary>

```json
{"title": "Most in blood are twisted forms", "body": "In human blood, various cis-isomers make up more than 60% of total lycopene concentration. This is surprising because plants produce the straight all-trans form, and the twisted forms are less stable.", "sources": ["s1"]}
```
</details>


### menthol

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Menthol is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 1 snippets · 4.8 s

> **First isolated in 1771**  
> In 1771, German chemist Hieronymus David Gaubius first isolated menthol. Early characterizations were conducted by Oppenheim, Beckett, Moriya, and Atkinson. The name 'menthol' was proposed by F. L. Alphonse Oppenheim in 1861.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 9.7 s

> **In minty snacks and medicines**  
> Menthol is found naturally in peppermint oil from Mentha × piperita, giving minty flavor and a cooling sensation. It appears in foods and pharmaceuticals as a flavor enhancer and local anesthetic. Japanese menthol also contains a small amount of neomenthol, a 1-epimer.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Occurrence)  
Fun: _ /5

**Weird fact** · rejected · 1 snippets · 13.2 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "This specificity suggests a precise biological origin."</summary>

```json
{"title": "Menthol comes in just one natural form", "body": "Natural menthol exists as a single pure stereoisomer, the (1R,2S,5R) form. Out of eight possible stereoisomers, only this one is naturally found. This specificity suggests a precise biological origin.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "Out of eight possible stereoisomers, only this one is common in nature."</summary>

```json
{"title": "Menthol has one preferred form", "body": "Natural menthol exists as one pure stereoisomer, the (1R,2S,5R) form. Out of eight possible stereoisomers, only this one is common in nature.", "sources": ["s1"]}
```
</details>


### morphine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Morphine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed. A minority of reports list 5 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 7.1 s

> **From opium to morphine in 1804**  
> In December 1804, German pharmacist Friedrich Sertürner extracted morphine from the opium poppy plant in Paderborn. He noticed the compound induced drowsiness in rats and stray dogs and reported his findings in 1805. Sertürner named it morphium after the Greek god of dreams, Morpheus, due to its sleep-inducing effect.

Sources: s3 (Wikipedia, History)  
Fun: _ /5

**Weird fact** · rejected · 1 snippets · 12.1 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "Sertürner administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died."; not stated in the cited snippets: "This shows how potent morphine was even in early experiments."</summary>

```json
{"title": "Morphine nearly killed those who tried it", "body": "Sertürner administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died. This shows how potent morphine was even in early experiments.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "Friedrich Sertürner administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died."; not stated in the cited snippets: "This shows how potent morphine was even in early experiments."</summary>

```json
{"title": "Morphine made people almost die", "body": "Friedrich Sertürner administered morphine to himself, three young boys, three dogs, and a mouse; all four people almost died. This shows how potent morphine was even in early experiments. Sertürner originally named the substance morphium after the Greek god of dreams, Morpheus, as it has a tendency to cause sleep.", "sources": ["s1"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 12.2 s

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

**Origin story** · passed · 4 snippets · 6.8 s

> **From tobacco to isolation**  
> In 1828, Wilhelm Heinrich Posselt and Karl Ludwig Reimann isolated nicotine from the tobacco plant in Germany, believing it was a poison. They identified its empirical formula in 1843 by Melsens, and its structure was discovered in 1893 by Adolf Pinner and Richard Wolffenstein. Nicotine was first synthesized in 1904 by Amé Pictet and A. Rotschy.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · rejected · 3 snippets · 15.3 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "People encounter it through smoking, vaping, or nicotine replacement therapies like patches and gums."</summary>

```json
{"title": "In tobacco and nightshade plants", "body": "Nicotine is found in tobacco leaves, making up 0.6–3% of their dry weight. It also occurs in trace amounts in tomatoes, potatoes, and eggplants. People encounter it through smoking, vaping, or nicotine replacement therapies like patches and gums.", "sources": ["s1", "s2", "s3"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "In tobacco and nicotine products"</summary>

```json
{"title": "In tobacco and nicotine products", "body": "Nicotine is found in tobacco leaves, constituting 0.6–3% of their dry weight. It also occurs in trace amounts in tomatoes, potatoes, and eggplants. People encounter it through tobacco smoking, vaping, or other nicotine delivery systems. Nicotine replacement therapy products like gums, patches, and lozenges deliver it medically.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · passed · 1 snippets · 9.0 s

> **Nicotine insecticide use dropped sharply**  
> After World War II, over 2,500 tons of nicotine insecticide were used worldwide. By the 1980s, use had declined below 200 tons. This decline was due to cheaper and less harmful alternatives.

Sources: s1 (Wikipedia, History)  
Fun: _ /5


### nitroglycerin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Nitroglycerin carries the official GHS signal word "Danger". Its hazard statements: explosive; mass explosion hazard, fatal if swallowed, fatal in contact with skin, fatal if inhaled, and may causes damage to organs through prolonged or repeated exposure. The official list has 1 more.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.6 s

> **First made in 1846 by Sobrero**  
> In 1846, Italian chemist Ascanio Sobrero synthesized nitroglycerin while working under Théophile-Jules Pelouze at the University of Turin. He named it 'pyroglycerin' and warned against its use as an explosive. The compound was later adopted by Alfred Nobel for commercial use after a deadly explosion in 1864.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · rejected · 3 snippets · 17.4 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "These formulations come in various doses and are applied in both medical and civil engineering contexts."</summary>

```json
{"title": "In heart pills and dynamite", "body": "Nitroglycerin is found in medications for angina pectoris and as an antihypertensive, where it dilates veins to reduce heart preload. It is also present in dynamite, patented in 1867, used in tunnel drilling, mining, and demolition. These formulations come in various doses and are applied in both medical and civil engineering contexts.", "sources": ["s1", "s2", "s3"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "In explosives, it was named dynamite and patented in 1867, supplied as sticks wrapped in greased waterproof paper."</summary>

```json
{"title": "In medicine and in dynamite", "body": "Nitroglycerin is probably most commonly prescribed for angina pectoris and used as a potent antihypertensive agent. It works by dilating veins more than arteries, reducing preload. In explosives, it was named dynamite and patented in 1867, supplied as sticks wrapped in greased waterproof paper. It was used in civil engineering tasks like tunnel drilling and quarrying.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · passed · 1 snippets · 9.0 s

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

**Origin story** · passed on retry · 4 snippets · 15.4 s

> **Jesuit apothecary sends bark to Rome**  
> In the 1570s, Jesuit Agostino Salumbrino, an apothecary in Lima, observed the Quechua using cinchona bark to treat shivering. He sent a small quantity to Rome for testing as a malaria treatment. By the 17th century, cinchona bark was known as Jesuit's bark and became a valuable commodity shipped from Peru to Europe.

Sources: s2 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "Spanish Jesuit missionaries, including Agostino Salumbrino, observed this practice and sent cinchona bark to Rome for testing in the 17th century."; not stated in the cited snippets: "Salumbrino, an apothecary in Lima, sent a small quantity to Rome to evaluate its effect on malaria-induced shivering."</summary>

```json
{"title": "From Quechua bark to Jesuit testing", "body": "The Quechua Indians of Peru, Bolivia, and Ecuador used ground cinchona bark mixed with sweetened water to treat shivering. Spanish Jesuit missionaries, including Agostino Salumbrino, observed this practice and sent cinchona bark to Rome for testing in the 17th century. Salumbrino, an apothecary in Lima, sent a small quantity to Rome to evaluate its effect on malaria-induced shivering.", "sources": ["s1", "s2", "s4"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 10.5 s

> **In tonic water and malaria treatment**  
> Quinine appears in tonic water and bitter lemon to give a bitter taste. It is used medically to treat malaria, especially in areas with resistance to chloroquine, and for nocturnal leg cramps—though side effects are a concern. It is also found in medicines as an alkaloid or salt.

Sources: s1 (Wikipedia, Introduction), s2 (PubChem, Uses), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.8 s

> **Quinine suppresses brain chemicals**  
> Quinine can act as a competitive inhibitor of monoamine oxidase (MAO), an enzyme that removes neurotransmitters from the brain. As an MAO inhibitor, it has potential to serve as a treatment for psychological disorders, similar to antidepressants.

Sources: s1 (Wikipedia, Uses)  
Fun: _ /5


### serotonin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Serotonin carries the official GHS signal word "Danger". Its hazard statements: toxic if swallowed, and suspected of damaging fertility or the unborn child.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 7.3 s

> **From enteramine to serotonin**  
> In 1935, Vittorio Erspamer in Pavia discovered a substance from enterochromaffin cells that caused intestinal contraction, which he named 'enteramine'. Two years later, he confirmed it was a new amine. In 1948, Maurice M. Rapport, Arda Green, and Irvine Page discovered a vasoconstrictor in blood serum and named it serotonin. In 1952, enteramine was shown to be the same as serotonin.

Sources: s1 (Wikipedia, History & etymology), s2 (Wikipedia, History & etymology)  
Fun: _ /5

**Where you meet it** · passed on retry · 3 snippets · 13.6 s

> **Where serotonin shows up**  
> Serotonin is found in the gastrointestinal tract, where it regulates intestinal movements. It also appears in the brainstem, skin, and taste cells in smaller amounts. In blood platelets, about 8% of the body's serotonin is stored and released during clotting to promote vasoconstriction and platelet aggregation.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Introduction)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "A smaller amount is in the brainstem and skin, while 8% is stored in blood platelets, which release it during clotting."</summary>

```json
{"title": "In your gut and brain", "body": "Serotonin is mostly found in the gastrointestinal tract, where it regulates intestinal movements. A smaller amount is in the brainstem and skin, while 8% is stored in blood platelets, which release it during clotting. In the central nervous system, it helps regulate mood, appetite, and sleep.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · passed · 1 snippets · 8.9 s

> **A parasite makes you feel sick with serotonin**  
> The parasite Entamoeba histolytica secretes serotonin, causing sustained secretory diarrhea. It also becomes more virulent when exposed to serotonin. Patients infected with it have highly elevated serotonin levels that return to normal after recovery.

Sources: s1 (Wikipedia, Comparative biology & evolution)  
Fun: _ /5


### strychnine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Strychnine carries the official GHS signal word "Danger". Its hazard statements: fatal if swallowed, fatal in contact with skin, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 8.1 s

> **First discovered in a bean**  
> In 1818, French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier first discovered strychnine in Saint-Ignatius' bean (Strychnos ignatii). The compound was isolated from plants in the genus Strychnos, where it has long been known for its toxic properties. Historic records suggest its use in Europe as early as 1640 to kill animals and humans.

Sources: s3 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 9.4 s

> **In seeds of the Strychnos tree**  
> Strychnine is found in the seeds of the Strychnos nux-vomica tree and other plants like Strychnos ignatii and Strychnos wallichiana. It is used as a pesticide to kill birds and rodents. When inhaled, swallowed, or absorbed through the eyes or mouth, it causes muscular convulsions and death through asphyxia.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · rejected · 1 snippets · 12.3 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "This use is particularly disturbing given its extreme toxicity and the context of wartime violence."</summary>

```json
{"title": "Used to kill civilians in WWII", "body": "Strychnine was allegedly used by Oskar Dirlewanger during World War II against civilians. This use is particularly disturbing given its extreme toxicity and the context of wartime violence.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "This use is documented in historic records."</summary>

```json
{"title": "Used to kill civilians in WWII", "body": "Strychnine was allegedly used by Oskar Dirlewanger during World War II against civilians. This use is documented in historic records. It was also reportedly used by William Palmer to kill John Cook.", "sources": ["s1"]}
```
</details>


### sucralose

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Sucralose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 4 snippets · 13.0 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "In 1975, scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College, mixed sulfuryl chloride with sugar."; not stated in the cited snippets: "This led to the discovery of sucralose."</summary>

```json
{"title": "A chlorinated sugar test", "body": "In 1975, scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College, mixed sulfuryl chloride with sugar. Phadnis was told to 'test' a chlorinated sugar compound and, believing Hough asked him to 'taste' it, did so and found it to be exceptionally sweet. This led to the discovery of sucralose.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "In 1975, scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College, mixed sulfuryl chloride with sugar."; not stated in the cited snippets: "The compound was sucralose."</summary>

```json
{"title": "A sweet taste from a chlorinated sugar", "body": "In 1975, scientists from Tate & Lyle, working with researchers Leslie Hough and Shashikant Phadnis at Queen Elizabeth College, mixed sulfuryl chloride with sugar. Phadnis was told to 'test' a chlorinated sugar compound and, believing Hough asked him to 'taste' it, he did and found it to be exceptionally sweet. The compound was sucralose.", "sources": ["s1"]}
```
</details>

**Where you meet it** · rejected · 3 snippets · 14.9 s

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "It's also in Splenda, which mixes sucralose with dextrose and maltodextrin, with sucralose making up only about 1.1% of the product."; not stated in the cited snippets: "Used in baked goods, confections, and table-top sugar substitutes, it adds sweetness without calories or affecting insulin levels."</summary>

```json
{"title": "Sweetness in your coffee and candy", "body": "Sucralose is found in coffee pods, breakfast bars, soft drinks, and canned fruits, where it replaces corn syrup-based additives. It's also in Splenda, which mixes sucralose with dextrose and maltodextrin, with sucralose making up only about 1.1% of the product. Used in baked goods, confections, and table-top sugar substitutes, it adds sweetness without calories or affecting insulin levels.", "sources": ["s1", "s2", "s3"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "In Splenda, sucralose is mixed with dextrose or maltodextrin, making up about 1.1% of the product."</summary>

```json
{"title": "In candy, coffee pods, and canned fruits", "body": "Sucralose is found in candy, breakfast bars, coffee pods, and soft drinks. It appears in canned fruits where water and sucralose replace corn syrup-based additives. In Splenda, sucralose is mixed with dextrose or maltodextrin, making up about 1.1% of the product.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · rejected · 1 snippets · 12.8 s

<details><summary>Attempt 1 rejected: title has 11 words, at most 8 allowed</summary>

```json
{"title": "He tasted a sweet compound thinking it was a taste test", "body": "Shashikant Phadnis was told to 'test' a chlorinated sugar compound and mistakenly thought it meant to taste it. He found it exceptionally sweet, leading to the discovery of sucralose. This accidental tasting sparked a major breakthrough in artificial sweeteners.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: not stated in the cited snippets: "This accidental taste test led to the discovery of sucralose."</summary>

```json
{"title": "He tasted a sweet sugar compound", "body": "Shashikant Phadnis was told to 'test' a chlorinated sugar compound. He thought Hough asked him to 'taste' it, so he did and found it exceptionally sweet. This accidental taste test led to the discovery of sucralose.", "sources": ["s1"]}
```
</details>


### tetrodotoxin

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Tetrodotoxin carries the official GHS signal word "Danger". Its hazard statements: fatal if swallowed, fatal in contact with skin, and fatal if inhaled.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 10.2 s

> **Tetrodotoxin linked to zombieism in Haiti**  
> Tetrodotoxin has been alleged to cause zombieism and is said to have been used in Haitian Vodou preparations by a sorcerer called the bokor. This idea first appeared in Zora Neale Hurston's 1938 book Tell My Horse and was later popularized by Wade Davis's 1985 book and Wes Craven's 1988 film, both titled The Serpent and the Rainbow.

Sources: s1 (Wikipedia, Popular culture)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 13.0 s

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

**Origin story** · passed · 4 snippets · 7.5 s

> **From discarded by-product to a 'wonder drug'**  
> In 1952, Chemical Industry Basel synthesized thalidomide but discarded it because it had no effect on animals. In 1957, Chemie Grünenthal acquired it and Heinrich Mückter led the discovery program. While preparing reagents, Mückter's assistant Wilhelm Kunz isolated a by-product recognized by pharmacologist Herbert Keller as an analog of glutethimide.

Sources: s1 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 10.2 s

> **In cancer treatments and rare skin conditions**  
> Thalidomide is found in medicines like Thalomid and Distaval, used to treat multiple myeloma and leprosy-related skin lesions. It is also prescribed for HIV-related complications and erythema nodosum leprosum. Once widely sold for morning sickness, it was removed from the market in 1961 after causing severe birth defects in over 10,000 possibly 20,000 infants.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction), s3 (Wikipedia, Medical uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 9.5 s

> **A discarded drug became a miracle cure**  
> Thalidomide was discarded in 1952 after being found to have no effect on animals. It was later discovered that a by-product isolated by Wilhelm Kunz was an analog of a sedative, marking a surprising pivot in its history.

Sources: s1 (Wikipedia, History)  
Fun: _ /5


### theobromine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Theobromine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed. A minority of reports list 2 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 3 snippets · 4.5 s

> **Discovered in cacao beans**  
> In 1841, chemist A. Woskresensky discovered theobromine in cacao beans. This discovery was followed by the first synthesis of theobromine from xanthine in 1882 by Hermann Emil Fischer.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed on retry · 3 snippets · 14.4 s

> **In chocolate, tea, and cola nuts**  
> Theobromine is found in chocolate and the cacao plant. It also appears in tea (Camellia sinensis), cola nuts, and some American hollies (yaupon and guayusa). It is a white or colourless solid, though commercial samples may appear yellowish.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (PubChem, Natural Pollution Sources)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "Sour chocolate and tea"</summary>

```json
{"title": "Sour chocolate and tea", "body": "Theobromine is found in chocolate, tea (Camellia sinensis), American hollies (yaupon and guayusa), and the kola nut. It is also present in cacao beans and appears as a white or colourless solid, though commercial samples may be yellowish.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · passed · 1 snippets · 9.4 s

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

**Origin story** · passed · 4 snippets · 7.1 s

> **From Mesoamerica to Germany**  
> Native Mesoamerican peoples, such as the Totonacs of modern-day Veracruz, Mexico, discovered and cultivated vanilla as a flavoring for beverages by the early 15th century. The Aztecs used vanilla in chocolate drinks called xocohotl. In 1874, Ferdinand Tiemann and Wilhelm Haarmann deduced vanillin's structure and developed a synthesis from coniferin, starting industrial production in Holzminden, Germany.

Sources: s2 (Wikipedia, Natural history), s3 (Wikipedia, Synthetic history)  
Fun: _ /5

**Where you meet it** · passed on retry · 3 snippets · 15.2 s

> **In chocolate and ice cream**  
> Vanillin is the primary component of vanilla bean extract and is widely used as a flavoring. It appears in sweet foods, especially in the ice cream and chocolate industries, which together make up 75% of vanillin's market. Synthetic vanillin is now more commonly used than natural extract in foods, beverages, and pharmaceuticals.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Natural history), s3 (Wikipedia, Uses)  
Fun: _ /5

<details><summary>Attempt 1 rejected: not stated in the cited snippets: "It appears in ice cream and chocolate, which together make up 75% of vanillin's market."</summary>

```json
{"title": "Scent of chocolate and ice cream", "body": "Vanillin is the primary component of vanilla bean extract and is widely used in sweet foods. It appears in ice cream and chocolate, which together make up 75% of vanillin's market. Synthetic vanillin is now more common than natural extract in foods, beverages, and pharmaceuticals.", "sources": ["s1", "s3"]}
```
</details>

**Weird fact** · passed · 1 snippets · 10.1 s

> **Vanillin in 2nd millennium BCE jars**  
> Vanillin residue was found in jars within a tomb in Canaan dating to the 2nd millennium BCE, suggesting possible cultivation of an unidentified Vanilla species there since the Middle Bronze Age. Traces were also found in wine jars in Jerusalem used by the Judahite elite before 586 BCE.

Sources: s1 (Wikipedia, Natural history)  
Fun: _ /5

