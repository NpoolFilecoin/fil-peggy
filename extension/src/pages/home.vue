<template>
  <div class='all-offers'>
    <div class='inner'>
      <img class='icon' :src='allOffersIcon' />
      <div class='text'>{{ allOffersText }}</div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'homePage',
  data () {
    return {
      allOffersIcon: '../assets/icons/all-offers-24x24.png',
      allOffersText: 'All Offers'
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
</style>