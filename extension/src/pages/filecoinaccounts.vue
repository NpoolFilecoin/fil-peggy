<template>
  <div class='page'>
    <div class='inner'>
      Coming soon ...
    </div>
  </div>
</template>

<script>
export default {
  name: 'filecoinAccounts',
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
</style>