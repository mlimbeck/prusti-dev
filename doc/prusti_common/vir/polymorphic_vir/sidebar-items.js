initSidebarItems({"enum":[["AssignKind",""],["BinaryOpKind",""],["BitVector",""],["BitVectorSize",""],["CastKind",""],["Const",""],["ContainerOpKind",""],["Expr",""],["Float",""],["FloatConst",""],["PermAmount","The permission amount."],["PermAmountError",""],["PlaceComponent","A component that can be used to represent a place as a vector."],["Predicate",""],["Stmt",""],["Successor",""],["Type",""],["TypeId",""],["UnaryOpKind",""]],"fn":[["collect_assigned_vars",""],["compute_identifier",""],["default_fallible_fold_expr",""],["default_fallible_walk_expr",""],["default_fold_expr",""],["default_walk_expr",""],["fallible_walk_method",""],["fallible_walk_methods","Walks all Statements and Expressions in the provided methods"],["stmts_to_str",""],["walk_functions","Walks all Expressions in the provided functions (including pre and post conditions)"],["walk_method",""],["walk_methods","Walks all Statements and Expressions in the provided methods"]],"mod":[["ast",""],["borrows",""],["cfg",""],["conversions",""],["gather_labels",""],["program",""],["to_string",""],["utils","Various utility methods for working with VIR."]],"struct":[["AddrOf",""],["ApplyMagicWand",""],["Assert",""],["Assign",""],["BeginFrame",""],["BinOp",""],["BitVectorConst",""],["BodylessMethod",""],["Borrow","The method-unique borrow identifier."],["Cast",""],["CfgBlock",""],["CfgBlockIndex",""],["CfgMethod",""],["Comment","Individual structs for different cases of Expr"],["Cond",""],["ConstExpr",""],["ContainerOp",""],["DAG","Reborrowing directed acyclic graph (DAG). It should not be mutated after it is constructed. For construction use `DAGBuilder`."],["DAGBuilder","A struct for constructing the reborrowing DAG."],["Domain",""],["DomainAxiom",""],["DomainFunc",""],["DomainFuncApp",""],["DomainType",""],["Downcast",""],["DowncastExpr",""],["EndFrame",""],["EnumPredicate","The predicate for types that have 0 or more than one variants."],["EnumVariantIndex",""],["Exhale",""],["Exists",""],["ExpireBorrows",""],["Field",""],["FieldAccessPredicate",""],["FieldExpr",""],["Fold",""],["ForAll",""],["FuncApp",""],["Function",""],["FunctionIdentifier",""],["If",""],["Inhale",""],["InhaleExhale",""],["Label",""],["LabelledOld",""],["LetExpr",""],["Local","Individual structs for different cases of Expr"],["LocalVar",""],["MagicWand",""],["MethodCall",""],["Node","Node of the reborrowing DAG."],["Obtain",""],["PackageMagicWand",""],["Position","The identifier of a statement. Used in error reporting. TODO: This should probably have custom `PartialEq, Eq, Hash, PartialOrd, Ord` impls, to ensure that it is not included in these calculations."],["PredicateAccessPredicate",""],["Program",""],["Seq",""],["SeqType",""],["SnapApp",""],["SnapshotType",""],["StructPredicate","The predicate for types that have exactly one variant."],["TransferPerm",""],["Trigger",""],["TypeVar",""],["TypedRef",""],["UnaryOp",""],["Unfold",""],["Unfolding",""],["Variant",""]],"trait":[["CfgReplacer","Visit the reachable blocks of a CFG with a forward pass. During the visit, statements can be modified and injected. However, the structure of the CFG can not change. For each branch a context is updated, duplicated at forks, and merged with other contexts at joins."],["CheckNoOpAction",""],["ExprFolder",""],["ExprIterator",""],["ExprWalker",""],["FallibleExprFolder",""],["FallibleExprWalker",""],["FallibleStmtFolder",""],["FallibleStmtWalker",""],["StmtFolder",""],["StmtWalker",""],["SuccessorFolder",""],["ToString",""]],"type":[["MaybeEnumVariantIndex",""]]});