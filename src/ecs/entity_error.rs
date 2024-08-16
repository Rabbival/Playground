use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum EntityError {
    EntityNotInQuery(&'static str),
    CommandsCouldntGetEntity(&'static str),
}

impl Display for EntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EntityNotInQuery(error_message) => {
                write!(f, "Error getting entity: {}", error_message)
            }
            Self::CommandsCouldntGetEntity(entity_type) => {
                write!(f, "Commands couldn't get entity: {}", entity_type)
            }
        }
    }
}
