<script setup lang="ts">
import type { User } from '@/types/databaseTypes';
import { useUserQuery } from '@/queries/databaseQuery';
const userQuery = useUserQuery()
</script>

<template>
  <span>Users</span>
  <v-data-table :items="userQuery.data.value as User[] ?? []"
    :headers="[{ title: 'First Name', key: 'first_name' }, { title: 'Last Name', key: 'last_name' }, { title: 'Department ID', key: 'department_id' }, { title: 'Hired At', key: 'hired_at' }]"><template
      v-slot:item.hired_at="{ item }">{{ item.hired_at.toDateString() }}</template></v-data-table>
</template>
