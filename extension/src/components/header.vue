<template>
  <div class='header'>
    <img :src='logoPath'/>
  </div>
</template>

<script>
export default {
  name: 'headerComponent',
  data () {
    return {
      network: 'Hyperspace',
      logoPath: '../assets/logos/64x64.png'
    }
  }
}

</script>

<style scoped>
.header {
  height: 64px;
  width: 100%;
  background-color: #f2f4f6;
}
</style>
