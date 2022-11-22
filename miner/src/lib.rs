use colored::Colorize;
use forest_key_management::{
    KeyInfo,
};
use fvm_shared::{
    address::Address,
    message::Message,
    econ::TokenAmount,
    sector::RegisteredPoStProof,
};
use fil_actor_power::{
    CreateMinerParams,
};
use libp2p::{
    PeerId,
};
use fvm_ipld_encoding::{
    RawBytes,
    BytesDe,
    Cbor,
};
use fil_actors_runtime::STORAGE_POWER_ACTOR_ADDR;
use forest_message::signed_message::SignedMessage;
use forest_json::{
    signed_message::json::SignedMessageJson,
    cid::CidJson,
};
use forest_rpc_api::{
    mpool_api,
};
use rpc::RpcEndpoint;
use log::debug;
use reqwest::Error;

pub struct Miner {
    pub owner: Address,
    pub owner_key_info: KeyInfo,
    pub worker: Address,
    pub window_post_proof_type: RegisteredPoStProof,
    pub peer_id: PeerId,
    pub rpc: RpcEndpoint,
    pub miner_id: Option<Address>,
    pub multiaddrs: Option<Vec<BytesDe>>,
}

impl Miner {
    pub async fn create_miner(&self) -> Result<String, Error> {
        let key_info = self.owner_key_info.clone();

        let params = CreateMinerParams {
            owner: self.owner,
            worker: self.worker,
            window_post_proof_type: self.window_post_proof_type,
            peer: self.peer_id.to_bytes(),
            multiaddrs: vec![BytesDe("".as_bytes().to_vec())],
        };
        let params = RawBytes::serialize(params).unwrap();
        let msg = Message {
            version: 0,
            to: STORAGE_POWER_ACTOR_ADDR,
            from: self.owner,
            method_num: 2,
            value: TokenAmount::from_atto(1000),
            sequence: 0,
            params: params,
            gas_fee_cap: TokenAmount::from_nano(10000),
            gas_limit: 301000,
            gas_premium: TokenAmount::from_atto(100),
        };
        let msg_cid = msg.cid().unwrap();
        let sig = forest_key_management::sign(
            *key_info.key_type(),
            key_info.private_key(),
            msg_cid.to_bytes().as_slice(),
            ).unwrap();
        let smsg = SignedMessage::new_from_parts(msg, sig).unwrap();
        debug!("{}", "  Create miner message:".green());
        debug!("{}{:?}", "    CID:".yellow(), msg_cid);
        debug!("{}{:?}", "    CidJson:".yellow(), CidJson(msg_cid));
        debug!("{}{:?}", "    Signed CID:".yellow(), smsg.cid().unwrap());
        debug!("{}{:?}", "    Key type:".yellow(), key_info.key_type());

        let res = self
            .rpc
            .post::<_, CidJson>(mpool_api::MPOOL_PUSH, vec![SignedMessageJson(smsg.clone())])
            .await?;

        debug!("{}", "Create miner: ".yellow());
        debug!("{}{:?}", "  Result: ".yellow(), res);

        Ok(String::default())
    }

    pub fn change_owner(&self) -> Result<String, &str> {
        Ok(String::default())
    }
}
