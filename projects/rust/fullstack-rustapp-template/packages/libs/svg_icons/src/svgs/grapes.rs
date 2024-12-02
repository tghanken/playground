use crate::{IconRef, Icons};
use crate::svg_data::geometry::{Coordinate, Dimension, GeometryData, ViewBox};
use crate::svg_data::paint::{PaintData, Stroke, StrokeLinecap, StrokeLineJoin};
use crate::svg_data::SvgData;

impl Icons {
    pub const GRAPES: IconRef = IconRef {
        name: "Grapes Icon",
        svg_data: SvgData {
            geometry_data: Some(GeometryData {
                view_box: Some(ViewBox(0, 0, 24, 24)),
                x: Some(Coordinate(24)),
                y: Some(Coordinate(24)),
                height: Some(Dimension(24)),
                width: Some(Dimension(24)),
            }),
            paint_data: Some(PaintData {
                stroke: Some(Stroke::CurrentColor),
                fill: None,
                stroke_width: Some(2),
                stroke_linecap: Some(StrokeLinecap::Round),
                stroke_linejoin: Some(StrokeLineJoin::Round),
            }),
            preserve_aspect_ratio: None,
            path: "<path d=\"M22 5V2l-5.89 5.89\"/>
                    <circle cx=\"16.6\" cy=\"15.89\" r=\"3\"/>
                    <circle cx=\"8.11\" cy=\"7.4\" r=\"3\"/>
                    <circle cx=\"12.35\" cy=\"11.65\" r=\"3\"/>
                    <circle cx=\"13.91\" cy=\"5.85\" r=\"3\"/>
                    <circle cx=\"18.15\" cy=\"10.09\" r=\"3\"/>
                    <circle cx=\"6.56\" cy=\"13.2\" r=\"3\"/>
                    <circle cx=\"10.8\" cy=\"17.44\" r=\"3\"/>
                    <circle cx=\"5\" cy=\"19\" r=\"3\"/>",
        },
    };
}
