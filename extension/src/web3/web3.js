import Web3 from 'web3'

export const play = () => {
  let web3 = new Web3('wss://wss.hyperspace.node.glif.io/apigw/lotus/rpc/v1')
  let contractAddress = '0x47711030F0674D324c9083D63B193eD24d5DEbE4'
  let abi = JSON.parse(`
    [
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "address",
            "name": "oldOwner",
            "type": "address"
          },
          {
            "indexed": true,
            "internalType": "address",
            "name": "newOwner",
            "type": "address"
          }
        ],
        "name": "OwnerSet",
        "type": "event"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "newOwner",
            "type": "address"
          }
        ],
        "name": "changeOwner",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [],
        "stateMutability": "nonpayable",
        "type": "constructor"
      },
      {
        "inputs": [],
        "name": "getOwner",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      }
    ]
  `)

  let contract = new web3.eth.Contract(abi, contractAddress)
  contract.methods.getOwner().call()
    .then(result => console.log(result))
}
