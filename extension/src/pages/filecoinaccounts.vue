<template>
  <div :class='["page", adding ? "blur" : ""]'>
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
        :blockchain='account.Blockchain'
      />
    </div>
  </div>
  <div v-if='adding' class='popup'>
    <div class='title'>Import Filecoin Account</div>
    <div class='area'>
      <div>Private Key</div>
      <div>
        <input type='text' placeholder='Input private key' v-model='privateKey' v-on:blur='onPrivateKeyEntered'>
      </div>
    </div>
    <div class='area'>
      <div>Address</div>
      <div>
        <input type='text' v-model='address' :disabled='privateKey.length > 0'>
      </div>
    </div>
    <div class='area'>
      <div>Name</div>
      <div>
        <input type='text' placeholder='Input address name' v-model='addressName'>
      </div>
    </div>
    <div class='btns'>
      <button class='btn' v-on:click='onImportAddressClick'>Import</button>
      <button class='btn' v-on:click='onCancelClick'>Cancel</button>
    </div>
    <div class='tips' v-on:click='onCreateClick'>
      <span>Don't have account ? </span>
      <span class='create'>Create</span>
    </div>
  </div>
</template>

<script>
import { GlobalEvents } from '../const/global_events'
import accountItem from '../components/accountitem.vue'
import { LocalStorageKeys } from '../const/store_keys'
import { evbus } from '../evbus/event_bus'
import { importWallet } from '../filapi/filapi'
import { privateKeyToAccount } from '../web3/web3'
import { Blockchains } from '../const/blockchain_def'

export default {
  name: 'filecoinAccounts',
  components: {
    accountItem,
  },
  data () {
    return {
      adding: false,
      privateKey: '',
      address: '',
      addressName: '',
      blockchain: Blockchains.FIL
    }
  },
  mounted () {
    let accounts = localStorage.getItem(LocalStorageKeys.FilecoinAccounts)
    this.$store.commit('setFilecoinAccounts', JSON.parse(accounts))

    this.$store.commit('setToolbarShowAddBtn', true)
    this.$store.commit('setToolbarShowSettingBtn', true)
    this.$store.commit('setShowFooterHelp', false)
    this.$store.commit('setToolbarTitle', 'Filecoin Accounts')

    evbus.on(GlobalEvents.ToolbarAddClick, this.onAddClick)
  },
  methods: {
    onAddClick: function () {
      this.adding = true
    },
    onImportAddressClick: function () {
      if (this.address.length === 0) {
        return
      }

      let address = this.$store.getters.filecoinAccountByAddress(this.address)
      if (address) {
        return
      }

      this.adding = false

      let accounts = this.$store.getters.filecoinAccounts
      if (accounts === null || accounts === undefined) {
        accounts = []
      }
      accounts.push({
        Address: this.address,
        Name: this.addressName,
        Warm: this.privateKey.length > 0,
        PrivateKey: this.privateKey,
        Blockchain: this.blockchain
      })

      this.$store.commit('setFilecoinAccounts', accounts)
      localStorage.setItem(LocalStorageKeys.FilecoinAccounts, JSON.stringify(accounts))
    },
    onCancelClick: function () {
      this.adding = false
    },
    onCreateClick: function () {
      this.adding = false
    },
    onPrivateKeyEntered: function () {
      try {
        let address = importWallet(this.privateKey)
        this.address = address.Address
      } catch {
        let address = privateKeyToAccount(this.privateKey)
        this.address = address.address
        this.blockchain = Blockchains.ETH
      }
    }
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
  width: 100%;
}

.popup {
  position: absolute;
  margin: 16px;
  padding: 16px;
  background-color: white;
  border-radius: 8px;
  top: 110px;
  min-height: 120px;
  width: 296px;
  color: #535A61;
}

.blur {
  filter: blur(8px);
  background-color: rgba(83, 90, 97, 0.2);
}

.popup .title {
  font-weight: bold;
  text-align: center;
  margin-bottom: 16px;
}

.popup .area {
  margin: 10px 0 0 0;
  width: 100%;
}

.popup input {
  border: none;
  border-bottom: 1px solid #D6D9DC;
  width: 100%;
}

.popup input:focus {
  outline: 1px solid #0D99FF;
}

.popup .btns {
  display: flex;
  margin-top: 24px;
}

.popup .btns .btn {
  width: 60px;
  height: 24px;
  border-radius: 8px;
  margin-right: 8px;
  border: 1px solid #0D99FF;
  color: #535A61;
  cursor: pointer;
}

.tips {
  color: #535A61;
  cursor: pointer;
  margin: 8px 0 16px 0;
}

.tips .create {
  color: #0D99FF;
}
</style>