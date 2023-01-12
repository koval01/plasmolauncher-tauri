use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum PlasmoError {
    #[error("Ошибка при отправке запроса: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Пользователь на найден")]
    UserNotFound,
    #[error("Нет проходки")]
    NotAuthorized
}

impl Serialize for PlasmoError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        Ok(format!("{self}").serialize(serializer)?)
    }
}