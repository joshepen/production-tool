<script setup lang="ts">
import type { ProductOrder } from '@/types/DatabaseTypes'
import { useQueryClient } from '@tanstack/vue-query'
import { useRouter } from 'vue-router'
import ThingTable from '@/components/ThingTable.vue'
import CreateProductOrderDialog from '@/pages/product_orders/CreateProductOrderDialog.vue'
import { useProductOrderQuery, useStatusQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
import { ref } from 'vue'

const productOrderQuery = useProductOrderQuery()
const statusQuery = useStatusQuery()
const queryClient = useQueryClient()
const backendApiStore = useBackendApiStore()
const router = useRouter()
const editingId = ref<number>(0)

const headers = [
  { title: 'Created At', key: 'created_at', isDatetime: true },
  { title: 'Order ID', key: 'id' },
  { title: 'Product', key: 'product_name' },
  { title: 'Status', key: 'status_id' },
  { title: 'Address', key: 'address' },
  { title: 'Work Orders', key: 'work_orders' },
  { title: '', key: 'edit', width: '80px' },
]

async function onDelete(id: number) {
  await backendApiStore._delete('/product_order/' + id)
  queryClient.invalidateQueries({ queryKey: ['product_orders'] })
}

function getName(productOrder: ProductOrder): string {
  return 'Order ' + productOrder.id + ' to ' + productOrder.address
}

function handleStatusChange(po: ProductOrder, status_id: number) {
  backendApiStore.post(`/product_order/${po.id}/status`, { status_id }).then(() => {
    queryClient.invalidateQueries({ queryKey: ['product_orders'] })
  })
}

function navigateToWorkOrders(po: ProductOrder) {
  router.push(`/work_orders/${po.id}`)
}

function onEdit(po: ProductOrder) {
  editingId.value = po.id
}
</script>

<template>
  <div :style="{ display: 'flex', flexDirection: 'column', margin: '10px' }">
    <h1>Product Orders</h1>
    <ThingTable
      :custom-columns="['status_id', 'work_orders', 'edit']"
      :get-name
      :headers
      :query="productOrderQuery"
      @delete="onDelete"
    >
      <template #create-dialog="{ open, close }"
        ><CreateProductOrderDialog :model-value="open" @update:model-value="close"
      /></template>
      <template #item.status_id="{ item }">
        <v-select
          density="compact"
          item-title="name"
          item-value="id"
          :items="statusQuery.data.value"
          :model-value="item.status_name"
          :style="{ display: 'flex', alignItems: 'center', margin: '0' }"
          variant="plain"
          @update:model-value="(newId) => handleStatusChange(item, newId)"
        />
      </template>
      <template #item.work_orders="{ item }">
        <v-btn variant="text" size="small" @click="navigateToWorkOrders(item)">
          View Work Orders
        </v-btn>
      </template>
      <template #item.edit="{ item }">
        <v-btn size="small" variant="text" @click="onEdit(item)">
          Edit
        </v-btn>
      </template>
    </ThingTable>
    <EditProductOrderDialog
      v-if="editingId"
      :open="true"
      :id="editingId"
      @update:open="editingId = 0"
    />
  </div>
</template>