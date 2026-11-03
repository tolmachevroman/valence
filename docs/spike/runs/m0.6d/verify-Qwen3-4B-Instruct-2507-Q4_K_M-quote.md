# Claim check: Qwen3-4B-Instruct-2507-Q4_K_M, quote style

239 model-written cards from the hand-reviewed run in `docs/spike/runs/m0.6b`. A card is flagged when any claim (title or sentence) is judged unsupported, or the reply doesn't parse.

## Flag rate by review label

| Writer | Review label | Cards | Flagged |
| --- | --- | --- | --- |
| Qwen3-4B-Instruct-2507-Q4_K_M | minor | 31 | 26 (84%) |
| Qwen3-4B-Instruct-2507-Q4_K_M | ok | 32 | 17 (53%) |
| Qwen3-4B-Instruct-2507-Q4_K_M | unsupported | 17 | 15 (88%) |
| Qwen3.5-4B-Q4_K_M | minor | 23 | 19 (83%) |
| Qwen3.5-4B-Q4_K_M | ok | 37 | 20 (54%) |
| Qwen3.5-4B-Q4_K_M | unsupported | 20 | 20 (100%) |
| all | minor | 83 | 66 (80%) |
| all | ok | 101 | 60 (59%) |
| all | unsupported | 55 | 52 (95%) |
| microsoft_Phi-4-mini-instruct-Q4_K_M | minor | 29 | 21 (72%) |
| microsoft_Phi-4-mini-instruct-Q4_K_M | ok | 32 | 23 (72%) |
| microsoft_Phi-4-mini-instruct-Q4_K_M | unsupported | 18 | 17 (94%) |

If flagged cards were dropped: 61 of 239 cards kept; unsupported among shown cards 23% → 5%.

## Cost

| Measure | Value |
| --- | --- |
| Check time per card | 5.2 s median, 7.2 s p90 |
| Unparseable replies | 0 |
| Model load | 7.0 s |
| Peak memory | 3.71 GB |

## Misses: unsupported cards that passed

- artemisinin where (Qwen3-4B-Instruct-2507-Q4_K_M): 'used in traditional medicine for malaria, cancer, viral...' merges s1 TCM herb with s3 review's list of treated conditions
- strychnine origin (Qwen3-4B-Instruct-2507-Q4_K_M): 'used in Europe as early as 1640 to kill animals and humans' - s3 says dogs, cats and birds; also implies compound long known
- serotonin weird (microsoft_Phi-4-mini-instruct-Q4_K_M): 'serves as a toxin for algae': snippet says SSRIs are toxic to algae

## False alarms: cards reviewed ok that were flagged

- aspirin where (Qwen3-4B-Instruct-2507-Q4_K_M): "In your medicine cabinet and beyond"
- batrachotoxin weird (Qwen3-4B-Instruct-2507-Q4_K_M): "Batrachotoxin is the most potent non-peptidal neurotoxin known."; "Its extreme toxicity makes it one of the most surprising natural compounds."
- benzylpenicillin where (Qwen3-4B-Instruct-2507-Q4_K_M): "In injections for infections"
- capsaicin where (Qwen3-4B-Instruct-2507-Q4_K_M): "The heat in your chili"
- ethylene origin (Qwen3-4B-Instruct-2507-Q4_K_M): "Johann Joachim Becher obtained ethylene by heating ethanol with sulfuric acid in 1669 and mentioned it in his Physica Subterranea."; "Joseph Priestley reported in 1779 that Jan Ingenhousz saw ethylene synthesized in the same way by a Mr."; "Enée in Amsterdam in 1777."
- geosmin where (Qwen3-4B-Instruct-2507-Q4_K_M): "You meet geosmin when it rains after dry weather, creating the scent known as petrichor."
- limonene where (Qwen3-4B-Instruct-2507-Q4_K_M): "The zesty scent in citrus peels"; "Limonene is the main fragrance in citrus fruit peels, especially lemons and oranges."; "It appears in food manufacturing as a flavoring agent and in medicines to mask bitter tastes."
- menthol origin (Qwen3-4B-Instruct-2507-Q4_K_M): "The molecule was later named by F."; "L."
- morphine origin (Qwen3-4B-Instruct-2507-Q4_K_M): "From opium to morphine in 1804"
- morphine where (Qwen3-4B-Instruct-2507-Q4_K_M): "In poppies and pain meds"
- nicotine where (Qwen3-4B-Instruct-2507-Q4_K_M): "In tobacco and your daily habits"
- serotonin where (Qwen3-4B-Instruct-2507-Q4_K_M): "A smaller amount is in the brainstem and skin, while 8% is stored in blood platelets, which release it during clotting."
- strychnine where (Qwen3-4B-Instruct-2507-Q4_K_M): "In seeds of the Strychnos tree"
- tetrodotoxin where (Qwen3-4B-Instruct-2507-Q4_K_M): "In pufferfish and beyond"
- theobromine origin (Qwen3-4B-Instruct-2507-Q4_K_M): "In 1841, chemist A."
- vanillin where (Qwen3-4B-Instruct-2507-Q4_K_M): "Scent of chocolate and ice cream"; "It appears in ice cream and chocolate, which together make up 75% of vanillin's market."
- vanillin weird (Qwen3-4B-Instruct-2507-Q4_K_M): "This implies an unidentified Vanilla species could have been grown in the Old World since the Middle Bronze Age."
- allicin where (Qwen3.5-4B-Q4_K_M): "The punchy bite in your garlic"
- artemisinin origin (Qwen3.5-4B-Q4_K_M): "From ancient tea to modern cure"
- ascorbic acid where (Qwen3.5-4B-Q4_K_M): "Citrus power in your diet"
- aspirin origin (Qwen3.5-4B-Q4_K_M): "From willow to Bayer's brand"; "The word Aspirin was Bayer's brand name, which was sold globally by 1899."
- aspirin where (Qwen3.5-4B-Q4_K_M): "Pain relief in your medicine cabinet"
- benzylpenicillin where (Qwen3.5-4B-Q4_K_M): "A vital shot for serious bugs"
- buckminsterfullerene where (Qwen3.5-4B-Q4_K_M): "Buckminsterfullerene is the most common naturally occurring fullerene found in small quantities of soot."
- caffeine where (Qwen3.5-4B-Q4_K_M): "The buzz in your cup"
- capsaicin where (Qwen3.5-4B-Q4_K_M): "Spice and pain relief"
- carvone origin (Qwen3.5-4B-Q4_K_M): "It was originally named carvol by Schweizer before Goldschmidt and Zürrer identified it as a ketone related to limonene."
- carvone weird (Qwen3.5-4B-Q4_K_M): "Carvone exists as two mirror-image forms that smell completely different."
- cubane where (Qwen3.5-4B-Q4_K_M): "Tiny cube, big energy"
- indigo where (Qwen3.5-4B-Q4_K_M): "The blue in your jeans"
- lycopene where (Qwen3.5-4B-Q4_K_M): "The red in your bowl"
- lycopene weird (Qwen3.5-4B-Q4_K_M): "A molecule with 1,056 possible shapes"
- menthol weird (Qwen3.5-4B-Q4_K_M): "Menthol tricks your skin into feeling cold by triggering TRPM8 receptors, just as capsaicin tricks you into feeling heat by stimulating heat sensors."
- nicotine weird (Qwen3.5-4B-Q4_K_M): "Nicotine's hidden danger"
- quinine weird (Qwen3.5-4B-Q4_K_M): "Quinine's glowing blue secret"
- thalidomide origin (Qwen3.5-4B-Q4_K_M): "From soap to wonder drug"
- theobromine origin (Qwen3.5-4B-Q4_K_M): "Food of the gods found in cacao"; "A."
- allicin where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Garlic's secret weapon"
- aspirin origin (microsoft_Phi-4-mini-instruct-Q4_K_M): "From ancient remedies to modern medicine"; "In 1897, scientists at Bayer identified a new way to synthesize it and produced acetylsalicylic acid in a pure, stable form."
- aspirin weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "Aspirin's crystal conundrum"; "Aspirin, known for its pain-relieving powers, has a secret: it can form more than one crystal structure."; "In 2005, scientists confirmed this mystery, revealing form II, a polymorph that had been debated since the 1960s and even dismissed as an impurity in 1981."
- batrachotoxin origin (microsoft_Phi-4-mini-instruct-Q4_K_M): "Discovery of a Potent Toxin"
- batrachotoxin where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Poison darts and frogs"
- buckminsterfullerene weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "The giant molecule with wave-particle duality"
- caffeine where (microsoft_Phi-4-mini-instruct-Q4_K_M): "The buzz in your morning brew"
- capsaicin where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Spicy adventures with capsaicin"
- carvone origin (microsoft_Phi-4-mini-instruct-Q4_K_M): "It was identified as a ketone related to limonene by Goldschmidt and Zürrer, and its structure was finally elucidated by Georg Wagner in 1894."
- carvone where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Carving flavors and scents"
- cinnamaldehyde weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "Cinnamaldehyde: A double agent"; "It's like a spicy bug killer and a spicy mouthful rolled into one!"
- ethylene weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "Oracle's trance from gas"
- geosmin weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "This means you can smell it at levels so low they're almost undetectable!"
- menthol where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Peppermint's chill"
- morphine where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Morphine: Nature's Painkiller"
- nicotine origin (microsoft_Phi-4-mini-instruct-Q4_K_M): "From poison to pleasure"
- quinine weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "Quinine's double-edged sword"; "It's a dangerous dance between cure and poison."
- tetrodotoxin where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Poison in the sea"
- thalidomide origin (microsoft_Phi-4-mini-instruct-Q4_K_M): "A wonder drug's dark side"; "The tragedy affected thousands of children worldwide."
- theobromine origin (microsoft_Phi-4-mini-instruct-Q4_K_M): "Godly Food Discovery"
- theobromine where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Chocolate's secret ingredient"
- theobromine weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "Theobromine's double life"
- vanillin where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Vanilla's sweet journey"
