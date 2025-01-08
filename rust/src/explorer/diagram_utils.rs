use crate::model::objects::{ImplicationFactor, Predicate, PredicateGroup, Relation};

fn diagram_domain(domain: &String) -> String {
    format!(r#"
                <span class='domain_span'>
                    <span class='domain_label'>{domain}</span>
                    <span><img src='/static/images/domains/{domain}.png' class='domain_icon'></img></span>
                </span>
    "#)
}

fn diagram_relation(relation: &Relation) -> String {
    let types_html = relation.types.iter()
        .map(|var_arg| format!("<span class='type'>{}</span>", var_arg)) // Assuming var_arg can be displayed
        .collect::<Vec<_>>()
        .join(", ");
    format!("<div class='relation'><strong>{}</strong>: [{}]</div>", relation.relation_name, types_html)
}

fn diagram_predicate(predicate: &Predicate) -> String {
    "".to_string()
}

fn diagram_predicate_group(group: &PredicateGroup) -> String {
    let mut parts = vec![];
    for predicate in &group.terms {
        parts.push(diagram_predicate(predicate));
    }
    let separator = "<span class='separator'>, </span>"; // Customize as needed
    let joined_parts = parts.join(separator);
    format!("<div class='predicate_group'>{}</div>", joined_parts)
}

fn diagram_implication(relation: &ImplicationFactor) -> String {
    format!(
        r#"
        <div class='implication_box'>
            <div class='implication_row'>
                {predicate_group_part}
            </div>
            <div class='implication_divider'>
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
