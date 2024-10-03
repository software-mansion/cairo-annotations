# Cairo Annotations

The `cairo-annotations` crate provides tools tailored for working with annotations in the Cairo language. These
annotations are part of the Sierra Debug Information format and serve to enrich the code by detailing information.

> 📝 **Note**  
> Although this crate is primarily used by projects like `cairo-coverage`, `cairo-profiler`, and `starknet-foundry`,
> it is also fully capable of functioning as a standalone library to work with Sierra Debug Information annotations.

## Features of Cairo Annotations

### Structured Annotations

`cairo-annotations` offers a structured representation, allowing for more ergonomic manipulation of annotations. Some
key features include:

- **Coverage Annotations**: Track locations in the Cairo code that correspond to specific Sierra statements.
- **Profiler Annotations**: Provide mappings from Sierra statements to fully qualified Cairo paths, detailing which
  functions in the Cairo code triggered them.

All annotations implement the `TryFromDebugInfo` trait, enabling their extraction from Sierra debug information. Here's
a simple example:

```rust
let annotations = VersionedCoverageAnnotations::try_from_debug_info(sierra_debug_info).unwrap();
```


### Coverage Annotations

Coverage annotations provide a mapping from Sierra statement indices to sources in the Cairo code that resulted in their
creation. For extensive documentation,
see [CoverageAnnotationsV1](./crates/cairo-annotations/src/annotations/coverage.rs).

Example to retrieve code location information:

```rust
use cairo_annotations::annotations::coverage::{
    CodeLocation, ColumnNumber, CoverageAnnotationsV1, LineNumber, SourceCodeLocation,
    SourceCodeSpan, SourceFileFullPath, VersionedCoverageAnnotations,
};
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_lang_sierra::program::StatementIdx;

let VersionedCoverageAnnotations::V1(annotations) =
    VersionedCoverageAnnotations::try_from_debug_info(sierra_debug_info).unwrap();

let code_locations = annotations
    .statements_code_locations
    .get(&StatementIdx(331))
    .unwrap();

assert_eq!(
    code_locations,
    &[CodeLocation(
        SourceFileFullPath("/path/to/my/file.cairo".into()),
        SourceCodeSpan {
            start: SourceCodeLocation { line: LineNumber(7), col: ColumnNumber(4) },
            end: SourceCodeLocation { line: LineNumber(7), col: ColumnNumber(4) },
        }
    )]
);
```

### Profiler Annotations

Profiler annotations map Sierra statement indices to Cairo function paths, showing which functions led to their
generation. Detailed information is available
in [ProfilerAnnotationsV1](./crates/cairo-annotations/src/annotations/profiler.rs).

Example to get the Cairo path:

```rust
use cairo_annotations::annotations::profiler::{
    FunctionName, ProfilerAnnotationsV1, VersionedProfilerAnnotations,
};
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_lang_sierra::program::StatementIdx;

let VersionedProfilerAnnotations::V1(annotations) =
    VersionedProfilerAnnotations::try_from_debug_info(sierra_debug_info).unwrap();

let functions_names = annotations
    .statements_functions
    .get(&StatementIdx(331))
    .unwrap();

assert_eq!(
    functions_names,
    &[FunctionName("scarb_template::fib".into())]
);
```

## Integration with snforge

Annotations are particularly useful for getting information about executed code. If you are using `snforge`, you can
leverage the `--save-trace-data` flag to generate trace data.

Deserialize this data using `VersionedCallTrace` from the `cairo-annotations` crate, and subsequently
use `map_pcs_to_sierra_statement_ids` to map the trace to Sierra statement IDs.
