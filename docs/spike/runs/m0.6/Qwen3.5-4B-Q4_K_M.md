# Spike run: Qwen3.5-4B-Q4_K_M

30 molecules, 111 cards with input. Temperature 0.6, JSON schema constraint, one stricter retry, shortest card first, KV cache: plain.

## Summary

| Measure | Value |
| --- | --- |
| Passed first try | 96 (86%) |
| Passed on retry | 13 (12%) |
| Skipped by model | 0 (0%) |
| Rejected twice (dropped) | 2 (2%) |
| No input (no card) | 9 |
| Molecules with 3+ cards | 29 of 30 |
| First token | 2.8 s median, 5.6 s p90, 6.3 s max |
| Card, including retry | 7.0 s median, 11.1 s p90, 17.7 s max |
| First card per molecule | 3.6 s median, 4.6 s p90, 12.3 s max |
| Model load | 13.1 s |
| Peak memory | 7.58 GB |

### Problems found (all attempts)

- code not in sources: 2
- name not in sources: 10
- number not in sources: 4
- sentence count: 4
- title: 2

## Cards

Fun score is for hand review in M0.6.

### allicin

**Danger level** · passed · 1 snippets · 3.6 s

> **Watch out for skin and eye irritation**  
> Allicin carries a Warning signal and is officially rated as an irritant. It can cause skin irritation, serious eye damage, and respiratory tract irritation upon exposure.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 5.1 s

> **Allicin's 1944 discovery in Japan**  
> In 1944, Chester J. Cavallito and John Hays Bailey first isolated and studied allicin in the laboratory. This discovery occurred during efforts to create thiamine derivatives, mainly in Japan, where allicin became a model for medicinal chemistry. The work led to other compounds like sulbutiamine and fursultiamine.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 7 snippets · 5.1 s

> **The punch in your garlic**  
> Allicin is the oily, slightly yellow liquid that gives fresh garlic its distinctive odor when you crush it. It acts as a botanical anti-fungal and antibacterial agent, serving as a natural defense against pests. This unstable compound forms only when garlic tissue is damaged by chopping or crushing.

Sources: s1 (PubChem, Record Description), s4 (Wikipedia, Introduction), s5 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed on retry · 11 snippets · 11.6 s

> **Garlic's secret weapon**  
> Allicin is the compound responsible for the aroma of fresh garlic when tissue damage occurs. It is formed by the enzyme alliinase acting on alliin, serving as a defense mechanism against pests. This oily, slightly yellow liquid is unstable and quickly changes into other sulfur-containing compounds.

Sources: s3 (Wikipedia, Structure and occurrence), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction)  
Fun: _ /5

<details><summary>Attempt 1 rejected: title has 9 words, at most 8 allowed</summary>

```json
{"title": "Garlic's secret weapon only appears when you crush it", "body": "Allicin doesn't exist in fresh garlic until you damage the plant's cells. The enzyme alliinase then instantly converts alliin into allicin, creating the sharp smell and acting as a defense against pests.", "sources": ["s3", "s7"]}
```
</details>


### artemisinin

**Danger level** · passed · 1 snippets · 3.2 s

> **Fire risk and aquatic toxicity**  
> Artemisinin is officially rated dangerous because heating it may cause a fire. It is also very toxic to aquatic life, with effects that can last a long time.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.5 s

> **From ancient tea to modern cure**  
> Li Shizhen recommended sweet wormwood tea for malaria in 1596, but Tu Youyou discovered artemisinin in 1972 during Project 523. She used a low-temperature extraction inspired by a 340 CE text to isolate the compound from Artemisia annua leaves. Her team published the results in 1979, leading to highly effective malaria treatments.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 10 snippets · 7.2 s

> **From wormwood to malaria cure**  
> Artemisinin comes from the sweet wormwood herb Artemisia annua, a plant used in traditional Chinese medicine. Today, it is a frontline therapy for malaria, often combined with other drugs to treat resistant strains. Its discovery in 1972 earned Tu Youyou the 2015 Nobel Prize in Physiology or Medicine.

Sources: s1 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (Wikipedia, Medical use)  
Fun: _ /5

**Weird fact** · passed · 10 snippets · 7.6 s

> **The drug that breaks itself open**  
> Artemisinin works by having its own endoperoxide ring snap apart when it meets iron in the parasite. This self-destruction creates free radicals that kill the malaria bug, all without us needing to force the reaction.

Sources: s8 (Wikipedia, Chemistry), s10 (Wikipedia, Mechanism of action)  
Fun: _ /5


### ascorbic acid

**Danger level** · passed · 1 snippets · 3.0 s

> **Ascorbic acid has no official GHS hazards**  
> Ascorbic acid is not classified under GHS hazard criteria. One company reported it does not meet any GHS hazard standards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.3 s

> **From scurvy to sour krout**  
> Scurvy was known to Hippocrates and described by Pliny the Elder and Strabo. James Lind conducted a controlled experiment in May 1747, showing citrus fruits prevented the disease, and published his findings in 1753. Later, Captain James Cook demonstrated the benefits of carrying 'Sour krout' on his 1772–75 voyage.

Sources: s1 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 17 snippets · 8.4 s

> **Citrus power in your diet**  
> This vitamin C lives naturally in citrus fruits, tomatoes, potatoes, and green leafy vegetables. Humans cannot make it themselves, so we must get it from our diet to keep our bones and skin strong. It also shows up as E300 in canned fruits to stop them from browning.

Sources: s3 (PubChem, Record Description), s5 (PubChem, Record Description), s9 (Wikipedia, Food preservation additive), s10 (Wikipedia, Food preservation additive)  
Fun: _ /5

**Weird fact** · passed · 15 snippets · 7.4 s

> **The vitamin that turns into its opposite**  
> Ascorbic acid is a weak sugar acid, but its form changes depending on the pH. At low pH it stays as ascorbic acid, yet above pH 5 it becomes predominantly ascorbate, its ionized form.

Sources: s5 (Wikipedia, Chemistry)  
Fun: _ /5


### aspirin

**Danger level** · passed · 1 snippets · 3.5 s

> **Warning: Irritant to skin, eyes, and lungs**  
> Aspirin carries a Warning signal and is officially classified as an irritant. It can harm the skin, cause serious eye damage, and irritate the respiratory tract if inhaled.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 13.9 s

> **From willow to Bayer's brand**  
> Medicines from willow plants appeared in clay tablets from ancient Sumer and the Ebers Papyrus from ancient Egypt. In 1853, Charles Frédéric Gerhardt treated sodium salicylate with acetyl chloride to produce acetylsalicylic acid. In 1897, Felix Hoffmann (or Arthur Eichengrün) of Bayer was the first to produce acetylsalicylic acid in a pure, stable form. The word Aspirin was Bayer's brand name for this drug.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Sumerians is not in the cited snippets; name Egyptians is not in the cited snippets</summary>

```json
{"title": "From willow to Bayer's brand", "body": "Ancient Sumerians and Egyptians used salicylate-rich plants, and Hippocrates recommended salicylic tea around 400 BC. In 1853, Charles Frédéric Gerhardt first produced acetylsalicylic acid, and by 1897, Felix Hoffmann (or Arthur Eichengrün) at Bayer created a pure, stable form. Bayer sold the drug globally under the brand name Aspirin by 1899.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Where you meet it** · passed · 13 snippets · 6.3 s

> **The painkiller in your medicine cabinet**  
> You likely find aspirin in pain reliever tablets or heart health supplements designed to stop blood clots. It is also used to treat conditions like Kawasaki disease and reduce the risk of stroke in high-risk people.

Sources: s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (Wikipedia, Medical use)  
Fun: _ /5

**Weird fact** · passed · 14 snippets · 7.2 s

> **The drug that poisons the air**  
> Factories making aspirin must control airborne powder because the dust can explode. This explosive risk is real enough that safety agencies set strict exposure limits for workers.

Sources: s8 (Wikipedia, Chemical properties)  
Fun: _ /5


### batrachotoxin

**Danger level** · no input · 0 snippets · -0.0 s

**Origin story** · passed · 1 snippets · 4.3 s

> **A 1963 discovery in Maryland**  
> In 1963, Fritz Märki and Bernhard Witkop discovered batrachotoxin at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland. They isolated it from the black-legged poison dart frog along with three other toxic steroidal alkaloids.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 12 snippets · 8.0 s

> **Frog skin's deadly secret**  
> Batrachotoxin hides in the specialized skin glands of poison dart frogs like Phyllobates aurotaenia, as well as in certain birds and beetles. Indigenous groups in western Colombia historically used this potent neurotoxin to poison blowgun darts for hunting. While it acts as a biochemical tool for studying sodium channels, no antidote is known.

Sources: s1 (PubChem, Record Description), s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (Wikipedia, Introduction), s6 (Wikipedia, Use), s8 (PubChem, Uses), s12 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 16 snippets · 7.4 s

> **The poison that never closes**  
> Batrachotoxin irreversibly opens nerve cells' sodium channels, locking them open forever. This permanent opening causes paralysis and death, with no known antidote to reverse the damage.

Sources: s8 (Wikipedia, Introduction), s15 (PubChem, Toxicity Summary)  
Fun: _ /5


### benzylpenicillin

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 3.6 s

> **Watch out for skin, lungs, and water**  
> Benzylpenicillin carries a Danger signal and can cause allergic skin reactions or breathing issues if inhaled. It also poses significant risks to aquatic life, both immediately and over the long term.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed on retry · 12 snippets · 15.8 s

> **The injection that fights infection**  
> Benzylpenicillin, also known as Penicillin G, is an antibiotic used to treat infections like pneumonia, syphilis, and strep throat. Because it is not well absorbed by the mouth, it is generally given as an injection into a vein or muscle. It is on the World Health Organization's List of Essential Medicines and is considered a drug of choice for certain gram-positive bacteria.

Sources: s3 (PubChem, Record Description), s6 (Wikipedia, Introduction), s8 (Wikipedia, Introduction)  
Fun: _ /5

<details><summary>Attempt 1 rejected: number 4500 is not in the cited snippets</summary>

```json
{"title": "The injection that fights infection", "body": "Benzylpenicillin, also known as Penicillin G, is a narrow-spectrum antibiotic given as an injection to treat serious infections like pneumonia, syphilis, and strep throat. It remains on the World Health Organization's List of Essential Medicines and is widely used in Germany, where over 4,500 kg were used in 2009. While generally safe during pregnancy, it requires careful handling due to potential allergic reactions.", "sources": ["s1", "s2", "s3", "s6", "s8", "s10"]}
```
</details>

**Weird fact** · passed · 15 snippets · 7.2 s

> **The drug that smells like mold**  
> Benzylpenicillin is a powerful antibiotic, yet it has been reported to grow in certain fungi like Sarocladium strictum. This means the medicine itself can be a food source for some organisms, which is quite unexpected for a drug designed to kill bacteria.

Sources: s12 (PubChem, Record Description)  
Fun: _ /5


### buckminsterfullerene

**Danger level** · passed · 1 snippets · 3.9 s

> **Warning: Irritates skin, eyes, and lungs**  
> Buckminsterfullerene carries a Warning signal word and can cause skin irritation, serious eye damage, and respiratory tract irritation. These hazards appear in nearly all tested concentrations for skin and eye contact, and almost all for breathing exposure.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · rejected · 4 snippets · 14.9 s

<details><summary>Attempt 1 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "From laser sparks to C60 cages", "body": "Theoretical predictions of buckminsterfullerene appeared in the late 1960s and early 1970s, but it was first generated in 1984 by Eric Rohlfing, Donald Cox, and Andrew Kaldor using a laser to vaporize carbon. In 1985, Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University recognized the structure of C60 as buckminsterfullerene. They used a solid rotating graphite disk to create hot plasma, which formed clusters including the cage-like C60 molecule.", "sources": ["s1", "s3"]}
```
</details>

<details><summary>Attempt 2 rejected: body has 5 sentences, needs 2 to 4</summary>

```json
{"title": "From theory to C60 cages", "body": "Theoretical predictions of buckminsterfullerene appeared in the late 1960s and early 1970s. In 1985, Harold Kroto, James R. Heath, Sean C. O'Brien, Robert Curl, and Richard Smalley at Rice University recognized the structure of C60 as buckminsterfullerene. They found that C60 is a cage-like molecule, a regular truncated icosahedron.", "sources": ["s1", "s3", "s4"]}
```
</details>

**Where you meet it** · passed · 16 snippets · 6.7 s

> **From soot to space**  
> You might find tiny amounts of buckminsterfullerene in soot, but it also floats in planetary nebulae and stars. While it has been studied since 1985, few real-world applications have been found yet.

Sources: s6 (Wikipedia, Introduction), s8 (Wikipedia, Occurrence), s9 (Wikipedia, Occurrence)  
Fun: _ /5

**Weird fact** · passed · 12 snippets · 8.1 s

> **A cage found in space and soot**  
> Buckminsterfullerene is the most common naturally occurring fullerene, hiding in soot and drifting through planetary nebulae in space. Despite being a man-made discovery, it is the most abundant fullerene found in nature.

Sources: s5 (Wikipedia, Occurrence), s6 (Wikipedia, Occurrence)  
Fun: _ /5


### caffeine

**Danger level** · passed · 1 snippets · 3.0 s

> **Warning: Harmful if swallowed**  
> Caffeine carries a Warning signal and is classified as harmful if swallowed. It is also marked as an irritant on its safety pictogram.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 11.1 s

> **Ancient drinks with caffeine**  
> Kola nuts were chewed in West African cultures to restore vitality. Theobromine and caffeine content in xocolatl was believed to fight fatigue. Xocolatl became a popular beverage by 1700 after being introduced to Europe by the Spaniards. Native Americans also brewed asi tea from yaupon holly leaves.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Africa is not in the cited snippets</summary>

```json
{"title": "Ancient drinks with a kick", "body": "Kola nuts have been chewed in West Africa for ancient times to restore vitality. Meanwhile, Mayans drank xocolatl around 600 BCE, a bitter, spicy beverage believed to fight fatigue. Later, Spaniards introduced xocolatl to Europe by 1700, while Native Americans brewed asi tea from yaupon holly leaves far into antiquity.", "sources": ["s1", "s2", "s3", "s4"]}
```
</details>

**Weird fact** · passed · 15 snippets · 7.8 s

> **The drink that fights its own fatigue**  
> Ancient Mayans believed chocolate's caffeine helped fight fatigue, yet caffeine actually works by blocking the chemical signal that makes you sleepy. This blockage prevents adenosine from activating receptors, artificially keeping you alert even though your body is tired.

Sources: s2 (Wikipedia, History), s13 (Wikipedia, Pharmacology)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 8.7 s

> **From coffee to cocoa**  
> Caffeine naturally lives in the seeds of coffee beans, cocoa beans, and tea leaves, where it protects plants from pests. Humans enjoy it in these foods and drinks, and it also appears in cosmetics and medicines for premature infants.

Sources: s2 (PubChem, Record Description), s5 (PubChem, Record Description), s7 (Wikipedia, Introduction), s11 (Wikipedia, Natural occurrence), s12 (PubChem, Uses), s14 (PubChem, Uses)  
Fun: _ /5


### capsaicin

**Danger level** · rejected · 1 snippets · 8.2 s

<details><summary>Attempt 1 rejected: name Capsaicicin is not in the cited snippets; name Swallowing is not in the cited snippets</summary>

```json
{"title": "Capsaicin burns skin and eyes", "body": "Capsaicicin is officially rated dangerous and can cause skin irritation or serious eye damage. Swallowing it may be toxic or harmful depending on the concentration.", "sources": ["s1"]}
```
</details>

<details><summary>Attempt 2 rejected: name Capsaicicin is not in the cited snippets</summary>

```json
{"title": "Capsaicin risks: toxic if swallowed, corrosive to skin", "body": "Capsaicicin carries a Danger signal word and is classified as corrosive, acutely toxic, and an irritant. It is toxic if swallowed (21.9%) and harmful if swallowed (78%), while it causes skin irritation (99.7%) and serious eye damage (97.2%).", "sources": ["s1"]}
```
</details>

**Origin story** · passed · 2 snippets · 4.1 s

> **Capsaicin's early discovery**  
> Christian Friedrich Bucholz first extracted capsaicin in impure form in 1816. Later, Rudolf Buchheim and Endre Hőgyes identified its burning effects in 1873 and 1878 respectively.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 15 snippets · 8.1 s

> **The heat in your chili**  
> Capsaicin gives chili peppers their spicy kick and is found in high amounts in the white pith inside the fruit. It also appears in hot sauces, pepper sprays, and even as a bird repellent to protect crops. While it burns your skin, birds can eat the seeds without feeling a thing.

Sources: s6 (Wikipedia, Introduction), s7 (Wikipedia, Natural function), s8 (Wikipedia, Natural function), s9 (Wikipedia, Uses), s11 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 14 snippets · 8.1 s

> **Birds love the burn, mammals don't**  
> Capsaicin burns mammal skin and eyes but leaves birds completely unbothered because their TRPV1 receptors ignore it. This trick lets birds eat the seeds and spread them, while mammals' teeth destroy the peppers before they can germinate.

Sources: s3 (Wikipedia, Natural function), s4 (Wikipedia, Natural function)  
Fun: _ /5


### carvone

**Danger level** · passed · 1 snippets · 2.9 s

> **Watch out for skin allergies**  
> Carvone carries a Warning signal and is classified as a skin sensitizer. It may cause an allergic skin reaction upon contact.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 1 snippets · 5.1 s

> **Carvone's 1849 discovery**  
> Although ancient Romans used caraway medicinally, carvone was likely not isolated as a pure compound until 1849 when Franz Varrentrapp obtained it. It was originally named carvol by Schweizer before Goldschmidt and Zürrer identified it as a ketone related to limonene. Georg Wagner finally elucidated its structure in 1894.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 14 snippets · 9.6 s

> **Flavoring your mint and caraway**  
> Carvone gives spearmint its fresh taste and caraway its warm spice, appearing in 50–80% of spearmint oil and 60–70% of caraway seed oil. You also find it in dill, mandarin peels, and even in the oils of gingergrass and kuromoji. Beyond cooking, it helps keep potatoes from sprouting and is used in air fresheners and dentifrices.

Sources: s4 (PubChem, Record Description), s5 (Wikipedia, Introduction), s6 (Wikipedia, Uses), s7 (Wikipedia, Uses), s8 (Wikipedia, Occurrence), s9 (PubChem, Uses), s11 (PubChem, Uses), s13 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 16 snippets · 8.5 s

> **Mirror images smell completely different**  
> R-(−)-carvone smells like sweet spearmint, while its mirror image, S-(+)-carvone, smells like spicy caraway. This proves that our noses can tell the difference between these identical molecules.

Sources: s4 (Wikipedia, Stereoisomerism and odor), s5 (Wikipedia, Stereoisomerism and odor)  
Fun: _ /5


### cinnamaldehyde

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 3.0 s

> **Watch out for skin allergies**  
> Cinnamaldehyde carries a Warning signal and is classified as a skin sensitizer. It may cause an allergic skin reaction upon contact.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 15 snippets · 8.9 s

> **The spice in your gum and tea**  
> Cinnamaldehyde gives cinnamon its signature flavor and odor, appearing in 75% of Sri Lanka cinnamon bark oil and up to 90% of cinnamon essential oil. You might taste it in chewing gum, ice cream, or beverages, where it is used as a flavoring agent. It also occurs naturally in grapefruit, celery seed, and clove buds.

Sources: s2 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Applications), s13 (PubChem, Natural Pollution Sources), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 19 snippets · 8.5 s

> **The spice that fixes broken DNA**  
> Cinnamaldehyde doesn't just flavor food; it acts as a dietary antimutagen that reduces spontaneous mutations by stimulating DNA repair in human cells and bacteria. This protective mechanism against genetic damage is surprisingly linked to its ability to trigger specific cellular repair pathways after radiation exposure.

Sources: s6 (Wikipedia, DNA repair), s7 (Wikipedia, DNA repair)  
Fun: _ /5


### cubane

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · no input · 0 snippets · -0.0 s

**Where you meet it** · passed · 2 snippets · 4.4 s

> **The cube in your fuel**  
> First made in 1964, cubane is a synthetic hydrocarbon where eight carbon atoms form a cube. Its high energy density makes it useful for storing energy in a small space, though it is currently studied for explosives rather than everyday products.

Sources: s1 (Wikipedia, Introduction), s2 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 4 snippets · 5.1 s

> **The cube that won't explode**  
> Cubane is built with sharp 90° angles that should be highly strained, yet it cannot decompose into pyramidal alkenes. This stability makes it a solid crystalline substance that stores massive energy in a tiny space without spontaneously exploding.

Sources: s1 (Wikipedia, Reactions and derivatives), s3 (Wikipedia, Introduction), s4 (Wikipedia, Introduction)  
Fun: _ /5


### dopamine

**Danger level** · passed · 1 snippets · 3.7 s

> **Warning: Harmful if swallowed and toxic to water**  
> Dopamine carries a Warning signal and is harmful if swallowed. It is suspected of damaging fertility or the unborn child, and is very toxic to aquatic life, both immediately and with long-lasting effects.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 2 snippets · 6.1 s

> **From lab bench to mussel glue**  
> George Barger and James Ewens first synthesized dopamine in 1910 at Wellcome Laboratories in London, England. Katharine Montagu identified it in the human brain in 1957, and Arvid Carlsson and Nils-Åke Hillarp recognized its function as a neurotransmitter in 1958. Later research in 2007 linked dopamine to adhesive proteins in mussels, revealing how it forms polydopamine coatings.

Sources: s1 (Wikipedia, History and development), s2 (Wikipedia, History and development)  
Fun: _ /5

**Where you meet it** · passed · 15 snippets · 8.9 s

> **From brain signals to hospital drips**  
> Dopamine acts as a neurotransmitter in your brain to signal motivation and regulate movement, while also being synthesized in plants like Magnolia officinalis. As a medicine, it is sold under names like Intropin to treat severe low blood pressure and cardiac arrest via intravenous drip. Its effects vary by dosage, influencing heart rate, blood pressure, and kidney function in critical care settings.

Sources: s2 (PubChem, Record Description), s4 (PubChem, Record Description), s7 (Wikipedia, Introduction), s9 (Wikipedia, Medical uses), s10 (Wikipedia, Medical uses)  
Fun: _ /5

**Weird fact** · passed · 12 snippets · 7.3 s

> **Dopamine's ancient bacterial roots**  
> Though we think of dopamine as a human brain chemical, it likely evolved from bacteria via horizontal gene transfer. Some bacteria even possess the exact enzymes animals use to make it, suggesting a deep evolutionary link.

Sources: s11 (Wikipedia, Comparative biology and evolution), s12 (Wikipedia, Comparative biology and evolution)  
Fun: _ /5


### ethylene

**Danger level** · passed · 1 snippets · 3.3 s

> **Extremely flammable gas that can daze you**  
> Ethylene is officially rated as an extremely flammable gas and is also a compressed gas. Exposure to its vapor may cause drowsiness or dizziness.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.1 s

> **From Delphi to the lab bench**  
> Geologists once linked ethylene to the trances of the Oracle of Delphi. Johann Joachim Becher first obtained the gas by heating ethanol with sulfuric acid in 1669, a discovery later echoed by Joseph Priestley in 1779. The name ethylene stuck even after August Wilhelm von Hofmann proposed ethene in 1866.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 14 snippets · 7.4 s

> **From fruit to plastic**  
> Nature releases ethylene from ripening fruits like avocados and apples, while humans use it to speed up that process in agriculture. Industrially, over 225 million tonnes were produced in 2022 to make plastics like polyethylene. It also appears in welding torches and once served as an anesthetic.

Sources: s5 (Wikipedia, Introduction), s6 (Wikipedia, Introduction), s13 (PubChem, Natural Pollution Sources), s14 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 18 snippets · 8.2 s

> **The gas that ripens fruit**  
> Ethylene is a simple hydrocarbon gas, yet it acts as a powerful plant hormone that triggers fruit ripening. This same molecule is also used industrially to make plastics like polyethylene.

Sources: s11 (Wikipedia, Introduction), s13 (Wikipedia, Introduction)  
Fun: _ /5


### geosmin

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 3.0 s

> **Warning: Harmful if swallowed**  
> Geosmin carries a Warning signal and is classified as harmful if swallowed. It is also marked as an irritant.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 6 snippets · 5.4 s

> **The smell of rain and mud**  
> Geosmin gives beetroots their earthy taste and creates petrichor when rain hits dry soil. It is produced by bacteria like cyanobacteria in water and soil, and was named in 1965 by Nancy N. Gerber and Hubert A. Lechevalier.

Sources: s5 (Wikipedia, Introduction), s6 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 9 snippets · 6.0 s

> **The smell that smells like nothing**  
> Geosmin is so potent that humans can detect it at concentrations as low as 400 parts per trillion. Yet, despite this extreme sensitivity, it is nontoxic to humans and does not kill them.

Sources: s1 (Wikipedia, Effects), s2 (Wikipedia, Effects)  
Fun: _ /5


### indigo

**Danger level** · passed · 1 snippets · 3.0 s

> **Indigo warns of long-term organ risks**  
> Indigo carries a Health Hazard pictogram and warns that prolonged or repeated exposure may damage organs. The official signal word for this risk is Warning.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 6 snippets · 4.8 s

> **The blue in your jeans**  
> Most indigo dye today is synthetic, making up around 80,000 tonnes each year as of 2023. It is the star ingredient in denim cloth, where a single pair of blue jeans typically uses 3–12 grams of dye.

Sources: s4 (Wikipedia, Introduction), s5 (Wikipedia, Uses)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 12.2 s

> **Ancient dye from India to Europe**  
> The Indigofera tinctoria species was domesticated in India, which was the earliest major center for its production and processing. Indigo made its way to the Greeks and the Romans, where it was valued as a luxury product. The Greek word for the dye, indikón (Ἰνδικόν, Indian), reflects this association with India.

Sources: s1 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Greece is not in the cited snippets</summary>

```json
{"title": "Ancient dye from India to Europe", "body": "The Indigofera tinctoria species was domesticated in India, the earliest major center for indigo production. Ancient civilizations in Mesopotamia, Egypt, and Greece used the dye, with the word indigo deriving from the Greek indikón meaning Indian. By the seventh century BC, recipes for dyeing wool existed in Mesopotamia, and later Arab merchants imported this luxury item to the Mediterranean.", "sources": ["s1", "s2", "s3"]}
```
</details>

**Weird fact** · passed · 17 snippets · 8.1 s

> **The dye that needs urine to work**  
> Before modern chemistry, European dyers dissolved indigo in stale urine because it contains ammonia, a pre-industrial trick to make the insoluble dye soluble. This method was so common it became a specific historical technique for producing indigo white.

Sources: s7 (Wikipedia, Dyeing technology), s8 (Wikipedia, Dyeing technology)  
Fun: _ /5


### limonene

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 4.0 s

> **Warning: Flammable, irritating, and toxic to water**  
> Limonene carries a Warning signal word and is classified as flammable, a skin irritant, and a potential allergen. It poses serious risks to aquatic life, causing both immediate and long-lasting toxicity in water environments.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 8.6 s

> **Citrus zest in your kitchen**  
> Limonene is the major scent component in citrus fruit peels, giving lemons and oranges their fresh aroma. You find it naturally in foods like oranges, lemons, and grapefruits, as well as in cleaning products and perfumes for its lemon-like smell.

Sources: s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Uses), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed on retry · 16 snippets · 17.5 s

> **The smell that switches itself off**  
> Limonene smells like fresh citrus, but at high concentrations it quickly deadens the sense of smell. This means the warning fades just when there is more of the gas around. The text notes that at elevated temperatures it cracks to form isoprene, yet the smell adaptation happens in the nose first.

Sources: s1 (Wikipedia, In plants), s2 (Wikipedia, Chemical reactions)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "The smell that switches itself off", "body": "Limonene smells like fresh citrus, but at high concentrations it quickly deadens your sense of smell, so the warning fades just when there is more of the gas around.", "sources": ["s1", "s2"]}
```
</details>


### lycopene

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 3.5 s

> **Lycopene has no official GHS hazards**  
> Lycopene is not classified under GHS hazard criteria, as 203 of 203 companies reported it meets no danger standards. No specific hazard statements are listed for this molecule.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 5.8 s

> **The red in your bowl**  
> Lycopene gives tomatoes, watermelon, and pink grapefruit their vibrant red hue. You might also spot it in baked beans and red guavas. While it has no vitamin A activity, it acts as a powerful antioxidant in nature.

Sources: s2 (PubChem, Record Description), s5 (PubChem, Record Description), s7 (Wikipedia, Occurrence)  
Fun: _ /5

**Weird fact** · passed · 19 snippets · 7.9 s

> **A molecule with 1,056 possible shapes**  
> Lycopene is a straight, flat molecule that gives red fruits their color. Yet, its structure allows for 1,056 different theoretical cis-trans configurations, though the all-trans form is the most common in food.

Sources: s11 (PubChem, Record Description)  
Fun: _ /5


### menthol

**Danger level** · passed · 1 snippets · 3.5 s

> **Menthol: Low official hazard rating**  
> Menthol carries a 'Warning' signal word and is classified as an irritant. However, it is not officially classified under GHS criteria, as one company reported it does not meet hazard standards.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 1 snippets · 4.3 s

> **Menthol's 1771 discovery**  
> In 1771, the German Hieronymus David Gaubius first isolated menthol. Later characterizations were performed by Oppenheim, Beckett, Moriya, and Atkinson, and the compound was named by F. L. Alphonse Oppenheim in 1861.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 18 snippets · 8.4 s

> **Cooling mint in your breath**  
> Menthol gives peppermint its signature taste and creates a cooling sensation when inhaled or applied to skin. You find it naturally in peppermint oil from plants like Mentha × piperita and corn mint, and it appears in toothpaste, cough drops, and many other products. Its local anesthetic properties make it popular for relieving minor throat irritation and as a flavoring agent.

Sources: s1 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s10 (Wikipedia, Occurrence), s13 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 25 snippets · 8.2 s

> **The smell that tricks your brain**  
> Menthol tricks your brain into feeling cold by activating heat sensors, just like capsaicin tricks it into feeling hot. This chemical illusion works without actually changing the temperature around you.

Sources: s4 (Wikipedia, Biological properties)  
Fun: _ /5


### morphine

**Danger level** · passed · 1 snippets · 3.8 s

> **Warning: Harmful if swallowed, irritates skin**  
> Morphine carries a Warning signal word and can be harmful if swallowed. It may cause allergic skin reactions, drowsiness, dizziness, and is suspected of damaging fertility or unborn children.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.0 s

> **From ancient tablets to Paderborn**  
> References to morphine date back to Sumerian clay tablets around 2100 BC and Theophrastus in the 3rd century BC. Friedrich Sertürner discovered it in December 1804 in Paderborn by extracting it from the opium poppy. He named it morphium after the Greek god Morpheus because it induced sleep.

Sources: s1 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

**Weird fact** · passed · 13 snippets · 7.3 s

> **The drug that nearly killed its discoverer**  
> Friedrich Sertürner tested morphine on himself and three boys in 1817, and all four almost died. He originally named it morphium after the Greek god of dreams because it induces sleep.

Sources: s3 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 18 snippets · 10.0 s

> **From poppy to pharmacy**  
> Morphine is the most abundant opiate found in the dried latex of the opium poppy (Papaver somniferum), making up 8–14% of its dry weight. First isolated in 1804, it is now a Schedule II controlled substance used primarily to treat severe acute and chronic pain. While it acts directly on the central nervous system to relieve pain, it carries a high potential for addiction and dependence.

Sources: s1 (PubChem, Record Description), s3 (PubChem, Record Description), s6 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s11 (Wikipedia, Natural occurrence), s13 (PubChem, Uses)  
Fun: _ /5


### nicotine

**Danger level** · passed · 1 snippets · 3.4 s

> **Nicotine is deadly if swallowed, touched, or breathed**  
> Nicotine carries a Danger rating and is fatal if swallowed, touches the skin, or is inhaled. It also poses a long-lasting threat to aquatic life.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.9 s

> **From tobacco to poison to cure**  
> Nicotine was first isolated from the tobacco plant in 1828 by Wilhelm Heinrich Posselt and Karl Ludwig Reimann from Germany. Its structure was later discovered by Adolf Pinner and Richard Wolffenstein in 1893, and it was synthesized by Amé Pictet and A. Rotschy in 1904.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed on retry · 17 snippets · 16.9 s

> **In tobacco, plants, and medicine**  
> Nicotine is found in tobacco plants like Nicotiana tabacum and N. rustica, where it makes up about 0.6–3% of dry weight. It also appears in tomatoes, potatoes, and eggplants, and is used medically to treat nicotine dependence. The molecule acts as a phytogenic insecticide and a neurotoxin in insects.

Sources: s3 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (Wikipedia, Uses), s11 (PubChem, Uses), s15 (PubChem, Natural Pollution Sources)  
Fun: _ /5

<details><summary>Attempt 1 rejected: name Nicotiana is not in the cited snippets</summary>

```json
{"title": "From tobacco leaves to medicine", "body": "Nicotine naturally lives in tobacco plants like Nicotiana tabacum, making up 0.6–3% of their dry weight. Humans encounter it in cigarettes, where an average puff delivers about 2 mg of absorbed nicotine. Medically, it helps quit smoking through replacement therapies like patches and gums.", "sources": ["s6", "s7", "s8", "s9"]}
```
</details>

**Weird fact** · passed · 15 snippets · 8.4 s

> **The poison that saves your sleep**  
> Nicotine is classified as an extremely hazardous poison, yet it can actually improve mood and sleep for depressed non-smokers. This paradoxical effect occurs because nicotine acts as a receptor agonist at most nicotinic acetylcholine receptors, which alters brain chemistry in ways that temporarily boost well-being before withdrawal hits.

Sources: s9 (Wikipedia, Adverse effects), s10 (Wikipedia, Adverse effects), s15 (Wikipedia, Pharmacology)  
Fun: _ /5


### nitroglycerin

**Danger level** · passed · 1 snippets · 4.6 s

> **Nitroglycerin: Explosive and fatal if exposed**  
> Nitroglycerin carries a Danger signal word and features explosive, acute toxic, health, and environmental hazard pictograms. It is officially classified as having a mass explosion hazard and is fatal if swallowed, touched, or inhaled. Prolonged exposure may damage organs, and it poses long-lasting risks to aquatic life.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.6 s

> **From pyroglycerin to blasting oil**  
> Ascanio Sobrero synthesized nitroglycerin in 1846 at the University of Turin, initially calling it pyroglycerin and warning against its explosive use. Alfred Nobel later adopted it commercially after a 1864 explosion in Heleneborg, Sweden, killed his brother and workers. His company eventually produced unstable Blasting Oil, leading to further catastrophes like a 1866 explosion in San Francisco that killed 15 people.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 16 snippets · 8.6 s

> **From heartache to dynamite**  
> You might find nitroglycerin in medicine to treat angina or in explosives like dynamite, which Nobel patented in 1867. It appears as a pale yellow, viscous liquid that is slightly soluble in water. While used medicinally, it can become shock sensitive if ethanol evaporates from certain solutions.

Sources: s2 (PubChem, Record Description), s10 (Wikipedia, Use as an explosive and a propellant), s11 (Wikipedia, Medical use), s13 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 14 snippets · 7.9 s

> **Frozen nitroglycerin is less explosive**  
> Liquid nitroglycerin explodes easily from shock, but freezing it between 7 and 13 °C makes it surprisingly less sensitive to impact. However, thawing it out can make it even more dangerous, especially if impurities are present.

Sources: s5 (Wikipedia, Instability and desensitization), s6 (Wikipedia, Instability and desensitization)  
Fun: _ /5


### quinine

**Danger level** · passed · 1 snippets · 3.7 s

> **Quinine risks: skin, eyes, and breathing**  
> Quinine is officially rated dangerous, posing risks of skin irritation, allergic reactions, and serious eye damage. Breathing it may trigger asthma symptoms or breathing difficulties, while swallowing it can be harmful.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 6.2 s

> **From Quechua bark to European medicine**  
> Quechua Indians in Peru, Bolivia, and Ecuador used cinchona bark mixed with sweetened water to stop shivering. Spanish Jesuit missionaries learned of this by the 1570s, and apothecary Agostino Salumbrino sent bark to Rome for testing. This unextracted bark became a valuable commodity for treating malaria in Europe.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History), s4 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed on retry · 16 snippets · 17.7 s

> **Bitter in your tonic, bark in the Andes**  
> Quinine is the active ingredient in tonic water and other beverages that give them a bitter taste. It is also found in the bark of the cinchona tree, which is native to Peru and other regions of South America. While once used for malaria, it is now primarily used for flavoring and treating specific muscle disorders.

Sources: s6 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s11 (PubChem, Uses), s14 (PubChem, Uses), s16 (PubChem, Natural Pollution Sources)  
Fun: _ /5

<details><summary>Attempt 1 rejected: number 1633 is not in the cited snippets; name Bolivia is not in the cited snippets</summary>

```json
{"title": "Bitter in your drink, bitter in history", "body": "Quinine comes from the bark of the cinchona tree, native to regions like Peru and Bolivia. You might taste it in tonic water or bitter lemon for that sharp, bitter flavor. Its medical history dates back to before 1633 when bark extracts were used to treat malaria.", "sources": ["s6", "s8", "s11", "s14", "s16"]}
```
</details>

**Weird fact** · passed · 12 snippets · 7.2 s

> **The bitter bark that glows blue**  
> Quinine comes from a bark so bitter Quechua Indians mixed with sweet water to make tonic-like drinks. Yet, in acid, this same molecule glows a bright blue-cyan hue under UV light.

Sources: s1 (Wikipedia, History), s12 (Wikipedia, Chemistry)  
Fun: _ /5


### serotonin

**Danger level** · passed on retry · 1 snippets · 6.6 s

> **Serotonin risks: poison and fertility**  
> Serotonin carries the signal word Danger and uses Acute Toxic and Health Hazard pictograms. It is toxic if swallowed, and there is a 98.5% chance it is suspected of damaging fertility or the unborn child.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

<details><summary>Attempt 1 rejected: body has 1 sentences, needs 2 to 4</summary>

```json
{"title": "Serotonin risks: poison and fertility", "body": "Serotonin is officially rated dangerous if swallowed, and it is suspected of damaging fertility or the unborn child.", "sources": ["s1"]}
```
</details>

**Origin story** · passed · 2 snippets · 5.5 s

> **From enteramine to serotonin**  
> In 1948, Maurice M. Rapport, Arda Green, and Irvine Page discovered a vasoconstrictor in blood serum and named it serotonin. Later, in 1952, it was confirmed that enteramine was the same substance, and the abbreviation 5-HT became preferred. In 1953, Betty Twarog and Page found serotonin in the central nervous system.

Sources: s1 (Wikipedia, History & etymology), s2 (Wikipedia, History & etymology)  
Fun: _ /5

**Where you meet it** · passed · 8 snippets · 5.7 s

> **Your gut's mood ring**  
> About 90% of your body's serotonin lives in the gut, where it regulates intestinal movements. The rest floats in your brain and blood platelets, helping with mood, clotting, and sleep. It is synthesized from the amino acid L-TRYPTOPHAN.

Sources: s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction)  
Fun: _ /5

**Weird fact** · passed · 14 snippets · 7.2 s

> **Seeds use serotonin to flush poison**  
> When seeds dry out, they produce serotonin specifically to trap and remove poisonous ammonia. This chemical trick converts the toxin into tryptamine and then serotonin, effectively cleaning the seed's internal chemistry.

Sources: s10 (Wikipedia, Comparative biology & evolution)  
Fun: _ /5


### strychnine

**Danger level** · passed · 1 snippets · 3.8 s

> **Strychnine is fatal if swallowed or touches skin**  
> Strychnine carries a Danger signal and is classified as acutely toxic if swallowed or in contact with skin. It also poses a severe threat to aquatic life, causing both immediate and long-lasting harm to water ecosystems.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 14.2 s

> **First discovery in Saint-Ignatius' bean**  
> French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier discovered strychnine in 1818 in the Saint-Ignatius' bean. They identified it in plants of the genus Strychnos, which includes species native to the Malabar Coast in Southern India, Sri Lanka, and Indonesia. The toxic effects of Strychnos nux-vomica were known from ancient India before the compound itself was characterized in the 19th century.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History), s3 (Wikipedia, History)  
Fun: _ /5

<details><summary>Attempt 1 rejected: number 19th is not in the cited snippets; name India is not in the cited snippets</summary>

```json
{"title": "From ancient beans to Nobel prizes", "body": "French chemists Joseph Bienaimé Caventou and Pierre-Joseph Pelletier discovered strychnine in 1818 in the Saint-Ignatius' bean. While its toxic effects were known since ancient India, the compound itself remained unidentified until the 19th century. Later, Sir Robert Robinson determined its structure in 1946, and Robert B. Woodward synthesized it in 1954, earning both Nobel prizes.", "sources": ["s3", "s4"]}
```
</details>

**Where you meet it** · passed · 11 snippets · 7.2 s

> **Hidden in seeds and baits**  
> You might find strychnine in the seeds of the Strychnos nux-vomica tree or as a rodenticide in colored grain baits for squirrels and gophers. It has also been used historically as a muscle stimulant and appears in some illicit drugs. While once a medicine, it is now mostly a pesticide or poison.

Sources: s3 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (PubChem, Uses), s10 (PubChem, Uses)  
Fun: _ /5

**Weird fact** · passed · 10 snippets · 7.2 s

> **The poison that tastes like a warning**  
> Strychnine is so intensely bitter that it activates specific human taste receptors, TAS2R10 and TAS2R46. This natural warning system makes the poison taste awful before it can even be swallowed.

Sources: s10 (Wikipedia, Pharmacokinetics)  
Fun: _ /5


### sucralose

**Danger level** · passed · 1 snippets · 3.6 s

> **Sucralose has no official GHS hazards**  
> Sucralose is not classified under GHS hazard criteria, as 309 of 309 companies reported it meets no specific danger thresholds. Consequently, there are no official pictograms or hazard statements assigned to it.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 4 snippets · 5.4 s

> **A sweet taste on his hands**  
> In 1975, scientists Leslie Hough and Shashikant Phadnis at Queen Elizabeth College mixed sulfuryl chloride with sugar. Phadnis tasted a chlorinated compound and found it exceptionally sweet. Tate & Lyle patented the substance in 1976.

Sources: s1 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 5.6 s

> **Sugar-free snacks, sweet and safe**  
> You find sucralose in candies, coffee pods, and soft drinks where it adds sweetness without calories. This artificial sweetener is about 600 times sweeter than table sugar and appears in products like Splenda.

Sources: s2 (PubChem, Record Description), s4 (Wikipedia, Introduction), s6 (Wikipedia, Uses), s7 (Wikipedia, Uses)  
Fun: _ /5

**Weird fact** · passed · 19 snippets · 8.3 s

> **The sweetener that survives baking but risks dioxins**  
> Sucralose stays sweet when baked, yet heating it above 120 °C can create dangerous chlorinated compounds like dioxins in unknown amounts. While it is generally safe for consumption, this hidden risk emerges only under specific high-heat conditions.

Sources: s8 (Wikipedia, Safety evaluation)  
Fun: _ /5


### tetrodotoxin

**Origin story** · no input · 0 snippets · -0.0 s

**Danger level** · passed · 1 snippets · 3.9 s

> **Tetrodotoxin: Fatal if swallowed, touched, or breathed**  
> Tetrodotoxin carries a Danger signal word and is classified as acutely toxic through swallowing, skin contact, or inhalation. Officially, it is fatal if swallowed, fatal in contact with skin, and fatal if inhaled.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Weird fact** · passed · 16 snippets · 8.4 s

> **The poison that saves cancer pain**  
> While tetrodotoxin is so deadly it kills mice with just 334 micrograms per kilogram, researchers are testing it to treat cancer pain. Early trials show it can significantly relieve pain in some patients, turning a lethal toxin into a potential medical tool.

Sources: s3 (Wikipedia, Poisoning), s5 (Wikipedia, Modern therapeutic research), s11 (Wikipedia, Introduction)  
Fun: _ /5

**Where you meet it** · passed · 15 snippets · 9.5 s

> **Hidden in pufferfish and newts**  
> You might find tetrodotoxin in the liver, ovaries, or skin of pufferfish, porcupinefish, and ocean sunfish, as well as in blue-ringed octopuses and rough-skinned newts. While it causes paralysis by blocking nerve signals, it is also being studied for treating cancer pain and opioid dependence.

Sources: s1 (PubChem, Record Description), s2 (PubChem, Record Description), s4 (PubChem, Record Description), s5 (PubChem, Record Description), s6 (Wikipedia, Introduction), s9 (PubChem, Uses), s12 (PubChem, Uses), s13 (PubChem, Natural Pollution Sources)  
Fun: _ /5


### thalidomide

**Danger level** · passed on retry · 1 snippets · 7.0 s

> **Thalidomide: Harmful if swallowed, damages fertility**  
> Thalidomide carries the signal word Danger and shows Irritant and Health Hazard pictograms. It is harmful if swallowed, and may damage fertility or the unborn child.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

<details><summary>Attempt 1 rejected: title has 10 words, at most 8 allowed</summary>

```json
{"title": "Thalidomide: Harmful if swallowed, risks to fertility and unborn children", "body": "Thalidomide carries a Danger signal word and is officially classified as harmful if swallowed. It may also damage fertility or harm an unborn child.", "sources": ["s1"]}
```
</details>

**Origin story** · passed · 4 snippets · 5.9 s

> **From soap to wonder drug**  
> Chemical Industry Basel synthesized thalidomide in 1952, but discarded it for having no effect on animals. In 1957, Chemie Grünenthal acquired it after Heinrich Mückter's team identified a by-product isolated by Wilhelm Kunz. The drug was marketed as a 'wonder drug' for insomnia and morning sickness in October 1957.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, History)  
Fun: _ /5

**Where you meet it** · passed · 12 snippets · 7.5 s

> **From sleep aid to life-saving drug**  
> First sold in 1957 as a sleep aid, thalidomide was withdrawn after causing severe birth defects in over 10,000 infants. Today, it is used to treat multiple myeloma, leprosy complications, and certain cancers. Strict safety programs ensure it is only given to non-pregnant patients.

Sources: s4 (PubChem, Record Description), s6 (Wikipedia, Introduction), s7 (Wikipedia, Introduction), s8 (Wikipedia, Introduction), s9 (Wikipedia, Medical uses)  
Fun: _ /5

**Weird fact** · passed · 14 snippets · 8.0 s

> **The 28-day window of doom**  
> Thalidomide's deformities depended on exactly how many days into pregnancy the mother took it. Taking it on day 20 caused brain damage, day 24 caused arm loss, and leg damage occurred if taken up to day 28.

Sources: s14 (Wikipedia, Society and culture)  
Fun: _ /5


### theobromine

**Danger level** · passed · 1 snippets · 3.0 s

> **Warning: Harmful if swallowed or irritating**  
> Theobromine carries a Warning signal word and may cause serious eye irritation. It is harmful if swallowed and suspected of causing cancer.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed · 3 snippets · 4.6 s

> **Food of the gods found in cacao**  
> A. Woskresensky discovered theobromine in cacao beans in 1841. Later, Hermann Emil Fischer reported synthesizing it from xanthine in 1882. Its name comes from Greek roots meaning 'food of the gods'.

Sources: s1 (Wikipedia, History), s2 (Wikipedia, Etymology), s3 (Wikipedia, Etymology)  
Fun: _ /5

**Where you meet it** · passed · 11 snippets · 6.9 s

> **Chocolate's secret heart stimulant**  
> Theobromine is the principal alkaloid found in cacao beans, giving chocolate its unique flavor and mild heart-stimulating effects. You also find it in tea, kola nuts, and some hollies, though it has practically no stimulant effect on your central nervous system.

Sources: s2 (PubChem, Record Description), s6 (Wikipedia, Introduction), s11 (PubChem, Natural Pollution Sources)  
Fun: _ /5

**Weird fact** · passed · 20 snippets · 8.4 s

> **The name that hides a stench**  
> Theobromine's name comes from Greek roots meaning 'food of the gods,' yet it contains absolutely no bromine. The element bromine is named after 'stench,' but this compound has no odor at all.

Sources: s3 (Wikipedia, Etymology), s4 (Wikipedia, Etymology)  
Fun: _ /5


### vanillin

**Danger level** · passed · 1 snippets · 3.1 s

> **Warning for eyes, water, and stomach**  
> Vanillin carries a Warning signal and is an irritant with environmental hazards. It is harmful if swallowed and poses long-lasting toxicity risks to aquatic life.

Sources: s1 (PubChem, GHS Classification)  
Fun: _ /5

**Origin story** · passed on retry · 4 snippets · 13.6 s

> **Vanillin's ancient roots and modern discovery**  
> Native Mesoamerican peoples, especially the Totonacs of Veracruz, Mexico, cultivated vanilla beans called tlilxochitl for beverages since at least the early 15th century. Théodore Nicolas Gobley isolated vanillin as a pure substance in 1858 by evaporating vanilla extract and recrystallizing it from hot water. Later, Ferdinand Tiemann and Wilhelm Haarmann deduced its structure in 1874 and began industrial production in Holzminden, Germany.

Sources: s2 (Wikipedia, Natural history), s3 (Wikipedia, Synthetic history)  
Fun: _ /5

<details><summary>Attempt 1 rejected: s2 is not in the cited snippets; s3 is not in the cited snippets</summary>

```json
{"title": "From Mesoamerica to Holzminden", "body": "Native Mesoamerican peoples like the Totonacs first cultivated vanilla beans in Veracruz, Mexico, while Aztecs flavored their xocohotl drinks with it since the early 15th century. Théodore Nicolas Gobley isolated pure vanillin in 1858 by evaporating extract and recrystallizing it. Later, Ferdinand Tiemann and Wilhelm Haarmann deduced its structure in 1874 and started industrial production in Holzminden, Germany. [s2][s3]", "sources": ["s2", "s3"]}
```
</details>

**Where you meet it** · passed on retry · 19 snippets · 16.8 s

> **From chocolate to moth traps**  
> Vanillin is the primary flavor in vanilla beans, which contain about 2% by dry weight of the compound. It appears in foods like ice cream and chocolate, which together make up 75% of the market for vanillin as a flavoring. Beyond sweets, it is used in perfumes, medicines, and even to control clothes moths and fleas.

Sources: s6 (Wikipedia, Introduction), s11 (Wikipedia, Occurrence), s13 (Wikipedia, Uses), s14 (Wikipedia, Uses), s15 (PubChem, Uses)  
Fun: _ /5

<details><summary>Attempt 1 rejected: number 3000 is not in the cited snippets</summary>

```json
{"title": "From ancient jars to your ice cream", "body": "Vanillin was likely used in Canaan over 3,000 years ago and became a chocolate flavor for the Aztecs. Today, it appears in 75% of the market for ice cream and chocolate, often as a synthetic version of the bean's natural scent.", "sources": ["s9", "s10", "s13"]}
```
</details>

**Weird fact** · passed · 21 snippets · 7.5 s

> **A tree signal for beetles**  
> The Dutch elm disease beetle uses vanillin as a chemical signal to find host trees. This surprising detail reveals that a common flavor compound also guides insects to their next meal.

Sources: s11 (Wikipedia, Ecology)  
Fun: _ /5

