# Quaternar.Qaros

The C# binding of the QAROS streaming C API, in the `Quaternar.Qaros` namespace: `SafeHandle`s,
`IDisposable` and a `Result<T>` return type, so no manual pointers and no exceptions for expected
native failures.

- Targets `netstandard2.0`, `netstandard2.1` and `net8.0`, so it works in Unity and modern .NET alike.
- The package is the binding only. `Library.Init` loads `qar-streaming-c` from the QAROS installed
  on the PC, release or demo, and returns a "QAROS is not installed" result when there is none.
  Ship no copy of the runtime with your app.

```xml
<ItemGroup>
  <PackageReference Include="Quaternar.Qaros" Version="0.*" />
</ItemGroup>
```

Documentation: https://docs.quaternar.com (Developer Guide, Getting Started).
Source: https://github.com/Quaternar/qaros (`qar-streaming-sharp/`).
