```c
point {
  // where contact points start when loaded in
  initial_position: { x: float, y: float },
  // whether this point contacts lines
  contact: bool,
  // friction in the usual sense
  contact_friction: float,
  // friction usually only applied to scarf
  air_friction: float,
}

bone {
  // points this bone connects (any)
  connected_points: (point_id, point_id),
  // weight (0 - 1) determining which point is pulled the most
  bias: float,
  // natural rest length of this bone = rest_length_factor * distance between initial points
  rest_length_factor: float,
  // Whether to only repel points (shoulder-foot behavior)
  repel_only: bool,
  // How much strain this bone can handle before it breaks (usually only on mount bones)
  endurance: float,
  // How much bone returns to its resting position when calculating adjustment
  adjustment_strength: float,
}

joint {
  // bones that cause a break if they cross
  // can either break a mount or break a skeleton (eg sled)
  bones_involved: (bone_id, bone_id),
}

skeleton {
  // contact and flutter points
  points: point[],
  // normal, repel, flutter (which are just normal) bones
  bones: bone[],
  // skeleton intact joints
  joints: joint[],
}

mount {
  // first skeleton involved (gets processed first by physics steps)
  skeleton1: {
    // the actual structure
    skeleton: skeleton,
    // mount bones (first point belongs to this skeleton and second point belongs to the other skeleton)
    mounts_bones: bone[],
    // how many frames until this skeleton is fully dismounted (from dismounting)
    frames_until_dismounted: int,
    // how many frames until this skeleton is actively remounting (from being dismounted)
    frames_until_remounting: int,
    // how many frames until this skeleton is fully mounted (from actively remounting)
    frames_until_mounted: int,
    // whether this skeleton can rejoin with another skeleton after dismount
    can_remount: bool,
  },
  // second skeleton involved
  skeleton2: {
    // the actual structure
    skeleton: skeleton,
    // mount bones
    mount_bones: bone[],
    // how many frames until this skeleton is fully dismounted (from dismounting)
    frames_until_dismounted: int,
    // how many frames until this skeleton is actively remounting (from being dismounted)
    frames_until_remounting: int,
    // how many frames until this skeleton is fully mounted (from actively remounting)
    frames_until_mounted: int,
    // whether this skeleton can rejoin with another skeleton after dismount
    can_remount: bool,
  },
  // How much to offset skeleton2 from skeleton1 initially (since both are normalized)
  skeleton2_initial_offset: { x: float, y: float },
  // mount intact joints
  joints: joint[],
}
```

physics process order by group (index is used for order within groups):
1) first skeleton points with `contact` set
2) second skeleton points with `contact` set
3) first skeleton points with `contact` not set
4) second skeleton points with `contact` not set
5) for 6 iterations (only applying to bones with both points having `contact` set)\
a) bones on first skeleton with `repel_only` not set\
b) mount bones associated with first skeleton
c) bones with `repel_only` set on first skeleton\
d) bones on second skeleton with `repel_only` not set\
e) mount bones associated with second skeleton
f) bones with `repel_only` set on second skeleton\
g) points with `contact` collide with lines
6) first skeleton bones with a single `contact` point
7) second skeleton bones with a single `contact` point
8) first skeleton bones with no `contact` points
9) second skeleton bones with no `contact` points
10) mount joints between skeletons
11) break joints on first skeleton
12) break joints on second skeleton
13) remount state between skeletons
