// Autogenerated file.
// TODO(Gil): push the generating code and point to it from here.

use syntax::node::db::SyntaxGroup;
use syntax::node::kind::SyntaxKind;
use syntax::node::SyntaxNode;

use crate::formatter::{
    BreakLinePointProperties, BreakLinePointType, BreakingPosition, SyntaxNodeFormat,
};

fn parent_kind(db: &dyn SyntaxGroup, syntax_node: &SyntaxNode) -> Option<SyntaxKind> {
    Some(syntax_node.parent()?.kind(db))
}
fn parent_parent_kind(db: &dyn SyntaxGroup, syntax_node: &SyntaxNode) -> Option<SyntaxKind> {
    Some(syntax_node.parent()?.parent()?.kind(db))
}

impl SyntaxNodeFormat for SyntaxNode {
    fn force_no_space_before(&self, db: &dyn SyntaxGroup) -> bool {
        match self.kind(db) {
            SyntaxKind::TokenDot
            | SyntaxKind::TokenColon
            | SyntaxKind::TokenColonColon
            | SyntaxKind::TokenComma
            | SyntaxKind::TokenSemicolon
            | SyntaxKind::TokenQuestionMark
            | SyntaxKind::TokenRParen
            | SyntaxKind::TokenRBrack => true,
            SyntaxKind::TokenLParen
                if matches!(
                    parent_parent_kind(db, self),
                    Some(SyntaxKind::FunctionSignature | SyntaxKind::AttributeArgs)
                ) =>
            {
                true
            }
            SyntaxKind::TokenLBrack
                if matches!(parent_parent_kind(db, self), Some(SyntaxKind::Attribute)) =>
            {
                true
            }
            SyntaxKind::TokenLT | SyntaxKind::TokenGT
                if matches!(
                    parent_parent_kind(db, self),
                    Some(
                        SyntaxKind::PathSegmentWithGenericArgs
                            | SyntaxKind::GenericArgs
                            | SyntaxKind::WrappedGenericParamList
                    )
                ) =>
            {
                true
            }
            _ => false,
        }
    }

    fn force_no_space_after(&self, db: &dyn SyntaxGroup) -> bool {
        match self.kind(db) {
            SyntaxKind::TokenDot
            | SyntaxKind::TokenNot
            | SyntaxKind::TokenColonColon
            | SyntaxKind::TokenLParen
            | SyntaxKind::TokenLBrack
            | SyntaxKind::TokenLBrace
            | SyntaxKind::TokenImplicits => true,
            SyntaxKind::ExprPath | SyntaxKind::TerminalIdentifier
                if matches!(
                    parent_kind(db, self),
                    Some(
                        SyntaxKind::ItemFreeFunction
                            | SyntaxKind::ItemExternFunction
                            | SyntaxKind::ExprFunctionCall
                            | SyntaxKind::PatternEnum
                            | SyntaxKind::PatternStruct
                    )
                ) =>
            {
                true
            }
            SyntaxKind::TokenMinus => {
                matches!(parent_parent_kind(db, self), Some(SyntaxKind::ExprUnary))
            }
            SyntaxKind::TokenLT
                if matches!(
                    parent_parent_kind(db, self),
                    Some(
                        SyntaxKind::PathSegmentWithGenericArgs
                            | SyntaxKind::GenericArgs
                            | SyntaxKind::WrappedGenericParamList
                    )
                ) =>
            {
                true
            }
            _ => false,
        }
    }

    fn allowed_empty_between(&self, db: &dyn SyntaxGroup) -> usize {
        match self.kind(db) {
            SyntaxKind::ItemList => 2,
            SyntaxKind::StatementList => 1,
            _ => 0,
        }
    }

    fn get_break_line_point_properties(
        &self,
        db: &dyn SyntaxGroup,
        position: BreakingPosition,
    ) -> Option<BreakLinePointProperties> {
        match self.kind(db) {
            SyntaxKind::ItemList => match parent_kind(db, self) {
                Some(SyntaxKind::SyntaxFile) => {
                    if let BreakingPosition::Leading = position {
                        None
                    } else {
                        Some(BreakLinePointProperties {
                            precedence: 0,
                            break_type: BreakLinePointType::ListBreak,
                            is_optional: false,
                            add_indent: false,
                            space_if_ignored: false,
                        })
                    }
                }
                Some(SyntaxKind::ImplBody) => Some(BreakLinePointProperties {
                    precedence: 1,
                    break_type: BreakLinePointType::ListBreak,
                    is_optional: false,
                    add_indent: true,
                    space_if_ignored: false,
                }),
                _ => None,
            },
            SyntaxKind::TraitItemList => Some(BreakLinePointProperties {
                precedence: 2,
                break_type: BreakLinePointType::ListBreak,
                is_optional: false,
                add_indent: true,
                space_if_ignored: false,
            }),
            SyntaxKind::StatementList => Some(BreakLinePointProperties {
                precedence: 3,
                break_type: BreakLinePointType::ListBreak,
                is_optional: false,
                add_indent: true,
                space_if_ignored: false,
            }),
            SyntaxKind::MatchArms => Some(BreakLinePointProperties {
                precedence: 4,
                break_type: BreakLinePointType::SeparatedListBreak,
                is_optional: false,
                add_indent: true,
                space_if_ignored: false,
            }),
            SyntaxKind::AttributeList => {
                if let BreakingPosition::Leading = position {
                    None
                } else {
                    Some(BreakLinePointProperties {
                        precedence: 5,
                        break_type: BreakLinePointType::ListBreak,
                        is_optional: false,
                        add_indent: false,
                        space_if_ignored: false,
                    })
                }
            }
            SyntaxKind::ExprList => Some(BreakLinePointProperties {
                precedence: 6,
                break_type: BreakLinePointType::SeparatedListBreak,
                is_optional: true,
                add_indent: true,
                space_if_ignored: matches!(position, BreakingPosition::Internal),
            }),
            SyntaxKind::StructArgList => Some(BreakLinePointProperties {
                precedence: 7,
                break_type: BreakLinePointType::SeparatedListBreak,
                is_optional: true,
                add_indent: true,
                space_if_ignored: true,
            }),
            SyntaxKind::MemberList => Some(BreakLinePointProperties {
                precedence: 8,
                break_type: BreakLinePointType::SeparatedListBreak,
                is_optional: true,
                add_indent: true,
                space_if_ignored: true,
            }),
            SyntaxKind::ParamList => Some(BreakLinePointProperties {
                precedence: 9,
                break_type: BreakLinePointType::SeparatedListBreak,
                is_optional: true,
                add_indent: true,
                space_if_ignored: matches!(position, BreakingPosition::Internal),
            }),
            SyntaxKind::TokenPlus | SyntaxKind::TokenMinus => {
                if let BreakingPosition::Leading = position {
                    Some(BreakLinePointProperties {
                        precedence: 10,
                        break_type: BreakLinePointType::Dangling,
                        is_optional: true,
                        add_indent: false,
                        space_if_ignored: true,
                    })
                } else {
                    None
                }
            }
            SyntaxKind::TokenMul | SyntaxKind::TokenDiv => {
                if let BreakingPosition::Leading = position {
                    Some(BreakLinePointProperties {
                        precedence: 11,
                        break_type: BreakLinePointType::Dangling,
                        is_optional: true,
                        add_indent: false,
                        space_if_ignored: true,
                    })
                } else {
                    None
                }
            }
            // The precedence of a NewLine breakpoint should be greater than all other break points.
            SyntaxKind::TokenNewline => Some(BreakLinePointProperties {
                precedence: 1000,
                break_type: BreakLinePointType::Newline,
                is_optional: false,
                add_indent: false,
                space_if_ignored: false,
            }),
            _ => None,
        }
    }

    fn get_protected_zone_precedence(&self, db: &dyn SyntaxGroup) -> Option<usize> {
        match parent_kind(db, self) {
            Some(SyntaxKind::ItemFreeFunction) => match self.kind(db) {
                SyntaxKind::AttributeList => Some(1),
                SyntaxKind::FunctionSignature => Some(3),
                SyntaxKind::ExprBlock => Some(2),
                _ => None,
            },
            _ => match self.kind(db) {
                SyntaxKind::ExprParenthesized
                | SyntaxKind::StructArgList
                | SyntaxKind::ParamList
                | SyntaxKind::ExprList
                | SyntaxKind::StatementList => Some(1),
                _ => None,
            },
        }
    }
}
