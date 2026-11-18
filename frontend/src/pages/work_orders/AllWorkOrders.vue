<script setup lang="ts">
import type { PrettyWorkOrder } from '@/types/DatabaseTypes'
import { useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed } from 'vue'
import ThingTable from '@/components/ThingTable.vue'
import { useBackendApiStore } from '@/stores/backendApi'
import { useStatusQuery, useUserQuery } from '@/queries/DatabaseQueries'

const backendApiStore = useBackendApiStore()
const statusQuery = useStatusQuery()
const userQuery = useUserQuery()
const queryClient = useQueryClient()

// Query for all work orders across all product orders
const allWorkOrdersQuery = useQuery({
  queryKey: ['all_work_orders'],
  queryFn: async () => {
    const response = await backendApiStore.get('/pretty_work_orders?product_order_id=0')
    return response.data as PrettyWorkOrder[]
  },
})

const isPending = computed(() => allWorkOrdersQuery.isPending.value)

const headers = [
  { title: 'Product Order', key: 'product_order_address' },
  { title: 'Step Order', key: 'step_order' },
  { title: 'Work Center', key: 'work_center_name' },
  { title: 'Assignee', key: 'assignee_name' },
  { title: 'Status', key: 'status_name' },
  { title: 'Estimated Hours', key: 'estimated_hours' },
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
      <h1>All Work Orders</h1>
      <v-btn color="primary" @click="allWorkOrdersQuery.refetch" :disabled="isPending">
        Refresh
      </v-btn>
    </div>
    <ThingTable
      :custom-columns="['assignee_name', 'status_name', 'product_order_address']"
      :get-name
      :headers
      :query="allWorkOrdersQuery"
    >
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
      <template #item.product_order_address="{ item }">
        <v-btn
          size="small"
          variant="text"
          @click="$router.push('/work_orders/' + item.product_order_id)"
        >
          #{{ item.product_order_id }} - {{ item.product_order_address }}
        </v-btn>
      </template>
      <template #item.started_at="{ item }">
        <div :style="{ display: 'flex', alignItems: 'center', gap: '5px' }">
          <span>{{ item.started_at ? new Date(item.started_at).toLocaleString() : '-' }}</span>
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
          <span>{{ item.completed_at ? new Date(item.completed_at).toLocaleString() : '-' }}</span>
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