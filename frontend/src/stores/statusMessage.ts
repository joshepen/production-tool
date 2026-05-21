import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useStatusMessageStore = defineStore('status_message', () => {
  const messages = ref<{ text: string; color: string }[]>([])

  function displayMessage(text: string, color: string) {
    messages.value.push({ text, color })
  }

  return { messages, displayMessage }
})
