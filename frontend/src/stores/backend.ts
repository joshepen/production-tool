import axios from 'axios'
import { defineStore } from 'pinia'
import { reactive } from 'vue'

export const useBackendStore = defineStore('backend', () => {
  const snackbar = reactive({
    show: false,
    text: '',
    color: 'error',
  })

  const axiosInstance = axios.create({ baseURL: import.meta.env.VITE_BACKEND_URL })
  axiosInstance.interceptors.response.use(
    config => {
      return config.data
    },
    error => {
      snackbar.text = `Error: ${error.response?.data || `Data request failed for endpoint ${error.response.config.url}.`}`
      snackbar.color = 'error'
      snackbar.show = true
    },
  )

  async function get (path: string) {
    return await axiosInstance.get(path)
  }
  return { snackbar, get }
})
