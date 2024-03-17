use std::sync::RwLock;

use taffy::{AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, GridAutoFlow, JustifyContent, Line, Overflow, Point, Position, Rect, Size};

use crate::{Convert, Dimension, GridPlacement, LengthPercentage, LengthPercentageAuto, NonRepeatedTrackSizingFunction, TaffyResult, TrackSizingFunction};

pub struct Style(pub(crate) RwLock<taffy::Style>);

impl Style {
    pub fn new() -> Self {
        Self(RwLock::new(taffy::Style::DEFAULT))
    }

    pub fn get_display(&self) -> TaffyResult<Display> {
        Ok(self.0.read()?.display)
    }

    pub fn set_display(&self, value: Display) -> TaffyResult<()> {
        self.0.write()?.display = value;
        Ok(())
    }

    pub fn get_overflow(&self) -> TaffyResult<Point<Overflow>> {
        Ok(self.0.read()?.overflow)
    }

    pub fn set_overflow(&self, value: Point<Overflow>) -> TaffyResult<()> {
        self.0.write()?.overflow = value;
        Ok(())
    }

    pub fn get_scrollbar_width(&self) -> TaffyResult<f32> {
        Ok(self.0.read()?.scrollbar_width)
    }

    pub fn set_scrollbar_width(&self, value: f32) -> TaffyResult<()> {
        self.0.write()?.scrollbar_width = value;
        Ok(())
    }

    pub fn get_position(&self) -> TaffyResult<Position> {
        Ok(self.0.read()?.position)
    }

    pub fn set_position(&self, value: Position) -> TaffyResult<()> {
        self.0.write()?.position = value;
        Ok(())
    }

    pub fn get_inset(&self) -> TaffyResult<Rect<LengthPercentageAuto>> {
        Ok(self.0.read()?.inset.convert())
    }

    pub fn set_inset(&self, value: Rect<LengthPercentageAuto>) -> TaffyResult<()> {
        self.0.write()?.inset = value.convert();
        Ok(())
    }

    pub fn get_size(&self) -> TaffyResult<Size<Dimension>> {
        Ok(self.0.read()?.size.convert())
    }

    pub fn set_size(&self, value: Size<Dimension>) -> TaffyResult<()> {
        self.0.write()?.size = value.convert();
        Ok(())
    }

    pub fn get_min_size(&self) -> TaffyResult<Size<Dimension>> {
        Ok(self.0.read()?.min_size.convert())
    }

    pub fn set_min_size(&self, value: Size<Dimension>) -> TaffyResult<()> {
        self.0.write()?.min_size = value.convert();
        Ok(())
    }

    pub fn get_max_size(&self) -> TaffyResult<Size<Dimension>> {
        Ok(self.0.read()?.max_size.convert())
    }

    pub fn set_max_size(&self, value: Size<Dimension>) -> TaffyResult<()> {
        self.0.write()?.max_size = value.convert();
        Ok(())
    }

    pub fn get_aspect_ratio(&self) -> TaffyResult<Option<f32>> {
        Ok(self.0.read()?.aspect_ratio)
    }

    pub fn set_aspect_ratio(&self, value: Option<f32>) -> TaffyResult<()> {
        self.0.write()?.aspect_ratio = value;
        Ok(())
    }

    pub fn get_margin(&self) -> TaffyResult<Rect<LengthPercentageAuto>> {
        Ok(self.0.read()?.margin.convert())
    }

    pub fn set_margin(&self, value: Rect<LengthPercentageAuto>) -> TaffyResult<()> {
        self.0.write()?.margin = value.convert();
        Ok(())
    }

    pub fn get_padding(&self) -> TaffyResult<Rect<LengthPercentage>> {
        Ok(self.0.read()?.padding.convert())
    }

    pub fn set_padding(&self, value: Rect<LengthPercentage>) -> TaffyResult<()> {
        self.0.write()?.padding = value.convert();
        Ok(())
    }

    pub fn get_border(&self) -> TaffyResult<Rect<LengthPercentage>> {
        Ok(self.0.read()?.border.convert())
    }

    pub fn set_border(&self, value: Rect<LengthPercentage>) -> TaffyResult<()> {
        self.0.write()?.border = value.convert();
        Ok(())
    }

    pub fn get_align_items(&self) -> TaffyResult<Option<AlignItems>> {
        Ok(self.0.read()?.align_items)
    }

    pub fn set_align_items(&self, value: Option<AlignItems>) -> TaffyResult<()> {
        self.0.write()?.align_items = value;
        Ok(())
    }

    pub fn get_align_self(&self) -> TaffyResult<Option<AlignSelf>> {
        Ok(self.0.read()?.align_self)
    }

    pub fn set_align_self(&self, value: Option<AlignSelf>) -> TaffyResult<()> {
        self.0.write()?.align_self = value;
        Ok(())
    }

    pub fn get_justify_items(&self) -> TaffyResult<Option<AlignItems>> {
        Ok(self.0.read()?.justify_items)
    }

    pub fn set_justify_items(&self, value: Option<AlignItems>) -> TaffyResult<()> {
        self.0.write()?.justify_items = value;
        Ok(())
    }

    pub fn get_justify_self(&self) -> TaffyResult<Option<AlignSelf>> {
        Ok(self.0.read()?.justify_self)
    }

    pub fn set_justify_self(&self, value: Option<AlignSelf>) -> TaffyResult<()> {
        self.0.write()?.justify_self = value;
        Ok(())
    }

    pub fn get_align_content(&self) -> TaffyResult<Option<AlignContent>> {
        Ok(self.0.read()?.align_content)
    }

    pub fn set_align_content(&self, value: Option<AlignContent>) -> TaffyResult<()> {
        self.0.write()?.align_content = value;
        Ok(())
    }

    pub fn get_justify_content(&self) -> TaffyResult<Option<JustifyContent>> {
        Ok(self.0.read()?.justify_content)
    }

    pub fn set_justify_content(&self, value: Option<JustifyContent>) -> TaffyResult<()> {
        self.0.write()?.justify_content = value;
        Ok(())
    }

    pub fn get_gap(&self) -> TaffyResult<Size<LengthPercentage>> {
        Ok(self.0.read()?.gap.convert())
    }

    pub fn set_gap(&self, value: Size<LengthPercentage>) -> TaffyResult<()> {
        self.0.write()?.gap = value.convert();
        Ok(())
    }

    pub fn get_flex_direction(&self) -> TaffyResult<FlexDirection> {
        Ok(self.0.read()?.flex_direction)
    }

    pub fn set_flex_direction(&self, value: FlexDirection) -> TaffyResult<()> {
        self.0.write()?.flex_direction = value;
        Ok(())
    }

    pub fn get_flex_wrap(&self) -> TaffyResult<FlexWrap> {
        Ok(self.0.read()?.flex_wrap)
    }

    pub fn set_flex_wrap(&self, value: FlexWrap) -> TaffyResult<()> {
        self.0.write()?.flex_wrap = value;
        Ok(())
    }

    pub fn get_flex_basis(&self) -> TaffyResult<Dimension> {
        Ok(self.0.read()?.flex_basis.convert())
    }

    pub fn set_flex_basis(&self, value: Dimension) -> TaffyResult<()> {
        self.0.write()?.flex_basis = value.convert();
        Ok(())
    }

    pub fn get_flex_grow(&self) -> TaffyResult<f32> {
        Ok(self.0.read()?.flex_grow.convert())
    }

    pub fn set_flex_grow(&self, value: f32) -> TaffyResult<()> {
        self.0.write()?.flex_grow = value;
        Ok(())
    }

    pub fn get_flex_shrink(&self) -> TaffyResult<f32> {
        Ok(self.0.read()?.flex_shrink.convert())
    }

    pub fn set_flex_shrink(&self, value: f32) -> TaffyResult<()> {
        self.0.write()?.flex_shrink = value;
        Ok(())
    }

    pub fn get_grid_template_rows(&self) -> TaffyResult<Vec<TrackSizingFunction>> {
        Ok(self.0.read()?.grid_template_rows.convert())
    }

    pub fn set_grid_template_rows(&self, value: Vec<TrackSizingFunction>) -> TaffyResult<()> {
        self.0.write()?.grid_template_rows = value.convert();
        Ok(())
    }

    pub fn get_grid_template_columns(&self) -> TaffyResult<Vec<TrackSizingFunction>> {
        Ok(self.0.read()?.grid_template_columns.convert())
    }

    pub fn set_grid_template_columns(&self, value: Vec<TrackSizingFunction>) -> TaffyResult<()> {
        self.0.write()?.grid_template_columns = value.convert();
        Ok(())
    }

    pub fn get_grid_auto_rows(&self) -> TaffyResult<Vec<NonRepeatedTrackSizingFunction>> {
        Ok(self.0.read()?.grid_auto_rows.convert())
    }

    pub fn set_grid_auto_rows(
        &self,
        value: Vec<NonRepeatedTrackSizingFunction>,
    ) -> TaffyResult<()> {
        self.0.write()?.grid_auto_rows = value.convert();
        Ok(())
    }

    pub fn get_grid_auto_columns(&self) -> TaffyResult<Vec<NonRepeatedTrackSizingFunction>> {
        Ok(self.0.read()?.grid_auto_columns.convert())
    }

    pub fn set_grid_auto_columns(
        &self,
        value: Vec<NonRepeatedTrackSizingFunction>,
    ) -> TaffyResult<()> {
        self.0.write()?.grid_auto_columns = value.convert();
        Ok(())
    }

    pub fn get_grid_auto_flow(&self) -> TaffyResult<GridAutoFlow> {
        Ok(self.0.read()?.grid_auto_flow)
    }

    pub fn set_grid_auto_flow(&self, value: GridAutoFlow) -> TaffyResult<()> {
        self.0.write()?.grid_auto_flow = value;
        Ok(())
    }

    pub fn get_grid_row(&self) -> TaffyResult<Line<GridPlacement>> {
        Ok(self.0.read()?.grid_row.convert())
    }

    pub fn set_grid_row(&self, value: Line<GridPlacement>) -> TaffyResult<()> {
        self.0.write()?.grid_row = value.convert();
        Ok(())
    }

    pub fn get_grid_column(&self) -> TaffyResult<Line<GridPlacement>> {
        Ok(self.0.read()?.grid_column.convert())
    }

    pub fn set_grid_column(&self, value: Line<GridPlacement>) -> TaffyResult<()> {
        self.0.write()?.grid_column = value.convert();
        Ok(())
    }
}

impl From<taffy::Style> for Style {
    fn from(value: taffy::Style) -> Self {
        Self(RwLock::new(value.to_owned()))
    }
}
