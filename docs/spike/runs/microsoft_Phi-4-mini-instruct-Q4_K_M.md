# Spike run: microsoft_Phi-4-mini-instruct-Q4_K_M

30 molecules, 111 cards with input. Temperature 0.6, JSON schema constraint, one stricter retry, shortest card first, KV cache: paged, fixed 8K tokens.

## Summary

| Measure | Value |
| --- | --- |
| Passed first try | 91 (82%) |
| Passed on retry | 17 (15%) |
| Skipped by model | 0 (0%) |
| Rejected twice (dropped) | 3 (3%) |
| No input (no card) | 9 |
| Molecules with 3+ cards | 29 of 30 |
| First token | 2.2 s median, 4.9 s p90, 5.8 s max |
| Card, including retry | 5.0 s median, 11.2 s p90, 16.3 s max |
| First card per molecule | 1.7 s median, 2.7 s p90, 3.5 s max |
| Model load | 6.9 s |
| Peak memory | 4.47 GB |

### Problems found (all attempts)

- code not in sources: 4
- name not in sources: 9
- number not in sources: 1
- sentence count: 12
- title: 1

## Cards

Fun score is for hand review in M0.6.

### allicin

**Danger level** · passed · 1 snippets · 2.7 s

> **Watch out for skin and eyes**  
> Allicin can cause skin irritation and serious eye irritation. Breathing it in might irritate your respiratory tract.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 2 snippets · 5.9 s

> **Discovery in Japan**  
> Allicin was discovered as part of efforts to create thiamine derivatives in the 1940s, mainly in Japan. Chester J. Cavallito and John Hays Bailey isolated and studied it in 1944.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: 1940s is not in the cited snippets; name Japan is not in the cited snippets</summary>

```json
{"title": "Discovery of Allicin", "body": "Allicin was first isolated and studied in 1944 by Chester J. Cavallito and John Hays Bailey. It was discovered as part of efforts to create thiamine derivatives in the 1940s, mainly in Japan.", "sources": ["s1"]}
```
</details>

**Where you meet it** · passed · 7 snippets · 4.7 s

> **Garlic's secret weapon**  
> Allicin, the compound that gives garlic its distinctive smell, is an antifeedant that protects garlic plants from pests. It's formed when garlic is chopped or crushed, thanks to the enzyme alliinase.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (Wikipedia, Introduction), s5 (Wikipedia, Introduction), s6 (Wikipedia, Structure and occurrence), s7 (Wikipedia, Structure and occurrence)  
Fun: _ /5

**Weird fact** · passed · 11 snippets · 4.6 s

> **Garlic's secret weapon**  
> Garlic's secret weapon, allicin, is formed when garlic is damaged, not just when it's eaten. It's unstable and quickly turns into other compounds, making it a fleeting defender against pests.

Sources: s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction)  
Fun: _ /5


### artemisinin

**Danger level** · passed · 1 snippets · 1.9 s

> **Beware of fire and water harm**  
> Artemisinin is officially rated highly flammable and can cause a fire when heated. It's very toxic to aquatic life, posing a long-term hazard to the environment.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.2 s

> **Sweet wormwood's secret to malaria**  
> Artemisinin, an antimalarial lactone derived from sweet wormwood (Artemisia annua), was discovered by Tu Youyou in 1972 during a secret military program to treat malaria. Influenced by ancient Chinese medicine, she found an effective antimalarial substance using a low-temperature extraction process.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 10 snippets · 4.9 s

> **Sweet wormwood's secret weapon**  
> Artemisinin, a sesquiterpene lactone from sweet wormwood (Artemisia annua), fights malaria. It's a Nobel Prize-winning discovery and a key player in ACTs.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (Wikipedia, Medical use), s10 (Wikipedia, Medical use)  
Fun: _ /5

**Weird fact** · passed · 10 snippets · 6.1 s

> **Artemisinin's mysterious ring**  
> Artemisinin's unique endoperoxide 1,2,4-trioxane ring is its main antimalarial center. Modifications at carbon 10 create more potent derivatives.

Sources: s8 (Wikipedia, Chemistry), s9 (Wikipedia, Chemistry)  
Fun: _ /5


### ascorbic acid

**Danger level** · passed · 1 snippets · 1.7 s

> **Ascorbic Acid: Safe but not classified**  
> Ascorbic acid is not classified as hazardous by GHS standards, according to one company. It's generally considered safe, but always check the latest guidelines.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.0 s

> **Citrus cure for scurvy**  
> Scurvy, known since ancient times, was scientifically linked to citrus fruits by James Lind in 1753. He conducted controlled experiments at sea, showing that oranges and lemons prevented the disease. By 1796, the British navy adopted lemon juice as standard issue, saving countless lives.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 17 snippets · 8.1 s

> **Vitamin C: Nature's Antioxidant**  
> Vitamin C, or ascorbic acid, is a vital nutrient found in citrus fruits, vegetables, and some meats. It's essential for collagen production, immune function, and acts as an antioxidant. Beyond nutrition, it's used in food preservation, cosmetics, and even as a therapeutic agent for scurvy.

Sources: s3 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s10 (Wikipedia, Food preservation additive), s11 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Natural Pollution Sources), s16 (PubChem, Natural Pollution Sources), s17 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed on retry · 15 snippets · 14.3 s

> **Vitamin C: More than just an antioxidant**  
> Vitamin C, or ascorbic acid, isn't just an antioxidant; it's a cofactor for enzymes involved in collagen, carnitine, and neurotransmitter synthesis. Despite its essential role in preventing scurvy, its effectiveness in treating various diseases remains disputed.

Sources: s5 (Wikipedia, Chemistry), s13 (Wikipedia, Pharmacology), s15 (Wikipedia, Health effects)  
Fun: _ /5

<details><summary>Attempt 1 rejected: 1500s is not in the cited snippets</summary>

```json
{"title": "Vitamin C: More than just an antioxidant", "body": "Did you know that vitamin C, also known as ascorbic acid, isn't just an antioxidant? It's a cofactor for enzymes involved in synthesizing collagen, carnitine, and neurotransmitters. Plus, it's been used since the 1500s to prevent scurvy on long sea voyages!", "sources": ["s3", "s5", "s14", "s15"]}
```
</details>


### aspirin

**Danger level** · passed · 1 snippets · 1.7 s

> **Watch out for swallowing and skin contact**  
> Aspirin can harm you if swallowed and may irritate your skin. It can also cause serious eye irritation and respiratory irritation.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.2 s

> **From ancient remedies to modern medicine**  
> Aspirin's journey began with salicylic acid extracted from meadowsweet in 1839. Charles Frédéric Gerhardt synthesized acetylsalicylic acid in 1853. Bayer's scientists, including Felix Hoffmann, produced pure aspirin in 1897. By 1899, Bayer was selling it globally under the brand name Aspirin.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 13 snippets · 5.8 s

> **Aspirin: Pain relief and heart protector**  
> Aspirin, known as acetylsalicylic acid, is a common pain reliever and fever reducer. It's also used to prevent heart attacks and strokes in high-risk individuals. Plus, it can help with inflammation and even reduce the risk of certain cancers.

Sources: s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (Wikipedia, Medical use), s11 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 14 snippets · 13.0 s

> **Aspirin's explosive secret**  
> Aspirin's powder can explode in factories, so NIOSH and OSHA set strict limits on airborne powder to keep workers safe. This shows aspirin's volatile nature beyond its medicinal uses.

Sources: s8 (Wikipedia, Chemical properties), s9 (Wikipedia, Physical properties), s10 (Wikipedia, Physical properties), s11 (Wikipedia, Mechanism of action), s12 (Wikipedia, Mechanism of action), s13 (Wikipedia, Formulations)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Aspirin's explosive secret", "body": "Aspirin's powder can be explosive in factories, requiring strict air control to prevent dangerous dust mixtures.", "sources": ["s8", "s9", "s10"]}
```
</details>


### batrachotoxin

**Danger level** · no input · 0 snippets · -0.0 s

**Origin story** · passed · 1 snippets · 2.4 s

> **Discovery of a Potent Toxin**  
> In 1963, Fritz Märki and Bernhard Witkop discovered batrachotoxin at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. They isolated this toxin from the black-legged poison dart frog's toxic alkaloids fraction.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 12 snippets · 5.8 s

> **Poison darts and frogs**  
> Poison dart frogs and certain birds use batrachotoxin for hunting. This potent toxin comes from frogs, beetles, and birds, and it's used to make darts that can paralyze prey instantly.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s6 (Wikipedia, Use), s7 (Wikipedia, Use), s11 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 16 snippets · 7.4 s

> **Frog's lethal secret**  
> Batrachotoxin, found in frogs like Phyllobates aurotaenia, is so potent it can paralyze and kill with just a tiny amount. It's used in hunting darts by indigenous people in Colombia.

Sources: s1 (Wikipedia, History), s8 (Wikipedia, Introduction), s10 (PubChem, Record Description), s11 (PubChem, Record Description), s13 (PubChem, Natural Pollution Sources), s15 (PubChem, Toxicity Summary)  
Fun: _ /5


### benzylpenicillin

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 2.7 s

> **Beware of skin reactions and aquatic life harm**  
> Benzylpenicillin is rated as dangerous due to its potential to cause allergic skin reactions and harm aquatic life. It may also cause breathing difficulties if inhaled.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 12 snippets · 5.7 s

> **Antibiotic's Adventures**  
> Benzylpenicillin, also known as PenG, fights off nasty bacteria causing pneumonia, strep throat, and even Lyme disease. It's injected because it doesn't like to be swallowed. Safe for pregnant women, though always check allergies first!

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s9 (Wikipedia, Medical uses), s11 (PubChem, Uses), s12 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 15 snippets · 11.6 s

> **Antibiotic's unexpected allergy risk**  
> Benzylpenicillin, an antibiotic for serious infections, can cause allergic reactions like anaphylaxis. Surprisingly, it's also a drug allergen, posing risks even for those without a history of penicillin allergy.

Sources: s2 (Wikipedia, Adverse effects), s6 (Wikipedia, Introduction), s8 (PubChem, Record Description), s9 (PubChem, Record Description), s11 (PubChem, Record Description), s12 (PubChem, Record Description), s13 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Uses)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Antibiotic with a twist of allergy", "body": "Benzylpenicillin, an antibiotic, can cause allergic reactions like anaphylaxis, even though it's generally safe during pregnancy.", "sources": ["s2", "s6"]}
```
</details>


### buckminsterfullerene

**Danger level** · passed · 1 snippets · 1.5 s

> **Watch out for skin and eyes**  
> Buckminsterfullerene can cause skin irritation and serious eye irritation. It may also irritate your respiratory tract.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 3.9 s

> **Laser-made carbon cage**  
> In 1984, Eric Rohlfing, Donald Cox, and Andrew Kaldor first generated buckminsterfullerene using a laser to vaporize carbon in a supersonic helium beam. Later, in 1985, Harold Kroto and his team at Rice University recognized the structure of C60 as buckminsterfullerene.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 5.6 s

> **Buckyballs in the cosmos and on Earth**  
> Buckyballs, or C60 fullerenes, are found in soot, space, and even in high-performance polymer films. They're like tiny soccer balls made of carbon atoms.

Sources: s2 (PubChem, Record Description), s8 (Wikipedia, Occurrence), s10 (Wikipedia, Potential applications), s12 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Uses), s16 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed on retry · 12 snippets · 16.3 s

> **Buckyballs and Boron Balls: A Fullerene Family Reunion**  
> Did you know that buckminsterfullerene (C60) and boron fullerenes share a similar structure? While C60 is a stable icosahedron with 60 carbon atoms, boron fullerenes like B80 were predicted to have an icosahedral shape too, but with an extra boron atom at the center of each hexagon. However, B80 turned out to be less stable than other boron configurations, possibly due to calculation errors in bond lengths.

Sources: s3 (Wikipedia, History), s7 (Wikipedia, Structure), s12 (Wikipedia, Boron fullerenes)  
Fun: _ /5

<details><summary>Attempt 1 rejected: number 60 is not in the cited snippets</summary>

```json
{"title": "Buckyballs and Boron Rings: A Fullerene Family Reunion", "body": "Did you know that buckminsterfullerene (C60) and its boron cousin (B80) share the same icosahedral structure? While C60 is a carbon marvel with 60 atoms, B80 was predicted to have 80 boron atoms, but it's less stable and not as commonly found.", "sources": ["s3", "s12"]}
```
</details>


### caffeine

**Danger level** · passed · 1 snippets · 1.3 s

> **Caffeine: Swallowing caution**  
> Caffeine is rated as harmful if swallowed, indicating acute toxicity. It's important to handle with care.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 3.3 s

> **Ancient Caffeine Origins**  
> Caffeine's journey began with ancient West African cultures chewing kola nuts for vitality. The Mayans consumed bitter chocolate with caffeine around 600 BCE. Xocolatl, a caffeinated drink, was believed to fight fatigue and became a luxury in Mesoamerica. The Spaniards introduced cacao to Europe by 1700, spreading caffeine's fame.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Weird fact** · passed · 15 snippets · 6.3 s

> **Caffeine's ancient roots**  
> Caffeine's history stretches back to ancient civilizations, with evidence of its use in Mayan xocolatl dating to 600 BCE. This ancient stimulant was even used as currency in pre-Columbian Mesoamerica!

Sources: s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 7.4 s

> **Caffeine: Nature's Buzz**  
> From your morning coffee to your afternoon tea, caffeine is a familiar friend. It's the secret behind the buzz in chocolate, the kick in your soda, and even the energy in your energy drink.

Sources: s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Uses), s10 (Wikipedia, Uses), s11 (Wikipedia, Natural occurrence), s12 (PubChem, Uses), s14 (PubChem, Uses), s16 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### capsaicin

**Danger level** · passed · 1 snippets · 1.6 s

> **Watch out for skin and eyes**  
> Capsaicin is rated as toxic if swallowed and can cause serious eye damage. It may also irritate your skin.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 2.5 s

> **Burning discovery**  
> In 1816, Christian Friedrich Bucholz first extracted capsaicin in impure form. Later, in 1873 and 1878, Rudolf Buchheim and Endre Hőgyes discovered its burning sensation on mucous membranes and its effect on gastric acid secretion.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 15 snippets · 7.2 s

> **Spice up your life with capsaicin!**  
> Capsaicin, the fiery star of chili peppers, not only adds heat to your meals but also serves as a topical analgesic and a neurobiological research tool. It's even used as a repellent for various pests!

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s5 (PubChem, Record Description), s9 (Wikipedia, Uses), s10 (Wikipedia, Uses), s11 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 14 snippets · 6.1 s

> **Capsaicin's spicy secret**  
> Did you know capsaicin, the spicy compound in chili peppers, evolved as an antifungal agent? It's like nature's own pest control!

Sources: s4 (Wikipedia, Natural function), s13 (Wikipedia, Evolution)  
Fun: _ /5


### carvone

**Danger level** · passed · 1 snippets · 1.5 s

> **Watch out for skin irritation**  
> Carvone is officially rated as an irritant that may cause an allergic skin reaction. It's important to handle it carefully to avoid skin sensitization.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 1 snippets · 2.6 s

> **Carving out carvone's history**  
> Carvone was first isolated as a pure compound in 1849 by Franz Varrentrapp. Originally called carvol by Schweizer, it was later identified as a ketone related to limonene by Goldschmidt and Zürrer. Georg Wagner finally elucidated its structure in 1894.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 14 snippets · 6.9 s

> **Nature's Flavorful Molecule**  
> Carvone, the star of spearmint and caraway, pops up in foods, medicines, and even air fresheners. It's a terpenoid that's been used for flavoring and in alternative medicine for ages.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s5 (Wikipedia, Introduction), s6 (Wikipedia, Uses), s8 (Wikipedia, Occurrence), s9 (PubChem, Uses), s11 (PubChem, Uses), s13 (PubChem, Natural Pollution Sources), s14 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 16 snippets · 8.5 s

> **Carvone's mysterious origins**  
> Carvone was first isolated in 1849 by Franz Varrentrapp, but its structure wasn't fully understood until 1894 by Georg Wagner. It's a terpenoid found in spearmint, caraway, and dill, with a complex history of discovery and chemical transformations.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, Uses), s9 (Wikipedia, Metabolism), s10 (Wikipedia, Introduction), s11 (PubChem, Record Description), s12 (PubChem, Record Description), s13 (PubChem, Record Description), s14 (PubChem, Record Description), s15 (PubChem, Natural Pollution Sources), s16 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### cinnamaldehyde

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 1.6 s

> **Watch out for skin irritation**  
> Cinnamaldehyde is officially rated as an irritant, which means it might cause an allergic skin reaction. It's important to handle it carefully to avoid skin contact.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 15 snippets · 7.0 s

> **Cinnamon's Secret Ingredient**  
> Cinnamaldehyde, the star of cinnamon's flavor and scent, pops up in everything from your morning coffee to chewing gum. It's even used as a natural insect repellent and in perfumes!

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s5 (PubChem, Record Description), s7 (Wikipedia, Applications), s8 (Wikipedia, Applications), s9 (PubChem, Uses), s10 (PubChem, Uses), s11 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Natural Pollution Sources), s14 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 19 snippets · 7.6 s

> **Cinnamon's double life**  
> Cinnamaldehyde, the essence of cinnamon's flavor, doubles as a natural insecticide and a potential allergen. It's both a culinary delight and a guardian of crops!

Sources: s1 (Wikipedia, Applications), s2 (Wikipedia, Applications), s4 (Wikipedia, Toxicology), s12 (PubChem, Record Description), s13 (PubChem, Record Description), s14 (PubChem, Record Description), s18 (PubChem, Toxicity Summary)  
Fun: _ /5


### cubane

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · no input · 0 snippets · -0.0 s

**Where you meet it** · passed · 2 snippets · 2.5 s

> **The cube-shaped energy storage**  
> Cubane, a synthetic hydrocarbon with a cubic structure, was first synthesized in 1964. Its unique shape and high potential energy make it useful for controlled energy storage and as a high-performance explosive.

Sources: s1 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 4 snippets · 3.0 s

> **Cubane's Cube-Sized Strain**  
> Cubane's carbon atoms are squeezed into a cube, with each corner forming a 90-degree bond angle, unlike the tetrahedral angle of 109.45 degrees. This makes cubane highly strained and energetically unfavorable.

Sources: s3 (Wikipedia, Introduction)  
Fun: _ /5


### dopamine

**Danger level** · passed · 1 snippets · 1.8 s

> **Watch out for water and babies**  
> Dopamine is officially rated harmful if swallowed and suspected of damaging fertility or the unborn child. It is very toxic to aquatic life, with long-lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 4.3 s

> **Dopamine's Discovery and Dopamine-Derived Polymers**  
> Dopamine was first synthesized in 1910 by George Barger and James Ewens at Wellcome Laboratories in London. Its role as a neurotransmitter was identified in 1958 by Carlsson and Hillarp. In 2007, dopamine's ability to coat materials led to the discovery of polydopamine, a polymer formed through spontaneous oxidation.

Sources: s1 (Wikipedia, History and development), s2 (Wikipedia, History and development)  
Fun: _ /5

**Where you meet it** · passed · 15 snippets · 6.9 s

> **Dopamine: Brain's Motivator and Heart's Helper**  
> Dopamine, a brain neurotransmitter, signals what's important, guiding our actions. Outside the brain, it helps regulate blood vessels, kidneys, and even heartbeats. It's a key player in treating low blood pressure and heart issues in newborns.

Sources: s2 (PubChem, Record Description), s6 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Medical uses), s11 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Uses), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed on retry · 12 snippets · 12.3 s

> **Dopamine's ancient bacterial roots**  
> Dopamine, a neurotransmitter in humans and many animals, traces its origins back to bacteria, suggesting a late evolutionary horizontal gene transfer. This fascinating connection highlights dopamine's deep evolutionary roots.

Sources: s11 (Wikipedia, Comparative biology and evolution), s12 (Wikipedia, Comparative biology and evolution)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Dopamine's ancient bacterial roots", "body": "Dopamine, a neurotransmitter in humans and many animals, traces its origins back to bacteria, suggesting a late evolutionary horizontal gene transfer.", "sources": ["s11", "s12"]}
```
</details>


### ethylene

**Danger level** · passed on retry · 1 snippets · 3.1 s

> **Beware of flames and dizziness**  
> Ethylene is an extremely flammable gas that can cause drowsiness or dizziness. It should be kept away from flames and handled with care.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Beware of flames and dizziness", "body": "Ethylene is extremely flammable as a gas and can cause drowsiness or dizziness if inhaled.", "sources": ["s1"]}
```
</details>

**Origin story** · passed · 4 snippets · 3.7 s

> **A gas from ancient Greece**  
> Ethylene, a gas linked to the trance-like states of the Greek Oracle of Delphi, was first discovered by Johann Joachim Becher in 1669 through heating ethanol with sulfuric acid. Later, in 1779, Joseph Priestley and Jan Ingenhousz contributed to its synthesis knowledge.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 14 snippets · 5.1 s

> **The gas that makes fruits ripe**  
> Ethylene, a sweet-smelling gas, is the plant hormone that ripens fruits like bananas and apples. It's also used in making plastics and as a refrigerant.

Sources: s4 (Wikipedia, Introduction), s6 (Wikipedia, Introduction), s13 (PubChem, Natural Pollution Sources), s14 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed on retry · 18 snippets · 14.3 s

> **Ethylene: Plant hormone and industrial marvel**  
> Ethylene, a plant hormone, also powers the massive production of polyethylene, a plastic used worldwide. It's both a natural emission from plants and a key player in the chemical industry.

Sources: s11 (Wikipedia, Introduction), s12 (Wikipedia, Introduction), s13 (Wikipedia, Introduction)  
Fun: _ /5

<details><summary>Attempt 1 rejected: title has 12 words, at most 8 allowed</summary>

```json
{"title": "Ethylene: The gas that smells like rotten eggs and makes plants ripen", "body": "Ethylene is a gas with a faint 'sweet and musky' odour, known for causing fruits to ripen. It's also used in the chemical industry to produce plastics like polyethylene.", "sources": ["s11", "s12"]}
```
</details>


### geosmin

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 1.5 s

> **Beware of swallowing geosmin**  
> Geosmin is officially rated harmful if swallowed. It can cause serious irritation if it comes into contact with your skin.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 6 snippets · 3.8 s

> **Earthy scents in nature**  
> Geosmin, a bicyclic alcohol with a distinct earthy smell, is produced by bacteria like cyanobacteria and is responsible for the earthy taste of beetroots and the petrichor scent after rain. It's found in soil, water, and even in farmed fish.

Sources: s4 (PubChem, Record Description), s5 (Wikipedia, Introduction), s6 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed on retry · 9 snippets · 9.1 s

> **Earthy smell detected by humans**  
> Humans can smell geosmin at incredibly low concentrations, as low as 400 parts per trillion to 5 parts per trillion. This extreme sensitivity helps us find water but also makes us dislike muddy-tasting fish and drinking water.

Sources: s1 (Wikipedia, Effects), s2 (Wikipedia, Effects), s4 (Wikipedia, Introduction)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Earthy scent detected by mere drops", "body": "Geosmin's earthy smell can be detected in water at astonishingly low concentrations, as little as 0.006 to 0.01 micrograms per liter!", "sources": ["s4"]}
```
</details>


### indigo

**Danger level** · passed · 1 snippets · 1.5 s

> **Watch out for long-term eye exposure**  
> Indigo is rated as a health hazard and may cause damage to organs through prolonged or repeated exposure. It's important to handle it with care.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 6 snippets · 3.2 s

> **The blue jeans mystery**  
> Indigo dye, a pigment known as C.I. Pigment Blue 66, is the secret behind the iconic blue jeans. It's a synthetic marvel, produced in massive quantities, and binds to fibers to create a range of shades.

Sources: s3 (Wikipedia, Introduction), s4 (Wikipedia, Introduction), s5 (Wikipedia, Uses), s6 (Wikipedia, Uses)  
Fun: _ /5

**Origin story** · rejected · 4 snippets · 9.6 s

<details><summary>Attempt 1 rejected: name Asia is not in the cited snippets</summary>

```json
{"title": "Ancient Dye of Luxury", "body": "Indigo, a dye used for centuries in Asia and ancient civilizations, was first discovered in Huaca Prieta, Peru, 6,000 years ago. India became the earliest major center for its production and processing, with Indigofera tinctoria species being domesticated there. The dye made its way to the Greeks and Romans, valued as a luxury product.", "sources": ["s1", "s2", "s3"]}
```
</details>

<details><summary>Attempt 2 rejected: name Greece is not in the cited snippets; name Rome is not in the cited snippets</summary>

```json
{
"title": "Ancient Dye with a Rich History",
"body": "Indigo dye, dating back 6,000 years to Huaca Prieta, Peru, was used by ancient civilizations across Mesopotamia, Egypt, Britain, Mesoamerica, and West Africa. India, the earliest major center for indigo production, domesticated Indigofera tinctoria and exported the dye to Greece, Rome, and eventually Europe. The term 'indigo' originates from the Greek 'indikón' (Ἰνδικόν), reflecting India's association with the dye.",
"sources": ["s1", "s2", "s3"]
}
```
</details>

**Weird fact** · passed · 17 snippets · 8.3 s

> **Indigo's mysterious origins**  
> Indigo dye, a deep blue pigment, has a fascinating history dating back 6,000 years to Huaca Prieta, Peru. Its journey from ancient civilizations like Mesopotamia and Egypt to becoming a luxury item in Europe is a tale of cultural exchange and trade.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History), s5 (Wikipedia, Uses), s6 (Wikipedia, Uses), s7 (Wikipedia, Dyeing technology), s8 (Wikipedia, Dyeing technology), s9 (Wikipedia, Chemical properties), s10 (Wikipedia, Chemical properties), s11 (Wikipedia, Indigo as an organic semiconductor), s12 (Wikipedia, Safety and the environment), s13 (Wikipedia, Safety and the environment), s14 (Wikipedia, Introduction), s15 (Wikipedia, Introduction), s16 (PubChem, Record Description), s17 (PubChem, Record Description)  
Fun: _ /5


### limonene

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 1.9 s

> **Watch out for flames and water!**  
> Limonene is a flammable liquid that can irritate your skin and cause dizziness. It's very toxic to aquatic life, so don't let it go down the drain!

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 7.9 s

> **Citrus scents and cleaner vibes**  
> Limonene, the scent of citrus, pops up in foods, cleaning products, and even medicines. It's a key player in flavors, fragrances, and as a solvent for oils and waxes.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Uses), s10 (Wikipedia, Uses), s11 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Natural Pollution Sources), s16 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 16 snippets · 8.0 s

> **Limonene's dual personality**  
> Limonene smells like oranges but smells like turpentine when it's not in its citrus form. It's a chiral molecule with two enantiomers, (+)-d-limonene and (−)-l-limonene, each with distinct smells.

Sources: s1 (Wikipedia, In plants), s8 (Wikipedia, Introduction), s10 (Wikipedia, Introduction), s14 (PubChem, Record Description), s15 (PubChem, Record Description), s16 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### lycopene

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 1.7 s

> **Lycopene: Safe and Non-Hazardous**  
> Lycopene is not classified as hazardous by the majority of companies and meets GHS criteria for safety. No official GHS hazard statements apply.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 4.0 s

> **Red in fruits and veggies**  
> Lycopene, a tetraterpene, gives tomatoes, pink grapefruit, and watermelon their red color. It's also found in rosehips and guava.

Sources: s2 (PubChem, Record Description), s6 (Wikipedia, Introduction), s8 (Wikipedia, Occurrence), s11 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 19 snippets · 6.5 s

> **Lycopene's rainbow of isomers**  
> Lycopene can twist into over a thousand different shapes, known as isomers, yet it's the all-trans form that paints our tomatoes red. It's like a molecular chameleon!

Sources: s11 (PubChem, Record Description), s14 (PubChem, Record Description)  
Fun: _ /5


### menthol

**Danger level** · passed · 1 snippets · 1.6 s

> **Beware of skin irritation**  
> Menthol is rated as an irritant, which means it can cause skin irritation. It's not classified as a more serious hazard by the companies that reviewed it.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 1 snippets · 2.7 s

> **A cool discovery in 1771**  
> Menthol was first isolated in 1771 by German chemist Hieronymus David Gaubius. It was later named by F. L. Alphonse Oppenheim in 1861.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 18 snippets · 6.2 s

> **Peppermint's cool sensation**  
> Menthol, found in peppermint oil, gives that refreshing cool feeling when you inhale it. It's used in everything from toothpaste to cough drops.

Sources: s1 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s10 (Wikipedia, Occurrence), s11 (Wikipedia, Applications), s12 (Wikipedia, Applications), s13 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Uses), s16 (PubChem, Uses), s17 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 25 snippets · 7.6 s

> **Menthol's mysterious origins**  
> Menthol was first isolated in 1771 by German scientist Hieronymus David Gaubius. Its name was coined by Oppenheim in 1861, making it a fascinating blend of history and chemistry.

Sources: s1 (Wikipedia, History), s1 (Wikipedia, History)  
Fun: _ /5


### morphine

**Danger level** · passed · 1 snippets · 2.2 s

> **Beware of morphine's effects**  
> Morphine is rated as a warning due to its potential to cause acute toxicity if swallowed, allergic skin reactions, drowsiness, dizziness, and reproductive toxicity. It may also harm breast-fed children.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 3.8 s

> **Dreamy discovery by a pharmacist**  
> In December 1804, Friedrich Sertürner discovered morphine, the first active alkaloid extracted from the opium poppy. He noticed its sleep-inducing effects and reported his findings in 1805. Morphine was first marketed as a pain medication in 1817.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Weird fact** · passed · 13 snippets · 6.5 s

> **Morphine's ancient roots and modern myths**  
> Morphine, discovered in 1804, has a history dating back to 2100 BC with opium-based cures. A myth suggests morphine killed Rebecca Wood, but she lived ten years longer than her husband.

Sources: s1 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History), s11 (Wikipedia, Overdose)  
Fun: _ /5

**Where you meet it** · passed · 18 snippets · 9.1 s

> **Morphine: Nature's Painkiller**  
> Morphine, the main alkaloid of opium, is found in the poppy plant (Papaver somniferum) and has been used since 1805 for pain relief. It's a potent analgesic but comes with risks of addiction and dependence.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Medical uses), s10 (Wikipedia, Medical uses), s11 (Wikipedia, Natural occurrence), s12 (Wikipedia, Natural occurrence), s13 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Uses), s16 (PubChem, Uses), s17 (PubChem, Natural Pollution Sources), s18 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### nicotine

**Danger level** · passed · 1 snippets · 1.6 s

> **Beware of smoke and water**  
> Nicotine is highly toxic if swallowed, absorbed through the skin, or inhaled. It can harm aquatic life if it enters water bodies.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.3 s

> **From poison to pleasure**  
> Nicotine was first isolated from the tobacco plant in 1828 by Wilhelm Heinrich Posselt and Karl Ludwig Reimann, who initially thought it was a poison. The tobacco plant, Nicotiana tabacum, was named after Jean Nicot de Villemain, who promoted its medicinal use in 1560. Nicotine's addictive properties led to its widespread popularity in Europe and its cultivation as a cash crop in Virginia colonies.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 17 snippets · 7.3 s

> **Nicotine: A double-edged sword**  
> Nicotine, the addictive component in tobacco, is also used medically to help people quit smoking. Found in tobacco leaves and other plants like tomatoes and potatoes, it's a potent neurotoxin in insects but a stimulant for humans.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Uses), s10 (Wikipedia, Uses), s11 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Natural Pollution Sources), s16 (PubChem, Natural Pollution Sources), s17 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 15 snippets · 6.3 s

> **Nicotine's double life**  
> Nicotine was once thought to be a poison, but it's now a key player in quitting smoking. It's addictive, but also used to help people stop smoking!

Sources: s1 (Wikipedia, History), s5 (Wikipedia, Uses), s9 (Wikipedia, Adverse effects)  
Fun: _ /5


### nitroglycerin

**Danger level** · passed · 1 snippets · 2.4 s

> **Beware of explosions and toxic fumes**  
> Nitroglycerin is officially rated as an explosive, toxic if swallowed, fatal upon skin contact, and fatal if inhaled. It may cause organ damage with prolonged exposure and is toxic to aquatic life with long-lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.0 s

> **The explosive discovery**  
> In 1846, Italian chemist Ascanio Sobrero synthesized nitroglycerin, calling it 'pyroglycerin'. Alfred Nobel later adopted it as an explosive, founding Nobel and Company after a tragic factory explosion in 1864. Despite its power, nitroglycerin's instability led to bans on its transportation, like the one in California after a devastating San Francisco explosion.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 8.1 s

> **Explosive in everyday life**  
> From the explosive energy in dynamite to the medicinal relief in nitroglycerin, this molecule powers both construction and health. It's a double-edged sword, used in medicine and industry with great care.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Use as an explosive and a propellant), s10 (Wikipedia, Use as an explosive and a propellant), s11 (Wikipedia, Medical use), s12 (Wikipedia, Medical use), s13 (PubChem, Uses), s14 (PubChem, Uses), s15 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 14 snippets · 7.4 s

> **Nitroglycerin's double life**  
> Nitroglycerin was first used as an explosive by Alfred Nobel in 1867, but it's also a lifesaver in medicine for angina and high blood pressure. It's like a superhero with a villainous past!

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s10 (Wikipedia, Use as an explosive and a propellant), s11 (Wikipedia, Medical use), s12 (Wikipedia, Medical use), s13 (Wikipedia, Industrial exposure), s14 (Wikipedia, Industrial exposure)  
Fun: _ /5


### quinine

**Danger level** · passed · 1 snippets · 2.2 s

> **Watch out for your eyes and skin!**  
> Quinine is officially rated as a danger to your eyes and skin. It can cause serious eye irritation and skin irritation. If inhaled, it may cause allergy or asthma symptoms or breathing difficulties.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 8.0 s

> **A sweet taste on his hands**  
> Quinine was used as a muscle relaxant by the Quechua Indians of Peru, Bolivia and Ecuador to halt shivering. The Spanish Jesuit missionaries were the first to bring cinchona to Europe. A popular story of how it was brought to Europe by the Countess of Chinchon was disproven by medical historian Alec Haggis around 1941.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Indians is not in the cited snippets</summary>

```json
{"title": "A Jesuit's discovery", "body": "In the 17th century, Jesuit Agostino Salumbrino observed Quechua Indians using cinchona bark to treat shivering. He sent a sample to Rome for malaria treatment, leading to Jesuit's bark becoming a valuable commodity shipped from Peru to Europe.", "sources": ["s3", "s4"]}
```
</details>

**Where you meet it** · rejected · 16 snippets · 15.2 s

<details><summary>Attempt 1 rejected: 1600s is not in the cited snippets</summary>

```json
{"title": "Quinine: Nature's Antimalarial", "body": "Quinine, the bitter-tasting compound from cinchona tree bark, has been fighting malaria since the 1600s. It's also found in tonic water, giving it that distinctive bitter taste.", "sources": ["s2", "s6", "s11", "s15"]}
```
</details>

<details><summary>Attempt 2 rejected: 1600s is not in the cited snippets</summary>

```json
{"title": "Quinine: Nature's Antimalarial", "body": "Quinine, an alkaloid from cinchona tree bark, has been fighting malaria since the 1600s. It's found in tonic water and was once a go-to for leg cramps, though now it's used more cautiously due to side effects.", "sources": ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16"]}
```
</details>

**Weird fact** · passed on retry · 12 snippets · 12.1 s

> **Quinine's fluorescent secret**  
> Quinine, known for its bitter taste and malaria-fighting properties, also glows under UV light with a bright blue/cyan hue. This fluorescent emission peaks at around 460 nm, making it a colorful secret hidden in plain sight.

Sources: s5 (Wikipedia, Uses), s12 (Wikipedia, Chemistry)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name UV is not in the cited snippets; body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Quinine's fluorescent secret", "body": "Quinine, the malaria fighter, also glows under UV light with a bright blue hue, revealing its hidden fluorescent secret.", "sources": ["s11"]}
```
</details>


### serotonin

**Danger level** · passed · 1 snippets · 1.6 s

> **Serotonin: Toxic if swallowed, fertility concerns**  
> Serotonin is classified as toxic if swallowed and may damage fertility or harm unborn children. Handle with care.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 2 snippets · 7.7 s

<details><summary>Attempt 1 rejected: body has 6 sentences, needs 2 to 4</summary>

```json
{"title": "A mysterious vasoconstrictor", "body": "In 1935, Vittorio Erspamer discovered a substance called 'enteramine' that made intestines contract. Two years later, he identified it as serotonin. In 1948, Maurice M. Rapport and colleagues discovered serotonin in blood serum. By 1952, it was confirmed that enteramine and serotonin were the same substance. In 1953, Betty Twarog and Page found serotonin in the central nervous system.", "sources": ["s1", "s2"]}
```
</details>

<details><summary>Attempt 2 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{
"title": "Discovery of a Vasoconstrictor Material",
"body": "In 1935, Vittorio Erspamer discovered a vasoconstrictor material in serum, initially believed to contain adrenaline. In 1948, Maurice M. Rapport and colleagues identified it as serotonin. By 1952, enteramine was confirmed as serotonin, leading to its widespread use in pharmacology. Synonyms include 5-hydroxytriptamine and enteramine.",
"sources": ["s1", "s2"]
}
```
</details>

**Where you meet it** · passed · 8 snippets · 5.0 s

> **Serotonin: Mood and More**  
> Serotonin, or 5-hydroxytryptamine (5-HT), is a neurotransmitter that plays a role in mood, cognition, and physiological processes like vomiting and vasoconstriction. It's synthesized in the gut and brain, influencing everything from our happiness to our blood pressure.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 14 snippets · 6.3 s

> **Serotonin's secret life**  
> Did you know serotonin, the mood regulator, also plays a role in plant stress signals? It's like plants have their own version of a 'feeling blue' moment when they're under stress!

Sources: s3 (Wikipedia, Cellular effects), s9 (Wikipedia, Comparative biology & evolution)  
Fun: _ /5


### strychnine

**Danger level** · passed · 1 snippets · 1.7 s

> **Dangerous to humans and fish**  
> Strychnine is officially rated fatal if swallowed or in contact with skin. It is very toxic to aquatic life, causing long-lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.3 s

> **Discovery of Strychnine**  
> In 1818, French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier discovered strychnine in the Saint-Ignatius' bean. Later, in 1946, Sir Robert Robinson determined its structure, and in 1954, Robert B. Woodward synthesized it in a lab, earning them Nobel prizes.

Sources: s1 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 5.0 s

> **Poisonous Plant Power**  
> Strychnine, a toxic alkaloid from Strychnos nux-vomica seeds, is used as a rodenticide and historically as a muscle stimulant. It's no longer medicinal but beware of its presence in certain plants.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (PubChem, Uses), s11 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed on retry · 10 snippets · 12.7 s

> **Strychnine's bitter bite**  
> Strychnine is a neurotoxin that's super bitter, activating bitter taste receptors TAS2R10 and TAS2R46 in humans. It's also used in rodenticides, but there's no antidote for poisoning.

Sources: s5 (Wikipedia, Mechanism of action), s6 (Wikipedia, Toxicity), s8 (Wikipedia, Treatment), s9 (Wikipedia, Treatment), s10 (Wikipedia, Pharmacokinetics)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Strychnine's bitter bite", "body": "Strychnine is so bitter it can activate taste receptors TAS2R10 and TAS2R46 in humans, making it a potent deterrent against ingestion.", "sources": ["s6", "s10"]}
```
</details>


### sucralose

**Danger level** · passed on retry · 1 snippets · 3.5 s

> **Sucralose: Safe but not classified**  
> Sucralose is not classified as hazardous by the majority of companies. For more details, visit the ECHA C&L website.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Sucralose: Safe but not classified", "body": "Sucralose is not classified as hazardous by the majority of companies, but it's always good to check the ECHA C&L website for more details.", "sources": ["s1"]}
```
</details>

**Origin story** · passed · 4 snippets · 4.2 s

> **Sweet discovery in 1975**  
> Sucralose was discovered in 1975 by scientists from Tate & Lyle, working with Leslie Hough and Shashikant Phadnis at Queen Elizabeth College (now King's College London). Phadnis found the chlorinated sugar compound exceptionally sweet after 'testing' it. Tate & Lyle patented sucralose in 1976.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 5.0 s

> **Sweetness without the sugar**  
> Sucralose, a sugar substitute, makes your favorite snacks and drinks taste sweet without the calories. It's found in candies, breakfast bars, and even canned fruits, replacing high-energy corn syrup.

Sources: s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (Wikipedia, Introduction), s5 (Wikipedia, Introduction), s6 (Wikipedia, Uses), s7 (Wikipedia, Uses), s9 (PubChem, Uses), s10 (PubChem, Uses), s11 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 19 snippets · 14.4 s

> **Sweetness 600 times stronger**  
> Sucralose is 600 times sweeter than table sugar, making it a potent sugar substitute. This artificial sweetener is used in many products, replacing sugar without adding calories.

Sources: s10 (Wikipedia, Introduction), s14 (PubChem, Record Description), s17 (PubChem, Uses), s19 (PubChem, Uses)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Sweetness 600 times stronger", "body": "Sucralose is 600 times sweeter than table sugar, making it a potent sugar substitute.", "sources": ["s10", "s14"]}
```
</details>


### tetrodotoxin

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 1.9 s

> **Beware of tetrodotoxin's deadly touch**  
> Tetrodotoxin is officially rated as fatal if swallowed, in contact with skin, or inhaled. It poses a serious risk of acute toxicity through these routes.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Weird fact** · passed · 16 snippets · 7.5 s

> **Tetrodotoxin: A deadly yet promising painkiller?**  
> Tetrodotoxin is a potent neurotoxin found in pufferfish and other animals, causing paralysis and death at high doses. Surprisingly, it's being explored as a painkiller for cancer patients, showing promise in clinical trials.

Sources: s1 (Wikipedia, Biochemistry), s2 (Wikipedia, Biochemistry), s3 (Wikipedia, Poisoning), s11 (Wikipedia, Introduction), s13 (PubChem, Record Description), s15 (PubChem, Record Description)  
Fun: _ /5

**Where you meet it** · passed · 15 snippets · 8.0 s

> **Poisonous Puffer Fish**  
> Tetrodotoxin, a potent neurotoxin, is found in the liver and ovaries of certain fish like pufferfish. It's also produced by bacteria and found in other animals like octopuses and newts.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (PubChem, Uses), s11 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Natural Pollution Sources), s14 (PubChem, Natural Pollution Sources), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### thalidomide

**Danger level** · passed · 1 snippets · 1.6 s

> **Beware of swallowing and pregnancy**  
> Thalidomide is dangerous if swallowed and may harm fertility or unborn children. It's officially rated as a health hazard and an irritant.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.6 s

> **A wonder drug's dark side**  
> In 1957, Chemie Grünenthal launched thalidomide as a 'wonder drug' for various ailments. Unbeknownst to them, it caused severe birth defects when taken by pregnant women. The drug was removed from the market in 1961 after reports of thousands of affected children emerged.

Sources: s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed on retry · 12 snippets · 11.2 s

> **A double-edged pill**  
> Thalidomide, once a sedative, now fights cancers and leprosy. But beware: it's a teratogen, causing birth defects if taken during pregnancy.

Sources: s1 (PubChem, Record Description), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Medical uses), s10 (Wikipedia, Medical uses), s11 (PubChem, Uses), s12 (PubChem, Uses)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name THALIDOMID is not in the cited snippets; name REMS is not in the cited snippets</summary>

```json
{"title": "A pill with a troubled past", "body": "Thalidomide, once a sedative, now treats cancers and skin disorders. Its history is marred by birth defects, leading to strict regulations like the THALIDOMID REMS program.", "sources": ["s1", "s6", "s8", "s9", "s10", "s11", "s12"]}
```
</details>

**Weird fact** · passed · 14 snippets · 7.5 s

> **Thalidomide's double life**  
> Thalidomide, once a 'wonder drug' for morning sickness, turned into a nightmare causing birth defects. Yet, it's now a hero in treating multiple myeloma and leprosy-related inflammation.

Sources: s2 (Wikipedia, History), s3 (Wikipedia, History), s5 (Wikipedia, Medical uses), s6 (Wikipedia, Medical uses), s9 (Wikipedia, Adverse effects), s10 (Wikipedia, Pharmacology), s11 (Wikipedia, Pharmacology), s12 (Wikipedia, Chemistry), s13 (Wikipedia, Chemistry), s14 (Wikipedia, Society and culture)  
Fun: _ /5


### theobromine

**Danger level** · passed · 1 snippets · 1.6 s

> **Beware of swallowing and eye contact**  
> Theobromine is rated as harmful if swallowed and can cause serious eye irritation. There's also a suspicion that it might cause cancer.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 3 snippets · 2.8 s

> **God's Food Discovery**  
> In 1841, A. Woskresensky discovered theobromine in cacao beans. Hermann Emil Fischer synthesized it from xanthine in 1882. The name 'theobromine' comes from Theobroma, the cacao tree's genus, meaning 'food of the gods'.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, Etymology), s3 (Wikipedia, Etymology)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 6.0 s

> **Chocolate's Secret Kick**  
> Theobromine, chocolate's secret kick, is a mild heart stimulant and bronchodilator. Found in chocolate, tea, and cola nuts, it's a metabolite of caffeine with a playful twist on your favorite treat.

Sources: s2 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (PubChem, Uses), s9 (PubChem, Uses), s10 (PubChem, Uses), s11 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 20 snippets · 9.1 s

> **Caffeine's mysterious cousin**  
> Theobromine, a cousin of caffeine, is found in chocolate and tea. Unlike caffeine, it doesn't significantly stimulate the brain, making chocolate safe for humans but toxic to pets like dogs and cats.

Sources: s1 (Wikipedia, History), s10 (Wikipedia, Introduction), s11 (PubChem, Record Description), s12 (PubChem, Record Description), s13 (PubChem, Record Description), s14 (PubChem, Record Description), s15 (PubChem, Record Description), s16 (PubChem, Natural Pollution Sources), s17 (PubChem, Toxicity Summary), s18 (PubChem, Toxicity Summary), s19 (PubChem, Uses), s20 (PubChem, Uses)  
Fun: _ /5


### vanillin

**Danger level** · passed · 1 snippets · 1.7 s

> **Vanillin: Swallowing harm and aquatic danger**  
> Vanillin is rated as harmful if swallowed and toxic to aquatic life with long-lasting effects. It poses a hazard to the aquatic environment over time.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 8.5 s

> **Vanilla's ancient and modern origins**  
> Vanilla was first isolated as a pure substance in 1858 by Théodore Nicolas Gobley. Traces of vanillin were found in jars from Canaan dating to the 2nd millennium BCE. Vanillin was first synthesized in 1874 by Ferdinand Tiemann and Wilhelm Haarmann from coniferin in pine bark.

Sources: s1 (Wikipedia, Natural history), s2 (Wikipedia, Natural history), s3 (Wikipedia, Synthetic history)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Canaanite is not in the cited snippets</summary>

```json
{"title": "Vanilla's ancient and modern origins", "body": "Vanilla was first domesticated in Mesoamerica and spread to the Old World in the 16th century. In 1858, vanillin was isolated by Théodore Nicolas Gobley. Later, in 1874, German scientists Ferdinand Tiemann and Wilhelm Haarmann synthesized vanillin from coniferin, starting industrial production in Holzminden, Germany. Vanillin was also found in ancient Canaanite jars dating back to the 2nd millennium BCE.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Where you meet it** · passed · 19 snippets · 6.1 s

> **Vanilla's sweet journey**  
> Vanillin, the star of vanilla beans, flavors our sweets and drinks. It's also a key note in fragrances and a helper in medicines.

Sources: s6 (Wikipedia, Introduction), s13 (Wikipedia, Uses), s14 (Wikipedia, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 21 snippets · 13.7 s

> **Vanillin's ancient origins**  
> Vanillin was found in jars from Canaan dating back to the 2nd millennium BCE, suggesting ancient cultivation of vanilla-like plants. This predates the domestication of vanilla in Mesoamerica by centuries!

Sources: s1 (Wikipedia, Natural history), s2 (Wikipedia, Natural history)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Vanillin's ancient origins", "body": "Vanillin, the flavor behind vanilla, was found in ancient Canaan jars, suggesting its cultivation there since the Middle Bronze Age!", "sources": ["s1", "s2"]}
```
</details>

