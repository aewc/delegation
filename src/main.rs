use candid::{Decode, Encode};
use ic_agent::{
    agent::http_transport::ReqwestHttpReplicaV2Transport,
    export::{Delegation, Principal, SignedDelegation},
    identity::DelegationIdentity,
    Agent, Identity,
};
use ic_types::Time;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    // the temp identiy, whose der_encode_public_key need be signed by main identity.
    let mut delegation_identity = DelegationIdentity::from_pem_file(
        "/Users/<username>/.config/dfx/identity/ecdsa01/identity.pem",
        false,
    )
    .expect("Could not read the key pair.");

    // because there are only temp identity now, so the der_encoded_public_key will return the temp identity's public key.
    // after set_delegation(), it will changed.
    let delegate_pubkey = delegation_identity.der_encoded_public_key.clone();
    if args[1] == "pubkey" {
        println!("{:?}", delegate_pubkey);
        return;
    };

    let der_encoded_public_key = hex::decode(&args[1]).unwrap();
    let expiration = args[2].parse::<u64>().unwrap();
    let signature = hex::decode(&args[3]).unwrap();
    let delegation = Delegation::new(
        delegation_identity.der_encoded_public_key.clone(),
        Time::from_nanos_since_unix_epoch(expiration),
    );

    let signed_delegation = SignedDelegation::new(delegation, signature);
    delegation_identity
        .set_delegation(der_encoded_public_key, signed_delegation)
        .unwrap();
    println!("{:}", delegation_identity.sender().unwrap());
    println!("{:}", whoami(delegation_identity).await);
}

async fn whoami(identity: impl Identity + 'static) -> Principal {
    let agent = Agent::builder()
        .with_transport(
            ReqwestHttpReplicaV2Transport::create("https://ic0.app")
                .expect("Failed to create Transport for Agent"),
        )
        .with_identity(identity)
        .build()
        .expect("Failed to build the Agent");

    let response = agent
        .query(
            &Principal::from_text("li5ot-tyaaa-aaaah-aa5ma-cai").unwrap(),
            "whoami",
        )
        .with_arg(&Encode!().unwrap())
        .call()
        .await
        .expect("faile to call");

    Decode!(response.as_slice(), Principal).expect("decode error")
}

#[tokio::test]
async fn test_delegation() {
    use ic_agent::identity::{BasicIdentity, Secp256k1Identity};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn new_delegation(
        second_identity_path: &str,
        second_base: bool,
        main_identity_path: &str,
        main_base: bool,
    ) -> DelegationIdentity {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let mut delegation = DelegationIdentity::from_pem_file(second_identity_path, second_base)
            .expect("Could not read the key pair.");

        delegation
            .set_delegation_from_pem_file(
                main_identity_path,
                main_base,
                Some((since_the_epoch.as_secs() + 30 * 60) * 1_000_000_000),
                None,
            )
            .expect("Could not read the key pair.");
        delegation
    }

    let ecdsa_01 = Secp256k1Identity::from_pem_file(
        "/Users/<username>/.config/dfx/identity/ecdsa01/identity.pem",
    )
    .expect("Could not read the key pair.");
    println!("ecdsa_01: {:}", ecdsa_01.sender().unwrap());

    let ed25519_01 =
        BasicIdentity::from_pem_file("/Users/<username>/.config/dfx/identity/1/identity.pem")
            .expect("Could not read the key pair.");
    println!("ed25519_01: {:}", ed25519_01.sender().unwrap());

    let delegation_01 = new_delegation(
        "/Users/<username>/.config/dfx/identity/ecdsa01/identity.pem",
        false,
        "/Users/<username>/.config/dfx/identity/1/identity.pem",
        true,
    );
    let delegation_01_principal = delegation_01.sender().unwrap();
    println!("delegation_01: {:}", delegation_01_principal);
    assert!(delegation_01_principal == ed25519_01.sender().unwrap());

    let delegation_02 = new_delegation(
        "/Users/<username>/.config/dfx/identity/1/identity.pem",
        true,
        "/Users/<username>/.config/dfx/identity/ecdsa01/identity.pem",
        false,
    );
    let delegation_02_principal = delegation_02.sender().unwrap();
    println!("delegation_02: {:}", delegation_02_principal);
    let whoami_01 = whoami(delegation_01).await;
    let whoami_02 = whoami(delegation_02).await;
    println!("delegation_01 call: {}", whoami_01);
    println!("delegation_02 call: {}", whoami_02);

    assert!(whoami_01 == delegation_01_principal);
    assert!(whoami_02 == delegation_02_principal);
}
