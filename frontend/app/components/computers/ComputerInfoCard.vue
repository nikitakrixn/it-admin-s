<template>
  <div class="card">
    <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
      <h2 class="text-lg font-semibold text-gray-900">Общая информация</h2>
    </div>
    <div class="p-6">
      <div class="grid grid-cols-2 gap-6">
        <div>
          <p class="text-xs text-gray-600 mb-1">Производитель</p>
          <p class="text-sm font-medium text-gray-900">{{ computer.manufacturer || '—' }}</p>
        </div>
        <div>
          <p class="text-xs text-gray-600 mb-1">Модель</p>
          <p class="text-sm font-medium text-gray-900">{{ computer.model || '—' }}</p>
        </div>
        <div>
          <p class="text-xs text-gray-600 mb-1">Серийный номер</p>
          <p class="text-sm font-medium text-gray-900 font-mono">
            {{ computer.serial_number || '—' }}
          </p>
        </div>
        <div>
          <p class="text-xs text-gray-600 mb-1">Тип</p>
          <span class="badge bg-blue-100 text-blue-800">{{ typeLabel }}</span>
        </div>
        <div>
          <p class="text-xs text-gray-600 mb-1">Местоположение</p>
          <p class="text-sm font-medium text-gray-900">{{ computer.location || '-' }}</p>
        </div>
        <div>
          <p class="text-xs text-gray-600 mb-1">Домен</p>
          <span
            class="badge"
            :class="[
              computer.domain_joined ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800',
            ]"
          >
            {{ computer.domain_joined ? 'Присоединен' : 'Не присоединен' }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface ComputerProps {
  computer: {
    manufacturer: string | null
    model: string | null
    serial_number: string | null
    computer_type: string
    location: string | null
    domain_joined: boolean | null
  }
}

const props = withDefaults(defineProps<ComputerProps>(), {})

const typeLabels: Record<string, string> = {
  desktop: 'Десктоп',
  laptop: 'Ноутбук',
  server: 'Сервер',
  workstation: 'Рабочая станция',
  thin_client: 'Тонкий клиент',
}

const typeLabel = computed(
  () => typeLabels[props.computer.computer_type] || props.computer.computer_type,
)
</script>
