/**
 * router/index.ts
 *
 * Manual routes for ./src/pages/*.vue
 */

// Composables
import { createRouter, createWebHistory } from 'vue-router'
import Departments from '@/pages/departments/Departments.vue'
import Index from '@/pages/index.vue'
import ProductOrders from '@/pages/product_orders/ProductOrders.vue'
import Products from '@/pages/products/Products.vue'
import Users from '@/pages/users/Users.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: Index,
    },
    {
      path: '/users',
      component: Users,
    },
    {
      path: '/departments',
      component: Departments,
    },
    {
      path: '/products',
      component: Products,
    },
    {
      path: '/product_orders',
      component: ProductOrders,
    },
  ],
})

export default router
