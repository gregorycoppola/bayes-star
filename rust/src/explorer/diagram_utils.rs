use crate::{
    inference::{graph::PropositionFactor, inference::MarginalTable},
    model::objects::{
        Argument, ImplicationFactor, Predicate, PredicateGroup, Proposition, PropositionGroup,
        Relation,
    },
};

fn diagram_domain(domain: &str) -> String {
    format!(
        r#"
                <span class='domain_span'>
                    <span class='domain_label'>{domain}</span>
                    <span><img src='/static/images/domains/{domain}.png' class='domain_icon' /></span>
                </span>
    "#
    )
}

fn diagram_argument(arg: &Argument) -> String {
    match arg {
        Argument::Constant(const_arg) => {
            format!(
                "<div>Constant Argument: <br>Domain: {}<br>Entity ID: {}</div>",
                const_arg.domain, const_arg.entity_id
            )
        }
        Argument::Variable(var_arg) => diagram_domain(&var_arg.domain),
    }
}

fn diagram_relation(relation: &Relation) -> String {
    let mut argument_part = "".to_string();
    for argument in &relation.types {
        argument_part += &format!(
            "<span class='argument_part'>{domain}</span>",
            domain = &argument.domain
        );
    }
    format!(
        r#"
        <span class='relation'>
            <span class='relation_name'>
                {relation_name}
            </span>
            {argument_part}
        </span>
    "#,
        relation_name = &relation.relation_name
    )
}

pub fn diagram_proposition(
    proposition: &Proposition,
    marginal_table: Option<&MarginalTable>,
) -> String {
    let score_part = match marginal_table {
        Some(table) => {
            let marginal = table.get_marginal(proposition).unwrap();

            // Calculate the color based on the marginal value
            let color = if marginal < 0.5 {
                // Gradient from red to yellow (0.0 to 0.5)
                format!(
                    "rgb({}, {}, 0)",
                    (255 * (1.0 - marginal * 2.0)) as u8,
                    (255 * marginal * 2.0) as u8
                )
            } else {
                // Gradient from yellow to green (0.5 to 1.0)
                format!(
                    "rgb(0, {}, {})",
                    (255 * (marginal - 0.5) * 2.0) as u8,
                    (255 * (1.0 - (marginal - 0.5) * 2.0)) as u8
                )
            };

            // Apply the color to the span
            format!(
                "<span class='marginal' style='background-color: {};'>{}</span>",
                color, marginal
            )
        }
        None => "".to_string(),
    };
    format!(
        r#"
        <span class='relation'>
            <span class='relation_name'>
                {predicate_part}
            </span>
            {score_part}
        </span>
    "#,
        predicate_part = &diagram_predicate(&proposition.predicate),
    )
}

pub fn diagram_predicate(predicate: &Predicate) -> String {
    let mut argument_buffer = "".to_string();
    for argument in &predicate.roles {
        let argument_part = diagram_argument(&argument.argument);
        argument_buffer += &format!(
            "<span class='role_name'>{role_name}</span>{argument_part}",
            role_name = &argument.role_name
        );
    }
    format!(
        r#"
        <span class='relation'>
            <span class='relation_name'>
                {relation_name}
            </span>
            {argument_buffer}
        </span>
    "#,
        relation_name = &predicate.relation.relation_name
    )
}

fn diagram_predicate_group(group: &PredicateGroup) -> String {
    let mut parts = vec![];
    for predicate in &group.terms {
        parts.push(diagram_predicate(predicate));
    }
    let separator = "<span class='and_separator'>&and;</span>"; // Customize as needed
    let joined_parts = parts.join(separator);
    format!("<div class='predicate_group'>{}</div>", joined_parts)
}

pub fn diagram_implication(relation: &ImplicationFactor) -> String {
    format!(
        r#"
        <div class='implication_box'>
            <div class='implication_row'>
                {predicate_group_part}
            </div>
            <div class='implication_divider'>
                &or;
            </div>
            <div class='implication_row'>
                {conclusion_part}
            </div>
        </div>
    "#,
        predicate_group_part = diagram_predicate_group(&relation.premise),
        conclusion_part = diagram_predicate(&relation.conclusion),
    )
}

pub fn diagram_proposition_factor(
    relation: &PropositionFactor,
    marginal_table: Option<&MarginalTable>,
) -> String {
    format!(
        r#"
        <div class='implication_box'>
            <div class='implication_row'>
                {predicate_group_part}
            </div>
            <div class='implication_divider'>
                &or;
            </div>
            <div class='implication_row'>
                {conclusion_part}
            </div>
        </div>
    "#,
        predicate_group_part = diagram_proposition_group(&relation.premise),
        conclusion_part = diagram_proposition(&relation.conclusion, marginal_table),
    )
}

pub fn diagram_proposition_group(group: &PropositionGroup) -> String {
    let mut parts = vec![];
    for predicate in &group.terms {
        parts.push(diagram_predicate(&predicate.predicate));
    }
    let separator = "<span class='and_separator'>&and;</span>"; // Customize as needed
    let joined_parts = parts.join(separator);
    format!("<div class='predicate_group'>{}</div>", joined_parts)
}

// pub fn diagram_proposition_group(proposition_group: &PropositionGroup) -> String {
//     let parts: Vec<String> = proposition_group
//         .terms
//         .iter()
//         .map(|f| "".to_string())
//         .collect();
//     format!(r#"
//         <div class='proposition_group'>
//             {proposition_group_part}
//         </div>
//     "#,
//         proposition_group_part = parts.join(""),
//     )
// }
