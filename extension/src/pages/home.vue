<template>
  <div class='all-offers'>
    <div class='inner'>
      <img class='icon' :src='allOffersIcon' />
      <div class='text'>{{ allOffersText }}</div>
    </div>
  </div>
  <div class='menus'>
    <mainItem
      v-for='menu in menus'
      :key='menu.title'
      class='menu-item'
      :title='menu.title'
      :icon='menu.icon'
      :content='menu.content'
      v-on:click='menu.click'
    />
  </div>
  <div class='help'>
    {{ needHelp }}
    <span class='contact'>{{ peggySupport }}</span>
  </div>
</template>

<script>
import mainItem from '../components/mainitem.vue'

export default {
  name: 'homePage',
  components: {
    mainItem
  },
  data () {
    return {
      allOffersIcon: '../assets/icons/all-offers-24x24.png',
      allOffersText: 'All Offers',
      needHelp: 'Need help ? ',
      peggySupport: 'Peggy support',
      menus: [
        {
          title: 'Storage Providers',
          icon: '../assets/icons/filecoin-64x64.png',
          content: 'Check Your On-Chain Storage Providers',
          click: this.clickStorageProviders
        }, {
          title: 'Investment Shares',
          icon: '../assets/icons/investment-shares-64x64.png',
          content: 'Deposit / Redeem / Withdraw Your Investment'
        }, {
          title: 'Custody Contracts',
          icon: '../assets/icons/custody-contracts-64x64.png',
          content: 'Custody Storage Provider to Smart Contract'
        }, {
          title: 'Filecoin Accounts',
          icon: '../assets/icons/accounts-64x64.png',
          content: 'Add / Remove / Set Your Filecoin Accounts'
        }
      ]
    }
  },
  methods: {
    clickStorageProviders: function () {
      this.$store.commit('setToolbarShowAddBtn', true)
      this.$store.commit('setToolbarTitle', 'Storage Providers')
    }
  }
}
</script>

<style scoped>
.all-offers {
  height: 32px;
  width: 100%;
  background-color: #F2F4F6;
  font-size: 12px;
  color: #0D99FF;
  text-align: center;
}

.all-offers .inner {
  display: flex;
  height: 24px;
  padding: 4px 0 4px 0;
  cursor: pointer;
  align-items: center;
  justify-content:center;
}


.all-offers .inner .icon {
  height: 24px;
  width: 24px;
}

.all-offers .inner .text {
  height: 12px;
  margin-left: 6px;
}

.help {
  position: absolute;
  top: 540px;
  background-color: #F2F4F6;
  height: 32px;
  width: 100%;
  line-height: 32px;
  text-align: center;
  font-size: 12px;
  cursor: pointer;
}

.help .contact {
  color: #0D99FF;
}

.menus {
  margin: 0 16px 0 16px;
  height: 396px;
}

.menu-item {
  height: 80px;
  border-bottom: 1px solid #D6D9DC;
}
</style>