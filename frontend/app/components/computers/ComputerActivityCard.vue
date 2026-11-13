<template>
  <div class="card">
    <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
      <div class="flex items-center gap-2">
        <Icon name="ri:history-line" class="text-gray-600" />
        <h2 class="text-lg font-semibold text-gray-900">Журнал активности</h2>
      </div>
    </div>
    <div class="p-6">
      <div class="space-y-4">
        <div v-for="(event, index) in activityLog" :key="event.id" class="flex gap-4">
          <div class="flex flex-col items-center">
            <div
              :class="[
                'h-8 w-8 rounded-full flex items-center justify-center flex-shrink-0',
                getActivityColor(event.action),
              ]"
            >
              <Icon :name="getActivityIcon(event.action)" class="text-sm text-white" />
            </div>
            <div v-if="index < activityLog.length - 1" class="w-0.5 flex-1 bg-gray-200 mt-2"></div>
          </div>
          <div class="flex-1 pb-4">
            <div class="flex items-start justify-between gap-4">
              <div>
                <p class="text-sm text-gray-900">
                  <span class="font-semibold">{{ event.user_email || 'Система' }}</span>
                  {{ getActivityDescription(event) }}
                </p>
                <p v-if="getEventSummary(event)" class="text-sm text-gray-600 mt-1">
                  {{ getEventSummary(event) }}
                </p>
              </div>
              <button
                v-if="event.details"
                class="text-xs text-primary-600 hover:text-primary-700 flex-shrink-0"
                @click="onShowDetails(event)"
              >
                Подробнее
              </button>
            </div>
            <p class="text-xs text-gray-500 mt-1">{{ formatEventDate(event.created_at) }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ActivityLog } from '~/composables/useActivityLog'

interface Props {
  activityLog: ActivityLog[]
  formatEventDate: (dateStr: string) => string
  getActivityDescription: (event: ActivityLog) => string
  getEventSummary: (event: ActivityLog) => string
  getActivityIcon: (type: string) => string
  getActivityColor: (type: string) => string
}

defineProps<Props>()

const emit = defineEmits<{
  showDetails: [event: ActivityLog]
}>()

const onShowDetails = (event: ActivityLog) => {
  emit('showDetails', event)
}
</script>
