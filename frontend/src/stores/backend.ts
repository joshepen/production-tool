import axios from 'axios'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useMessageStore } from '@/stores/message'

export const useBackendStore = defineStore('backend', () => {
  const messageStore = useMessageStore()
  const axiosInstance = axios.create({ baseURL: import.meta.env.VITE_BACKEND_URL })
  axiosInstance.interceptors.response.use(
    config => {
      return config.data
    },
    error => {
      console.log('HERE')
      messageStore.sendMessage(`Error: ${error.response?.data || `Data request failed for endpoint ${error.response.config.url}.`}`, 'error')
    },
  )

  async function get (path: string) {
    return await axiosInstance.get(path)
  }
  return { get }
})
