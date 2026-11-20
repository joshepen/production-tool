<script setup lang="ts">
  import { computed, ref } from 'vue'
  import { useDisplay } from 'vuetify'

  // Treat screens >= 960px as "desktop": the drawer is permanently open there.
  // On smaller screens the drawer collapses into a temporary overlay toggled by
  // the floating menu button in the bottom-left corner.
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
  <v-navigation-drawer
    :model-value="drawer"
    :permanent="false"
    width="256"
    @update:model-value="drawer = $event"
  >
    <div class="d-flex align-center ga-2 px-4 py-3">
      <v-btn
        icon="mdi-menu-open"
        color="indigo-lighten-1"
        variant="text"
        density="compact"
        aria-label="Toggle navigation"
        @click="drawer = !drawer"
      />
      <span class="text-subtitle-2 text-medium-emphasis">Production Tool</span>
    </div>

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

  <v-btn
    v-if="!drawer"
    prepend-icon="mdi-menu"
    color="indigo-lighten-1"
    location="bottom left"
    position="fixed"
    class="ma-4"
    style="z-index: 3000"
    aria-label="Open navigation"
    @click="drawer = !drawer"
  >
    Menu
  </v-btn>
</template>