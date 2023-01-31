import { createStore } from 'vuex'

import { CustodyTypes, ActivityTypes } from '../const/contract_types'

export const store = createStore({
  state () {
    return {
      ToolbarTitle: 'Home',
      ToolbarShowAddBtn: false,
      ShowFooterHelp: true,
      NetworkName: 'Hyperspace',
      NetworkRpcEndpoint: 'https://localhost:1234/v1',
      NetworkRpcConnected: true,
      Contracts: [
        {
          Title: 'f078235',
          Subtitle: 't410fafsypcszjsrfkm4k36snjbcj62jef24pn7ysykq',
          CustodyType: CustodyTypes.FixedIncome,
          Value: 30,
          Miners: [
            "f0182365",
            "f0135689",
            "f0135699",
            "f0135619",
            "f0135629",
            "f0135639"
          ],
          RawPowerBytes: 179895522222356n,
          AdjPowerBytes: 12456789465642323n,
          EstimateDailyReward: 245.9,
          TotalAttoFilAmount: 832571312649679456n,
          BalanceAttoFilAmount: 356823546458798n,
          TotalRewardAttoFilAmount: 123465789789765465132n,
          AvgPowerLostIntervalSeconds: 1234567896544,
          AvgPowerLostRecoverSeconds: 1245,
          LastMonthRawPowerBytesIncrement: 454132432132n,
          LastMonthAdjPowerBytesIncrement: 454132432132n,
          LastHalfYearRawPowerBytesIncrement: 12465789453413n,
          LastHalfYearAdjPowerBytesIncrement: 12465789453413n,
          LastYearPowerRawBytesIncrement: 1234657987n,
          LastYearPowerAdjBytesIncrement: 1234657987n,
          TotalSlashPenaltyAttoFilAmount: 12456746789465432n,
          Activities: [
            {
              Activity: ActivityTypes.Withdraw,
              Target: 't410fafsypcszjsrfkm4k36snjbcj62jef24pn7ysykq',
              Timestamp: 1256456465,
              AttoFilAmount: 123456456452265613221n
            }, {
              Activity: ActivityTypes.Deposit,
              Target: 't410fafsypcszjsrfkm4k36snjbcj62jef24pn7ysykq',
              Timestamp: 1236456465,
              AttoFilAmount: 1234564564523454879n
            }, {
              Activity: ActivityTypes.Redeem,
              Target: 't410fafsypcszjsrfkm4k36snjbcj62jef24pn7ysykq',
              Timestamp: 1216456465,
              AttoFilAmount: 123456456798423465132n
            }, {
              Activity: ActivityTypes.WithdrawMinerBalance,
              Target: 'f0178352',
              Timestamp: 1206456465,
              AttoFilAmount: 12345645640265132456n
            }
          ]
        }
      ],
      NetworkRawPowerBytes: 124597984564321321323131232n,
      NetworkAdjPowerBytes: 1204597984564321321323131232n
    }
  },
  mutations: {
    setToolbarShowAddBtn (state, show) {
      state.ToolbarShowAddBtn = show
    },
    setToolbarTitle (state, title) {
      state.ToolbarTitle = title
    },
    setContracts (state, contracts) {
      state.Contracts = contracts
    },
    setShowFooterHelp (state, show) {
      state.ShowFooterHelp = show
    },
    deleteContractById (state, id) {
      let index = state.Contracts.findIndex(contract => contract.Title === id)
      if (index < 0) {
        return
      }
      state.Contracts.splice(index, 1)
    }
  },
  getters: {
    toolbarShowAddBtn (state) {
      return state.ToolbarShowAddBtn
    },
    toolbarTitle (state) {
      return state.ToolbarTitle
    },
    contracts (state) {
      return state.Contracts
    },
    contractById: (state) => (id) => {
      return state.Contracts.find(contract => contract.Title === id)
    },
    networkRawPowerBytes (state) {
      return state.NetworkRawPowerBytes
    },
    networkAdjPowerBytes (state) {
      return state.NetworkAdjPowerBytes
    },
    showFooterHelp (state) {
      return state.ShowFooterHelp
    }
  }
})