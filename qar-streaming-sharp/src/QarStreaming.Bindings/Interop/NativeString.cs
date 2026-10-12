using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros;

namespace Quaternar.Qaros.Interop
{
    internal static class NativeString
    {
        internal delegate ResultNative NativeStringCall(byte[] buffer, UIntPtr size);

        internal static Result<string> GetString(NativeStringCall call, int bufferSize = NativeConstants.MaxStringLength)
        {
            var buffer = new byte[bufferSize];
            var info = Results.ToInfo(call(buffer, (UIntPtr)buffer.Length));
            return info.IsSuccess ? Result<string>.Success(info, DecodeNullTerminated(buffer)) : Result<string>.Failure(info);
        }

        internal static string DecodeNullTerminated(byte[] buffer)
        {
            var length = 0;
            while (length < buffer.Length && buffer[length] != 0)
                length++;
            return Encoding.UTF8.GetString(buffer, 0, length);
        }

        /// <summary>
        /// A NUL-terminated string the native side owns, read as UTF-8 like every string of the C
        /// API. <c>Marshal.PtrToStringAnsi</c> would read it in the ANSI code page on Windows, and
        /// <c>Marshal.PtrToStringUTF8</c> is missing from netstandard2.0. Null for a null pointer.
        /// </summary>
        internal static string? DecodeNullTerminated(IntPtr pointer)
        {
            if (pointer == IntPtr.Zero)
                return null;
            var length = 0;
            while (Marshal.ReadByte(pointer, length) != 0)
                length++;
            var bytes = new byte[length];
            Marshal.Copy(pointer, bytes, 0, length);
            return Encoding.UTF8.GetString(bytes, 0, length);
        }
    }

    internal sealed class NativeUtf8String : IDisposable
    {
        private NativeUtf8String(IntPtr pointer)
        {
            Pointer = pointer;
        }

        public IntPtr Pointer { get; private set; }

        /// <summary>
        /// A NUL-terminated UTF-8 copy of a UTF-16 string: every <c>char*</c> of the C API is
        /// UTF-8, whatever the ANSI code page. Null for null.
        /// </summary>
        public static NativeUtf8String FromUtf8(string? value)
        {
            if (value == null)
                return new NativeUtf8String(IntPtr.Zero);
            var bytes = Encoding.UTF8.GetBytes(value);
            var ptr = Marshal.AllocHGlobal(bytes.Length + 1);
            Marshal.Copy(bytes, 0, ptr, bytes.Length);
            Marshal.WriteByte(ptr, bytes.Length, 0);
            return new NativeUtf8String(ptr);
        }

        public void Dispose()
        {
            if (Pointer != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(Pointer);
                Pointer = IntPtr.Zero;
            }
        }
    }
}

