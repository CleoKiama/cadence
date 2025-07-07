pub struct NeededMetrics {
    pub metrics: Vec<String>,
}

impl NeededMetrics {
    pub fn new(metrics: Vec<&str>) -> Self {
        NeededMetrics {
            metrics: metrics.into_iter().map(String::from).collect(),
        }
    }
}
