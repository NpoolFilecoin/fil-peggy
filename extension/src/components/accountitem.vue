<template>
  <div class='item'>
    <img class='icon icon1' src='../assets/icons/filecoin-24x24.png' />
    <div class='content'>
      <div class='address'>{{ address }}</div>
      <div class='name'>{{ name }}</div>
      <div class='balance'>{{ balanceStr }}</div>
      <div class='info'>
        <span v-if='usedFor' class='tag'>{{ usedFor }}</span>
        <span class='tag'>{{ warm ? 'Warm' : 'Cold' }}</span>
        <span v-if='blockchain' class='tag'>{{ blockchain }}</span>
      </div>
    </div>
    <img class='icon' src='../assets/icons/delete-24x24.png' />
    <img class='icon' src='../assets/icons/setting-24x24.png' />
  </div>
</template>

<script>
import { amountDisplay } from '../utils/amount_display';

export default {
  name: 'filecoinAccounts',
  props: {
    address: {
      type: String,
      required: true
    },
    name: {
      type: String,
      required: false
    },
    usedFor: {
      type: String,
      required: false
    },
    warm: {
      type: Boolean,
      required: true
    },
    balance: {
      // eslint-disable-next-line no-undef
      type: BigInt,
      required: true
    },
    blockchain: {
      type: String,
      required: true
    }
  },
  data () {
    return {

    }
  },
  computed: {
    balanceStr () {
      return amountDisplay(this.balance)
    }
  }
}
</script>

<style scoped>
.item {
  display: flex;
  padding: 16px 0 16px 0;
}

.item .content {
  width: 100%;
  margin-right: 10px;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
  width: calc(100% - 58px);
}

.item .icon {
  height: 24px;
  width: 24px;
}

.item .icon1 {
  margin-right: 10px;
}

.item .address {
  font-weight: bold;
  color: #535A61;
  text-align: left;
}

.item .name {
  color: #535A61;
  text-align: left;
}

.item .balance {
  color: #535A61;
  text-align: left;
  margin-top: 10px;
}

.item .info {
  color: #535A61;
  text-align: left;
  margin-top: 4px;
}

.item .tag {
  border: 1px solid white;
  background-color: #28D90C;
  color: white;
  padding: 2px 6px 2px 6px;
  border-radius: 4px;
}
</style>
