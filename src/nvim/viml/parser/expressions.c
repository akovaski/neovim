#include "nvim/viml/parser/expressions.h"
#include "nvim/viml/parser/parser.h"

const char *const east_node_type_tab[] = {
  [kExprNodeMissing] = "Missing",
  [kExprNodeOpMissing] = "OpMissing",
  [kExprNodeTernary] = "Ternary",
  [kExprNodeTernaryValue] = "TernaryValue",
  [kExprNodeRegister] = "Register",
  [kExprNodeSubscript] = "Subscript",
  [kExprNodeListLiteral] = "ListLiteral",
  [kExprNodeUnaryPlus] = "UnaryPlus",
  [kExprNodeBinaryPlus] = "BinaryPlus",
  [kExprNodeNested] = "Nested",
  [kExprNodeCall] = "Call",
  [kExprNodePlainIdentifier] = "PlainIdentifier",
  [kExprNodePlainKey] = "PlainKey",
  [kExprNodeComplexIdentifier] = "ComplexIdentifier",
  [kExprNodeUnknownFigure] = "UnknownFigure",
  [kExprNodeLambda] = "Lambda",
  [kExprNodeDictLiteral] = "DictLiteral",
  [kExprNodeCurlyBracesIdentifier] = "CurlyBracesIdentifier",
  [kExprNodeComma] = "Comma",
  [kExprNodeColon] = "Colon",
  [kExprNodeArrow] = "Arrow",
  [kExprNodeComparison] = "Comparison",
  [kExprNodeConcat] = "Concat",
  [kExprNodeConcatOrSubscript] = "ConcatOrSubscript",
  [kExprNodeInteger] = "Integer",
  [kExprNodeFloat] = "Float",
  [kExprNodeSingleQuotedString] = "SingleQuotedString",
  [kExprNodeDoubleQuotedString] = "DoubleQuotedString",
  [kExprNodeOr] = "Or",
  [kExprNodeAnd] = "And",
  [kExprNodeUnaryMinus] = "UnaryMinus",
  [kExprNodeBinaryMinus] = "BinaryMinus",
  [kExprNodeNot] = "Not",
  [kExprNodeMultiplication] = "Multiplication",
  [kExprNodeDivision] = "Division",
  [kExprNodeMod] = "Mod",
  [kExprNodeOption] = "Option",
  [kExprNodeEnvironment] = "Environment",
  [kExprNodeAssignment] = "Assignment",
};

const char *const eltkn_cmp_type_tab[] = {
  [kExprCmpEqual] = "Equal",
  [kExprCmpMatches] = "Matches",
  [kExprCmpGreater] = "Greater",
  [kExprCmpGreaterOrEqual] = "GreaterOrEqual",
  [kExprCmpIdentical] = "Identical",
};

const char *const ccs_tab[] = {
  [kCCStrategyUseOption] = "UseOption",
  [kCCStrategyMatchCase] = "MatchCase",
  [kCCStrategyIgnoreCase] = "IgnoreCase",
};

const char *const expr_asgn_type_tab[] = {
  [kExprAsgnPlain] = "Plain",
  [kExprAsgnAdd] = "Add",
  [kExprAsgnSubtract] = "Subtract",
  [kExprAsgnConcat] = "Concat",
};
