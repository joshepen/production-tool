<script setup lang="ts">
  import ThingTable from '@/components/ThingTable.vue'
  import { useUserQuery } from '@/queries/DatabaseQuery'
  const userQuery = useUserQuery()
  const headers = [{ title: 'First Name', key: 'first_name' }, { title: 'Last Name', key: 'last_name' }, { title: 'Department ID', key: 'department_id' }, { title: 'Hired At', key: 'hired_at', isDate: true }]
</script>

<template>
  <div :style="{display: 'flex', flexDirection: 'column', margin: '10px'}">
    <h1>Users</h1>
    <ThingTable :headers :query="userQuery" />
  </div>
</template>
