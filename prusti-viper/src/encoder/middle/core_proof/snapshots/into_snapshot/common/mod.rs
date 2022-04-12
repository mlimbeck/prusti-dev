use super::super::super::lowerer::Lowerer;
use crate::encoder::{
    errors::SpannedEncodingResult,
    high::types::HighTypeEncoderInterface,
    middle::core_proof::{
        into_low::IntoLowInterface,
        lowerer::FunctionsLowererInterface,
        snapshots::{SnapshotDomainsInterface, SnapshotValuesInterface},
    },
};
use vir_crate::{
    common::identifier::WithIdentifier,
    low::{self as vir_low, //operations::ToLowLowerer
    },
    middle::{self as vir_mid, operations::ty::Typed},
};

pub(super) trait IntoSnapshotLowerer<'p, 'v: 'p, 'tcx: 'v> {
    fn expression_vec_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        expressions: &[vir_mid::Expression],
    ) -> SpannedEncodingResult<Vec<vir_low::Expression>> {
        let mut vec = Vec::new();
        for element in expressions {
            vec.push(Self::expression_to_snapshot(lowerer, element)?);
        }
        Ok(vec)
    }

    fn expression_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        expression: &vir_mid::Expression,
    ) -> SpannedEncodingResult<vir_low::Expression> {
        match expression {
            vir_mid::Expression::Local(expression) => Self::local_to_snapshot(lowerer, expression),
            vir_mid::Expression::Constructor(expression) => {
                Self::constructor_to_snapshot(lowerer, expression)
            }
            vir_mid::Expression::Variant(expression) => {
                Self::variant_to_snapshot(lowerer, expression)
            }
            vir_mid::Expression::Field(expression) => Self::field_to_snapshot(lowerer, expression),
            // vir_mid::Expression::Deref(expression) => Self::deref_to_snapshot(lowerer, expression),
            // vir_mid::Expression::AddrOf(expression) => Self::addrof_to_snapshot(lowerer, expression),
            // vir_mid::Expression::LabelledOld(expression) => Self::labelledold_to_snapshot(lowerer, expression),
            vir_mid::Expression::Constant(expression) => {
                Self::constant_to_snapshot(lowerer, expression)
            }
            vir_mid::Expression::UnaryOp(expression) => {
                Self::unary_op_to_snapshot(lowerer, expression)
            }
            vir_mid::Expression::BinaryOp(expression) => {
                Self::binary_op_to_snapshot(lowerer, expression)
            }
            // vir_mid::Expression::ContainerOp(expression) => Self::containerop_to_snapshot(lowerer, expression),
            // vir_mid::Expression::Seq(expression) => Self::seq_to_snapshot(lowerer, expression),
            vir_mid::Expression::Conditional(expression) => {
                Self::conditional_to_snapshot(lowerer, expression)
            }
            // vir_mid::Expression::Quantifier(expression) => Self::quantifier_to_snapshot(lowerer, expression),
            // vir_mid::Expression::LetExpr(expression) => Self::letexpr_to_snapshot(lowerer, expression),
            vir_mid::Expression::FuncApp(expression) => {
                Self::func_app_to_snapshot(lowerer, expression)
            }
            // vir_mid::Expression::Downcast(expression) => Self::downcast_to_snapshot(lowerer, expression),
            x => unimplemented!("{:?}", x),
        }
    }

    fn variable_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        local: &vir_mid::VariableDecl,
    ) -> SpannedEncodingResult<vir_low::VariableDecl>;

    fn local_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        local: &vir_mid::Local,
    ) -> SpannedEncodingResult<vir_low::Expression> {
        let snapshot_variable = Self::variable_to_snapshot(lowerer, &local.variable)?;
        Ok(vir_low::Expression::local(
            snapshot_variable,
            local.position,
        ))
    }

    fn constructor_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        constructor: &vir_mid::Constructor,
    ) -> SpannedEncodingResult<vir_low::Expression> {
        let mut arguments = Vec::new();
        for argument in &constructor.arguments {
            arguments.push(Self::expression_to_snapshot(lowerer, argument)?);
        }
        lowerer.construct_struct_snapshot(&constructor.ty, arguments, constructor.position)
    }

    fn variant_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        variant: &vir_mid::Variant,
    ) -> SpannedEncodingResult<vir_low::Expression> {
        let base_snapshot = Self::expression_to_snapshot(lowerer, &variant.base)?;
        lowerer.obtain_enum_variant_snapshot(
            variant.base.get_type(),
            &variant.variant_index,
            base_snapshot,
            variant.position,
        )
    }

    fn field_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        field: &vir_mid::Field,
    ) -> SpannedEncodingResult<vir_low::Expression> {
        let base_snapshot = Self::expression_to_snapshot(lowerer, &field.base)?;
        if field.field.is_discriminant() {
            let ty = field.base.get_type();
            // FIXME: Create a method for obtainging the discriminant type.
            let type_decl = lowerer.encoder.get_type_decl_mid(ty)?;
            let enum_decl = type_decl.unwrap_enum();
            let discriminant_call =
                lowerer.obtain_enum_discriminant(base_snapshot, ty, field.position)?;
            lowerer.construct_constant_snapshot(
                &enum_decl.discriminant_type,
                discriminant_call,
                field.position,
            )
        } else {
            lowerer.obtain_struct_field_snapshot(
                field.base.get_type(),
                &field.field,
                base_snapshot,
                field.position,
            )
        }
    }

    fn constant_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        constant: &vir_mid::Constant,
    ) -> SpannedEncodingResult<vir_low::Expression> {
        let low_type = match constant.ty {
            vir_mid::Type::MBool => vir_low::Type::Bool,
            vir_mid::Type::MInt => vir_low::Type::Int,
            vir_mid::Type::MFloat32 => unimplemented!(),
            vir_mid::Type::MFloat64 => unimplemented!(),
            vir_mid::Type::Bool => vir_low::Type::Bool,
            vir_mid::Type::Int(_) => vir_low::Type::Int,
            _ => unimplemented!("constant: {:?}", constant),
        };
        let argument = vir_low::Expression::Constant(vir_low::expression::Constant {
            value: Self::constant_value_to_snapshot(lowerer, &constant.value)?,
            ty: low_type,
            position: constant.position,
        });
        if matches!(constant.ty,
            vir_mid::Type::MBool
            | vir_mid::Type::MInt
            | vir_mid::Type::MFloat32
            | vir_mid::Type::MFloat64
        ) {
            Ok(argument)
        } else
        {lowerer.construct_constant_snapshot(&constant.ty, argument, constant.position)}
    }

    fn constant_value_to_snapshot(
        _lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        value: &vir_mid::expression::ConstantValue,
    ) -> SpannedEncodingResult<vir_low::expression::ConstantValue> {
        let low_value = match value {
            vir_mid::expression::ConstantValue::Bool(value) => {
                vir_low::expression::ConstantValue::Bool(*value)
            }
            vir_mid::expression::ConstantValue::Int(value) => {
                vir_low::expression::ConstantValue::Int(*value)
            }
            vir_mid::expression::ConstantValue::BigInt(value) => {
                vir_low::expression::ConstantValue::BigInt(value.clone())
            }
            vir_mid::expression::ConstantValue::Float(_value) => {
                unimplemented!();
            }
            vir_mid::expression::ConstantValue::FnPtr => {
                unimplemented!();
            }
        };
        Ok(low_value)
    }

    fn unary_op_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        op: &vir_mid::UnaryOp,
    ) -> SpannedEncodingResult<vir_low::Expression> {
        let argument_snapshot = Self::expression_to_snapshot(lowerer, &op.argument)?;
        lowerer.construct_unary_op_snapshot(
            op.op_kind,
            op.get_type(),
            argument_snapshot,
            op.position,
        )
    }

    fn unary_op_kind_to_snapshot(
        _lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        op: &vir_mid::UnaryOpKind,
    ) -> SpannedEncodingResult<vir_low::UnaryOpKind> {
        Ok(match op {
            vir_mid::UnaryOpKind::Not => vir_low::UnaryOpKind::Not,
            vir_mid::UnaryOpKind::Minus => vir_low::UnaryOpKind::Minus,
        })
    }

    fn binary_op_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        op: &vir_mid::BinaryOp,
    ) -> SpannedEncodingResult<vir_low::Expression> {
        let left_snapshot = Self::expression_to_snapshot(lowerer, &op.left)?;
        let right_snapshot = Self::expression_to_snapshot(lowerer, &op.right)?;
        let arg_type = op.left.get_type();
        assert_eq!(arg_type, op.right.get_type());
        lowerer.construct_binary_op_snapshot(
            op.op_kind,
            op.get_type(),
            arg_type,
            left_snapshot,
            right_snapshot,
            op.position,
        )
    }

    fn binary_op_kind_to_snapshot(
        _lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        op: &vir_mid::BinaryOpKind,
    ) -> SpannedEncodingResult<vir_low::BinaryOpKind> {
        Ok(match op {
            vir_mid::BinaryOpKind::EqCmp  => vir_low::BinaryOpKind::EqCmp,
            vir_mid::BinaryOpKind::NeCmp  => vir_low::BinaryOpKind::NeCmp,
            vir_mid::BinaryOpKind::GtCmp  => vir_low::BinaryOpKind::GtCmp,
            vir_mid::BinaryOpKind::GeCmp  => vir_low::BinaryOpKind::GeCmp,
            vir_mid::BinaryOpKind::LtCmp  => vir_low::BinaryOpKind::LtCmp,
            vir_mid::BinaryOpKind::LeCmp  => vir_low::BinaryOpKind::LeCmp,
            vir_mid::BinaryOpKind::Add  => vir_low::BinaryOpKind::Add,
            vir_mid::BinaryOpKind::Sub  => vir_low::BinaryOpKind::Sub,
            vir_mid::BinaryOpKind::Mul  => vir_low::BinaryOpKind::Mul,
            vir_mid::BinaryOpKind::Div  => vir_low::BinaryOpKind::Div,
            vir_mid::BinaryOpKind::Mod  => vir_low::BinaryOpKind::Mod,
            vir_mid::BinaryOpKind::And  => vir_low::BinaryOpKind::And,
            vir_mid::BinaryOpKind::Or  => vir_low::BinaryOpKind::Or,
            vir_mid::BinaryOpKind::Implies  => vir_low::BinaryOpKind::Implies,
        })
    }

    fn conditional_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        conditional: &vir_mid::Conditional,
    ) -> SpannedEncodingResult<vir_low::Expression> {
        let guard_snapshot =
            lowerer.lower_expression_into_snapshot_constant_value(&conditional.guard)?;
        let then_expr_snapshot = Self::expression_to_snapshot(lowerer, &conditional.then_expr)?;
        let else_expr_snapshot = Self::expression_to_snapshot(lowerer, &conditional.else_expr)?;
        let arg_type = conditional.then_expr.get_type();
        assert_eq!(arg_type, conditional.else_expr.get_type());
        Ok(vir_low::Expression::conditional(
            guard_snapshot,
            then_expr_snapshot,
            else_expr_snapshot,
            conditional.position,
        ))
    }

    fn func_app_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        app: &vir_mid::FuncApp,
    ) -> SpannedEncodingResult<vir_low::Expression> {
        let arguments = Self::expression_vec_to_snapshot(lowerer, &app.arguments)?;
        let return_type = Self::type_to_snapshot(lowerer, &app.return_type)?;
        let func_app =
            lowerer.create_func_app(app.get_identifier(), arguments, return_type, app.position)?;
        Ok(vir_low::Expression::FuncApp(func_app))
    }

    fn type_to_snapshot(
        lowerer: &mut Lowerer<'p, 'v, 'tcx>,
        ty: &vir_mid::Type,
    ) -> SpannedEncodingResult<vir_low::Type> {
        // This ensures that the domain is included into the program.
        lowerer.encode_snapshot_domain_type(ty)
    }
}
