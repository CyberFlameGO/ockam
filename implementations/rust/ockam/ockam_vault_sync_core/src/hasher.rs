use crate::{Vault, VaultRequestMessage, VaultResponseMessage, VaultSyncCoreError};
use ockam_core::Result;
use ockam_node::block_future;
use ockam_vault_core::{Hasher, Secret, SecretAttributes, SmallBuffer};

impl Hasher for Vault {
    fn sha256(&mut self, data: &[u8]) -> Result<[u8; 32]> {
        block_future(&self.ctx().runtime(), async move {
            self.send_message(VaultRequestMessage::Sha256 { data: data.into() })
                .await?;

            let resp = self.receive_message().await?;

            if let VaultResponseMessage::Sha256(s) = resp {
                Ok(s)
            } else {
                Err(VaultSyncCoreError::InvalidResponseType.into())
            }
        })
    }

    fn hkdf_sha256(
        &mut self,
        salt: &Secret,
        info: &[u8],
        ikm: Option<&Secret>,
        output_attributes: SmallBuffer<SecretAttributes>,
    ) -> Result<SmallBuffer<Secret>> {
        block_future(&self.ctx().runtime(), async move {
            self.send_message(VaultRequestMessage::HkdfSha256 {
                salt: salt.clone(),
                info: info.into(),
                ikm: ikm.map(|v| v.clone()),
                output_attributes,
            })
            .await?;

            let resp = self.receive_message().await?;

            if let VaultResponseMessage::HkdfSha256(s) = resp {
                Ok(s)
            } else {
                Err(VaultSyncCoreError::InvalidResponseType.into())
            }
        })
    }
}