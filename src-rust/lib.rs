use std::collections::HashMap;

pub struct InterrogateResults {
    pub total: usize,
    pub covered: usize,
    pub missing: usize,
}

impl InterrogateResults {
    pub fn new(total: usize, covered: usize, missing: usize) -> Self {
        Self {
            total,
            covered,
            missing,
        }
    }

    pub fn perc_covered(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.covered as f64 / self.total as f64) * 100.0
        }
    }
}

pub fn analyze_docstring_coverage(source_code: &str) -> InterrogateResults {
    let mut total = 0;
    let mut covered = 0;
    let mut missing = 0;

    for line in source_code.lines() {
        if line.trim().starts_with("def ") || line.trim().starts_with("class ") {
            total += 1;
            if line.contains("\"\"\"") || line.contains("'''") {
                covered += 1;
            } else {
                missing += 1;
            }
        }
    }

    InterrogateResults::new(total, covered, missing)
}

pub fn generate_badge(result: &InterrogateResults) -> String {
    let color = match result.perc_covered() {
        x if x >= 95.0 => "brightgreen",
        x if x >= 90.0 => "green",
        x if x >= 75.0 => "yellowgreen",
        x if x >= 60.0 => "yellow",
        x if x >= 40.0 => "orange",
        _ => "red",
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"20\">
            <rect width=\"100\" height=\"20\" fill=\"#555\" />
            <rect x=\"37\" width=\"63\" height=\"20\" fill=\"{}\" />
            <text x=\"19\" y=\"14\" fill=\"#fff\" font-family=\"Verdana\" font-size=\"11\">docs</text>
            <text x=\"50\" y=\"14\" fill=\"#fff\" font-family=\"Verdana\" font-size=\"11\">{:.1}%</text>
        </svg>",
        color,
        result.perc_covered()
    )
}
