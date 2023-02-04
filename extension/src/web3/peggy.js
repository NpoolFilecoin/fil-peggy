import { idFromAddress, newFromString } from '@glif/filecoin-address'
import Web3 from 'web3'

const abi = JSON.parse(`[
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "_custodyType",
				"type": "string"
			},
			{
				"internalType": "uint8",
				"name": "value",
				"type": "uint8"
			}
		],
		"stateMutability": "nonpayable",
		"type": "constructor"
	},
	{
		"anonymous": false,
		"inputs": [
			{
				"components": [
					{
						"components": [
							{
								"internalType": "bytes",
								"name": "val",
								"type": "bytes"
							},
							{
								"internalType": "bool",
								"name": "neg",
								"type": "bool"
							}
						],
						"internalType": "struct BigInt",
						"name": "raw_byte_power",
						"type": "tuple"
					},
					{
						"internalType": "bool",
						"name": "meets_consensus_minimum",
						"type": "bool"
					}
				],
				"indexed": false,
				"internalType": "struct PowerTypes.MinerRawPowerReturn",
				"name": "ret",
				"type": "tuple"
			}
		],
		"name": "RawPowerReturn",
		"type": "event"
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
				"internalType": "uint64",
				"name": "minerId",
				"type": "uint64"
			},
			{
				"components": [
					{
						"internalType": "address",
						"name": "beneficiary",
						"type": "address"
					},
					{
						"internalType": "uint8",
						"name": "percent",
						"type": "uint8"
					}
				],
				"internalType": "struct Beneficiary.FeeBeneficiary[]",
				"name": "feeBeneficiaries",
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
				"internalType": "struct Beneficiary.RewardBeneficiary[]",
				"name": "rewardBeneficiaries",
				"type": "tuple[]"
			}
		],
		"name": "custodyMiner",
		"outputs": [],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "uint64",
				"name": "minerId",
				"type": "uint64"
			}
		],
		"name": "getMiner",
		"outputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			}
		],
		"stateMutability": "view",
		"type": "function"
	},
	{
		"inputs": [],
		"name": "getMinerIds",
		"outputs": [
			{
				"internalType": "uint64[]",
				"name": "",
				"type": "uint64[]"
			}
		],
		"stateMutability": "view",
		"type": "function"
	},
	{
		"inputs": [],
		"name": "getMiners",
		"outputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			}
		],
		"stateMutability": "view",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "uint64",
				"name": "minerId",
				"type": "uint64"
			}
		],
		"name": "playPeggy",
		"outputs": [],
		"stateMutability": "nonpayable",
		"type": "function"
	}
]`)

const peggyFlag = 'Peggy TZJCLSYW 09231006 .--././--./--./-.--/-/--../.---/-.-./.-../.../-.--/.--/-----/----./..---/...--/.----/-----/-----/-....'

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

export const custodyMiner = (rpc, from, fromPrivKey, contractAddress, minerId, feeBeneficiaries, rewardBeneficiaries) => {
	let idAddress = newFromString(minerId)
	let mid = idFromAddress(idAddress)

	return new Promise((resolve, reject) => {
    let web3 = new Web3(rpc)
    let contract = new web3.eth.Contract(abi, contractAddress)
		web3.eth.accounts.wallet.add(fromPrivKey)
    contract.methods.custodyMiner(mid, feeBeneficiaries, rewardBeneficiaries).send({
			from: from,
			gas: 3000000
		})
    .then(result => {
      resolve(result)
    })
    .catch(error => {
      reject(error)
    })
  })
}

