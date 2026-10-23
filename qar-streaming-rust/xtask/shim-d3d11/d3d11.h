/* Minimal stand-in for d3d11.h, used only to generate bindings. The COM
 * interfaces are opaque: Rust callers pass pointers obtained from their own
 * Direct3D bindings (for example the `windows` crate's `Interface::as_raw`). */
#ifndef QAROS_SHIM_D3D11_H
#define QAROS_SHIM_D3D11_H

typedef struct ID3D11Texture2D ID3D11Texture2D;
typedef struct IDXGIKeyedMutex IDXGIKeyedMutex;

#endif
