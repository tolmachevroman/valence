# Spike run: Qwen3-4B-Instruct-2507-Q4_K_M

30 molecules, 111 cards with input. Temperature 0.6, JSON schema constraint, one stricter retry, shortest card first, KV cache: paged, fixed 8K tokens.

## Summary

| Measure | Value |
| --- | --- |
| Passed first try | 89 (80%) |
| Passed on retry | 17 (15%) |
| Skipped by model | 1 (1%) |
| Rejected twice (dropped) | 4 (4%) |
| No input (no card) | 9 |
| Molecules with 3+ cards | 29 of 30 |
| First token | 2.0 s median, 5.1 s p90, 5.9 s max |
| Card, including retry | 6.2 s median, 13.5 s p90, 17.5 s max |
| First card per molecule | 2.1 s median, 3.3 s p90, 4.2 s max |
| Model load | 7.6 s |
| Peak memory | 4.73 GB |

### Problems found (all attempts)

- name not in sources: 10
- number not in sources: 3
- sentence count: 6
- title: 9

## Cards

Fun score is for hand review in M0.6.

### allicin

**Danger level** · passed · 1 snippets · 2.9 s

> **May irritate skin, eyes, and lungs**  
> Allicin can cause skin irritation and serious eye irritation. It may also cause respiratory tract irritation when inhaled.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 4.4 s

> **Discovered in Japan in the 1940s**  
> In 1944, Chester J. Cavallito and John Hays Bailey first isolated and studied allicin in the laboratory. It was discovered during efforts to create thiamine derivatives in Japan. Allicin became a model for developing other thiamine disulfides like sulbutiamine, fursultiamine, and benfothiamine.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed on retry · 7 snippets · 8.8 s

> **In garlic and leeks**  
> Allicin is obtained from garlic and leeks when fresh tissue is chopped or crushed. It is formed by the enzyme alliinase acting on alliin. The molecule gives garlic its distinctive odor and acts as a defense against pests.

Sources: s4 (Wikipedia, Introduction), s6 (Wikipedia, Structure and occurrence)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Follicular is not in the cited snippets; name Lymphoma is not in the cited snippets</summary>

```json
{"title": "The pungent power of garlic", "body": "Allicin forms when garlic is chopped or crushed, giving fresh garlic its sharp odor. It's found in garlic and leeks and acts as a natural defense against pests. This molecule is also studied in trials for treating Follicular Lymphoma.", "sources": ["s1", "s3", "s4", "s5", "s6", "s7"]}
```
</details>

**Weird fact** · passed · 11 snippets · 5.7 s

> **Garlic's smell comes from a broken enzyme reaction**  
> Allicin is formed only when garlic is damaged, triggering the enzyme alliinase to convert alliin into allicin. The surprise? This reaction is a defense mechanism, not just a smell — it's a biological antifeedant that deters pests.

Sources: s3 (Wikipedia, Structure and occurrence), s7 (Wikipedia, Introduction)  
Fun: _ /5


### artemisinin

**Danger level** · passed · 1 snippets · 2.1 s

> **Flammable and harmful to water life**  
> Artemisinin is classified as self-reactive and may cause fire when heated. It is very toxic to aquatic life, both acutely and with long-lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.3 s

> **From ancient herb to antimalarial breakthrough**  
> In 1596, Li Shizhen recommended tea made from sweet wormwood to treat malaria in his Compendium of Materia Medica. In 1967, a secret military program called 'Project 523' was launched under the People's Liberation Army to find a malaria treatment, ordered by Mao Zedong at the request of North Vietnamese leaders. In 1972, Tu Youyou discovered artemisinin in the leaves of Artemisia annua, inspired by a 340 CE text by Ge Hong recommending cold water steeping.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 10 snippets · 6.4 s

> **In your malaria treatment**  
> Artemisinin is found in sweet wormwood (Artemisia annua) and is used in traditional Chinese medicine. It’s a key ingredient in artemisinin-based combination therapies (ACTs), the global standard for treating malaria. These ACTs are recommended by the WHO for both uncomplicated and severe malaria cases.

Sources: s1 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (Wikipedia, Medical use), s10 (Wikipedia, Medical use)  
Fun: _ /5

**Weird fact** · rejected · 10 snippets · 15.3 s

<details><summary>Attempt 1 rejected: title has 10 words, at most 8 allowed</summary>

```json
{"title": "A malaria drug that turns itself on with blood iron", "body": "Artemisinin's antimalarial power comes from a ring that breaks when it meets iron(II) in red blood cells, releasing free radicals that kill the parasite. This means the drug is activated inside the body — not by a pill, but by the very blood it targets.", "sources": ["s10"]}
```
</details>

<details><summary>Attempt 2 rejected: title has 12 words, at most 8 allowed</summary>

```json
{"title": "A malaria drug kills parasites with a ring that breaks on iron", "body": "Artemisinin has an endoperoxide 1,2,4-trioxane ring that breaks when it contacts iron(II) oxide in red blood cells. This breakage produces free radicals that destroy the malaria parasite. The mechanism was confirmed in 2016, showing artemisinin acts on many targets, suggesting a promiscuous effect.", "sources": ["s8", "s10"]}
```
</details>


### ascorbic acid

**Danger level** · passed · 1 snippets · 1.7 s

> **No known hazards per GHS**  
> Ascorbic acid is not classified as hazardous under GHS criteria by the reporting company. No warning pictograms or hazard statements are assigned.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.7 s

> **Lind's citrus experiment**  
> In May 1747, James Lind, a ship's surgeon in the Royal Navy, conducted one of the first controlled experiments by giving some crew members oranges and lemons while others received cider, vinegar, sulfuric acid, or seawater. He found citrus fruits prevented scurvy. Lind published his findings in 1753 in his Treatise on the Scurvy.

Sources: s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 17 snippets · 8.4 s

> **In your citrus and in your medicine**  
> You'll find ascorbic acid in citrus fruits, tomatoes, potatoes, and green leafy vegetables. It's also used as a food additive (E300) to prevent browning and preserve color in canned fruits. As a vitamin, it's found in supplements and used to treat scurvy.

Sources: s1 (PubChem, Record Description), s3 (PubChem, Record Description), s5 (PubChem, Record Description), s9 (Wikipedia, Food preservation additive), s10 (Wikipedia, Food preservation additive), s15 (PubChem, Natural Pollution Sources), s16 (PubChem, Natural Pollution Sources), s7 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed on retry · 15 snippets · 15.7 s

> **Vitamin C is a cofactor for collagen synthesis**  
> Ascorbic acid is a cofactor for enzymes that synthesize collagen, a key structural protein in connective tissues. This role is vital in preventing scurvy, where deficiency leads to weakened tissues and bleeding. The function is so critical that vitamin C is essential for maintaining skin, blood vessels, and bones.

Sources: s13 (Wikipedia, Pharmacology), s14 (Wikipedia, Pharmacology), s15 (Wikipedia, Health effects)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name U is not in the cited snippets</summary>

```json
{"title": "Vitamin C is the 226th most prescribed drug", "body": "Ascorbic acid is not just a nutrient—it's the 226th most commonly prescribed medication in the U.S. in 2023, with over 1 million prescriptions. This surprising level of medical use highlights its critical role in treating scurvy and supporting enzyme functions, despite limited evidence for other disease treatments.", "sources": ["s15"]}
```
</details>


### aspirin

**Danger level** · passed · 1 snippets · 1.8 s

> **Avoid swallowing and eye contact**  
> Aspirin is harmful if swallowed and can cause skin and eye irritation. It may also irritate the respiratory tract.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.4 s

> **From willow bark to pure aspirin**  
> In 1853, Charles Frédéric Gerhardt first produced acetylsalicylic acid by treating sodium salicylate with acetyl chloride. In 1897, Felix Hoffmann (or Arthur Eichengrün) at Bayer synthesized it in a pure, stable form. This compound was derived from salicylic acid, extracted in 1839 from meadowsweet, which inspired the name 'Aspirin'.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 13 snippets · 7.3 s

> **In your medicine cabinet and beyond**  
> Aspirin, known as acetylsalicylic acid (ASA), is found in tablets for pain, fever, and inflammation. It’s also used long-term to prevent heart attacks and strokes in high-risk individuals. You might encounter it in treatments for conditions like rheumatoid arthritis, pericarditis, and Kawasaki disease.

Sources: s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s9 (Wikipedia, Medical use), s11 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · rejected · 14 snippets · 16.8 s

<details><summary>Attempt 1 rejected: name U is not in the cited snippets</summary>

```json
{"title": "Aspirin has a hidden explosive risk", "body": "Aspirin powder can form explosive mixtures with air. Fact: factories producing aspirin must control airborne powder levels because the mixture can ignite. The U.S. set a safe exposure limit of 5 mg/m³ in 1989, though it was later overturned in 1993.", "sources": ["s8"]}
```
</details>

<details><summary>Attempt 2 rejected: number 1897 is not in the cited snippets</summary>

```json
{"title": "Aspirin can explode in factories", "body": "Aspirin powder-air mixtures can be explosive, requiring strict controls in manufacturing. Factories must limit airborne powder to 5 mg/m³, a limit set by NIOSH in 1897 and later reaffirmed by OSHA in 1989, though it was vacated in 1993. This risk is due to the powder's combustible nature, not its chemical reactivity in the body.", "sources": ["s8"]}
```
</details>


### batrachotoxin

**Danger level** · no input · 0 snippets · -0.0 s

**Origin story** · passed · 1 snippets · 4.2 s

> **Toxin from a frog's skin**  
> In 1963, Fritz Märki and Bernhard Witkop isolated the toxic alkaloids fraction from the black-legged poison dart frog at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. They identified four major toxic steroidal alkaloids, including batrachotoxin, homobatrachotoxin, pseudobatrachotoxin, and batrachotoxinin A. The process was challenging due to the toxin's potency and the tiny amounts available.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 12 snippets · 8.7 s

> **In frog skin and darts**  
> Batrachotoxin is found in the skin of poison dart frogs (genus Phyllobates and Dendrobates), certain beetles (genus Choresine), and birds (genus Pitohui and Ifrita). It's used by the Noanamá Chocó and Emberá Chocó people to poison blowgun darts for hunting. The toxin binds to sodium channels, causing paralysis and death, and is one of the most potent non-peptidal neurotoxins known.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (Wikipedia, Introduction), s6 (Wikipedia, Use), s7 (Wikipedia, Use), s8 (PubChem, Uses), s9 (PubChem, Uses), s11 (PubChem, Uses), s12 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 16 snippets · 7.0 s

> **Frogs release poison when in pain**  
> Poison dart frogs release their toxic batrachotoxin when agitated, feeling threatened, or in pain. This reflexive response means the frog's own distress triggers the poison release — a biological alarm system that protects it from predators.

Sources: s3 (Wikipedia, Toxicity)  
Fun: _ /5


### benzylpenicillin

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 2.7 s

> **Allergies and aquatic risks**  
> Benzylpenicillin may cause allergic skin reactions and respiratory issues if inhaled. It is also very toxic to aquatic life and harmful to aquatic organisms with long-lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 12 snippets · 6.8 s

> **Injections for infections**  
> Benzylpenicillin, known as Penicillin G, is found in injectable forms used to treat infections like strep throat, syphilis, and gas gangrene. It's administered intravenously or intramuscularly due to poor oral absorption. Found in the WHO List of Essential Medicines, it's also used in veterinary medicine and approved in Taiwan.

Sources: s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Medical uses), s11 (PubChem, Uses), s12 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 15 snippets · 6.4 s

> **It causes seizures in the brain**  
> Benzylpenicillin can cause central nervous system toxicity, including convulsions — especially in high doses or in people with severe kidney failure. This is surprising because it's considered low-toxicity overall, making it one of the most active β-lactam drugs in this regard.

Sources: s2 (Wikipedia, Adverse effects), s3 (Wikipedia, Adverse effects)  
Fun: _ /5


### buckminsterfullerene

**Danger level** · passed · 1 snippets · 1.8 s

> **Avoid skin, eyes, and lungs**  
> Buckminsterfullerene may cause skin irritation and serious eye irritation. It can also cause respiratory tract irritation if inhaled.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 4 snippets · 12.7 s

<details><summary>Attempt 1 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "Laser vaporization sparks discovery", "body": "In 1984, Eric Rohlfing, Donald Cox, and Andrew Kaldor generated buckminsterfullerene using laser vaporization of carbon in a helium beam, but did not recognize the structure. In 1985, Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University repeated the experiment and identified C60 as a cage-like molecule with 60 carbon atoms. Their work revealed a strong peak at 720 daltons and proposed an icosahedral symmetry structure.", "sources": ["s1", "s2", "s3", "s4"]}
```
</details>

<details><summary>Attempt 2 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "Laser vaporization reveals C60", "body": "In 1984, Eric Rohlfing, Donald Cox, and Andrew Kaldor generated buckminsterfullerene using a laser to vaporize carbon in a supersonic helium beam, though they did not recognize the structure. In 1985, Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University repeated the experiment and identified the C60 structure as buckminsterfullerene. They found a strong peak at 720 daltons and concluded it was a spheroidal molecule with icosahedral symmetry.", "sources": ["s1", "s2", "s3", "s4"]}
```
</details>

**Where you meet it** · passed · 16 snippets · 6.2 s

> **Buckyballs in soot and space**  
> You can find buckminsterfullerene in soot and in space, where it appears in planetary nebulae and star environments. It's also present in shungite, a rare green rock. Though not common in daily products, it's studied in solar cells and high-performance polymers.

Sources: s8 (Wikipedia, Occurrence), s9 (Wikipedia, Occurrence), s16 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed on retry · 12 snippets · 15.0 s

> **C60 found in space and soot**  
> Buckminsterfullerene is the most common naturally occurring fullerene. It can be found in soot and has been observed in planetary nebulae and stars. Its ionized form, C+60, causes diffuse interstellar bands in the near-infrared.

Sources: s5 (Wikipedia, Occurrence), s6 (Wikipedia, Occurrence)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Earth is not in the cited snippets; title has 11 words, at most 8 allowed</summary>

```json
{"title": "Found in space and soot, it's the most common natural fullerene", "body": "Buckminsterfullerene is the most common naturally occurring fullerene. It's found in soot and in space, including planetary nebulae and star environments. This dual presence—on Earth and in the cosmos—makes it surprisingly widespread.", "sources": ["s5", "s6"]}
```
</details>


### caffeine

**Danger level** · passed · 1 snippets · 1.6 s

> **Avoid swallowing — can be harmful**  
> Caffeine is classified as harmful if swallowed. It acts as an irritant and poses acute toxicity risks when ingested.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · skipped by model · 4 snippets · 1.0 s

**Weird fact** · passed · 15 snippets · 7.4 s

> **Caffeine stops drowsiness by blocking sleep signals**  
> Caffeine works by blocking adenosine receptors in the brain, which normally promote drowsiness. When caffeine binds to these receptors, it prevents the brain from feeling sleepy—so you stay alert. This mechanism is so precise that knocking out the A2A receptor in mice mimics caffeine’s wakefulness effect.

Sources: s13 (Wikipedia, Pharmacology), s14 (Wikipedia, Pharmacology)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 10.9 s

> **In your coffee, tea, and chocolate**  
> Caffeine shows up in coffee beans, tea leaves, cocoa beans, and kola nuts — typically at 1.3% in coffee beans or 0.9–2.6% in green coffee beans. It’s also found in mate leaves, guarana paste, and yaupon holly. You’ll encounter it in beverages, cosmetics, and as a treatment for apnea of prematurity, where caffeine citrate was first approved by the FDA in 1999.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Uses), s10 (Wikipedia, Uses), s11 (Wikipedia, Natural occurrence), s12 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Uses), s16 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### capsaicin

**Danger level** · passed · 1 snippets · 1.8 s

> **Avoid ingestion and eye contact**  
> Capsaicin is toxic if swallowed and harmful if ingested. It causes skin irritation and can lead to serious eye damage.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 3.3 s

> **First extracted in 1816**  
> In 1816, Christian Friedrich Bucholz first extracted capsaicin in impure form. This marked the earliest known discovery of the compound. The burning sensation was later linked to capsaicin by Rudolf Buchheim in 1873 and Endre Hőgyes in 1878.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed on retry · 15 snippets · 16.0 s

> **In chili peppers and hot sauces**  
> Capsaicin is found in the placental tissue and white pith of chili peppers, especially in Capsicum annuum and C. frutescens. It gives heat to foods like chili powder and paprika. It is also used in topical analgesics and as a flavoring agent in food products.

Sources: s3 (PubChem, Record Description), s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (Wikipedia, Uses), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Tabasco is not in the cited snippets; name Mexican is not in the cited snippets</summary>

```json
{"title": "The heat in your chili", "body": "You meet capsaicin in hot peppers like Capsicum annuum and C. frutescens, where it's concentrated in the white pith around the seeds. It's also in spicy foods like Tabasco sauce and Mexican salsa, and in products such as chili powder and pepper spray. The molecule is used in 8% capsaicin patches for neuropathic pain and as a bird repellent.", "sources": ["s3", "s4", "s6", "s7", "s9", "s11", "s14", "s15"]}
```
</details>

**Weird fact** · passed on retry · 14 snippets · 15.0 s

> **Birds don't feel the burn**  
> Capsaicin causes burning in mammals but not in birds, because birds lack the TRPV1 receptor. This helps chili seeds survive digestion and spread via birds, not mammals. The plant evolved capsaicin to avoid being eaten by animals that destroy seeds.

Sources: s4 (Wikipedia, Natural function)  
Fun: _ /5

<details><summary>Attempt 1 rejected: title has 11 words, at most 8 allowed</summary>

```json
{"title": "Birds don't feel the burn, so chilies evolved to fool mammals", "body": "Chili peppers produce capsaicin to deter mammals, which are sensitive to its pain. Birds, however, don’t feel the burn because their TRPV1 receptors don’t respond to it—making them ideal dispersers of seeds. This evolutionary trick ensures seeds survive digestion and germinate later.", "sources": ["s3", "s4"]}
```
</details>


### carvone

**Danger level** · passed on retry · 1 snippets · 3.3 s

> **Allergic skin reaction possible**  
> Carvone may cause an allergic skin reaction. The hazard is classified under sensitization to the skin. Signal word is Warning. Pictogram shows irritant.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "May cause skin allergies", "body": "Carvone is classified as a skin sensitizer and may cause an allergic skin reaction.", "sources": ["s1"]}
```
</details>

**Origin story** · passed · 1 snippets · 3.2 s

> **From caraway to carvone**  
> In 1849, Franz Varrentrapp obtained carvone from caraway and first isolated it as a pure compound. It was initially called carvol by Schweizer. Goldschmidt and Zürrer identified it as a ketone related to limonene. The structure was finally elucidated by Georg Wagner in 1894.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 14 snippets · 8.2 s

> **Sour mint and caraway spice**  
> You’ll find carvone in spearmint and caraway seeds, where it makes up 50–80% of the essential oil. It’s also in dill seed oil (40–60%) and mandarin orange peel oil. Used in flavoring liqueurs, soaps, dentifrices, and as a pesticide to stop potatoes from sprouting.

Sources: s5 (Wikipedia, Introduction), s8 (Wikipedia, Occurrence), s9 (PubChem, Uses), s10 (PubChem, Uses), s11 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Natural Pollution Sources), s14 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 16 snippets · 8.4 s

> **Mint and caraway smell mirror images**  
> R-(−)-Carvone smells like spearmint, while its mirror image, S-(+)-carvone, smells like caraway. This shows that our noses detect tiny differences in molecular handedness — a surprising link between chemistry and smell.

Sources: s4 (Wikipedia, Stereoisomerism and odor), s5 (Wikipedia, Stereoisomerism and odor), s14 (PubChem, Record Description), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### cinnamaldehyde

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed on retry · 1 snippets · 3.3 s

> **Allergic skin reaction possible**  
> Cinnamaldehyde may cause an allergic skin reaction. The hazard is classified as a skin sensitizer under GHS. Signal word is Warning. Pictograms include Irritant.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "May cause skin allergies", "body": "Cinnamaldehyde is classified as a skin sensitizer and may cause an allergic skin reaction.", "sources": ["s1"]}
```
</details>

**Where you meet it** · passed · 15 snippets · 8.1 s

> **Sour spice in your snack**  
> You meet cinnamaldehyde in cinnamon bark oil, where it makes up 75% of the oil from Sri Lanka cinnamon. It’s the flavor behind chewing gum, ice cream, and beverages, used at levels from 9 to 4,900 ppm. Also found in grapefruit, celery seed, and clove bud, it gives sweet, spicy scents and repels cats and mosquitoes.

Sources: s2 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Applications), s9 (PubChem, Uses), s13 (PubChem, Natural Pollution Sources), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 19 snippets · 8.7 s

> **Cinnamon's scent is in grapefruit too**  
> Cinnamaldehyde, the compound behind cinnamon's smell, is also found in grapefruit, celery seed, clove bud, and lemon balm. This means the aroma of cinnamon isn't just in cinnamon trees—it's hiding in unexpected fruits and herbs too.

Sources: s17 (PubChem, Natural Pollution Sources), s16 (PubChem, Natural Pollution Sources), s14 (PubChem, Record Description), s11 (PubChem, Record Description), s9 (Wikipedia, Introduction)  
Fun: _ /5


### cubane

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · no input · 0 snippets · -0.0 s

**Where you meet it** · passed · 2 snippets · 3.8 s

> **Not in your daily life yet**  
> Cubane is a synthetic compound first made in 1964 and is not found in nature or common consumer products. It is studied for high-energy applications like explosives, such as octanitrocubane and heptanitrocubane, due to its dense, stable structure. Its unique cubic shape makes it a potential isostere for benzene rings.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 4 snippets · 3.8 s

> **Cubane's bonds are sharper than expected**  
> Cubane's carbon atoms bond at 90° angles, far from the typical 109.45° tetrahedral angle. Before synthesis, Philip Eaton believed cubane was impossible due to this strain. The molecule's existence proves that such sharp angles can be stabilized in a rigid cube structure.

Sources: s3 (Wikipedia, Introduction)  
Fun: _ /5


### dopamine

**Danger level** · passed · 1 snippets · 2.1 s

> **Avoid ingestion and aquatic life**  
> Dopamine is harmful if swallowed and may damage fertility or the unborn child. It is also very toxic to aquatic life, both acutely and with long-term effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 4.3 s

> **First made and found in the brain**  
> In 1910, George Barger and James Ewens first synthesized dopamine at Wellcome Laboratories in London, England. It was identified in the human brain in 1957 by Katharine Montagu. Arvid Carlsson and Nils-Åke Hillarp recognized its function as a neurotransmitter in 1958.

Sources: s1 (Wikipedia, History and development)  
Fun: _ /5

**Where you meet it** · passed · 15 snippets · 7.5 s

> **In your brain and beyond**  
> Dopamine is a key neurotransmitter in the brain, helping regulate movement and motivation. It's found in plants like Magnolia officinalis and produced by Escherichia coli. Outside the brain, it acts as a local messenger in blood vessels, kidneys, and the digestive system, where it helps control blood pressure, urine output, and gut protection.

Sources: s2 (PubChem, Record Description), s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s3 (PubChem, Record Description), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 12 snippets · 6.3 s

> **Dopamine came from bacteria**  
> Dopamine's synthesis machinery in animals likely originated from bacteria via horizontal gene transfer. This means animals didn't evolve dopamine independently but inherited the ability from ancient bacterial relatives, possibly through early symbiotic relationships.

Sources: s11 (Wikipedia, Comparative biology and evolution)  
Fun: _ /5


### ethylene

**Danger level** · passed · 1 snippets · 1.9 s

> **Flammable and eye irritant**  
> Ethylene is an extremely flammable gas and can cause drowsiness or dizziness. It is also classified as an irritant with potential narcotic effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.9 s

> **From ethanol to ethylene gas**  
> Johann Joachim Becher discovered ethylene in 1669 by heating ethanol with sulfuric acid and described it in his Physica Subterranea. Joseph Priestley reported in 1779 that Jan Ingenhousz saw it synthesized by a Mr. Enée in Amsterdam in 1777, and later produced it himself. The name ethylene was first used in this sense as early as 1852.

Sources: s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 14 snippets · 6.7 s

> **Ripening fruit and plastic bags**  
> You meet ethylene in ripening apples, avocados, and flowers — it's a natural plant hormone that speeds up fruit maturation. It's also in polyethylene plastic, used in bags and containers, and in industrial processes like making ethylene oxide and ethanol. Natural emissions from plants and volcanoes add to its presence in the atmosphere.

Sources: s3 (PubChem, Record Description), s4 (Wikipedia, Introduction), s5 (Wikipedia, Introduction), s6 (Wikipedia, Introduction), s12 (PubChem, Uses), s13 (PubChem, Natural Pollution Sources), s14 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 18 snippets · 7.4 s

> **Ancient oracle may have inhaled ethylene**  
> Some scholars believe the Pythia of Delphi entered trance states due to ethylene rising from ground faults. This ancient theory links a mystical ritual to a real chemical gas — a surprisingly modern scientific idea applied to ancient history.

Sources: s1 (Wikipedia, History)  
Fun: _ /5


### geosmin

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 1.5 s

> **Avoid swallowing — may be harmful**  
> Geosmin is harmful if swallowed. It acts as an irritant and poses a risk through oral exposure.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 6 snippets · 4.8 s

> **The smell of rain on soil**  
> Geosmin gives beetroots their earthy taste and causes the petrichor scent after rain. It's found in soil, water, and bacteria like cyanobacteria, and can be detected at just 0.006 to 0.01 micrograms per liter. It's also present in plants such as Basella alba and produced by Saccharomyces cerevisiae.

Sources: s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 9 snippets · 6.2 s

> **Earth's smell comes from bacteria**  
> Geosmin, the earthy smell after rain, is produced by bacteria like cyanobacteria in soil. It's so sensitive that humans can detect it at just 0.006 to 0.01 micrograms per liter in water — a concentration that's incredibly low. This tiny molecule, made by microbes, is the reason we smell the earth after a storm.

Sources: s1 (Wikipedia, Effects), s4 (Wikipedia, Introduction), s5 (Wikipedia, Introduction), s7 (PubChem, Record Description), s8 (PubChem, Record Description)  
Fun: _ /5


### indigo

**Danger level** · passed · 1 snippets · 1.6 s

> **Avoid long-term exposure**  
> Indigo may cause organ damage with prolonged or repeated exposure. The hazard is classified as a health risk through repeated exposure, not acute danger.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 6 snippets · 4.5 s

> **Blue in your jeans and nature**  
> Indigo is found in the leaves of Indigofera tinctoria and is a natural blue pigment. Today, about 80,000 tonnes are produced annually, mostly for denim cloth. A pair of blue jeans uses 3–12 grams of dye, and indigo’s colorfastness allows for repeated dyeing and effects like acid washing.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Introduction), s4 (Wikipedia, Introduction), s5 (Wikipedia, Uses), s6 (Wikipedia, Uses)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 10.3 s

> **Indigo traced to ancient India**  
> The oldest known indigo-dyed fabric dates to 6,000 years ago and was found in Huaca Prieta, Peru. The Indigofera tinctoria species was domesticated in India, which was the earliest major center for its production and processing. India was the primary supplier of indigo to Europe as early as the Greco-Roman era. The Greek word for the dye was indikón, Latinized to indicum, which became indigo in English.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Latinization is not in the cited snippets</summary>

```json
{"title": "Indigo's ancient roots in India", "body": "The oldest known indigo-dyed fabric dates to 6,000 years ago and was found in Huaca Prieta, Peru. The Indigofera tinctoria species was domesticated in India, which became the earliest major center for its production and processing. India's role as a primary supplier to Europe began in the Greco-Roman era, with the Greek term indikón and Roman Latinization to indicum.", "sources": ["s1", "s3"]}
```
</details>

**Weird fact** · passed · 17 snippets · 7.6 s

> **Indigo turns blue when exposed to air**  
> Indigo is insoluble in water and must be reduced to a soluble form called 'white indigo' to dye fabric. When the fabric is removed from the bath, it reverts to blue as the white indigo reacts with oxygen in the air — a process that made early European dyers struggle with the dye.

Sources: s7 (Wikipedia, Dyeing technology)  
Fun: _ /5


### limonene

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 2.3 s

> **Flammable, irritant, harms aquatic life**  
> Limonene is flammable and can irritate the skin. It may cause an allergic skin reaction and is very toxic to aquatic life, both acutely and with long-term exposure.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 8.6 s

> **Scent of citrus and cleaning**  
> You meet limonene in orange and lemon peels, where it’s the main fragrance component. It’s also in hand cleansers, air fresheners, and floor waxes for its citrus scent and oil-dissolving power. Found naturally in citrus, caraway, and dill, it’s used in food flavoring and as a solvent in cleaning products.

Sources: s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Uses), s10 (Wikipedia, Uses), s13 (PubChem, Uses), s15 (PubChem, Natural Pollution Sources), s16 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed on retry · 16 snippets · 17.5 s

> **Limonene turns into a different molecule when heated**  
> Limonene isomerizes to α-terpinene when warmed with mineral acid. This transformation is supported by the formation of Diels–Alder adducts with maleic anhydride. The process occurs at temperatures above 300 °C, where the (R)-enantiomer racemizes.

Sources: s3 (Wikipedia, Chemical reactions)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Scientists is not in the cited snippets</summary>

```json
{"title": "Limonene fights cancer in trials", "body": "Limonene is being studied in a clinical trial (NCT05525260) for chemoprevention of pulmonary nodules. Scientists suggest its metabolites may inhibit tumor growth and induce apoptosis. This is surprising because it's not just a fragrance — it's being tested as a potential cancer-fighting agent.", "sources": ["s13", "s15"]}
```
</details>


### lycopene

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 1.7 s

> **No known hazards identified**  
> Lycopene is not classified as hazardous under GHS criteria. No warning signals or pictograms are assigned. Based on industry assessments, it meets safety standards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 5.3 s

> **Red in your fruit and veggies**  
> Lycopene gives tomatoes, watermelons, pink grapefruit, and red guavas their bright red color. It's also found in baked beans and other red fruits and vegetables. This molecule is naturally present in plants and algae, where it helps protect against oxidative stress.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Occurrence), s8 (Wikipedia, Occurrence)  
Fun: _ /5

**Weird fact** · passed on retry · 19 snippets · 15.8 s

> **Most red pigment has 1056 isomer forms**  
> Lycopene can undergo extensive isomerization, allowing 1056 theoretical cis-trans configurations. The all-trans form is predominant in foods, but in human blood, over 60% of lycopene exists as cis-isomers. This shift from a linear to bent shape in the body is surprising and not fully understood.

Sources: s11 (PubChem, Record Description), s4 (Wikipedia, Structure and physical properties)  
Fun: _ /5

<details><summary>Attempt 1 rejected: title has 10 words, at most 8 allowed</summary>

```json
{"title": "Most of it in your blood is a twisted version", "body": "While tomatoes contain mostly the straight, all-trans form of lycopene, in human blood over 60% is found as twisted cis-isomers. This shift happens due to light or heat, and the body's version is less stable than the original — a surprising twist in how nature and digestion reshape the molecule.", "sources": ["s4", "s11"]}
```
</details>


### menthol

**Danger level** · passed · 1 snippets · 2.1 s

> **Menthol: Mild irritant only**  
> Menthol is classified with a warning signal word and an irritant pictogram. It is not officially classified as hazardous under GHS by one company's assessment. No other hazards are reported.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 1 snippets · 6.0 s

> **First isolated in 1771**  
> In 1771, German chemist Hieronymus David Gaubius first isolated menthol. The molecule was later named by F. L. Alphonse Oppenheim in 1861.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "First isolated in 1771", "body": "In 1771, German chemist Hieronymus David Gaubius first isolated menthol. Early characterizations were later conducted by Oppenheim, Beckett, Moriya, and Atkinson. The name 'menthol' was officially assigned by F. L. Alphonse Oppenheim in 1861.", "sources": ["s1"]}
```
</details>

**Where you meet it** · passed · 18 snippets · 8.6 s

> **The cool in your minty breath**  
> Menthol shows up in peppermint and cornmint oils, giving a cooling sensation when inhaled or applied. Found in toothpastes, cough drops, and hair conditioners, it’s also in cigarettes, perfumes, and nasal inhalers. Naturally present in Mentha × piperita and Mentha arvensis, it’s a key flavor and local anesthetic in many everyday products.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s9 (Wikipedia, Occurrence), s10 (Wikipedia, Occurrence), s12 (Wikipedia, Applications), s13 (PubChem, Uses), s16 (PubChem, Uses), s17 (PubChem, Natural Pollution Sources), s18 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 25 snippets · 8.5 s

> **Menthol tricks your brain into feeling cold**  
> Menthol doesn't cool things down—it activates TRPM8 receptors in your skin, tricking your brain into thinking it's cold. This is similar to how capsaicin makes you feel hot, even though it doesn’t raise temperature.

Sources: s4 (Wikipedia, Biological properties), s14 (Wikipedia, Introduction)  
Fun: _ /5


### morphine

**Danger level** · passed · 1 snippets · 2.6 s

> **Avoid ingestion and skin contact**  
> Morphine is harmful if swallowed and may cause drowsiness or dizziness. It can trigger allergic skin reactions and may affect fertility or the unborn child. Breast-fed children may be harmed by exposure.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.5 s

> **From ancient cures to a sleep-inducing discovery**  
> Possible references to morphine date back to 2100 BC in Sumerian clay tablets listing opium-based cures. In 1804, Friedrich Sertürner discovered morphine as the first active alkaloid from the opium poppy in Paderborn. He named it morphium after the Greek god Morpheus due to its sleep-inducing effect and found it six times stronger than opium.

Sources: s1 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Weird fact** · passed on retry · 13 snippets · 14.6 s

> **Morphine made someone almost die in 1817**  
> Friedrich Sertürner administered morphine to himself, three boys, three dogs, and a mouse; all four nearly died. This shows the drug’s extreme potency, as Sertürner noted it was six times stronger than opium. The fact that he tested it on himself and animals makes it startlingly direct.

Sources: s3 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: title has 12 words, at most 8 allowed</summary>

```json
{"title": "Morphine made him nearly die — and he tried it on himself", "body": "Friedrich Sertürner, who discovered morphine, administered it to himself, three boys, three dogs, and a mouse — all nearly died. This self-experimentation highlights the drug’s extreme potency, which he named morphium after the Greek god of dreams.", "sources": ["s3"]}
```
</details>

**Where you meet it** · passed · 18 snippets · 9.8 s

> **In opium poppies and pain meds**  
> Morphine is found naturally in the opium poppy, making up 8–14% of dry opium weight. It's used in hospitals for severe pain, including post-surgery or labor pains, and comes in forms like hydrochloride or acetate. You may also encounter it in medicines, though its use is tightly regulated due to high addiction risk.

Sources: s1 (PubChem, Record Description), s3 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (Wikipedia, Medical uses), s10 (Wikipedia, Medical uses), s11 (Wikipedia, Natural occurrence), s17 (PubChem, Natural Pollution Sources), s18 (PubChem, Natural Pollution Sources), s15 (PubChem, Uses)  
Fun: _ /5


### nicotine

**Danger level** · passed · 1 snippets · 2.0 s

> **Fatal if swallowed, inhaled, or on skin**  
> Nicotine is fatal if swallowed, in contact with skin, or inhaled. It is also toxic to aquatic life with long-lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.3 s

> **From tobacco to a chemical discovery**  
> In 1828, Wilhelm Heinrich Posselt and Karl Ludwig Reimann isolated nicotine from the tobacco plant in Germany, believing it was a poison. Its structure was discovered in 1893 by Adolf Pinner and Richard Wolffenstein. It was first synthesized in 1904 by Amé Pictet and A. Rotschy.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 17 snippets · 8.9 s

> **In tobacco, plants, and quitting tools**  
> Nicotine is found in tobacco leaves, making up 0.6–3% of dry weight, and in trace amounts in tomatoes, potatoes, and eggplants. It appears in cigarettes, where an average yields about 2 mg, and is used in nicotine replacement therapy (NRT) products like patches and gum to help quit smoking. It’s also found in pesticides such as Black Leaf 40, used in ornamental plants like poinsettias.

Sources: s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Uses), s10 (Wikipedia, Uses), s11 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Natural Pollution Sources), s16 (PubChem, Natural Pollution Sources), s17 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 15 snippets · 7.6 s

> **Nicotine blocks pain in two receptors**  
> Nicotine acts as a receptor antagonist at nAChRα9 and nAChRα10 subunits, causing mild analgesia — a surprising twist since it's mostly known as a stimulant and poison. This antagonism results in pain relief, which contradicts its typical role in the nervous system.

Sources: s15 (Wikipedia, Pharmacology)  
Fun: _ /5


### nitroglycerin

**Danger level** · passed · 1 snippets · 2.8 s

> **Explosive and fatally toxic**  
> Nitroglycerin is classified as explosive and poses a mass explosion hazard. It is fatal if swallowed, in contact with skin, or inhaled. Prolonged exposure may damage organs, and it is toxic to aquatic life with long-term effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.7 s

> **From lab to explosion**  
> In 1846, Italian chemist Ascanio Sobrero synthesized nitroglycerin at the University of Turin, working under Théophile-Jules Pelouze. He named it 'pyroglycerin' and warned against its explosive use. Later, Alfred Nobel adopted it as an explosive after a deadly explosion in 1864 killed his brother Emil Oskar and workers in Heleneborg, Sweden.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 8.5 s

> **In dynamite and heart pills**  
> You meet nitroglycerin in dynamite sticks used for tunneling and mining, and in heart pills for angina pectoris. It's also found in rocket fuel and pharmaceuticals, where it acts as a nitrate converted to nitric oxide in the body.

Sources: s1 (PubChem, Record Description), s10 (Wikipedia, Use as an explosive and a propellant), s11 (Wikipedia, Medical use), s12 (Wikipedia, Medical use), s13 (PubChem, Uses), s14 (PubChem, Uses), s16 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 14 snippets · 7.4 s

> **Frozen nitroglycerin can be less stable than liquid**  
> Frozen nitroglycerin is less energetic and more unpredictable when shocked—especially when broken or crushed. This makes it surprisingly dangerous to handle, as thawing it can suddenly re-activate its sensitivity. The instability is tied to its purity and temperature, making safe handling a constant challenge.

Sources: s6 (Wikipedia, Instability and desensitization)  
Fun: _ /5


### quinine

**Danger level** · passed · 1 snippets · 2.3 s

> **Avoid skin, eyes, and inhalation**  
> Quinine is harmful if swallowed and can cause skin irritation. It may lead to allergic skin reactions or respiratory issues if inhaled. It can also cause serious eye irritation.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.0 s

> **From Quechua bark to Jesuit testing**  
> The Quechua Indians of Peru, Bolivia, and Ecuador used ground cinchona bark to treat shivering, mixing it with sweetened water to mask its bitterness. Spanish Jesuit missionaries, including Agostino Salumbrino, observed this practice and brought cinchona bark to Europe. In 1564–1642, Salumbrino sent a sample to Rome for testing, where it was used as a malaria treatment. This led to the widespread use of cinchona bark, known as Jesuit's bark or Peruvian bark.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 7.7 s

> **In tonic water and malaria treatment**  
> You'll find quinine in tonic water and bitter lemon as a bitter flavoring. It's also used medically to treat malaria and babesiosis, especially when artemisinins aren't available. Historically, it was extracted from the bark of the cinchona tree, native to South America.

Sources: s6 (Wikipedia, Introduction), s11 (PubChem, Uses), s14 (PubChem, Uses), s16 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 12 snippets · 6.5 s

> **Quinine can cause blindness from a sip**  
> Even minor levels in common beverages can trigger severe adverse effects, including blindness and hearing loss. This is especially concerning because quinine is used in tonic water, which many people consume daily.

Sources: s7 (Wikipedia, Contraindications), s9 (Wikipedia, Adverse effects), s10 (Wikipedia, Adverse effects)  
Fun: _ /5


### serotonin

**Danger level** · passed · 1 snippets · 1.8 s

> **Toxic if swallowed, may harm fertility**  
> Serotonin is toxic if swallowed and can damage fertility or the unborn child. These are official GHS hazard statements based on scientific evaluation.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 4.3 s

> **From enteramine to serotonin**  
> In 1935, Vittorio Erspamer in Pavia discovered a substance from enterochromaffin cells that caused intestinal contraction, naming it enteramine. In 1948, Maurice M. Rapport, Arda Green, and Irvine Page identified a vasoconstrictor in blood serum and named it serotonin. In 1952, enteramine was shown to be the same as serotonin.

Sources: s1 (Wikipedia, History & etymology), s2 (Wikipedia, History & etymology)  
Fun: _ /5

**Where you meet it** · passed · 8 snippets · 5.7 s

> **Where serotonin works in your body**  
> You meet serotonin in your gut, where about 90% of it is made to regulate digestion. It also shows up in the brain, blood platelets, and skin, helping with mood, sleep, and blood vessel control. Found in small amounts in taste cells and the brainstem, it acts as a messenger in both mind and body.

Sources: s1 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed on retry · 14 snippets · 13.2 s

> **Algae poisoned by serotonin**  
> Serotonin is toxic to algae. The gastrointestinal parasite Entamoeba histolytica secretes serotonin, making it more virulent, and patients infected with it have highly elevated serum serotonin levels.

Sources: s9 (Wikipedia, Comparative biology & evolution)  
Fun: _ /5

<details><summary>Attempt 1 rejected: title has 9 words, at most 8 allowed</summary>

```json
{"title": "A parasite makes you feel sick by secreting serotonin", "body": "The gut parasite Entamoeba histolytica secretes serotonin, causing sustained diarrhea. This leads to elevated serum serotonin levels in infected patients, and the parasite becomes more virulent in response to serotonin's presence.", "sources": ["s9"]}
```
</details>


### strychnine

**Danger level** · passed · 1 snippets · 1.9 s

> **Fatal if swallowed or touched**  
> Strychnine is fatal if swallowed or in contact with skin. It is also very toxic to aquatic life, both acutely and with long-term effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.0 s

> **First discovered in a bean**  
> In 1818, French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier first discovered strychnine in Saint-Ignatius' bean. They isolated the compound from the plant, marking the first identification of an alkaloid in the genus Strychnos. Historic records show its use as a poison in Europe as early as 1640.

Sources: s3 (Wikipedia, History), s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 6.4 s

> **In poison baits and seeds**  
> Strychnine is found in the seeds of the Strychnos nux-vomica tree and Strychnos ignatii beans. It's used in poison baits for rodents, rabbits, and squirrels, often at 0.5–1% sulfate concentration. These baits are designed to kill small vertebrates, though ingestion causes severe convulsions and death.

Sources: s1 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s7 (PubChem, Uses), s10 (PubChem, Uses), s11 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed on retry · 10 snippets · 13.5 s

> **Strychnine kills by stopping the brakes**  
> Strychnine acts as an antagonist of glycine receptors in the spinal cord, which normally act as brakes on muscle movement. Without this inhibition, muscles go into violent spasms, leading to death. This makes it a neurotoxin that disables the body’s natural calming system.

Sources: s5 (Wikipedia, Mechanism of action)  
Fun: _ /5

<details><summary>Attempt 1 rejected: title has 9 words, at most 8 allowed</summary>

```json
{"title": "Poison used to kill a murderer and a bat", "body": "Strychnine was allegedly used by murderer William Palmer to kill his victim, and fruit bats have evolved resistance to it because they eat toxic Strychnos fruit. This shows a bizarre mix of human crime and natural evolution.", "sources": ["s3", "s7"]}
```
</details>


### sucralose

**Danger level** · passed · 1 snippets · 1.9 s

> **No known hazards per GHS**  
> Sucralose is not classified as hazardous under GHS criteria. No official hazard statements are reported, as 309 companies confirm it meets the criteria for non-classification.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.1 s

> **A chlorinated sugar test**  
> In 1975, scientists from Tate & Lyle, working with Leslie Hough and Shashikant Phadnis at Queen Elizabeth College, mixed sulfuryl chloride with sugar. Phadnis was told to 'test' a chlorinated sugar compound and, believing it was to taste it, did so and found it exceptionally sweet. The compound was sucralose, patented by Tate & Lyle in 1976.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 6.4 s

> **Sweet spots in your drink**  
> Sucralose, about 600 times sweeter than sucrose, is found in soft drinks, candy, breakfast bars, and canned fruits where it replaces sugar. It’s also in coffee pods and table-top sweeteners like Splenda, which contains sucralose at 1.1% with bulking agents such as dextrose and maltodextrin. In personal care products, it appears in 39 consumer items listed by PubChem.

Sources: s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (Wikipedia, Introduction), s6 (Wikipedia, Uses), s7 (Wikipedia, Uses), s9 (PubChem, Uses), s11 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 19 snippets · 7.7 s

> **It was 'tasted' by mistake**  
> Scientists thought they were asked to taste a chlorinated sugar compound, but Phadnis actually tasted it and found it exceptionally sweet. The anecdote suggests the entire discovery was based on a miscommunication — a sweetener born from a simple, accidental taste test.

Sources: s1 (Wikipedia, History)  
Fun: _ /5


### tetrodotoxin

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 2.1 s

> **Fatal if swallowed, touched, or inhaled**  
> Tetrodotoxin is officially rated as fatal if swallowed, in contact with skin, or inhaled. The hazard statements confirm acute toxicity through all exposure routes.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Weird fact** · passed · 16 snippets · 7.9 s

> **Bacteria make the deadliest toxin in fish**  
> Tetrodotoxin is not made by pufferfish, but by bacteria like Vibrio alginolyticus and Pseudoalteromonas tetraodonis. These bacteria produce the toxin, which then accumulates in animals such as pufferfish, blue-ringed octopuses, and newts.

Sources: s10 (Wikipedia, Introduction), s16 (PubChem, Record Description)  
Fun: _ /5

**Where you meet it** · passed · 15 snippets · 9.2 s

> **In pufferfish, newts, and octopuses**  
> You might find tetrodotoxin in pufferfish, especially in their liver and ovaries, where it's most concentrated. It also appears in blue-ringed octopuses, rough-skinned newts, and some shellfish. The toxin blocks nerve signals, causing paralysis — a danger when eating improperly prepared fish.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Natural Pollution Sources), s14 (PubChem, Natural Pollution Sources), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### thalidomide

**Danger level** · passed on retry · 1 snippets · 3.3 s

> **Harmful if swallowed, may harm fertility**  
> Thalidomide is harmful if swallowed. It may damage fertility or the unborn child.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Harmful if swallowed, may harm fertility", "body": "Thalidomide is harmful if swallowed and may damage fertility or the unborn child.", "sources": ["s1"]}
```
</details>

**Origin story** · passed · 4 snippets · 5.0 s

> **From discarded by-product to a 'wonder drug'**  
> In 1952, Chemical Industry Basel synthesized thalidomide but discarded it after finding it had no effect on animals. In 1957, Chemie Grünenthal acquired it and Heinrich Mückter led the discovery program. While preparing reagents, Mückter's assistant Wilhelm Kunz isolated a by-product recognized by pharmacologist Herbert Keller as an analog of glutethimide.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 12 snippets · 6.5 s

> **In cancer and leprosy treatments**  
> You might find thalidomide in medicines for multiple myeloma, leprosy skin lesions, and graft-versus-host disease. It’s also used for HIV-related conditions and severe brain inflammation linked to tuberculosis. Once marketed for sleep and morning sickness, it’s now strictly controlled due to severe birth defects.

Sources: s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (Wikipedia, Medical uses), s10 (Wikipedia, Medical uses), s11 (PubChem, Uses), s12 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · rejected · 14 snippets · 16.4 s

<details><summary>Attempt 1 rejected: title has 9 words, at most 8 allowed</summary>

```json
{"title": "It turns into its mirror image in your body", "body": "Thalidomide is a chiral molecule that can racemize in the body. The S-enantiomer is the active form, but it can convert to its mirror-image version due to an acidic hydrogen at the chiral center. This chiral inversion happens in vivo, which may explain why the drug has both therapeutic and teratogenic effects.", "sources": ["s12"]}
```
</details>

<details><summary>Attempt 2 rejected: number 21st is not in the cited snippets</summary>

```json
{"title": "It turns itself into the harmful form", "body": "Thalidomide is a chiral molecule that racemizes in the body, meaning it can convert from the safe form to the harmful one. This chiral inversion occurs in vivo, even after synthesis, making the drug unpredictable and dangerous during pregnancy. The process is called chiral inversion and was only fully understood in the 21st century.", "sources": ["s12", "s13", "s14"]}
```
</details>


### theobromine

**Danger level** · passed · 1 snippets · 1.9 s

> **Avoid swallowing and eye contact**  
> Theobromine is harmful if swallowed and can cause serious eye irritation. It is also suspected of causing cancer, though this is not confirmed.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 3 snippets · 2.8 s

> **Found in cacao beans in 1841**  
> In 1841, chemist A. Woskresensky discovered theobromine in cacao beans. Later, in 1882, Hermann Emil Fischer first reported synthesizing it from xanthine.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 6.8 s

> **In chocolate, tea, and hollies**  
> You'll find theobromine in chocolate from cacao beans, tea from Camellia sinensis, and American hollies like yaupon and guayusa. It's also present in kola nuts and is a natural component of the cacao plant. The molecule is used in cosmetics and was once used in diuretics and heart treatments.

Sources: s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s11 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 20 snippets · 8.6 s

> **Its name has no bromine at all**  
> Theobromine contains no bromine, despite the name. The suffix '-ine' was used for alkaloids, and 'bromine' in the name comes from the Greek 'bromos' meaning 'stench', not because of the element bromine.

Sources: s3 (Wikipedia, Etymology), s4 (Wikipedia, Etymology)  
Fun: _ /5


### vanillin

**Danger level** · passed · 1 snippets · 1.8 s

> **Avoid ingestion and aquatic life**  
> Vanillin is harmful if swallowed and can cause acute toxicity through oral exposure. It is also toxic to aquatic life, with long-lasting environmental effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 10.3 s

> **Vanillin first isolated in 1858**  
> In 1858, Théodore Nicolas Gobley isolated vanillin as a pure substance by evaporating vanilla extract and recrystallizing the solids from hot water. He achieved this while working with vanilla extract, a process that marked the first purification of vanillin. The chemical structure was deduced in 1874 by Ferdinand Tiemann and Wilhelm Haarmann, who also developed a synthesis from coniferin in pine bark.

Sources: s3 (Wikipedia, Synthetic history)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Mesoamerica is not in the cited snippets</summary>

```json
{"title": "From ancient beans to lab synthesis", "body": "Vanilla beans were first used by the Totonacs in Mesoamerica, notably in Veracruz, Mexico, as a flavoring for drinks. In 1858, Théodore Nicolas Gobley isolated vanillin by evaporating vanilla extract and recrystallizing it. In 1874, Ferdinand Tiemann and Wilhelm Haarmann deduced its structure and developed a synthesis from coniferin, starting industrial production in Holzminden, Germany.", "sources": ["s2", "s3"]}
```
</details>

**Where you meet it** · passed · 19 snippets · 8.0 s

> **Scent of chocolate and ice cream**  
> Vanillin is the main flavor in vanilla beans, making up about 2% by dry weight. You’ll find it in ice cream, chocolate, and confections—where it accounts for 75% of the flavoring market. It’s also in perfumes, medicines, and cleaning products to mask odors or add a creamy note.

Sources: s6 (Wikipedia, Introduction), s13 (Wikipedia, Uses), s14 (Wikipedia, Uses), s19 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed on retry · 21 snippets · 17.2 s

> **Vanillin in a 2nd millennium BCE tomb**  
> Traces of vanillin were found in jars from a tomb in Canaan dating to the 2nd millennium BCE, suggesting vanilla cultivation existed in the Old World long before its domestication in Mesoamerica. This predates known vanilla use by thousands of years and implies an unidentified Vanilla species was grown in ancient times.

Sources: s1 (Wikipedia, Natural history)  
Fun: _ /5

<details><summary>Attempt 1 rejected: number 3000 is not in the cited snippets</summary>

```json
{"title": "Vanillin in ancient Canaanite wine jars", "body": "Traces of vanillin were found in wine jars in Jerusalem from before 586 BCE, suggesting vanilla-like plants were cultivated in Canaan as early as the Middle Bronze Age — over 3,000 years ago. This predates the known domestication of vanilla in Mesoamerica by centuries.", "sources": ["s1"]}
```
</details>

