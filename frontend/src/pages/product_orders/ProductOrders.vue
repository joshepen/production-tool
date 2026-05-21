<script setup lang="ts">
  import { useQueryClient } from '@tanstack/vue-query'
  import ThingTable from '@/components/ThingTable.vue'
  import CreateProductOrderDialog from '@/pages/product_orders/CreateProductOrderDialog'
  import { useProductOrderQuery } from '@/queries/DatabaseQueries'
  import { useBackendApiStore } from '@/stores/backendApi'
  const productOrderQuery = useProductOrderQuery()
  const queryClient = useQueryClient()
  const backendApiStore = useBackendApiStore()
  const headers = [{ title: 'Created At', key: 'created_at', isDatetime: true }, { title: 'Order ID', key: 'id' }, { title: 'Status', key: 'status_name' }, { title: 'Address', key: 'address' }]

  async function onDelete (id: number) {
    await backendApiStore._delete('/product_order/' + id)
    queryClient.invalidateQueries(['product_orders'])
  }

  function getName (productOrder: ProductOrder): string {
    return 'Order ' + productOrder.id + ' to ' + productOrder.address
  }
</script>

<template>
  <div :style="{display: 'flex', flexDirection: 'column', margin: '10px'}">
    <h1>Product Orders</h1>
    <ThingTable
      :get-name
      :headers
      :query="productOrderQuery"
      @delete="onDelete"
    >
      <template #create-dialog="{open,close}"><CreateProductOrderDialog :model-value="open" @update:model-value="close" /></template>
    </ThingTable>
  </div>
</template>
