// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub struct ReportCard {
    pub grade: f32,  // ✅ 仍然是 f32 类型
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        let grade_str = match self.grade {
            g if g >= 4.0 => "A+",
            g if g >= 3.5 => "A",
            g if g >= 3.0 => "B+",
            g if g >= 2.5 => "B",
            g if g >= 2.0 => "C",
            g if g >= 1.0 => "D",
            _ => "F",
        };

        format!(
            "{} ({}) - achieved a grade of {}",
            self.student_name, self.student_age, grade_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,  // ✅ 仍然用数值
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of C"  // ✅ 2.1 对应 "C"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: 4.0,  // ✅ 4.0 对应 "A+"
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
