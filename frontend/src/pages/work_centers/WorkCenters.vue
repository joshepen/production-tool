<script setup lang="ts">
import type { WorkCenter } from '@/types/DatabaseTypes'
import { useQueryClient } from '@tanstack/vue-query'
import ThingTable from '@/components/ThingTable.vue'
import CreateWorkCenterDialog from '@/pages/work_centers/CreateWorkCenterDialog.vue'
import { useWorkCenterQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
const workCenterQuery = useWorkCenterQuery()
const queryClient = useQueryClient()
const backendApiStore = useBackendApiStore()
const headers = [{ title: 'Name', key: 'name' }, { title: 'Description', key: 'description' }]

async function onDelete(id: number) {
  await backendApiStore._delete('/work_center/' + id)
  queryClient.invalidateQueries({ queryKey: ['work_centers'] })
}

function getName(workCenter: WorkCenter): string {
  return workCenter.name
}
</script>

<template>
  <div :style="{ display: 'flex', flexDirection: 'column', margin: '10px' }">
    <h1>Work Centers</h1>
    <ThingTable :get-name :headers :query="workCenterQuery" @delete="onDelete">
      <template #create-dialog="{ open, close }"
        ><CreateWorkCenterDialog :model-value="open" @update:model-value="close"
      /></template>
    </ThingTable>
  </div>
</template>