mod filters;
mod handlers;
mod job;

use job::Job;
use std::env;
use std::path::Path;
use tokio::net::TcpListener;
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use warp::Filter;

#[tokio::main]
async fn main() {
    let listener_v4 = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let listener_v6 = TcpListener::bind("[::1]:8080").await.unwrap();

    let incoming_connections =
        TcpListenerStream::new(listener_v4).merge(TcpListenerStream::new(listener_v6));

    // TODO(jared): parse these from a spec file (e.g. a Nix flake's `hydraJobs`).
    let jobs = (vec!["job1"])
        .iter()
        .map(|job_name| {
            let runtime_directory = env::var("RUNTIME_DIRECTORY").unwrap_or("/tmp".to_string());
            let session_path = Path::new(&runtime_directory).join(job_name.to_string());
            Job {
                name: job_name.to_string(),
                session_path,
                command: vec![],
            }
        })
        .collect::<Vec<Job>>();

    for job in jobs {}

    let api = filters::api();
    let routes = api.with(warp::log("builder"));

    println!("listening on http://localhost:8080");
    warp::serve(routes).run_incoming(incoming_connections).await;
}
