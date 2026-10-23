/* Minimal stand-in for the Windows SDK header, used only to generate bindings
 * on machines without the Windows SDK. Declares the Windows types that
 * qar_streaming.h uses, with their documented layouts. */
#ifndef QAROS_SHIM_WINDOWS_H
#define QAROS_SHIM_WINDOWS_H

typedef unsigned long DWORD;
typedef long LONG;

typedef struct _LUID
{
    DWORD LowPart;
    LONG HighPart;
} LUID;

#endif
