//! Public facing traits.

use super::{PureSnapshot};
use crate::encoder::{
    errors::SpannedEncodingResult,
    middle::core_proof::{lowerer::Lowerer, snapshots::into_snapshot::common::IntoSnapshotLowerer},
};
use vir_crate::{
    low::{self as vir_low},
    middle::{self as vir_mid},
};

// /// Converts `self` into expression that evaluates to a Viper Bool.
// pub(in super::super::super::super) trait IntoPureBoolExpression {
//     type Target;
//     fn to_pure_bool_expression<'p, 'v: 'p, 'tcx: 'v>(
//         &self,
//         lowerer: &mut Lowerer<'p, 'v, 'tcx>,
//     ) -> SpannedEncodingResult<Self::Target>;
// }

// impl IntoPureBoolExpression for vir_mid::Expression {
//     type Target = vir_low::Expression;
//     fn to_pure_bool_expression<'p, 'v: 'p, 'tcx: 'v>(
//         &self,
//         lowerer: &mut Lowerer<'p, 'v, 'tcx>,
//     ) -> SpannedEncodingResult<Self::Target> {
//         PureSnapshot::expression_to_bool(lowerer, self)
//     }
// }

// impl IntoPureBoolExpression for Vec<vir_mid::Expression> {
//     type Target = Vec<vir_low::Expression>;
//     fn to_pure_bool_expression<'p, 'v: 'p, 'tcx: 'v>(
//         &self,
//         lowerer: &mut Lowerer<'p, 'v, 'tcx>,
//     ) -> SpannedEncodingResult<Self::Target> {
//         let expressions = Vec::new();
//         for mid_expr in self {
//             expressions.push(mid_expr.to_pure_bool_expression(lowerer)?);
//         }
//         Ok(expressions)
//     }
// }

/// Converts `self` into expression that evaluates to a snapshot.
pub(in super::super::super::super) trait IntoPureSnapshot {
    type Target;
    fn to_pure_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target>;
}

impl IntoPureSnapshot for vir_mid::Expression {
    type Target = vir_low::Expression;
    fn to_pure_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target> {
        PureSnapshot::expression_to_snapshot(lowerer, self)
    }
}

impl IntoPureSnapshot for vir_mid::UnaryOpKind {
    type Target = vir_low::expression::UnaryOpKind;
    fn to_pure_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target> {
        PureSnapshot::unary_op_kind_to_snapshot(lowerer, self)
    }
}

impl IntoPureSnapshot for vir_mid::BinaryOpKind {
    type Target = vir_low::expression::BinaryOpKind;
    fn to_pure_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target> {
        PureSnapshot::binary_op_kind_to_snapshot(lowerer, self)
    }
}

impl IntoPureSnapshot for vir_mid::Type {
    type Target = vir_low::Type;
    fn to_pure_snapshot<'p, 'v: 'p, 'tcx: 'v>(
        &self,
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
    ) -> SpannedEncodingResult<Self::Target> {
        PureSnapshot::type_to_snapshot(lowerer, self)
    }
}