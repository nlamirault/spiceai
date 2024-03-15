use std::time::Instant;

/// `measure_scope_ms!` measures the time for which the designated scope lives.
///
/// ## Usage
///   - Example, measure a function's whole duration:
///   ```
///   fn my_function() {
///     measure_scope_ms!("process_data");
///     sleep(Duration::from_secs(1))
///   
///   } // 'process_data' measures until the end of the function scope (via implementing `Drop`).
///   ```
///
///   - Example, measure a specific scope
///   ```
///   fn my_function() {
///     // Some work
///     sleep(Duration::from_secs(1))
///     {
///         // Some work we don't want to measure
///         let x = 1+2;
///
///         // Some work we want to measure
///         measure_scope_ms!("process_data");
///         let y = 2*3;
///         sleep(Duration::from_secs(1))
///     } // 'process_data' duration ends here.
///   }
///   ```
///   - **Example**: Add properties to the measurement (key `&str`, value `ToString`)
///   ```
///   fn my_function(x: int, y: String) {
///     measure_scope_ms!("process_data", "x" => x, "y" => y);
///   }
///   ```
/// ## Parameters
///
/// - `$name:expr` — A string literal representing the name of the scope being measured.
/// - `$key:expr => $value:expr` — Optional key-value pairs provided as additional metadata
///   for the timing measurement.
///
/// ```
#[macro_export]
macro_rules! measure_scope_ms {
    ($name:expr, $($key:expr => $value:expr),+ $(,)?) => {
        let args = vec![$(($key, $value.to_string())),+];
        let _ = $crate::timing::TimeMeasurement::new($name, args);
    };
    ($name:expr) => {
        let _ = $crate::timing::TimeMeasurement::new($name, vec![]);
    };
}

pub struct TimeMeasurement {
    start: Instant,
    metric_name: &'static str,
    labels: Vec<(&'static str, String)>,
}

impl TimeMeasurement {
    #[must_use]
    pub fn new(metric_name: &'static str, labels: Vec<(&'static str, String)>) -> Self {
        Self {
            start: Instant::now(),
            metric_name,
            labels,
        }
    }
}

impl Drop for TimeMeasurement {
    fn drop(&mut self) {
        metrics::histogram!(self.metric_name, &self.labels)
            .record(1000_f64 * self.start.elapsed().as_secs_f64());
    }
}