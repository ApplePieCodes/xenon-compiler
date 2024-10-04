use std::borrow::{Borrow, BorrowMut};

use xenon_parser::node::{Definition, Namespace, Program};


struct SemanticParser {
    program: Program,
    variables: Vec<Variable>,
    namespaces: Vec<String>
}

impl SemanticParser {
    pub fn new(program: Program) -> Self {
        SemanticParser {
            program,
            variables: Vec::new(),
            namespaces: Vec::new()
        }
    }

    pub fn parse(&mut self) {
        let namespaces = std::mem::take(&mut self.program.namespaces);
        for namespace in namespaces {
            self.parse_namespace(namespace);
        }
    }
    
    

    pub fn parse_namespace(&mut self, namespace: Namespace) {
        if ! self.namespaces.contains(&namespace.name) {
            self.namespaces.push(namespace.name);
        }
        let definitions = std::mem::take(&mut namespace.definitions);
        for definition in definitions {
            self.parse_definition(definition);
        }
    }

    pub fn parse_definition(&mut self, &mut definition: Definition) {
        
    }
}


struct Variable {
    pub name: String,
    pub vtype: String
}