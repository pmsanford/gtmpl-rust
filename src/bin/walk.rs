use std::{collections::HashMap, env::args, fs::read_to_string};
use walkdir::WalkDir;

use gtmpl::{
    node::{FieldNode, Nodes},
    FuncError, Template, Value,
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

fn include_func(_args: &[Value]) -> Result<Value, FuncError> {
    Ok(Value::from(""))
}

fn trim(_args: &[Value]) -> Result<Value, FuncError> {
    Ok(Value::from(""))
}

fn mul_func(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 2 {
        return Err(FuncError::Generic("too many args".into()));
    }

    let a1 = match args[0] {
        Value::Number(ref n) => n,
        _ => return Err(FuncError::Generic("Bad arg type".into())),
    };
    let a2 = match args[1] {
        Value::Number(ref n) => n,
        _ => return Err(FuncError::Generic("Bad arg type".into())),
    };

    Ok(Value::from(a1.as_f64().unwrap() * a2.as_f64().unwrap()))
}

fn def_func(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 2 {
        return Err(FuncError::Generic("Too many args".into()));
    }
    Ok(if let Value::NoValue = args[0] {
        args[1].clone()
    } else {
        args[0].clone()
    })
}

fn main() {
    let dir = args().into_iter().skip(1).next();
    let mut fields = Vec::new();
    for entry in WalkDir::new(dir.unwrap())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().to_str().unwrap().ends_with(".yaml") {
            let cont = read_to_string(entry.path()).unwrap();
            let mut tmpl = Template::default();
            tmpl.add_func("include", include_func);
            tmpl.add_func("mul", mul_func);
            tmpl.add_func("default", def_func);
            tmpl.add_func("trimSuffix", trim);
            tmpl.add_func("trimAll", trim);
            tmpl.add_func("b64enc", trim);
            tmpl.add_func("quote", trim);
            tmpl.add_func("int", trim);
            tmpl.parse(cont).unwrap();
            let fst = tmpl.tree_set.values_mut().last().unwrap();
            walk_nodes(fst.root.clone().unwrap(), &mut fields);
        }
    }
    let mut context: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut values: HashMap<String, String> = HashMap::new();
    values.insert("name".into(), "paul".into());
    context.insert("Values".into(), values);
    let mut tmpl = Template::default();
    tmpl.parse("Hello {{ if .Values.name }}{{ .Values.name }}{{ else }}nobody{{ end }}")
        .unwrap();
    let mut all_fields: Vec<String> = fields.into_iter().map(transform_field).collect();
    all_fields.sort();
    all_fields.dedup();
    for field in all_fields {
        println!("{}", field);
    }
}
