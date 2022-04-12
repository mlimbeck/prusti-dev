//! Public facing traits.

use crate::encoder::{
    errors::SpannedEncodingResult,
    middle::core_proof::{lowerer::Lowerer, snapshots::into_snapshot::common::IntoSnapshotLowerer},
};
use vir_crate::{
    low::{self as vir_low},
    middle::{self as vir_mid},
};
use super::ProcedureSnapshot;

pub(in super::super::super::super) trait IntoProcedureSnapshot {
    type Target;
    fn to_procedure_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target>;
}

impl IntoProcedureSnapshot for vir_mid::Expression {
    type Target = vir_low::Expression;
    fn to_procedure_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target> {
        ProcedureSnapshot::expression_to_snapshot(lowerer, self)
    }
}

impl IntoProcedureSnapshot for vir_mid::Type {
    type Target = vir_low::Type;
    fn to_procedure_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target> {
        ProcedureSnapshot::type_to_snapshot(lowerer, self)
    }
}