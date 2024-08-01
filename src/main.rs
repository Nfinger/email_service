use emailservice::configuration::get_configuration;
use emailservice::issue_delivery_worker::run_worker_until_stopped;
use emailservice::startup::Application;
use emailservice::telemetry::{get_subscriber, init_subscriber};
use tokio::task::JoinError;
use std::fmt::{Debug, Display};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("emailservice".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration.clone()));
    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task => report_exit("Worker", o),
    }
    Ok(())
}

fn report_exit(
    task_name: &str,
    outcome: Result<Result<(), impl Debug + Display>, JoinError>,
) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} task failed to complete",
                task_name
            )
        }
    }
}
