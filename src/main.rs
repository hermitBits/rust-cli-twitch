use db::RepositoryError;

use crate::db::{Channel, ChannelRepository};

pub mod db;



fn main() {
    match ChannelRepository::new() {
        Ok(mut channel_repo) => {
            // Agora você pode usar channel_repo para realizar operações no repositório

            // Exemplo: Adicione um novo canal
            let new_channel = Channel {
                name: "New Channel".to_string(),
                url: "https://example.com/new_channel".to_string(),
                online: true,
            };

            match channel_repo.add(new_channel) {
                Ok(_) => println!("Canal adicionado com sucesso."),
                Err(err) => {
                    match err {
                        RepositoryError::AlreadyExists => println!("O canal já existe."),
                        _ => println!("Erro ao adicionar o canal: {:?}", err),
                    }
                }
            }

            // Exemplo: Recupere um canal pelo nome
            if let Some(channel) = channel_repo.get_by_name("New Channel") {
                println!("Canal recuperado pelo nome: {:?}", channel);
            } else {
                println!("Canal não encontrado pelo nome.");
            }

            // Exemplo: Exclua um canal pelo nome
            match channel_repo.delete_by_name("New Channel") {
                Ok(_) => println!("Canal excluído com sucesso."),
                Err(err) => {
                    match err {
                        RepositoryError::NotFound => println!("Canal não encontrado."),
                        _ => println!("Erro ao excluir o canal: {:?}", err),
                    }
                }
            }
        }
        Err(err) => {
            println!("Erro ao criar o repositório: {:?}", err);
        }
    }
    
}