This software package introduces the **Quantified Bayesian Network** (**QBN**).
The QBN generalizes:
1. Traditional (generative) Bayesian Networks
    - **Bayesian Networks** are graphical models representing probabilistic relationships among variables. They use directed acyclic graphs to encode joint probability distributions, allowing for efficient reasoning and inference in complex systems. Bayesian Networks are widely used in various fields like machine learning, data analysis, and artificial intelligence for tasks like prediction, anomaly detection, and decision making.
    - Learn more:
        - [Bayesian Networks and their Applications](https://www.sciencedirect.com/topics/computer-science/bayesian-network)
2. First-Order Logic
    - **First-Order Logic** (FOL), also known as predicate logic or first-order predicate calculus, is a collection of formal systems used in mathematics, philosophy, linguistics, and computer science. It provides a framework for expressing statements with quantifiers and variables, allowing for the formulation of hypotheses about objects and their relationships. FOL is fundamental in formal systems, theorem proving, and is foundational in artificial intelligence for knowledge representation and reasoning.
    - Learn more:
        - [First-Order Logic: Basics](https://plato.stanford.edu/entries/logic-classical/)
        - [Understanding First-Order Logic](https://www.britannica.com/topic/formal-logic/Higher-order-and-modal-logic)

## How Does the QBN Avoid Hallucinations?
The QBN avoids hallucinations by:
1. using logic
2. understanding how to explain its argument
3. understands that there are things it does not know

How does it do this?
1. using logic -- the QBN generalizes (though in a complex way) first-order logic
2. using ideas from classical Bayesian Networks -- allows us to create a generative story based on "causality"