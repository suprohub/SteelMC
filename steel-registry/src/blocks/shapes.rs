/// Axis-Aligned Bounding Box used for block collision and outline shapes.
///
/// Coordinates are in block-local space (0.0 to 1.0 for a standard block).
/// Values can extend beyond 0.0-1.0 for blocks like fences (collision height 1.5).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    /// Creates a new AABB from min and max coordinates.
    ///
    /// # Panics
    ///
    /// Panics if `debug_assertions` are enabled and `min` is not less than or
    /// equal to `max` componentwise.
    #[must_use]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        debug_assert!(
            min.x <= max.x && min.y <= max.y && min.z <= max.z,
            "`min` must be less than or equal to `max` componentwise (min = {min}, max = {max})"
        );

        Self { min, max }
    }

    // TODO: remove when the assertion in `new` can be done in a `const` context.
    #[doc(hidden)]
    pub const fn new_unchecked(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// A full block (0,0,0) to (1,1,1).
    pub const FULL_BLOCK: AABB = AABB::new_unchecked(Vec3::ZERO, Vec3::ONE);

    /// An empty shape (no collision).
    #[doc(alias = "ZERO")]
    pub const EMPTY: AABB = AABB::new_unchecked(Vec3::ZERO, Vec3::ZERO);

    /// Returns true if this AABB has no volume.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y || self.min.z >= self.max.z
    }

    /// Returns the width (X dimension) of this AABB.
    #[must_use]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Returns the height (Y dimension) of this AABB.
    #[must_use]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Returns the depth (Z dimension) of this AABB.
    #[must_use]
    pub fn depth(&self) -> f32 {
        self.max.z - self.min.z
    }

    /// Returns the average dimension size of this AABB.
    ///
    /// Vanilla equivalent: `AABB.getSize()`.
    #[must_use]
    pub fn get_size(&self) -> f32 {
        (self.width() + self.height() + self.depth()) / 3.0
    }

    /// Returns a new AABB deflated (shrunk inward) by the given amount on all sides.
    ///
    /// This is used for collision detection to avoid floating-point edge cases.
    /// Matches vanilla `AABB.deflate()`.
    #[must_use]
    pub fn deflate(&self, amount: f32) -> Self {
        Self {
            min: self.min + amount,
            max: self.max - amount,
        }
    }

    /// Returns a new AABB inflated (expanded outward) by the given amount on all sides.
    ///
    /// Matches vanilla `AABB.inflate()`.
    #[must_use]
    pub fn inflate(&self, amount: f32) -> Self {
        Self {
            min: self.min - amount,
            max: self.max - amount,
        }
    }

    /// Returns a new AABB moved by the given delta.
    ///
    /// Matches vanilla `AABB.move()`.
    #[must_use]
    pub fn translate(&self, d: Vec3) -> Self {
        Self {
            min: self.min + d,
            max: self.max + d,
        }
    }

    /// Returns a new AABB positioned at the given block coordinates.
    ///
    /// Converts a block-local AABB (0-1 space) to world coordinates.
    #[must_use]
    pub fn at_block(&self, pos: IVec3) -> Self {
        let pos = pos.as_vec3();
        Self {
            min: self.min + pos,
            max: self.max + pos,
        }
    }

    /// Checks if this AABB intersects with another AABB.
    ///
    /// Returns true if the two AABBs overlap in all three dimensions.
    /// Matches vanilla `AABB.intersects()`.
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        self.max.x > other.min.x
            && self.min.x < other.max.x
            && self.max.y > other.min.y
            && self.min.y < other.max.y
            && self.max.z > other.min.z
            && self.min.z < other.max.z
    }

    /// Checks if this AABB contains the given point.
    #[must_use]
    pub fn contains(&self, x: f32, y: f32, z: f32) -> bool {
        x >= self.min.x
            && x <= self.max.x
            && y >= self.min.y
            && y <= self.max.y
            && z >= self.min.z
            && z <= self.max.z
    }
}

/// Double-precision Axis-Aligned Bounding Box used for entity collision.
///
/// Coordinates are in world space. Used for player and entity bounding boxes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABBd {
    pub min: DVec3,
    pub max: DVec3,
}

impl AABBd {
    /// Creates a new double-precision AABB from min and max coordinates.
    #[must_use]
    pub const fn new(min: DVec3, max: DVec3) -> Self {
        Self { min, max }
    }

    /// Creates an entity bounding box centered at the given position.
    ///
    /// The box extends `half_width` in X and Z directions,
    /// and from `y` to `y + height` in the Y direction.
    #[must_use]
    pub fn entity_box(pos: DVec3, half_width: f64, height: f64) -> Self {
        Self {
            min: pos + DVec3::new(-half_width, 0.0, -half_width),
            max: pos + DVec3::new(half_width, height, half_width),
        }
    }

    /// Returns a new AABB deflated (shrunk inward) by the given amount on all sides.
    ///
    /// This is used for collision detection to avoid floating-point edge cases.
    /// Matches vanilla's collision epsilon of 1.0E-5.
    #[must_use]
    pub fn deflate(&self, amount: f64) -> Self {
        Self {
            min: self.min + amount,
            max: self.max - amount,
        }
    }

    /// Returns a new AABB inflated (expanded outward) by the given amount on all sides.
    #[must_use]
    pub fn inflate(&self, amount: f64) -> Self {
        Self {
            min: self.min - amount,
            max: self.max + amount,
        }
    }

    /// Returns a new AABB inflated by different amounts on each axis.
    #[must_use]
    pub fn inflate_xyz(&self, amount: DVec3) -> Self {
        Self {
            min: self.min - amount,
            max: self.max + amount,
        }
    }

    /// Checks if this AABB intersects with another AABB.
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        self.max.x > other.min.x
            && self.min.x < other.max.x
            && self.max.y > other.min.y
            && self.min.y < other.max.y
            && self.max.z > other.min.z
            && self.min.z < other.max.z
    }

    /// Checks if this AABB intersects with a single-precision block AABB.
    #[must_use]
    pub fn intersects_block_aabb(&self, other: &AABB) -> bool {
        self.max.x > f64::from(other.min.x)
            && self.min.x < f64::from(other.max.x)
            && self.max.y > f64::from(other.min.y)
            && self.min.y < f64::from(other.max.y)
            && self.max.z > f64::from(other.min.z)
            && self.min.z < f64::from(other.max.z)
    }
}

// Iterator over block positions intersected by an AABB (f32).
pub struct AABBBlockIter {
    current: IVec3,
    min: IVec3,
    max: IVec3,
}

impl AABBBlockIter {
    fn new(aabb: &AABB) -> Self {
        let min = aabb.min.as_ivec3();
        Self {
            current: min,
            min,
            max: aabb.max.as_ivec3(),
        }
    }
}

impl Iterator for AABBBlockIter {
    type Item = BlockPos;

    fn next(&mut self) -> Option<Self::Item> {
        // If the range is empty (any min > max), yield nothing.
        if self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z {
            return None;
        }

        // If we've already passed the last z coordinate, iteration is done.
        if self.current.z > self.max.z {
            return None;
        }

        let pos = BlockPos(self.current);

        // Advance to the next position (x fastest, then y, then z).
        if self.current.x < self.max.x {
            self.current.x += 1;
        } else {
            self.current.x = self.min.x;
            if self.current.y < self.max.y {
                self.current.y += 1;
            } else {
                self.current.y = self.min.y;
                self.current.z += 1;
            }
        }

        Some(pos)
    }
}

impl AABB {
    /// Returns an iterator over all integer block positions that this AABB overlaps.
    ///
    /// The iteration order matches nested loops over x, y, z with the fastest varying x.
    /// The conversion from f32 to i32 uses `as i32` (truncation toward zero) to match
    /// the original `check_inside_blocks` implementation.
    pub fn blocks(&self) -> AABBBlockIter {
        AABBBlockIter::new(self)
    }
}

/// Iterator over block positions intersected by an AABBd (f64).
pub struct AABBdBlockIter {
    current: IVec3,
    min: IVec3,
    max: IVec3,
}

impl AABBdBlockIter {
    fn new(aabb: &AABBd) -> Self {
        let min = aabb.min.as_ivec3();
        Self {
            current: min,
            min,
            max: aabb.max.as_ivec3(),
        }
    }
}

impl Iterator for AABBdBlockIter {
    type Item = BlockPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z {
            return None;
        }
        if self.current.z > self.max.z {
            return None;
        }

        let pos = BlockPos(self.current);

        if self.current.x < self.max.x {
            self.current.x += 1;
        } else {
            self.current.x = self.min.x;
            if self.current.y < self.max.y {
                self.current.y += 1;
            } else {
                self.current.y = self.min.y;
                self.current.z += 1;
            }
        }

        Some(pos)
    }
}

impl AABBd {
    /// Returns an iterator over all integer block positions that this double-precision AABB overlaps.
    ///
    /// The conversion from f64 to i32 uses `as i32` (truncation toward zero) to match
    /// the original `check_inside_blocks` implementation.
    pub fn blocks(&self) -> AABBdBlockIter {
        AABBdBlockIter::new(self)
    }
}

/// A VoxelShape is a collection of AABBs that define the shape of a block.
///
/// For simple blocks, this is typically a single AABB (full block or empty).
/// For complex blocks like stairs or fences, this is multiple AABBs combined.
pub type VoxelShape = &'static [AABB];

/// An ID referencing a registered VoxelShape in the ShapeRegistry.
///
/// Use this to refer to shapes in a compact way. The actual shape data
/// can be retrieved from the ShapeRegistry using this ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShapeId(pub u16);

impl ShapeId {
    /// The empty shape (no AABBs).
    pub const EMPTY: ShapeId = ShapeId(0);

    /// A full block shape.
    pub const FULL_BLOCK: ShapeId = ShapeId(1);
}

/// Registry for VoxelShapes.
///
/// Shapes are registered once and referenced by ShapeId. This allows
/// deduplication of shapes and compact storage of shape references.
///
/// Vanilla shapes are registered at startup. Plugins can register
/// additional shapes for custom blocks.
pub struct ShapeRegistry {
    shapes: Vec<&'static [AABB]>,
    allows_registering: bool,
}

impl Default for ShapeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ShapeRegistry {
    /// Creates a new shape registry with the standard empty and full block shapes.
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Self {
            shapes: Vec::new(),
            allows_registering: true,
        };

        // Register the two standard shapes - IDs must match ShapeId::EMPTY and ShapeId::FULL_BLOCK
        let empty_id = registry.register(&[]);
        debug_assert_eq!(empty_id, ShapeId::EMPTY);

        let full_id = registry.register(FULL_BLOCK_SHAPE);
        debug_assert_eq!(full_id, ShapeId::FULL_BLOCK);

        registry
    }

    /// Registers a new shape and returns its ID.
    ///
    /// # Panics
    /// Panics if the registry has been frozen.
    pub fn register(&mut self, shape: &'static [AABB]) -> ShapeId {
        assert!(
            self.allows_registering,
            "Cannot register shapes after the registry has been frozen"
        );

        let id = ShapeId(self.shapes.len() as u16);
        self.shapes.push(shape);
        id
    }

    /// Gets the shape for a given ID.
    ///
    /// Returns an empty slice if the ID is invalid.
    #[must_use]
    pub fn get(&self, id: ShapeId) -> &'static [AABB] {
        self.shapes.get(id.0 as usize).copied().unwrap_or(&[])
    }

    /// Returns the number of registered shapes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    /// Returns true if no shapes are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    /// Freezes the registry, preventing further registrations.
    pub fn freeze(&mut self) {
        self.allows_registering = false;
    }
}

// Static shape for full block - used during registry initialization
static FULL_BLOCK_SHAPE: &[AABB] = &[AABB::FULL_BLOCK];

/// Shape data for a block state, containing both collision and outline shapes.
#[derive(Debug, Clone, Copy)]
pub struct BlockShapes {
    pub collision: VoxelShape,
    pub outline: VoxelShape,
}

impl BlockShapes {
    /// Creates new block shapes.
    #[must_use]
    pub const fn new(collision: VoxelShape, outline: VoxelShape) -> Self {
        Self { collision, outline }
    }

    /// Full block collision and outline.
    pub const FULL_BLOCK: BlockShapes = BlockShapes::new(&[AABB::FULL_BLOCK], &[AABB::FULL_BLOCK]);

    /// Empty shapes (no collision, no outline).
    pub const EMPTY: BlockShapes = BlockShapes::new(&[], &[]);
}

use glam::{DVec3, IVec3, Vec3};
use steel_utils::types::BlockPos;

use super::properties::Direction;

/// Returns the overall bounding box of a voxel shape (union of all AABBs).
///
/// The shape must be non-empty; panics otherwise.
#[must_use]
pub fn bounding_box(shape: VoxelShape) -> AABB {
    debug_assert!(!shape.is_empty(), "bounding_box called on empty shape");
    let mut result = shape[0];
    for aabb in &shape[1..] {
        if aabb.min.x < result.min.x {
            result.min.x = aabb.min.x;
        }
        if aabb.min.y < result.min.y {
            result.min.y = aabb.min.y;
        }
        if aabb.min.z < result.min.z {
            result.min.z = aabb.min.z;
        }
        if aabb.max.x > result.max.x {
            result.max.x = aabb.max.x;
        }
        if aabb.max.y > result.max.y {
            result.max.y = aabb.max.y;
        }
        if aabb.max.z > result.max.z {
            result.max.z = aabb.max.z;
        }
    }
    result
}

/// Checks if a shape is a full block (covers the entire 0-1 cube).
///
/// This matches vanilla's `Block.isShapeFullBlock()` used by `isSolid()`.
///
/// TODO: Handle multi-AABB shapes whose union covers the full block (e.g. stacked slabs).
/// Vanilla uses exact boolean voxel arithmetic (`Shapes.joinIsNotEmpty`). No vanilla blocks
/// currently have multi-AABB full-block shapes, so single-AABB fast path suffices for now.
#[must_use]
pub fn is_shape_full_block(shape: VoxelShape) -> bool {
    // A full block shape must have exactly one AABB that covers 0-1 on all axes
    if shape.len() != 1 {
        return false;
    }

    let aabb = &shape[0];
    aabb.min.x <= 0.0
        && aabb.max.x >= 1.0
        && aabb.min.y <= 0.0
        && aabb.max.y >= 1.0
        && aabb.min.z <= 0.0
        && aabb.max.z >= 1.0
}

/// Support type for `is_face_sturdy` checks.
///
/// Determines what kind of support a block face provides for other blocks.
/// Used by fences, walls, torches, etc. to decide if they can connect/attach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportType {
    /// Full face support - the entire face must be solid.
    /// Used by most blocks that need a solid surface.
    Full,
    /// Center support - only the center of the face needs to be solid.
    /// Used by things like hanging signs that only need a small attachment point.
    Center,
    /// Rigid support - most of the face must be solid, but allows small gaps.
    /// Used by bells and similar blocks.
    Rigid,
}

/// Center support shape: a 4x4 pixel column in the center (2-14 in pixel coords = 0.125-0.875).
const CENTER_SUPPORT_MIN: f32 = 0.125; // 2/16
const CENTER_SUPPORT_MAX: f32 = 0.875; // 14/16

/// Rigid support requires coverage except for a 2-pixel border.
const RIGID_BORDER: f32 = 0.125; // 2/16

/// Checks if a shape fully covers a face (for `SupportType::Full`).
///
/// Returns true if the 2D projection of the shape on the given face
/// completely covers the 1x1 face area.
#[must_use]
pub fn is_face_full(shape: VoxelShape, direction: Direction) -> bool {
    if shape.is_empty() {
        return false;
    }

    // For a face to be "full", the shape's projection onto that face must cover 0.0-1.0
    // on both axes perpendicular to the direction.
    match direction {
        Direction::Down => covers_face_xy(shape, |aabb| aabb.min.y <= 0.0),
        Direction::Up => covers_face_xy(shape, |aabb| aabb.max.y >= 1.0),
        Direction::North => covers_face_xy_for_z(shape, |aabb| aabb.min.z <= 0.0),
        Direction::South => covers_face_xy_for_z(shape, |aabb| aabb.max.z >= 1.0),
        Direction::West => covers_face_yz(shape, |aabb| aabb.min.x <= 0.0),
        Direction::East => covers_face_yz(shape, |aabb| aabb.max.x >= 1.0),
    }
}

/// Checks if a shape provides center support on a face.
///
/// The center area is a 12x12 pixel region (0.125 to 0.875 on each axis).
#[must_use]
pub fn is_face_center_supported(shape: VoxelShape, direction: Direction) -> bool {
    if shape.is_empty() {
        return false;
    }

    // Check if any AABB in the shape covers the center region on the given face
    match direction {
        Direction::Down => shape.iter().any(|aabb| {
            aabb.min.y <= 0.0
                && aabb.min.x <= CENTER_SUPPORT_MIN
                && aabb.max.x >= CENTER_SUPPORT_MAX
                && aabb.min.z <= CENTER_SUPPORT_MIN
                && aabb.max.z >= CENTER_SUPPORT_MAX
        }),
        Direction::Up => shape.iter().any(|aabb| {
            aabb.max.y >= 1.0
                && aabb.min.x <= CENTER_SUPPORT_MIN
                && aabb.max.x >= CENTER_SUPPORT_MAX
                && aabb.min.z <= CENTER_SUPPORT_MIN
                && aabb.max.z >= CENTER_SUPPORT_MAX
        }),
        Direction::North => shape.iter().any(|aabb| {
            aabb.min.z <= 0.0
                && aabb.min.x <= CENTER_SUPPORT_MIN
                && aabb.max.x >= CENTER_SUPPORT_MAX
                && aabb.min.y <= CENTER_SUPPORT_MIN
                && aabb.max.y >= CENTER_SUPPORT_MAX
        }),
        Direction::South => shape.iter().any(|aabb| {
            aabb.max.z >= 1.0
                && aabb.min.x <= CENTER_SUPPORT_MIN
                && aabb.max.x >= CENTER_SUPPORT_MAX
                && aabb.min.y <= CENTER_SUPPORT_MIN
                && aabb.max.y >= CENTER_SUPPORT_MAX
        }),
        Direction::West => shape.iter().any(|aabb| {
            aabb.min.x <= 0.0
                && aabb.min.y <= CENTER_SUPPORT_MIN
                && aabb.max.y >= CENTER_SUPPORT_MAX
                && aabb.min.z <= CENTER_SUPPORT_MIN
                && aabb.max.z >= CENTER_SUPPORT_MAX
        }),
        Direction::East => shape.iter().any(|aabb| {
            aabb.max.x >= 1.0
                && aabb.min.y <= CENTER_SUPPORT_MIN
                && aabb.max.y >= CENTER_SUPPORT_MAX
                && aabb.min.z <= CENTER_SUPPORT_MIN
                && aabb.max.z >= CENTER_SUPPORT_MAX
        }),
    }
}

/// Checks if a shape provides rigid support on a face.
///
/// Rigid support requires coverage of most of the face except a small border.
#[must_use]
pub fn is_face_rigid_supported(shape: VoxelShape, direction: Direction) -> bool {
    if shape.is_empty() {
        return false;
    }

    // For rigid support, we need the shape to cover from RIGID_BORDER to 1-RIGID_BORDER
    let min_bound = RIGID_BORDER;
    let max_bound = 1.0 - RIGID_BORDER;

    match direction {
        Direction::Down => shape.iter().any(|aabb| {
            aabb.min.y <= 0.0
                && aabb.min.x <= min_bound
                && aabb.max.x >= max_bound
                && aabb.min.z <= min_bound
                && aabb.max.z >= max_bound
        }),
        Direction::Up => shape.iter().any(|aabb| {
            aabb.max.y >= 1.0
                && aabb.min.x <= min_bound
                && aabb.max.x >= max_bound
                && aabb.min.z <= min_bound
                && aabb.max.z >= max_bound
        }),
        Direction::North => shape.iter().any(|aabb| {
            aabb.min.z <= 0.0
                && aabb.min.x <= min_bound
                && aabb.max.x >= max_bound
                && aabb.min.y <= min_bound
                && aabb.max.y >= max_bound
        }),
        Direction::South => shape.iter().any(|aabb| {
            aabb.max.z >= 1.0
                && aabb.min.x <= min_bound
                && aabb.max.x >= max_bound
                && aabb.min.y <= min_bound
                && aabb.max.y >= max_bound
        }),
        Direction::West => shape.iter().any(|aabb| {
            aabb.min.x <= 0.0
                && aabb.min.y <= min_bound
                && aabb.max.y >= max_bound
                && aabb.min.z <= min_bound
                && aabb.max.z >= max_bound
        }),
        Direction::East => shape.iter().any(|aabb| {
            aabb.max.x >= 1.0
                && aabb.min.y <= min_bound
                && aabb.max.y >= max_bound
                && aabb.min.z <= min_bound
                && aabb.max.z >= max_bound
        }),
    }
}

/// Checks if a shape is sturdy on a face for the given support type.
#[must_use]
pub fn is_face_sturdy(shape: VoxelShape, direction: Direction, support_type: SupportType) -> bool {
    match support_type {
        SupportType::Full => is_face_full(shape, direction),
        SupportType::Center => is_face_center_supported(shape, direction),
        SupportType::Rigid => is_face_rigid_supported(shape, direction),
    }
}

// Helper: checks if shape covers full X-Y face (for Up/Down directions)
fn covers_face_xy(shape: VoxelShape, face_check: impl Fn(&AABB) -> bool) -> bool {
    // Simple check: if there's a single AABB that covers 0-1 on X and Z and touches the face
    shape.iter().any(|aabb| {
        face_check(aabb)
            && aabb.min.x <= 0.0
            && aabb.max.x >= 1.0
            && aabb.min.z <= 0.0
            && aabb.max.z >= 1.0
    })
}

// Helper: checks if shape covers full X-Y face (for North/South directions)
fn covers_face_xy_for_z(shape: VoxelShape, face_check: impl Fn(&AABB) -> bool) -> bool {
    shape.iter().any(|aabb| {
        face_check(aabb)
            && aabb.min.x <= 0.0
            && aabb.max.x >= 1.0
            && aabb.min.y <= 0.0
            && aabb.max.y >= 1.0
    })
}

// Helper: checks if shape covers full Y-Z face (for East/West directions)
fn covers_face_yz(shape: VoxelShape, face_check: impl Fn(&AABB) -> bool) -> bool {
    shape.iter().any(|aabb| {
        face_check(aabb)
            && aabb.min.y <= 0.0
            && aabb.max.y >= 1.0
            && aabb.min.z <= 0.0
            && aabb.max.z >= 1.0
    })
}
