use crate::model::objects::{ImplicationFactor, Predicate, PredicateGroup, Relation};

fn diagram_domain(domain: &String) -> String {
    format!(
        r#"
                <span class='domain_span'>
                    <span class='domain_label'>{domain}</span>
                    <span><img src='/static/images/domains/{domain}.png' class='domain_icon' /></span>
                </span>
    "#
    )
}

fn diagram_relation(relation: &Relation) -> String {
    format!(
        r#"
        <span class='relation'>
            <span class='relation_name'>
                {relation_name}
            </span>
        </span>
    "#,
        relation_name = &relation.relation_name
    )
}

fn diagram_predicate(predicate: &Predicate) -> String {
    let relation_html = diagram_relation(&predicate.relation);
    let roles_html = predicate
        .roles
        .iter()
        .map(|role| {
            format!(
                "<span class='labeled_argument'><strong>{}</strong>: {}</span>",
                role.role_name,
                role.argument.hash_string()
            )
        }) // Assuming argument can display hash_string
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "<div class='predicate'>{}<div class='roles'>[{}]</div></div>",
        relation_html, roles_html
    )
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

pub fn diagram_implication(relation: &ImplicationFactor) -> String {
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
