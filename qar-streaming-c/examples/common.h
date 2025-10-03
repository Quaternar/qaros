#pragma once
#include "qar_streaming.h"
#include <inttypes.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/** \brief Print a fixed-size identifier as hex bytes separated by colons. */
static void
print_hex_id(const uint8_t* data, size_t len)
{
    for(size_t i = 0; i < len; ++i)
    {
        printf("%02X", (unsigned)data[i]);
        if(i + 1 < len)
        {
            printf(":");
        }
    }
}

/** \brief Log a \ref QarResult with a label to stdout. */
static void
log_result(const char* label, QarResult r)
{
    if(qar_result_is_success(r))
    {
        printf("%s: SUCCESS (code=%d)\n", label, (int)r.code);
    }
    else
    {
        char msg[256] = { 0 };
        qar_result_message(r, msg, sizeof(msg));
        printf("%s: ERROR (code=%d) msg='%s'\n", label, (int)r.code, msg);
    }
}

/** \brief Extract directory portion from a path (both '/' and '\\' on Windows).
 */
static const char*
get_dir_from_path(const char* path, char* out_dir, size_t out_sz)
{
    if(!path || !out_dir || out_sz == 0)
    {
        return NULL;
    }
    size_t n = strlen(path);
    size_t i = n;
    while(i > 0)
    {
        char c = path[i - 1];
        if(c == '/' || c == '\\')
        {
            break;
        }
        --i;
    }
    if(i == 0)
    {
        // No separator found; use current directory
        snprintf(out_dir, out_sz, ".");
        return out_dir;
    }
    size_t dir_len = (i > out_sz - 1) ? (out_sz - 1) : i;
    memcpy(out_dir, path, dir_len);
    out_dir[dir_len] = '\0';
    return out_dir;
}