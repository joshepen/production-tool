<script setup lang="ts">
import type { PrettyWorkOrder } from '@/types/DatabaseTypes'
import { useQuery } from '@tanstack/vue-query'
import { ref, computed } from 'vue'
import ThingTable from '@/components/ThingTable.vue'
import { useBackendApiStore } from '@/stores/backendApi'
import { useStatusQuery, useUserQuery, useProductOrderQuery } from '@/queries/DatabaseQueries'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()
const backendApiStore = useBackendApiStore()
const statusQuery = useStatusQuery()
const userQuery = useUserQuery()
const productOrderQuery = useProductOrderQuery()

const selectedProductOrderId = ref<number | null>(
  route.params.productOrderId ? parseInt(route.params.productOrderId as string) : null,
)

const productOrderItems = computed(() => {
  const orders = productOrderQuery.data.value ?? []
  return [
    { id: null, label: 'All product orders' },
    ...orders.map((po) => ({ id: po.id, label: `#${po.id} - ${po.address}` })),
  ]
})

// Query for all work orders across all product orders
const allWorkOrdersQuery = useQuery({
  queryKey: ['all_work_orders'],
  queryFn: async () => {
    const response = await backendApiStore.get('/pretty_work_orders?product_order_id=0')
    return response.data as PrettyWorkOrder[]
  },
})

const headers = [
  { title: 'Product Order', key: 'product_name' },
  { title: 'Step Order', key: 'step_order' },
  { title: 'Work Center', key: 'work_center_name' },
  { title: 'Assignee', key: 'assignee_name' },
  { title: 'Status', key: 'status_name' },
  { title: 'Started At', key: 'started_at', isDatetime: true },
  { title: 'Completed At', key: 'completed_at', isDatetime: true },
]

async function handleStatusChange(wo: PrettyWorkOrder, status_id: number) {
  await backendApiStore.post(`/work_order/${wo.id}/status`, { status_id })
  allWorkOrdersQuery.refetch()
}

async function handleAssigneeChange(wo: PrettyWorkOrder, userId: number | null) {
  await backendApiStore.post(`/work_order/${wo.id}/assignee`, { assignee_id: userId })
  allWorkOrdersQuery.refetch()
}

async function handleStart(wo: PrettyWorkOrder) {
  await backendApiStore.post(`/work_order/${wo.id}/start`, {})
  allWorkOrdersQuery.refetch()
}

async function handleComplete(wo: PrettyWorkOrder) {
  await backendApiStore.post(`/work_order/${wo.id}/complete`, {})
  allWorkOrdersQuery.refetch()
}

function getName(wo: PrettyWorkOrder): string {
  return `Work Order ${wo.id} - ${wo.work_center_name}`
}
</script>

<template>
  <div :style="{ display: 'flex', flexDirection: 'column', margin: '10px' }">
    <div :style="{ display: 'flex', alignItems: 'center', gap: '10px', marginBottom: '20px' }">
      <h1>Work Orders</h1>
    </div>
    <ThingTable
      :custom-columns="['assignee_name', 'status_name', 'product_name']"
      :get-name
      :headers
      :query="allWorkOrdersQuery"
    >
      <template #filter>
        <v-select
          :model-value="selectedProductOrderId"
          :items="productOrderItems"
          item-title="label"
          item-value="id"
          :style="{ minWidth: '260px' }"
          variant="outlined"
          clearable
          hide-details="auto"
          placeholder="Filter by product order"
          @update:model-value="
            (newId) => {
              if (newId) router.push(`/work_orders/${newId}`)
              else router.push('/work_orders')
            }
          "
        />
      </template>
      <template #item.assignee_name="{ item }">
        <v-autocomplete
          density="compact"
          item-title="name"
          item-value="id"
          :items="userQuery.data.value ?? []"
          :model-value="item.assignee_id ?? null"
          :style="{ minWidth: '180px' }"
          variant="plain"
          clearable
          @update:model-value="(newId) => handleAssigneeChange(item, newId)"
        />
      </template>
      <template #item.status_name="{ item }">
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
      <template #item.product_name="{ item }">
        <span>#{{ item.product_order_id }} - {{ item.product_name }}</span>
      </template>
      <template #item.started_at="{ item }">
        <div :style="{ display: 'flex', alignItems: 'center', gap: '5px' }">
          <span>{{ item.started_at && !isNaN(new Date(item.started_at).getTime()) ? new Date(item.started_at).toLocaleString() : 'Not started' }}</span>
          <v-btn
            v-if="item.status_id < 4"
            size="x-small"
            color="primary"
            @click="handleStart(item)"
          >
            Start
          </v-btn>
        </div>
      </template>
      <template #item.completed_at="{ item }">
        <div :style="{ display: 'flex', alignItems: 'center', gap: '5px' }">
          <span>{{ item.completed_at && !isNaN(new Date(item.completed_at).getTime()) ? new Date(item.completed_at).toLocaleString() : 'Incomplete' }}</span>
          <v-btn
            v-if="item.status_id === 4"
            size="x-small"
            color="success"
            @click="handleComplete(item)"
          >
            Complete
          </v-btn>
        </div>
      </template>
    </ThingTable>
  </div>
</template>