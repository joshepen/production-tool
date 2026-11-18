<script setup lang="ts">
import { ref } from 'vue'
import type { WorkCenter } from '@/types/DatabaseTypes'
import { useQueryClient } from '@tanstack/vue-query'
import ThingTable from '@/components/ThingTable.vue'
import CreateWorkCenterDialog from '@/pages/work_centers/CreateWorkCenterDialog.vue'
import EditWorkCenterDialog from '@/pages/work_centers/EditWorkCenterDialog.vue'
import { useWorkCenterQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
const workCenterQuery = useWorkCenterQuery()
const queryClient = useQueryClient()
const backendApiStore = useBackendApiStore()
const editingId = ref<number>(0)
const headers = [{ title: 'Name', key: 'name' }, { title: 'Description', key: 'description' }, { title: '', key: 'edit', width: '80px' }]

async function onDelete(id: number) {
  await backendApiStore._delete('/work_center/' + id)
  queryClient.invalidateQueries({ queryKey: ['work_centers'] })
}

function getName(workCenter: WorkCenter): string {
  return workCenter.name
}

function onEdit(workCenter: WorkCenter) {
  editingId.value = workCenter.id
}
</script>

<template>
  <div :style="{ display: 'flex', flexDirection: 'column', margin: '10px' }">
    <h1>Work Centers</h1>
    <ThingTable :custom-columns="['edit']" :get-name :headers :query="workCenterQuery" @delete="onDelete">
      <template #create-dialog="{ open, close }"
        ><CreateWorkCenterDialog :model-value="open" @update:model-value="close"
      /></template>
      <template #item.edit="{ item }">
        <v-btn size="small" variant="text" @click="onEdit(item)">
          Edit
        </v-btn>
      </template>
    </ThingTable>
    <EditWorkCenterDialog
      v-if="editingId"
      :open="true"
      :id="editingId"
      @update:open="editingId = 0"
    />
  </div>
</template>