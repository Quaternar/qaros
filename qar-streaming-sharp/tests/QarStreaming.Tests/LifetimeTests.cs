using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using System.Reflection.Emit;
using Quaternar.Qaros;
using Quaternar.Qaros.Interop;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    public sealed class LifetimeTests
    {
        [Fact]
        public void RuntimeHandleReleaseDoesNotDestroyTheSharedRuntime()
        {
            // Inspect the actual cleanup call target instead of loading a native runtime:
            // calling the wrong destructor here would stop a user's shared Hub.
            var release = typeof(RuntimeSafeHandle).GetMethod("ReleaseHandle", BindingFlags.Instance | BindingFlags.NonPublic)!;
            var nativeCalls = CalledMethods(release).Where(method => method.DeclaringType == typeof(NativeMethods)).ToArray();
            var call = Assert.Single(nativeCalls);
            Assert.Equal(nameof(NativeMethods.qar_impl_runtime_handle_destroy), call.Name);
            var dispose = typeof(Runtime).GetMethod(nameof(Runtime.Dispose))!;
            Assert.DoesNotContain(CalledMethods(dispose), method => method.Name == nameof(NativeMethods.qar_impl_runtime_destroy));
        }

        private static IEnumerable<MethodBase> CalledMethods(MethodInfo method)
        {
            var opcodes = typeof(OpCodes).GetFields(BindingFlags.Public | BindingFlags.Static)
                .Where(field => field.FieldType == typeof(OpCode))
                .Select(field => (OpCode)field.GetValue(null)!).ToDictionary(opcode => opcode.Value);
            var body = method.GetMethodBody()!.GetILAsByteArray()!;
            for (var offset = 0; offset < body.Length;)
            {
                var code = (short)body[offset++];
                if (code == 0xfe)
                    code = (short)(0xfe00 | body[offset++]);
                var opcode = opcodes[code];
                if (opcode == OpCodes.Call || opcode == OpCodes.Callvirt)
                    yield return method.Module.ResolveMethod(BitConverter.ToInt32(body, offset))!;
                switch (opcode.OperandType)
                {
                    case OperandType.InlineNone: break;
                    case OperandType.ShortInlineBrTarget:
                    case OperandType.ShortInlineI:
                    case OperandType.ShortInlineVar: offset += 1; break;
                    case OperandType.InlineVar: offset += 2; break;
                    case OperandType.InlineI8:
                    case OperandType.InlineR: offset += 8; break;
                    case OperandType.InlineSwitch: offset += 4 + 4 * BitConverter.ToInt32(body, offset); break;
                    default: offset += 4; break;
                }
            }
        }

        [Fact]
        public void EnsureSuccessThrowsForFailedResultInfo()
        {
            var info = new ResultInfo(StatusCode.Timeout, 42, "timeout");
            var exception = Assert.Throws<NativeCallException>(() => info.EnsureSuccess());
            Assert.Equal(42u, exception.Result.ErrorHandle);
        }

        [Fact]
        public void QueryHandlesReturnsSnapshotFailure()
        {
            var handle = new SessionSafeHandle(new System.IntPtr(1));
            var context = new SessionNativeContext(handle);
            var success = new ResultNative { Code = StatusCode.Success };
            var snapshotFailure = new ResultInfo(StatusCode.LogicError, 3, "snapshot failed");
            var result = context.QueryHandles(
                (System.IntPtr session, System.IntPtr[] handles, System.UIntPtr bufferSize, out System.UIntPtr written) =>
                {
                    handles[0] = new System.IntPtr(10);
                    handles[1] = new System.IntPtr(20);
                    written = (System.UIntPtr)2;
                    return success;
                },
                nativeHandle => nativeHandle.ToInt64() == 20 ? Result<int>.Failure(snapshotFailure) : Result<int>.Success(new ResultInfo(StatusCode.Success, 0, string.Empty), 10));
            handle.SetHandleAsInvalid();
            context.Dispose();
            Assert.False(result.IsSuccess);
            Assert.Equal(StatusCode.LogicError, result.Info.StatusCode);
            Assert.Equal(3u, result.Info.ErrorHandle);
        }

        [Fact]
        public void QueryHandlesRepeatsWithALargerBufferWhenTheBufferFills()
        {
            var handle = new SessionSafeHandle(new System.IntPtr(1));
            var context = new SessionNativeContext(handle);
            var success = new ResultNative { Code = StatusCode.Success };
            const int listed = 50;
            var calls = 0;
            var released = 0;
            var result = context.QueryHandles(
                (System.IntPtr session, System.IntPtr[] handles, System.UIntPtr bufferSize, out System.UIntPtr written) =>
                {
                    calls++;
                    var count = System.Math.Min(listed, (int)bufferSize);
                    for (var i = 0; i < count; i++)
                        handles[i] = new System.IntPtr(i + 1);
                    written = (System.UIntPtr)count;
                    return success;
                },
                nativeHandle =>
                {
                    released++;
                    return Result<long>.Success(new ResultInfo(StatusCode.Success, 0, string.Empty), nativeHandle.ToInt64());
                });
            handle.SetHandleAsInvalid();
            context.Dispose();
            Assert.True(result.IsSuccess);
            Assert.Equal(listed, result.Value.Length);
            Assert.Equal(2, calls);
            Assert.Equal(32 + listed, released);
        }
    }
}
