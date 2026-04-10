import axios from 'axios'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useMessageStore = defineStore('message', () => {
  const messages = ref([])

  function sendMessage (text: string, color: string) {
    messages.value.push({ text, color })
  }

  return { messages, sendMessage }
})
