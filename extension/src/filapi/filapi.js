import axios from 'axios'
import Web3 from 'web3'
import { serializeBigNum } from './sign.js'
import { generateAddress, KeyTypes, privateKeyToAddress } from './wallet.js'

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

  let address = generateAddress(KeyTypes.Secp256k1)
  console.log(address)

  let hex = '7b2254797065223a22736563703235366b31222c22507269766174654b6579223a226976316e7a4a4f743566646b657847543655336735787263634c4c436d793163427341416e3247694c566f3d227d'
  address = privateKeyToAddress(hex)
  console.log(address)
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
