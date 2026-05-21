<script setup lang="ts">
import { useQueryClient } from '@tanstack/vue-query'
import ThingTable from '@/components/ThingTable.vue'
import CreateDepartmentDialog from '@/pages/departments/CreateDepartmentDialog'
import { useDepartmentQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
const departmentQuery = useDepartmentQuery()
const queryClient = useQueryClient()
const backendApiStore = useBackendApiStore()
const headers = [{ title: 'Department Name', key: 'name' }]

async function onDelete(id: number) {
  await backendApiStore._delete('/department/' + id)
  queryClient.invalidateQueries(['departments'])
}

function getName(department: Department): string {
  return department.name
}
</script>

<template>
  <div :style="{ display: 'flex', flexDirection: 'column', margin: '10px' }">
    <h1>Departments</h1>
    <ThingTable :get-name :headers :query="departmentQuery" @delete="onDelete">
      <template #create-dialog="{ open, close }"
        ><CreateDepartmentDialog :model-value="open" @update:model-value="close"
      /></template>
    </ThingTable>
  </div>
</template>
