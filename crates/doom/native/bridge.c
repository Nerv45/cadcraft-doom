/* CADCraft DOOM host bridge. GPL-2.0-or-later.
 * Only the worker enters engine code. longjmp never crosses an FFI callback.
 * Engine storage is deliberately retained: upstream has no safe reset API.
 */
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <setjmp.h>
#include <stdarg.h>
#include <stdio.h>
#include <string.h>
#include "cadcraft_platform.h"

static SRWLOCK frame_lock = SRWLOCK_INIT;
static SRWLOCK key_lock = SRWLOCK_INIT;
static unsigned char rgba[CC_DOOM_FRAME_PIXELS * 4];
static unsigned char desired[256], delivered[256];
static unsigned short events[1024];
static unsigned int head, tail;
static unsigned long long sequence;
static volatile LONG stopping;
static jmp_buf escape;
static char failure[512];
static ULONGLONG started, last_draw;
static int result;

void cc_prepare(void) {
    InterlockedExchange(&stopping, 0);
    AcquireSRWLockExclusive(&key_lock);
    head = tail = 0;
    memset(desired, 0, sizeof(desired));
    memset(delivered, 0, sizeof(delivered));
    ReleaseSRWLockExclusive(&key_lock);
}
void cc_stop(void) { InterlockedExchange(&stopping, 1); }
void cc_keys(const unsigned char *keys) {
    AcquireSRWLockExclusive(&key_lock);
    int i;
    for (i = 0; i < 256; ++i) {
        if (desired[i] != keys[i]) {
            unsigned int next = (tail + 1) % 1024;
            if (next == head) { head = tail = 0; next = 1; }
            events[tail] = (unsigned short)(i | (keys[i] ? 256 : 0));
            tail = next;
            desired[i] = keys[i] ? 1 : 0;
        }
    }
    ReleaseSRWLockExclusive(&key_lock);
}
int cc_frame(unsigned long long *last, unsigned char *out) {
    int fresh = 0;
    if (!TryAcquireSRWLockShared(&frame_lock)) return 0;
    if (*last != sequence) {
        memcpy(out, rgba, sizeof(rgba));
        *last = sequence;
        fresh = 1;
    }
    ReleaseSRWLockShared(&frame_lock);
    return fresh;
}
__declspec(noreturn) void cc_engine_exit(int code) {
    result = code;
    if (code && !failure[0]) snprintf(failure, sizeof(failure), "DoomGeneric exited with code %d", code);
    longjmp(escape, 1);
}
__declspec(noreturn) void cc_fail(const char *format, ...) {
    va_list args;
    va_start(args, format);
    vsnprintf(failure, sizeof(failure), format, args);
    va_end(args);
    cc_engine_exit(1);
}
static void check_stop(void) {
    if (InterlockedCompareExchange(&stopping, 0, 0)) cc_engine_exit(0);
}
void DG_Init(void) {
    if (!DG_ScreenBuffer) cc_fail("Cannot allocate DOOM framebuffer");
    check_stop();
}
void DG_DrawFrame(void) {
    unsigned int i;
    ULONGLONG now = GetTickCount64();
    check_stop();
    if (last_draw && now - last_draw < 28) DG_SleepMs((uint32_t)(28 - (now - last_draw)));
    last_draw = GetTickCount64();
    AcquireSRWLockExclusive(&frame_lock);
    for (i = 0; i < CC_DOOM_FRAME_PIXELS; ++i) {
        unsigned int p = DG_ScreenBuffer[i]; /* i_video.c packs 0x00RRGGBB */
        rgba[4*i] = (unsigned char)(p >> 16);
        rgba[4*i+1] = (unsigned char)(p >> 8);
        rgba[4*i+2] = (unsigned char)p;
        rgba[4*i+3] = 255;
    }
    ++sequence;
    ReleaseSRWLockExclusive(&frame_lock);
}
void DG_SleepMs(uint32_t ms) {
    do {
        DWORD part = ms > 10 ? 10 : ms;
        check_stop();
        Sleep(part ? part : 1);
        ms -= part;
    } while (ms);
}
uint32_t DG_GetTicksMs(void) { check_stop(); return (uint32_t)(GetTickCount64() - started); }
int DG_GetKey(int *pressed, unsigned char *key) {
    int i;
    check_stop();
    AcquireSRWLockExclusive(&key_lock);
    if (head != tail) {
        unsigned short event = events[head];
        head = (head + 1) % 1024;
        *key = (unsigned char)event;
        *pressed = (event & 256) != 0;
        delivered[*key] = (unsigned char)*pressed;
        ReleaseSRWLockExclusive(&key_lock);
        return 1;
    }
    for (i = 0; i < 256; ++i) {
        if (desired[i] != delivered[i]) {
            *pressed = desired[i];
            *key = (unsigned char)i;
            delivered[i] = desired[i];
            ReleaseSRWLockExclusive(&key_lock);
            return 1;
        }
    }
    ReleaseSRWLockExclusive(&key_lock);
    return 0;
}
void DG_SetWindowTitle(const char *title) { (void)title; }
static void run_guarded(int argc, char **argv) {
    if (setjmp(escape) == 0) {
        check_stop();
        cc_start_game(argc, argv);
        for (;;) {
            check_stop();
            doomgeneric_Tick();
            Sleep(1); /* never busy spin, even when Tick returns early */
        }
    }
}
int cc_run(int argc, char **argv, char *error, unsigned int capacity) {
    failure[0] = 0;
    result = 0;
    started = GetTickCount64();
    __try { run_guarded(argc, argv); }
    __except(EXCEPTION_EXECUTE_HANDLER) {
        result = 1;
        snprintf(failure, sizeof(failure), "DoomGeneric native fault 0x%08lx; restart CADCraft before playing again", GetExceptionCode());
    }
    if (capacity) {
        strncpy(error, failure, capacity - 1);
        error[capacity - 1] = 0;
    }
    return result;
}




