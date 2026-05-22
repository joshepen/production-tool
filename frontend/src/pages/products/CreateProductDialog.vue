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

const name = ref<string>('')

function createProduct() {
  const data = { name: name.value }
  backendApiStore.post('/product', data).then(() => {
    model.value = false
    messageStore.displayMessage('Successfully created product', 'success')
    queryClient.invalidateQueries({ queryKey: ['products'] })
  })
}

function clearFields() {
  name.value = ''
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
      <v-card-item><v-card-title>Create Product</v-card-title></v-card-item>
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
          label="Product Name"
          :style="{ minWidth: '250px' }"
        />
      </div>
      <template #actions>
        <v-btn color="indigo-lighten-1" @click="createProduct">Confirm</v-btn>
        <v-btn color="red-lighten-1" @click="model = false">Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>
