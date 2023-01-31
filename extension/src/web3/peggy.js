import Web3 from 'web3'

const abi = JSON.parse(`[
	{
		"inputs": [],
		"stateMutability": "nonpayable",
		"type": "constructor"
	},
	{
		"inputs": [],
		"name": "checkPeggy",
		"outputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			}
		],
		"stateMutability": "pure",
		"type": "function"
	},
	{
		"inputs": [],
		"name": "creator",
		"outputs": [
			{
				"internalType": "address",
				"name": "",
				"type": "address"
			}
		],
		"stateMutability": "view",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "address",
				"name": "minerId",
				"type": "address"
			},
			{
				"internalType": "bytes",
				"name": "powerActorState",
				"type": "bytes"
			},
			{
				"components": [
					{
						"internalType": "address",
						"name": "beneficiary",
						"type": "address"
					},
					{
						"internalType": "uint32",
						"name": "percent",
						"type": "uint32"
					}
				],
				"internalType": "struct Beneficiary.PercentBeneficiary[]",
				"name": "percentBeneficiaries",
				"type": "tuple[]"
			},
			{
				"components": [
					{
						"internalType": "address",
						"name": "beneficiary",
						"type": "address"
					},
					{
						"internalType": "uint256",
						"name": "amount",
						"type": "uint256"
					}
				],
				"internalType": "struct Beneficiary.AmountBeneficiary[]",
				"name": "amountBeneficiaries",
				"type": "tuple[]"
			}
		],
		"name": "custodyMiner",
		"outputs": [
			{
				"internalType": "address",
				"name": "",
				"type": "address"
			}
		],
		"stateMutability": "nonpayable",
		"type": "function"
	}
]`)

const peggyFlag = 'Peggy TZJCLSYW 09231006 .--.----.--....--..--'

export const checkPeggy = (rpc, contractAddress) => {
  return new Promise((resolve, reject) => {
    let web3 = new Web3(rpc)
    let contract = new web3.eth.Contract(abi, contractAddress)
    contract.methods.checkPeggy().call()
    .then(result => {
      if (result !== peggyFlag) {
        reject()
        return
      }
      resolve()
    })
    .catch(error => {
      reject(error)
    })
  })
}
