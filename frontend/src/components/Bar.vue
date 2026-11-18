<script setup lang="ts">
  import { computed, ref, watch } from 'vue'
  import { useRouter } from 'vue-router'
  import { useDisplay } from 'vuetify'

  const router = useRouter()

  // Treat screens >= 960px as "desktop": the drawer is permanently open there.
  // On smaller screens the drawer collapses into a temporary overlay toggled by
  // the hamburger button in the app bar.
  const display = useDisplay({ mobileBreakpoint: 'md' })
  const isDesktop = computed(() => display.mdAndUp.value)

  const drawer = ref(isDesktop.value)

  const items = [
    { name: 'Users', path: '/users', icon: 'account-group' },
    { name: 'Departments', path: '/departments', icon: 'domain' },
    { name: 'Products', path: '/products', icon: 'package-variant' },
    { name: 'Product Orders', path: '/product_orders', icon: 'clipboard-list' },
    { name: 'Work Centers', path: '/work_centers', icon: 'cog' },
    { name: 'Work Orders', path: '/work_orders', icon: 'format-list-bulleted' },
  ]
</script>

<template>
  <v-app-bar color="surface" elevation="1">
    <v-btn icon="mdi-menu" @click="drawer = !drawer" />
    <v-app-bar-title @click="router.push('/')">Production Tool</v-app-bar-title>
  </v-app-bar>

  <v-navigation-drawer
    :model-value="drawer"
    :permanent="false"
    width="256"
    @update:model-value="drawer = $event"
  >
    <v-list-item class="pa-4 pb-2">
      <v-list-item-title class="text-subtitle-2 text-medium-emphasis">
        Production Tool
      </v-list-item-title>
    </v-list-item>

    <v-list density="comfortable" nav>
      <v-list-group color="primary" :model-value="true">
        <template #activator="{ props }">
          <v-list-item
            prepend-icon="mdi-shield-account"
            title="Admin"
            v-bind="props"
          />
        </template>
        <v-list-item
          v-for="item in items"
          :key="item.path"
          :prepend-icon="item.icon"
          :title="item.name"
          :to="item.path"
        />
      </v-list-group>
    </v-list>
  </v-navigation-drawer>
</template>
