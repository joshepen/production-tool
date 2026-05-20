<script setup lang=ts>
  import { ref } from 'vue'
  defineEmits(['confirm', 'cancel'])
  defineProps<{ name: string }>()
  const open = ref<boolean>(false)

</script>

<template>
  <v-btn
    color="red-lighten-1"
    density="compact"
    icon="mdi-delete"
    rounded="lg"
    size="x-large"
    :style="{justifySelf:'end'}"
    @click="open = true"
  />
  <v-dialog v-model="open">
    <v-card>
      <v-card-item>
        <v-card-title>
          Are you sure?
        </v-card-title>
      </v-card-item>
      <v-card-text>
        Are you sure you want to delete {{ name }}?
      </v-card-text>
      <template #actions>
        <v-btn color="indigo-lighten-1" @click="$emit('confirm')">Confirm</v-btn>
        <v-btn
          color="red-lighten-1"
          @click="open = false"
        >Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>
