# Build Status

## Last Run
- **Version:** 0.1.0-snapshot.1
- **Date:** 2026-03-25T13:17:40Z
- **Commit:** ecd1e90
- **Branch:** main
- **Machine:** TUF_WARRIOR_DK

## Rust Core (crates/)
- [x] cargo build: PASS
- [x] cargo test: PASS
- [x] cargo clippy: PASS
- [x] cargo fmt --check: PASS

## Python API (api/)
- [-] pip install: SKIP
- [-] pytest: SKIP

## Studio Frontend (studio/)
- [-] npm install: SKIP
- [-] npm build: SKIP

## Error Logs
### rust-build.log
```n
error: could not compile `proc-macro2` (build script) due to 1 previous error
error: could not compile `getrandom` (build script) due to 1 previous error
error: could not compile `crc32fast` (build script) due to 1 previous error
error: could not compile `typenum` (build script) due to 1 previous error
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.50.35717\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\serde-5c8f9baa5cb62b21\\rustcLOjUNI\\symbols.o" "<3 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-w
indows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_a
lloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" 
"ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\serde-5c8f9baa5cb62b21\\build_script_build-5c8f9baa5cb62b21.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
          

error: could not compile `serde` (build script) due to 1 previous error
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.50.35717\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\libc-560f76a09dc7ff36\\rustcLEXdDV\\symbols.o" "<5 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-wi
ndows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_al
loc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" 
"ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\libc-560f76a09dc7ff36\\build_script_build-560f76a09dc7ff36.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
          

error: could not compile `libc` (build script) due to 1 previous error
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.50.35717\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\target-lexicon-fed51ee4df92a2d2\\rustcHd7EDF\\symbols.o" "<3 object files omitted>" "<sysroot>\\lib\\rustlib\\x8
6_64-pc-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_wo
rkspace_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" 
"kernel32.lib" "ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\target-lexicon-fed51ee4df92a2d2\\build_script_build-fed51ee4df92a2d2.exe" "/OPT:REF,NOICF" "/DEBUG" 
"/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
          

error: could not compile `target-lexicon` (build script) due to 1 previous error
```

### rust-clippy.log
```n          

error: could not compile `serde_core` (build script) due to 1 previous error
error: could not compile `wasm-bindgen-shared` (build script) due to 1 previous error
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.50.35717\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\serde-5c8f9baa5cb62b21\\rustcdEtOIK\\symbols.o" "<3 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-w
indows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_a
lloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" 
"ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\serde-5c8f9baa5cb62b21\\build_script_build-5c8f9baa5cb62b21.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
          

error: could not compile `getrandom` (build script) due to 1 previous error
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.50.35717\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\libc-560f76a09dc7ff36\\rustcbMdyoS\\symbols.o" "<5 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-wi
ndows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_al
loc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" 
"ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\libc-560f76a09dc7ff36\\build_script_build-560f76a09dc7ff36.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
          

error: could not compile `serde` (build script) due to 1 previous error
error: could not compile `libc` (build script) due to 1 previous error
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.50.35717\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\target-lexicon-fed51ee4df92a2d2\\rustcRwmR5b\\symbols.o" "<3 object files omitted>" "<sysroot>\\lib\\rustlib\\x8
6_64-pc-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_wo
rkspace_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" 
"kernel32.lib" "ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\target-lexicon-fed51ee4df92a2d2\\build_script_build-fed51ee4df92a2d2.exe" "/OPT:REF,NOICF" "/DEBUG" 
"/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
          

error: could not compile `target-lexicon` (build script) due to 1 previous error
```

### rust-test.log
```n  = note: LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
          

error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.50.35717\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\typenum-5afa3ed4e2d05ff9\\rustcVCMqxs\\symbols.o" "<3 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc
-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace
_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" 
"ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\typenum-5afa3ed4e2d05ff9\\build_script_build-5afa3ed4e2d05ff9.exe" "/OPT:REF,NOICF" "/DEBUG" 
"/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
          

error: could not compile `proc-macro2` (build script) due to 1 previous error
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.50.35717\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\libc-560f76a09dc7ff36\\rustc7z3hSc\\symbols.o" "<5 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-wi
ndows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_al
loc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" 
"ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\libc-560f76a09dc7ff36\\build_script_build-560f76a09dc7ff36.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
          

error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.50.35717\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\serde_core-a174d9b7f022b44e\\rustcQYb21p\\symbols.o" "<3 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64
-pc-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_worksp
ace_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" 
"ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\SPDF 
DEVELOPMENT\\SPDF\\target\\debug\\build\\serde_core-a174d9b7f022b44e\\build_script_build-a174d9b7f022b44e.exe" "/OPT:REF,NOICF" "/DEBUG" 
"/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" 
"/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
          

error: could not compile `typenum` (build script) due to 1 previous error
error: could not compile `serde` (build script) due to 1 previous error
error: could not compile `libc` (build script) due to 1 previous error
error: could not compile `serde_core` (build script) due to 1 previous error
```

