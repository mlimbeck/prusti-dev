//! Public facing traits.

use super::IntoSnapshot;
use crate::encoder::{
    errors::SpannedEncodingResult,
    high::types::HighTypeEncoderInterface,
    middle::core_proof::{
        into_low::IntoLowInterface,
        lowerer::{FunctionsLowererInterface, Lowerer},
        snapshots::{SnapshotValuesInterface, into_snapshot::common::IntoSnapshotLowerer},
    },
};
use vir_crate::{
    common::identifier::WithIdentifier,
    low::{self as vir_low},
    middle::{self as vir_mid, operations::ty::Typed},
};


impl super::IntoProcedureSnapshot for vir_mid::Expression {
    type Target = vir_low::Expression;
    fn create_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target> {
        super::ProcedureSnapshot::expression_to_snapshot(lowerer, self)
    }
}