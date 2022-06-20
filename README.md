# nvidia-aftermath-rs
Rust bindings for obtaining GPU crashdumps with the Nvidia Aftermath SDK

This rust crate contains bindgen bindings for the Aftermath SDK downloadable from Nvidia's website. To use this crate, you'll simply create an Aftermath instance. This instance enables aftermath in the current program, and disables it when dropped.

```
// Calls GFSDK_Aftermath_EnableGpuCrashDumps
let aftermath = Aftermath::initialize();

...
if (error_from_vulkan_code == VK_ERROR_DEVICE_LOST) {
  // Calls GFSDK_Aftermath_GetCrashDumpStatus
  aftermath.wait_for_dump();
  // exit here, or handle the error...
}

// GFSDK_Aftermath_DisableGpuCrashDumps is called when aftermath is dropped
```

Please refer to the aftermath SDK's readme for more. **This crate does not provide the API initialization bits of enabling aftermath**. You need to use something like [VkDeviceDiagnosticsConfigFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html) to tell Vulkan to enable event interest.

### Installation
Add this crate to your `Cargo.toml`, and place the `libGFSDK_Aftermath_Lib.x64.so` from the aftermath download somewhere in your system path.
