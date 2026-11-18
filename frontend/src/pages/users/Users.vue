<script setup lang="ts">
import { ref } from 'vue'
import type { User } from '@/types/DatabaseTypes'
import { useQueryClient } from '@tanstack/vue-query'
import ThingTable from '@/components/ThingTable.vue'
import CreateUserDialog from '@/pages/users/CreateUserDialog.vue'
import EditUserDialog from '@/pages/users/EditUserDialog.vue'
import { useUserQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
const userQuery = useUserQuery()
const queryClient = useQueryClient()
const backendApiStore = useBackendApiStore()
const editingId = ref<number>(0)
const headers = [
  { title: 'First Name', key: 'first_name' },
  { title: 'Last Name', key: 'last_name' },
  { title: 'Department', key: 'department_name' },
  { title: 'User Type', key: 'user_type_name' },
  { title: 'Hired At', key: 'hired_at', isDate: true },
  { title: '', key: 'edit', width: '80px' },
]

async function onDelete(id: number) {
  await backendApiStore._delete('/user/' + id)
  queryClient.invalidateQueries({ queryKey: ['users'] })
}
function getName(user: User): string {
  return user.first_name + ' ' + user.last_name
}

function onEdit(user: User) {
  editingId.value = user.id
}
</script>

<template>
  <div :style="{ display: 'flex', flexDirection: 'column', margin: '10px' }">
    <h1>Users</h1>
    <ThingTable :custom-columns="['edit']" :get-name :headers :query="userQuery" @delete="onDelete">
      <template #create-dialog="{ open, close }"
        ><CreateUserDialog :model-value="open" @update:model-value="close"
      /></template>
      <template #item.edit="{ item }">
        <v-btn size="small" variant="text" @click="onEdit(item)">
          Edit
        </v-btn>
      </template>
    </ThingTable>
    <EditUserDialog
      v-if="editingId"
      :open="true"
      :id="editingId"
      @update:open="editingId = 0"
    />
  </div>
</template>
