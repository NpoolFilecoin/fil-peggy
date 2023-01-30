<template>
  <div :class='["page", adding ? "blur" : ""]'>
    <div
      class='contract-item'
      v-for='contract in contracts'
      :key='contract.Title'
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
  <div v-if='adding' class='popup'>
    <div class='title'>Add My Contract</div>
    <div class='area'>
      <div>Code ID</div>
      <div>
        <input type='text' placeholder='Input code id of contract'>
      </div>
    </div>
    <div class='area'>
      <div>Actor ID</div>
      <div>
        <input type='text' placeholder='Input actor id of contract'>
      </div>
    </div>
    <div class='btns'>
      <button class='btn' v-on:click='onAddContractClick'>Add</button>
      <button class='btn' v-on:click='onCancelClick'>Cancel</button>
    </div>
  </div>
</template>

<script>
import contractItem from '../components/contractitem.vue'
import { GlobalEvents } from '../const/global_events'
import { evbus } from '../evbus/event_bus'

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
      adding: false
    }
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', true)
    this.$store.commit('setToolbarTitle', 'Custody Contracts')

    evbus.on(GlobalEvents.ToolbarAddClick, this.onAddClick)
  },
  unmounted () {
    evbus.off(GlobalEvents.ToolbarAddClick)
  },
  methods: {
    onContractsClick: function () {
      this.curTab = 'contracts'
    },
    onMinersClick: function () {
      this.curTab = 'miners'
    },
    onContractClick: function (contract) {
      if (this.adding) {
        return
      }
      this.$router.push({
        path: '/mycontract',
        query: {
          contractId: contract.Title
        }
      })
    },
    onAddClick: function () {
      this.adding = true
    },
    onAddContractClick: function () {
      this.adding = false
    },
    onCancelClick: function () {
      this.adding = false
    }
  },
  computed: {
    contracts () {
      return this.$store.getters.contracts
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
</style>