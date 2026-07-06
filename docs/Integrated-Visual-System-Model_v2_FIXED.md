# Integrated Visual System Model v2 FIXED

## Diagnosis

The previous integration could fail because HTML previews often break when image paths depend on external folder resolution, or when Mermaid strings are injected into JavaScript unsafely.

## Fix strategy

1. Keep PNG/JPG files in `assets/charts/`.
2. Embed PNG charts directly in the HTML as base64 for reliable preview.
3. Store Mermaid files in `assets/diagrams/`.
4. Inject Mermaid content through JSON-safe data.
5. Show Mermaid source as fallback when rendering is unavailable.
6. Keep active execution stepper in the same page.

## Low-level core

```text
jal label
→ $ra ← PC + 4
→ PC ← address(label)
→ Trace event
```

## Business layer

The system can be positioned as:

- Computer Architecture Lab Tool
- Interactive low-level debugging visualizer
- Technical portfolio showcase
- Future Rust CLI + Web dashboard product
