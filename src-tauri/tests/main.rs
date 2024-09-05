use std::sync::Arc;

use bip39::Mnemonic;
use bitcoin::hex::{Case, DisplayHex};
use dev_wallet::{
    account::AccountBuilder,
    path_builder::PathBuilder,
    storage::{self},
    utils::{decrypt, encrypt},
    wallet::WalletBuilder,
};
use rand::RngCore;
use rand_core::{self, OsRng};

fn mnemonic_helper() -> Mnemonic {
    let mut entropy = [0u8; 32];
    let mut rng = OsRng;
    rng.fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
    mnemonic
}

#[tokio::test]
async fn create_wallet() {
    let connection = storage::DbFacade::new(Some("sqlite::memory:")).await;
    connection.migrate().await;
    let db = connection.pool;
    let mnemonic = mnemonic_helper();
    let mut wallet = WalletBuilder::new(&mnemonic.to_string());
    let wallet_name = "Main wallet";
    wallet.name(&wallet_name);

    let wallet = wallet.build();

    wallet.save(&db).await.unwrap();
}

#[tokio::test]
async fn load_wallet() {
    let conn_facade = storage::DbFacade::new(Some("sqlite::memory:")).await;
    conn_facade.migrate().await;

    let mnemonic = mnemonic_helper();

    let mut wallet = WalletBuilder::new(&mnemonic.to_string());
    let wallet_name = "Main wallet";
    let wallet_pass = "PassPhrase";
    wallet.passphrase(&wallet_pass);
    wallet.name(&wallet_name);
    wallet.build().save(&conn_facade.pool).await.unwrap();

    let mut wallet = WalletBuilder::from_existing("Main wallet");
    let wallet = wallet.authenticate(&wallet_pass, &conn_facade.pool).await;

    assert_eq!(wallet.unwrap().name, "Main wallet")
}

#[test]
fn can_build_bip32_path() {
    let path = PathBuilder::new();
    let path = path.build().to_string();

    assert_eq!(path, "49'/0'/0'/0/0");
}
#[test]
fn can_build_account() {
    // Account
    let mnemonic = mnemonic_helper();
    let seed = mnemonic.to_seed("passphrase");
    let mut account_builder = AccountBuilder::new();

    account_builder.seed(&seed.to_hex_string(Case::Lower));

    let account = account_builder.build().unwrap();
    println!("Address: {}", account.address);
    println!("Address length: {}", account.address.len());

    dbg!(&account.address);
}

#[tokio::test]
async fn can_store_accounts_for_wallet() {
    let conn_facade = storage::DbFacade::new(Some("sqlite::memory:")).await;
    conn_facade.migrate().await;

    let mnemonic = mnemonic_helper();

    let mut wallet = WalletBuilder::new(&mnemonic.to_string());
    let wallet_name = "Main wallet";
    let wallet_pass = "PassPhrase";
    wallet.passphrase(&wallet_pass);
    wallet.name(&wallet_name);
    wallet.build().save(&conn_facade.pool).await.unwrap();
    let mut wallet = WalletBuilder::from_existing(&wallet_name);

    let mut wallet = wallet
        .authenticate(&wallet_pass, &conn_facade.pool)
        .await
        .unwrap();

    let account_builder = wallet.create_account();
    let account = account_builder.build().unwrap();
    account.save(&conn_facade.pool).await;

    // Now lets try to load the wallet + accounts
    let wallet = wallet
        .authenticate(&wallet_pass, &conn_facade.pool)
        .await
        .unwrap();
    let accounts_ref = Arc::clone(&wallet.accounts);
    let accounts_ref = accounts_ref.lock().await;
    let accounts_len = accounts_ref.len();
    assert!(accounts_len > 0);
}

#[test]
fn can_encrypt_and_decrypt_data() {
    let key = [1u8; 32];

    let text = b"Hello world";

    let encrypted_data = encrypt(&key, text);

    let decrypted = decrypt(&key, &encrypted_data);
    println!("{}", &decrypted.to_hex_string(Case::Lower));
    let decrypted = decrypted.to_hex_string(Case::Lower);
    let text = text.to_hex_string(Case::Lower);
    assert_eq!(text, decrypted);
}
