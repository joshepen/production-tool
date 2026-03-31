<script setup lang=ts>
  import { useRouter } from 'vue-router'
  const router = useRouter()

  const buttons = [{ name: 'Users', path: '/users' }, { name: 'Departments', path: '/departments' }, { name: 'Products', path: '/products' }, { name: 'Product Orders', path: '/product_orders' }]
</script>

<template>
  <v-app-bar :style="{ paddingLeft: '16px', paddingRight: '16px' }">
    <v-app-bar-title @click="router.push('/')">Production Tool</v-app-bar-title>
    <div :style="{ display: 'flex', gap: '10px' }">
      <v-btn v-for="button in buttons" :key="button.path" @click="router.push(button.path)">
        {{ button.name }}
      </v-btn>
    </div>
  </v-app-bar>
</template>
