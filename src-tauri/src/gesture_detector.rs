use crate::GesturePoint;
use serde_json::json;

pub fn detect_gesture(
    points: &[GesturePoint],
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    if points.len() < 3 {
        return Ok(json!({
            "gesture": "none",
            "confidence": 0.0
        }));
    }

    let simplified = simplify_path(points, 5.0);

    if simplified.len() < 2 {
        return Ok(json!({
            "gesture": "none",
            "confidence": 0.0
        }));
    }

    let direction = calculate_direction(&simplified);
    let gesture_type = classify_gesture(&simplified, &direction);

    Ok(json!({
        "gesture": gesture_type,
        "confidence": 0.85,
        "points_count": points.len(),
        "simplified_points": simplified.len()
    }))
}

fn simplify_path(points: &[GesturePoint], epsilon: f32) -> Vec<(i32, i32)> {
    if points.len() < 3 {
        return points.iter().map(|p| (p.x, p.y)).collect();
    }

    let coords: Vec<(f32, f32)> = points.iter().map(|p| (p.x as f32, p.y as f32)).collect();

    let mut simplified = vec![coords[0]];
    let mut index = 0;

    while index < coords.len() - 1 {
        let max_dist = coords[index + 1..]
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                (
                    i + 1,
                    perpendicular_distance(p, coords[index], coords[coords.len() - 1]),
                )
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        if let Some((i, dist)) = max_dist {
            if dist > epsilon as f32 {
                index += i;
                simplified.push(coords[index]);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    simplified.push(*coords.last().unwrap());
    simplified
        .into_iter()
        .map(|(x, y)| (x as i32, y as i32))
        .collect()
}

fn perpendicular_distance(point: (f32, f32), start: (f32, f32), end: (f32, f32)) -> f32 {
    let px = end.0 - start.0;
    let py = end.1 - start.1;

    if px == 0.0 && py == 0.0 {
        return ((point.0 - start.0).powi(2) + (point.1 - start.1).powi(2)).sqrt();
    }

    let t = ((point.0 - start.0) * px + (point.1 - start.1) * py) / (px * px + py * py);
    let t = t.max(0.0).min(1.0);

    let proj_x = start.0 + t * px;
    let proj_y = start.1 + t * py;

    ((point.0 - proj_x).powi(2) + (point.1 - proj_y).powi(2)).sqrt()
}

fn calculate_direction(points: &[(i32, i32)]) -> String {
    if points.len() < 2 {
        return "none".to_string();
    }

    let start = points[0];
    let end = points[points.len() - 1];

    let dx = (end.0 - start.0) as f32;
    let dy = (end.1 - start.1) as f32;

    let angle = dy.atan2(dx).to_degrees();
    let normalized_angle = ((angle + 360.0) % 360.0) as i32;

    match normalized_angle {
        0..=45 | 315..=359 => "right".to_string(),
        46..=135 => "down".to_string(),
        136..=225 => "left".to_string(),
        226..=314 => "up".to_string(),
        _ => "unknown".to_string(),
    }
}

fn classify_gesture(points: &[(i32, i32)], direction: &str) -> String {
    if points.len() < 2 {
        return "unknown".to_string();
    }

    let start = points[0];
    let end = points[points.len() - 1];

    let total_distance = points
        .windows(2)
        .map(|w| {
            let dx = (w[1].0 - w[0].0) as f32;
            let dy = (w[1].1 - w[0].1) as f32;
            (dx * dx + dy * dy).sqrt()
        })
        .sum::<f32>();

    let direct_distance =
        ((end.0 - start.0).pow(2) as f32 + (end.1 - start.1).pow(2) as f32).sqrt();

    let straightness = if total_distance > 0.0 {
        direct_distance / total_distance
    } else {
        0.0
    };

    if straightness > 0.7 {
        match direction.as_str() {
            "left" => "swipe_left",
            "right" => "swipe_right",
            "up" => "swipe_up",
            "down" => "swipe_down",
            _ => "unknown",
        }
        .to_string()
    } else if points.len() > 10 {
        check_for_complex_gesture(points)
    } else {
        "unknown".to_string()
    }
}

fn check_for_complex_gesture(points: &[(i32, i32)]) -> String {
    if points.len() < 4 {
        return "unknown".to_string();
    }

    let start = points[0];
    let end = points[points.len() - 1];
    let mid = points[points.len() / 2];

    let dx_start = (mid.0 - start.0) as f32;
    let dy_start = (mid.1 - start.1) as f32;
    let dx_end = (end.0 - mid.0) as f32;
    let dy_end = (end.1 - mid.1) as f32;

    let dot_product = dx_start * dx_end + dy_start * dy_end;

    if dot_product < -100.0 {
        "angle".to_string()
    } else {
        "curve".to_string()
    }
}
