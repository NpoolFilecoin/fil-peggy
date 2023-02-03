import axios from 'axios'
import Web3 from 'web3'
import { serializeBigNum } from './sign.js'
import {
  generateAddress,
  KeyTypes,
  privateKeyToAddress
} from './wallet.js'
import {
  ethAddressFromDelegated,
  Network,
  delegatedFromEthAddress,
  newFromString
} from '@glif/filecoin-address'
import { SingleKeyProvider } from '@glif/local-managed-provider'

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

export const importWallet = (hexSecKey) => {
  return privateKeyToAddress(hexSecKey)
}

export const stateAccountKey = (rpc, accountId) => {
  let rpcId = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
  return axios
    .post(rpc, {
      jsonrpc: '2.0',
      method: 'Filecoin.StateAccountKey',
      params: [accountId, []],
      id: rpcId
    }, {
      headers: {
        "Content-Type": "application/json"
      }
    })
}

export const stateLookupId = (rpc, address) => {
  let rpcId = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
  return axios
    .post(rpc, {
      jsonrpc: '2.0',
      method: 'Filecoin.StateLookupID',
      params: [address, []],
      id: rpcId
    }, {
      headers: {
        "Content-Type": "application/json"
      }
    })
}

export const stateGetActor = (rpc, actorId) => {
  let rpcId = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
  return axios
    .post(rpc, {
      jsonrpc: '2.0',
      method: 'Filecoin.StateGetActor',
      params: [actorId, []],
      id: rpcId
    }, {
      headers: {
        "Content-Type": "application/json"
      }
    })
}

export const ethAddress = (filAddr) => {
  return ethAddressFromDelegated(filAddr)
}

export const delegateAddress = (ethAddr) => {
  return delegatedFromEthAddress(ethAddr)
}

export const mpoolGetNonce = (rpc, accountId) => {
  let rpcId = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
  return axios
    .post(rpc, {
      jsonrpc: '2.0',
      method: 'Filecoin.MpoolGetNonce',
      params: [accountId],
      id: rpcId
    }, {
      headers: {
        "Content-Type": "application/json"
      }
    })
}

export const setOwner = (rpc, minerId, curOwnerAddress, curOwnerPrivKey, curOwnerNonce, newOwner) => {
  let addr = newFromString(newOwner)
  // TODO: use cbor encode

  let array = new Uint8Array(addr.bytes.length + 1)
  array[0] = 2 << 5 | addr.bytes.length
  for (let i = 0; i < addr.bytes.length; i++) {
    array[i + 1] = addr.bytes[i]
  }

  // f07113: QwDJNw==

  let params = Buffer.from(array).toString('base64')

  const message = {
    To: minerId,
    From: curOwnerAddress,
    Nonce: curOwnerNonce,
    Value: '0',
    GasLimit: 3000000,
    GasFeeCap: '100000000',
    GasPremium: '100000000',
    Method: 23,
    Params: params
  }

  const provider = new SingleKeyProvider(curOwnerPrivKey, Network.TEST)
  return new Promise((resolve, reject) => {
    provider.sign(curOwnerAddress, message)
      .then((resp) => {
        let rpcId = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
        axios
          .post(rpc, {
            jsonrpc: '2.0',
            method: 'Filecoin.MpoolPush',
            params: [resp],
            id: rpcId
          }, {
            headers: {
              "Content-Type": "application/json"
            }
          })
          .then((resp) => {
            console.log(resp)
            resolve(resp)
          })
          .catch((error) => {
            reject(error)
          })
      })
      .catch((error) => {
        reject(error)
      })
  })
}
