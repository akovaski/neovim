// VimL expression parser

// Planned incompatibilities (to be included into vim_diff.txt when this parser
// will be an actual part of VimL evaluation process):
//
// 1. Expressions are first fully parsed and only then executed.  This means
//    that while ":echo [system('touch abc')" will create file "abc" in Vim and
//    only then raise syntax error regarding missing comma in list in Neovim
//    trying to execute that will immediately raise syntax error regarding
//    missing list end without actually executing anything.
// 2. Expressions are first fully parsed, without considering any runtime
//    information.  This means things like that "d.a" does not change its
//    meaning depending on type of "d" (or whether Vim is currently executing or
//    skipping).  For compatibility reasons the dot thus may either be “concat
//    or subscript” operator or just “concat” operator.
// 3. Expressions parser is aware whether it is called for :echo or <C-r>=.
//    This means that while "<C-r>=1 | 2<CR>" is equivalent to "<C-r>=1<CR>"
//    because "| 2" part is left to be treated as a command separator and then
//    ignored in Neovim it is an error.
// 4. Expressions parser has generally better error reporting.  But for
//    compatibility reasons most errors have error code E15 while error messages
//    are significantly different from Vim’s E15.  Also some error codes were
//    retired because of being harder to emulate or because of them being
//    a result of differences in parsing process: e.g. with ":echo {a, b}" Vim
//    will attempt to parse expression as lambda, fail, check whether it is
//    a curly-braces-name, fail again, and evaluate that as a dictionary, giving
//    error regarding undefined variable "a" (or about missing colon).  Neovim
//    will not try to evaluate anything here: comma right after an argument name
//    means that expression may not be anything, but lambda, so the resulting
//    error message will never be about missing variable or colon: it will be
//    about missing arrow (or a continuation of argument list).
// 5. Failing to parse expression always gives exactly one error message: no
//    more stack of error messages like >
//
//        :echo [1,
//        E697: Missing end of List ']':
//        E15: Invalid expression: [1,
//
// <  , just exactly one E697 message.
// 6. Some expressions involving calling parenthesis which are treated
//    separately by Vim even when not separated by spaces are treated as one
//    expression by Neovim: e.g. ":echo (1)(1)" will yield runtime error after
//    failing to call "1", while Vim will echo "1 1". Reasoning is the same:
//    type of what is in the first expression is generally not known when
//    parsing, so to have separate expressions like this separate them with
//    spaces.
// 7. 'isident' no longer applies to environment variables, they always include
//    ASCII alphanumeric characters and underscore and nothing except this.

use crate::*;
use bitflags::bitflags;
use std::ffi::CString;
use std::mem;
use std::ptr;

// Defines whether to ignore case:
//    ==   kCCStrategyUseOption
//    ==#  kCCStrategyMatchCase
//    ==?  kCCStrategyIgnoreCase
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub enum ExprCaseCompareStrategy {
    kCCStrategyUseOption = 0,
    kCCStrategyMatchCase = '#' as isize,
    kCCStrategyIgnoreCase = '?' as isize,
}
impl From<u8> for ExprCaseCompareStrategy {
    fn from(f: u8) -> Self {
        match f as char {
            '\u{0}' => kCCStrategyUseOption,
            '#' => kCCStrategyMatchCase,
            '?' => kCCStrategyIgnoreCase,
            _ => panic!("tried to convert invalid value into ExprCaseCompareStrategy"),
        }
    }
}
pub use ExprCaseCompareStrategy::*;

/// Lexer token type
/// When modifying this enum you need to also modify eltkn_type_tab in
/// expressions.c and tests and, possibly, viml_pexpr_repr_token.
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub enum LexExprTokenType {
    kExprLexInvalid = 0, //< Invalid token, indicaten an error.
    kExprLexMissing,     //< Missing token, for use in parser.
    kExprLexSpacing,     //< Spaces, tabs, newlines, etc.
    kExprLexEOC,         //< End of command character: NL, |, just end of stream.

    kExprLexQuestion,       //< Question mark, for use in ternary.
    kExprLexColon,          //< Colon, for use in ternary.
    kExprLexOr,             //< Logical or operator.
    kExprLexAnd,            //< Logical and operator.
    kExprLexComparison,     //< One of the comparison operators.
    kExprLexPlus,           //< Plus sign.
    kExprLexMinus,          //< Minus sign.
    kExprLexDot,            //< Dot: either concat or subscript, also part of the float.
    kExprLexMultiplication, //< Multiplication, division or modulo operator.

    kExprLexNot, //< Not: !.

    kExprLexNumber,             //< Integer number literal, or part of a float.
    kExprLexSingleQuotedString, //< Single quoted string literal.
    kExprLexDoubleQuotedString, //< Double quoted string literal.
    kExprLexOption,             //< &optionname option value.
    kExprLexRegister,           //< @r register value.
    kExprLexEnv,                //< Environment $variable value.
    kExprLexPlainIdentifier,    //< Identifier without scope: `abc`, `foo#bar`.

    kExprLexBracket,     //< Bracket, either opening or closing.
    kExprLexFigureBrace, //< Figure brace, either opening or closing.
    kExprLexParenthesis, //< Parenthesis, either opening or closing.
    kExprLexComma,       //< Comma.
    kExprLexArrow,       //< Arrow, like from lambda expressions.
    kExprLexAssignment,  //< Assignment: `=` or `{op}=`.
}
pub use LexExprTokenType::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub enum ExprComparisonType {
    kExprCmpEqual,          //< Equality, unequality.
    kExprCmpMatches,        //< Matches regex, not matches regex.
    kExprCmpGreater,        //< `>` or `<=`
    kExprCmpGreaterOrEqual, //< `>=` or `<`.
    kExprCmpIdentical,      //< `is` or `isnot`
}
pub use ExprComparisonType::*;

/// All possible option scopes
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub enum ExprOptScope {
    kExprOptScopeUnspecified = 0,
    kExprOptScopeGlobal = 'g' as isize,
    kExprOptScopeLocal = 'l' as isize,
}
impl From<u8> for ExprOptScope {
    fn from(f: u8) -> Self {
        match f as char {
            '\u{0}' => kExprOptScopeUnspecified,
            'g' => kExprOptScopeGlobal,
            'l' => kExprOptScopeLocal,
            _ => panic!("tried to convert invalid value into ExprOptScope"),
        }
    }
}
pub use ExprOptScope::*;

/// All possible assignment types: `=` and `{op}=`.
#[derive(Copy, Clone)]
#[repr(C)]
pub enum ExprAssignmentType {
    kExprAsgnPlain = 0, //< Plain assignment: `=`.
    kExprAsgnAdd,       //< Assignment augmented with addition: `+=`.
    kExprAsgnSubtract,  //< Assignment augmented with subtraction: `-=`.
    kExprAsgnConcat,    //< Assignment augmented with concatenation: `.=`.
}
pub use ExprAssignmentType::*;

const EXPR_OPT_SCOPE_LIST: [u8; 2] = [kExprOptScopeGlobal as u8, kExprOptScopeLocal as u8];

/// All possible variable scopes
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub enum ExprVarScope {
    kExprVarScopeMissing = 0,
    kExprVarScopeScript = 's' as isize,
    kExprVarScopeGlobal = 'g' as isize,
    kExprVarScopeVim = 'v' as isize,
    kExprVarScopeBuffer = 'b' as isize,
    kExprVarScopeWindow = 'w' as isize,
    kExprVarScopeTabpage = 't' as isize,
    kExprVarScopeLocal = 'l' as isize,
    kExprVarScopeArguments = 'a' as isize,
}
impl From<char> for ExprVarScope {
    fn from(f: char) -> Self {
        match f {
            '\u{0}' => kExprVarScopeMissing,
            's' => kExprVarScopeScript,
            'g' => kExprVarScopeGlobal,
            'v' => kExprVarScopeVim,
            'b' => kExprVarScopeBuffer,
            'w' => kExprVarScopeWindow,
            't' => kExprVarScopeTabpage,
            'l' => kExprVarScopeLocal,
            'a' => kExprVarScopeArguments,
            _ => panic!("tried to convert invalid value into ExprVarScope"),
        }
    }
}
use ExprVarScope::*;

const EXPR_VAR_SCOPE_LIST: [u8; 9] = [
    kExprVarScopeScript as u8,
    kExprVarScopeGlobal as u8,
    kExprVarScopeVim as u8,
    kExprVarScopeBuffer as u8,
    kExprVarScopeWindow as u8,
    kExprVarScopeTabpage as u8,
    kExprVarScopeLocal as u8,
    kExprVarScopeBuffer as u8,
    kExprVarScopeArguments as u8,
];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexExprToken {
    pub start: ParserPosition,
    pub len: libc::size_t,
    pub type_0: LexExprTokenType,
    pub data: Let_data,
}
impl Default for LexExprToken {
    fn default() -> LexExprToken {
        unsafe { mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Let_data {
    pub cmp: Let_data_cmp,   //< For kExprLexComparison
    pub mul: Let_data_mul,   //< For kExprLexMultiplication
    pub brc: Let_data_brc,   //< For brackets/braces/parenthesis
    pub reg: Let_data_reg,   //< For kExprLexRegister
    pub str_0: Let_data_str, //< For kExprLexSingleQuotedString and kExprLexDoubleQuotedString
    pub opt: Let_data_opt,   //< Option properties
    pub var: Let_data_var,   //< For kExprLexPlainIdentifier
    pub err: Let_data_err,   //< For kExprLexInvalid
    pub num: Let_data_num,   //< For kExprLexNumber
    pub ass: Let_data_ass,   //< For kExprLexAssignment
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Let_data_cmp {
    pub type_0: ExprComparisonType,   //< Comparison type.
    pub ccs: ExprCaseCompareStrategy, //< Case comparison strategy.
    pub inv: bool,                    //< True if comparison is to be inverted.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Let_data_mul {
    pub type_0: Let_data_mul_type, //< Multiplication type.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub enum Let_data_mul_type {
    kExprLexMulMul, //< Real multiplication.
    kExprLexMulDiv, //< Division.
    kExprLexMulMod, //< Modulo.
}
pub use Let_data_mul_type::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Let_data_brc {
    pub closing: bool, //< True if bracket/etc is a closing one.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Let_data_reg {
    pub name: libc::c_int, //< Register name, may be -1 if name not present.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Let_data_str {
    pub closed: bool, //< True if quote was closed.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Let_data_opt {
    pub name: *const u8,     //< Option name start.
    pub len: libc::size_t,   //< Option name length.
    pub scope: ExprOptScope, //< Option scope: &l:, &g: or not specified.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Let_data_var {
    pub scope: ExprVarScope, //< Scope character or 0 if not present.
    pub autoload: bool,      //< Has autoload characters.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Let_data_err {
    pub type_0: LexExprTokenType, //< Suggested type for parsing incorrect code.
    pub msg: *const u8,           //< Error message.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Let_data_num {
    pub val: Let_data_num_val, //< Number value.
    pub base: u8,              //< Base: 2, 8, 10 or 16.
    pub is_float: bool,        //< True if number is a floating-point.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Let_data_num_val {
    pub floating: float_T,
    pub integer: uvarnumber_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Let_data_ass {
    pub type_0: ExprAssignmentType,
}

bitflags! {
    /// Whenever you add a new flag, alter klee_assume() statement in
    /// viml_expressions_lexer.c.
    pub struct LexExprFlags: u32 {
        /// If set, “pointer” to the current byte in pstate will not be shifted
        const Peek = (1 << 0);
        /// Determines whether scope is allowed to come before the identifier
        const ForbidScope = (1 << 1);
        /// Determines whether floating-point numbers are allowed
        ///
        /// I.e. whether dot is a decimal point separator or is not a part of
        /// a number at all.
        const AllowFloat = (1 << 2);
        /// Determines whether `is` and `isnot` are seen as comparison operators
        ///
        /// If set they are supposed to be just regular identifiers.
        const IsNotCmp = (1 << 3);
        /// Determines whether EOC tokens are allowed
        ///
        /// If set then it will yield Invalid token with E15 in place of EOC one if
        /// “EOC” is something like "|". It is fine with emitting EOC at the end of
        /// string still, with or without this flag set.
        const ForbidEOC = (1 << 4);
    }
}

/// Expression AST node type
/// When modifying this list also modify east_node_type_tab both in parser
/// and in tests, and you most likely will also have to alter list of
/// highlight groups stored in highlight_init_cmdline variable.
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub enum ExprASTNodeType {
    kExprNodeMissing = 0,
    kExprNodeOpMissing,
    kExprNodeTernary,      //< Ternary operator.
    kExprNodeTernaryValue, //< Ternary operator, colon.
    kExprNodeRegister,     //< Register.
    kExprNodeSubscript,    //< Subscript.
    kExprNodeListLiteral,  //< List literal.
    kExprNodeUnaryPlus,
    kExprNodeBinaryPlus,
    kExprNodeNested, //< Nested parenthesised expression.
    kExprNodeCall,   //< Function call.
    // Plain identifier: simple variable/function name
    //
    // Looks like "string", "g:Foo", etc: consists from a single
    // kExprLexPlainIdentifier token.
    kExprNodePlainIdentifier,
    // Plain dictionary key, for use with kExprNodeConcatOrSubscript
    kExprNodePlainKey,
    // Complex identifier: variable/function name with curly braces
    kExprNodeComplexIdentifier,
    // Figure brace expression which is not yet known
    //
    // May resolve to any of kExprNodeDictLiteral, kExprNodeLambda or
    // kExprNodeCurlyBracesIdentifier.
    kExprNodeUnknownFigure,
    kExprNodeLambda,                //< Lambda.
    kExprNodeDictLiteral,           //< Dictionary literal.
    kExprNodeCurlyBracesIdentifier, //< Part of the curly braces name.
    kExprNodeComma,                 //< Comma “operator”.
    kExprNodeColon,                 //< Colon “operator”.
    kExprNodeArrow,                 //< Arrow “operator”.
    kExprNodeComparison,            //< Various comparison operators.
    // Concat operator
    //
    // To be only used in cases when it is known for sure it is not a subscript.
    kExprNodeConcat,
    // Concat or subscript operator
    //
    // For cases when it is not obvious whether expression is a concat or
    // a subscript. May only have either number or plain identifier as the second
    // child. To make it easier to avoid curly braces in place of
    // kExprNodePlainIdentifier node kExprNodePlainKey is used.
    kExprNodeConcatOrSubscript,
    kExprNodeInteger, //< Integral number.
    kExprNodeFloat,   //< Floating-point number.
    kExprNodeSingleQuotedString,
    kExprNodeDoubleQuotedString,
    kExprNodeOr,
    kExprNodeAnd,
    kExprNodeUnaryMinus,
    kExprNodeBinaryMinus,
    kExprNodeNot,
    kExprNodeMultiplication,
    kExprNodeDivision,
    kExprNodeMod,
    kExprNodeOption,
    kExprNodeEnvironment,
    kExprNodeAssignment,
}
pub use ExprASTNodeType::*;

/// Structure representing one AST node
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprASTNode {
    pub type_0: ExprASTNodeType, //< Node type.
    // Node children: e.g. for 1 + 2 nodes 1 and 2 will be children of +.
    pub children: *mut ExprASTNode,
    // Next node: e.g. for 1 + 2 child nodes 1 and 2 are put into a single-linked
    // list: `(+)->children` references only node 1, node 2 is in
    // `(+)->children->next`.
    pub next: *mut ExprASTNode,
    pub start: ParserPosition,
    pub len: libc::size_t,
    pub data: EastN_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union EastN_data {
    pub reg: EastN_data_reg,   //< For kExprNodeRegister
    pub fig: EastN_data_fig,   //< For kExprNodeUnknownFigure
    pub var: EastN_data_var,   //< For kExprNodePlainIdentifier and kExprNodePlainKey
    pub ter: EastN_data_ter,   //< For kExprNodeTernaryValue
    pub cmp: EastN_data_cmp,   //< For kExprNodeComparison
    pub num: EastN_data_num,   //< For kExprNodeInteger
    pub flt: EastN_data_flt,   //< For kExprNodeFloat
    pub str_0: EastN_data_str, //< For kExprNodeSingleQuotedString and kExprNodeDoubleQuotedString
    pub opt: EastN_data_opt,   //< For kExprNodeOption.
    pub env: EastN_data_env,   //< For kExprNodeEnvironment
    pub ass: EastN_data_ass,   //< For kExprNodeAssignment
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_reg {
    pub name: libc::c_int, //< Register name, may be -1 if name not present.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_fig {
    pub type_guesses: EastN_data_fig_guesses, // Which nodes UnknownFigure can’t possibly represent.
    pub opening_hl_idx: libc::size_t, // Highlight chunk index, used for rehighlighting if needed
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_fig_guesses {
    pub allow_dict: bool, // True if UnknownFigure may actually represent dictionary literal.
    pub allow_lambda: bool, // True if UnknownFigure may actually represent lambda.
    pub allow_ident: bool, // True if UnknownFigure may actually be part of curly braces name.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_var {
    pub scope: ExprVarScope, //< Scope character or 0 if not present.
    // Actual identifier without scope.
    //
    // Points to inside parser reader state.
    pub ident: *const u8,
    pub ident_len: libc::size_t, //< Actual identifier length.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_ter {
    pub got_colon: bool, //< True if colon was seen.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_cmp {
    pub type_0: ExprComparisonType,   //< Comparison type.
    pub ccs: ExprCaseCompareStrategy, //< Case comparison strategy.
    pub inv: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_num {
    pub value: uvarnumber_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_flt {
    pub value: float_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_str {
    pub value: *mut u8,
    pub size: libc::size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_opt {
    pub ident: *const u8,        //< Option name start.
    pub ident_len: libc::size_t, //< Option name length.
    pub scope: ExprOptScope,     //< Option scope: &l:, &g: or not specified.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_env {
    pub ident: *const u8,        //< Environment variable name start.
    pub ident_len: libc::size_t, //< Environment variable name length.
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EastN_data_ass {
    pub type_0: ExprAssignmentType,
}

bitflags! {
    ///  Whenever you add a new flag, alter klee_assume() statement in
    ///  viml_expressions_parser.c, nvim_parse_expression() flags parsing
    ///  alongside with its documentation and flag sets in check_parsing()
    ///  function in expressions parser functional and unit tests.
    pub struct ExprParserFlags: u32 {
        // Allow multiple expressions in a row: e.g. for :echo
        //
        // Parser will still parse only one of them though.
        const Multi = (1 << 0);
        // Allow NL, NUL and bar to be EOC
        //
        // When parsing expressions input by user bar is assumed to be a binary
        // operator and other two are spacings.
        const DisallowEOC = (1 << 1);
        // Parse :let argument
        //
        // That mean that top level node must be an assignment and first nodes
        // belong to lvalues.
        const ParseLet = (1 << 2);
    }
}

/// AST error definition
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprASTError {
    /// Error message. Must contain a single printf format atom: %.*s.
    pub msg: *const u8,
    /// Error message argument: points to the location of the error.
    pub arg: *const u8,
    /// Message argument length: length till the end of string.
    pub arg_len: libc::c_int,
}

/// Structure representing complety AST for one expression
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprAST {
    /// When AST is not correct this message will be printed.
    ///
    /// Uses `emsgf(msg, arg_len, arg);`, `msg` is assumed to contain only `%.*s`.
    pub err: ExprASTError,
    /// Root node of the AST.
    pub root: *mut ExprASTNode,
}

/// Function mapping ExprASTNodeType to the maximum amount of children node may have
fn node_maxchildren(node_type: ExprASTNodeType) -> u8 {
    match node_type {
        kExprNodeMissing => 0,
        kExprNodeOpMissing => 2,
        kExprNodeTernary => 2,
        kExprNodeTernaryValue => 2,
        kExprNodeRegister => 0,
        kExprNodeSubscript => 2,
        kExprNodeListLiteral => 1,
        kExprNodeUnaryPlus => 1,
        kExprNodeBinaryPlus => 2,
        kExprNodeNested => 1,
        kExprNodeCall => 2,
        kExprNodePlainIdentifier => 0,
        kExprNodePlainKey => 0,
        kExprNodeComplexIdentifier => 2,
        kExprNodeUnknownFigure => 1,
        kExprNodeLambda => 2,
        kExprNodeDictLiteral => 1,
        kExprNodeCurlyBracesIdentifier => 1,
        kExprNodeComma => 2,
        kExprNodeColon => 2,
        kExprNodeArrow => 2,
        kExprNodeComparison => 2,
        kExprNodeConcat => 2,
        kExprNodeConcatOrSubscript => 2,
        kExprNodeInteger => 0,
        kExprNodeFloat => 0,
        kExprNodeSingleQuotedString => 0,
        kExprNodeDoubleQuotedString => 0,
        kExprNodeOr => 2,
        kExprNodeAnd => 2,
        kExprNodeUnaryMinus => 1,
        kExprNodeBinaryMinus => 2,
        kExprNodeNot => 1,
        kExprNodeMultiplication => 2,
        kExprNodeDivision => 2,
        kExprNodeMod => 2,
        kExprNodeOption => 0,
        kExprNodeEnvironment => 0,
        kExprNodeAssignment => 2,
    }
}

/// Function mapping ExprASTNodeType values to their stringified versions
pub fn east_node_type_tab(node_type: ExprASTNodeType) -> *const u8 {
    match node_type {
        kExprNodeMissing => "Missing",
        kExprNodeOpMissing => "OpMissing",
        kExprNodeTernary => "Ternary",
        kExprNodeTernaryValue => "TernaryValue",
        kExprNodeRegister => "Register",
        kExprNodeSubscript => "Subscript",
        kExprNodeListLiteral => "ListLiteral",
        kExprNodeUnaryPlus => "UnaryPlus",
        kExprNodeBinaryPlus => "BinaryPlus",
        kExprNodeNested => "Nested",
        kExprNodeCall => "Call",
        kExprNodePlainIdentifier => "PlainIdentifier",
        kExprNodePlainKey => "PlainKey",
        kExprNodeComplexIdentifier => "ComplexIdentifier",
        kExprNodeUnknownFigure => "UnknownFigure",
        kExprNodeLambda => "Lambda",
        kExprNodeDictLiteral => "DictLiteral",
        kExprNodeCurlyBracesIdentifier => "CurlyBracesIdentifier",
        kExprNodeComma => "Comma",
        kExprNodeColon => "Colon",
        kExprNodeArrow => "Arrow",
        kExprNodeComparison => "Comparison",
        kExprNodeConcat => "Concat",
        kExprNodeConcatOrSubscript => "ConcatOrSubscript",
        kExprNodeInteger => "Integer",
        kExprNodeFloat => "Float",
        kExprNodeSingleQuotedString => "SingleQuotedString",
        kExprNodeDoubleQuotedString => "DoubleQuotedString",
        kExprNodeOr => "Or",
        kExprNodeAnd => "And",
        kExprNodeUnaryMinus => "UnaryMinus",
        kExprNodeBinaryMinus => "BinaryMinus",
        kExprNodeNot => "Not",
        kExprNodeMultiplication => "Multiplication",
        kExprNodeDivision => "Division",
        kExprNodeMod => "Mod",
        kExprNodeOption => "Option",
        kExprNodeEnvironment => "Environment",
        kExprNodeAssignment => "Assignment",
    }
    .as_ptr()
}

/// Function mapping ExprComparisonType values to their stringified versions
pub fn eltkn_cmp_type_tab(ecmp: ExprComparisonType) -> *const u8 {
    match ecmp {
        kExprCmpEqual => "Equal",
        kExprCmpMatches => "Matches",
        kExprCmpGreater => "Greater",
        kExprCmpGreaterOrEqual => "GreaterOrEqual",
        kExprCmpIdentical => "Identical",
    }
    .as_ptr()
}

/// Function mapping ExprCaseCompareStrategy values to their stringified versions
pub fn ccs_tab(strat: ExprCaseCompareStrategy) -> *const u8 {
    match strat {
        kCCStrategyUseOption => "UseOption",
        kCCStrategyMatchCase => "MatchCase",
        kCCStrategyIgnoreCase => "IgnoreCase",
    }
    .as_ptr()
}

/// Function mapping ExprAssignmentType values to their stringified versions
pub fn expr_asgn_type_tab(asgn: ExprAssignmentType) -> *const u8 {
    match asgn {
        kExprAsgnPlain => "Plain",
        kExprAsgnAdd => "Add",
        kExprAsgnSubtract => "Subtract",
        kExprAsgnConcat => "Concat",
    }
    .as_ptr()
}

type ExprASTStack = kvec_withinit_t!(*mut *mut ExprASTNode, 16);

/// Which nodes may be wanted
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
enum ExprASTWantedNode {
    /// Operators: function call, subscripts, binary operators, …
    ///
    /// For unrestricted expressions.
    kENodeOperator,
    /// Values: literals, variables, nested expressions, unary operators.
    ///
    /// For unrestricted expressions as well, implies that top item in AST stack
    /// points to NULL.
    kENodeValue,
}
use ExprASTWantedNode::*;

/// Parse type: what is being parsed currently
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
enum ExprASTParseType {
    /// Parsing regular VimL expression
    kEPTExpr = 0,
    /// Parsing lambda arguments
    ///
    /// Just like parsing function arguments, but it is valid to be ended with an
    /// arrow only.
    kEPTLambdaArguments,
    /// Assignment: parsing for :let
    kEPTAssignment,
    /// Single assignment: used when lists are not allowed (i.e. when nesting)
    kEPTSingleAssignment,
}
use ExprASTParseType::*;

type ExprASTParseTypeStack = kvec_withinit_t!(ExprASTParseType, 4);

/// Operator priority level
#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum ExprOpLvl {
    kEOpLvlInvalid = 0,
    kEOpLvlComplexIdentifier,
    kEOpLvlParens,
    kEOpLvlAssignment,
    kEOpLvlArrow,
    kEOpLvlComma,
    kEOpLvlColon,
    kEOpLvlTernaryValue,
    kEOpLvlTernary,
    kEOpLvlOr,
    kEOpLvlAnd,
    kEOpLvlComparison,
    kEOpLvlAddition,       //< Addition, subtraction and concatenation.
    kEOpLvlMultiplication, //< Multiplication, division and modulo.
    kEOpLvlUnary,          //< Unary operations: not, minus, plus.
    kEOpLvlSubscript,      //< Subscripts.
    kEOpLvlValue,          //< Values: literals, variables, nested expressions, …
}
use ExprOpLvl::*;

/// Operator associativity
#[derive(Copy, Clone, PartialEq)]
enum ExprOpAssociativity {
    kEOpAssNo,    //< Not associative / not applicable.
    kEOpAssLeft,  //< Left associativity.
    kEOpAssRight, //< Right associativity.
}
use ExprOpAssociativity::*;

/// Character used as a separator in autoload function/variable names.
const AUTOLOAD_CHAR: char = '#';

/// Scale number by a given factor
///
/// Used to apply exponent to a number. Idea taken from uClibc.
///
/// @param[in]  num  Number to scale. Does not bother doing anything if it is
///                  zero.
/// @param[in]  base  Base, should be 10 since non-decimal floating-point
///                   numbers are not supported.
/// @param[in]  exponent  Exponent to scale by.
/// @param[in]  exponent_negative  True if exponent is negative.
#[inline(always)]
#[must_use]
unsafe fn scale_number(
    num: float_T,
    base: u8,
    exponent: uvarnumber_T,
    exponent_negative: bool,
) -> float_T {
    if num == 0.0 || exponent == 0 {
        return num;
    }
    assert!(base != 0);
    let mut exp = exponent;
    let mut p_base = base as float_T;
    let mut ret = num;
    while exp != 0 {
        if (exp & 1) != 0 {
            if exponent_negative {
                ret /= p_base
            } else {
                ret *= p_base
            }
        }
        exp >>= 1;
        p_base *= p_base
    }
    return ret;
}

/// Get next token for the VimL expression input
///
/// @param  pstate  Parser state.
/// @param[in]  flags  Flags, @see LexExprFlags.
///
/// @return Next token.
#[no_mangle]
#[must_use]
pub unsafe fn viml_pexpr_next_token(pstate: &mut ParserState, flags: LexExprFlags) -> LexExprToken {
    let mut ret = {
        let mut init = LexExprToken::default();
        init.type_0 = kExprLexInvalid;
        init.start = pstate.pos;
        init
    };
    let mut pline = ParserLine::default();
    if !viml_parser_get_remaining_line(pstate, &mut pline) {
        ret.type_0 = kExprLexEOC;
        return ret;
    }
    if pline.size <= 0 {
        ret.len = 0;
        ret.type_0 = kExprLexEOC
    } else {
        ret.len = 1;
        let schar = *pline.data.offset(0) as char;
        macro_rules! BRACKET {
            ($typ: expr, $clsing: literal) => {{
                ret.type_0 = $typ;
                ret.data.brc.closing = schar == $clsing;
            }};
        }
        macro_rules! CHARREG {
            ($typ: expr, $cond: ident) => {{
                ret.type_0 = $typ;
                while ret.len < pline.size && $cond(*pline.data.offset(ret.len as isize) as char) {
                    ret.len += 1;
                }
            }};
        }
        macro_rules! GET_CCS {
            ($ret: expr, $pline: expr) => {{
                if $ret.len < $pline.size
                    && !strchr(
                        b"?#\x00" as *const u8 as *const libc::c_char,
                        *$pline.data.offset($ret.len as isize) as libc::c_int,
                    )
                    .is_null()
                {
                    $ret.data.cmp.ccs = (*$pline.data.offset($ret.len as isize)).into();
                    $ret.len += 1;
                } else {
                    $ret.data.cmp.ccs = kCCStrategyUseOption;
                }
            }};
        }
        macro_rules! CHAR_OR_ASSIGN {
            ($ch_type: expr, $ass_type: expr) => {{
                if pline.size > 1 && *pline.data.offset(1) as char == '=' {
                    ret.len += 1;
                    ret.type_0 = kExprLexAssignment;
                    ret.data.ass.type_0 = $ass_type;
                } else {
                    ret.type_0 = $ch_type;
                }
            }};
        }
        match schar {
            // Paired brackets.
            '(' | ')' => BRACKET!(kExprLexParenthesis, ')'),
            '[' | ']' => BRACKET!(kExprLexBracket, ']'),
            '{' | '}' => BRACKET!(kExprLexFigureBrace, '}'),

            // Single character tokens without data.
            '?' => ret.type_0 = kExprLexQuestion,
            ':' => ret.type_0 = kExprLexColon,
            ',' => ret.type_0 = kExprLexComma,

            // Multiplication/division/modulo.
            '*' => {
                ret.type_0 = kExprLexMultiplication;
                ret.data.mul.type_0 = kExprLexMulMul
            }
            '/' => {
                ret.type_0 = kExprLexMultiplication;
                ret.data.mul.type_0 = kExprLexMulDiv
            }
            '%' => {
                ret.type_0 = kExprLexMultiplication;
                ret.data.mul.type_0 = kExprLexMulMod
            }

            // Whitespace.
            ' ' | '\t' => CHARREG!(kExprLexSpacing, ascii_iswhite),

            // Control character, except for NUL, NL and TAB.
            Ctrl_A | Ctrl_B | Ctrl_C | Ctrl_D | Ctrl_E | Ctrl_F | Ctrl_G | Ctrl_H | Ctrl_K
            | Ctrl_L | Ctrl_M | Ctrl_N | Ctrl_O | Ctrl_P | Ctrl_Q | Ctrl_R | Ctrl_S | Ctrl_T
            | Ctrl_U | Ctrl_V | Ctrl_W | Ctrl_X | Ctrl_Y | Ctrl_Z => {
                let is_ctrl = |s| s < ' ';
                CHARREG!(kExprLexInvalid, is_ctrl);
                ret.data.err.type_0 = kExprLexSpacing;
                ret.data.err.msg = gettext("E15: Invalid control character present in input: %.*s")
            }

            // Number.
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                ret.data.num.is_float = false;
                ret.data.num.base = 10;
                let mut frac_start = 0;
                let mut exp_start = 0;
                let mut frac_end = 0;
                let mut exp_negative = false;
                CHARREG!(kExprLexNumber, ascii_isdigit);
                if flags.contains(LexExprFlags::AllowFloat) {
                    let non_float_ret = ret;
                    if pline.size > ret.len + 1
                        && *pline.data.offset(ret.len as isize) as char == '.'
                        && ascii_isdigit(*pline.data.offset((ret.len + 1) as isize) as char)
                    {
                        ret.len += 1;
                        frac_start = ret.len;
                        frac_end = ret.len;
                        ret.data.num.is_float = true;
                        while ret.len < pline.size
                            && ascii_isdigit(*pline.data.offset(ret.len as isize) as char)
                        {
                            // A small optimization: trailing zeroes in fractional part do not
                            // add anything to significand, so it is useless to include them in
                            // frac_end.
                            if *pline.data.offset(ret.len as isize) as char != '0' {
                                frac_end = ret.len + 1;
                            }
                            ret.len += 1;
                        }
                        if pline.size > ret.len + 1
                            && ['e', 'E'].contains(&(*pline.data.offset(ret.len as isize) as char))
                            && (pline.size > ret.len + 2
                                && ['+', '-'].contains(
                                    &(*pline.data.offset((ret.len + 1) as isize) as char),
                                )
                                && ascii_isdigit(*pline.data.offset((ret.len + 2) as isize) as char)
                                || ascii_isdigit(*pline.data.offset((ret.len + 1) as isize) as char))
                        {
                            ret.len += 1;
                            if *pline.data.offset(ret.len as isize) as char == '+' || {
                                exp_negative = *pline.data.offset(ret.len as isize) as char == '-';
                                exp_negative
                            } {
                                ret.len += 1;
                            }
                            exp_start = ret.len;
                            CHARREG!(kExprLexNumber, ascii_isdigit);
                        }
                    }
                    if pline.size > ret.len
                        && (*pline.data.offset(ret.len as isize) as char == '.'
                            || ASCII_ISALPHA(*pline.data.offset(ret.len as isize) as char))
                    {
                        ret = non_float_ret
                    }
                }
                // TODO(ZyX-I): detect overflows
                if ret.data.num.is_float {
                    // Vim used to use string2float here which in turn uses strtod(). There
                    // are two problems with this approach:
                    // 1. strtod() is locale-dependent. Not sure how it is worked around so
                    //    that I do not see relevant bugs, but it still does not look like
                    //    a good idea.
                    // 2. strtod() does not accept length argument.
                    //
                    // The below variant of parsing floats was recognized as acceptable
                    // because it is basically how uClibc does the thing: it generates
                    // a number ignoring decimal point (but recording its position), then
                    // uses recorded position to scale number down when processing exponent.
                    let mut significand_part = 0.0;
                    let mut exp_part: uvarnumber_T = 0;
                    let frac_size = frac_end - frac_start;
                    for i in 0..frac_end {
                        if i == frac_start - 1 {
                            continue;
                        }
                        significand_part = significand_part * 10.0
                            + (*pline.data.offset(i as isize) - '0' as u8) as f64
                    }
                    if exp_start != 0 {
                        vim_str2nr(
                            pline.data.offset(exp_start as isize) as *const u8,
                            ptr::null_mut(),
                            ptr::null_mut(),
                            0,
                            ptr::null_mut(),
                            &mut exp_part,
                            (ret.len - exp_start) as libc::c_int,
                        );
                    }
                    if exp_negative {
                        exp_part += frac_size as u64;
                    } else if exp_part < frac_size as u64 {
                        exp_negative = true;
                        exp_part = frac_size as u64 - exp_part;
                    } else {
                        exp_part -= frac_size as u64;
                    }
                    ret.data.num.val.floating =
                        scale_number(significand_part, 10, exp_part, exp_negative);
                } else {
                    let mut len: libc::c_int = 0;
                    let mut prep: libc::c_int = 0;
                    vim_str2nr(
                        pline.data as *const libc::c_uchar,
                        &mut prep,
                        &mut len,
                        STR2NR_ALL,
                        ptr::null_mut(),
                        &mut ret.data.num.val.integer,
                        pline.size as libc::c_int,
                    );
                    ret.len = len as libc::size_t;
                    ret.data.num.base = match prep as u8 as char {
                        '\u{0}' => 10,
                        '0' => 8,
                        'x' | 'X' => 16,
                        'b' | 'B' => 2,
                        _ => panic!("unknown base"),
                    };
                }
            }

            // Environment variable.
            '$' => {
                CHARREG!(kExprLexEnv, ascii_isident);
            }

            // Normal variable/function name.
            'a'..='z' | 'A'..='Z' | '_' => {
                ret.data.var.scope = kExprVarScopeMissing;
                ret.data.var.autoload = false;
                CHARREG!(kExprLexPlainIdentifier, ascii_isident);
                let ISWORD_OR_AUTOLOAD = |x| ascii_isident(x) || x == AUTOLOAD_CHAR;
                // "is" and "isnot" operators.
                if !flags.contains(LexExprFlags::IsNotCmp)
                    && (ret.len == 2 && memcmp(pline.data, b"is\x00" as *const u8, 2) == 0
                        || ret.len == 5 && memcmp(pline.data, b"isnot\x00" as *const u8, 5) == 0)
                {
                    ret.type_0 = kExprLexComparison;
                    ret.data.cmp.type_0 = kExprCmpIdentical;
                    ret.data.cmp.inv = ret.len == 5;
                    GET_CCS!(ret, pline);
                // Scope: `s:`, etc.
                } else if ret.len == 1
                    && pline.size > 1
                    && !memchr(
                        EXPR_VAR_SCOPE_LIST.as_ptr(),
                        schar,
                        var_size(EXPR_VAR_SCOPE_LIST),
                    )
                    .is_null()
                    && *pline.data.offset(ret.len as isize) as char == ':'
                    && !flags.contains(LexExprFlags::ForbidScope)
                {
                    ret.len += 1;
                    ret.data.var.scope = schar.into();
                    CHARREG!(kExprLexPlainIdentifier, ISWORD_OR_AUTOLOAD);
                    ret.data.var.autoload =
                        !memchr(pline.data.offset(2), AUTOLOAD_CHAR, ret.len - 2).is_null()
                // Previous CHARREG stopped at autoload character in order to make it
                // possible to detect `is#`. Continue now with autoload characters
                // included.
                //
                // Warning: there is ambiguity for the lexer: `is#Foo(1)` is a call of
                // function `is#Foo()`, `1is#Foo(1)` is a comparison `1 is# Foo(1)`. This
                // needs to be resolved on the higher level where context is available.
                } else if pline.size > ret.len
                    && *pline.data.offset(ret.len as isize) as char == AUTOLOAD_CHAR
                {
                    ret.data.var.autoload = true;
                    CHARREG!(kExprLexPlainIdentifier, ISWORD_OR_AUTOLOAD);
                }
            }

            // Option.
            '&' => {
                let OPTNAMEMISS = |ret: &mut LexExprToken| {
                    ret.type_0 = kExprLexInvalid;
                    ret.data.err.type_0 = kExprLexOption;
                    ret.data.err.msg = gettext("E112: Option name missing: %.*s")
                };
                if pline.size > 1 && *pline.data.offset(1) as char == '&' {
                    ret.type_0 = kExprLexAnd;
                    ret.len += 1;
                } else if pline.size == 1 || !ASCII_ISALPHA(*pline.data.offset(1) as char) {
                    OPTNAMEMISS(&mut ret);
                } else {
                    ret.type_0 = kExprLexOption;
                    if pline.size > 2
                        && *pline.data.offset(2) as char == ':'
                        && !memchr(
                            EXPR_OPT_SCOPE_LIST.as_ptr(),
                            *pline.data.offset(1),
                            var_size(EXPR_OPT_SCOPE_LIST),
                        )
                        .is_null()
                    {
                        ret.len += 2;
                        ret.data.opt.scope = (*pline.data.offset(1)).into();
                        ret.data.opt.name = pline.data.offset(3)
                    } else {
                        ret.data.opt.scope = kExprOptScopeUnspecified;
                        ret.data.opt.name = pline.data.offset(1)
                    }
                    let mut p = ret.data.opt.name;
                    let e = pline.data.offset(pline.size as isize);
                    if e.offset_from(p) >= 4
                        && *p.offset(0) as char == 't'
                        && *p.offset(1) as char == '_'
                    {
                        ret.data.opt.len = 4;
                        ret.len += 4;
                    } else {
                        while p < e && ASCII_ISALPHA(*p as char) {
                            p = p.offset(1)
                        }
                        ret.data.opt.len = p.offset_from(ret.data.opt.name) as libc::size_t;
                        if ret.data.opt.len == 0 {
                            OPTNAMEMISS(&mut ret);
                        } else {
                            ret.len += ret.data.opt.len;
                        }
                    }
                }
            }

            // Register.
            '@' => {
                ret.type_0 = kExprLexRegister;
                if pline.size > 1 {
                    ret.len += 1;
                    ret.data.reg.name = *pline.data.offset(1) as libc::c_int
                } else {
                    ret.data.reg.name = -1
                }
            }

            // Single quoted string.
            '\'' => {
                ret.type_0 = kExprLexSingleQuotedString;
                ret.data.str_0.closed = false;
                while ret.len < pline.size && !ret.data.str_0.closed {
                    if *pline.data.offset(ret.len as isize) as char == '\'' {
                        if ret.len + 1 < pline.size
                            && *pline.data.offset((ret.len + 1) as isize) as char == '\''
                        {
                            ret.len += 1;
                        } else {
                            ret.data.str_0.closed = true;
                        }
                    }
                    ret.len += 1;
                }
            }

            // Double quoted string.
            '"' => {
                ret.type_0 = kExprLexDoubleQuotedString;
                ret.data.str_0.closed = false;
                while ret.len < pline.size && !ret.data.str_0.closed {
                    if *pline.data.offset(ret.len as isize) as char == '\\' {
                        if ret.len + 1 < pline.size {
                            ret.len += 1;
                        }
                    } else if *pline.data.offset(ret.len as isize) as char == '\"' {
                        ret.data.str_0.closed = true;
                    }
                    ret.len += 1;
                }
            }

            // Unary not, (un)equality and regex (not) match comparison operators.
            '!' | '=' => {
                if pline.size == 1 {
                    ret.type_0 = if schar == '!' {
                        kExprLexNot
                    } else {
                        kExprLexAssignment
                    };
                    ret.data.ass.type_0 = kExprAsgnPlain
                } else {
                    ret.type_0 = kExprLexComparison;
                    ret.data.cmp.inv = schar == '!';
                    if *pline.data.offset(1) as char == '=' {
                        ret.data.cmp.type_0 = kExprCmpEqual;
                        ret.len += 1;
                    } else if *pline.data.offset(1) as char == '~' {
                        ret.data.cmp.type_0 = kExprCmpMatches;
                        ret.len += 1;
                    } else if schar == '!' {
                        ret.type_0 = kExprLexNot
                    } else {
                        ret.type_0 = kExprLexAssignment;
                        ret.data.ass.type_0 = kExprAsgnPlain
                    }
                    GET_CCS!(ret, pline);
                }
            }

            // Less/greater [or equal to] comparison operators.
            '>' | '<' => {
                ret.type_0 = kExprLexComparison;
                let haseqsign = pline.size > 1 && *pline.data.offset(1) as char == '=';
                if haseqsign {
                    ret.len += 1;
                }
                GET_CCS!(ret, pline);
                ret.data.cmp.inv = schar == '<';
                ret.data.cmp.type_0 = if ret.data.cmp.inv ^ haseqsign {
                    kExprCmpGreaterOrEqual
                } else {
                    kExprCmpGreater
                }
            }

            // Minus sign, arrow from lambdas or augmented assignment.
            '-' => {
                if pline.size > 1 && *pline.data.offset(1) as char == '>' {
                    ret.len += 1;
                    ret.type_0 = kExprLexArrow
                } else if pline.size > 1 && *pline.data.offset(1) as char == '=' {
                    ret.len += 1;
                    ret.type_0 = kExprLexAssignment;
                    ret.data.ass.type_0 = kExprAsgnSubtract
                } else {
                    ret.type_0 = kExprLexMinus
                }
            }

            // Sign or augmented assignment.
            '+' => CHAR_OR_ASSIGN!(kExprLexPlus, kExprAsgnAdd),
            '.' => CHAR_OR_ASSIGN!(kExprLexDot, kExprAsgnConcat),

            // Expression end because Ex command ended.
            '\u{0}' | '\n' => {
                if flags.contains(LexExprFlags::ForbidEOC) {
                    ret.type_0 = kExprLexInvalid;
                    ret.data.err.msg = gettext("E15: Unexpected EOC character: %.*s");
                    ret.data.err.type_0 = kExprLexSpacing
                } else {
                    ret.type_0 = kExprLexEOC
                }
            }

            '|' => {
                if pline.size >= 2 && *pline.data.offset(ret.len as isize) as char == '|' {
                    // "||" is or.
                    ret.len += 1;
                    ret.type_0 = kExprLexOr
                } else if flags.contains(LexExprFlags::ForbidEOC) {
                    // Note: `<C-r>=1 | 2<CR>` actually yields 1 in Vim without any
                    //       errors. This will be changed here.
                    ret.type_0 = kExprLexInvalid;
                    ret.data.err.msg = gettext("E15: Unexpected EOC character: %.*s");
                    ret.data.err.type_0 = kExprLexOr
                } else {
                    ret.type_0 = kExprLexEOC
                }
            }

            // Everything else is not valid.
            _ => {
                ret.len = utfc_ptr2len_len(pline.data, pline.size as libc::c_int) as libc::size_t;
                ret.type_0 = kExprLexInvalid;
                ret.data.err.type_0 = kExprLexPlainIdentifier;
                ret.data.err.msg = gettext("E15: Unidentified character: %.*s")
            }
        }
    }
    if !flags.contains(LexExprFlags::Peek) {
        viml_parser_advance(pstate, ret.len);
    }
    return ret;
}

fn eltkn_type_tab(typ: LexExprTokenType) -> *const u8 {
    match typ {
        kExprLexInvalid => "Invalid",
        kExprLexMissing => "Missing",
        kExprLexSpacing => "Spacing",
        kExprLexEOC => "EOC",

        kExprLexQuestion => "Question",
        kExprLexColon => "Colon",
        kExprLexOr => "Or",
        kExprLexAnd => "And",
        kExprLexComparison => "Comparison",
        kExprLexPlus => "Plus",
        kExprLexMinus => "Minus",
        kExprLexDot => "Dot",
        kExprLexMultiplication => "Multiplication",

        kExprLexNot => "Not",

        kExprLexNumber => "Number",
        kExprLexSingleQuotedString => "SingleQuotedString",
        kExprLexDoubleQuotedString => "DoubleQuotedString",
        kExprLexOption => "Option",
        kExprLexRegister => "Register",
        kExprLexEnv => "Env",
        kExprLexPlainIdentifier => "PlainIdentifier",

        kExprLexBracket => "Bracket",
        kExprLexFigureBrace => "FigureBrace",
        kExprLexParenthesis => "Parenthesis",
        kExprLexComma => "Comma",
        kExprLexArrow => "Arrow",
        kExprLexAssignment => "Assignment",
    }
    .as_ptr()
}

fn eltkn_mul_type_tab(mul_type: Let_data_mul_type) -> *const u8 {
    match mul_type {
        kExprLexMulMul => "Mul",
        kExprLexMulDiv => "Div",
        kExprLexMulMod => "Mod",
    }
    .as_ptr()
}

fn eltkn_opt_scope_tab(scope: ExprOptScope) -> *const u8 {
    match scope {
        kExprOptScopeUnspecified => "Unspecified",
        kExprOptScopeGlobal => "Global",
        kExprOptScopeLocal => "Local",
    }
    .as_ptr()
}

/// Represent token as a string
///
/// Intended for testing and debugging purposes.
///
/// @param[in]  pstate  Parser state, needed to get token string from it. May be
///                     NULL, in which case in place of obtaining part of the
///                     string represented by token only token length is
///                     returned.
/// @param[in]  token  Token to represent.
/// @param[out]  ret_size  Return string size, for cases like NULs inside
///                        a string. May be NULL.
///
/// @return Token represented in a string form, in a static buffer (overwritten
///         on each call).
#[must_use]
#[allow(dead_code)]
unsafe fn viml_pexpr_repr_token(
    pstate: Option<&ParserState>,
    token: LexExprToken,
    ret_size: *mut libc::size_t,
) -> *const libc::c_char {
    static mut ret: [libc::c_char; 1024] = [0; 1024];
    let mut p = ret.as_mut_ptr();
    let e: *const libc::c_char =
        (&mut *ret.as_mut_ptr().offset(1024) as *mut libc::c_char).offset(-1);

    macro_rules! ADDSTR {
        ($s: literal, $($e: expr),+ $(,)?) => {{
            let sn_len = snprintf(
        p,
        var_size(ret) as u64 -
            p.offset_from(ret.as_ptr()) as u64,
            $s.as_ptr() as *const i8,
        $($e),+
    );
            p = p.offset(sn_len as isize);
            if p >= e as *mut libc::c_char {
                return;
            }
        }}
    }
    (|| {
        ADDSTR!(
            b"%zu:%zu:%s\x00",
            token.start.line,
            token.start.col,
            eltkn_type_tab(token.type_0),
        );
        match token.type_0 {
            kExprLexComparison => ADDSTR!(
                b"(type=%s,ccs=%s,inv=%i)\x00",
                eltkn_cmp_type_tab(token.data.cmp.type_0),
                ccs_tab(token.data.cmp.ccs),
                token.data.cmp.inv as i32,
            ),
            kExprLexMultiplication => {
                ADDSTR!(b"(type=%s)\x00", eltkn_mul_type_tab(token.data.mul.type_0))
            }
            kExprLexAssignment => {
                ADDSTR!(b"(type=%s)\x00", expr_asgn_type_tab(token.data.ass.type_0))
            }
            kExprLexRegister => ADDSTR!(b"(name=%s)\x00", intchar2str(token.data.reg.name)),
            kExprLexDoubleQuotedString | kExprLexSingleQuotedString => {
                ADDSTR!(b"(closed=%i)\x00", token.data.str_0.closed as i32)
            }
            kExprLexOption => ADDSTR!(
                b"(scope=%s,name=%.*s)\x00",
                eltkn_opt_scope_tab(token.data.opt.scope),
                token.data.opt.len as i32,
                token.data.opt.name,
            ),
            kExprLexPlainIdentifier => ADDSTR!(
                b"(scope=%s,autoload=%i)\x00",
                intchar2str(token.data.var.scope as i32),
                token.data.var.autoload as i32,
            ),
            kExprLexNumber => ADDSTR!(
                b"(is_float=%i,base=%i,val=%lg)\x00",
                token.data.num.is_float as i32,
                token.data.num.base as i32,
                if token.data.num.is_float {
                    token.data.num.val.floating
                } else {
                    token.data.num.val.integer as f64
                },
            ),
            kExprLexInvalid => ADDSTR!(b"(msg=%s)\x00", token.data.err.msg),
            _ => (), // No additional arguments
        }
        if let Some(pstate) = pstate {
            *p = ':' as libc::c_char;
            p = p.offset(1);
            memmove(
                p as *mut libc::c_void,
                &*(*pstate.reader.lines.items.offset(token.start.line as isize))
                    .data
                    .offset(token.start.col as isize) as *const u8
                    as *const libc::c_void,
                token.len as u64,
            );
            p = p.offset(token.len as isize);
            *p = '\u{0}' as i32 as libc::c_char
        } else {
            ADDSTR!(b"::%zu\x00", token.len);
        }
    })();
    if !ret_size.is_null() {
        *ret_size = p.offset_from(ret.as_mut_ptr()) as libc::size_t
    }
    return ret.as_mut_ptr();
}

/// Represent `int` character as a string
///
/// Converts
/// - ASCII digits into '{digit}'
/// - ASCII printable characters into a single-character strings
/// - everything else to numbers.
///
/// @param[in]  ch  Character to convert.
///
/// @return Converted string, stored in a static buffer (overriden after each
///         call).
#[must_use]
unsafe fn intchar2str(ch: i32) -> *const u8 {
    static mut buf: Option<CString> = None;
    let result = if ' ' as i32 <= ch && ch < 0x7f {
        let ch = ch as u8 as char;
        if ascii_isdigit(ch) {
            format!("'{}'", ch)
        } else {
            format!("{}", ch)
        }
    } else {
        format!("{}", ch)
    };
    buf = Some(CString::new(result).expect("CString::new failed"));
    buf.as_ref().unwrap().as_ptr() as *const u8
}

/// Free memory occupied by AST
///
/// @param  ast  AST stack to free.
#[no_mangle]
pub unsafe extern "C" fn viml_pexpr_free_ast(mut ast: ExprAST) {
    let mut ast_stack = ExprASTStack::init();
    ast_stack.push(&mut ast.root);
    while !ast_stack.is_empty() {
        let cur_node = *ast_stack.last();
        if cfg!(debug_assertions) {
            // Explicitly check for AST recursiveness.
            for i in 0..(ast_stack.size() - 1) {
                assert!(**ast_stack.A(i) != *cur_node);
            }
        }
        if (*cur_node).is_null() {
            assert!(ast_stack.size() == 1);
            ast_stack.lop(1);
        } else if !(**cur_node).children.is_null() {
            if cfg!(debug_assertions) {
                let maxchildren = node_maxchildren((**cur_node).type_0);
                assert!(maxchildren > 0);
                assert!(maxchildren <= 2);
                assert!(if maxchildren == 1 {
                    (*(**cur_node).children).next == ptr::null_mut()
                } else {
                    (*(**cur_node).children).next.is_null()
                        || (*(*(**cur_node).children).next).next.is_null()
                })
            }
            ast_stack.push(&mut (**cur_node).children)
        } else if !(**cur_node).next.is_null() {
            ast_stack.push(&mut (**cur_node).next)
        } else if !(*cur_node).is_null() {
            ast_stack.lop(1);
            match (**cur_node).type_0 {
                kExprNodeDoubleQuotedString | kExprNodeSingleQuotedString => {
                    xfree((**cur_node).data.str_0.value);
                }
                kExprNodeMissing
                | kExprNodeOpMissing
                | kExprNodeTernary
                | kExprNodeTernaryValue
                | kExprNodeRegister
                | kExprNodeSubscript
                | kExprNodeListLiteral
                | kExprNodeUnaryPlus
                | kExprNodeBinaryPlus
                | kExprNodeNested
                | kExprNodeCall
                | kExprNodePlainIdentifier
                | kExprNodePlainKey
                | kExprNodeComplexIdentifier
                | kExprNodeUnknownFigure
                | kExprNodeLambda
                | kExprNodeDictLiteral
                | kExprNodeCurlyBracesIdentifier
                | kExprNodeAssignment
                | kExprNodeComma
                | kExprNodeColon
                | kExprNodeArrow
                | kExprNodeComparison
                | kExprNodeConcat
                | kExprNodeConcatOrSubscript
                | kExprNodeInteger
                | kExprNodeFloat
                | kExprNodeOr
                | kExprNodeAnd
                | kExprNodeUnaryMinus
                | kExprNodeBinaryMinus
                | kExprNodeNot
                | kExprNodeMultiplication
                | kExprNodeDivision
                | kExprNodeMod
                | kExprNodeOption
                | kExprNodeEnvironment => {}
            }
            xfree(*cur_node);
            *cur_node = ptr::null_mut()
        }
    }
    ast_stack.destroy();
}

// Binary operator precedence and associativity:
//
// Operator | Precedence | Associativity
// ---------+------------+-----------------
// ||       | 2          | left
// &&       | 3          | left
// cmp*     | 4          | not associative
// + - .    | 5          | left
// * / %    | 6          | left
//
// * comparison operators:
//
// == ==# ==?  != !=# !=?
// =~ =~# =~?  !~ !~# !~?
//  >  >#  >?  <= <=# <=?
//  <  <#  <?  >= >=# >=?
// is is# is?  isnot isnot# isnot?

/// Allocate a new node and set some of the values
///
/// @param[in]  type  Node type to allocate.
/// @param[in]  level  Node level to allocate
#[inline]
#[must_use]
unsafe fn viml_pexpr_new_node(type_0: ExprASTNodeType) -> *mut ExprASTNode {
    let mut ret = xmalloc(mem::size_of::<ExprASTNode>() as usize) as *mut ExprASTNode;
    (*ret).type_0 = type_0;
    (*ret).children = ptr::null_mut();
    (*ret).next = ptr::null_mut();
    return ret;
}

#[derive(Copy, Clone)]
struct NodeProps {
    lvl: ExprOpLvl,
    ass: ExprOpAssociativity,
}
fn node_type_to_node_props(typ: ExprASTNodeType) -> NodeProps {
    let (lvl, ass) = match typ {
        kExprNodeMissing => (kEOpLvlInvalid, kEOpAssNo),
        kExprNodeOpMissing => (kEOpLvlMultiplication, kEOpAssNo),

        kExprNodeNested => (kEOpLvlParens, kEOpAssNo),
        // Note: below nodes are kEOpLvlSubscript for “binary operator” itself, but
        //       kEOpLvlParens when it comes to inside the parenthesis.
        kExprNodeCall => (kEOpLvlParens, kEOpAssNo),
        kExprNodeSubscript => (kEOpLvlParens, kEOpAssNo),

        kExprNodeUnknownFigure => (kEOpLvlParens, kEOpAssLeft),
        kExprNodeLambda => (kEOpLvlParens, kEOpAssNo),
        kExprNodeDictLiteral => (kEOpLvlParens, kEOpAssNo),
        kExprNodeListLiteral => (kEOpLvlParens, kEOpAssNo),

        kExprNodeArrow => (kEOpLvlArrow, kEOpAssNo),

        // Right associativity for comma because this means easier access to arguments
        // list, etc: for "[a, b, c, d]" you can access "a" in one step if it is
        // represented as "list(comma(a, comma(b, comma(c, d))))" then if it is
        // "list(comma(comma(comma(a, b), c), d))" in which case you will need to
        // traverse all three comma() structures. And with comma operator (including
        // actual comma operator from C which is not present in VimL) nobody cares
        // about associativity, only about order of execution.
        kExprNodeComma => (kEOpLvlComma, kEOpAssRight),

        // Colons are not eligible for chaining, so nobody cares about associativity.
        kExprNodeColon => (kEOpLvlColon, kEOpAssNo),

        kExprNodeTernary => (kEOpLvlTernary, kEOpAssRight),

        kExprNodeOr => (kEOpLvlOr, kEOpAssLeft),

        kExprNodeAnd => (kEOpLvlAnd, kEOpAssLeft),

        kExprNodeTernaryValue => (kEOpLvlTernaryValue, kEOpAssRight),

        kExprNodeComparison => (kEOpLvlComparison, kEOpAssRight),

        kExprNodeBinaryPlus => (kEOpLvlAddition, kEOpAssLeft),
        kExprNodeBinaryMinus => (kEOpLvlAddition, kEOpAssLeft),
        kExprNodeConcat => (kEOpLvlAddition, kEOpAssLeft),

        kExprNodeMultiplication => (kEOpLvlMultiplication, kEOpAssLeft),
        kExprNodeDivision => (kEOpLvlMultiplication, kEOpAssLeft),
        kExprNodeMod => (kEOpLvlMultiplication, kEOpAssLeft),

        kExprNodeUnaryPlus => (kEOpLvlUnary, kEOpAssNo),
        kExprNodeUnaryMinus => (kEOpLvlUnary, kEOpAssNo),
        kExprNodeNot => (kEOpLvlUnary, kEOpAssNo),

        kExprNodeConcatOrSubscript => (kEOpLvlSubscript, kEOpAssLeft),

        kExprNodeCurlyBracesIdentifier => (kEOpLvlComplexIdentifier, kEOpAssLeft),

        kExprNodeAssignment => (kEOpLvlAssignment, kEOpAssLeft),

        kExprNodeComplexIdentifier => (kEOpLvlValue, kEOpAssLeft),

        kExprNodePlainIdentifier => (kEOpLvlValue, kEOpAssNo),
        kExprNodePlainKey => (kEOpLvlValue, kEOpAssNo),
        kExprNodeRegister => (kEOpLvlValue, kEOpAssNo),
        kExprNodeInteger => (kEOpLvlValue, kEOpAssNo),
        kExprNodeFloat => (kEOpLvlValue, kEOpAssNo),
        kExprNodeDoubleQuotedString => (kEOpLvlValue, kEOpAssNo),
        kExprNodeSingleQuotedString => (kEOpLvlValue, kEOpAssNo),
        kExprNodeOption => (kEOpLvlValue, kEOpAssNo),
        kExprNodeEnvironment => (kEOpLvlValue, kEOpAssNo),
    };
    NodeProps { lvl: lvl, ass: ass }
}

/// Get AST node priority level
///
/// Used primary to reduce line length, so keep the name short.
///
/// @param[in]  node  Node to get priority for.
///
/// @return Node priority level.
#[inline(always)]
#[must_use]
fn node_lvl(node: ExprASTNode) -> ExprOpLvl {
    return node_type_to_node_props(node.type_0).lvl;
}

/// Get AST node associativity, to be used for operator nodes primary
///
/// Used primary to reduce line length, so keep the name short.
///
/// @param[in]  node  Node to get priority for.
///
/// @return Node associativity.
#[inline(always)]
#[must_use]
fn node_ass(node: ExprASTNode) -> ExprOpAssociativity {
    return node_type_to_node_props(node.type_0).ass;
}

/// Handle binary operator
///
/// This function is responsible for handling priority levels as well.
///
/// @param[in]  pstate  Parser state, used for error reporting.
/// @param  ast_stack  AST stack. May be popped of some values and will
///                    definitely receive new ones.
/// @param  bop_node  New node to handle.
/// @param[out]  want_node_p  New value of want_node.
/// @param[out]  ast_err  Location where error is saved, if any.
///
/// @return True if no errors occurred, false otherwise.
unsafe fn viml_pexpr_handle_bop(
    pstate: &ParserState,
    ast_stack: &mut ExprASTStack,
    bop_node: &mut ExprASTNode,
    want_node_p: &mut ExprASTWantedNode,
    ast_err: &mut ExprASTError,
) -> bool {
    let mut ret = true;
    let mut top_node_p: *mut *mut ExprASTNode = ptr::null_mut();
    let mut top_node = ptr::null_mut();
    let mut top_node_lvl = kEOpLvlInvalid;
    let mut top_node_ass = kEOpAssNo;
    assert!(!ast_stack.is_empty());
    let bop_node_lvl = if bop_node.type_0 == kExprNodeCall || bop_node.type_0 == kExprNodeSubscript
    {
        kEOpLvlSubscript
    } else {
        node_lvl(*bop_node)
    };
    let bop_node_ass = if bop_node.type_0 == kExprNodeCall || bop_node.type_0 == kExprNodeSubscript
    {
        kEOpAssLeft
    } else {
        node_ass(*bop_node)
    };
    loop {
        let new_top_node_p = *ast_stack.last();
        let new_top_node = *new_top_node_p;
        assert!(!new_top_node.is_null());
        let new_top_node_lvl = node_lvl(*new_top_node);
        let new_top_node_ass = node_ass(*new_top_node);
        assert!(bop_node_lvl != new_top_node_lvl || bop_node_ass == new_top_node_ass);
        if !top_node_p.is_null()
            && (bop_node_lvl > new_top_node_lvl
                || bop_node_lvl == new_top_node_lvl && new_top_node_ass == kEOpAssNo)
        {
            break;
        }
        ast_stack.lop(1);
        top_node_p = new_top_node_p;
        top_node = new_top_node;
        top_node_lvl = new_top_node_lvl;
        top_node_ass = new_top_node_ass;
        if bop_node_lvl == top_node_lvl && top_node_ass == kEOpAssRight {
            break;
        }
        if ast_stack.is_empty() {
            break;
        }
    }
    if top_node_ass == kEOpAssLeft || top_node_lvl != bop_node_lvl {
        // outer(op(x,y)) -> outer(new_op(op(x,y),*))
        //
        // Before: top_node_p = outer(*), points to op(x,y)
        //         Other stack elements unknown
        //
        // After: top_node_p = outer(*), points to new_op(op(x,y))
        //        &bop_node->children->next = new_op(op(x,y),*), points to NULL
        *top_node_p = bop_node;
        bop_node.children = top_node;
        assert!((*bop_node.children).next.is_null());
        ast_stack.push(top_node_p);
        ast_stack.push(&mut (*bop_node.children).next);
    } else {
        assert!(top_node_lvl == bop_node_lvl && top_node_ass == kEOpAssRight);
        assert!(!(*top_node).children.is_null() && !(*(*top_node).children).next.is_null());
        // outer(op(x,y)) -> outer(op(x,new_op(y,*)))
        //
        // Before: top_node_p = outer(*), points to op(x,y)
        //         Other stack elements unknown
        //
        // After: top_node_p = outer(*), points to op(x,new_op(y))
        //        &top_node->children->next = op(x,*), points to new_op(y)
        //        &bop_node->children->next = new_op(y,*), points to NULL
        bop_node.children = (*(*top_node).children).next;
        (*(*top_node).children).next = bop_node;
        assert!((*bop_node.children).next.is_null());
        ast_stack.push(top_node_p);
        ast_stack.push(&mut (*(*top_node).children).next);
        ast_stack.push(&mut (*bop_node.children).next);
        // TODO(ZyX-I): Make this not error, but treat like Python does
        if bop_node.type_0 == kExprNodeComparison {
            east_set_error(
                pstate,
                ast_err,
                gettext("E15: Operator is not associative: %.*s"),
                bop_node.start,
            );
            ret = false
        }
    }
    *want_node_p = kENodeValue;
    return ret;
}

/// ParserPosition literal based on ParserPosition pos with columns shifted
///
/// Function does not check whether resulting position is valid.
///
/// @param[in]  pos  Position to shift.
/// @param[in]  shift  Number of bytes to shift.
///
/// @return Shifted position.
#[inline(always)]
#[must_use]
fn shifted_pos(pos: ParserPosition, shift: libc::size_t) -> ParserPosition {
    ParserPosition {
        line: pos.line,
        col: pos.col + shift,
    }
}

/// ParserPosition literal based on ParserPosition pos with specified column
///
/// Function does not check whether remaining position is valid.
///
/// @param[in]  pos  Position to adjust.
/// @param[in]  new_col  New column.
///
/// @return Shifted position.
#[inline(always)]
#[must_use]
fn recol_pos(pos: ParserPosition, new_col: libc::size_t) -> ParserPosition {
    ParserPosition {
        line: pos.line,
        col: new_col,
    }
}

/// Set AST error, unless AST already is not correct
///
/// @param[out]  ret_ast  AST to set error in.
/// @param[in]  pstate  Parser state, used to get error message argument.
/// @param[in]  msg  Error message, assumed to be already translated and
///                  containing a single %token "%.*s".
/// @param[in]  start  Position at which error occurred.
#[inline(always)]
unsafe fn east_set_error(
    pstate: &ParserState,
    ret_ast_err: &mut ExprASTError,
    msg: *const u8,
    start: ParserPosition,
) {
    if !ret_ast_err.msg.is_null() {
        return;
    }
    let pline = *pstate.reader.lines.items.offset(start.line as isize);
    ret_ast_err.msg = msg;
    ret_ast_err.arg_len = (pline.size - start.col) as libc::c_int;
    ret_ast_err.arg = pline.data.offset(start.col as isize);
}

/// Determine whether given parse type is an assignment
///
/// @param[in]  pt  Checked parse type.
///
/// @return true if parsing an assignment, false otherwise.
#[inline(always)]
#[must_use]
unsafe fn pt_is_assignment(pt: ExprASTParseType) -> bool {
    pt == kEPTAssignment || pt == kEPTSingleAssignment
}

/// Structure used to define “string shifts” necessary to map string
/// highlighting to actual strings.
#[derive(Copy, Clone)]
#[repr(C)]
struct StringShift {
    start: libc::size_t,    //< Where special character starts in original string.
    orig_len: libc::size_t, //< Length of orininal string (e.g. 4 for "\x80").
    act_len: libc::size_t,  //< Length of resulting character(s) (e.g. 1 for "\x80").
    escape_not_known: bool, //< True if escape sequence in original is not known.
}
macro_rules! GEN_HL {
    ($is_invalid: ident) => {
        /// Get highlight group name
        macro_rules! HL {
                    ($g: literal) => {
                        if $is_invalid {
                            byte_strings::concat_bytes!(b"NvimInvalid", $g, b"\x00") as *const u8
                        } else {
                            byte_strings::concat_bytes!(b"Nvim" , $g, b"\x00") as *const u8
                        } as *const libc::c_char
                    };
                    ($g1: literal, $g2: literal) => {
                        if $is_invalid {
                            byte_strings::concat_bytes!(b"NvimInvalid", $g1, $g2, b"\x00") as *const u8
                        } else {
                            byte_strings::concat_bytes!(b"Nvim", $g1, $g2, b"\x00") as *const u8
                        } as *const libc::c_char
                    };
                }
    };
}

/// Parse and highlight single- or double-quoted string
///
/// Function is supposed to detect and highlight regular expressions (but does
/// not do now).
///
/// @param[out]  pstate  Parser state which also contains a place where
///                      highlighting is saved.
/// @param[out]  node  Node where string parsing results are saved.
/// @param[in]  token  Token to highlight.
/// @param[in]  ast_stack  Parser AST stack, used to detect whether current
///                        string is a regex.
/// @param[in]  is_invalid  Whether currently processed token is not valid.
unsafe fn parse_quoted_string(
    pstate: &mut ParserState,
    node: &mut ExprASTNode,
    token: LexExprToken,
    is_invalid: bool,
) {
    GEN_HL!(is_invalid);
    let pline = *pstate.reader.lines.items.offset(token.start.line as isize);
    let s = pline.data.offset(token.start.col as isize);
    let e = s
        .offset(token.len as isize)
        .offset(-(token.data.str_0.closed as isize));
    let mut p = s.offset(1);
    let is_double = token.type_0 == kExprLexDoubleQuotedString;
    let mut size = token.len - (token.data.str_0.closed as usize) - 1;
    type ShiftsVec = kvec_withinit_t!(StringShift, 16);
    let mut shifts = ShiftsVec::init();
    if !is_double {
        viml_parser_highlight(pstate, token.start, 1, HL!(b"SingleQuote"));
        while p < e {
            let chunk_e = memchr(p, '\'', e.offset_from(p) as u64) as *const u8;
            if chunk_e.is_null() {
                break;
            }
            size -= 1;
            p = chunk_e.offset(2);
            if !pstate.colors.is_null() {
                shifts.push(StringShift {
                    start: token.start.col + (chunk_e.offset_from(s) as libc::size_t),
                    orig_len: 2,
                    act_len: 1,
                    escape_not_known: false,
                });
            }
        }
        node.data.str_0.size = size;
        if size == 0 {
            node.data.str_0.value = ptr::null_mut()
        } else {
            node.data.str_0.value = xmallocz(size) as *mut u8;
            let mut v_p = node.data.str_0.value;
            p = s.offset(1);
            while p < e {
                let chunk_e_0 = memchr(p, '\'', e.offset_from(p) as u64) as *const u8;
                if chunk_e_0.is_null() {
                    memcpy(v_p, p, e.offset_from(p) as libc::size_t);
                    break;
                } else {
                    memcpy(v_p, p, chunk_e_0.offset_from(p) as libc::size_t);
                    v_p = v_p.offset(chunk_e_0.offset_from(p) as isize + 1);
                    *v_p.offset(-1) = '\'' as u8;
                    p = chunk_e_0.offset(2)
                }
            }
        }
    } else {
        viml_parser_highlight(pstate, token.start, 1, HL!(b"DoubleQuote"));
        p = s.offset(1);
        while p < e {
            if *p as char == '\\' && p.offset(1) < e {
                p = p.offset(1);
                if p.offset(1) == e {
                    size -= 1;
                    break;
                } else {
                    match *p as char {
                        // A "\<x>" form occupies at least 4 characters, and produces up to
                        // 6 characters: reserve space for 2 extra, but do not compute actual
                        // length just now, it would be costy.
                        '<' => {
                            size += 2;
                        }
                        // Hexadecimal, always single byte, but at least three bytes each.
                        'x' | 'X' => {
                            size -= 1;
                            if ascii_isxdigit(*p.offset(1) as char) {
                                size -= 1;
                                if p.offset(2) < e && ascii_isxdigit(*p.offset(2) as char) {
                                    size -= 1;
                                }
                            }
                        }
                        // Unicode
                        //
                        // \uF takes 1 byte which is 2 bytes less then escape sequence.
                        // \uFF: 2 bytes, 2 bytes less.
                        // \uFFF: 3 bytes, 2 bytes less.
                        // \uFFFF: 3 bytes, 3 bytes less.
                        // \UFFFFF: 4 bytes, 3 bytes less.
                        // \UFFFFFF: 5 bytes, 3 bytes less.
                        // \UFFFFFFF: 6 bytes, 3 bytes less.
                        // \U7FFFFFFF: 6 bytes, 4 bytes less.
                        'u' | 'U' => {
                            let esc_start = p;
                            let mut n = if *p as char == 'u' { 4 } else { 8 } as libc::size_t;
                            let mut nr = 0;
                            p = p.offset(1);
                            while p.offset(1) < e
                                && {
                                    let fresh = n != 0;
                                    if fresh {
                                        n -= 1;
                                    }
                                    fresh
                                }
                                && ascii_isxdigit(*p.offset(1) as char)
                            {
                                p = p.offset(1);
                                nr = (nr << 4) + hex2nr(*p as libc::c_int)
                            }
                            // Escape length: (esc_start - 1) points to "\\", esc_start to "u"
                            // or "U", p to the byte after last byte. So escape sequence
                            // occupies p - (esc_start - 1), but it stands for a utf_char2len
                            // bytes.
                            size -= (p.offset_from(esc_start.offset(-(1)))
                                - utf_char2len(nr) as isize)
                                as usize;
                            p = p.offset(-1)
                        }
                        // Octal, always single byte, but at least two bytes each.
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => {
                            size -= 1;
                            p = p.offset(1);
                            if *p as char >= '0' && *p as char <= '7' {
                                size -= 1;
                                p = p.offset(1);
                                if p < e && *p as char >= '0' && *p as char <= '7' {
                                    size -= 1;
                                    p = p.offset(1)
                                }
                            }
                        }
                        _ => size -= 1,
                    }
                }
            }
            p = p.offset(1)
        }
        if size == 0 {
            node.data.str_0.value = ptr::null_mut();
            node.data.str_0.size = 0
        } else {
            node.data.str_0.value = xmalloc(size) as *mut u8;
            let mut v_p_0 = node.data.str_0.value;
            p = s.offset(1);
            while p < e {
                let chunk_e_1 = memchr(p, '\\', e.offset_from(p) as libc::size_t) as *const u8;
                if chunk_e_1.is_null() {
                    memcpy(v_p_0, p, e.offset_from(p) as libc::size_t);
                    v_p_0 = v_p_0.offset(e.offset_from(p) as isize);
                    break;
                }
                memcpy(
                    v_p_0 as *mut libc::c_void,
                    p as *const libc::c_void,
                    chunk_e_1.offset_from(p) as libc::size_t,
                );
                v_p_0 = v_p_0.offset(chunk_e_1.offset_from(p) as isize);
                p = chunk_e_1.offset(1);
                if p == e {
                    *v_p_0 = '\\' as u8;
                    v_p_0 = v_p_0.offset(1);
                    break;
                }
                let mut is_unknown = false;
                let v_p_start: *const u8 = v_p_0;
                macro_rules! SINGLE_CHAR_ESC {
                    ($real_ch: expr) => {{
                        *v_p_0 = $real_ch as u8;
                        v_p_0 = v_p_0.offset(1);
                        p = p.offset(1)
                    }};
                }
                match *p as char {
                    'b' => SINGLE_CHAR_ESC!(BS),
                    'e' => SINGLE_CHAR_ESC!(ESC),
                    'f' => SINGLE_CHAR_ESC!(FF),
                    'n' => SINGLE_CHAR_ESC!(NL),
                    'r' => SINGLE_CHAR_ESC!(CAR),
                    't' => SINGLE_CHAR_ESC!(TAB),
                    '"' => SINGLE_CHAR_ESC!('"'),
                    '\\' => SINGLE_CHAR_ESC!('\\'),

                    // Hexadecimal or unicode.
                    'x' | 'X' | 'u' | 'U' => {
                        if p.offset(1) < e && ascii_isxdigit(*p.offset(1) as char) {
                            let mut n: libc::size_t;
                            let mut nr_0: libc::c_int = 0;
                            let is_hex = *p as char == 'x' || *p as char == 'X';

                            if is_hex {
                                n = 2
                            } else if *p as char == 'u' {
                                n = 4
                            } else {
                                n = 8
                            }
                            while p.offset(1) < e
                                && {
                                    let fresh = n != 0;
                                    if fresh {
                                        n -= 1;
                                    }
                                    fresh
                                }
                                && ascii_isxdigit(*p.offset(1) as char)
                            {
                                p = p.offset(1);
                                nr_0 = (nr_0 << 4) + hex2nr(*p as libc::c_int)
                            }
                            p = p.offset(1);
                            if is_hex {
                                *v_p_0 = nr_0 as u8;
                                v_p_0 = v_p_0.offset(1);
                            } else {
                                v_p_0 = v_p_0.offset(utf_char2bytes(nr_0, v_p_0) as isize)
                            }
                        } else {
                            is_unknown = true;
                            *v_p_0 = *p;
                            v_p_0 = v_p_0.offset(1);
                            p = p.offset(1)
                        }
                    }
                    // Octal: "\1", "\12", "\123".
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => {
                        let mut ch = *p - '0' as u8;
                        p = p.offset(1);
                        if p < e && *p as char >= '0' && *p as char <= '7' {
                            ch = (ch << 3) + (*p - '0' as u8);
                            p = p.offset(1);
                            if p < e && *p as char >= '0' && *p as char <= '7' {
                                ch = (ch << 3) + (*p - '0' as u8);
                                p = p.offset(1);
                            }
                        }
                        *v_p_0 = ch;
                        v_p_0 = v_p_0.offset(1);
                    }
                    // Special key, e.g.: "\<C-W>"
                    '<' => {
                        let special_len = trans_special(
                            &mut p,
                            e.offset_from(p) as libc::size_t,
                            v_p_0,
                            true,
                            true,
                        ) as libc::size_t;
                        if special_len != 0 {
                            v_p_0 = v_p_0.offset(special_len as isize)
                        } else {
                            is_unknown = true;
                            mb_copy_char(&mut p, &mut v_p_0);
                        }
                    }
                    _ => {
                        is_unknown = true;
                        mb_copy_char(&mut p, &mut v_p_0);
                    }
                }
                if !pstate.colors.is_null() {
                    shifts.push(StringShift {
                        start: token.start.col + chunk_e_1.offset_from(s) as libc::size_t,
                        orig_len: p.offset_from(chunk_e_1) as libc::size_t,
                        act_len: v_p_0.offset_from(v_p_start) as libc::size_t,
                        escape_not_known: is_unknown,
                    });
                }
            }
            node.data.str_0.size = v_p_0.offset_from(node.data.str_0.value) as libc::size_t;
        }
    }
    if !pstate.colors.is_null() {
        // TODO(ZyX-I): use ast_stack to determine and highlight regular expressions
        // TODO(ZyX-I): use ast_stack to determine and highlight printf format str
        // TODO(ZyX-I): use ast_stack to determine and highlight expression strings
        let mut next_col = token.start.col + 1;
        let body_str = if is_double {
            HL!(b"DoubleQuotedBody")
        } else {
            HL!(b"SingleQuotedBody")
        };
        let esc_str = if is_double {
            HL!(b"DoubleQuotedEscape")
        } else {
            HL!(b"SingleQuotedQuote")
        };
        let ukn_esc_str = if is_double {
            HL!(b"DoubleQuotedUnknownEscape")
        } else {
            HL!(b"SingleQuotedUnknownEscape")
        };
        for i in 0..shifts.size() {
            let cur_shift = *shifts.A(i);
            if cur_shift.start > next_col {
                viml_parser_highlight(
                    pstate,
                    recol_pos(token.start, next_col),
                    cur_shift.start - next_col,
                    body_str,
                );
            }
            viml_parser_highlight(
                pstate,
                recol_pos(token.start, cur_shift.start),
                cur_shift.orig_len,
                if cur_shift.escape_not_known {
                    ukn_esc_str
                } else {
                    esc_str
                },
            );
            next_col = cur_shift.start + cur_shift.orig_len;
        }
        if next_col - token.start.col < token.len - token.data.str_0.closed as usize {
            viml_parser_highlight(
                pstate,
                recol_pos(token.start, next_col),
                token.start.col + token.len - token.data.str_0.closed as usize - next_col,
                body_str,
            );
        }
    }
    if token.data.str_0.closed {
        if is_double {
            viml_parser_highlight(
                pstate,
                shifted_pos(token.start, token.len - 1),
                1,
                HL!(b"DoubleQuote"),
            );
        } else {
            viml_parser_highlight(
                pstate,
                shifted_pos(token.start, token.len - 1),
                1,
                HL!(b"SingleQuote"),
            );
        }
    }
    shifts.destroy();
}

/// Additional flags to pass to lexer depending on want_node
fn want_node_to_lexer_flags(want_node: ExprASTWantedNode) -> LexExprFlags {
    match want_node {
        kENodeValue => LexExprFlags::IsNotCmp,
        kENodeOperator => LexExprFlags::ForbidScope,
    }
}

/// Number of characters to highlight as NumberPrefix depending on the base
fn base_to_prefix_length(base: u8) -> u8 {
    match base {
        2 => 2,
        8 => 1,
        10 => 0,
        16 => 2,
        _ => panic!("unsupported base"),
    }
}

/// Parse one VimL expression
///
/// @param  pstate  Parser state.
/// @param[in]  flags  Additional flags, see ExprParserFlags
///
/// @return Parsed AST.
#[no_mangle]
pub unsafe extern "C" fn viml_pexpr_parse(
    pstate: &mut ParserState,
    flags: ExprParserFlags,
) -> ExprAST {
    let mut pline: ParserLine;
    let mut top_node_p: *mut *mut ExprASTNode;
    let mut cur_node: *mut ExprASTNode = ptr::null_mut();
    let mut want_value: bool;
    let mut node_is_key: bool;
    let mut is_single_assignment: bool;
    let mut eastnode_p: *const *mut ExprASTNode;
    let mut eastnode_type: ExprASTNodeType;
    let mut eastnode_lvl: ExprOpLvl;

    let mut ast = ExprAST {
        err: {
            let init = ExprASTError {
                msg: ptr::null(),
                arg_len: 0,
                arg: ptr::null(),
            };
            init
        },
        root: ptr::null_mut(),
    };
    // Expression stack contains current branch in AST tree: that is
    // - Stack item 0 contains root of the tree, i.e. &ast->root.
    // - Stack item i points to the previous stack items’ last child.
    //
    // When parser expects “value” node that is something like identifier or "["
    // (list start) last stack item contains NULL. Otherwise last stack item is
    // supposed to contain last “finished” value: e.g. "1" or "+(1, 1)" (node
    // representing "1+1").
    let mut ast_stack = ExprASTStack::init();
    ast_stack.push(&mut ast.root);
    let mut want_node = kENodeValue;
    let mut pt_stack = ExprASTParseTypeStack::init();
    pt_stack.push(kEPTExpr);
    if flags.contains(ExprParserFlags::ParseLet) {
        pt_stack.push(kEPTAssignment);
    }
    let mut prev_token = LexExprToken {
        type_0: kExprLexMissing,
        ..LexExprToken::default()
    };
    let mut highlighted_prev_spacing = false;
    // Lambda node, valid when parsing lambda arguments only.
    let mut lambda_node: *mut ExprASTNode = ptr::null_mut();
    let mut asgn_level = 0;

    's_59: loop {
        let is_concat_or_subscript = want_node == kENodeValue
            && ast_stack.size() > 1
            && (***ast_stack.Z(1)).type_0 == kExprNodeConcatOrSubscript;
        let lexer_additional_flags = LexExprFlags::Peek
            | if flags.contains(ExprParserFlags::DisallowEOC) {
                LexExprFlags::ForbidEOC
            } else {
                LexExprFlags::empty()
            }
            | if want_node == kENodeValue
                && (ast_stack.size() == 1
                    || (***ast_stack.Z(1)).type_0 != kExprNodeConcat
                        && (***ast_stack.Z(1)).type_0 != kExprNodeConcatOrSubscript)
            {
                LexExprFlags::AllowFloat
            } else {
                LexExprFlags::empty()
            };
        let mut cur_token = viml_pexpr_next_token(
            pstate,
            want_node_to_lexer_flags(want_node) | lexer_additional_flags,
        );
        if cur_token.type_0 == kExprLexEOC {
            break;
        }
        let mut tok_type = cur_token.type_0;
        let token_invalid = tok_type == kExprLexInvalid;
        let mut is_invalid = token_invalid;

        GEN_HL!(is_invalid);

        /// Highlight current token with the given group
        macro_rules! HL_CUR_TOKEN {
            ($g: literal) => {
                viml_parser_highlight(pstate, cur_token.start, cur_token.len, HL!($g));
            };
            ($g1: literal, $g2: literal) => {
                viml_parser_highlight(pstate, cur_token.start, cur_token.len, HL!($g1, $g2));
            };
        }

        /// Allocate new node, saving some values
        macro_rules! NEW_NODE {
            ($type: expr) => {
                viml_pexpr_new_node($type)
            };
        }

        /// Set position of the given node to position from the given token
        ///
        /// @param  cur_node  Node to modify.
        /// @param  cur_token  Token to set position from.
        macro_rules! POS_FROM_TOKEN {
            ($cur_node: expr, $cur_token: ident) => {{
                (*$cur_node).start = $cur_token.start;
                (*$cur_node).len = $cur_token.len;
            }};
        }

        /// Allocate new node and set its position from the current token
        ///
        /// If previous token happened to contain spacing then it will be included.
        ///
        /// @param  cur_node  Variable to save allocated node to.
        /// @param  typ  Node type.
        macro_rules! NEW_NODE_WITH_CUR_POS {
            ($cur_node: expr, $typ: expr) => {{
                $cur_node = NEW_NODE!($typ);
                POS_FROM_TOKEN!($cur_node, cur_token);
                if prev_token.type_0 == kExprLexSpacing {
                    (*$cur_node).start = prev_token.start;
                    (*$cur_node).len += prev_token.len;
                }
            }};
        }

        /// Check whether it is possible to have next expression after current
        ///
        /// For :echo: `:echo @a @a` is a valid expression. `:echo (@a @a)` is not.
        macro_rules! MAY_HAVE_NEXT_EXPR {
            () => {{
                ast_stack.size() == 1
            }};
        }

        /// Add operator node
        ///
        /// @param[in]  cur_node  Node to add.
        macro_rules! ADD_OP_NODE {
            ($cur_node: ident) => {
                is_invalid |= !viml_pexpr_handle_bop(
                    pstate,
                    &mut ast_stack,
                    &mut *$cur_node,
                    &mut want_node,
                    &mut ast.err,
                );
                let _ = is_invalid;
            };
        }

        /// Record missing operator: for things like
        ///
        ///     :echo @a @a
        ///
        /// (allowed) or
        ///
        ///     :echo (@a @a)
        ///
        /// (parsed as OpMissing(@a, @a)).
        macro_rules! OP_MISSING {
            () => {{
                if flags.contains(ExprParserFlags::Multi) && MAY_HAVE_NEXT_EXPR!() {
                    // Multiple expressions allowed, return without calling
                    // viml_parser_advance().
                    break 's_59; // goto viml_pexpr_parse_end
                }
                assert!(!(*top_node_p).is_null());
                ERROR_FROM_TOKEN_AND_MSG!(cur_token, gettext("E15: Missing operator: %.*s"));
                NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeOpMissing);
                (*cur_node).len = 0;
                ADD_OP_NODE!(cur_node);
                continue; // goto viml_pexpr_parse_process_token
            }};
        }

        /// Record missing value: for things like "* 5"
        ///
        /// @param[in]  msg  Error message.
        macro_rules! ADD_VALUE_IF_MISSING {
            ($msg: expr) => {{
                if want_node == kENodeValue {
                    ERROR_FROM_TOKEN_AND_MSG!(cur_token, $msg);
                    NEW_NODE_WITH_CUR_POS!(*top_node_p, kExprNodeMissing);
                    (**top_node_p).len = 0;
                    want_node = kENodeOperator;
                }
            }};
        }

        /// Set error from the given token and given message
        macro_rules! ERROR_FROM_TOKEN_AND_MSG {
            ($cur_token: ident, $msg: expr $(,)?) => {{
                is_invalid = true;
                let _ = is_invalid;
                east_set_error(pstate, &mut ast.err, $msg, $cur_token.start);
            }};
        }

        /// Like #ERROR_FROM_TOKEN_AND_MSG, but gets position from a node
        macro_rules! ERROR_FROM_NODE_AND_MSG {
            ($node: expr, $msg: expr) => {{
                is_invalid = true;
                east_set_error(pstate, &mut ast.err, $msg, (*$node).start);
            }};
        }

        /// Set error from the given kExprLexInvalid token
        macro_rules! ERROR_FROM_TOKEN {
            ($cur_token: ident) => {{
                ERROR_FROM_TOKEN_AND_MSG!($cur_token, $cur_token.data.err.msg)
            }};
        }

        /// Select figure brace type, altering highlighting as well if needed
        ///
        /// @param[out]  node  Node to modify type.
        /// @param[in]  new_type  New type, one of ExprASTNodeType values without
        ///                       kExprNode prefix.
        /// @param[in]  hl  Corresponding highlighting, passed as an argument to #HL.
        macro_rules! SELECT_FIGURE_BRACE_TYPE {
            ($node: expr, $new_type: ident, $hl: literal) => {{
                assert!(
                    (*$node).type_0 == kExprNodeUnknownFigure
                        || (*$node).type_0 == concat_idents!(kExprNode, $new_type)
                );
                (*$node).type_0 = concat_idents!(kExprNode, $new_type);
                if !pstate.colors.is_null() {
                    (*(*pstate).colors)
                        .A((*$node).data.fig.opening_hl_idx)
                        .group = HL!($hl);
                }
            }};
        }

        /// Add identifier which should constitute complex identifier node
        ///
        /// This one is to be called only in case want_node is kENodeOperator.
        ///
        /// @param  new_ident_node_code  Code used to create a new identifier node and
        ///                              update want_node and ast_stack, without
        ///                              a trailing semicolon.
        /// @param  hl  Highlighting name to use, passed as an argument to #HL.
        macro_rules! ADD_IDENT {
            ($new_ident_node_code: block, $hl: literal) => {{
                assert!(want_node == kENodeOperator);
                // Operator: may only be curly braces name, but only under certain
                // conditions.

                // First condition is that there is no space before a part of complex
                // identifier.
                if prev_token.type_0 == kExprLexSpacing {
                    OP_MISSING!();
                }
                match (**top_node_p).type_0 {
                    kExprNodeComplexIdentifier
                    | kExprNodePlainIdentifier
                    | kExprNodeCurlyBracesIdentifier => {
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeComplexIdentifier);
                        (*cur_node).len = 0;
                        (*cur_node).children = *top_node_p;
                        *top_node_p = cur_node;
                        ast_stack.push(&mut (*(*cur_node).children).next);
                        let new_top_node_p = *ast_stack.last();
                        assert!((*new_top_node_p).is_null());
                        $new_ident_node_code;
                        *new_top_node_p = cur_node;
                        HL_CUR_TOKEN!($hl);
                    }
                    _ => {
                        OP_MISSING!();
                    }
                }
            }};
        }
        macro_rules! SIMPLE_UB_OP {
            ($op: ident, $bop: literal) => {{
                if want_node == kENodeValue {
                    /* Value level: assume unary operator. */
                    NEW_NODE_WITH_CUR_POS!(cur_node, concat_idents!(kExprNodeUnary, $op));
                    *top_node_p = cur_node;
                    ast_stack.push(&mut (*cur_node).children);
                    HL_CUR_TOKEN!(b"Unary", $bop);
                } else {
                    NEW_NODE_WITH_CUR_POS!(cur_node, concat_idents!(kExprNodeBinary, $op));
                    ADD_OP_NODE!(cur_node);
                    HL_CUR_TOKEN!(b"Binary", $bop);
                }
                want_node = kENodeValue;
            }};
        }
        macro_rules! SIMPLE_B_OP {
            ($op: ident, $bop: literal, $msg: literal) => {{
                ADD_VALUE_IF_MISSING!(gettext(concat!("E15: Unexpected ", $msg, ": %.*s")));
                NEW_NODE_WITH_CUR_POS!(cur_node, concat_idents!(kExprNode, $op));
                HL_CUR_TOKEN!($bop);
                ADD_OP_NODE!(cur_node);
            }};
        }

        // viml_pexpr_parse_process_token
        loop {
            // May use different flags this time.
            cur_token = viml_pexpr_next_token(
                pstate,
                want_node_to_lexer_flags(want_node) | lexer_additional_flags,
            );
            if tok_type == kExprLexSpacing {
                if is_invalid {
                    HL_CUR_TOKEN!(b"Spacing");
                } else {
                    // Do not do anything: let regular spacing be highlighted as normal.
                    // This also allows later to highlight spacing as invalid.
                }
                break; // goto viml_pexpr_parse_cycle_end
            }
            if is_invalid && prev_token.type_0 == kExprLexSpacing && !highlighted_prev_spacing {
                viml_parser_highlight(pstate, prev_token.start, prev_token.len, HL!(b"Spacing"));
                is_invalid = false;
                highlighted_prev_spacing = true
            }
            pline = *pstate
                .reader
                .lines
                .items
                .offset(cur_token.start.line as isize);
            top_node_p = *ast_stack.last();
            assert!(!ast_stack.is_empty());
            if cfg!(debug_assertions) {
                want_value = want_node == kENodeValue;
                assert!(want_value == (*top_node_p).is_null());
                assert!(*ast_stack.A(0) == &mut ast.root);
                // Check that stack item i + 1 points to stack items’ i *last* child.
                for i in 0..(ast_stack.size() - 1) {
                    let item_null = want_value && i + 2 == ast_stack.size();
                    assert!(
                        &mut (***ast_stack.A(i)).children as *mut *mut ExprASTNode
                            == *ast_stack.A(i + 1)
                            && if item_null {
                                (***ast_stack.A(i)).children.is_null()
                            } else {
                                (*(***ast_stack.A(i)).children).next.is_null()
                            }
                            || &mut (*(***ast_stack.A(i)).children).next as *mut *mut ExprASTNode
                                == *ast_stack.A(i + 1)
                                && if item_null {
                                    (*(***ast_stack.A(i)).children).next.is_null()
                                } else {
                                    (*(*(***ast_stack.A(i)).children).next).next.is_null()
                                }
                    );
                }
            }
            // Note: in Vim whether expression "cond?d.a:2" is valid depends both on
            // "cond" and whether "d" is a dictionary: expression is valid if condition
            // is true and "d" is a dictionary (with "a" key or it will complain about
            // missing one, but this is not relevant); if any of the requirements is
            // broken then this thing is parsed as "d . a:2" yielding missing colon
            // error. This parser does not allow such ambiguity, especially because it
            // simply can’t: whether "d" is a dictionary is not known at the parsing
            // time.
            //
            // Here example will always contain a concat with "a:2" sucking colon,
            // making expression invalid both because there is no longer a spare colon
            // for ternary and because concatenating dictionary with anything is not
            // valid. There are more cases when this will make a difference though.
            node_is_key = is_concat_or_subscript
                && if cur_token.type_0 == kExprLexPlainIdentifier {
                    !cur_token.data.var.autoload && cur_token.data.var.scope == kExprVarScopeMissing
                } else {
                    cur_token.type_0 == kExprLexNumber
                }
                && prev_token.type_0 != kExprLexSpacing;
            if is_concat_or_subscript && !node_is_key {
                // Note: in Vim "d. a" (this is the reason behind `prev_token.type !=
                // kExprLexSpacing` part of the condition) as well as any other "d.{expr}"
                // where "{expr}" does not look like a key is invalid whenever "d" happens
                // to be a dictionary. Since parser has no idea whether preceding
                // expression is actually a dictionary it can’t outright reject anything,
                // so it turns kExprNodeConcatOrSubscript into kExprNodeConcat instead,
                // which will yield different errors then Vim does in a number of
                // circumstances, and in any case runtime and not parse time errors.
                (***ast_stack.Z(1)).type_0 = kExprNodeConcat
            }
            // Pop some stack pt_stack items in case of misplaced nodes.
            is_single_assignment = *pt_stack.last() == kEPTSingleAssignment;
            match *pt_stack.last() {
                kEPTExpr => {}
                kEPTLambdaArguments => {
                    if want_node == kENodeOperator
                        && tok_type != kExprLexComma
                        && tok_type != kExprLexArrow
                        || want_node == kENodeValue
                            && !(cur_token.type_0 == kExprLexPlainIdentifier
                                && cur_token.data.var.scope == kExprVarScopeMissing
                                && !cur_token.data.var.autoload)
                            && tok_type != kExprLexArrow
                    {
                        (*lambda_node).data.fig.type_guesses.allow_lambda = false;
                        if !(*lambda_node).children.is_null()
                            && (*(*lambda_node).children).type_0 == kExprNodeComma
                        {
                            // If lambda has comma child this means that parser has already seen
                            // at least "{arg1,", so node cannot possibly be anything, but
                            // lambda.
                            // Vim may give E121 or E720 in this case, but it does not look
                            // right to have either because both are results of reevaluation
                            // possibly-lambda node as a dictionary and here this is not going
                            // to happen.
                            ERROR_FROM_TOKEN_AND_MSG!(
                                cur_token,
                                gettext("E15: Expected lambda arguments list or arrow: %.*s")
                            );
                        } else {
                            // Else it may appear that possibly-lambda node is actually
                            // a dictionary or curly-braces-name identifier.
                            lambda_node = ptr::null_mut();
                            pt_stack.lop(1);
                        }
                    }
                }
                kEPTSingleAssignment | kEPTAssignment => {
                    if want_node == kENodeValue
                        && tok_type != kExprLexBracket
                        && tok_type != kExprLexPlainIdentifier
                        && (tok_type != kExprLexFigureBrace || cur_token.data.brc.closing)
                        && !(node_is_key && tok_type == kExprLexNumber)
                        && tok_type != kExprLexEnv
                        && tok_type != kExprLexOption
                        && tok_type != kExprLexRegister
                    {
                        ERROR_FROM_TOKEN_AND_MSG!(
                            cur_token,
                            gettext("E15: Expected value part of assignment lvalue: %.*s")
                        );
                        pt_stack.lop(1);
                    } else if want_node == kENodeOperator
                        && tok_type != kExprLexBracket
                        && (tok_type != kExprLexFigureBrace || cur_token.data.brc.closing)
                        && tok_type != kExprLexDot
                        && (tok_type != kExprLexComma || !is_single_assignment)
                        && tok_type != kExprLexAssignment
                        // Curly brace identifiers: will contain plain identifier or
                        // another curly brace in position where operator is wanted.
                        && !((tok_type == kExprLexPlainIdentifier
                            || tok_type == kExprLexFigureBrace && !cur_token.data.brc.closing)
                            && prev_token.type_0 != kExprLexSpacing)
                    {
                        if flags.contains(ExprParserFlags::Multi) && MAY_HAVE_NEXT_EXPR!() {
                            break 's_59; // goto viml_pexpr_parse_end
                        }
                        ERROR_FROM_TOKEN_AND_MSG!(
                            cur_token,
                            gettext("E15: Expected assignment operator or subscript: %.*s")
                        );
                        pt_stack.lop(1);
                    }
                    assert!(!pt_stack.is_empty());
                }
            }
            assert!(!pt_stack.is_empty());
            let cur_pt = *pt_stack.last();
            assert!(lambda_node.is_null() || cur_pt == kEPTLambdaArguments);
            match tok_type {
                kExprLexMissing | kExprLexSpacing | kExprLexEOC => {
                    unreachable!();
                }
                kExprLexInvalid => {
                    ERROR_FROM_TOKEN!(cur_token);
                    tok_type = cur_token.data.err.type_0;
                    continue; // goto viml_pexpr_parse_process_token
                }
                kExprLexRegister => {
                    if want_node == kENodeOperator {
                        // Register in operator position: e.g. @a @a
                        OP_MISSING!();
                    }
                    NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeRegister);
                    (*cur_node).data.reg.name = cur_token.data.reg.name;
                    *top_node_p = cur_node;
                    want_node = kENodeOperator;
                    HL_CUR_TOKEN!(b"Register");
                }
                kExprLexPlus => SIMPLE_UB_OP!(Plus, b"Plus"),
                kExprLexMinus => SIMPLE_UB_OP!(Minus, b"Minus"),
                kExprLexOr => SIMPLE_B_OP!(Or, b"Or", "or operator"),
                kExprLexAnd => SIMPLE_B_OP!(And, b"And", "and operator"),
                kExprLexMultiplication => {
                    ADD_VALUE_IF_MISSING!(gettext(
                        "E15: Unexpected multiplication-like operator: %.*s"
                    ));
                    macro_rules! MUL_OP {
                        ($node_op_tail: ident, $bnode_op_tail: literal) => {{
                            NEW_NODE_WITH_CUR_POS!(
                                cur_node,
                                concat_idents!(kExprNode, $node_op_tail)
                            );
                            HL_CUR_TOKEN!($bnode_op_tail);
                        }};
                    }
                    match cur_token.data.mul.type_0 {
                        kExprLexMulMul => MUL_OP!(Multiplication, b"Multiplication"),
                        kExprLexMulDiv => MUL_OP!(Division, b"Division"),
                        kExprLexMulMod => MUL_OP!(Mod, b"Mod"),
                    }
                    ADD_OP_NODE!(cur_node);
                }
                kExprLexOption => {
                    if want_node == kENodeOperator {
                        OP_MISSING!();
                    }
                    NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeOption);
                    if cur_token.type_0 == kExprLexInvalid {
                        assert!(
                            cur_token.len == 1
                                || cur_token.len == 3
                                    && *pline.data.offset((cur_token.start.col + 2) as isize)
                                        as char
                                        == ':'
                        );
                        (*cur_node).data.opt.ident = pline
                            .data
                            .offset((cur_token.start.col + cur_token.len) as isize);
                        (*cur_node).data.opt.ident_len = 0;
                        (*cur_node).data.opt.scope = if cur_token.len == 3 {
                            (*pline.data.offset((cur_token.start.col + 1) as isize)).into()
                        } else {
                            kExprOptScopeUnspecified
                        }
                    } else {
                        (*cur_node).data.opt.ident = cur_token.data.opt.name;
                        (*cur_node).data.opt.ident_len = cur_token.data.opt.len;
                        (*cur_node).data.opt.scope = cur_token.data.opt.scope
                    }
                    *top_node_p = cur_node;
                    want_node = kENodeOperator;
                    viml_parser_highlight(pstate, cur_token.start, 1, HL!(b"OptionSigil"));
                    let scope_shift = if cur_token.data.opt.scope == kExprOptScopeUnspecified {
                        0
                    } else {
                        2
                    };
                    if scope_shift != 0 {
                        viml_parser_highlight(
                            pstate,
                            shifted_pos(cur_token.start, 1),
                            1,
                            HL!(b"OptionScope"),
                        );
                        viml_parser_highlight(
                            pstate,
                            shifted_pos(cur_token.start, 2),
                            1,
                            HL!(b"OptionScopeDelimiter"),
                        );
                    }
                    viml_parser_highlight(
                        pstate,
                        shifted_pos(cur_token.start, scope_shift + 1),
                        cur_token.len - (scope_shift + 1),
                        HL!(b"OptionName"),
                    );
                }
                kExprLexEnv => {
                    if want_node == kENodeOperator {
                        OP_MISSING!();
                    }
                    NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeEnvironment);
                    (*cur_node).data.env.ident =
                        pline.data.offset((cur_token.start.col + 1) as isize);
                    (*cur_node).data.env.ident_len = cur_token.len - 1;
                    if (*cur_node).data.env.ident_len == 0 {
                        ERROR_FROM_TOKEN_AND_MSG!(
                            cur_token,
                            gettext("E15: Environment variable name missing")
                        );
                    }
                    *top_node_p = cur_node;
                    want_node = kENodeOperator;
                    viml_parser_highlight(pstate, cur_token.start, 1, HL!(b"EnvironmentSigil"));
                    viml_parser_highlight(
                        pstate,
                        shifted_pos(cur_token.start, 1),
                        cur_token.len - 1,
                        HL!(b"EnvironmentName"),
                    );
                }
                kExprLexNot => {
                    if want_node == kENodeOperator {
                        OP_MISSING!();
                    }
                    NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeNot);
                    *top_node_p = cur_node;
                    ast_stack.push(&mut (*cur_node).children);
                    HL_CUR_TOKEN!(b"Not");
                }
                kExprLexComparison => {
                    ADD_VALUE_IF_MISSING!(gettext(
                        "E15: Expected value, got comparison operator: %.*s"
                    ));
                    NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeComparison);
                    if cur_token.type_0 == kExprLexInvalid {
                        (*cur_node).data.cmp.ccs = kCCStrategyUseOption;
                        (*cur_node).data.cmp.type_0 = kExprCmpEqual;
                        (*cur_node).data.cmp.inv = false
                    } else {
                        (*cur_node).data.cmp.ccs = cur_token.data.cmp.ccs;
                        (*cur_node).data.cmp.type_0 = cur_token.data.cmp.type_0;
                        (*cur_node).data.cmp.inv = cur_token.data.cmp.inv
                    }
                    ADD_OP_NODE!(cur_node);
                    if cur_token.data.cmp.ccs != kCCStrategyUseOption {
                        viml_parser_highlight(
                            pstate,
                            cur_token.start,
                            cur_token.len - 1,
                            HL!(b"Comparison"),
                        );
                        viml_parser_highlight(
                            pstate,
                            shifted_pos(cur_token.start, cur_token.len - 1),
                            1,
                            HL!(b"ComparisonModifier"),
                        );
                    } else {
                        HL_CUR_TOKEN!(b"Comparison");
                    }
                    want_node = kENodeValue;
                }
                kExprLexComma => {
                    assert!(!(want_node == kENodeValue && cur_pt == kEPTLambdaArguments));
                    if want_node == kENodeValue {
                        // Value level: comma appearing here is not valid.
                        // Note: in Vim string(,x) will give E116, this is not the case here.
                        ERROR_FROM_TOKEN_AND_MSG!(
                            cur_token,
                            gettext("E15: Expected value, got comma: %.*s")
                        );
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeMissing);
                        (*cur_node).len = 0;
                        *top_node_p = cur_node;
                        want_node = kENodeOperator
                    }
                    if cur_pt == kEPTLambdaArguments {
                        assert!(!lambda_node.is_null());
                        assert!((*lambda_node).data.fig.type_guesses.allow_lambda);
                        SELECT_FIGURE_BRACE_TYPE!(lambda_node, Lambda, b"Lambda");
                    }
                    loop {
                        macro_rules! viml_pexpr_parse_invalid_comma {
                            () => {{
                                ERROR_FROM_TOKEN_AND_MSG!(
                                    cur_token,
                                    gettext("E15: Comma outside of call, lambda or literal: %.*s")
                                );
                                break;
                            }};
                        }
                        if ast_stack.size() < 2 {
                            viml_pexpr_parse_invalid_comma!();
                        }
                        for i in 1..ast_stack.size() {
                            eastnode_p = *ast_stack.Z(i);
                            eastnode_type = (**eastnode_p).type_0;
                            eastnode_lvl = node_lvl(**eastnode_p);
                            if eastnode_type == kExprNodeLambda {
                                assert!(
                                    cur_pt == kEPTLambdaArguments && want_node == kENodeOperator
                                );
                                break;
                            } else if eastnode_type == kExprNodeDictLiteral
                                || eastnode_type == kExprNodeListLiteral
                                || eastnode_type == kExprNodeCall
                            {
                                break;
                            } else if eastnode_type == kExprNodeComma
                                || eastnode_type == kExprNodeColon
                                || eastnode_lvl > kEOpLvlComma
                            {
                                // Do nothing
                            } else {
                                viml_pexpr_parse_invalid_comma!();
                            }
                            if i == ast_stack.size() - 1 {
                                viml_pexpr_parse_invalid_comma!();
                            }
                        }
                        break;
                    }
                    NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeComma);
                    ADD_OP_NODE!(cur_node);
                    HL_CUR_TOKEN!(b"Comma");
                }
                kExprLexColon => {
                    const EXP_VAL_COLON: &str = "E15: Expected value, got colon: %.*s";
                    (|| {
                        let mut is_ternary = false;
                        macro_rules! viml_pexpr_parse_invalid_colon {
                            () => {{
                                ERROR_FROM_TOKEN_AND_MSG!(
                                    cur_token,
                                    gettext("E15: Colon outside of dictionary or ternary operator: %.*s")
                                    );
                                viml_pexpr_parse_valid_colon!();
                            }};
                        }
                        macro_rules! viml_pexpr_parse_valid_colon {
                            () => {{
                                ADD_VALUE_IF_MISSING!(gettext(EXP_VAL_COLON));
                                if is_ternary {
                                    HL_CUR_TOKEN!(b"TernaryColon");
                                } else {
                                    NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeColon);
                                    ADD_OP_NODE!(cur_node);
                                    HL_CUR_TOKEN!(b"Colon");
                                }
                                return;
                            }};
                        }
                        if ast_stack.size() < 2 {
                            viml_pexpr_parse_invalid_colon!();
                        }
                        let mut can_be_ternary = true;
                        let mut is_subscript = false;
                        for i in 1..ast_stack.size() {
                            let eastnode_p_0 = *ast_stack.Z(i);
                            let eastnode_type_0 = (**eastnode_p_0).type_0;
                            let eastnode_lvl_0 = node_lvl(**eastnode_p_0);
                            assert!(kEOpLvlTernary > kEOpLvlComma); // operator priorities
                            if can_be_ternary
                                && eastnode_type_0 == kExprNodeTernaryValue
                                && !(**eastnode_p_0).data.ter.got_colon
                            {
                                ast_stack.lop(i);
                                (**eastnode_p_0).start = cur_token.start;
                                (**eastnode_p_0).len = cur_token.len;
                                if prev_token.type_0 == kExprLexSpacing {
                                    (**eastnode_p_0).start = prev_token.start;
                                    (**eastnode_p_0).len += prev_token.len;
                                }
                                is_ternary = true;
                                (**eastnode_p_0).data.ter.got_colon = true;
                                ADD_VALUE_IF_MISSING!(gettext(EXP_VAL_COLON));
                                assert!(!(**eastnode_p_0).children.is_null());
                                assert!((*(**eastnode_p_0).children).next.is_null());
                                ast_stack.push(&mut (*(**eastnode_p_0).children).next);
                                break;
                            } else if eastnode_type_0 == kExprNodeUnknownFigure {
                                SELECT_FIGURE_BRACE_TYPE!(*eastnode_p_0, DictLiteral, b"Dict");
                                break;
                            } else if eastnode_type_0 == kExprNodeDictLiteral {
                                break;
                            } else if eastnode_type_0 == kExprNodeSubscript {
                                is_subscript = true;
                                // can_be_ternary = false;
                                assert!(!is_ternary);
                                break;
                            } else if eastnode_type_0 == kExprNodeColon {
                                viml_pexpr_parse_invalid_colon!();
                            } else if eastnode_lvl_0 >= kEOpLvlTernaryValue {
                                // Do nothing
                            } else if eastnode_lvl_0 >= kEOpLvlComma {
                                can_be_ternary = false
                            } else {
                                viml_pexpr_parse_invalid_colon!();
                            }
                            if i == ast_stack.size() - 1 {
                                viml_pexpr_parse_invalid_colon!();
                            }
                        }
                        if is_subscript {
                            assert!(ast_stack.size() > 1);
                            // Colon immediately following subscript start: it is empty subscript
                            // part like a[:2].
                            if want_node == kENodeValue
                                && (***ast_stack.Z(1)).type_0 == kExprNodeSubscript
                            {
                                NEW_NODE_WITH_CUR_POS!(*top_node_p, kExprNodeMissing);
                                (**top_node_p).len = 0;
                                want_node = kENodeOperator
                            } else {
                                ADD_VALUE_IF_MISSING!(gettext(EXP_VAL_COLON));
                            }
                            NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeColon);
                            ADD_OP_NODE!(cur_node);
                            HL_CUR_TOKEN!(b"SubscriptColon");
                        } else {
                            viml_pexpr_parse_valid_colon!();
                        }
                    })();
                    want_node = kENodeValue;
                }
                kExprLexBracket => {
                    if cur_token.data.brc.closing {
                        let mut new_top_node_p: *mut *mut ExprASTNode = ptr::null_mut();
                        // Always drop the topmost value:
                        //
                        // 1. When want_node != kENodeValue topmost item on stack is
                        //    a *finished* left operand, which may as well be "{@a}" which
                        //    needs not be finished again.
                        // 2. Otherwise it is pointing to NULL what nobody wants.
                        ast_stack.lop(1);
                        macro_rules! viml_pexpr_parse_bracket_closing_error {
                            () => {{
                                assert!(ast_stack.is_empty());
                                ERROR_FROM_TOKEN_AND_MSG!(
                                    cur_token,
                                    gettext("E15: Unexpected closing figure brace: %.*s")
                                );
                                HL_CUR_TOKEN!(b"List");
                            }};
                        }
                        if ast_stack.is_empty() {
                            NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeListLiteral);
                            (*cur_node).len = 0;
                            if want_node != kENodeValue {
                                (*cur_node).children = *top_node_p
                            }
                            *top_node_p = cur_node;
                            viml_pexpr_parse_bracket_closing_error!();
                        } else {
                            if want_node == kENodeValue {
                                // It is OK to want value if
                                //
                                // 1. It is empty list literal, in which case top node will be
                                //    ListLiteral.
                                // 2. It is list literal with trailing comma, in which case top node
                                //    will be that comma.
                                // 3. It is subscript with colon, but without one of the values:
                                //    e.g. "a[:]", "a[1:]", top node will be colon in this case.
                                if (***ast_stack.last()).type_0 != kExprNodeListLiteral
                                    && (***ast_stack.last()).type_0 != kExprNodeComma
                                    && (***ast_stack.last()).type_0 != kExprNodeColon
                                {
                                    ERROR_FROM_TOKEN_AND_MSG!(
                                        cur_token,
                                        gettext("E15: Expected value, got closing bracket: %.*s")
                                    );
                                }
                            }
                            loop {
                                new_top_node_p = ast_stack.pop();
                                if !(!ast_stack.is_empty()
                                    && (new_top_node_p.is_null()
                                        || (**new_top_node_p).type_0 != kExprNodeListLiteral
                                            && (**new_top_node_p).type_0 != kExprNodeSubscript))
                                {
                                    break;
                                }
                            }
                            let new_top_node = *new_top_node_p;
                            match (*new_top_node).type_0 {
                                kExprNodeListLiteral => {
                                    if pt_is_assignment(cur_pt)
                                        && (*new_top_node).children.is_null()
                                    {
                                        ERROR_FROM_TOKEN_AND_MSG!(
                                            cur_token,
                                            gettext("E475: Unable to assign to empty list: %.*s")
                                        );
                                    }
                                    HL_CUR_TOKEN!(b"List");
                                }
                                kExprNodeSubscript => {
                                    HL_CUR_TOKEN!(b"SubscriptBracket");
                                }
                                _ => {
                                    viml_pexpr_parse_bracket_closing_error!();
                                }
                            }
                        }
                        ast_stack.push(new_top_node_p);
                        want_node = kENodeOperator;
                        if ast_stack.size() <= asgn_level {
                            assert!(ast_stack.size() == asgn_level);
                            asgn_level = 0;
                            if cur_pt == kEPTAssignment {
                                assert!(!ast.err.msg.is_null());
                            } else if cur_pt == kEPTExpr
                                && pt_stack.size() > 1
                                && pt_is_assignment(*pt_stack.Z(1))
                            {
                                pt_stack.lop(1);
                            }
                        }
                        if cur_pt == kEPTSingleAssignment && ast_stack.size() == 1 {
                            pt_stack.lop(1);
                        }
                    } else if want_node == kENodeValue {
                        // Value means list literal or list assignment.
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeListLiteral);
                        *top_node_p = cur_node;
                        ast_stack.push(&mut (*cur_node).children);
                        want_node = kENodeValue;
                        if cur_pt == kEPTAssignment {
                            // Additional assignment parse type allows to easily forbid nested
                            // lists.
                            pt_stack.push(kEPTSingleAssignment);
                        } else if cur_pt == kEPTSingleAssignment {
                            ERROR_FROM_TOKEN_AND_MSG!(
                                cur_token,
                                gettext("E475: Nested lists not allowed when assigning: %.*s")
                            );
                        }
                        HL_CUR_TOKEN!(b"List");
                    } else {
                        // Operator means subscript, also in assignment. But in assignment
                        // subscript may be pretty much any expression, so need to push
                        // kEPTExpr.
                        if prev_token.type_0 == kExprLexSpacing {
                            OP_MISSING!();
                        }
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeSubscript);
                        ADD_OP_NODE!(cur_node);
                        HL_CUR_TOKEN!(b"SubscriptBracket");
                        if pt_is_assignment(cur_pt) {
                            assert!(want_node == kENodeValue);
                            // Subtract 1 for NULL at top.
                            asgn_level = ast_stack.size() - 1;
                            pt_stack.push(kEPTExpr);
                        }
                    }
                }
                kExprLexFigureBrace => {
                    if cur_token.data.brc.closing {
                        let mut new_top_node_p;
                        // Always drop the topmost value:
                        //
                        // 1. When want_node != kENodeValue topmost item on stack is
                        //    a *finished* left operand, which may as well be "{@a}" which
                        //    needs not be finished again.
                        // 2. Otherwise it is pointing to NULL what nobody wants.
                        ast_stack.lop(1);
                        macro_rules! viml_pexpr_parse_figure_brace_closing_error {
                            () => {{
                                assert!(ast_stack.is_empty());
                                ERROR_FROM_TOKEN_AND_MSG!(
                                    cur_token,
                                    gettext("E15: Unexpected closing figure brace: %.*s")
                                );
                                HL_CUR_TOKEN!(b"FigureBrace");
                            }};
                        }
                        if ast_stack.is_empty() {
                            NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeUnknownFigure);
                            (*cur_node).data.fig.type_guesses.allow_lambda = false;
                            (*cur_node).data.fig.type_guesses.allow_dict = false;
                            (*cur_node).data.fig.type_guesses.allow_ident = false;
                            (*cur_node).len = 0;
                            if want_node != kENodeValue {
                                (*cur_node).children = *top_node_p
                            }
                            *top_node_p = cur_node;
                            new_top_node_p = top_node_p;
                            viml_pexpr_parse_figure_brace_closing_error!();
                        } else {
                            if want_node == kENodeValue {
                                if (***ast_stack.last()).type_0 != kExprNodeUnknownFigure
                                    && (***ast_stack.last()).type_0 != kExprNodeComma
                                {
                                    // kv_last being UnknownFigure may occur for empty dictionary
                                    // literal, while Comma is expected in case of non-empty one.
                                    ERROR_FROM_TOKEN_AND_MSG!(
                                        cur_token,
                                        gettext(
                                            "E15: Expected value, got closing figure brace: %.*s"
                                        )
                                    );
                                }
                            }
                            loop {
                                new_top_node_p = ast_stack.pop();
                                if !(!ast_stack.is_empty()
                                    && (new_top_node_p.is_null()
                                        || (**new_top_node_p).type_0 != kExprNodeUnknownFigure
                                            && (**new_top_node_p).type_0 != kExprNodeDictLiteral
                                            && (**new_top_node_p).type_0
                                                != kExprNodeCurlyBracesIdentifier
                                            && (**new_top_node_p).type_0 != kExprNodeLambda))
                                {
                                    break;
                                }
                            }
                            let new_top_node = *new_top_node_p;
                            match (*new_top_node).type_0 {
                                kExprNodeUnknownFigure => {
                                    if (*new_top_node).children.is_null() {
                                        // No children of curly braces node indicates empty dictionary.
                                        assert!(want_node == kENodeValue);
                                        assert!((*new_top_node).data.fig.type_guesses.allow_dict);
                                        SELECT_FIGURE_BRACE_TYPE!(
                                            new_top_node,
                                            DictLiteral,
                                            b"Dict"
                                        );
                                        HL_CUR_TOKEN!(b"Dict");
                                    } else if (*new_top_node).data.fig.type_guesses.allow_ident {
                                        SELECT_FIGURE_BRACE_TYPE!(
                                            new_top_node,
                                            CurlyBracesIdentifier,
                                            b"Curly"
                                        );
                                        HL_CUR_TOKEN!(b"Curly");
                                    } else {
                                        // If by this time type of the node has not already been
                                        // guessed, but it definitely is not a curly braces name then
                                        // it is invalid for sure.
                                        ERROR_FROM_NODE_AND_MSG!(
                                            new_top_node,
                                            gettext(
                                                "E15: Don't know what figure brace means: %.*s"
                                            )
                                        );
                                        if !pstate.colors.is_null() {
                                            // Will reset to NvimInvalidFigureBrace.
                                            (*pstate.colors)
                                                .A((*new_top_node).data.fig.opening_hl_idx)
                                                .group = HL!(b"FigureBrace");
                                        }
                                        HL_CUR_TOKEN!(b"FigureBrace");
                                    }
                                }
                                kExprNodeDictLiteral => {
                                    HL_CUR_TOKEN!(b"Dict");
                                }
                                kExprNodeCurlyBracesIdentifier => {
                                    HL_CUR_TOKEN!(b"Curly");
                                }
                                kExprNodeLambda => {
                                    HL_CUR_TOKEN!(b"Lambda");
                                }
                                _ => {
                                    viml_pexpr_parse_figure_brace_closing_error!();
                                }
                            }
                        }
                        ast_stack.push(new_top_node_p);
                        want_node = kENodeOperator;
                        if ast_stack.size() <= asgn_level {
                            assert!(ast_stack.size() == asgn_level);
                            if cur_pt == kEPTExpr
                                && pt_stack.size() > 1
                                && pt_is_assignment(*pt_stack.Z(1))
                            {
                                pt_stack.lop(1);
                                asgn_level = 0
                            }
                        }
                    } else if want_node == kENodeValue {
                        HL_CUR_TOKEN!(b"FigureBrace");
                        // Value: may be any of lambda, dictionary literal and curly braces
                        // name.
                        // Though if we are in an assignment this may only be a curly braces
                        // name.
                        if pt_is_assignment(cur_pt) {
                            NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeCurlyBracesIdentifier);
                            (*cur_node).data.fig.type_guesses.allow_lambda = false;
                            (*cur_node).data.fig.type_guesses.allow_dict = false;
                            (*cur_node).data.fig.type_guesses.allow_ident = true;
                            pt_stack.push(kEPTExpr);
                        } else {
                            NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeUnknownFigure);
                            (*cur_node).data.fig.type_guesses.allow_lambda = true;
                            (*cur_node).data.fig.type_guesses.allow_dict = true;
                            (*cur_node).data.fig.type_guesses.allow_ident = true
                        }
                        if !pstate.colors.is_null() {
                            (*cur_node).data.fig.opening_hl_idx = (*pstate.colors).size() - 1;
                        }
                        *top_node_p = cur_node;
                        ast_stack.push(&mut (*cur_node).children);
                        pt_stack.push(kEPTLambdaArguments);
                        lambda_node = cur_node;
                    } else {
                        ADD_IDENT!(
                            {
                                NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeCurlyBracesIdentifier);
                                (*cur_node).data.fig.opening_hl_idx = (*pstate.colors).size();
                                (*cur_node).data.fig.type_guesses.allow_lambda = false;
                                (*cur_node).data.fig.type_guesses.allow_dict = false;
                                (*cur_node).data.fig.type_guesses.allow_ident = true;
                                ast_stack.push(&mut (*cur_node).children);
                                if pt_is_assignment(cur_pt) {
                                    pt_stack.push(kEPTExpr);
                                }
                                want_node = kENodeValue;
                            },
                            b"Curly"
                        );
                    }
                    if pt_is_assignment(cur_pt) && !pt_is_assignment(*pt_stack.last()) {
                        assert!(want_node == kENodeValue);
                        asgn_level = ast_stack.size() - 1;
                    }
                }
                kExprLexArrow => {
                    if cur_pt == kEPTLambdaArguments {
                        pt_stack.lop(1);
                        assert!(!pt_stack.is_empty());
                        if want_node == kENodeValue {
                            // Wanting value means trailing comma and NULL at the top of the
                            // stack.
                            ast_stack.lop(1);
                        }
                        assert!(!ast_stack.is_empty());
                        while (***ast_stack.last()).type_0 != kExprNodeLambda
                            && (***ast_stack.last()).type_0 != kExprNodeUnknownFigure
                        {
                            ast_stack.lop(1);
                        }
                        assert!(**ast_stack.last() == lambda_node);
                        SELECT_FIGURE_BRACE_TYPE!(lambda_node, Lambda, b"Lambda");
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeArrow);
                        if (*lambda_node).children.is_null() {
                            assert!(want_node == kENodeValue);
                            (*lambda_node).children = cur_node;
                            ast_stack.push(&mut (*lambda_node).children);
                        } else {
                            assert!((*(*lambda_node).children).next.is_null());
                            (*(*lambda_node).children).next = cur_node;
                            ast_stack.push(&mut (*(*lambda_node).children).next);
                        }
                        ast_stack.push(&mut (*cur_node).children);
                        lambda_node = ptr::null_mut()
                    } else {
                        // Only first branch is valid.
                        ADD_VALUE_IF_MISSING!(gettext("E15: Unexpected arrow: %.*s"));
                        ERROR_FROM_TOKEN_AND_MSG!(
                            cur_token,
                            gettext("E15: Arrow outside of lambda: %.*s")
                        );
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeArrow);
                        ADD_OP_NODE!(cur_node);
                    }
                    want_node = kENodeValue;
                    HL_CUR_TOKEN!(b"Arrow");
                }
                kExprLexPlainIdentifier => {
                    let scope = if cur_token.type_0 == kExprLexInvalid {
                        kExprVarScopeMissing
                    } else {
                        cur_token.data.var.scope
                    };
                    if want_node == kENodeValue {
                        want_node = kENodeOperator;
                        NEW_NODE_WITH_CUR_POS!(
                            cur_node,
                            if node_is_key {
                                kExprNodePlainKey
                            } else {
                                kExprNodePlainIdentifier
                            }
                        );
                        (*cur_node).data.var.scope = scope;
                        let scope_shift = if scope == kExprVarScopeMissing { 0 } else { 2 };
                        (*cur_node).data.var.ident = pline
                            .data
                            .offset((cur_token.start.col + scope_shift) as isize);
                        (*cur_node).data.var.ident_len = cur_token.len - scope_shift;
                        *top_node_p = cur_node;
                        if scope_shift != 0 {
                            assert!(!node_is_key);
                            viml_parser_highlight(
                                pstate,
                                cur_token.start,
                                1,
                                HL!(b"IdentifierScope"),
                            );
                            viml_parser_highlight(
                                pstate,
                                shifted_pos(cur_token.start, 1),
                                1,
                                HL!(b"IdentifierScopeDelimiter"),
                            );
                        }
                        viml_parser_highlight(
                            pstate,
                            shifted_pos(cur_token.start, scope_shift),
                            cur_token.len - scope_shift,
                            if node_is_key {
                                HL!(b"IdentifierKey")
                            } else {
                                HL!(b"IdentifierName")
                            },
                        );
                    } else if scope == kExprVarScopeMissing {
                        assert!(want_node == kENodeOperator);
                        ADD_IDENT!(
                            {
                                NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodePlainIdentifier);
                                (*cur_node).data.var.scope = scope;
                                (*cur_node).data.var.ident =
                                    pline.data.offset(cur_token.start.col as isize);
                                (*cur_node).data.var.ident_len = cur_token.len;
                                want_node = kENodeOperator;
                            },
                            b"IdentifierName"
                        );
                    } else {
                        OP_MISSING!();
                    }
                }
                kExprLexNumber => {
                    if want_node != kENodeValue {
                        OP_MISSING!();
                    }
                    if node_is_key {
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodePlainKey);
                        (*cur_node).data.var.ident =
                            pline.data.offset(cur_token.start.col as isize);
                        (*cur_node).data.var.ident_len = cur_token.len;
                        HL_CUR_TOKEN!(b"IdentifierKey");
                    } else if cur_token.data.num.is_float {
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeFloat);
                        (*cur_node).data.flt.value = cur_token.data.num.val.floating;
                        HL_CUR_TOKEN!(b"Float");
                    } else {
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeInteger);
                        (*cur_node).data.num.value = cur_token.data.num.val.integer;
                        let prefix_length = base_to_prefix_length(cur_token.data.num.base);
                        viml_parser_highlight(
                            pstate,
                            cur_token.start,
                            prefix_length as libc::size_t,
                            HL!(b"NumberPrefix"),
                        );
                        viml_parser_highlight(
                            pstate,
                            shifted_pos(cur_token.start, prefix_length as libc::size_t),
                            cur_token.len - prefix_length as usize,
                            HL!(b"Number"),
                        );
                    }
                    want_node = kENodeOperator;
                    *top_node_p = cur_node;
                }
                kExprLexDot => {
                    ADD_VALUE_IF_MISSING!(gettext("E15: Unexpected dot: %.*s"));
                    if prev_token.type_0 == kExprLexSpacing {
                        if cur_pt == kEPTAssignment {
                            ERROR_FROM_TOKEN_AND_MSG!(
                                cur_token,
                                gettext("E15: Cannot concatenate in assignments: %.*s")
                            );
                        }
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeConcat);
                        HL_CUR_TOKEN!(b"Concat");
                    } else {
                        NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeConcatOrSubscript);
                        HL_CUR_TOKEN!(b"ConcatOrSubscript");
                    }
                    ADD_OP_NODE!(cur_node);
                }
                kExprLexParenthesis => {
                    if cur_token.data.brc.closing {
                        if want_node == kENodeValue {
                            let mut goto_no_paren_closing_error = false;
                            if ast_stack.size() > 1 {
                                let prev_top_node = **ast_stack.Z(1);
                                if (*prev_top_node).type_0 == kExprNodeCall {
                                    // Function call without arguments, this is not an error.
                                    // But further code does not expect NULL nodes.
                                    ast_stack.lop(1);
                                    goto_no_paren_closing_error = true;
                                }
                            }
                            if !goto_no_paren_closing_error {
                                ERROR_FROM_TOKEN_AND_MSG!(
                                    cur_token,
                                    gettext("E15: Expected value, got parenthesis: %.*s")
                                );
                                NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeMissing);
                                (*cur_node).len = 0;
                                *top_node_p = cur_node
                            }
                        } else {
                            // Always drop the topmost value: when want_node != kENodeValue
                            // topmost item on stack is a *finished* left operand, which may as
                            // well be "(@a)" which needs not be finished again.
                            ast_stack.lop(1);
                        }
                        let mut new_top_node_p: *mut *mut ExprASTNode = ptr::null_mut();
                        while !ast_stack.is_empty()
                            && (new_top_node_p.is_null()
                                || (**new_top_node_p).type_0 != kExprNodeNested
                                    && (**new_top_node_p).type_0 != kExprNodeCall)
                        {
                            new_top_node_p = ast_stack.pop();
                        }
                        if !new_top_node_p.is_null()
                            && ((**new_top_node_p).type_0 == kExprNodeNested
                                || (**new_top_node_p).type_0 == kExprNodeCall)
                        {
                            if (**new_top_node_p).type_0 == kExprNodeNested {
                                HL_CUR_TOKEN!(b"NestingParenthesis");
                            } else {
                                HL_CUR_TOKEN!(b"CallingParenthesis");
                            }
                        } else {
                            // “Always drop the topmost value” branch has got rid of the single
                            // value stack had, so there is nothing known to enclose. Correct
                            // this.
                            if new_top_node_p.is_null() {
                                new_top_node_p = top_node_p
                            }
                            ERROR_FROM_TOKEN_AND_MSG!(
                                cur_token,
                                gettext("E15: Unexpected closing parenthesis: %.*s")
                            );
                            HL_CUR_TOKEN!(b"NestingParenthesis");
                            cur_node = NEW_NODE!(kExprNodeNested);
                            (*cur_node).start = cur_token.start;
                            (*cur_node).len = 0;
                            // Unexpected closing parenthesis, assume that it was wanted to
                            // enclose everything in ().
                            (*cur_node).children = *new_top_node_p;
                            *new_top_node_p = cur_node;
                            assert!((*cur_node).next.is_null());
                        }
                        ast_stack.push(new_top_node_p);
                        want_node = kENodeOperator;
                    } else {
                        match want_node {
                            kENodeValue => {
                                NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeNested);
                                *top_node_p = cur_node;
                                ast_stack.push(&mut (*cur_node).children);
                                HL_CUR_TOKEN!(b"NestingParenthesis");
                            }
                            kENodeOperator => {
                                if prev_token.type_0 == kExprLexSpacing {
                                    // For some reason "function (args)" is a function call, but
                                    // "(funcref) (args)" is not. AFAIR this somehow involves
                                    // compatibility and Bram was commenting that this is
                                    // intentionally inconsistent and he is not very happy with the
                                    // situation himself.
                                    if (**top_node_p).type_0 != kExprNodePlainIdentifier
                                        && (**top_node_p).type_0 != kExprNodeComplexIdentifier
                                        && (**top_node_p).type_0 != kExprNodeCurlyBracesIdentifier
                                    {
                                        OP_MISSING!();
                                    }
                                }
                                NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeCall);
                                ADD_OP_NODE!(cur_node);
                                HL_CUR_TOKEN!(b"CallingParenthesis");
                            }
                        }
                        want_node = kENodeValue;
                    }
                }
                kExprLexQuestion => {
                    ADD_VALUE_IF_MISSING!(gettext("E15: Expected value, got question mark: %.*s"));
                    NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeTernary);
                    ADD_OP_NODE!(cur_node);
                    HL_CUR_TOKEN!(b"Ternary");
                    let mut ter_val_node;
                    NEW_NODE_WITH_CUR_POS!(ter_val_node, kExprNodeTernaryValue);
                    (*ter_val_node).data.ter.got_colon = false;
                    assert!(!(*cur_node).children.is_null());
                    assert!((*(*cur_node).children).next.is_null());
                    assert!(*ast_stack.last() == &mut (*(*cur_node).children).next);
                    **ast_stack.last() = ter_val_node;
                    ast_stack.push(&mut (*ter_val_node).children);
                }
                kExprLexDoubleQuotedString | kExprLexSingleQuotedString => {
                    let is_double = tok_type == kExprLexDoubleQuotedString;
                    if !cur_token.data.str_0.closed {
                        // It is weird, but Vim has two identical errors messages with
                        // different error numbers: "E114: Missing quote" and
                        // "E115: Missing quote".
                        ERROR_FROM_TOKEN_AND_MSG!(
                            cur_token,
                            if is_double {
                                gettext("E114: Missing double quote: %.*s")
                            } else {
                                gettext("E115: Missing single quote: %.*s")
                            },
                        );
                    }
                    if want_node == kENodeOperator {
                        OP_MISSING!();
                    }
                    NEW_NODE_WITH_CUR_POS!(
                        cur_node,
                        if is_double {
                            kExprNodeDoubleQuotedString
                        } else {
                            kExprNodeSingleQuotedString
                        }
                    );
                    *top_node_p = cur_node;
                    parse_quoted_string(pstate, cur_node.as_mut().unwrap(), cur_token, is_invalid);
                    want_node = kENodeOperator;
                }
                kExprLexAssignment => {
                    if cur_pt == kEPTAssignment {
                        pt_stack.lop(1);
                    } else if cur_pt == kEPTSingleAssignment {
                        pt_stack.lop(2);
                        ERROR_FROM_TOKEN_AND_MSG!(
                            cur_token,
                            gettext(
                                "E475: Expected closing bracket to end list assignment lvalue: %.*s"
                            )
                        );
                    } else {
                        ERROR_FROM_TOKEN_AND_MSG!(
                            cur_token,
                            gettext("E15: Misplaced assignment: %.*s")
                        );
                    }
                    assert!(!pt_stack.is_empty());
                    assert!(*pt_stack.last() == kEPTExpr);
                    ADD_VALUE_IF_MISSING!(gettext("E15: Unexpected assignment: %.*s"));
                    NEW_NODE_WITH_CUR_POS!(cur_node, kExprNodeAssignment);
                    (*cur_node).data.ass.type_0 = cur_token.data.ass.type_0;
                    match cur_token.data.ass.type_0 {
                        kExprAsgnPlain => HL_CUR_TOKEN!(b"PlainAssignment"),
                        kExprAsgnAdd => HL_CUR_TOKEN!(b"AssignmentWithAddition"),
                        kExprAsgnSubtract => HL_CUR_TOKEN!(b"AssignmentWithSubtraction"),
                        kExprAsgnConcat => HL_CUR_TOKEN!(b"AssignmentWithConcatenation"),
                    }
                    ADD_OP_NODE!(cur_node);
                }
            }
            break;
        }
        // viml_pexpr_parse_cycle_end
        prev_token = cur_token;
        highlighted_prev_spacing = false;
        viml_parser_advance(pstate, cur_token.len);
    }
    // viml_pexpr_parse_end
    assert!(!pt_stack.is_empty());
    assert!(!ast_stack.is_empty());
    if want_node == kENodeValue
        // Blacklist some parse type entries as their presence means better error
        // message in the other branch.
        && *pt_stack.last() != kEPTLambdaArguments
    {
        east_set_error(
            pstate,
            &mut ast.err,
            gettext("E15: Expected value, got EOC: %.*s"),
            pstate.pos,
        );
    } else if ast_stack.size() != 1 {
        // Something may be wrong, check whether it really is.

        // Pointer to ast.root must never be dropped, so “!= 1” is expected to be
        // the same as “> 1”.
        assert!(!ast_stack.is_empty());
        // Topmost stack item must be a *finished* value, so it must not be
        // analyzed. E.g. it may contain an already finished nested expression.
        ast_stack.lop(1);
        while ast.err.msg.is_null() && !ast_stack.is_empty() {
            let cur_node_0: *const ExprASTNode = *ast_stack.pop();
            // This should only happen when want_node == kENodeValue.
            assert!(!cur_node_0.is_null());
            // TODO(ZyX-I): Rehighlight as invalid?
            match (*cur_node_0).type_0 {
                kExprNodeOpMissing | kExprNodeMissing => {
                    // Error should’ve been already reported.
                }
                kExprNodeCall => {
                    east_set_error(
                        pstate,
                        &mut ast.err,
                        gettext("E116: Missing closing parenthesis for function call: %.*s"),
                        (*cur_node_0).start,
                    );
                }
                kExprNodeNested => {
                    east_set_error(
                        pstate,
                        &mut ast.err,
                        gettext("E110: Missing closing parenthesis for nested expression: %.*s"),
                        (*cur_node_0).start,
                    );
                }
                kExprNodeListLiteral => {
                    // For whatever reason "[1" yields "E696: Missing comma in list" error
                    // in Vim while "[1," yields E697.
                    east_set_error(
                        pstate,
                        &mut ast.err,
                        gettext("E697: Missing end of List \']\': %.*s"),
                        (*cur_node_0).start,
                    );
                }
                kExprNodeDictLiteral => {
                    // Same problem like with list literal with E722 (missing comma) vs
                    // E723, but additionally just "{" yields only E15.
                    east_set_error(
                        pstate,
                        &mut ast.err,
                        gettext("E723: Missing end of Dictionary \'}\': %.*s"),
                        (*cur_node_0).start,
                    );
                }
                kExprNodeUnknownFigure => {
                    east_set_error(
                        pstate,
                        &mut ast.err,
                        gettext("E15: Missing closing figure brace: %.*s"),
                        (*cur_node_0).start,
                    );
                }
                kExprNodeLambda => {
                    east_set_error(
                        pstate,
                        &mut ast.err,
                        gettext("E15: Missing closing figure brace for lambda: %.*s"),
                        (*cur_node_0).start,
                    );
                }
                kExprNodeCurlyBracesIdentifier => {
                    // Until trailing "}" it is impossible to distinguish curly braces
                    // identifier and dictionary, so it must not appear in the stack like
                    // this.
                    unreachable!();
                }
                kExprNodeInteger
                | kExprNodeFloat
                | kExprNodeSingleQuotedString
                | kExprNodeDoubleQuotedString
                | kExprNodeOption
                | kExprNodeEnvironment
                | kExprNodeRegister
                | kExprNodePlainIdentifier
                | kExprNodePlainKey => {
                    // These are plain values and not containers, for them it should only
                    // be possible to show up in the topmost stack element, but it was
                    // unconditionally popped at the start.
                    unreachable!();
                }
                kExprNodeComma | kExprNodeColon | kExprNodeArrow => {
                    // It is actually only valid inside something else, but everything
                    // where one of the above is valid requires to be closed and thus is
                    // to be caught later.
                }
                kExprNodeSubscript
                | kExprNodeConcatOrSubscript
                | kExprNodeComplexIdentifier
                | kExprNodeAssignment
                | kExprNodeMod
                | kExprNodeDivision
                | kExprNodeMultiplication
                | kExprNodeNot
                | kExprNodeAnd
                | kExprNodeOr
                | kExprNodeConcat
                | kExprNodeComparison
                | kExprNodeUnaryMinus
                | kExprNodeUnaryPlus
                | kExprNodeBinaryMinus
                | kExprNodeTernary
                | kExprNodeBinaryPlus => {
                    // It is OK to see these in the stack.
                }
                kExprNodeTernaryValue => {
                    if !(*cur_node_0).data.ter.got_colon {
                        // Actually Vim throws E109 in more cases.
                        east_set_error(
                            pstate,
                            &mut ast.err,
                            gettext("E109: Missing \':\' after \'?\': %.*s"),
                            (*cur_node_0).start,
                        );
                    }
                }
            }
        }
    }
    ast_stack.destroy();
    return ast;
}
