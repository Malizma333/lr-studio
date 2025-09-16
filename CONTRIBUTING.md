TODO - look into how to write one of these things

Code Style Guidelines
- No generics
- No external dependencies (but external dev dependencies allowed)
- Avoid macros

Goals
- Disable saving tracks with legacy features (eg 6.0/6.1 Grid, Beta Line Mods, etc)
  - Ensures that we don't need to document improper legacy implementations within the file
- Customizable physics interface for custom line types, custom entity types (basic rider/sled ?)
