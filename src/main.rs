use domrs::{HtmlBodyElement, HtmlDocument, HtmlElement, HtmlHeadElement, SvgElement};
use std::f64::consts::PI;
use std::fmt::Write;

const BG_COLOR: &str = "#336633";
const FG_COLOR: &str = "#99CC00";

/// Converts degrees into radians.
fn deg_to_rad(deg: f64) -> f64 {
  deg * 2.0 * PI / 360.0
}

///
fn get_path_points(x: f64, y: f64, r: f64) -> Vec<(f64, f64)> {
  let mut points = vec![];
  let a = r * deg_to_rad(30.0).cos();
  let b = r * deg_to_rad(30.0).sin();
  points.push((x + a, y - b));
  points.push((x, y - r));
  points.push((x - a, y - b));
  points.push((x - a, y + b));
  points.push((x, y + r));
  points.push((x + a, y + b));
  points
}

fn get_svg_rect(x: f64, y: f64, width: f64, height: f64, bg_color: &str) -> HtmlElement {
  let mut rect = HtmlElement::new("rect");
  rect.set_attr("x", format!("{:.1}", x));
  rect.set_attr("y", format!("{:.1}", y));
  rect.set_attr("width", format!("{:.1}", width));
  rect.set_attr("height", format!("{:.1}", height));
  rect.set_attr("stroke", "none");
  rect.set_attr("fill", bg_color);
  rect
}

fn get_svg_path(points: &[(f64, f64)], fg_color: &str) -> HtmlElement {
  let mut d = String::new();
  let _ = write!(&mut d, "M {:.1},{:.1}", points[0].0, points[0].1);
  let _ = write!(&mut d, " L {:.1},{:.1}", points[1].0, points[1].1);
  let _ = write!(&mut d, " L {:.1},{:.1}", points[2].0, points[2].1);
  let _ = write!(&mut d, " L {:.1},{:.1}", points[3].0, points[3].1);
  let _ = write!(&mut d, " L {:.1},{:.1}", points[4].0, points[4].1);
  let _ = write!(&mut d, " L {:.1},{:.1}", points[5].0, points[5].1);
  let _ = write!(&mut d, " Z");
  let mut path = HtmlElement::new("path");
  path.set_attr("d", d);
  path.set_attr("stroke", "none");
  path.set_attr("fill", fg_color);
  path
}

fn get_svg_line(x1: f64, y1: f64, x2: f64, y2: f64, bg_color: &str, line_width: f64) -> HtmlElement {
  let mut line = HtmlElement::new("line");
  line.set_attr("x1", format!("{:.1}", x1));
  line.set_attr("y1", format!("{:.1}", y1));
  line.set_attr("x2", format!("{:.1}", x2));
  line.set_attr("y2", format!("{:.1}", y2));
  line.set_attr("stroke", bg_color);
  line.set_attr("stroke-width", format!("{:.1}", line_width));
  line.set_attr("stroke-linecap", "square");
  line
}

fn get_svg(width: f64, height: f64, line_width: f64) -> HtmlElement {
  let mut svg: HtmlElement = SvgElement::default().with_width(format!("{:.1}", width)).with_height(format!("{:.1}", height)).into();

  let w_2 = width / 2.0;
  let h_2 = height / 2.0;
  let radius = (if w_2 < h_2 { w_2 } else { h_2 }) * 0.8;

  let points = get_path_points(w_2, h_2, radius);
  let coeff = points[3].1 - points[2].1;

  svg.add_child(get_svg_rect(0.0, 0.0, width, height, BG_COLOR));
  svg.add_child(get_svg_path(&points, FG_COLOR));
  svg.add_child(get_svg_line(
    points[2].0,
    points[2].1 + 0.15 * coeff,
    points[4].0 + 0.35 * coeff,
    points[4].1,
    BG_COLOR,
    line_width,
  ));
  svg.add_child(get_svg_line(
    points[1].0 - 0.23 * coeff,
    points[1].1,
    points[5].0,
    points[5].1 + 0.12 * coeff,
    BG_COLOR,
    line_width,
  ));
  svg.add_child(get_svg_line(points[1].0 + 0.11 * coeff, points[1].1, points[3].0, points[4].1, BG_COLOR, line_width));
  svg.add_child(get_svg_line(
    points[1].0 + 0.65 * coeff,
    points[1].1,
    points[3].0 + 0.35 * coeff,
    points[4].1,
    BG_COLOR,
    line_width,
  ));
  svg.add_child(get_svg_line(
    points[0].0 + 2.0,
    points[5].1 - 0.45 * coeff,
    points[4].0 + 0.35 * coeff,
    points[4].1,
    BG_COLOR,
    line_width,
  ));
  svg
}

fn main() {
  let head = HtmlHeadElement::default().with_charset("UTF-8").with_title("DSNTK LOGO");

  let mut body = HtmlBodyElement::default();
  body.add_child(get_svg(700.0, 700.0, 9.0));
  body.add_br();
  body.add_child(get_svg(400.0, 400.0, 7.0));
  body.add_br();
  body.add_child(get_svg(100.0, 100.0, 3.0));

  let doc = HtmlDocument::new("en", head.into(), body.into());
  doc.save("./out/dsntk-logo.html").expect("writing file failed");
}
