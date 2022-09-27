https://github.com/googlefonts/picosvg takes in a raw svg and converts it to a "pico" svg.
A picosvg guarrantees:

* Exactly 1 `<defs>` element, first child of root
* Only gradients defined under `<defs>`
* After the initial `<defs>`, only `<g>` and `<path>`
   * `<g>` is eliminated when possible, but may be retained for opacity
* Only absolute coordinates
* Only commands that specify full coordinates, no shorthand (H, S, etc)
* Clip paths and strokes are rendered into equivalent paths
* `<use>` references are materialized

In Python we use the same types for raw and pico svg. In Rust they should be different types.
Parse an incoming svg file into a "raw" svg, call a function to get a "pico" svg of it. Something akin to:

```rust
let svg:Svg = Svg::parse(raw_xml);
let pico:Picosvg = Picosvg::from(svg);
```

We want fast parsing, so we will rely on https://docs.rs/quick-xml/latest/quick_xml/.

We want typed fields, therefore we will not use https://crates.io/crates/svg, instead choosing to
declare our own structs. We regretted not going further with types in the Python version, lets not
make that mistake again.
