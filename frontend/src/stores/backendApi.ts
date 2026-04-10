import axios, { type AxiosResponse } from 'axios'
import { defineStore } from 'pinia'
import { useStatusMessageStore } from '@/stores/statusMessage'

export const useBackendApiStore = defineStore('backend_api', () => {
  const messageStore = useStatusMessageStore()
  const axiosInstance = axios.create({ baseURL: import.meta.env.VITE_BACKEND_URL })
  axiosInstance.interceptors.response.use(
    (config) => {
      return config
    },
    (error) => {
      messageStore.displayMessage(
        `Error: ${error.response?.data || `Data request failed for endpoint ${error.response.config.url}.`}`,
        'error',
      )
      return error
    },
  )

  async function get(path: string): Promise<AxiosResponse> {
    return await axiosInstance.get(path)
  }
  return { get }
})
