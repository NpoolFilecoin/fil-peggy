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
import { AccountUsedFors } from '../const/account_used_for'

export default {
  name: 'filecoinAccounts',
  components: {
    accountItem,
  },
  data () {
    return {
      accounts: [
        {
          Address: 't121346456456546',
          Name: 'Filecoin Hyperspace Test Account',
          Balance: 2574561321313212312312321n,
          UsedFor: AccountUsedFors.FundsAccount,
          Warm: true
        }, {
          Address: 't1213464564565safdsaasfd46',
          Name: 'Filecoin Hyperspace Test Account1',
          Balance: 2574561321313212312312321n,
          UsedFor: AccountUsedFors.InitialOwner,
          Warm: true
        }, {
          Address: 't1213464564565safdsaasfd46',
          Name: 'F07824 Worker',
          Balance: 2574561321313212312312321n,
          Warm: false
        }
      ]
    }
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', true)
    this.$store.commit('setToolbarShowSettingBtn', true)
    this.$store.commit('setShowFooterHelp', false)
    this.$store.commit('setToolbarTitle', 'Filecoin Accounts')
  },
  methods: {
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