<template>
  <div>
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
        v-on:click='() => onMenuClick(menu)'
      />
    </div>
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
      menus: [
        {
          title: 'Storage Providers',
          icon: '../assets/icons/filecoin-64x64.png',
          content: 'Check Your On-Chain Storage Providers',
          path: '/storageproviders'
        }, {
          title: 'Investment Shares',
          icon: '../assets/icons/investment-shares-64x64.png',
          content: 'Deposit / Redeem / Withdraw Your Investment',
          path: '/investmentshares'
        }, {
          title: 'Custody Contracts',
          icon: '../assets/icons/custody-contracts-64x64.png',
          content: 'Custody Storage Provider to Smart Contract',
          path: '/custodycontracts'
        }, {
          title: 'Filecoin Accounts',
          icon: '../assets/icons/accounts-64x64.png',
          content: 'Add / Remove / Set Your Filecoin Accounts',
          path: '/filecoinaccounts'
        }
      ]
    }
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', false)
    this.$store.commit('setShowFooterHelp', true)
    this.$store.commit('setToolbarTitle', 'Home')
  },
  methods: {
    onMenuClick: function (menu) {
      this.$router.push(menu.path)
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

.menus {
  margin: 0 16px 0 16px;
  height: 396px;
}

.menu-item {
  height: 80px;
  border-bottom: 1px solid #D6D9DC;
}
</style>