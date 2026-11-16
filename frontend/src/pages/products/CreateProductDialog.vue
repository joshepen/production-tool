<script setup lang="ts">
import { useQueryClient } from '@tanstack/vue-query'
import { ref, watch } from 'vue'
import { useWorkCenterQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
import { useStatusMessageStore } from '@/stores/statusMessage'
const model = defineModel<boolean>()
const backendApiStore = useBackendApiStore()
const workCenterQuery = useWorkCenterQuery()
const messageStore = useStatusMessageStore()
const queryClient = useQueryClient()

const name = ref<string>('')
const workCenterIds = ref<number[]>([])

function createProduct() {
  const data = { name: name.value, work_center_ids: workCenterIds.value }
  backendApiStore.post('/product', data).then(() => {
    model.value = false
    messageStore.displayMessage('Successfully created product', 'success')
    queryClient.invalidateQueries({ queryKey: ['products'] })
  })
}

function clearFields() {
  name.value = ''
  workCenterIds.value = []
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
        <v-autocomplete
          v-model="workCenterIds"
          multiple
          chips
          closable-chips
          hide-details="auto"
          item-title="name"
          item-value="id"
          :items="workCenterQuery.data.value ?? []"
          label="Work centers (in order)"
          :style="{ minWidth: '320px' }"
        />
      </div>
      <template #actions>
        <v-btn color="indigo-lighten-1" @click="createProduct">Confirm</v-btn>
        <v-btn color="red-lighten-1" @click="model = false">Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>
