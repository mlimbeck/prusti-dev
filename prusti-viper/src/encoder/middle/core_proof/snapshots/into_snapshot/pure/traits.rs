//! Public facing traits.

use super::{IntoPureSnapshot, PureSnapshot};
use crate::encoder::{
    errors::SpannedEncodingResult,
    middle::core_proof::{lowerer::Lowerer, snapshots::into_snapshot::common::IntoSnapshotLowerer},
};
use vir_crate::{
    low::{self as vir_low},
    middle::{self as vir_mid},
};

impl IntoPureSnapshot for vir_mid::Expression {
    type Target = vir_low::Expression;
    fn create_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target> {
        PureSnapshot::expression_to_snapshot(lowerer, self)
    }
}
