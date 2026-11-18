<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useQueryClient } from '@tanstack/vue-query'
import { useWorkCenterQuery } from '@/queries/DatabaseQueries'
import { useBackendApiStore } from '@/stores/backendApi'
import { useStatusMessageStore } from '@/stores/statusMessage'

const props = defineProps<{ open: boolean; id: number }>()
const emit = defineEmits(['update:open'])

const backendApiStore = useBackendApiStore()
const workCenterQuery = useWorkCenterQuery()
const messageStore = useStatusMessageStore()
const queryClient = useQueryClient()

const name = ref<string>('')
const workCenterIds = ref<number[]>([])

async function loadProduct() {
  if (!props.id) return
  const res = await backendApiStore.get(`/product/${props.id}`)
  name.value = res.data.name
  const wcRes = await backendApiStore.get(`/product_work_centers/${props.id}`)
  workCenterIds.value = wcRes.data.map((w: any) => w.work_center_id)
}

onMounted(() => {
  loadProduct()
})

function close() {
  emit('update:open', false)
}

function saveProduct() {
  backendApiStore.put(`/product/${props.id}`, { name: name.value, work_center_ids: workCenterIds.value }).then(() => {
    messageStore.displayMessage('Successfully updated product', 'success')
    queryClient.invalidateQueries({ queryKey: ['products'] })
    queryClient.invalidateQueries({ queryKey: ['pretty_products'] })
    close()
  })
}

watch(() => props.id, () => {
  if (props.open) {
    loadProduct()
  }
})
</script>

<template>
  <v-dialog :model-value="open" width="auto" @update:model-value="$emit('update:open', $event)">
    <v-card>
      <v-card-item><v-card-title>Edit Product</v-card-title></v-card-item>
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
        <v-btn color="indigo-lighten-1" @click="saveProduct">Confirm</v-btn>
        <v-btn color="red-lighten-1" @click="close">Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>