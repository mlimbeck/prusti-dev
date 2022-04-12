//! Code for encoding expressions into snapshots in pure contexts such as domain
//! axioms. Most importantly, this encoding does not use SSA.

use super::{IntoSnapshot, common::IntoSnapshotLowerer};
use crate::encoder::{
    errors::SpannedEncodingResult,
    high::types::HighTypeEncoderInterface,
    middle::core_proof::{
        into_low::IntoLowInterface,
        lowerer::{FunctionsLowererInterface, Lowerer},
        snapshots::SnapshotValuesInterface,
    },
};
use vir_crate::{
    common::identifier::WithIdentifier,
    low::{self as vir_low},
    middle::{self as vir_mid, operations::ty::Typed},
};

mod traits;

pub(in super::super::super) trait IntoPureSnapshot {
    type Target;
    fn create_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target>;
}

struct PureSnapshot;

impl<'p, 'v: 'p, 'tcx: 'v> IntoSnapshotLowerer<'p, 'v, 'tcx> for PureSnapshot {
    fn variable_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        variable: &vir_mid::VariableDecl,
    ) -> SpannedEncodingResult<vir_low::VariableDecl> {
        Ok(vir_low::VariableDecl {
            name: variable.name.clone(),
            ty: Self::type_to_snapshot(lowerer, &variable.ty)?
        })
    }
}