# Frequently Asked Questions (FAQ)

## Did you Literally Build AGI?
The QBN as I am presented it is trained on **artificial data**.

It will be AGI when the QBN is trained on **real web-scale data**.

Right now, the QBN only "thinks about" very simple worlds that I encoded by hand.
But, if we assume that the LLM has "world knowledge", then the only problem to get full AGI is to transfer the knowledge from the LLM to the QBN.

That, I claim would be full AGI. Right now, I repeat, the QBN is trained on "toy universes" that I made up programmatically.

## Is it Trivial to Transfer Knowledge from LLM to QBN?
No. This is not trivial. It will require that the LLM model be re-written to generate a **tree-structured** analysis of a sentence, mapping the **surface form** of the sentence to its **logical form**.

This **logical form** is **latent**--meaning we can't observe it, and neither can actual people (this is why misunderstandings arise).

So, the following new abilities need to be developed before "full AGI" exists:
1. parse to logical forms, which are:
    a. latent (not observed)
    b. structured (recursively tree-structured)
2. concretize the continuous knowledge of the LLM into the discrete knowledge of the QBN

## Does the QBN Help us Understand the LLM?
Yes, I believe so. The QBN uses "semantic roles", which might explain why the "key-value" nature of the attention mechanism can learn world knowledge:
that is, the **key-value** knowledge of the LLM is actually learning the **semantic role** knowledge of linguistics.
