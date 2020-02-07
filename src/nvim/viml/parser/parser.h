#ifndef NVIM_VIML_PARSER_PARSER_H
#define NVIM_VIML_PARSER_PARSER_H

#include <stdbool.h>
#include <stddef.h>
#include <assert.h>

#include "nvim/lib/kvec.h"
#include "nvim/func_attr.h"
#include "nvim/mbyte.h"
#include "nvim/memory.h"

/// One parsed line
typedef struct {
  const char *data;  ///< Parsed line pointer
  size_t size;  ///< Parsed line size
  bool allocated;  ///< True if line may be freed.
} ParserLine;

/// Line getter type for parser
///
/// Line getter must return {NULL, 0} for EOF.
typedef void (*ParserLineGetter)(void *cookie, ParserLine *ret_pline);

/// Parser position in the input
typedef struct {
  size_t line;  ///< Line index in ParserInputReader.lines.
  size_t col;  ///< Byte index in the line.
} ParserPosition;

/// Parser state item.
typedef struct {
  enum {
    kPTopStateParsingCommand = 0,
    kPTopStateParsingExpression,
  } type;
  union {
    struct {
      enum {
        kExprUnknown = 0,
      } type;
    } expr;
  } data;
} ParserStateItem;

/// Structure defining input reader
typedef struct {
  /// Function used to get next line.
  ParserLineGetter get_line;
  /// Data for get_line function.
  void *cookie;
  /// All lines obtained by get_line.
  kvec_withinit_t(ParserLine, 4) lines;
  /// Conversion, for :scriptencoding.
  vimconv_T conv;
} ParserInputReader;

/// Highlighted region definition
///
/// Note: one chunk may highlight only one line.
typedef struct {
  ParserPosition start;  ///< Start of the highlight: line and column.
  size_t end_col;  ///< End column, points to the start of the next character.
  const char *group;  ///< Highlight group.
} ParserHighlightChunk;

/// Highlighting defined by a parser
typedef kvec_withinit_t(ParserHighlightChunk, 16) ParserHighlight;

/// Structure defining parser state
typedef struct {
  /// Line reader.
  ParserInputReader reader;
  /// Position up to which input was parsed.
  ParserPosition pos;
  /// Parser state stack.
  kvec_withinit_t(ParserStateItem, 16) stack;
  /// Highlighting support.
  ParserHighlight *colors;
  /// True if line continuation can be used.
  bool can_continuate;
} ParserState;

static inline void viml_parser_init(
    ParserState *const ret_pstate,
    const ParserLineGetter get_line, void *const cookie,
    ParserHighlight *const colors)
  REAL_FATTR_NONNULL_ARG(1, 2);

/// Initialize a new parser state instance
///
/// @param[out]  ret_pstate  Parser state to initialize.
/// @param[in]  get_line  Line getter function.
/// @param[in]  cookie  Argument for the get_line function.
/// @param[in]  colors  Where to save highlighting. May be NULL if it is not
///                     needed.
static inline void viml_parser_init(
    ParserState *const ret_pstate,
    const ParserLineGetter get_line, void *const cookie,
    ParserHighlight *const colors)
{
  *ret_pstate = (ParserState) {
    .reader = {
      .get_line = get_line,
      .cookie = cookie,
      .conv = MBYTE_NONE_CONV,
    },
    .pos = { 0, 0 },
    .colors = colors,
    .can_continuate = false,
  };
  kvi_init(ret_pstate->reader.lines);
  kvi_init(ret_pstate->stack);
}

static inline void viml_parser_destroy(ParserState *const pstate)
  REAL_FATTR_NONNULL_ALL ;

/// Free all memory allocated by the parser on heap
///
/// @param  pstate  Parser state to free.
static inline void viml_parser_destroy(ParserState *const pstate)
{
  for (size_t i = 0; i < kv_size(pstate->reader.lines); i++) {
    ParserLine pline = kv_A(pstate->reader.lines, i);
    if (pline.allocated) {
      xfree((void *)pline.data);
    }
  }
  kvi_destroy(pstate->reader.lines);
  kvi_destroy(pstate->stack);
}

void viml_preader_get_line(ParserInputReader *const preader,
                                         ParserLine *const ret_pline)
  REAL_FATTR_NONNULL_ALL;

bool viml_parser_get_remaining_line(ParserState *const pstate,
                                                  ParserLine *const ret_pline)
  REAL_FATTR_WARN_UNUSED_RESULT REAL_FATTR_NONNULL_ALL;

void viml_parser_advance(ParserState *const pstate,
                                       const size_t len)
  REAL_FATTR_NONNULL_ALL;

void viml_parser_highlight(ParserState *const pstate,
                                         const ParserPosition start,
                                         const size_t end_col,
                                         const char *const group)
  REAL_FATTR_NONNULL_ALL;
void parser_simple_get_line(void *cookie, ParserLine *ret_pline);

#endif  // NVIM_VIML_PARSER_PARSER_H
