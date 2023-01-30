<template>
  <div class='page'>
    <div class='inner'>
      Coming soon ...
    </div>
  </div>
</template>

<script>
export default {
  name: 'custodyContracts',
  data () {
    return {
    }
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', true)
    this.$store.commit('setToolbarTitle', 'Custody Contracts')
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