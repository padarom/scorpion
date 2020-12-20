# Build Deps
- `.\vcpkg.exe install realsense2:x64-windows`
- Set the env variable `VCPKGRS_DYNAMIC=1`
- After builds, copy the `realsense2.dll` to the target .exe directory

### Visual Studio 2019
In order to build `realsense-sys`, Visual Studio 2019 needs to be installed,
with the additional toolkits for MCF and ATL with Spectre mitigation.
