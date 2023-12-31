use db::{RepositoryError, GLOBAL_REPOSITORY};

use crate::db::Channel;

pub mod db;



fn main() {

    let mut repo: std::sync::MutexGuard<'_, db::ChannelRepository> = GLOBAL_REPOSITORY.lock().unwrap();

    let new_channel: Channel = Channel {
        name: "New Channel".to_string(),
        url: "https://example.com/new_channel".to_string(),
        online: true,
    };

    // Exemplo: Adionando um canal
    match repo.add(new_channel) {
        Ok(_) => println!("Canal adicionado com sucesso."),
        Err(err) => {
            match err {
                RepositoryError::AlreadyExists => println!("O canal já existe."),
                _ => println!("Erro ao adicionar o canal: {:?}", err),
            }
        }
    }

    // Exemplo: Recupere um canal pelo nome
    if let Some(channel) = repo.get_by_name("New Channel") {
        println!("Canal recuperado pelo nome: {:?}", channel);
    } else {
        println!("Canal não encontrado pelo nome.");
    }

    // Exemplo: Exclua um canal pelo nome
    match repo.delete_by_name("New Channel") {
        Ok(_) => println!("Canal excluído com sucesso."),
        Err(err) => {
            match err {
                RepositoryError::NotFound => println!("Canal não encontrado."),
                _ => println!("Erro ao excluir o canal: {:?}", err),
            }
        }
    }
    
}