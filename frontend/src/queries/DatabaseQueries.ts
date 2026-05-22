import type { ProductOrder, User } from '@/types/DatabaseTypes'
import { useQuery } from '@tanstack/vue-query'
import { useBackendApiStore } from '@/stores/backendApi'

export function useUserQuery() {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['users'],
    queryFn: async () => {
      const response = await backendApiStore.get('/pretty_users')
      return response.data
    },
    select: (data: any[]) =>
      data.map(
        (item) =>
          ({
            ...item,
            hired_at: new Date(item.hired_at),
          }) as User,
      ),
  })
}

export function useDepartmentQuery() {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['departments'],
    queryFn: async () => {
      const response = await backendApiStore.get('/departments')
      return response.data
    },
  })
}

export function useProductQuery() {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['products'],
    queryFn: async () => {
      const response = await backendApiStore.get('/products')
      return response.data
    },
  })
}

export function useProductOrderQuery() {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['product_orders'],
    queryFn: async () => {
      const response = await backendApiStore.get('/pretty_product_orders')
      return response.data
    },
    select: (data: any[]) =>
      data.map(
        (item) =>
          ({
            ...item,
            created_at: new Date(item.created_at),
          }) as ProductOrder,
      ),
  })
}

export function useStatusQuery() {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['statuses'],
    queryFn: async () => {
      const response = await backendApiStore.get('/statuses')
      return response.data
    },
  })
}
