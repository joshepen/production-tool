<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useQueryClient } from '@tanstack/vue-query'
import { useProductQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
import { useStatusMessageStore } from '@/stores/statusMessage'

const props = defineProps<{ open: boolean; id: number }>()
const emit = defineEmits(['update:open'])

const backendApiStore = useBackendApiStore()
const productQuery = useProductQuery()
const messageStore = useStatusMessageStore()
const queryClient = useQueryClient()

const address = ref<string>('')
const productId = ref<number>(0)

async function loadProductOrder() {
  if (!props.id) return
  const res = await backendApiStore.get(`/product_order/${props.id}`)
  address.value = res.data.address
  productId.value = res.data.product_id
}

onMounted(() => {
  loadProductOrder()
})

function close() {
  emit('update:open', false)
}

function saveProductOrder() {
  backendApiStore.put(`/product_order/${props.id}`, { address: address.value, product_id: productId.value }).then(() => {
    messageStore.displayMessage('Successfully updated product order', 'success')
    queryClient.invalidateQueries({ queryKey: ['product_orders'] })
    close()
  })
}

watch(() => props.id, () => {
  if (props.open) {
    loadProductOrder()
  }
})
</script>

<template>
  <v-dialog :model-value="open" width="auto" @update:model-value="$emit('update:open', $event)">
    <v-card>
      <v-card-item><v-card-title>Edit Product Order</v-card-title></v-card-item>
      <div
        :style="{
          display: 'flex',
          flexDirection: 'column',
          gap: '20px',
          alignItems: 'start',
          marginInline: '15px',
          paddingInline: '4px',
        }"
      >
        <v-text-field
          v-model="address"
          density="compact"
          hide-details="auto"
          label="Delivery Address"
          :style="{ minWidth: '280px' }"
        />
        <v-autocomplete
          v-model="productId"
          hide-details="auto"
          item-title="name"
          item-value="id"
          :items="productQuery.data.value"
          label="Product"
          :style="{ minWidth: '320px' }"
        />
      </div>
      <template #actions>
        <v-btn color="indigo-lighten-1" @click="saveProductOrder">Confirm</v-btn>
        <v-btn color="red-lighten-1" @click="close">Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>