// Nvidia Aftermath GPU crash dump utility
//
// These are set of bindings for the Nvidia Aftermath SDK, which
// allows for GPU minidump generation.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod aftermath_sys;
use aftermath_sys::*;

use std::io::Write;

/// An instance of an Aftermath minidump generator.
///
/// This is equivalent to the GpuCrashTracker found in the github
/// examples at:
/// https://github.com/NVIDIA/nsight-aftermath-samples/blob/master/VkHelloNsightAftermath/NsightAftermathGpuCrashTracker.cpp
pub struct Aftermath {
    a_initialized: bool,
}

unsafe extern "C" fn GpuCrashDumpCallback(
    pGpuCrashDump: *const ::std::os::raw::c_void,
    gpuCrashDumpSize: u32,
    pUserData: *mut ::std::os::raw::c_void,
) {
    let crash_dump_bytes: &[u8] =
        std::slice::from_raw_parts(pGpuCrashDump as *const u8, gpuCrashDumpSize as usize);

    let mut dump_file = std::fs::File::create(format!(
        "Nvidia_Aftermath_GPU_CrashDump_{}.{}",
        std::process::id(),
        "nv-gpudmp"
    ))
    .unwrap();
    dump_file
        .write_all(crash_dump_bytes)
        .expect("Could not write GPU crashdump file");
}

unsafe extern "C" fn ShaderDebugInfoCallback(
    pShaderDebugInfo: *const ::std::os::raw::c_void,
    shaderDebugInfoSize: u32,
    pUserData: *mut ::std::os::raw::c_void,
) {
    let crash_dump_bytes: &[u8] =
        std::slice::from_raw_parts(pShaderDebugInfo as *const u8, shaderDebugInfoSize as usize);

    let mut dump_file = std::fs::File::create(format!(
        "Nvidia_Aftermath_GPU_Shader_DebugInfo_{}.{}",
        std::process::id(),
        "nvdbg"
    ))
    .unwrap();
    dump_file
        .write_all(crash_dump_bytes)
        .expect("Could not write shader debuginfo file");
}

unsafe extern "C" fn CrashDumpDescriptionCallback(
    addValue: PFN_GFSDK_Aftermath_AddGpuCrashDumpDescription,
    pUserData: *mut ::std::os::raw::c_void,
) {
}

static data: u32 = 0;

impl Aftermath {
    pub fn initialize() -> Result<Self, GFSDK_Aftermath_Result> {
        unsafe {
            let res = GFSDK_Aftermath_EnableGpuCrashDumps(
                GFSDK_Aftermath_Version_API,
                GFSDK_Aftermath_GpuCrashDumpWatchedApiFlags_Vulkan,
                GFSDK_Aftermath_GpuCrashDumpFeatureFlags_DeferDebugInfoCallbacks, // Let the Nsight Aftermath library cache shader debug information.
                Some(GpuCrashDumpCallback), // Register callback for GPU crash dumps.
                Some(ShaderDebugInfoCallback), // Register callback for shader debug information.
                Some(CrashDumpDescriptionCallback), // Register callback for GPU crash dump description.
                None, // Register callback for resolving application-managed markers.
                &data as *const _ as *mut std::os::raw::c_void, // Set the GpuCrashTracker object as user data for the above callbacks.
            );

            match res {
                GFSDK_Aftermath_Result_Success => Ok(Self {
                    a_initialized: true,
                }),
                _ => Err(res),
            }
        }
    }

    /// This is a handler for when a Device lost error is found. You call this and
    /// this crate takes care of waiting for the dump to finish
    pub fn wait_for_dump(&self) {
        println!("------ NVIDIA Aftermath: Waiting for GPU Crashdump ------");
        println!("...");

        loop {
            // Add a sleep so we give it a chance to work
            std::thread::sleep(std::time::Duration::from_millis(100));

            let mut status = GFSDK_Aftermath_CrashDump_Status_Unknown;

            let res = unsafe { GFSDK_Aftermath_GetCrashDumpStatus(&mut status) };

            println!("Crashdump result is {:?}", res);
            println!("Crashdump status is {:?}", status);

            if status == GFSDK_Aftermath_CrashDump_Status_Finished
                || status == GFSDK_Aftermath_CrashDump_Status_CollectingDataFailed
            {
                break;
            }
        }

        println!("------ NVIDIA Aftermath: Crashdump Complete ------");
    }
}

impl Drop for Aftermath {
    fn drop(&mut self) {
        unsafe {
            GFSDK_Aftermath_DisableGpuCrashDumps();
        }
    }
}
