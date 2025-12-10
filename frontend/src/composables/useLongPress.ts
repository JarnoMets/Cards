import { onUnmounted, ref } from 'vue'

type LongPressHandler<T> = (payload: T) => void

export interface UseLongPressOptions {
  delay?: number
}

export const useLongPress = <T>(handler: LongPressHandler<T>, options: UseLongPressOptions = {}) => {
  const delay = options.delay ?? 250
  const timeoutId = ref<number | null>(null)
  const payloadRef = ref<T | null>(null)

  const cancel = () => {
    if (timeoutId.value !== null) {
      window.clearTimeout(timeoutId.value)
      timeoutId.value = null
      payloadRef.value = null
    }
  }

  const start = (payload: T) => {
    cancel()
    payloadRef.value = payload
    timeoutId.value = window.setTimeout(() => {
      if (payloadRef.value !== null) {
        handler(payloadRef.value as T)
      }
      cancel()
    }, delay)
  }

  onUnmounted(cancel)

  return {
    start,
    cancel
  }
}
