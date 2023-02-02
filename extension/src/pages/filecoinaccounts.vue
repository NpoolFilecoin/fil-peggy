<template>
  <div class='page'>
    <div class='inner'>
      <accountItem
        class='account-item'
        v-for='(account, index) in accounts'
        :key='index'
        :address='account.Address'
        :used-for='account.UsedFor'
        :warm='account.Warm'
        :balance='account.Balance'
        :name='account.Name'
      />
    </div>
  </div>
</template>

<script>
import accountItem from '../components/accountitem.vue'

export default {
  name: 'filecoinAccounts',
  components: {
    accountItem,
  },
  data () {
    return {
    }
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', true)
    this.$store.commit('setToolbarShowSettingBtn', true)
    this.$store.commit('setShowFooterHelp', false)
    this.$store.commit('setToolbarTitle', 'Filecoin Accounts')
  },
  methods: {
  },
  computed: {
    accounts () {
      return this.$store.getters.filecoinAccounts
    }
  }
}
</script>

<style scoped>
.page {
  margin: 0 16px 0 16px;
}

.page .inner {
  margin: 16px 0 16px 0;
  text-align: center;
}

.page .account-item {
  border-bottom: 1px solid #D6D9DC;
}
</style>