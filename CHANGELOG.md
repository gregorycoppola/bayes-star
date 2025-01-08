# CHANGELOG

## implications0108
Render the "implications".

Make there "libraries" so that each part can be made from smaller parts (compositionality).


## itdomain0107
Figure out how to:
* print out all of the information for each "entity" in a domain

This involves:
* make sure we register a domain
* figure out how to keep a list of all entities per domain
* actually make a binary to iterate through all the entities
* run each of the tests like this

## namespaceit0107
This PR is about seting up all of the objects in the ontology or universe to live in:
* consistent namespaces -- so that all of the namespaces can run together
* iterating over objects -- define and make iterable all the things we surface to the user

### Namespaces
Want each experiment to exist in its on persistent "namespace".

Maybe this can work by prepending the "experiment" name in front.

Then, we have a list of "experiment names".. that gets you all of these other lists.

### Iteration
We have to be able to iterate over:

* DOMAIN Names
* VERBs
* entities in domains
* specific propositions "that we know"


## cleanup0107
* just clean up the code
* run through all the scenarios and see if they all still work
