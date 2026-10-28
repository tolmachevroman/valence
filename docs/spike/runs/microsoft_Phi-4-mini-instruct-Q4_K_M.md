# Spike run: microsoft_Phi-4-mini-instruct-Q4_K_M

30 molecules, 111 cards with input. Temperature 0.2, JSON schema constraint, one stricter retry, shortest card first, pick-then-write for Weird and Where, Danger from GHS data, KV cache: paged, fixed 8K tokens.

## Summary

| Measure | Value |
| --- | --- |
| From GHS data (no model) | 28 (25%) |
| Passed first try | 60 (54%) |
| Passed on retry | 19 (17%) |
| Skipped by model | 0 (0%) |
| Rejected twice (dropped) | 4 (4%) |
| No input (no card) | 9 |
| Molecules with 3+ cards | 28 of 30 |
| First text of a model card | 4.6 s median, 6.3 s p90, 6.8 s max |
| Model card, including pick and retry | 7.0 s median, 10.2 s p90, 11.3 s max |
| First card per molecule | 0.0 s median, 0.0 s p90, 3.2 s max |
| First model card per molecule | 4.6 s median, 8.0 s p90, 14.4 s max |
| Model load | 5.3 s |
| Peak memory | 4.46 GB |

### Problems found (all attempts)

- code not in sources: 1
- name not in sources: 3
- number not in sources: 2
- sentence count: 23

## Cards

Fun score is for hand review in M0.6.

### allicin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Allicin carries the official GHS signal word "Warning". Its hazard statements: causes skin irritation, causes serious eye irritation, and may cause respiratory irritation.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 2 snippets · 6.1 s

> **Discovery in Japan**  
> Allicin was discovered as part of efforts to create thiamine derivatives in the 1940s, mainly in Japan. Chester J. Cavallito and John Hays Bailey first isolated and studied it in 1944.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: 1940s is not in the cited snippets; name Japan is not in the cited snippets</summary>

```json
{"title": "Discovery of Allicin", "body": "Allicin was first isolated and studied in 1944 by Chester J. Cavallito and John Hays Bailey. It was discovered as part of efforts to create thiamine derivatives in the 1940s, mainly in Japan.", "sources": ["s1"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 4.9 s

> **Garlic's secret weapon**  
> Allicin, the compound that gives garlic its distinctive smell, is also its defense against pests. When garlic is crushed, the enzyme alliinase transforms alliin into allicin, which then breaks down into other sulfur compounds.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 5.8 s

> **Garlic's secret weapon**  
> Allicin, the compound that gives garlic its punch, was first discovered in 1944 by scientists Chester J. Cavallito and John Hays Bailey. But here's the twist: allicin is unstable and quickly transforms into other sulfur compounds, making it a fleeting but potent defender against pests.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, Introduction)  
Fun: _ /5


### artemisinin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Artemisinin carries the official GHS signal word "Danger". Its hazard statements: heating may cause a fire, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.2 s

> **Sweet wormwood's secret**  
> Artemisinin, an antimalarial lactone, was discovered by Tu Youyou in 1972 during a secret military program to treat malaria. Inspired by ancient texts, she found it in sweet wormwood (Artemisia annua) and isolated it using a low-temperature extraction process. Her discovery led to effective treatments and combination therapies for malaria.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 5.9 s

> **Malaria's ancient remedy**  
> Artemisinin, a sesquiterpene lactone, comes from sweet wormwood, Artemisia annua. Discovered in 1972 by Tu Youyou, it earned her a Nobel Prize in 2015. It's used in antimalarial treatments worldwide.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 6.5 s

> **Artemisinin's mysterious action**  
> Artemisinin's exact mechanism of action remains a mystery as of 2018. It's a prodrug that transforms inside red blood cells to kill parasites by damaging proteins with free radicals.

Sources: s1 (Wikipedia, Mechanism of action)  
Fun: _ /5


### ascorbic acid

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Ascorbic acid is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.2 s

> **The cure for scurvy**  
> Scurvy was known since ancient times, with Hippocrates and Pliny describing its symptoms. In 1747, ship's surgeon James Lind conducted an experiment showing citrus fruits prevented scurvy. By 1796, the British navy adopted lemon juice as standard issue at sea, leading to the nickname 'limey' for British sailors.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.4 s

> **Vitamin C in your fruit!**  
> Ascorbic acid, a water-soluble vitamin, is essential for maintaining connective tissue and bones. Found naturally in citrus fruits and vegetables, it's an antioxidant and reducing agent that fights bacterial infections and detoxifies reactions.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 7.7 s

> **The sour secret of scurvy prevention**  
> Captain James Cook proved that carrying 'Sour krout' (lime juice) on board could prevent scurvy, a deadly disease at sea. Surprisingly, it wasn't until 1860 that the Royal Navy adopted this practice for all its ships, long after Lind's 1753 discovery.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5


### aspirin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Aspirin carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed. A minority of reports list 3 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 9.2 s

> **From ancient remedies to modern medicine**  
> Medicines made from willow and other salicylate-rich plants appeared in clay tablets from ancient Sumer as well as the Ebers Papyrus from ancient Egypt. In 1853, Charles Frédéric Gerhardt treated sodium salicylate with acetyl chloride to produce acetylsalicylic acid for the first time. In 1897, scientists at Bayer identified a new way to synthesize it and produced acetylsalicylic acid in a pure, stable form.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Sumerian is not in the cited snippets</summary>

```json
{"title": "From ancient remedies to modern medicine", "body": "Aspirin's journey began with ancient Sumerian clay tablets and Hippocrates' salicylic tea. In 1853, chemist Charles Frédéric Gerhardt synthesized acetylsalicylic acid. Bayer scientists, led by Felix Hoffmann in 1897, produced pure aspirin. By 1899, Bayer marketed it globally under the brand name Aspirin.", "sources": ["s1", "s2", "s3", "s4"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 6.6 s

> **Aspirin: Pain relief and beyond**  
> Aspirin, or acetylsalicylic acid (ASA), is a common drug used for pain relief, fever reduction, and inflammation. It's also used to prevent blood clots and reduce the risk of certain cancers.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Medical use)  
Fun: _ /5

**Weird fact** · passed on retry · 2 snippets · 10.3 s

> **Aspirin's crystal conundrum**  
> Aspirin, known for its pain-relieving powers, has a secret: it can form more than one crystal structure. In 2005, scientists confirmed this mystery, revealing form II, a polymorph that had been debated since the 1960s and even dismissed as an impurity in 1981.

Sources: s1 (Wikipedia, Mechanism of action), s2 (Wikipedia, Physical properties)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Aspirin's secret polymorph", "body": "Aspirin, known for its pain-relieving powers, hides a secret: besides its common form I, there's a debated form II, discovered in 1981 but initially dismissed as an impurity.", "sources": ["s2"]}
```
</details>


### batrachotoxin

**Danger level** · no input · 0 snippets · 0.0 s

**Origin story** · passed · 1 snippets · 2.4 s

> **Discovery of a Potent Toxin**  
> In 1963, Fritz Märki and Bernhard Witkop discovered batrachotoxin at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. They isolated the toxin from the black-legged poison dart frog's toxic alkaloids fraction.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 6.4 s

> **Poison darts and frogs**  
> The Noanamá Chocó and Emberá Chocó of western Colombia use batrachotoxin to poison blowgun darts for hunting. This toxin is also found in poison dart frogs and certain birds.

Sources: s1 (Wikipedia, Use), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 2 snippets · 10.4 s

> **Potent poison from tiny frogs**  
> Batrachotoxin was discovered by Fritz Märki and Bernhard Witkop in 1963 from the black-legged poison dart frog. It's extremely potent, acting directly on sodium channels, making it the most potent non-peptidal neurotoxin known.

Sources: s1 (Wikipedia, History), s2 (PubChem, Record Description)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Poison dart frogs' deadly secret", "body": "Batrachotoxin, discovered by Fritz Märki and Bernhard Witkop in 1963, is the most potent non-peptidal neurotoxin known, found in poison dart frogs and other creatures.", "sources": ["s1", "s2"]}
```
</details>


### benzylpenicillin

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Benzylpenicillin carries the official GHS signal word "Danger". Its hazard statements: may cause an allergic skin reaction. A minority of reports list 3 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.0 s

> **Battling bacteria with benzylpenicillin**  
> Benzylpenicillin, also known as PenG, is a narrow-spectrum antibiotic used to treat serious bacterial infections like pneumonia, meningitis, gonorrhea, and syphilis. It's administered intravenously or intramuscularly due to poor oral absorption and is not a first-line agent for pneumococcal meningitis.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · rejected · 2 snippets · 8.2 s

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Antibiotic with a double-edged sword", "body": "Benzylpenicillin fights off nasty bacteria but can also cause serious reactions like anaphylaxis and even convulsions in rare cases.", "sources": ["s1", "s2"]}
```
</details>

<details><summary>Attempt 2 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Antibiotic's nervous system nemesis", "body": "Benzylpenicillin is an antibiotic that fights bacteria but can cause convulsions in the nervous system, especially at high doses or in severe renal impairment.", "sources": ["s2"]}
```
</details>


### buckminsterfullerene

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Buckminsterfullerene carries the official GHS signal word "Warning". Its hazard statements: causes serious eye irritation, and may cause respiratory irritation. A minority of reports list 1 other hazard.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 3.9 s

> **Laser vaporization discovery**  
> In 1984, Eric Rohlfing, Donald Cox, and Andrew Kaldor first generated buckminsterfullerene using a laser to vaporize carbon in a supersonic helium beam. Later, in 1985, Harold Kroto and his team at Rice University recognized the structure of C60 as buckminsterfullerene.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed on retry · 3 snippets · 7.3 s

> **Nature's carbon ball**  
> Buckminsterfullerene, a carbon molecule shaped like a soccer ball, is found in tiny amounts in soot. It was first discovered in shungite, a rock rich in carbon.

Sources: s1 (Wikipedia, Occurrence), s2 (PubChem, Natural Pollution Sources)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Nature's carbon ball", "body": "Buckminsterfullerene, a carbon ball, pops up in soot and shungite, a rock rich in carbon.", "sources": ["s1", "s2"]}
```
</details>

**Weird fact** · passed on retry · 2 snippets · 9.3 s

> **The giant molecule with wave-particle duality**  
> Buckminsterfullerene was once the largest molecule known to exhibit wave-particle duality. In 2020, a dye molecule called phthalocyanine showed this duality too.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, Properties)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "The giant molecule with wave-particle duality", "body": "Buckminsterfullerene was once the largest molecule known to exhibit wave-particle duality, a property more famously attributed to light and electrons.", "sources": ["s1", "s2"]}
```
</details>


### caffeine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Caffeine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 3.0 s

> **Ancient caffeine origins**  
> Caffeine's history traces back to ancient times. West African cultures chewed kola nuts for vitality, while ancient Mayans consumed bitter chocolate with caffeine. Xocolatl, a spicy chocolate drink, was believed to fight fatigue. The Spaniards introduced cacao to Europe around 1700.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Weird fact** · passed on retry · 2 snippets · 9.4 s

> **Caffeine's complex receptor dance**  
> Caffeine blocks all four adenosine receptor subtypes, but it's especially good at blocking A2A receptors, which scientists think is why it keeps us awake. It's like caffeine has a favorite dance move in the brain's receptor party.

Sources: s1 (Wikipedia, Overdose), s2 (Wikipedia, Pharmacology)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Caffeine's double life", "body": "Caffeine blocks adenosine receptors to keep us awake, but too much can lead to a dependency with symptoms like nervousness and headaches.", "sources": ["s1", "s2"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 8.3 s

> **The buzz in your morning brew**  
> Caffeine, a methylxanthine, is the kick in your coffee or tea. It's also found in cacao beans and used in cosmetics and medicines.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (Wikipedia, Introduction)  
Fun: _ /5


### capsaicin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Capsaicin carries the official GHS signal word "Danger". Its hazard statements: harmful if swallowed, causes skin irritation, and causes serious eye damage. A minority of reports list 1 other hazard.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 2.4 s

> **Burning discovery**  
> In 1816, Christian Friedrich Bucholz extracted capsaicin in impure form. Later, in 1873 and 1878, Rudolf Buchheim and Endre Hőgyes respectively identified capsaicin's effects on mucous membranes and gastric acid secretion.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.6 s

> **Spicy adventures with capsaicin**  
> Capsaicin, the fiery molecule in chili peppers, spices up our meals and even our neurobiological research. It's also used as a topical analgesic and a repellent for various animals and insects.

Sources: s1 (Wikipedia, Uses), s2 (Wikipedia, Uses), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 7.8 s

> **Capsaicin's mysterious evolution**  
> Capsaicin, the spicy compound in peppers, evolved across the Capsicum genus over millions of years, likely due to genetic drift and natural selection. What's surprising is that this evolution happened after the genus split from Solanaceae 19.6 million years ago and only started to rapidly diversify in the past 2 million years.

Sources: s1 (Wikipedia, Evolution)  
Fun: _ /5


### carvone

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Carvone carries the official GHS signal word "Warning". Its hazard statements: may cause an allergic skin reaction.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 1 snippets · 2.5 s

> **Carvol's Ketone Journey**  
> Carvone, originally called carvol, was probably isolated as a pure compound in 1849 by Franz Varrentrapp. It was identified as a ketone related to limonene by Goldschmidt and Zürrer, and its structure was finally elucidated by Georg Wagner in 1894.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 6.9 s

> **Carving flavors and scents**  
> Carvone, a terpenoid, is found in caraway, spearmint, and dill oils. It's used in food, air fresheners, and even to prevent potatoes from sprouting prematurely.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Uses), s3 (Wikipedia, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 2 snippets · 11.3 s

> **Two smells, one molecule**  
> Carvone smells different depending on its enantiomer: R-(−)-carvone smells like spearmint, while S-(+)-carvone smells like caraway seeds. Each smell comes from a different mirror image of the same molecule.

Sources: s1 (Wikipedia, Stereoisomerism and odor), s2 (PubChem, Natural Pollution Sources)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Mirror image smells", "body": "Carvone has two mirror image forms, or enantiomers: R-(−)-carvone smells sweet like spearmint, while its mirror image, S-(+)-carvone smells spicy like caraway seeds.", "sources": ["s1"]}
```
</details>


### cinnamaldehyde

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Cinnamaldehyde carries the official GHS signal word "Warning". Its hazard statements: may cause an allergic skin reaction.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.7 s

> **Cinnamon's cousin in flavor and fight**  
> Cinnamaldehyde, a cousin to cinnamon, adds sweet, spicy notes to gum and ice cream. It's also a natural insecticide against mosquitoes, with a 29 ppm concentration killing half in 24 hours.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Applications), s3 (Wikipedia, Applications)  
Fun: _ /5

**Weird fact** · passed on retry · 2 snippets · 10.8 s

> **Cinnamaldehyde: A double agent**  
> Cinnamaldehyde is tested as a safe insecticide against mosquito larvae, killing half at 29 ppm in 24 hours. It also activates TRPA1, causing a warming sensation when you eat cinnamon. It's like a spicy bug killer and a spicy mouthful rolled into one!

Sources: s1 (Wikipedia, Applications), s2 (Wikipedia, Biological effects)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Cinnamaldehyde: Mosquito killer and spice sensation", "body": "Cinnamaldehyde not only kills mosquito larvae and repels adults but also activates TRPA1 receptors, causing the warming sensation we associate with spices like cinnamon.", "sources": ["s1", "s2"]}
```
</details>


### cubane

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · no input · 0 snippets · 0.0 s

**Where you meet it** · passed · 1 snippets · 3.2 s

> **Explosive Cubane**  
> Cubane's high potential energy and kinetic stability make it useful for controlled energy storage. Octanitrocubane and heptanitrocubane, derivatives of cubane, have been studied as high-performance explosives.

Sources: s1 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 3.6 s

> **Strained yet stable cubane**  
> Cubane is highly strained, yet it can't decompose into pyramidal alkenes, which would be too high-energy. Instead, it rearranges into cuneane with the help of certain metallic ions.

Sources: s1 (Wikipedia, Reactions and derivatives)  
Fun: _ /5


### dopamine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Dopamine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed, suspected of damaging fertility or the unborn child, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 4.4 s

> **Dopamine's Discovery and Dopamine-Derived Polymers**  
> Dopamine was first synthesized in 1910 by George Barger and James Ewens at Wellcome Laboratories in London. Its role as a neurotransmitter was identified in 1958 by Carlsson and Hillarp. In 2007, researchers discovered that materials coated with dopamine, known as polydopamine, can form through a spontaneous oxidation reaction.

Sources: s1 (Wikipedia, History and development), s2 (Wikipedia, History and development)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.4 s

> **Brain's messenger in medicine**  
> Dopamine, a brain neurotransmitter, helps regulate movement and is the precursor to norepinephrine and epinephrine. Medically, it's used as a stimulant for severe low blood pressure and heart rate issues in newborns, administered intravenously due to its short plasma half-life.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Medical uses), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 6.6 s

> **Bacteria's secret to animal dopamine?**  
> Dopamine, a brain chemical, is found in some bacteria and protozoans, not archaea. Surprisingly, bacteria might have given animals their dopamine-making tools through a late evolutionary gift called horizontal gene transfer.

Sources: s1 (Wikipedia, Comparative biology and evolution)  
Fun: _ /5


### ethylene

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Ethylene carries the official GHS signal word "Danger". Its hazard statements: extremely flammable gas, and may cause drowsiness or dizziness.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 3.7 s

> **A gas from ancient Greece**  
> Ethylene, a gas linked to the trance-like states of the Greek Oracle of Delphi, was first discovered by Johann Joachim Becher in 1669. Later, in 1866, August Wilhelm von Hofmann proposed a naming system that included ethylene as ethene.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 2 snippets · 5.2 s

> **The ripening agent**  
> Ethylene is a plant hormone that helps fruits ripen. It's also used in making plastics like polythene and polystyrene.

Sources: s1 (Wikipedia, Introduction), s2 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 2 snippets · 9.9 s

> **Oracle's trance from gas**  
> Some scholars believe the Greek Oracle of Delphi (the Pythia) entered a trance-like state due to ethylene rising from ground faults. Ethylene is a colorless gas with a sweet odor and taste, easily ignited, and can cause explosions.

Sources: s1 (Wikipedia, History), s2 (PubChem, Record Description)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Oracle's trance from gas", "body": "The Pythia, the famous Greek Oracle of Delphi, may have entered her trance-like state due to ethylene rising from ground faults.", "sources": ["s1"]}
```
</details>


### geosmin

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Geosmin carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 5.2 s

> **Earthy scents and muddy water**  
> Geosmin, a bicyclic alcohol with formula C12H22O, is responsible for the earthy taste of beetroots and the strong scent known as petrichor. It's produced by bacteria, especially cyanobacteria, often present in soil and water supplies.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 5.5 s

> **Earthy scent with a tiny threshold**  
> Geosmin has an incredibly low odor detection threshold, ranging from 0.006 to 0.01 micrograms per liter in water. This means you can smell it at levels so low they're almost undetectable!

Sources: s2 (Wikipedia, Introduction)  
Fun: _ /5


### indigo

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Indigo carries the official GHS signal word "Warning". Its hazard statements: may causes damage to organs through prolonged or repeated exposure.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 4.4 s

> **The blue jeans mystery**  
> Indigo dye, a rare blue pigment, is famously used in denim cloth production, especially for blue jeans. A pair of jeans typically needs 3–12 grams of indigo dye. Most indigo dye today is synthetic, produced in large quantities each year.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction), s3 (Wikipedia, Uses)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.3 s

> **Ancient dye discovery**  
> Indigo dye was first discovered in Huaca Prieta, Peru, around 6,000 years ago. It was used by ancient civilizations across Mesopotamia, Egypt, Britain, Mesoamerica, Peru, Iran, and West Africa. India became the earliest major center for indigo production and processing, with Indigofera tinctoria being domesticated there. The dye spread to the Greeks and Romans, valued as a luxury product imported from India.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 7.0 s

> **Indigo's disappearing act**  
> Indigo dye undergoes a magical transformation when removed from water: it turns white and then magically reverts to its deep blue color upon exposure to air. This dyeing trick puzzled Europeans in the 16th century!

Sources: s1 (Wikipedia, Dyeing technology)  
Fun: _ /5


### limonene

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Limonene carries the official GHS signal word "Warning". Its hazard statements: flammable liquid and vapor, causes skin irritation, may cause an allergic skin reaction, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 8.0 s

> **Citrus scent in everyday life**  
> Limonene, the main component of citrus fruit peels, gives them their characteristic scent. It's used in food manufacturing as a flavoring agent and in cleaning products for its citrus aroma.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction), s3 (Wikipedia, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 1 snippets · 10.4 s

> **Nature's single-handed citrus scent**  
> Limonene, the scent of citrus, is produced mainly by one enantiomer in nature. Most citrus fruit peels, the main commercial source, yield just the (R)-enantiomer, d-limonene.

Sources: s1 (Wikipedia, Introduction)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Nature's single-handed citrus scent", "body": "Limonene, the scent of citrus, is produced mainly by one enantiomer in nature, with d-limonene being the most common in citrus fruit peels.", "sources": ["s1"]}
```
</details>


### lycopene

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Lycopene is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 5.4 s

> **The red in your fruit**  
> Lycopene, a carotenoid, is found in tomatoes, watermelons, grapefruits, red guavas, and baked beans. It's known for its antioxidant properties and is believed to help prevent chronic diseases.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Occurrence), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 8.2 s

> **Lycopene's mysterious color**  
> Lycopene's deep red color comes from its 11 conjugated double bonds, which absorb light in the green-blue region of the spectrum. This makes it a mysterious pigment, as it's not just the presence of double bonds but their specific arrangement that gives lycopene its unique color.

Sources: s1 (Wikipedia, Structure and physical properties), s2 (PubChem, Record Description)  
Fun: _ /5


### menthol

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Menthol is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 1 snippets · 5.6 s

<details><summary>Attempt 1 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "Menthol's Discovery Journey", "body": "Menthol was first isolated in 1771 by German chemist Hieronymus David Gaubius. Early characterizations were done by Oppenheim, Beckett, Moriya, and Atkinson. It was named by F. L. Alphonse Oppenheim in 1861.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "Menthol's Discovery", "body": "Menthol was first isolated in 1771 by German chemist Hieronymus David Gaubius. Early characterizations were done by Oppenheim, Beckett, Moriya, and Atkinson. It was named by F. L. Alphonse Oppenheim in 1861.", "sources": ["s1"]}
```
</details>

**Where you meet it** · passed on retry · 3 snippets · 8.7 s

> **Peppermint's chill**  
> Menthol, a white crystalline solid with a peppermint odor and taste, is found in peppermint and mint oils. It's used in pharmaceuticals and foods as a fortifier for peppermint flavors and as a counterirritant for local analgesic effects.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Record Description)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Peppermint's chill", "body": "Menthol, a white crystalline solid with a peppermint scent, is found in peppermint oil and used in pharmaceuticals and foods for its flavoring and anesthetic properties.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · passed on retry · 2 snippets · 10.6 s

> **Menthol's dual sensation trick**  
> Menthol cools by triggering cold-sensitive TRPM8 receptors, unlike capsaicin which causes heat sensation. Interestingly, menthol is used in both cooling products and cigarettes.

Sources: s1 (Wikipedia, Biological properties), s2 (PubChem, Toxicity Summary)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Menthol's dual sensation trick", "body": "Menthol cools by triggering cold-sensitive receptors, yet it's also used in products like cigarettes and sweets, showing its versatility beyond just a cooling sensation.", "sources": ["s1", "s2"]}
```
</details>


### morphine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Morphine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed. A minority of reports list 5 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.8 s

> **Discovery of Morphine**  
> Morphine was first extracted as an active alkaloid from the opium poppy plant in December 1804 by German pharmacist Friedrich Sertürner in Paderborn. He noticed its sedative effects and reported his findings in 1805. Sertürner named the substance morphium, after Morpheus, the Greek god of dreams, due to its sleep-inducing properties. Morphine was initially marketed as a pain medication and treatment for addiction in 1817.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Weird fact** · passed on retry · 2 snippets · 9.4 s

> **Morphine's ancient origins**  
> Morphine's earliest known reference dates back to Theophrastus in the 3rd century BC, but possible references to morphine may go as far back as 2100 BC in Sumerian clay tablets. This suggests morphine's use in medical prescriptions could be even older than previously thought.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, Adverse effects)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Morphine's ancient origins", "body": "Morphine's earliest known reference dates back to Theophrastus in the 3rd century BC, but possible references may go as far back as 2100 BC in Sumerian clay tablets.", "sources": ["s1"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 8.8 s

> **Morphine: Nature's Painkiller**  
> Morphine, the main alkaloid of opium, was first isolated from poppy seeds in 1805. It's a potent analgesic used to treat severe pain, with FDA approval granted in 1941. Morphine can be administered orally, via injection, or through other methods, acting directly on the CNS.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Medical uses)  
Fun: _ /5


### nicotine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Nicotine carries the official GHS signal word "Danger". Its hazard statements: fatal if swallowed, fatal in contact with skin, fatal if inhaled, and toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.4 s

> **From poison to pleasure**  
> Nicotine was first isolated from the tobacco plant in 1828 by Wilhelm Heinrich Posselt and Karl Ludwig Reimann, who initially believed it was a poison. The tobacco plant, Nicotiana tabacum, was named after French ambassador Jean Nicot de Villemain, who promoted its medicinal use in 1560. Nicotine's addictive properties and the tobacco plant's popularity as a cure-all led to its widespread cultivation in Virginia colonies by the early 17th century.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 6.6 s

> **The buzz in your cigarette**  
> Nicotine, a stimulant found in tobacco, makes up about 0.6–3% of tobacco leaves. Cigarettes typically contain 1-2% nicotine.

Sources: s1 (Wikipedia, Introduction), s2 (PubChem, Uses), s3 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 7.2 s

> **Nicotine's mild analgesic effect**  
> Nicotine is classified as a poison and extremely hazardous, yet at doses typically used by consumers, it presents little if any hazard to adult users. Interestingly, at low amounts, nicotine has a mild analgesic effect.

Sources: s1 (Wikipedia, Adverse effects)  
Fun: _ /5


### nitroglycerin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Nitroglycerin carries the official GHS signal word "Danger". Its hazard statements: (Deleted) Explosive; mass explosion hazard, fatal if swallowed, fatal in contact with skin, fatal if inhaled, and toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 8.2 s

> **First practical explosive**  
> Nitroglycerin was the first practical explosive stronger than black powder, synthesized by Ascanio Sobrero in 1846 at the University of Turin. Alfred Nobel adopted it commercially after a factory explosion killed his brother in 1864. Nobel's Krümmel factory in Germany faced numerous catastrophes due to its instability. In 1866, California banned liquid nitroglycerin transport after an explosion killed 15 people.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: number 1865 is not in the cited snippets</summary>

```json
{"title": "The explosive discovery", "body": "In 1846, Italian chemist Ascanio Sobrero synthesized nitroglycerin, calling it 'pyroglycerin'. He warned against its explosive use. Alfred Nobel later improved its handling, founding Nobel and Company in 1865 after a factory explosion killed several workers. Despite safety efforts, nitroglycerin's instability led to bans and tragedies, like the 1866 San Francisco explosion.", "sources": ["s1", "s2", "s3", "s4"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 7.6 s

> **Explosive in dynamite**  
> Nitroglycerin is the explosive heart of dynamite, discovered by Alfred Nobel in 1867. It's used in civil engineering tasks like drilling tunnels and clearing farmland, as well as in demolition work.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Use as an explosive and a propellant), s3 (Wikipedia, Use as an explosive and a propellant)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 7.3 s

> **Nitroglycerin: Headache hero?**  
> Nitroglycerin was first discovered in 1846 by Ascanio Sobrero. Surprisingly, while it's a powerful explosive, it's also known for causing severe headaches called 'NG head' or 'bang head'.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, Industrial exposure)  
Fun: _ /5


### quinine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Quinine carries the official GHS signal word "Danger". Its hazard statements: may cause an allergic skin reaction, and may cause allergy or asthma symptoms or breathing difficulties if inhaled. A minority of reports list 3 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.6 s

> **Quinine's Quechua Roots**  
> Quinine's story begins with the Quechua Indians of Peru, Bolivia, and Ecuador, who used cinchona tree bark as a muscle relaxant to stop shivering. Spanish Jesuit missionaries brought cinchona to Europe in the 1570s, where it was used to treat diarrhea. Quinine has been used in Europe since the early 17th century, despite a myth about its introduction by the Countess of Chinchon.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 7.6 s

> **Quinine: Malaria's Bitter Ally**  
> Quinine, an alkaloid from cinchona tree bark, has been fighting malaria since 1633. It's also found in tonic water, giving it a distinctive bitter taste.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 6.8 s

> **Quinine's double-edged sword**  
> Quinine, known for treating malaria, can also cause severe blood disorders like thrombocytopenia and thrombotic microangiopathy. It's a dangerous dance between cure and poison.

Sources: s1 (Wikipedia, Contraindications), s2 (Wikipedia, Adverse effects)  
Fun: _ /5


### serotonin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Serotonin carries the official GHS signal word "Danger". Its hazard statements: toxic if swallowed, and suspected of damaging fertility or the unborn child.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 2 snippets · 7.7 s

> **Discovery of a vasoconstrictor**  
> In 1935, Vittorio Erspamer discovered a substance called 'enteramine' that made intestines contract. He later identified it as serotonin in 1948. In 1952, it was confirmed that enteramine and serotonin were the same substance. Betty Twarog and Page found serotonin in the central nervous system in 1953.

Sources: s1 (Wikipedia, History & etymology), s2 (Wikipedia, History & etymology)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 6 sentences, needs 2 to 4</summary>

```json
{"title": "Discovery of Serotonin", "body": "In 1935, Vittorio Erspamer identified a vasoconstrictor in enterochromaffin cells, naming it 'enteramine'. Two years later, he confirmed it was a new amine. In 1948, Maurice M. Rapport and colleagues discovered a similar substance in blood serum, naming it 'serotonin'. By 1952, it was established that enteramine and serotonin were the same. In 1953, Betty Twarog and Page found serotonin in the central nervous system.", "sources": ["s1", "s2"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 6.3 s

> **Serotonin: The body's mood and gut messenger**  
> Serotonin is a versatile molecule found mainly in the gut, brain, and blood platelets. In the gut, it's made by enterochromaffin cells and helps move food along. In the brain, it's produced in the raphe nuclei and helps regulate mood and sleep. Blood platelets release serotonin to help blood clot and keep blood pressure stable.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 7.1 s

> **Serotonin's dual roles in nature**  
> Serotonin serves as a toxin for algae, yet it's also a secretory agent for parasites and a byproduct in seed drying. It's a molecule with a surprising range of functions across different organisms.

Sources: s1 (Wikipedia, Comparative biology & evolution), s2 (Wikipedia, Comparative biology & evolution)  
Fun: _ /5


### strychnine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Strychnine carries the official GHS signal word "Danger". Its hazard statements: fatal if swallowed, fatal in contact with skin, very toxic to aquatic life, and very toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.0 s

> **Discovery of Strychnine**  
> In 1818, French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier discovered strychnine in the Saint-Ignatius' bean. Later, in 1946, Sir Robert Robinson determined its structure, and in 1954, Robert B. Woodward synthesized it in a laboratory.

Sources: s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 5.5 s

> **Poisonous seeds**  
> Strychnine, an alkaloid from Strychnos nux-vomica seeds, is used in rodent poison baits and as a pesticide. It's also been explored for medical uses like treating sleep apnea.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 2 snippets · 10.8 s

> **Strychnine's deadly history**  
> Strychnine was first discovered in 1818 and has a dark history of being used for murder, including by William Palmer and Oskar Dirlewanger. It's also been used in World War II against civilians. Interestingly, while strychnine is extremely poisonous, its less toxic cousin brucine is found in some Strychnos plants.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, Treatment)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Strychnine's ancient and deadly uses", "body": "Strychnine, discovered in 1818, has a dark history of being used for murder and warfare, including by William Palmer and Oskar Dirlewanger.", "sources": ["s1"]}
```
</details>


### sucralose

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Not classified as hazardous**  
> Sucralose is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.7 s

> **Sweet discovery in 1975**  
> Sucralose was discovered in 1975 by scientists from Tate & Lyle, working with Leslie Hough and Shashikant Phadnis at Queen Elizabeth College (now part of King's College London). Phadnis found the chlorinated sugar compound exceptionally sweet after 'testing' it, leading to its patent in 1976. Sucralose gained worldwide approval by 2008.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 5.6 s

> **Sweetening your life**  
> Sucralose, a sugar substitute, sweetens your favorite snacks and drinks without the calories. It's found in candies, breakfast bars, coffee pods, and even canned fruits, replacing high-energy corn syrup.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Uses), s3 (Wikipedia, Uses)  
Fun: _ /5

**Weird fact** · passed on retry · 2 snippets · 10.1 s

> **Sweetness and heat stability**  
> Sucralose is about 600 times sweeter than table sugar and retains its sweetness even when baked at normal temperatures. However, when heated above 120 °C, it can form unknown amounts of potentially harmful compounds like PCDD and PCDF.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Safety evaluation)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Sweetness 600 times stronger", "body": "Sucralose is an artificial sweetener that's 600 times sweeter than table sugar, making it a potent sugar substitute.", "sources": ["s1"]}
```
</details>


### tetrodotoxin

**Origin story** · no input · 0 snippets · 0.0 s

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Tetrodotoxin carries the official GHS signal word "Danger". Its hazard statements: fatal if swallowed, fatal in contact with skin, and fatal if inhaled.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Weird fact** · passed · 2 snippets · 7.5 s

> **Tetrodotoxin: Deadlier than cyanide**  
> Tetrodotoxin is a sodium channel blocker that's more poisonous than cyanide when taken orally. Even a tiny amount can be lethal if injected.

Sources: s1 (Wikipedia, Poisoning), s2 (Wikipedia, Introduction)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 8.7 s

> **Poison in the sea**  
> Tetrodotoxin lurks in the liver and gonads of puffer fish and other marine creatures like globefish and toadfish. It's also found in some amphibians, octopuses, and shellfish. Improperly prepared fish can lead to fatal poisoning.

Sources: s1 (PubChem, Uses), s2 (PubChem, Uses), s3 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### thalidomide

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Danger**  
> Thalidomide carries the official GHS signal word "Danger". Its hazard statements: harmful if swallowed, and may damage fertility or the unborn child.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 4.6 s

> **A wonder drug's dark side**  
> In 1957, Chemie Grünenthal launched thalidomide as Contergan, hailed as a 'wonder drug' for various ailments. However, it was later discovered to cause severe birth defects when taken by pregnant women, leading to its withdrawal in 1961. The tragedy affected thousands of children worldwide.

Sources: s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 6.0 s

> **A controversial cure**  
> Thalidomide, once a sedative, now treats multiple myeloma and skin disorders like leprosy. It's a first-line treatment for ENL and used in HIV-related skin conditions.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Medical uses), s3 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · rejected · 1 snippets · 10.2 s

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Thalidomide's complex birth", "body": "Thalidomide's journey from L-glutamic acid to its final form is a chemical odyssey involving L-glutamine and carbonyldiimidazole, with yields jumping from a mere 31% to a whopping 85–93%!", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Thalidomide's complex birth", "body": "Celgene Corporation's original thalidomide synthesis started with L-glutamic acid but now uses L-glutamine, achieving a much higher yield of 85–93% pure product compared to the initial 31% yield of S-thalidomide.", "sources": ["s1"]}
```
</details>


### theobromine

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Theobromine carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed. A minority of reports list 2 other hazards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 3 snippets · 2.7 s

> **Godly Food Discovery**  
> Theobromine was discovered in 1841 in cacao beans by chemist A. Woskresensky. Hermann Emil Fischer synthesized it from xanthine in 1882. Its name comes from Theobroma, meaning 'food of the gods', despite containing no bromine.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, Etymology), s3 (Wikipedia, Etymology)  
Fun: _ /5

**Where you meet it** · passed · 3 snippets · 6.3 s

> **Chocolate's secret ingredient**  
> Theobromine, a bitter alkaloid from the cacao plant, is the star of chocolate and other foods like tea and cola nuts. It's also a white or colorless solid that can sometimes look yellowish.

Sources: s1 (PubChem, Record Description), s2 (Wikipedia, Introduction), s3 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 1 snippets · 8.4 s

> **Theobromine's double life**  
> Theobromine is a metabolite of caffeine that peaks in the blood 2–3 hours after ingestion and acts as a mild heart stimulant and bronchodilator in humans. Surprisingly, it's toxic or even fatal to animals like dogs and cats due to their slower metabolism of the compound.

Sources: s1 (Wikipedia, Introduction)  
Fun: _ /5


### vanillin

**Danger level** · from GHS data · 1 snippets · 0.0 s

> **Official signal word: Warning**  
> Vanillin carries the official GHS signal word "Warning". Its hazard statements: harmful if swallowed, and toxic to aquatic life with long lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 7.5 s

> **Vanilla's ancient origins**  
> Vanillin was first isolated in 1858 by Théodore Nicolas Gobley. Traces of vanillin were found in jars from Canaan dating to the 2nd millennium BCE. Vanillin was first synthesized in 1874 by Ferdinand Tiemann and Wilhelm Haarmann.

Sources: s1 (Wikipedia, Natural history), s3 (Wikipedia, Synthetic history)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Canaanite is not in the cited snippets</summary>

```json
{"title": "Vanilla's ancient and modern origins", "body": "Vanilla was domesticated in Mesoamerica and spread to the Old World in the 16th century. In 2019, researchers found vanillin residue in ancient Canaanite jars, suggesting cultivation since the Middle Bronze Age. Vanillin was first isolated in 1858 by Théodore Nicolas Gobley and synthesized industrially by Ferdinand Tiemann and Wilhelm Haarmann in 1874.", "sources": ["s1", "s3"]}
```
</details>

**Where you meet it** · passed · 3 snippets · 7.2 s

> **Vanilla's sweet journey**  
> Vanillin, the star of sweet treats, flavors ice cream and chocolate, making up 75% of its use in the flavor industry. It also adds a creamy note to drinks like cream soda and hides less pleasant tastes in medicines and cleaning products.

Sources: s1 (Wikipedia, Uses), s2 (Wikipedia, Uses)  
Fun: _ /5

**Weird fact** · rejected · 2 snippets · 10.7 s

<details><summary>Attempt 1 rejected: number 2000 is not in the cited snippets; body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Vanillin's ancient origins", "body": "Vanillin, the flavor behind vanilla, was found in ancient Canaan jars dating back to 2000 BCE, suggesting it was cultivated there since the Middle Bronze Age, much earlier than its domestication in Mesoamerica.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Vanillin's ancient origins", "body": "Vanillin was first isolated in 1858, but traces were found in 2nd millennium BCE jars in Canaan, suggesting it was cultivated much earlier.", "sources": ["s1", "s2"]}
```
</details>

