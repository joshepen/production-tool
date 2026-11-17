import type { ProductOrder, User, WorkCenter, PrettyWorkOrder, ProductWorkCenter, UserType } from '@/types/DatabaseTypes'
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

export function useWorkCenterQuery() {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['work_centers'],
    queryFn: async () => {
      const response = await backendApiStore.get('/work_centers')
      return response.data
    },
  })
}


export function useWorkOrderQuery(productOrderId: number) {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['work_orders', productOrderId],
    queryFn: async () => {
      const response = await backendApiStore.get(`/pretty_work_orders?product_order_id=${productOrderId}`)
      return response.data
    },
    select: (data: any[]) =>
      data.map(
        (item) =>
          ({
            ...item,
            started_at: item.started_at ? new Date(item.started_at) : undefined,
            completed_at: item.completed_at ? new Date(item.completed_at) : undefined,
          }) as PrettyWorkOrder,
      ),
    enabled: productOrderId > 0,
  })
}

export function useProductWorkCenterQuery(productId: number) {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['product_work_centers', productId],
    queryFn: async () => {
      const response = await backendApiStore.get(`/product_work_centers/${productId}`)
      return response.data
    },
    select: (data: any[]) =>
      data.map(
        (item) =>
          ({
            ...item,
          }) as ProductWorkCenter,
      ),
    enabled: productId > 0,
  })
}

export function useUserTypeQuery() {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['user_types'],
    queryFn: async () => {
      const response = await backendApiStore.get('/user_types')
      return response.data
    },
  })
}
