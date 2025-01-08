use crate::model::objects::{ImplicationFactor, Predicate, PredicateGroup, Relation};

fn diagram_domain(domain: &String) -> String {
    // let mut buffer = format!(
    //     r#"
    //     <div class='section_header'>
    //         Domains
    //     </div>
    // "#
    // );
    // let all_domains = graph.get_all_domains().unwrap();
    // println!("all_domains {:?}", &all_domains);
    // for domain in &all_domains {
    //     let elements = graph.get_entities_in_domain(domain).unwrap();
    //     println!("elements: {:?}", &elements);
    //     buffer += &format!(
    //         r#"
    //             <div class='row_element'>
    //                 <span class='domain_label'>{domain}</span>
    //                 <span><img src='/static/images/domains/{domain}.png' class='domain_icon'></img></span>
    //             </div>
    //         "#,
    //     )
    // }
    // buffer
    "".to_string()
}

fn diagram_relation(relation: &Relation) -> String {
    "".to_string()
}

fn diagram_predicate(relation: &Predicate) -> String {
    "".to_string()
}

fn diagram_predicate_group(relation: &PredicateGroup) -> String {
    "".to_string()
}

fn diagram_implication(relation: &ImplicationFactor) -> String {
    format!(
        r#"
        <div class='implication_box'>
            <div class='implication_row'>
                {predicate_group_part}
            </div>
        </div>
    "#,
        predicate_group_part = diagram_predicate_group(&relation.premise),
    )
}
