<script setup lang="ts">
import { useQueryClient } from '@tanstack/vue-query'
import { ref, watch } from 'vue'
import { useBackendApiStore } from '@/stores/backendApi'
import { useStatusMessageStore } from '@/stores/statusMessage'
const model = defineModel<boolean>()
const backendApiStore = useBackendApiStore()
const messageStore = useStatusMessageStore()
const queryClient = useQueryClient()

const name = ref<string>('')
const description = ref<string>('')

function createWorkCenter() {
  const data = { name: name.value, description: description.value }
  backendApiStore.post('/work_center', data).then(() => {
    model.value = false
    messageStore.displayMessage('Successfully created work center', 'success')
    queryClient.invalidateQueries({ queryKey: ['work_centers'] })
  })
}

function clearFields() {
  name.value = ''
  description.value = ''
}

watch(model, (val) => {
  if (val) {
    clearFields()
  }
})
</script>

<template>
  <v-dialog v-model="model" width="auto">
    <v-card>
      <v-card-item><v-card-title>Create Work Center</v-card-title></v-card-item>
      <div
        :style="{
          display: 'flex',
          flexDirection: 'column',
          gap: '20px',
          alignItems: 'start',
          marginInline: '15px',
        }"
      >
        <v-text-field
          v-model="name"
          density="compact"
          hide-details="auto"
          label="Work Center Name"
          :style="{ minWidth: '250px' }"
        />
        <v-textarea
          v-model="description"
          density="compact"
          hide-details="auto"
          label="Description"
          :style="{ minWidth: '300px', minHeight: '100px' }"
        />
      </div>
      <template #actions>
        <v-btn color="indigo-lighten-1" @click="createWorkCenter">Confirm</v-btn>
        <v-btn color="red-lighten-1" @click="model = false">Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>