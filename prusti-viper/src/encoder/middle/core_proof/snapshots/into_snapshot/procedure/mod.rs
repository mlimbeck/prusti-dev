//! Code for encoding expressions into snapshots in non-pure contexts such as
//! procedure bodies. Most important difference from `pure` is that this
//! encoding uses SSA.

use super::common::IntoSnapshotLowerer;
use crate::encoder::{
    errors::SpannedEncodingResult,
    middle::core_proof::{lowerer::Lowerer, snapshots::SnapshotVariablesInterface},
};
use vir_crate::{
    low::{self as vir_low},
    middle::{self as vir_mid},
};

mod traits;

pub(in super::super::super) trait IntoProcedureSnapshot {
    type Target;
    fn create_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target>;
}

struct ProcedureSnapshot;

impl<'p, 'v: 'p, 'tcx: 'v> IntoSnapshotLowerer<'p, 'v, 'tcx> for ProcedureSnapshot {
    fn variable_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        variable: &vir_mid::VariableDecl,
    ) -> SpannedEncodingResult<vir_low::VariableDecl> {
        lowerer.current_snapshot_variable_version(variable)
    }
}
