<script setup lang="ts">
import type { PrettyWorkOrder } from '@/types/DatabaseTypes'
import { useQueryClient } from '@tanstack/vue-query'
import { ref, watch, computed } from 'vue'
import ThingTable from '@/components/ThingTable.vue'
import { useWorkOrderQuery, useStatusQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()
const productOrderId = computed(() => parseInt(route.params.productOrderId as string) || 0)
const workOrderQuery = useWorkOrderQuery(productOrderId.value)
const statusQuery = useStatusQuery()
const queryClient = useQueryClient()
const backendApiStore = useBackendApiStore()

const headers = [
  { title: 'Step Order', key: 'step_order' },
  { title: 'Work Center', key: 'work_center_name' },
  { title: 'Status', key: 'status_name' },
  { title: 'Estimated Hours', key: 'estimated_hours' },
  { title: 'Started At', key: 'started_at', isDatetime: true },
  { title: 'Completed At', key: 'completed_at', isDatetime: true },
]

async function handleStatusChange(wo: PrettyWorkOrder, status_id: number) {
  await backendApiStore.post(`/work_order/${wo.id}/status`, { status_id })
  queryClient.invalidateQueries({ queryKey: ['work_orders', productOrderId.value] })
}

async function handleStart(wo: PrettyWorkOrder) {
  await backendApiStore.post(`/work_order/${wo.id}/start`, {})
  queryClient.invalidateQueries({ queryKey: ['work_orders', productOrderId.value] })
}

async function handleComplete(wo: PrettyWorkOrder) {
  await backendApiStore.post(`/work_order/${wo.id}/complete`, {})
  queryClient.invalidateQueries({ queryKey: ['work_orders', productOrderId.value] })
}

function getName(wo: PrettyWorkOrder): string {
  return `Work Order ${wo.id} - ${wo.work_center_name}`
}
</script>

<template>
  <div :style="{ display: 'flex', flexDirection: 'column', margin: '10px' }">
    <div :style="{ display: 'flex', alignItems: 'center', gap: '10px', marginBottom: '20px' }">
      <v-btn icon="mdi-arrow-left" @click="router.push('/product_orders')" />
      <h1>Work Orders for Product Order #{{ productOrderId }}</h1>
    </div>
    <ThingTable
      :custom-columns="['status_name']"
      :get-name
      :headers
      :query="workOrderQuery"
    >
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