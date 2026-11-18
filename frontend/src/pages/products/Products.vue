<script setup lang="ts">
import { ref } from 'vue'
import type { PrettyProduct } from '@/types/DatabaseTypes'
import { useQueryClient } from '@tanstack/vue-query'
import ThingTable from '@/components/ThingTable.vue'
import CreateProductDialog from '@/pages/products/CreateProductDialog.vue'
import EditProductDialog from '@/pages/products/EditProductDialog.vue'
import { usePrettyProductQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
const productQuery = usePrettyProductQuery()
const queryClient = useQueryClient()
const backendApiStore = useBackendApiStore()
const editingId = ref<number>(0)
const headers = [
  { title: 'Product Name', key: 'name' },
  { title: 'Work Centers', key: 'work_centers' },
  { title: 'Edit', key: 'edit' },
]

async function onDelete(id: number) {
  await backendApiStore._delete('/product/' + id)
  queryClient.invalidateQueries({ queryKey: ['products'] })
  queryClient.invalidateQueries({ queryKey: ['pretty_products'] })
}

function getName(product: PrettyProduct): string {
  return product.name
}

function onEdit(product: PrettyProduct) {
  editingId.value = product.id
}
</script>

<template>
  <div :style="{ display: 'flex', flexDirection: 'column', margin: '10px' }">
    <h1>Products</h1>
    <ThingTable :custom-columns="['edit']" :get-name :headers :query="productQuery" @delete="onDelete">
      <template #create-dialog="{ open, close }"
        ><CreateProductDialog :model-value="open" @update:model-value="close"
      /></template>
      <template #item.edit="{ item }">
        <v-btn size="small" variant="text" @click="onEdit(item)">
          Edit
        </v-btn>
      </template>
    </ThingTable>
    <EditProductDialog
      v-if="editingId"
      :open="true"
      :id="editingId"
      @update:open="editingId = 0"
    />
  </div>
</template>