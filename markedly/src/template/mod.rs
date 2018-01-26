//! Templates parsed in from markup

mod component;
mod template;

mod parser {
    #[derive(Parser)]
    #[grammar = "template/template.pest"]
    pub struct TemplateParser;
}

pub use self::component::{ComponentTemplate};
pub use self::template::{Template};

#[cfg(test)]
mod test {
    use super::{Template, Value};

    #[test]
    fn it_parses_single_root() {
        let result = Template::from_str("root\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().root.class, "root");
    }

    #[test]
    fn it_parses_root_with_child() {
        let result = Template::from_str("root\n    child\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.children.len(), 1, "Incorrect children length on root");
        assert_eq!(root.children[0].class, "child");
    }

    #[test]
    fn it_parses_root_with_nested_children() {
        let result = Template::from_str("root\n    child\n        nested_child\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.children.len(), 1, "Incorrect children length on root");
        assert_eq!(root.children[0].class, "child");
        assert_eq!(root.children[0].children.len(), 1, "Incorrect children length on child");
        assert_eq!(root.children[0].children[0].class, "nested_child");
    }

    #[test]
    fn it_parses_root_with_two_children() {
        let result = Template::from_str("root\n    child1\n    child2\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.children.len(), 2, "Incorrect children length on root");
        assert_eq!(root.children[0].class, "child1");
        assert_eq!(root.children[1].class, "child2");
    }

    #[test]
    fn it_parses_varied_children_depth() {
        let result = Template::from_str("root\n    child1\n        nested_child\n    child2\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.children.len(), 2, "Incorrect children length on root");
        assert_eq!(root.children[0].class, "child1");
        assert_eq!(root.children[1].class, "child2");
        assert_eq!(root.children[0].children.len(), 1, "Incorrect children length on child1");
        assert_eq!(root.children[0].children[0].component, "nested_child");
    }

    #[test]
    fn it_parses_root_attributes() {
        let result = Template::from_str("root { key: \"value\" }\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.attributes.len(), 1);
        assert_eq!(root.attributes.get("key"), Some(&Value::String("value".into())));
    }

    #[test]
    fn it_parses_newlines_in_attributes_while_parsing_children() {
        let result = Template::from_str(
r#"root {
    key: "value",
    key2: "value2",
}
    child
"#
        );

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.children.len(), 1, "Incorrect children length on root");
        assert_eq!(root.children[0].component, "child");
    }

    #[test]
    fn it_parses_number_attributes() {
        let result = Template::from_str("root { key1: 5, key2: 2.5, key3: 69% }\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.attributes.len(), 3);
        assert_eq!(root.attributes.get("key1"), Some(&Value::Integer(5)));
        assert_eq!(root.attributes.get("key2"), Some(&Value::Float(2.5)));
        assert_eq!(root.attributes.get("key3"), Some(&Value::Percentage(69)));
    }

    #[test]
    fn it_parses_tuple_attributes() {
        let result = Template::from_str("root { key: (50, \"text\") }\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.attributes.len(), 1);
        assert_eq!(
            root.attributes.get("key"),
            Some(&Value::Tuple(vec!(Value::Integer(50), Value::String("text".into()))))
        );
    }

    #[test]
    fn it_fails_two_roots() {
        let result = Template::from_str("root\nroot2\n");

        println!("Result: {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn it_fails_excessive_indentation() {
        let result = Template::from_str("root\n        excessive_child1\n");

        println!("Result: {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn it_fails_non_4_indentation() {
        let result1 = Template::from_str("root\n  bad_child\n");
        let result2 = Template::from_str("root\n     bad_child\n");

        println!("Result1: {:?}", result1);
        println!("Result2: {:?}", result2);
        assert!(result1.is_err());
        assert!(result2.is_err());
    }

    #[test]
    fn it_fails_duplicate_keys() {
        let result = Template::from_str("root { key1: 5, key1: 10 }\n");

        println!("Result: {:?}", result);
        assert!(result.is_err());
    }
}
