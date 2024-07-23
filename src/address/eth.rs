pub struct EthereumAddressGenerator;
impl super::AddressGenerator for EthereumAddressGenerator {
    fn generate_address(
        &self,
        key: &coins_bip32::xkeys::XPriv,
        index: u32,
    ) -> Result<String, anyhow::Error> {
        let path = crate::constant::add_index(crate::constant::ETH_DERIVATION_PATH, index);
        let derive = key.derive_path(path.as_str())?;
        let signingkey: &coins_bip32::ecdsa::SigningKey = derive.as_ref();

        let address = alloy::signers::utils::secret_key_to_address(signingkey);
        Ok(address.to_string())
    }
}
