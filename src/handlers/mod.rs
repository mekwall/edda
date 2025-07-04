mod doc;
mod query;
mod state;
mod sync;
mod system;
mod task;

pub use doc::handle_doc_commands;
pub use query::handle_query_command;
pub use state::handle_state_commands;
pub use sync::{handle_github_sync_commands, handle_sync_commands};
pub use system::handle_system_commands;
pub use task::handle_task_commands;
