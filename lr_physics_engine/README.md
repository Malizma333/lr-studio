## Physics Engine API

### Creating a New Engine
```rust
Engine::new(grid_version) -> Engine
```

### Timeline Viewing
```rust
engine.view_frame(frame) -> Vec<&Skeleton>
// Similar to LRA (iteration, subiteration) captured in a "Moment"
engine.view_moment(frame, moment) -> Vec<&Skeleton>
```

### Line Grid Modifications
```rust
engine.create_line(line: Line) -> LineId
engine.update_line(line_id: Id, new_line: Line) -> ()
engine.delete_line(line_id: Id) -> ()
```

### Entity Registry
```rust
let registry = engine.registry();

registry.register_skeleton_type(skeleton_template) -> SkeletonTemplateId
registry.create_skeleton(skeleton_template_id) -> SkeletonId
registry.delete_skeleton(skeleton_id) -> ()
```

### Skeleton Template Builder
```rust
let mut my_skeleton = registry.skeleton_template_builder();

let (my_skeleton, p1_id) = my_skeleton.point(Vec2df::new(0.0, 0.0)).contact().build();
let (my_skeleton, p2_id) = my_skeleton.point(Vec2df::new(1.0, 0.0)).friction(0.5).build();
let (my_skeleton, p3_id) = my_skeleton.point(Vec2df::new(1.0, 1.0)).build();
let (my_skeleton, b1_id) = my_skeleton.bone(p1_id, p2_id).repel().bias(1).build();
let (my_skeleton, b2_id) = my_skeleton.bone(p2_id, p3_id).adjustment_strength(1).build();
let (my_skeleton, j1_id) = my_skeleton.joint(b1_id, b2_id).build();

let skeleton_template_id = my_skeleton.build();
```

## Architecture

Four entity class types:
- Point
- Bone (Point-Point connections)
- Joint (Bone-Bone connections)
- Skeleton (contains Points, Bones, Joints)

Each has four sub-types:
- Builder (interface for building Templates)
- Template (reference for how to construct Entity)
- Entity (contains props populated by Template as well as helper functions operating on State)
- State (contains everything that needs to be copied across frames, plugged into entities for calculations)
