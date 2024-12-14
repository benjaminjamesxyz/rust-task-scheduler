#[allow(dead_code)]
#[derive(Debug)]
pub enum SchedulerError {
    TaskBuildError(&'static str),
    SchedulerRunError(&'static str),
}
