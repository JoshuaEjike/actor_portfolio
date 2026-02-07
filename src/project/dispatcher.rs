use tokio::sync::mpsc;

use crate::project::{actor::ProjectActor, messages::ProjectMessage};

pub async fn project_dispatcher(actor: &ProjectActor, mut rx: mpsc::Receiver<ProjectMessage>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            ProjectMessage::Create {
                project,
                respond_to,
            } => {
                let _ = respond_to.send(actor.create_project(project).await);
            }
            ProjectMessage::GetSingleProject {
                project_id,
                respond_to,
            } => {
                let _ = respond_to.send(actor.get_single_project(project_id).await);
            }

            ProjectMessage::GetAllProject { respond_to } => {
                let _ = respond_to.send(actor.get_all_project().await);
            }

            ProjectMessage::UpdateProject {
                project,
                respond_to,
            } => {
                let _ = respond_to.send(actor.update_project(project).await);
            }

            ProjectMessage::DeleteProject {
                project_id,
                respond_to,
            } => {
                let _ = respond_to.send(actor.delete_project(project_id).await);
            }
        }
    }
}
