use prusti_interface::data::ProcedureDefId;
use prusti_rustc_interface::errors::MultiSpan;
use crate::encoder::Encoder;
use vir_crate::low::{self as vir_low};
use super::interface::MirProcedureMappingInterface;
use crate::encoder::errors::PositionManager;
use super::mapping::vir_low::ast::statement::Assume;//FIXME incorrect import
use super::mapping::vir_low::ast::statement::Inhale;
use super::mapping::vir_low::ast::expression::BinaryOp;
use super::mapping::vir_low::ast::expression::DomainFuncApp;
use super::mapping::vir_low::ast::expression::BinaryOpKind;
use rustc_hash::{FxHashMap};
use log::{info};

#[derive(Default)]
pub(crate) struct VarMapping{
    pub(crate) var_snaphot_mapping: FxHashMap<String, FxHashMap<String, Vec<SnapshotVar>>>, //Mapping from mir var to list of ssa snapshot vars per basic block
    pub(crate) labels_successor_mapping: FxHashMap<String, Vec<String>>, //Mapping of all labels and its successors
    //asserts: FxHasMap<>, //
}

#[derive(Debug)]
pub(crate) struct SnapshotVar{
    pub(crate) name: String, 
    pub(crate) position: vir_low::Position
}

pub(crate) trait VarMappingInterface {
    fn create_mapping(&mut self, def_id: ProcedureDefId, encoder: &Encoder);
    fn get_successor(&self, label: &String, label_markers: &FxHashMap<String, bool>) -> Option<&String>;
    fn get_span(&self, position_manager: &PositionManager, position: &vir_low::Position) -> MultiSpan;
    //fn get_snapshots_per_label(&self, var: &String, label: &String) -> &Vec<SnapshotVar>;
    fn extract_var_from_assume(&self, statement: &Assume) -> Option<SnapshotVar>;
    fn extract_var_from_inhale(&self, statement: &Inhale) -> Option<SnapshotVar>; //TODO: not implemented 
    //fn extract_var_from_assert(&self, statement: &Assert) -> Option<SnapshotVar>; 

}

impl<'ce, 'tcx, 'v> VarMappingInterface for super::counterexample_translation_snapshot::CounterexampleTranslator<'ce, 'tcx, 'v> {
    fn create_mapping(&mut self, proc_def_id: ProcedureDefId, encoder: &Encoder){
        let name = encoder.env().get_absolute_item_name(proc_def_id);
        //let mut mapping:FxHashMap<String, FxHashMap<String, Vec<SnapshotVar>>> = FxHashMap::default();
        if let Some(mir_procedure_mapping) = encoder.get_mapping(name){
            for basic_block in mir_procedure_mapping{
                let label = &basic_block.label;
                self.var_mapping.labels_successor_mapping.insert(label.clone(), basic_block.successor.clone());
                for statement in &basic_block.stmts{
                    let snapshot_var_option = match statement{
                        vir_low::Statement::Assume(assume ) => self.extract_var_from_assume(assume),
                        _ => None,
                    };

                    if let Some(snapshot_var) = snapshot_var_option{
                        let mir_var = snapshot_var.name.split_once("$").unwrap().0.to_string();
                        match self.var_mapping.var_snaphot_mapping.get_mut(&mir_var) {
                            Some(label_snapshot_mapping) => {
                                match label_snapshot_mapping.get_mut(label){
                                    Some(snapshot_var_vec) => {
                                        if !position_equality(&snapshot_var_vec.last().unwrap().position, &snapshot_var.position){ //safe vec is always initialized with one element     
                                            snapshot_var_vec.push(snapshot_var);
                                        } else {
                                            info!("Duplicate");
                                        }
                                        
                                    } 
                                    None => {
                                        let snapshot_var_vec = vec![snapshot_var];
                                        label_snapshot_mapping.insert(label.clone(), snapshot_var_vec);
                                    }
                                }
                            }
                            None => {
                                let snapshot_var_vec = vec![snapshot_var];
                                let mut label_snapshot_mapping = FxHashMap::default();
                                label_snapshot_mapping.insert(label.clone(), snapshot_var_vec);
                                //mapping.insert(mir_var, FxHashMap::from([(label, snapshot_var_vec)]));          
                                self.var_mapping.var_snaphot_mapping.insert(mir_var, label_snapshot_mapping);          
                            }   
                        }
                    }
                } 
            }
        }
        info!("Print mapping: {:?}", self.var_mapping.var_snaphot_mapping);
    }

    fn get_successor(&self, label: &String, label_markers: &FxHashMap<String, bool>) -> Option<&String>{
        match self.var_mapping.labels_successor_mapping.get(label){
            Some(possible_successors) => {
               let successor = possible_successors.iter().filter(|x| *label_markers.get(*x).unwrap_or(&false)).collect::<Vec<&String>>();
               if successor.len() == 1 { //there should always be at most one successor
                    Some(successor.first()?)
               } else { 
                   //the block has no successor (end of method)
                   //the block as mulitple successors, both should not be possible

                   //all succesors label_markers are false, which means this is the block where the error occurred
                    None
               }
            }
            None => None
        }
    }
    fn get_span(&self, position_manager: &PositionManager, position: &vir_low::Position) -> MultiSpan {
        position_manager.source_span.get(&position.id).unwrap().clone() //there is always a span available
    }
    fn extract_var_from_assume(&self, statement: &Assume) -> Option<SnapshotVar>{
        match &statement.expression {
            vir_low::Expression::Local(local) | 
            vir_low::Expression::BinaryOp(BinaryOp{ op_kind:BinaryOpKind::EqCmp ,  left:box vir_low::Expression::Local(local), .. }) => {
                info!("local assume stmt: {:?}", local);
                if local.variable.name.contains("snapshot"){
                    Some(SnapshotVar{
                        name: local.variable.name.clone(),
                        position: local.position.clone(),
                    })
                } else {
                    None
                }
            }, 
            vir_low::Expression::DomainFuncApp(DomainFuncApp{domain_name: _, function_name, arguments, .. }) | 
            vir_low::Expression::BinaryOp(BinaryOp{ op_kind:BinaryOpKind::EqCmp ,  left:box vir_low::Expression::DomainFuncApp(DomainFuncApp{domain_name: _, function_name, arguments, .. }), ..}) => {
                if arguments.len() == 1 && function_name.contains("target_current") { //difference between target_final and target_current //pointer
                    self.extract_var_from_assume(&Assume{expression: arguments.first().unwrap().clone(), position: vir_low::Position::default()})
                } else if arguments.len() == 1 && function_name.contains("destructor") {
                    info!("field update found");
                    self.extract_var_from_assume(&Assume{expression: arguments.first().unwrap().clone(), position: vir_low::Position::default()})
                } else {
                    None
                }
            },
            _ => None
        }
    }
    fn extract_var_from_inhale(&self, _statement: &Inhale) -> Option<SnapshotVar>{ //not implemented
        None
    }
}

fn position_equality(p1: &vir_low::Position, p2: &vir_low::Position) -> bool{
    p1.line == p2.line && p1.column == p2.column
}