use super::handlers;
use crate::job::Job;
use warp::Filter;

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    jobs_list()
        .or(jobs_view())
        .or(entries_create())
        .or(entries_view())
}

pub fn jobs_list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("jobs")
        .and(warp::get())
        .and_then(handlers::jobs_list)
}

pub fn jobs_view() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("jobs" / String)
        .and(warp::get())
        .and_then(handlers::jobs_view)
}

pub fn entries_create(
    _jobs: Vec<Job>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("jobs" / String)
        .and(warp::post())
        .and_then(handlers::entries_create)
}

pub fn entries_view() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("jobs" / String / u32)
        .and(warp::get())
        .and_then(handlers::entries_view)
}
