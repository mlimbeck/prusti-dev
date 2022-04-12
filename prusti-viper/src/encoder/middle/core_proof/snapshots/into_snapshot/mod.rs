//! The trait `IntoSnapshot` and its implementations.

mod expression;
mod ty;
mod variable;
mod vec;

use super::super::lowerer::Lowerer;
use crate::encoder::errors::SpannedEncodingResult;

pub(in super::super) trait IntoSnapshot {
    type Target;
    fn create_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target>;
}

// use vir_crate::{
//     common::identifier::WithIdentifier,
//     low::{self as vir_low},
//     middle::{self as vir_mid, operations::ty::Typed},
// };

// trait IntoSnapshotGeneric<const is_pure: bool> {
//     fn expression_to_snapshot(&mut self, expression: vir_mid::Expression
//     ) -> SpannedEncodingResult<vir_low::Expression>;
// }

// impl<'p, 'v: 'p, 'tcx: 'v> IntoSnapshotGeneric<true> for Lowerer<'p, 'v, 'tcx> {
//     fn expression_to_snapshot(&mut self, expression: vir_mid::Expression
//     ) -> SpannedEncodingResult<vir_low::Expression> {
//         unimplemented!();
//     }
// }

// impl<'p, 'v: 'p, 'tcx: 'v> IntoSnapshotGeneric<false> for Lowerer<'p, 'v, 'tcx> {
//     fn expression_to_snapshot(&mut self, expression: vir_mid::Expression
//     ) -> SpannedEncodingResult<vir_low::Expression> {
//         unimplemented!();
//     }
// }

mod common;
mod procedure;
mod pure;
