<template>
  <div class='page'>
    <div class='inner'>
      Coming soon ...
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
export default {
  name: 'custodyContracts',
  data () {
    return {
      contractsIcon: '../assets/icons/contracts-40x40.png',
      contractsText: 'My Contracts',
      minersIcon: '../assets/icons/miners-40x40.png',
      minersText: 'My Miners',
      curTab: 'contracts'
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
    }
  }
}
</script>

<style scoped>
.page {
  padding: 0 16px 0 16px;
  height: 394px;
}

.page .inner {
  padding: 16px 0 16px 0;
  text-align: center;
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