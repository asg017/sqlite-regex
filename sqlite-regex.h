#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "sqlite3ext.h"

/**
 * # Safety
 *
 * Should only be called by underlying SQLite C APIs,
 * like sqlite3_auto_extension and sqlite3_cancel_auto_extension.
 */
unsigned int sqlite3_regex_init(sqlite3 *db, char **pz_err_msg, sqlite3_api_routines *p_api);
