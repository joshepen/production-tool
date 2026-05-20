<script setup lang="ts">
  import { useQueryClient } from '@tanstack/vue-query'
  import { ref, watch } from 'vue'
  import { useDepartmentQuery } from '@/queries/DatabaseQueries'
  import { useBackendApiStore } from '@/stores/backendApi'
  import { useStatusMessageStore } from '@/stores/statusMessage'
  const model = defineModel<boolean>()
  const backendApiStore = useBackendApiStore()
  const departmentQuery = useDepartmentQuery()
  const messageStore = useStatusMessageStore()
  const queryClient = useQueryClient()

  const firstName = ref('')
  const lastName = ref('')
  const departmentId = ref(null)
  const hiredAt = ref(new Date())

  function createUser () {
    const data = { first_name: firstName.value, last_name: lastName.value, department_id: departmentId.value, hired_at: hiredAt.value }
    backendApiStore.post('/user', data).then(() => {
      model.value = false
      messageStore.displayMessage('Successfully created user', 'success')
      queryClient.invalidateQueries(['users'])
    })
  }

  function clearFields () {
    firstName.value = ''
    lastName.value = ''
    departmentId.value = null
    hiredAt.value = new Date()
  }

  watch(model, val => {
    if (val) {
      clearFields()
    }
  })
</script>

<template>
  <v-dialog v-model="model" width="auto">
    <v-card>
      <v-card-item><v-card-title>Create User</v-card-title></v-card-item>
      <div :style="{display: 'flex', flexDirection: 'column', gap: '20px', alignItems: 'start', marginInline: '15px'}">
        <div :style="{display: 'flex', alignItems: 'center', gap: '10px'}">
          <v-text-field
            v-model="firstName"
            density="compact"
            hide-details="auto"
            label="First Name"
            :style="{minWidth: '250px'}"
          />
          <v-text-field
            v-model="lastName"
            density="compact"
            hide-details="auto"
            label="Last Name"
            :style="{minWidth: '250px'}"
          />
        </div>
        <v-autocomplete
          v-model="departmentId"
          hide-details="auto"
          item-title="name"
          item-value="id"
          :items="departmentQuery.data.value"
          label="Department"
          :style="{minWidth:'300px'}"
        />
        <v-date-picker
          v-model="hiredAt"
          landscape
          :style="{ transform: 'scale(0.65)', transformOrigin: 'top center', marginBottom: '-110px' }"
          title="Hired At"
        /></div>
      <template #actions>
        <v-btn color="indigo-lighten-1" @click="createUser">Confirm</v-btn>
        <v-btn
          color="red-lighten-1"
          @click="model = false"
        >Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>
