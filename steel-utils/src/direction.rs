//! Direction enum representing the six cardinal directions in Minecraft.
//!
//! This is moved from `steel-registry::blocks::properties::Direction`.

use std::io::{self, Cursor};

use crate::{codec::VarInt, math::Axis, serial::ReadFrom, types::BlockPos};

/// The six cardinal directions in Minecraft.
#[derive(Clone, Copy, Debug)]
#[derive_const(PartialEq, Eq)]
pub enum Direction {
    /// Negative Y direction.
    Down,
    /// Positive Y direction.
    Up,
    /// Negative Z direction.
    North,
    /// Positive Z direction.
    South,
    /// Negative X direction.
    West,
    /// Positive X direction.
    East,
}

impl ReadFrom for Direction {
    fn read(data: &mut Cursor<&[u8]>) -> io::Result<Self> {
        let id = VarInt::read(data)?.0;
        match id {
            0 => Ok(Self::Down),
            1 => Ok(Self::Up),
            2 => Ok(Self::North),
            3 => Ok(Self::South),
            4 => Ok(Self::West),
            5 => Ok(Self::East),
            _ => Err(io::Error::other("Invalid Direction id")),
        }
    }
}

impl Direction {
    /// Returns the block position offset for this direction.
    #[must_use]
    pub const fn offset(&self) -> (i32, i32, i32) {
        match self {
            Self::Down => (0, -1, 0),
            Self::Up => (0, 1, 0),
            Self::North => (0, 0, -1),
            Self::South => (0, 0, 1),
            Self::West => (-1, 0, 0),
            Self::East => (1, 0, 0),
        }
    }

    /// Returns the block position relative to the given position in this direction.
    #[must_use]
    pub const fn relative(&self, pos: &BlockPos) -> BlockPos {
        let (dx, dy, dz) = self.offset();
        pos.offset(dx, dy, dz)
    }

    /// Returns the axis this direction is on.
    #[must_use]
    pub const fn get_axis(&self) -> Axis {
        match self {
            Self::Down | Self::Up => Axis::Y,
            Self::North | Self::South => Axis::Z,
            Self::West | Self::East => Axis::X,
        }
    }

    /// Returns the opposite direction.
    #[must_use]
    pub const fn opposite(&self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }

    /// Returns the horizontal direction from a yaw rotation.
    ///
    /// Yaw values follow Minecraft's convention:
    /// - 0° = South (+Z)
    /// - 90° = West (-X)
    /// - 180° = North (-Z)
    /// - 270° = East (+X)
    #[must_use]
    pub fn from_yaw(yaw: f32) -> Self {
        let adjusted = yaw.rem_euclid(360.0);
        match adjusted {
            y if !(45.0..315.0).contains(&y) => Self::South,
            y if y < 135.0 => Self::West,
            y if y < 225.0 => Self::North,
            _ => Self::East,
        }
    }

    /// Returns the yaw rotation for this direction.
    ///
    /// Only meaningful for horizontal directions.
    /// Vertical directions return 0.
    #[must_use]
    pub const fn to_yaw(&self) -> f32 {
        match self {
            Self::North => 180.0,
            Self::South | Self::Up | Self::Down => 0.0,
            Self::West => 90.0,
            Self::East => 270.0,
        }
    }

    /// Returns the axis this direction is on.
    #[must_use]
    pub const fn axis(&self) -> Axis {
        self.get_axis()
    }

    /// Returns whether this direction is horizontal (not up or down).
    #[must_use]
    pub const fn is_horizontal(&self) -> bool {
        matches!(
            self,
            Self::North | Self::South | Self::East | Self::West
        )
    }

    /// Rotates this direction 90 degrees clockwise around the Y axis.
    ///
    /// Vertical directions are unchanged.
    #[must_use]
    pub const fn rotate_y_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            other => *other,
        }
    }

    /// Rotates this direction 90 degrees counter-clockwise around the Y axis.
    ///
    /// Vertical directions are unchanged.
    #[must_use]
    pub const fn rotate_y_counter_clockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
            other => *other,
        }
    }

    /// The order in which neighbor shape updates are processed.
    /// This matches vanilla's `BlockBehaviour.UPDATE_SHAPE_ORDER`.
    pub const UPDATE_SHAPE_ORDER: [Self; 6] = [
        Self::West,
        Self::East,
        Self::North,
        Self::South,
        Self::Down,
        Self::Up,
    ];

    /// Vanilla: `LiquidBlock.POSSIBLE_FLOW_DIRECTIONS` mapped through `getOpposite()`.
    /// Used by `LiquidBlock.shouldSpreadLiquid()` to check neighbors for lava-water interactions.
    pub const FLOW_NEIGHBOR_CHECK: [Self; 5] = [
        Self::Up,
        Self::North,
        Self::South,
        Self::West,
        Self::East,
    ];

    /// The 4 horizontal directions.
    pub const HORIZONTAL: [Self; 4] = [
        Self::North,
        Self::South,
        Self::West,
        Self::East,
    ];

    /// The 6 directions.
    pub const ALL: [Self; 6] = [
        Self::North,
        Self::South,
        Self::West,
        Self::East,
        Self::Down,
        Self::Up,
    ];

    /// Returns all directions ordered by how closely they match the player's look direction.
    ///
    /// This matches vanilla's `Direction.orderedByNearest(Entity)`.
    /// - `yaw`: Player's yaw rotation in degrees (0 = South, 90 = West, 180 = North, 270 = East)
    /// - `pitch`: Player's pitch rotation in degrees (negative = looking up, positive = looking down)
    #[must_use]
    pub fn ordered_by_nearest(yaw: f32, pitch: f32) -> [Self; 6] {
        // Convert to radians and negate yaw to match vanilla's coordinate system
        let pitch_rad = pitch.to_radians();
        let yaw_rad = (-yaw).to_radians();

        let pitch_sin = pitch_rad.sin();
        let pitch_cos = pitch_rad.cos();
        let yaw_sin = yaw_rad.sin();
        let yaw_cos = yaw_rad.cos();

        // Determine which direction on each axis the player is looking
        let x_pos = yaw_sin > 0.0;
        let y_pos = pitch_sin < 0.0; // Negative pitch = looking up
        let z_pos = yaw_cos > 0.0;

        // Calculate magnitude of look direction on each axis
        let x_yaw = if x_pos { yaw_sin } else { -yaw_sin };
        let y_mag = if y_pos { -pitch_sin } else { pitch_sin };
        let z_yaw = if z_pos { yaw_cos } else { -yaw_cos };
        let x_mag = x_yaw * pitch_cos;
        let z_mag = z_yaw * pitch_cos;

        // Determine the primary direction on each axis
        let axis_x = if x_pos {
            Self::East
        } else {
            Self::West
        };
        let axis_y = if y_pos {
            Self::Up
        } else {
            Self::Down
        };
        let axis_z = if z_pos {
            Self::South
        } else {
            Self::North
        };

        // Sort axes by magnitude and build the direction array
        if x_yaw > z_yaw {
            if y_mag > x_mag {
                Self::make_direction_array(axis_y, axis_x, axis_z)
            } else if z_mag > y_mag {
                Self::make_direction_array(axis_x, axis_z, axis_y)
            } else {
                Self::make_direction_array(axis_x, axis_y, axis_z)
            }
        } else if y_mag > z_mag {
            Self::make_direction_array(axis_y, axis_z, axis_x)
        } else if x_mag > y_mag {
            Self::make_direction_array(axis_z, axis_x, axis_y)
        } else {
            Self::make_direction_array(axis_z, axis_y, axis_x)
        }
    }

    /// Creates an array of all 6 directions ordered by magnitude.
    ///
    /// The order is: 3 primary directions by magnitude, then their opposites in reverse order.
    /// This matches vanilla's `Direction.makeDirectionArray()`.
    const fn make_direction_array(
        axis1: Self,
        axis2: Self,
        axis3: Self,
    ) -> [Self; 6] {
        [
            axis1,
            axis2,
            axis3,
            axis3.opposite(),
            axis2.opposite(),
            axis1.opposite(),
        ]
    }

    /// Returns the direction name as a string (for `PropertyEnum` compatibility).
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Down => "down",
            Self::Up => "up",
            Self::North => "north",
            Self::South => "south",
            Self::West => "west",
            Self::East => "east",
        }
    }
}
