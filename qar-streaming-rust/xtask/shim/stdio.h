/* Freestanding stand-in: binding generation never needs stdio declarations,
 * because the header's printf/fprintf calls sit behind
 * QAR_ENABLE_DYNAMIC_LOADING, which generation leaves undefined. */
#ifndef QAROS_SHIM_STDIO_H
#define QAROS_SHIM_STDIO_H
#endif
