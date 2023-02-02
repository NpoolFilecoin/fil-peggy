import axios from 'axios';
import Web3 from 'web3';
import { serializeBigNum } from './sign.js';
import { generateAddress, KeyTypes } from './wallet.js';

export const checkAlive = (rpc) => {
  return new Promise((resolve, reject) => {
    let web3 = new Web3(rpc);
    web3.eth.getChainId()
      .then(() => {
        resolve()
      })
      .catch(() => {
        reject()
      })
  })
}

export const playFil = () => {
  let bn = serializeBigNum(100000)
  console.log(bn)

  generateAddress(KeyTypes.Secp256k1)
}

export const minerInfo = (rpc, minerId) => {
  let rpcId = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
  return axios
    .post(rpc, {
      jsonrpc: '2.0',
      method: 'Filecoin.StateMinerInfo',
      params: [minerId, []],
      id: rpcId
    }, {
      headers: {
        "Content-Type": "application/json"
      }
    })
}

/*
export const setOwner = (rpc, minerId, curOwnerPrivKey, newOwner) => {
  let web3 = new Web3(rpc);
}
*/
