use crate::intervals::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntervalTransform {
    pub interval: Interval,
    pub addend: i64
}

impl IntervalTransform {
    pub fn transform_point(self: &Self, point: i64) -> i64 {
        point + self.addend
    }

    pub fn transform_interval(self: &Self, interval: &Interval) -> Interval{
        Interval(interval.0 + self.addend, interval.1 + self.addend)
    }
}


pub fn apply_transform_layer_to_point(layer: &Vec<IntervalTransform>, point: i64) -> i64 {
    for transform in layer.iter() {
        if transform.interval.contains(point) {
            return transform.transform_point(point);
        }
    }

    point
}

// IMPORTANT: This function only works correctly if the transforms in the layer are sorted
pub fn apply_transform_layer_to_interval(layer: &Vec<IntervalTransform>, interval: Interval) -> Vec<Interval> {
    let mut output_intervals = Vec::<Interval>::new();
    let mut remaining_interval = Some(interval);
    let mut last_antecedent = None;
    let mut last_consequent = None;

    for transform in layer.iter() {
        if remaining_interval == None {
            break;
        }

        let intersection_result = remaining_interval.unwrap()
            .intersect_with(&transform.interval);

        last_antecedent = intersection_result.antecedent;
        last_consequent = intersection_result.consequent;

        if intersection_result.intersection != None {
            let intersection = intersection_result.intersection.unwrap();
            let transformed_intersection = transform.transform_interval(&intersection);
            output_intervals.push(transformed_intersection);

            remaining_interval = last_consequent;
        }
    }

    if last_antecedent != None {
        output_intervals.push(last_antecedent.unwrap());
    }

    if last_consequent != None {
        output_intervals.push(last_consequent.unwrap());
    }

    if remaining_interval != None {
        output_intervals.push(remaining_interval.unwrap());
    }

    output_intervals
}

pub fn apply_transform_layer_to_intervals(layer: &Vec<IntervalTransform>, intervals: &Vec<Interval>) -> Vec<Interval> {
    intervals.iter()
        .map(|interval| apply_transform_layer_to_interval(layer, *interval))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_catches_boundary() {
        let layer = vec![
            IntervalTransform{
                interval: Interval(56, 92),
                addend: 4
            },
            IntervalTransform{
                interval: Interval(93, 96),
                addend: -30
            }
        ];

        let intervals = vec![
            Interval(78, 80),
            Interval(46, 56)
        ];

        let out_intervals = apply_transform_layer_to_intervals(&layer, &intervals);

        assert_eq!(out_intervals.len(), 3);
        assert_eq!(out_intervals[0], Interval(82, 84));
        assert_eq!(out_intervals[1], Interval(60, 60));
        assert_eq!(out_intervals[2], Interval(46, 55));
    }
}