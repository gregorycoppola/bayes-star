struct GraphicalModel {
    graph: Graph,
    model: Box<dyn FactorModel>,
}

struct Factor {
    conjunctions: Vec<Conjunction>,
    conclusion: Proposition,
}

trait FactorModel {
    pub fn score_factor(factor: &Factor) -> Result<PredictStatistics, Box<dyn Error>>;
}
