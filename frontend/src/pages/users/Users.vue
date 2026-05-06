<script setup lang="ts">
  import { useQueryClient } from '@tanstack/vue-query'
  import ThingTable from '@/components/ThingTable.vue'
  import { useUserQuery } from '@/queries/DatabaseQuery'
  import { useBackendApiStore } from '@/stores/backendApi'
  const userQuery = useUserQuery()
  const queryClient = useQueryClient()
  const backendApiStore = useBackendApiStore()
  const headers = [{ title: 'First Name', key: 'first_name' }, { title: 'Last Name', key: 'last_name' }, { title: 'Department ID', key: 'department_id' }, { title: 'Hired At', key: 'hired_at', isDate: true }]

  async function onDelete (id: number) {
    await backendApiStore._delete('/user/' + id)
    queryClient.invalidateQueries(['users'])
  }
</script>

<template>
  <div :style="{display: 'flex', flexDirection: 'column', margin: '10px'}">
    <h1>Users</h1>
    <ThingTable :headers :query="userQuery" @add="()=>console.log('Add pressed')" @delete="onDelete" />
  </div>
</template>
