use plotters::style::{RGBAColor, ShapeStyle};

pub const OUTPUT_GRAPH_DIR: &str = "graph/";
pub const OUTPUT_RESULT_GRAPH: &str = "result.png";
pub const OUTPUT_TRAINING_GRAPH: &str = "training.gif";

pub const RESULT_TITLE: &str = "Linear Regression";
pub const TRAINING_TITLE: &str = "Linear Regression Training";

pub const RESULT_GRAPH_SIZE: (u32, u32) = (1000, 750);
pub const TRAINING_GRAPH_SIZE: (u32, u32) = (RESULT_GRAPH_SIZE.0, RESULT_GRAPH_SIZE.1);

pub const GIF_FRAME_DELAY: u32 = 42;

pub const GIF_FRAME_START_STEP: usize = 1;

pub const GIF_FRAME_START_PHASE: usize = 2usize.pow(8) - 1;
pub const GIF_FRAME_MID_PHASE: usize = GIF_FRAME_START_PHASE + 1;

pub const GIF_FRAME_MID_STEP: usize = 2usize.pow(8);

pub const GIF_FRAME_END_PHASE: usize = GIF_FRAME_MID_PHASE + 1;

pub const GIF_FRAME_END_STEP: usize = 2usize.pow(10);

pub const REGRESSION_LINE_STYLE: ShapeStyle = ShapeStyle {
	color: RGBAColor(3, 136, 252, 0.42),
	filled: true,
	stroke_width: 2,
};
pub const DATA_DOT_STYLE: ShapeStyle = ShapeStyle {
	color: RGBAColor(16, 227, 82, 1.0),
	filled: true,
	stroke_width: 1,
};
pub const DIFF_STYLE: ShapeStyle = ShapeStyle {
	color: RGBAColor(255, 0, 0, 1.0),
	filled: true,
	stroke_width: 1,
};
