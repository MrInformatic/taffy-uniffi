uniffi::include_scaffolding!("taffy");

mod error;
mod style;
mod tree;

pub use crate::{error::*, style::*, tree::*};
use taffy::{
    style_helpers::{TaffyAuto, TaffyGridLine, TaffyGridSpan},
    Line, MinMax, Point, Rect, Size,
};
pub use taffy::{
    AlignContent, AlignItems, Display, FlexDirection, FlexWrap, GridAutoFlow, Layout, NodeId,
    Overflow, Position,
};

pub type PointFloat = Point<f32>;
pub type SizeFloat = Size<f32>;
pub type RectFloat = Rect<f32>;
pub type SizeAvailableSpace = Size<AvailableSpace>;
pub type SizeOptionFloat = Size<Option<f32>>;
pub type SizeDimension = Size<Dimension>;
pub type RectLengthPercentage = Rect<LengthPercentage>;
pub type PointOverflow = Point<Overflow>;
pub type RectLengthPercentageAuto = Rect<LengthPercentageAuto>;
pub type NonRepeatedTrackSizingFunction = MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>;
pub type LineGridPlacement = Line<GridPlacement>;
pub type SizeLengthPercentage = Size<LengthPercentage>;

macro_rules! uniffi_enum {
    (pub enum $name:ident { $( $variant:ident $( { $( $field_name:ident : $field_type:ty ),* } )? ),* } ) => {
        pub enum $name {
            $(
                $variant $( { $( $field_name: $field_type ),* } )*
            ),*
        }

        impl Convert<$name> for taffy::$name {
            fn convert(&self) -> $name {
                match self {
                    $(
                        taffy::$name::$variant $( ( $( $field_name ),* ) )* => $name::$variant $( { $( $field_name: $field_name.convert() ),* } )*
                    ),*
                }
            }
        }

        impl Convert<taffy::$name> for $name {
            fn convert(&self) -> taffy::$name {
                match self {
                    $(
                        $name::$variant $( { $( $field_name ),* } )* => taffy::$name::$variant $( ( $( $field_name.convert() ),* ) )*
                    ),*
                }
            }
        }
    }
}

uniffi_enum! {
    pub enum AvailableSpace {
        Definite { value: f32 },
        MinContent,
        MaxContent
    }
}

uniffi_enum! {
    pub enum Dimension {
        Length { value: f32 },
        Percent { value: f32 },
        Auto
    }
}

uniffi_enum! {
    pub enum LengthPercentage {
        Length { value: f32 },
        Percent { value: f32 }
    }
}

uniffi_enum! {
    pub enum LengthPercentageAuto {
        Length { value: f32 },
        Percent { value: f32 },
        Auto
    }
}

uniffi_enum! {
    pub enum TrackSizingFunction {
        Single {
            func: NonRepeatedTrackSizingFunction
        },
        Repeat {
            rep: GridTrackRepetition,
            funcs: Vec<NonRepeatedTrackSizingFunction>
        }
    }
}

uniffi_enum! {
    pub enum MinTrackSizingFunction {
        Fixed { value: LengthPercentage },
        MinContent,
        MaxContent,
        Auto
    }
}

uniffi_enum! {
    pub enum MaxTrackSizingFunction {
        Fixed { value: LengthPercentage },
        MinContent,
        MaxContent,
        FitContent { value: LengthPercentage },
        Auto,
        Fraction { value: f32 }
    }
}

uniffi_enum! {
    pub enum GridTrackRepetition {
        AutoFill,
        AutoFit,
        Count { value: u16 }
    }
}

pub enum GridPlacement {
    Line { index: i16 },
    Span { span: u16 },
    Auto,
}

impl Convert<taffy::GridPlacement> for GridPlacement {
    fn convert(&self) -> taffy::GridPlacement {
        match self {
            Self::Line { index } => taffy::GridPlacement::from_line_index(index.convert()),
            Self::Span { span } => taffy::GridPlacement::from_span(span.convert()),
            Self::Auto => taffy::GridPlacement::AUTO,
        }
    }
}

impl Convert<GridPlacement> for taffy::GridPlacement {
    fn convert(&self) -> GridPlacement {
        match self {
            Self::Line(index) => GridPlacement::Line {
                index: index.as_i16(),
            },
            Self::Span(span) => GridPlacement::Span { span: span.convert() },
            Self::Auto => GridPlacement::Auto,
        }
    }
}

pub trait Convert<T> {
    fn convert(&self) -> T;
}

impl<T, U> Convert<Vec<U>> for Vec<T>
where
    T: Convert<U>,
{
    fn convert(&self) -> Vec<U> {
        self.into_iter().map(Convert::convert).collect()
    }
}

impl Convert<u16> for u16 {
    fn convert(&self) -> u16 {
        *self
    }
}

impl Convert<i16> for i16 {
    fn convert(&self) -> i16 {
        *self
    }
}

impl Convert<f32> for f32 {
    fn convert(&self) -> f32 {
        *self
    }
}

impl<T, U> Convert<Size<U>> for Size<T>
where
    T: Convert<U>,
{
    fn convert(&self) -> Size<U> {
        Size {
            width: self.width.convert(),
            height: self.height.convert(),
        }
    }
}

impl<T, U> Convert<Rect<U>> for Rect<T>
where
    T: Convert<U>,
{
    fn convert(&self) -> Rect<U> {
        Rect {
            top: self.top.convert(),
            bottom: self.bottom.convert(),
            left: self.left.convert(),
            right: self.right.convert(),
        }
    }
}

impl<T, U> Convert<Point<U>> for Point<T>
where
    T: Convert<U>,
{
    fn convert(&self) -> Point<U> {
        Point {
            x: self.x.convert(),
            y: self.y.convert(),
        }
    }
}

impl<T, U> Convert<Line<U>> for Line<T>
where
    T: Convert<U>,
{
    fn convert(&self) -> Line<U> {
        Line {
            start: self.start.convert(),
            end: self.end.convert(),
        }
    }
}

impl<T1, T2, U1, U2> Convert<MinMax<U1, U2>> for MinMax<T1, T2>
where
    T1: Convert<U1>,
    T2: Convert<U2>,
{
    fn convert(&self) -> MinMax<U1, U2> {
        MinMax {
            min: self.min.convert(),
            max: self.max.convert(),
        }
    }
}

impl UniffiCustomTypeConverter for NodeId {
    type Builtin = u64;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        Ok(val.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.into()
    }
}
