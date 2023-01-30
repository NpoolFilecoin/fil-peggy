<template>
  <div class='page'>
    <div
      class='contract-item'
      v-for='contract in contracts'
      :key='contract.title'
    >
      <contractItem
        :title='contract.title'
        :icon='contract.icon'
        :subtitle='contract.subtitle'
        :custody-type='contract.custodyType'
        :value='contract.value'
        :miners='contract.miners'
        :raw-power-bytes='contract.rawPowerBytes'
        :adj-power-bytes='contract.adjPowerBytes'
        :estimate-daily-reward='contract.estimateDailyReward'
        v-on:click='() => onContractClick(contract)'
      />
    </div>
  </div>
  <div class='tabs'>
    <div :class='["tab", curTab == "contracts" ? "tab-selected" : ""]' v-on:click='onContractsClick'>
      <img class='icon' :src='contractsIcon' />
      <div class='text'>{{ contractsText }}</div>
    </div>
    <div :class='["tab", curTab == "miners" ? "tab-selected" : ""]' v-on:click='onMinersClick'>
      <img class='icon' :src='minersIcon' />
      <div class='text'>{{ minersText }}</div>
    </div>
  </div>
</template>

<script>
import { CustodyTypes } from '../const/contract_types'
import contractItem from '../components/contractitem.vue'

export default {
  name: 'custodyContracts',
  components: {
    contractItem
  },
  data () {
    return {
      contractsIcon: '../assets/icons/contracts-40x40.png',
      contractsText: 'My Contracts',
      minersIcon: '../assets/icons/miners-40x40.png',
      minersText: 'My Miners',
      curTab: 'contracts',
      contracts: [
        {
          title: 'f078235',
          subtitle: 't410fafsypcszjsrfkm4k36snjbcj62jef24pn7ysykq',
          custodyType: CustodyTypes.FixedIncome,
          value: 30,
          miners: [
            "f0182365",
            "f0135689",
            "f0135689",
            "f0135689",
            "f0135689",
            "f0135689"
          ],
          rawPowerBytes: 17989552222,
          adjPowerBytes: 1245678946564,
          estimateDailyReward: 245.9,
          icon: '../assets/icons/custody-contracts-64x64.png'
        }
      ]
    }
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', true)
    this.$store.commit('setToolbarTitle', 'Custody Contracts')
  },
  methods: {
    onContractsClick: function () {
      this.curTab = 'contracts'
    },
    onMinersClick: function () {
      this.curTab = 'miners'
    },
    onContractClick: function (contract) {
      console.log(contract)
    }
  }
}
</script>

<style scoped>
.page {
  padding: 0 16px 0 16px;
  height: 394px;
}

.page .contract-item {
  border-bottom: 1px solid #D6D9DC;
}

.tabs {
  position: absolute;
  top: 508px;
  display: flex;
  height: 64px;
  width: 100%;
  border-top: 1px solid #D6D9DC;
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
</style>