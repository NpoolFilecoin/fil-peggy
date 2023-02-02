<template>
  <div v-if='curTab === "contracts"' :class='["page", addingContract ? "blur" : ""]'>
    <div
      class='inner-item'
      v-for='(contract, index) in contracts'
      :key='index'
    >
      <contractItem
        :title='contract.Title'
        :icon='contract.Icon'
        :subtitle='contract.Subtitle'
        :custody-type='contract.CustodyType'
        :value='contract.Value'
        :miners='contract.Miners'
        :raw-power-bytes='contract.RawPowerBytes'
        :adj-power-bytes='contract.AdjPowerBytes'
        :estimate-daily-reward='contract.EstimateDailyReward'
        v-on:click='() => onContractClick(contract)'
      />
    </div>
  </div>
  <div v-if='curTab === "miners"' :class='["page", addingMiner ? "blur" : ""]'>
    <div
      class='inner-item'
      v-for='(miner, index) in miners'
      :key='index'
    >
      <minerItem
        :title='miner.MinerId'
        :subtitle='miner.CustodyContract'
        :raw-power-bytes='miner.RawPowerBytes'
        :adj-power-bytes='miner.AdjPowerBytes'
        :estimate-daily-reward='miner.EstimateDailyReward'
        v-on:click='() => onMinerClick(miner)'
      />
    </div>
  </div>
  <div class='tabs'>
    <div :class='["tab", curTab === "contracts" ? "tab-selected" : ""]' v-on:click='onContractsClick'>
      <img class='icon' :src='contractsIcon' />
      <div class='text'>{{ contractsText }}</div>
    </div>
    <div :class='["tab", curTab === "miners" ? "tab-selected" : ""]' v-on:click='onMinersClick'>
      <img class='icon' :src='minersIcon' />
      <div class='text'>{{ minersText }}</div>
    </div>
  </div>
  <div v-if='addingContract' class='popup'>
    <div class='title'>Add My Contract</div>
    <div class='area'>
      <div>Code ID</div>
      <div>
        <input type='text' placeholder='Input code id of contract' v-model='contractCodeId'>
      </div>
    </div>
    <div class='area'>
      <div>Actor ID</div>
      <div>
        <input type='text' placeholder='Input actor id of contract' v-model='contractActorId'>
      </div>
    </div>
    <div class='area'>
      <div>Actor Robust Address</div>
      <div>
        <input type='text' placeholder='Input robust address of contract' v-model='contractRobustAddress'>
      </div>
    </div>
    <div class='btns'>
      <button class='btn' v-on:click='onAddContractClick'>Add</button>
      <button class='btn' v-on:click='onCancelContractClick'>Cancel</button>
      <button class='btn' v-on:click='onVerifyContractClick'>Verify</button>
    </div>
    <div class='tips' v-on:click='onDeployClick'>
      <span>Don't have a Peggy Actor ? </span>
      <span class='deploy'>Deploy</span>
    </div>
  </div>
  <div v-if='addingMiner' class='popup'>
    <div class='title'>Add My Miner</div>
    <div class='area'>
      <div>Miner ID</div>
      <div>
        <input type='text' placeholder='Input miner id of contract' v-model='minerId'>
      </div>
    </div>
    <div class='btns'>
      <button class='btn' v-on:click='onAddMinerClick'>Add</button>
      <button class='btn' v-on:click='onCancelMinerClick'>Cancel</button>
      <button class='btn' v-on:click='onVerifyMinerClick'>Verify</button>
    </div>
  </div>
</template>

<script>
import contractItem from '../components/contractitem.vue'
import minerItem from '../components/mineritem.vue'
import { GlobalEvents } from '../const/global_events'
import { evbus } from '../evbus/event_bus'
import { LocalStorageKeys } from '../const/store_keys'
import { checkPeggy } from '../web3/peggy'
import { minerInfo } from '../filapi/filapi'

export default {
  name: 'custodyContracts',
  components: {
    contractItem,
    minerItem
  },
  data () {
    return {
      contractsIcon: '../assets/icons/contracts-40x40.png',
      contractsText: 'My Contracts',
      minersIcon: '../assets/icons/miners-40x40.png',
      minersText: 'My Miners',
      addingContract: false,
      addingMiner: false,
      contractCodeId: '',
      contractActorId: '',
      contractRobustAddress: '',
      minerId: ''
    }
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', true)
    this.$store.commit('setToolbarShowSettingBtn', true)
    this.$store.commit('setShowFooterHelp', false)
    this.$store.commit('setToolbarTitle', 'Custody Contracts')

    let contracts = localStorage.getItem(LocalStorageKeys.Contracts)
    this.$store.commit('setContracts', JSON.parse(contracts))

    let miners = localStorage.getItem(LocalStorageKeys.Miners)
    this.$store.commit('setMiners', JSON.parse(miners))

    evbus.on(GlobalEvents.ToolbarAddClick, this.onAddClick)
  },
  unmounted () {
    evbus.off(GlobalEvents.ToolbarAddClick)
  },
  methods: {
    onContractsClick: function () {
      this.$store.commit('setContractTab', 'contracts')
    },
    onMinersClick: function () {
      this.$store.commit('setContractTab', 'miners')
    },
    onContractClick: function (contract) {
      if (this.addingContract) {
        return
      }
      this.$router.push({
        path: '/mycontract',
        query: {
          contractId: contract.Title
        }
      })
    },
    onMinerClick: function (miner) {
      if (this.addingMiner) {
        return
      }
      this.$router.push({
        path: '/myminer',
        query: {
          minerId: miner.MinerId
        }
      })
    },
    onAddClick: function () {
      switch (this.curTab) {
      case 'contracts':
        this.addingContract = true
        break
      case 'miners':
        this.addingMiner = true
        break
      }
    },
    onAddContractClick: function () {
      if (this.contractActorId.length === 0 ||
          this.contractCodeId.length === 0 ||
          this.contractRobustAddress.length === 0) {
        return
      }

      let contract = this.$store.getters.contractById(this.contractActorId)
      if (contract) {
        return
      }

      this.validatePeggyContract(
        this.contractActorId,
        this.contractCodeId,
        this.contractRobustAddress,
        (valid) => {
          if (!valid) {
            this.$store.commit('setShowGlobalTip', true)
            this.$store.commit('setGlobalTipText', '<span style="color: red">Invalid Peggy Contract<span>')
            return
          }

          this.addingContract = false

          let contracts = this.$store.getters.contracts
          contracts.push({
            CodeID: this.contractCodeId,
            Title: this.contractActorId,
            Subtitle: this.contractRobustAddress
          })
          this.$store.commit('setContracts', contracts)

          localStorage.setItem(LocalStorageKeys.Contracts, JSON.stringify(contracts))
        }
      )
    },
    onCancelContractClick: function () {
      this.addingContract = false
    },
    onVerifyContractClick: function () {
      if (this.contractActorId.length === 0 ||
          this.contractCodeId.length === 0 ||
          this.contractRobustAddress.length === 0) {
        return
      }
      this.validatePeggyContract(
        this.contractActorId,
        this.contractCodeId,
        this.contractRobustAddress,
        (valid) => {
          this.$store.commit('setShowGlobalTip', true)
          if (!valid) {
            this.$store.commit('setGlobalTipText', '<span style="color: red">Invalid Peggy Contract<span>')
            return
          }
          this.$store.commit('setGlobalTipText', '<span style="color: green">Valid Peggy Contract<span>')
        }
      )
    },
    validatePeggyContract: function (actorId, codeId, robustAddress, handler) {
      let network = this.$store.getters.selectedNetwork
      if (!network) {
        return
      }

      checkPeggy(network.RpcEndpoint, robustAddress)
        .then(() => {
          handler(true)
        })
        .catch(() => {
          handler(false)
        })
    },
    onDeployClick: function () {
      window.open('https://remix.ethereum.org/')
    },
    onAddMinerClick: function () {
      if (this.minerId.length === 0) {
        return
      }

      let network = this.$store.getters.selectedNetwork
      if (!network) {
        return
      }

      let miner = this.$store.getters.minerById(this.minerId)
      if (miner) {
        return
      }

      this.addingMiner = false

      this.validateMiner(this.minerId, (miner) => {
        let miners = this.$store.getters.miners
        if (miners === null || miners === undefined) {
          miners = []
        }

        miner.MinerId = this.minerId

        // TODO: get other miner info
        miner.EstimateDailyReward = 123.0

        miners.push(miner)

        this.$store.commit('setMiners', miners)
        localStorage.setItem(LocalStorageKeys.Miners, JSON.stringify(miners))
      })
    },
    onCancelMinerClick: function () {
      this.addingMiner = false
    },
    onVerifyMinerClick: function () {
      if (this.minerId.length === 0) {
        return
      }

      let network = this.$store.getters.selectedNetwork
      if (!network) {
        return
      }

      this.validateMiner(this.minerId, () => {})
    },
    validateMiner: function (minerId, handler) {
      let network = this.$store.getters.selectedNetwork
      if (!network) {
        return
      }

      let self = this
      this.$store.commit('setShowGlobalTip', true)

      minerInfo(network.HttpEndpoint, minerId)
        .then((resp) => {
          handler(resp.data.result)
          self.$store.commit('setGlobalTipText', '<span style="color: green">Valid Miner<span>')
        })
        .catch(() => {
          self.$store.commit('setGlobalTipText', '<span style="color: red">Invalid Miner<span>')
        })
    }
  },
  computed: {
    contracts () {
      return this.$store.getters.contracts
    },
    miners () {
      return this.$store.getters.miners
    },
    curTab () {
      if (!this.$store.getters.contractTab) {
        return 'contracts'
      }
      return this.$store.getters.contractTab
    }
  }
}
</script>

<style scoped>
.page {
  padding: 0 16px 0 16px;
  height: 100%;
}

.page .inner-item {
  border-bottom: 1px solid #D6D9DC;
}

.tabs {
  position: absolute;
  top: 508px;
  display: flex;
  height: 64px;
  width: 100%;
  border-top: 1px solid #D6D9DC;
  background-color: white;
}

.tabs .tab {
  display: flex;
  height: 40px;
  width: 50%;
  color: #535A61;
  font-size: 14px;
  cursor: pointer;
  margin: 12px 0 12px 0;
  justify-content: center;
}

.tabs .tab-selected {
  color: #0D99FF;
}

.tabs .tab .icon {
  height: 40px;
  width: 40px;
}

.tabs .tab .text {
  height: 14px;
  max-width: 50%;
  line-height: 40px;
  margin-left: 10px;
}

.popup {
  position: absolute;
  margin: 16px;
  padding: 16px;
  background-color: white;
  border-radius: 8px;
  top: 110px;
  min-height: 120px;
  width: 296px;
  color: #535A61;
}

.blur {
  filter: blur(8px);
  background-color: rgba(83, 90, 97, 0.2);
}

.popup .title {
  font-weight: bold;
  text-align: center;
  margin-bottom: 16px;
}

.popup .area {
  margin: 10px 0 0 0;
  width: 100%;
}

.popup input {
  border: none;
  border-bottom: 1px solid #D6D9DC;
  width: 100%;
}

.popup input:focus {
  outline: 1px solid #0D99FF;
}

.popup .btns {
  display: flex;
  margin-top: 24px;
}

.popup .btns .btn {
  width: 60px;
  height: 24px;
  border-radius: 8px;
  margin-right: 8px;
  border: 1px solid #0D99FF;
  color: #535A61;
  cursor: pointer;
}

.tips {
  color: #535A61;
  cursor: pointer;
  margin: 8px 0 16px 0;
}

.tips .deploy {
  color: #0D99FF;
}
</style>