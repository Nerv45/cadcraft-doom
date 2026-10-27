/* SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 CADCraft DOOM contributors.
 * Native host contract for the retained GPL engine and the CADCraft worker.
 * Implemented for this integration; see vendor/PROVENANCE.md.
 */
#ifndef CADCRAFT_PLATFORM_CONTRACT_H
#define CADCRAFT_PLATFORM_CONTRACT_H

#include <stdint.h>

#if defined(CMAP256) || defined(FEATURE_SOUND)
#error CADCraft supports the 32-bit silent embedded engine configuration only
#endif

enum {
    CC_DOOM_FRAME_WIDTH = 320,
    CC_DOOM_FRAME_HEIGHT = 200,
    CC_DOOM_FRAME_PIXELS = CC_DOOM_FRAME_WIDTH * CC_DOOM_FRAME_HEIGHT
};

/* Engine output is worker-owned; DG_DrawFrame copies it into the locked mailbox. */
extern uint32_t *const DG_ScreenBuffer;

/* Entry points supplied by engine_host.c and the retained d_main.c. */
void cc_start_game(int argument_count, char **argument_values);
void doomgeneric_Tick(void);

/* Callbacks consumed by the engine's input/video/timer code. */
void DG_Init(void);
void DG_DrawFrame(void);
void DG_SleepMs(uint32_t delay_ms);
uint32_t DG_GetTicksMs(void);
int DG_GetKey(int *down, unsigned char *code);
void DG_SetWindowTitle(const char *title);

#endif
