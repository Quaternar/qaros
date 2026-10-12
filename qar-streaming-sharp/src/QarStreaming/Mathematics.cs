using System;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros.Mathematics
{
    /// <summary>
    /// A 3D vector. It carries no space of its own; the member holding it names the space (see
    /// <see cref="Pose"/>).
    /// </summary>
    public readonly struct Vector3
    {
        public Vector3(float x, float y, float z)
        {
            X = x;
            Y = y;
            Z = z;
        }

        public float X { get; }
        public float Y { get; }
        public float Z { get; }

        internal Vector3Native ToNative()
        {
            return new Vector3Native { X = X, Y = Y, Z = Z };
        }

        internal static Vector3 FromNative(Vector3Native value)
        {
            return new Vector3(value.X, value.Y, value.Z);
        }
    }

    /// <summary>Double-precision 3D vector, used for Earth-fixed world coordinates in <see cref="GeoAnchorFrame"/>.</summary>
    public readonly struct Vector3d
    {
        public Vector3d(double x, double y, double z)
        {
            X = x;
            Y = y;
            Z = z;
        }

        public double X { get; }
        public double Y { get; }
        public double Z { get; }

        internal Vector3dNative ToNative()
        {
            return new Vector3dNative { X = X, Y = Y, Z = Z };
        }

        internal static Vector3d FromNative(Vector3dNative value)
        {
            return new Vector3d(value.X, value.Y, value.Z);
        }
    }

    /// <summary>
    /// Rotation quaternion, Hamilton convention, stored and constructed in x, y, z, w order (w
    /// last). <c>default(Quaternion)</c> is all zeros, not a rotation; use <see cref="Identity"/>.
    /// </summary>
    public readonly struct Quaternion
    {
        public Quaternion(float x, float y, float z, float w)
        {
            X = x;
            Y = y;
            Z = z;
            W = w;
        }

        public float X { get; }
        public float Y { get; }
        public float Z { get; }
        public float W { get; }

        public static Quaternion Identity { get { return new Quaternion(0.0f, 0.0f, 0.0f, 1.0f); } }

        internal QuaternionNative ToNative()
        {
            return new QuaternionNative { X = X, Y = Y, Z = Z, W = W };
        }

        internal static Quaternion FromNative(QuaternionNative value)
        {
            return new Quaternion(value.X, value.Y, value.Z, value.W);
        }
    }

    /// <summary>
    /// Position plus orientation (x, y, z, w). A pose carries no space of its
    /// own: every member that holds one names the space it is in. The bindings
    /// convert no coordinates; each value is exactly what the native library
    /// produced.
    /// </summary>
    /// <remarks>
    /// Every 3D space follows OpenXR: right-handed, +Y up, -Z forward, so a view
    /// looks along -Z. The spaces, as in the C header (types.h, "Coordinate
    /// systems") and https://docs.quaternar.com/docs/developer-guide/coordinate-systems/:
    /// <list type="bullet">
    /// <item><description><b>Room space</b>: the frame every peer in a session places things in, in metres. Whatever arrives through data streams and shared objects is in the publishing peer's room space: app volume poses, GUI panel poses, peer data.</description></item>
    /// <item><description><b>App volume space</b>: one app volume's local frame, origin at the cuboid centre, +X along width, +Y up along height, +Z along length, in metres. The app pose and the deltas of a mapped app volume gesture are in it.</description></item>
    /// <item><description><b>App content space</b>: the application's own world inside the volume, the space it renders in: app volume space taken into the app pose, positions divided by the app scale, so in app metres. Everything a render sender hands the source application is in the bound volume's app content space: view poses, the rendered pose of a view override, hand aim poses, joint poses, joint radii and joint velocities.</description></item>
    /// <item><description><b>View space</b>: one render view's camera frame, looking along -Z. <see cref="Fov"/> angles are in it.</description></item>
    /// </list>
    /// </remarks>
    public readonly struct Pose
    {
        public Pose(Quaternion orientation, Vector3 position)
        {
            Orientation = orientation;
            Position = position;
        }

        public Quaternion Orientation { get; }
        public Vector3 Position { get; }

        public static Pose Identity { get { return new Pose(Quaternion.Identity, new Vector3(0, 0, 0)); } }

        internal PoseNative ToNative()
        {
            return new PoseNative { Orientation = Orientation.ToNative(), Position = Position.ToNative() };
        }

        internal static Pose FromNative(PoseNative value)
        {
            return new Pose(Quaternion.FromNative(value.Orientation), Vector3.FromNative(value.Position));
        }
    }
}
