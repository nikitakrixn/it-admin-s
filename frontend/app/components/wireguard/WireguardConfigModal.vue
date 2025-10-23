<template>
  <UiModal
    :model-value="modelValue"
    size="lg"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #header>
      <div class="flex items-center gap-3">
        <div class="h-12 w-12 rounded-xl bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white shadow-lg">
          <Icon name="ri:file-text-line" class="text-2xl" />
        </div>
        <div>
          <h3 class="text-xl font-semibold text-gray-900">
            Конфигурация WireGuard
          </h3>
          <p class="text-sm text-gray-600">{{ peerName }}</p>
        </div>
      </div>
    </template>

    <div class="bg-gray-900 rounded-xl p-4 overflow-x-auto">
      <pre class="text-sm text-green-400 font-mono">{{ config }}</pre>
    </div>
    
    <div class="mt-4 p-4 bg-blue-50 rounded-lg border border-blue-200">
      <div class="flex items-start gap-3">
        <Icon name="ri:information-line" class="text-blue-600 text-xl flex-shrink-0 mt-0.5" />
        <div class="text-sm text-blue-900">
          <p class="font-medium mb-1">Как использовать:</p>
          <ol class="list-decimal list-inside space-y-1 text-blue-800">
            <li>Скачайте конфигурационный файл</li>
            <li>Импортируйте его в WireGuard клиент</li>
            <li>Активируйте подключение</li>
          </ol>
        </div>
      </div>
    </div>

    <template #footer>
      <UiButton variant="secondary" @click="$emit('update:modelValue', false)">
        Закрыть
      </UiButton>
      <UiButton variant="ghost" icon="ri:file-copy-line" @click="$emit('copy')">
        Копировать
      </UiButton>
      <UiButton icon="ri:download-line" @click="$emit('download')">
        Скачать
      </UiButton>
    </template>
  </UiModal>
</template>

<script setup lang="ts">
interface Props {
  modelValue: boolean
  config: string
  peerName: string
}

defineProps<Props>()

defineEmits<{
  'update:modelValue': [value: boolean]
  copy: []
  download: []
}>()
</script>
