

use super::nick_to_offline_uuid::nick_to_offline_uuid;

pub fn offline_nick_to_skin_name(nick: impl Into<String>) -> String {

    let uuid = nick_to_offline_uuid(nick);

    let (most, least) = uuid.as_u64_pair();

    let hilo = i64::from_be_bytes(most.to_be_bytes()) ^ i64::from_be_bytes(least.to_be_bytes());

    let hash_code = (hilo>>32) as i32 ^ hilo as i32;

    let length = (SKINS.len() * 2) as i32;

    let mut modulus = hash_code % length;

    if (modulus^length) < 0 && modulus != 0 {
        modulus += length;
    }

    SKINS.iter()
        .cycle()
        .skip(modulus as usize)
        .next()
        .unwrap_or(&"steve")
        .to_owned()
        .to_string()
}

const SKINS: &'static [&'static str] = &[
    "alex",
    "ari",
    "efe",
    "kai",
    "makena",
    "noor",
    "steve",
    "sunny",
    "zuri"
];

// #[tokio::test]
// async fn test() -> Result<()> {
//     dbg!(offline_nick_to_skin("gdgdfgergegsdgsg").await);
//     Ok(())
// }