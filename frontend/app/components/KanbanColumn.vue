<template>
  <div class="card h-fit">
    <div :class="['px-4 py-3 border-b border-gray-200', headerClass]">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <div :class="['h-3 w-3 rounded-full', dotClass]" />
          <h3 class="text-sm font-semibold text-gray-900">{{ title }}</h3>
        </div>
        <span class="badge bg-gray-100 text-gray-800 text-xs">{{ requests.length }}</span>
      </div>
    </div>
    
    <div
      class="p-3 min-h-[400px] space-y-3"
      @drop="onDrop"
      @dragover.prevent
      @dragenter.prevent
    >
      <NuxtLink
        v-for="request in requests"
        :key="request.id"
        :to="`/requests/${request.id}`"
        class="card card-hover cursor-pointer group block"
        draggable="true"
        @dragstart="onDragStart(request, $event)"
      >
        <div class="p-4">
          <div class="flex items-start justify-between gap-2 mb-3">
            <h4 class="text-sm font-medium text-gray-900 line-clamp-2 group-hover:text-primary-600 transition-colors">
              {{ request.title }}
            </h4>
            <span
              :class="[
                'badge flex-shrink-0',
                getPriorityColor(request.priority)
              ]"
            >
              {{ getPriorityLabel(request.priority) }}
            </span>
          </div>
          
          <p class="text-xs text-gray-600 mb-3 line-clamp-2">{{ request.description }}</p>
          
          <div class="flex items-center justify-between text-xs text-gray-500">
            <div class="flex items-center gap-1">
              <Icon name="ri:user-line" />
              <span>{{ request.requester }}</span>
            </div>
            <div class="flex items-center gap-1">
              <Icon name="ri:time-line" />
              <span>{{ request.created_at }}</span>
            </div>
          </div>
        </div>
      </NuxtLink>
      
      <div v-if="requests.length === 0" class="flex flex-col items-center justify-center py-8 text-gray-400">
        <Icon name="ri:inbox-line" class="text-3xl mb-2" />
        <p class="text-sm">Нет заявок</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Request {
  id: number
  title: string
  description: string
  requester: string
  status: string
  priority: string
  created_at: string
}

const props = defineProps<{
  title: string
  status: string
  color: string
  requests: Request[]
}>()

const emit = defineEmits<{
  drop: [requestId: number, newStatus: string]
}>()

const headerClass = computed(() => {
  const classes = {
    blue: 'bg-blue-50 border-blue-200',
    amber: 'bg-amber-50 border-amber-200',
    green: 'bg-green-50 border-green-200',
    red: 'bg-red-50 border-red-200'
  }
  return classes[props.color as keyof typeof classes] || 'bg-gray-50'
})

const dotClass = computed(() => {
  const classes = {
    blue: 'bg-blue-500',
    amber: 'bg-amber-500',
    green: 'bg-green-500',
    red: 'bg-red-500'
  }
  return classes[props.color as keyof typeof classes] || 'bg-gray-500'
})

const onDragStart = (request: Request, event: DragEvent) => {
  if (event.dataTransfer) {
    event.dataTransfer.setData('text/plain', request.id.toString())
    event.dataTransfer.effectAllowed = 'move'
  }
}

const onDrop = (event: DragEvent) => {
  event.preventDefault()
  const requestId = event.dataTransfer?.getData('text/plain')
  if (requestId) {
    emit('drop', parseInt(requestId), props.status)
  }
}

const getPriorityColor = (priority: string) => {
  const colors: Record<string, string> = {
    low: 'bg-gray-100 text-gray-800',
    medium: 'bg-blue-100 text-blue-800',
    high: 'bg-red-100 text-red-800'
  }
  return colors[priority] || 'bg-gray-100 text-gray-800'
}

const getPriorityLabel = (priority: string) => {
  const labels: Record<string, string> = {
    low: 'Низкий',
    medium: 'Средний',
    high: 'Высокий'
  }
  return labels[priority] || priority
}
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
