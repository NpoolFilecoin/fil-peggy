export const abi = JSON.parse(`[
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
	},
	{
		"inputs": [],
		"stateMutability": "nonpayable",
		"type": "constructor"
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
	}
]`)