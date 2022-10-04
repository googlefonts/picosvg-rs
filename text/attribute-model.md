## Model for SVG Attributes

### Requirements

SVG elements may contain a large number of attributes, some required, but most
optional. We have a few requirements for representing attributes:

- Sparse storage. A fixed set of fields for all possible attributes would waste
memory and make element construction annoying.

- Internally consistent. We don't want to store unrecognized attributes or
attributes with associated values of the wrong type. This means two things:

    1. Statically verified when possible. If we're inserting or accessing a known
attribute, we want the compiler to ensure that we're using values of the 
correct type.

    2. Dynamically verified otherwise. We may need to dynamically insert
attributes during parsing. These insertions should also maintain consistency, 
so dynamic validation is necessary.

- Enumerable. We want to be able to iterate over the attributes of an element
in a uniform manner.

### Design

This leads us to a `HashMap` for attribute storage. To ensure the expected invariants,
the hash map is hidden behind an interface that provides both strongly typed and 
dynamically validated mutators and accessors. This type is called `AttributeMap`.

For type saftey, there are two traits that work together to describe values and attributes:

- `ValueDescriptor`: Implemented for all potential attribute value types such
as `Length` or `Color`. It provides methods to convert between the `Value` enum and the
concrete value type.

- `AttributeDescriptor`: Associates an attribute id (the SVG spelling) with the concrete
value type for that attribute. The value type must have a `ValueDescriptor` implementation.
This is implemented for a set of empty structs in the `attribute::id` module that are used
as keys for insertion and retrieval.

The `AttributeMap` type provides two sets of methods:

- `insert`/`get`: These are checked at compile time and work with the empty structs from
the `attribute::id` module as keys concrete value types as values.

- `insert_by_id`/`get_by_id`: These are checked at runtime and work with raw string attribute
ids as keys and the `Value` enum as values.

### DRY

We use a `build.rs` script to avoid repetitive code such as implementing `ValueDescriptor`
and `AttributeDescriptor` and also for generating a table that maps attribute ids to validator
functions that ensure a `Value` instance is a variant of the appropriate type. This table 
is sorted by id, and a binary search is used during dynamic attribute insertion to both verify
that it is a known attribute id and to retrieve the respective validator function.

The current input in the build script is an array of tuples of the form `(&str, &str, &str)`. 
The values represent the attribute name, SVG spelling of the attribute name, and attribute
value type, respectively.

As an example, gradient stop colors are represented as such:
`("StopColor", "stop-color", "Color")`. This will be used to generate an emtpy
`attribute::id::StopColor` struct with an `AttributeDescriptor` implementation that
associates the "stop-color" attribute id to the concrete `Color` value type.

All attribute value types are also collected, deduplicated and used to generate `ValueDescriptor` 
implementations.

The collection of tuples as a whole is used to generate the sorted table.
