use std::convert::Infallible;
use std::fs;
use tmux_interface::TmuxCommand;
use warp::http::StatusCode;

pub async fn jobs_list() -> Result<impl warp::Reply, Infallible> {
    let _tmux = TmuxCommand::new();

    Ok(StatusCode::OK)
}

pub async fn jobs_view(job: String) -> Result<impl warp::Reply, Infallible> {
    let _tmux = TmuxCommand::new();

    println!("{}", job);
    Ok(StatusCode::OK)
}

pub async fn entries_create(job_name: String) -> Result<impl warp::Reply, Infallible> {
    let tmux = TmuxCommand::new();
    let session_path = get_session_path(job_name.to_string());

    let current_entries = fs::read_dir(session_path.clone())
        .unwrap()
        .map(|path| match path {
            Ok(p) => p.file_name().to_string_lossy().parse::<u32>().unwrap_or(0),
            _ => 0,
        })
        .filter(|num| num > &0)
        .collect::<Vec<u32>>();

    let has_session = match tmux
        .has_session()
        .target_session(job_name.to_string())
        .output()
    {
        Ok(output) => output.success(),
        _ => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if !has_session {
        return Ok(StatusCode::BAD_REQUEST);
    }

    if current_entries.len() >= 10 {
        tmux.kill_window()
            .target_window(format!("{}:{}", job_name.to_string(), current_entries[0]))
            .output()
            .unwrap();
    }

    let next = match current_entries.last() {
        Some(last) => last + 1,
        None => 0,
    };

    let target = tmux
        .new_window()
        .start_directory(session_path.to_str().unwrap())
        .target_window(job_name.to_string())
        .print()
        .output()
        .unwrap()
        .to_string()
        .trim()
        .to_string();

    let job_path = session_path.join(next.to_string());
    fs::create_dir_all(job_path.clone()).unwrap();

    tmux.send_keys()
        .target_pane(target.to_string())
        .key(format!("cd {}", job_path.to_string_lossy()))
        .key("Enter")
        .output()
        .unwrap();

    Ok(StatusCode::CREATED)
}

pub async fn entries_view(job: String, entry: u32) -> Result<impl warp::Reply, Infallible> {
    let _tmux = TmuxCommand::new();

    println!("{} {}", job, entry);

    Ok(warp::reply::json(&1))
}
