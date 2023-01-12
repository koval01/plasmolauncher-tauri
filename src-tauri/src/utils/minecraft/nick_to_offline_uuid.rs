use uuid::Uuid;

pub fn nick_to_offline_uuid(nick: impl Into<String>) -> Uuid {

    let nick: String = nick.into();
    let format = format!("OfflinePlayer:{nick}");

    let mut bytes = md5::compute(format.as_bytes());

    bytes[6] = bytes[6] & 0x0f | 0x30;
    bytes[8] = bytes[8] & 0x3f | 0x80;

    uuid::Builder::from_md5_bytes(bytes.0).into_uuid()
}