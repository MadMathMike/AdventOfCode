/// Represents a closed interval (i.e., includes its endpoints)
/// Because this is a tuple, the start of the interval is accessed
/// with ".0", and the end of the interval is accessed with ".1"
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval(pub i64, pub i64);

pub struct IntersectResult{
    pub antecedent: Option<Interval>,
    pub intersection: Option<Interval>,
    pub consequent: Option<Interval>
}

impl Interval {
    pub fn contains(self: &Self, point: i64) -> bool {
        self.0 <= point && point <= self.1
    }
    
    pub fn intersect_with(self: &Self, other: &Interval) -> IntersectResult {
        // self is before other
        if self.1 < other.0 {
            return IntersectResult { 
                antecedent: Some(*self), 
                intersection: None, 
                consequent: None 
            };
        }

        // self is after other
        if self.0 > other.1 {
            return IntersectResult { 
                antecedent: None, 
                intersection: None, 
                consequent: Some(*self) 
            };
        }

        let antecedent = 
            if self.0 < other.0 { Some(Interval(self.0, other.0 - 1)) }
            else { None };

        let consequent = 
            if self.1 > other.1 { Some(Interval(other.1 + 1, self.1)) }
            else { None };

        let intersection_start = if self.contains(other.0) 
            { other.0 }
            else { self.0};

        let intersection_end = if self.contains(other.1)
            { other.1 }
            else { self. 1};

        let intersection = Some(Interval(intersection_start, intersection_end));

        IntersectResult { antecedent, intersection, consequent }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_point_returns_true() {
        let interval = Interval(-1, 1);
        
        assert!(interval.contains(-1));
        assert!(interval.contains(0));
        assert!(interval.contains(1));
    }

    #[test]
    fn contains_point_returns_false() {
        let interval = Interval(-1, 1);
        
        assert!(!interval.contains(-2));
        assert!(!interval.contains(2));
    }

    #[test]
    fn trivial_a_before_trivial_b() {
        let a = Interval(1, 1);
        let b = Interval (2, 2);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, Some(Interval(1, 1)));
        assert_eq!(intersect_result.intersection, None);
        assert_eq!(intersect_result.consequent, None);
    }

    #[test]
    fn trivial_a_on_trivial_b() {
        let a = Interval(2, 2);
        let b = Interval (2, 2);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, None);
        assert_eq!(intersect_result.intersection, Some(Interval(2, 2)));
        assert_eq!(intersect_result.consequent, None);
    }

    #[test]
    fn trivial_a_after_trivial_b() {
        let a = Interval(3, 3);
        let b = Interval (2, 2);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, None);
        assert_eq!(intersect_result.intersection, None);
        assert_eq!(intersect_result.consequent, Some(Interval(3, 3)));
    }

    #[test]
    fn trivial_a_before_b(){
        let a = Interval(1, 1);
        let b = Interval(2, 5);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, Some(Interval(1, 1)));
        assert_eq!(intersect_result.intersection, None);
        assert_eq!(intersect_result.consequent, None);
    }

    #[test]
    fn trivial_a_on_start_of_b(){
        let a = Interval(2, 2);
        let b = Interval(2, 5);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, None);
        assert_eq!(intersect_result.intersection, Some(Interval(2, 2)));
        assert_eq!(intersect_result.consequent, None);
    }

    #[test]
    fn trivial_a_in_b(){
        let a = Interval(3, 3);
        let b = Interval(2, 5);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, None);
        assert_eq!(intersect_result.intersection, Some(Interval(3, 3)));
        assert_eq!(intersect_result.consequent, None);
    }

    #[test]
    fn trivial_a_on_end_of_b(){
        let a = Interval(5, 5);
        let b = Interval(2, 5);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, None);
        assert_eq!(intersect_result.intersection, Some(Interval(5, 5)));
        assert_eq!(intersect_result.consequent, None);
    }

    #[test]
    fn trivial_a_after_b(){
        let a = Interval(6, 6);
        let b = Interval(2, 5);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, None);
        assert_eq!(intersect_result.intersection, None);
        assert_eq!(intersect_result.consequent, Some(Interval(6, 6)));
    }

    #[test]
    fn a_before_trivial_b(){
        let a = Interval(1, 4);
        let b = Interval(5, 5);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, Some(Interval(1, 4)));
        assert_eq!(intersect_result.intersection, None);
        assert_eq!(intersect_result.consequent, None);
    }

    #[test]
    fn end_of_a_on_trivial_b(){
        let a = Interval(1, 5);
        let b = Interval(5, 5);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, Some(Interval(1, 4)));
        assert_eq!(intersect_result.intersection, Some(Interval(5, 5)));
        assert_eq!(intersect_result.consequent, None);
    }

    #[test]
    fn a_contains_trivial_b(){
        let a = Interval(1, 6);
        let b = Interval(5, 5);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, Some(Interval(1, 4)));
        assert_eq!(intersect_result.intersection, Some(Interval(5, 5)));
        assert_eq!(intersect_result.consequent, Some(Interval(6, 6)));
    }

    #[test]
    fn start_of_a_on_trivial_b(){
        let a = Interval(5, 6);
        let b = Interval(5, 5);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, None);
        assert_eq!(intersect_result.intersection, Some(Interval(5, 5)));
        assert_eq!(intersect_result.consequent, Some(Interval(6, 6)));
    }
    
    #[test]
    fn a_after_trivial_b(){
        let a = Interval(6, 7);
        let b = Interval(5, 5);

        let intersect_result = a.intersect_with(&b);

        assert_eq!(intersect_result.antecedent, None);
        assert_eq!(intersect_result.intersection, None);
        assert_eq!(intersect_result.consequent, Some(Interval(6, 7)));
    }

    #[test]
    fn intersect_with_returns_only_antecedent() {
        let a = Interval(1, 3);
        let b = Interval(4, 6);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, Some(Interval(1, 3)));
        assert_eq!(result.intersection, None);
        assert_eq!(result.consequent, None);
    }

    #[test]
    fn intersect_with_returns_antecedent_and_intersection_at_first_boundary() {
        let a = Interval(1, 4);
        let b = Interval(4, 6);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, Some(Interval(1, 3)));
        assert_eq!(result.intersection, Some(Interval(4, 4)));
        assert_eq!(result.consequent, None);
    }

    #[test]
    fn intersect_with_returns_antecedent_and_intersection_at_last_boundary() {
        let a = Interval(1, 6);
        let b = Interval(4, 6);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, Some(Interval(1, 3)));
        assert_eq!(result.intersection, Some(Interval(4, 6)));
        assert_eq!(result.consequent, None);
    }

    #[test]
    fn intersect_with_returns_only_consequent() {
        let a = Interval(4, 6);
        let b = Interval(1, 3);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, None);
        assert_eq!(result.intersection, None);
        assert_eq!(result.consequent, Some(Interval(4, 6)));
    }

    #[test]
    fn intersect_with_returns_intersection_and_consequent_at_first_boundary() {
        let a = Interval(1, 6);
        let b = Interval(1, 3);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, None);
        assert_eq!(result.intersection, Some(Interval(1, 3)));
        assert_eq!(result.consequent, Some(Interval(4, 6)));
    }

    #[test]
    fn intersect_with_returns_intersection_and_consequent_at_last_boundary() {
        let a = Interval(3, 6);
        let b = Interval(1, 3);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, None);
        assert_eq!(result.intersection, Some(Interval(3, 3)));
        assert_eq!(result.consequent, Some(Interval(4, 6)));
    }

    #[test]
    fn intersect_with_returns_antecedent_b_and_consequent_when_a_contains_b(){
        let a = Interval(1, 6);
        let b = Interval(3, 4);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, Some(Interval(1, 2)));
        assert_eq!(result.intersection, Some(Interval(3, 4)));
        assert_eq!(result.consequent, Some(Interval(5, 6)));
    }

    #[test]
    fn intersect_with_returns_antecedent_and_intersection_when_a_contains_start_of_b(){
        let a = Interval(1, 6);
        let b = Interval(5, 8);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, Some(Interval(1, 4)));
        assert_eq!(result.intersection, Some(Interval(5, 6)));
        assert_eq!(result.consequent, None);
    }

    #[test]
    fn intersect_with_returns_antecedent_and_intersection_when_a_ends_at_start_of_b(){
        let a = Interval(1, 6);
        let b = Interval(6, 8);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, Some(Interval(1, 5)));
        assert_eq!(result.intersection, Some(Interval(6, 6)));
        assert_eq!(result.consequent, None);
    }

    #[test]
    fn intersect_with_returns_intersection_and_consequent_when_a_contains_end_of_b(){
        let a = Interval(7, 10);
        let b = Interval(5, 8);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, None);
        assert_eq!(result.intersection, Some(Interval(7, 8)));
        assert_eq!(result.consequent, Some(Interval(9, 10)));
    }

    #[test]
    fn intersect_with_returns_intersection_and_consequent_when_a_starts_at_end_of_b(){
        let a = Interval(8, 10);
        let b = Interval(6, 8);

        let result = a.intersect_with(&b);

        assert_eq!(result.antecedent, None);
        assert_eq!(result.intersection, Some(Interval(8, 8)));
        assert_eq!(result.consequent, Some(Interval(9, 10)));
    }
}