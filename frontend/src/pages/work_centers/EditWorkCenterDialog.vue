<script setup lang="ts">
import { ref, watch } from 'vue'
import { useQueryClient } from '@tanstack/vue-query'
import { useBackendApiStore } from '@/stores/backendApi'
import { useStatusMessageStore } from '@/stores/statusMessage'

const props = defineProps<{ open: boolean; id: number }>()
const emit = defineEmits(['update:open'])

const backendApiStore = useBackendApiStore()
const messageStore = useStatusMessageStore()
const queryClient = useQueryClient()

const name = ref<string>('')
const description = ref<string>('')

async function loadWorkCenter() {
  if (!props.id) return
  const res = await backendApiStore.get(`/work_center/${props.id}`)
  name.value = res.data.name
  description.value = res.data.description ?? ''
}

function close() {
  emit('update:open', false)
}

function saveWorkCenter() {
  backendApiStore.put(`/work_center/${props.id}`, { name: name.value, description: description.value }).then(() => {
    messageStore.displayMessage('Successfully updated work center', 'success')
    queryClient.invalidateQueries({ queryKey: ['work_centers'] })
    close()
  })
}

watch(() => props.open, (val) => {
  if (val) {
    loadWorkCenter()
  }
})
</script>

<template>
  <v-dialog :model-value="open" width="auto" @update:model-value="$emit('update:open', $event)">
    <v-card>
      <v-card-item><v-card-title>Edit Work Center</v-card-title></v-card-item>
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
        <v-btn color="indigo-lighten-1" @click="saveWorkCenter">Confirm</v-btn>
        <v-btn color="red-lighten-1" @click="close">Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>