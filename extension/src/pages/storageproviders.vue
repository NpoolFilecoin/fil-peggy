<template>
  <div class='page'>
    <div class='inner'>
      Coming soon ...
    </div>
  </div>
</template>

<script>
export default {
  name: 'storageProviders',
  data () {
    return {
    }
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', true)
    this.$store.commit('setToolbarTitle', 'Storage Providers')
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