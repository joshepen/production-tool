import type { User } from '@/types/databaseTypes'
import { useQuery } from '@tanstack/vue-query'
import { useBackendApiStore } from '@/stores/backendApi'

export function useUserQuery (): Query {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['users'],
    queryFn: async () => {
      const response = await backendApiStore.get('/users')
      return response.data
    },
    select: (data: any[]) =>
      data.map(
        item =>
          ({
            ...item,
            hired_at: new Date(item.hired_at),
          }) as User,
      ),
  })
}

export function useDepartmentQuery (): Query {
  const backendApiStore = useBackendApiStore()
  return useQuery({
    queryKey: ['departments'],
    queryFn: async () => {
      const response = await backendApiStore.get('/departments')
      return response.data
    },
  })
}
