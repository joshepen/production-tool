<script setup lang="ts">
import { useQueryClient } from '@tanstack/vue-query'
import ThingTable from '@/components/ThingTable.vue'
import CreateUserDialog from '@/pages/users/CreateUserDialog'
import { useUserQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
const userQuery = useUserQuery()
const queryClient = useQueryClient()
const backendApiStore = useBackendApiStore()
const headers = [
  { title: 'First Name', key: 'first_name' },
  { title: 'Last Name', key: 'last_name' },
  { title: 'Department', key: 'department_name' },
  { title: 'Hired At', key: 'hired_at', isDate: true },
]

async function onDelete(id: number) {
  await backendApiStore._delete('/user/' + id)
  queryClient.invalidateQueries(['users'])
}

function getName(user: User): string {
  return user.first_name + ' ' + user.last_name
}
</script>

<template>
  <div :style="{ display: 'flex', flexDirection: 'column', margin: '10px' }">
    <h1>Users</h1>
    <ThingTable :get-name :headers :query="userQuery" @delete="onDelete">
      <template #create-dialog="{ open, close }"
        ><CreateUserDialog :model-value="open" @update:model-value="close"
      /></template>
    </ThingTable>
  </div>
</template>
