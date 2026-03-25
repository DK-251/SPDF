# Check Results

## Run Info
- **Version:** 0.1.0-snapshot.3
- **Commit:** cddcfe7
- **Branch:** main
- **Date:** 2026-03-25T15:53:05Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAIL

## Steps
- [ ] cargo fmt --check: FAIL
- [x] cargo clippy: PASS
- [x] cargo test: PASS

### cargo fmt --check (last 80 lines)
````
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\src\container.rs:155:
         "layers/semantic.json",
         &mut total_decompressed,
     )?;
[31m-    let layout = read_entry(
[0m[31m-        &mut archive,
[0m[31m-        "layers/layout.json",
[0m[31m-        &mut total_decompressed,
[0m[31m-    )?;
[0m[31m-    let styles = read_entry(
[0m[31m-        &mut archive,
[0m[31m-        "layers/styles.json",
[0m[31m-        &mut total_decompressed,
[0m[31m-    )?;
[0m[32m+    let layout = read_entry(&mut archive, "layers/layout.json", &mut total_decompressed)?;
[0m[32m+    let styles = read_entry(&mut archive, "layers/styles.json", &mut total_decompressed)?;
[0m     let render = read_entry(&mut archive, "layers/render.pdf", &mut total_decompressed)?;
     let metadata = read_entry(
         &mut archive,
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\src\container.rs:171:
         "layers/metadata.json",
         &mut total_decompressed,
     )?;
[31m-    let audit = read_entry(
[0m[31m-        &mut archive,
[0m[31m-        "layers/audit.json",
[0m[31m-        &mut total_decompressed,
[0m[31m-    )?;
[0m[32m+    let audit = read_entry(&mut archive, "layers/audit.json", &mut total_decompressed)?;
[0m 
     // Verify layer checksums
     verify_checksum("semantic", &manifest.layers.semantic, &semantic)?;
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\dom_tests.rs:590:
                 }),
                 Element::LineItemTable(LineItemTableElement {
                     eid: eid(),
[31m-                    headers: vec![
[0m[31m-                        "Item".into(),
[0m[31m-                        "Qty".into(),
[0m[31m-                        "Rate".into(),
[0m[31m-                        "Amount".into(),
[0m[31m-                    ],
[0m[32m+                    headers: vec!["Item".into(), "Qty".into(), "Rate".into(), "Amount".into()],
[0m                     rows: vec![vec![
                         TableCell {
                             value: "API Integration".into(),
`````n
