<script setup lang="ts">
import { useQueryClient } from '@tanstack/vue-query'
import { ref, watch } from 'vue'
import { useProductQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
import { useStatusMessageStore } from '@/stores/statusMessage'
const model = defineModel<boolean>()
const backendApiStore = useBackendApiStore()
const productQuery = useProductQuery()
const messageStore = useStatusMessageStore()
const queryClient = useQueryClient()

const address = ref<string>('')
const productId = ref<number>(null)

function createProductOrder() {
  const data = { address: address.value, product_id: productId.value }
  backendApiStore.post('/product_order', data).then(() => {
    model.value = false
    messageStore.displayMessage('Successfully created product order', 'success')
    queryClient.invalidateQueries(['product_orders'])
  })
}

function clearFields() {
  address.value = ''
  productId.value = null
}

watch(model, (val) => {
  if (val) {
    clearFields()
  }
})
</script>

<template>
  <v-dialog v-model="model" width="auto">
    <v-card>
      <v-card-item><v-card-title>Create Product Order</v-card-title></v-card-item>
      <div
        :style="{
          display: 'flex',
          flexDirection: 'column',
          gap: '20px',
          alignItems: 'start',
          marginInline: '15px',
        }"
      >
        <v-text-field
          v-model="address"
          density="compact"
          hide-details="auto"
          label="Delivery Address"
          :style="{ minWidth: '250px' }"
        />
        <v-autocomplete
          v-model="productId"
          hide-details="auto"
          item-title="name"
          item-value="id"
          :items="productQuery.data.value"
          label="Product"
          :style="{ minWidth: '300px' }"
        />
      </div>
      <template #actions>
        <v-btn color="indigo-lighten-1" @click="createProductOrder">Confirm</v-btn>
        <v-btn color="red-lighten-1" @click="model = false">Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>
