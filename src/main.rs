mod config;
mod event;
mod identity;

#[cfg(not(target_arch = "wasm32"))]
mod relay;

mod web;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use event::event::{Event, Payload};
    use event::kinds;
    use event::tags;
    use identity::Identity;
    use web::server;

    server();

    const IDENTITY_PATH: &str = ".murm_id";

    let identity = match Identity::load(IDENTITY_PATH) {
        Ok(identity) => {
            println!("identity loaded from {IDENTITY_PATH}");
            identity
        }
        Err(_) => {
            let identity = Identity::generate();
            identity
                .save(IDENTITY_PATH)
                .expect("failed to persist identity");
            println!("new identity saved to {IDENTITY_PATH}");
            identity
        }
    };
    println!("pubkey: {}", identity.pubkey_hex());

    let event = Event::new(
        identity.pubkey_hex(),
        kinds::POST,
        vec![tags::topic("murm"), tags::lang("pt")],
        "# Hello murm".to_string(),
    );

    println!("canonical payload: {}", event.canonical_payload());
    println!("id: {}", event.id());

    let payload = event.sign(&identity);
    println!("--- signed payload ---");
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());

    let valid = payload.verify();
    println!("id + signature valid: {valid}");
    assert!(valid, "signed event must be valid");

    let tampered = Payload { content: "tampered".to_string(), ..payload };
    println!("tampered event valid: {}", tampered.verify());
}

#[cfg(target_arch = "wasm32")]
fn main() {
    web::server();
}
