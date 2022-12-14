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
* `transform` attributes are applied to the path.d coordinates
* nested svg are inlined
* paths with fill-rule='evenodd' are converted to 'nonzero'
* floats are rounded to 3 digits by default
* inlined CSS `style` attributes are converted to equivalent svg attributes
* empty or unpainted paths are removed
* optionally clip svg to its viewbox
* discard non-svg or useless content, e.g. comments, etc

In Python we use the same types for raw and pico svg. In Rust they should be different types.
Parse an incoming svg file into a "raw" svg, call a function to get a "pico" svg of it. Something akin to:

```rust
let svg: Svg = Svg::parse(raw_xml);
let pico: Picosvg = Picosvg::from(svg);
```

We want fast parsing, so we will rely on https://docs.rs/quick-xml/latest/quick_xml/.

We want typed fields, therefore we will not use https://crates.io/crates/svg, instead choosing to
declare our own structs. We regretted not going further with types in the Python version, lets not
make that mistake again.

We want to avoid duplication with `kurbo`, which already has many of the core types, and we need the ability 
to write, so we will not use https://docs.rs/svgtypes/latest/svgtypes/.

In Python the picosvg svg types are useful for general svg manipulation. That seems likely to recur.

Unlike Python we want to parse svg => typed objects and not mix and match xml manipulation with
object manipulation.

Consider structuring things so you can use our svg types w/o picosvg. Per discussion with @dfrg perhaps:

1. `kurbo` crate provides our basic graphics contructs including svg path parsing
1. `vg-types` (new) provides core vector graphic constructs
   * depends on `kurbo`
1. `simple-svg` (new) provides basic svg parsing, roughly the equivalent of [svg_types.py](https://github.com/googlefonts/picosvg/blob/main/src/picosvg/svg_types.py) in picosvg
   * depnds on `kurbo`, `vg-types`
1. `picosvg-rs` provides picosvg conversion, from simple svg to pico svg
   * depnds on `kurbo`, `vg-types`, `simple-svg`
