# Claim check: Qwen3-4B-Instruct-2507-Q4_K_M, verdict style

239 model-written cards from the hand-reviewed run in `docs/spike/runs/m0.6b`. A card is flagged when any claim (title or sentence) is judged unsupported, or the reply doesn't parse.

## Flag rate by review label

| Writer | Review label | Cards | Flagged |
| --- | --- | --- | --- |
| Qwen3-4B-Instruct-2507-Q4_K_M | minor | 31 | 17 (55%) |
| Qwen3-4B-Instruct-2507-Q4_K_M | ok | 32 | 10 (31%) |
| Qwen3-4B-Instruct-2507-Q4_K_M | unsupported | 17 | 13 (76%) |
| Qwen3.5-4B-Q4_K_M | minor | 23 | 13 (57%) |
| Qwen3.5-4B-Q4_K_M | ok | 37 | 11 (30%) |
| Qwen3.5-4B-Q4_K_M | unsupported | 20 | 20 (100%) |
| all | minor | 83 | 42 (51%) |
| all | ok | 101 | 34 (34%) |
| all | unsupported | 55 | 50 (91%) |
| microsoft_Phi-4-mini-instruct-Q4_K_M | minor | 29 | 12 (41%) |
| microsoft_Phi-4-mini-instruct-Q4_K_M | ok | 32 | 13 (41%) |
| microsoft_Phi-4-mini-instruct-Q4_K_M | unsupported | 18 | 17 (94%) |

If flagged cards were dropped: 113 of 239 cards kept; unsupported among shown cards 23% → 4%.

## Cost

| Measure | Value |
| --- | --- |
| Check time per card | 1.6 s median, 2.2 s p90 |
| Unparseable replies | 0 |
| Model load | 6.0 s |
| Peak memory | 3.62 GB |

## Misses: unsupported cards that passed

- limonene weird (Qwen3-4B-Instruct-2507-Q4_K_M): Title 'Limonene's metabolites fight cancer' states as fact what s1 says is 'suggested'
- nicotine origin (Qwen3-4B-Instruct-2507-Q4_K_M): 'They identified its empirical formula in 1843 by Melsens' - wrongly credits Posselt and Reimann; s1 credits Melsens
- strychnine origin (Qwen3-4B-Instruct-2507-Q4_K_M): 'used in Europe as early as 1640 to kill animals and humans' - s3 says dogs, cats and birds; also implies compound long known
- thalidomide origin (Qwen3-4B-Instruct-2507-Q4_K_M): Title 'From discarded by-product to a wonder drug' - 'wonder drug' not in snippets; conflates discarded compound with Kunz by-product
- buckminsterfullerene where (microsoft_Phi-4-mini-instruct-Q4_K_M): 'shaped like a soccer ball' not in snippets; shungite claim is about 'fullerene' generally

## False alarms: cards reviewed ok that were flagged

- aspirin weird (Qwen3-4B-Instruct-2507-Q4_K_M): "NIOSH and OSHA both set a limit of 5 mg/m3, though OSHA's rule was later vacated in 1993."
- batrachotoxin weird (Qwen3-4B-Instruct-2507-Q4_K_M): "Batrachotoxin is the most potent non-peptidal neurotoxin known."; "Its extreme toxicity makes it one of the most surprising natural compounds."
- dopamine origin (Qwen3-4B-Instruct-2507-Q4_K_M): "The neurotransmitter function of dopamine was first recognized in 1958 by Arvid Carlsson and Nils-Åke Hillarp at the Laboratory for Chemical Pharmacology of the National Heart Institute of Sweden."
- ethylene origin (Qwen3-4B-Instruct-2507-Q4_K_M): "Johann Joachim Becher obtained ethylene by heating ethanol with sulfuric acid in 1669 and mentioned it in his Physica Subterranea."
- indigo origin (Qwen3-4B-Instruct-2507-Q4_K_M): "The oldest known indigo-dyed fabric dates to 6,000 years ago and was found in Huaca Prieta, Peru."
- menthol origin (Qwen3-4B-Instruct-2507-Q4_K_M): "The molecule was later named by F."; "L."
- serotonin where (Qwen3-4B-Instruct-2507-Q4_K_M): "A smaller amount is in the brainstem and skin, while 8% is stored in blood platelets, which release it during clotting."
- strychnine where (Qwen3-4B-Instruct-2507-Q4_K_M): "When inhaled, swallowed, or absorbed through the eyes or mouth, it causes muscular convulsions and death."
- theobromine origin (Qwen3-4B-Instruct-2507-Q4_K_M): "In 1841, chemist A."
- vanillin where (Qwen3-4B-Instruct-2507-Q4_K_M): "It appears in ice cream and chocolate, which together make up 75% of vanillin's market."
- allicin where (Qwen3.5-4B-Q4_K_M): "The punchy bite in your garlic"
- ascorbic acid where (Qwen3.5-4B-Q4_K_M): "Citrus power in your diet"
- aspirin origin (Qwen3.5-4B-Q4_K_M): "The word Aspirin was Bayer's brand name, which was sold globally by 1899."
- batrachotoxin origin (Qwen3.5-4B-Q4_K_M): "In 1963, Fritz Märki and Bernhard Witkop isolated batrachotoxin from the black-legged poison dart frog at the National Institute of Arthritis and Musculoskeletal and Skin Diseases in Bethesda, Maryland."
- buckminsterfullerene where (Qwen3.5-4B-Q4_K_M): "Buckminsterfullerene is the most common naturally occurring fullerene found in small quantities of soot."
- carvone origin (Qwen3.5-4B-Q4_K_M): "It was originally named carvol by Schweizer before Goldschmidt and Zürrer identified it as a ketone related to limonene."
- dopamine weird (Qwen3.5-4B-Q4_K_M): "Dopamine's ancient bacterial roots"
- lycopene where (Qwen3.5-4B-Q4_K_M): "The red in your bowl"
- morphine weird (Qwen3.5-4B-Q4_K_M): "Friedrich Sertürner named morphine after the Greek god of dreams because it induces sleep."; "Yet, in his own experiments, he and three boys almost died after taking it."
- serotonin origin (Qwen3.5-4B-Q4_K_M): "In 1948, Maurice M."
- theobromine origin (Qwen3.5-4B-Q4_K_M): "A."
- aspirin origin (microsoft_Phi-4-mini-instruct-Q4_K_M): "In 1897, scientists at Bayer identified a new way to synthesize it and produced acetylsalicylic acid in a pure, stable form."
- aspirin weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "Aspirin, known for its pain-relieving powers, has a secret: it can form more than one crystal structure."; "In 2005, scientists confirmed this mystery, revealing form II, a polymorph that had been debated since the 1960s and even dismissed as an impurity in 1981."
- batrachotoxin where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Poison darts and frogs"
- benzylpenicillin where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Benzylpenicillin, also known as PenG, is a narrow-spectrum antibiotic used to treat serious bacterial infections like pneumonia, meningitis, gonorrhea, and syphilis."
- buckminsterfullerene weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "The giant molecule with wave-particle duality"
- carvone where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Carving flavors and scents"
- cinnamaldehyde weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "It's like a spicy bug killer and a spicy mouthful rolled into one!"
- geosmin weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "This means you can smell it at levels so low they're almost undetectable!"
- menthol where (microsoft_Phi-4-mini-instruct-Q4_K_M): "Peppermint's chill"
- nicotine origin (microsoft_Phi-4-mini-instruct-Q4_K_M): "The tobacco plant, Nicotiana tabacum, was named after French ambassador Jean Nicot de Villemain, who promoted its medicinal use in 1560."
- quinine weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "Quinine's double-edged sword"; "It's a dangerous dance between cure and poison."
- thalidomide origin (microsoft_Phi-4-mini-instruct-Q4_K_M): "The tragedy affected thousands of children worldwide."
- theobromine weird (microsoft_Phi-4-mini-instruct-Q4_K_M): "Theobromine's double life"
