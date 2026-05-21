<script setup lang="ts">
  import { useQueryClient } from '@tanstack/vue-query'
  import ThingTable from '@/components/ThingTable.vue'
  import CreateProductDialog from '@/pages/products/CreateProductDialog'
  import { useProductQuery } from '@/queries/DatabaseQueries'
  import { useBackendApiStore } from '@/stores/backendApi'
  const productQuery = useProductQuery()
  const queryClient = useQueryClient()
  const backendApiStore = useBackendApiStore()
  const headers = [{ title: 'Product Name', key: 'name' }]

  async function onDelete (id: number) {
    await backendApiStore._delete('/product/' + id)
    queryClient.invalidateQueries(['products'])
  }

  function getName (product: Product): string {
    return product.name
  }
</script>

<template>
  <div :style="{display: 'flex', flexDirection: 'column', margin: '10px'}">
    <h1>Products</h1>
    <ThingTable
      :get-name
      :headers
      :query="productQuery"
      @delete="onDelete"
    >
      <template #create-dialog="{open,close}"><CreateProductDialog :model-value="open" @update:model-value="close" /></template>
    </ThingTable>
  </div>
</template>
