import axios from 'axios'
import { defineStore } from 'pinia'
import { useStatusMessageStore } from '@/stores/statusMessage'

export const useBackendStore = defineStore('backend', () => {
  const messageStore = useStatusMessageStore()
  const axiosInstance = axios.create({ baseURL: import.meta.env.VITE_BACKEND_URL })
  axiosInstance.interceptors.response.use(
    config => {
      return config.data
    },
    error => {
      messageStore.sendMessage(`Error: ${error.response?.data || `Data request failed for endpoint ${error.response.config.url}.`}`, 'error')
    },
  )

  async function get (path: string) {
    return await axiosInstance.get(path)
  }
  return { get }
})
