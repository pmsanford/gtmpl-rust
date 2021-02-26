use std::collections::HashMap;

use gtmpl::{
    node::{FieldNode, Nodes},
    Context, Template,
};

fn walk_nodes(current: Nodes, fields: &mut Vec<FieldNode>) {
    if let Nodes::Field(ref fnode) = current {
        fields.push(fnode.clone());
    }

    if let Some(children) = current.children() {
        for node in children {
            walk_nodes(node, fields);
        }
    }
}

fn transform_field(field: FieldNode) -> String {
    field.ident.join(".")
}

fn main() {
    let mut context: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut values: HashMap<String, String> = HashMap::new();
    values.insert("name".into(), "paul".into());
    context.insert("Values".into(), values);
    let mut tmpl = Template::default();
    tmpl.parse("Hello {{ if .Values.name }}{{ .Values.name }}{{ else }}nobody{{ end }}")
        .unwrap();
    let fst = tmpl.tree_set.values_mut().last().unwrap();
    let mut fields = Vec::new();
    walk_nodes(fst.root.clone().unwrap(), &mut fields);
    let mut all_fields: Vec<String> = fields.into_iter().map(transform_field).collect();
    all_fields.sort();
    all_fields.dedup();
    println!("Fields: {:#?})", all_fields);

    println!("{}", tmpl.render(&Context::from(context)).unwrap());
}
