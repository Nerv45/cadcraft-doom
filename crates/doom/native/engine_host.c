/* SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 CADCraft DOOM contributors.
 * CADCraft-owned engine bootstrap and framebuffer storage.
 * Interface requirements come from the retained GPL engine and DG callback API;
 * no upstream doomgeneric.c, dummy.c or autoheader implementation is included.
 */
#include "cadcraft_platform.h"
#include "m_argv.h"
#include "net_client.h"

extern void D_DoomMain(void);

static uint32_t host_framebuffer[CC_DOOM_FRAME_PIXELS];
uint32_t *const DG_ScreenBuffer = host_framebuffer;

/* Definitions for the externs declared by the GPL-2.0-or-later net_client.h.
 * No networking backend is linked into this single-player build.
 */
boolean drone = false;
boolean net_client_connected = false;

void cc_start_game(int argument_count, char **argument_values)
{
    /* The caller owns a process-lifetime, validated argument vector. The host
     * surface is static: startup does not allocate or export a heap buffer.
     */
    DG_Init();
    myargv = argument_values;
    myargc = argument_count;
    M_FindResponseFile();
    D_DoomMain();
}
