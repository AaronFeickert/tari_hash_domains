use tari_crypto::hash_domain;

hash_domain!(
    ConfidentialOutputHashDomain,
    "com.tari.dan.confidential_output"
);
hash_domain!(TariDanConsensusHashDomain, "com.tari.dan.consensus");
hash_domain!(TariEngineHashDomain, "com.tari.dan.engine");

// Hash domain used to derive the final AEAD encryption key for encrypted data in UTXOs
hash_domain!(
    TransactionSecureNonceKdfDomain,
    "com.tari.base_layer.core.transactions.secure_nonce_kdf"
);
hash_domain!(
    ValidatorNodeBmtHashDomain,
    "com.tari.base_layer.core.validator_node_mmr"
);
hash_domain!(
    WalletOutputEncryptionKeysDomain,
    "com.tari.base_layer.wallet.output_encryption_keys"
);
