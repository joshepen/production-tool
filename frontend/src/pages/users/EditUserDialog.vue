<script setup lang="ts">
import { ref, watch } from 'vue'
import { useQueryClient } from '@tanstack/vue-query'
import { useDepartmentQuery, useUserTypeQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
import { useStatusMessageStore } from '@/stores/statusMessage'

const props = defineProps<{ open: boolean; id: number }>()
const emit = defineEmits(['update:open'])

const backendApiStore = useBackendApiStore()
const departmentQuery = useDepartmentQuery()
const userTypeQuery = useUserTypeQuery()
const messageStore = useStatusMessageStore()
const queryClient = useQueryClient()

const firstName = ref<string>('')
const lastName = ref<string>('')
const departmentId = ref<number>(0)
const userTypeId = ref<number>(0)
const hiredAt = ref<Date>(new Date())

async function loadUser() {
  if (!props.id) return
  const res = await backendApiStore.get(`/user/${props.id}`)
  firstName.value = res.data.first_name
  lastName.value = res.data.last_name
  departmentId.value = res.data.department_id
  userTypeId.value = res.data.user_type_id
  hiredAt.value = new Date(res.data.hired_at)
}

function close() {
  emit('update:open', false)
}

function saveUser() {
  const data = {
    first_name: firstName.value,
    last_name: lastName.value,
    department_id: departmentId.value,
    user_type_id: userTypeId.value,
    hired_at: hiredAt.value,
  }
  backendApiStore.put(`/user/${props.id}`, data).then(() => {
    messageStore.displayMessage('Successfully updated user', 'success')
    queryClient.invalidateQueries({ queryKey: ['users'] })
    close()
  })
}

watch(() => props.open, (val) => {
  if (val) {
    loadUser()
  }
})
</script>

<template>
  <v-dialog :model-value="open" width="auto" @update:model-value="$emit('update:open', $event)">
    <v-card>
      <v-card-item><v-card-title>Edit User</v-card-title></v-card-item>
      <div
        :style="{
          display: 'flex',
          flexDirection: 'column',
          gap: '20px',
          alignItems: 'start',
          marginInline: '15px',
        }"
      >
        <div :style="{ display: 'flex', alignItems: 'center', gap: '10px' }">
          <v-text-field
            v-model="firstName"
            density="compact"
            hide-details="auto"
            label="First Name"
            :style="{ minWidth: '250px' }"
          />
          <v-text-field
            v-model="lastName"
            density="compact"
            hide-details="auto"
            label="Last Name"
            :style="{ minWidth: '250px' }"
          />
        </div>
        <v-autocomplete
          v-model="departmentId"
          hide-details="auto"
          item-title="name"
          item-value="id"
          :items="departmentQuery.data.value"
          label="Department"
          :style="{ minWidth: '300px' }"
        />
        <v-autocomplete
          v-model="userTypeId"
          hide-details="auto"
          item-title="name"
          item-value="id"
          :items="userTypeQuery.data.value"
          label="User Type"
          :style="{ minWidth: '300px' }"
        />
        <v-date-picker
          v-model="hiredAt"
          landscape
          :style="{
            transform: 'scale(0.65)',
            transformOrigin: 'top center',
            marginBottom: '-110px',
          }"
          title="Hired At"
        />
      </div>
      <template #actions>
        <v-btn color="indigo-lighten-1" @click="saveUser">Confirm</v-btn>
        <v-btn color="red-lighten-1" @click="close">Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>