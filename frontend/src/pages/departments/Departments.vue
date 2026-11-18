<script setup lang="ts">
import { ref } from 'vue'
import type { Department } from '@/types/DatabaseTypes'
import { useQueryClient } from '@tanstack/vue-query'
import ThingTable from '@/components/ThingTable.vue'
import CreateDepartmentDialog from '@/pages/departments/CreateDepartmentDialog.vue'
import EditDepartmentDialog from '@/pages/departments/EditDepartmentDialog.vue'
import { useDepartmentQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
const departmentQuery = useDepartmentQuery()
const queryClient = useQueryClient()
const backendApiStore = useBackendApiStore()
const editingId = ref<number>(0)
const headers = [{ title: 'Department Name', key: 'name' }, { title: '', key: 'edit', width: '80px' }]

async function onDelete(id: number) {
  await backendApiStore._delete('/department/' + id)
  queryClient.invalidateQueries({ queryKey: ['departments'] })
}

function getName(department: Department): string {
  return department.name
}

function onEdit(department: Department) {
  editingId.value = department.id
}
</script>

<template>
  <div :style="{ display: 'flex', flexDirection: 'column', margin: '10px' }">
    <h1>Departments</h1>
    <ThingTable :custom-columns="['edit']" :get-name :headers :query="departmentQuery" @delete="onDelete">
      <template #create-dialog="{ open, close }"
        ><CreateDepartmentDialog :model-value="open" @update:model-value="close"
      /></template>
      <template #item.edit="{ item }">
        <v-btn size="small" variant="text" @click="onEdit(item)">
          Edit
        </v-btn>
      </template>
    </ThingTable>
    <EditDepartmentDialog
      v-if="editingId"
      :open="true"
      :id="editingId"
      @update:open="editingId = 0"
    />
  </div>
</template>
