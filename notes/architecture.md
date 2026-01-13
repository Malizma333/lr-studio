# Architecture Notes

## Resources
Some notes on [Designing a Good API](https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/32713.pdf)
Helpful video about [Plugin Architecture](https://www.youtube.com/watch?v=sSpULGNHyoI)

## Questions/Answers
Should one instance of PhysicsEngine be created during the entire program?
- No, a new engine will be created when loading a track or creating a new track
- Otherwise the engine will be modified

Who should be responsible for storing mutable line data and passing it around when it updates?
- physics lines should be owned by and accessed through the engine, with exposed update methods
- each module owns the information that it needs
- modules communicate necessary data between each other

Where should physics types belong for reusability in other crates?
- respective physics crates, with duplicate types in format for serialization only

When should builders versus getters/setters be used?
- Prefer builders for immutable or complex objects
- For mutable complex objects, builders can still be used with a from cast
- Setters/getters can be used on simple mutable structs

When should there be defaults when deserializing tracks?
- Optional field defaults should be provided by the type of track being deserialized, because they need that context
- Required field defaults should default at the format struct

Should we use external packages?
- Primarily for development (testing, benchmarking, etc)
- Relying on production packages increases binary size
  - Also may break in future if platforms update or library can't be downloaded anymore

Should we use macros?
- Primarily derive macros
- Avoid at all costs macro by example
  - Usually indicates a refactor is needed anyway
- Don't define our own macros

How should line triggers interact with frame triggers?
- TODO

How should version specific features (eg LRA Remount/bugfixes or .com Scarf) be plugins?
  - Enabling feature flags when building engine (eg `.legacy_remount()`)

Should custom scripts be allowed?
  - Would say no, harder to support across different languages
  - easier to support serializable, structured data format case-by-case

How to make sprites rendering over skeletons a plugin?
  - just use skeleton views (allow for rich skeleton views)