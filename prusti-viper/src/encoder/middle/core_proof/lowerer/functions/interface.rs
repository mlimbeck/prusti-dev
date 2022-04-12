use crate::encoder::{
    errors::SpannedEncodingResult,
    high::pure_functions::HighPureFunctionEncoderInterface,
    middle::core_proof::{
        into_low::IntoLow,
        lowerer::{DomainsLowererInterface, Lowerer},
        snapshots::{IntoPureBoolExpression, IntoPureSnapshot},
    },
};
use std::collections::BTreeMap;
use vir_crate::{
    common::expression::QuantifierHelpers,
    low::{self as vir_low},
};

#[derive(Default)]
pub(in super::super) struct FunctionsLowererState {
    functions: BTreeMap<String, vir_low::FunctionDecl>,
}

impl FunctionsLowererState {
    pub fn destruct(self) -> Vec<vir_low::FunctionDecl> {
        self.functions.into_values().collect()
    }
}

pub(in super::super::super) trait FunctionsLowererInterface {
    fn create_func_app(
        &mut self,
        function_name: impl ToString,
        arguments: Vec<vir_low::Expression>,
        return_type: vir_low::Type,
        position: vir_low::Position,
    ) -> SpannedEncodingResult<vir_low::expression::FuncApp>;
    fn declare_function(&mut self, function: vir_low::FunctionDecl) -> SpannedEncodingResult<()>;
}

impl<'p, 'v: 'p, 'tcx: 'v> FunctionsLowererInterface for Lowerer<'p, 'v, 'tcx> {
    fn create_func_app(
        &mut self,
        function_name: impl ToString,
        arguments: Vec<vir_low::Expression>,
        return_type: vir_low::Type,
        position: vir_low::Position,
    ) -> SpannedEncodingResult<vir_low::expression::FuncApp> {
        assert!(
            !return_type.is_ref(),
            "return_type of a pure function cannot be Ref"
        );
        let function_name = function_name.to_string();
        let caller_function_name = format!("caller_for${}", function_name);
        let parameters = self.create_parameters(&arguments);
        #[allow(clippy::map_entry)]
        // Clippy false positive: it does not realize that we need multiple mutable references to self here.
        if !self.functions_state.functions.contains_key(&function_name) {
            let (pres, posts) = self.encoder.get_pure_function_specs_mid(&function_name)?;
            let pres = pres.to_pure_bool_expression(self)?;
            let posts = posts.to_pure_bool_expression(self)?;
            let function = vir_low::FunctionDecl::new(
                caller_function_name.clone(),
                parameters.clone(),
                return_type.clone(),
                pres,
                posts,
                Some(
                    self.create_domain_func_app(
                        "Functions",
                        function_name.clone(),
                        parameters
                            .iter()
                            .map(|parameter| parameter.clone().into())
                            .chain(self.function_gas_constant(2))
                            .collect(),
                        return_type.clone(),
                        Default::default(),
                    )?,
                ),
            );
            self.functions_state
                .functions
                .insert(function_name.clone(), function);

            // Encode the function body and postconditions if any.
            //
            // TODO: This should be done as a fix-point finalization action that
            // takes into account gas, (potentially mutual) recursion, predicate
            // unfoldings.
            let function_decl = self.encoder.get_pure_function_decl_mid(&function_name)?;
            if let Some(body) = function_decl.body {
                use vir_low::macros::*;
                let mut variables = function_decl.parameters.to_pure_snapshot(self)?;
                let mut arguments = variables
                    .clone()
                    .into_iter()
                    .map(|parameter| parameter.into())
                    .collect();
                let gas = vir_low::VariableDecl::new("gas$", self.function_gas_type()?);
                variables.push(gas);
                arguments.push(self.apply_funtion_gas(gas.into())?);
                let call = vir_low::Expression::domain_function_call(
                    "Functions",
                    function_decl.name.clone(),
                    arguments,
                    return_type.clone(),
                );
                let axiom_body = vir_low::Expression::forall(
                    variables,
                    vec![vir_low::Trigger::new(vec![call])],
                    expr! { [call] == [body.to_pure_snapshot(self)?] },
                );
                let axiom = vir_low::DomainAxiomDecl {
                    name: format!("{}$definitional_axiom", function_decl.name),
                    body: axiom_body,
                };
                self.declare_axiom("Functions", axiom)?;
            }
        }
        Ok(vir_low::expression::FuncApp {
            function_name: caller_function_name,
            arguments,
            parameters,
            return_type,
            position,
        })
    }
    /// The function must not be already declared.
    fn declare_function(&mut self, function: vir_low::FunctionDecl) -> SpannedEncodingResult<()> {
        assert!(!self.functions_state.functions.contains_key(&function.name));
        self.functions_state
            .functions
            .insert(function.name.clone(), function);
        Ok(())
    }
}
