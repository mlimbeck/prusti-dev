use super::super::{
    ast::statement::visitors::StatementWalker,
    cfg::procedure::{BasicBlock, BasicBlockId, ProcedureDecl, Successor},
    visitors::ExpressionWalker,
    Predicate, Quantifier, Type, VariableDecl,
};
use std::collections::{BTreeMap, BTreeSet};

impl ProcedureDecl {
    /// Note: The traversal order is undefined.
    pub fn walk<Walker: StatementWalker + ExpressionWalker>(&self, walker: &mut Walker) {
        for block in self.basic_blocks.values() {
            for statement in &block.statements {
                walker.walk_statement(statement);
            }
            match &block.successor {
                Successor::Goto(_) => {}
                Successor::GotoSwitch(targets) => {
                    for (test, _) in targets {
                        ExpressionWalker::walk_expression(walker, test);
                    }
                }
                Successor::Return => {}
            }
        }
    }
    /// Note: The traversal order is undefined.
    pub fn walk_expressions<Walker: ExpressionWalker>(&self, expression_walker: &mut Walker) {
        struct Delegate<'a, Walker: ExpressionWalker> {
            expression_walker: &'a mut Walker,
        }
        impl<'a, Walker: ExpressionWalker> StatementWalker for Delegate<'a, Walker> {
            fn walk_predicate(&mut self, predicate: &Predicate) {
                match predicate {
                    Predicate::MemoryBlockStack(predicate) => {
                        self.expression_walker.walk_expression(&predicate.place);
                        self.expression_walker.walk_expression(&predicate.size);
                        self.expression_walker.walk_position(&predicate.position);
                    }
                    Predicate::MemoryBlockStackDrop(predicate) => {
                        self.expression_walker.walk_expression(&predicate.place);
                        self.expression_walker.walk_expression(&predicate.size);
                        self.expression_walker.walk_position(&predicate.position);
                    }
                    Predicate::MemoryBlockHeap(predicate) => {
                        self.expression_walker.walk_expression(&predicate.address);
                        self.expression_walker.walk_expression(&predicate.size);
                        self.expression_walker.walk_position(&predicate.position);
                    }
                    Predicate::MemoryBlockHeapDrop(predicate) => {
                        self.expression_walker.walk_expression(&predicate.address);
                        self.expression_walker.walk_expression(&predicate.size);
                        self.expression_walker.walk_position(&predicate.position);
                    }
                }
            }
        }
        impl<'a, Walker: ExpressionWalker> ExpressionWalker for Delegate<'a, Walker> {}
        let mut statement_walker = Delegate { expression_walker };
        self.walk(&mut statement_walker);
    }
    pub fn collect_locals(&self) -> Vec<VariableDecl> {
        #[derive(Default)]
        struct LocalCollector {
            locals: BTreeMap<String, Type>,
            bound_variables: Vec<BTreeSet<String>>,
        }
        impl ExpressionWalker for LocalCollector {
            fn walk_variable_decl(&mut self, variable: &VariableDecl) {
                if !self
                    .bound_variables
                    .iter()
                    .any(|set| set.contains(&variable.name))
                {
                    self.locals
                        .insert(variable.name.clone(), variable.ty.clone());
                }
            }
            fn walk_quantifier(&mut self, quantifier: &Quantifier) {
                self.bound_variables.push(
                    quantifier
                        .variables
                        .iter()
                        .map(|variable| variable.name.clone())
                        .collect(),
                );
                for trigger in &quantifier.triggers {
                    self.walk_trigger(trigger);
                }
                self.walk_expression(&quantifier.body);
                self.bound_variables.pop();
            }
        }
        let mut collector = LocalCollector::default();
        self.walk_expressions(&mut collector);
        collector
            .locals
            .into_iter()
            .map(|(name, ty)| VariableDecl { name, ty })
            .collect()
    }
    pub fn get_topological_sort(&self) -> Vec<BasicBlockId> {
        if self.basic_blocks.is_empty() {
            Vec::new()
        } else {
            let mut visited: BTreeMap<_, _> = self
                .basic_blocks
                .keys()
                .map(|label| (label.clone(), false))
                .collect();
            let mut topo_sorted = Vec::<BasicBlockId>::with_capacity(self.basic_blocks.len());
            *visited.get_mut(&self.entry).unwrap() = true;
            for label in self.basic_blocks.keys() {
                if !visited[label] {
                    self.topological_sort_impl(&mut visited, &mut topo_sorted, label);
                }
            }
            topo_sorted.push(self.entry.clone());
            topo_sorted.reverse();
            topo_sorted
        }
    }
    fn topological_sort_impl(
        &self,
        visited: &mut BTreeMap<BasicBlockId, bool>,
        topo_sorted: &mut Vec<BasicBlockId>,
        current_label: &BasicBlockId,
    ) {
        assert!(!visited[current_label]);
        *visited.get_mut(current_label).unwrap() = true;
        let current_block = &self.basic_blocks[current_label];
        for block_index in current_block.successor.get_following() {
            if !visited[block_index] {
                self.topological_sort_impl(visited, topo_sorted, block_index);
            }
        }
        topo_sorted.push(current_label.clone())
    }
}