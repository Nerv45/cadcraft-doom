/* SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 CADCraft DOOM contributors.
 * Minimal build settings for the Windows single-player host. This is not an
 * edited copy of the upstream autoheader output.
 */
#ifndef CADCRAFT_ENGINE_BUILD_SETTINGS_H
#define CADCRAFT_ENGINE_BUILD_SETTINGS_H

#define PACKAGE_NAME "CADCraft DOOM"
#define PACKAGE_STRING "CADCraft DOOM embedded engine 0.1.0"
#define PROGRAM_PREFIX "cadcraft-doom-"
#define FILES_DIR "."

/* Undefined optional flags keep mmap, PNG screenshots, networking and sound off. */
#if defined(ORIGCODE) || defined(FEATURE_SOUND) || defined(CMAP256)
#error Unsupported legacy, sound or indexed-framebuffer build option
#endif

#endif
