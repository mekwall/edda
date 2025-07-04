use crate::cli::DocCommands;
use crate::core::EddaResult;

pub async fn handle_doc_commands(_subcommand: DocCommands) -> EddaResult<()> {
    // TODO: Implement document commands
    println!("Document commands not yet implemented");
    Ok(())
}
