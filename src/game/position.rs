#[derive(Copy, Clone)]
pub struct Position(u8);

impl Position {
    pub const MAX_COLUMN: u8 = 15;

    pub fn new(column: u8, row: u8) -> Self {
        Self((column & Self::MAX_COLUMN) << 1 | row & 1)
    }

    pub fn column(self) -> u8 {
        self.0 >> 1
    }

    pub fn row(self) -> u8 {
        self.0 & 1
    }

    #[must_use]
    pub fn with_row(self, row: u8) -> Self {
        if row & 1 != 0 {
            Self(self.0 | 1)
        } else {
            Self(self.0 & !1)
        }
    }

    #[must_use]
    pub fn with_column(self, column: u8) -> Self {
        Self((column & Self::MAX_COLUMN) << 1 | self.0 & 1)
    }

    #[must_use]
    pub fn nudge_column_saturating(self, offset: i8) -> Self {
        self.with_column(
            self.column()
                .saturating_add_signed(offset)
                .clamp(0, Self::MAX_COLUMN),
        )
    }

    #[must_use]
    pub fn nudge_column_overflowing(self, offset: i8) -> (Self, bool) {
        (
            self.nudge_column_saturating(offset),
            self.column().saturating_add_signed(offset) > Self::MAX_COLUMN
                || offset < 0 && self.column() < offset.abs_diff(0),
        )
    }
}
