/* CADCraft DOOM modifications, GPL-2.0-or-later. Include CRT before redirect. */
#ifndef CADCRAFT_EMBED_REDIRECT
#define CADCRAFT_EMBED_REDIRECT
#include <stdlib.h>
__declspec(noreturn) void cc_engine_exit(int code);
__declspec(noreturn) void cc_fail(const char *format, ...);
#define exit cc_engine_exit
#endif
