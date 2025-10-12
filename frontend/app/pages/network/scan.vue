<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4">
        <div class="h-16 w-16 rounded-2xl bg-gradient-to-br from-purple-500 to-purple-700 flex items-center justify-center shadow-lg">
          <Icon name="ri:radar-line" class="text-3xl text-white" />
        </div>
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Сканирование сети</h1>
          <p class="mt-1 text-gray-600">Поиск и анализ устройств в локальной сети</p>
        </div>
      </div>
    </div>

    <!-- Статистика -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Найдено устройств</p>
              <p class="text-2xl font-bold text-purple-600">{{ foundDevices.length }}</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center">
              <Icon name="ri:device-line" class="text-2xl text-purple-600" />
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Онлайн</p>
              <p class="text-2xl font-bold text-green-600">{{ onlineDevices }}</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center">
              <Icon name="ri:checkbox-circle-line" class="text-2xl text-green-600" />
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Прогресс</p>
              <p class="text-2xl font-bold text-blue-600">{{ scanProgress }}%</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center">
              <Icon name="ri:pie-chart-line" class="text-2xl text-blue-600" />
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Открытых портов</p>
              <p class="text-2xl font-bold text-amber-600">{{ totalOpenPorts }}</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-amber-100 flex items-center justify-center">
              <Icon name="ri:door-open-line" class="text-2xl text-amber-600" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="lg:col-span-2 card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-2">
            <Icon name="ri:settings-3-line" class="text-gray-600" />
            <h2 class="text-lg font-semibold text-gray-900">Параметры сканирования</h2>
          </div>
        </div>
        <div class="p-6 space-y-6">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              <Icon name="ri:ip-line" class="inline mr-1" />
              Диапазон IP-адресов
            </label>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <input
                  v-model="scanConfig.startIp"
                  type="text"
                  placeholder="192.168.1.1"
                  class="input-field"
                />
                <p class="mt-1 text-xs text-gray-500">Начальный IP</p>
              </div>
              <div>
                <input
                  v-model="scanConfig.endIp"
                  type="text"
                  placeholder="192.168.1.254"
                  class="input-field"
                />
                <p class="mt-1 text-xs text-gray-500">Конечный IP</p>
              </div>
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              <Icon name="ri:door-line" class="inline mr-1" />
              Порты для проверки
            </label>
            <input
              v-model="scanConfig.ports"
              type="text"
              placeholder="22,80,443,3389"
              class="input-field"
            />
            <p class="mt-1 text-xs text-gray-500">Укажите порты через запятую</p>
          </div>

          <div class="space-y-3">
            <label class="block text-sm font-medium text-gray-700">Режим сканирования</label>
            <div class="flex items-center gap-2 p-3 rounded-lg border border-gray-200 hover:border-primary-300 transition-colors cursor-pointer">
              <input
                v-model="scanConfig.pingOnly"
                type="checkbox"
                id="ping-only"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="ping-only" class="flex-1 text-sm text-gray-700 cursor-pointer">
                <span class="font-medium">Только ping</span>
                <span class="text-gray-500 block text-xs">Быстрое сканирование без проверки портов</span>
              </label>
            </div>
          </div>

          <button
            @click="startScan"
            :disabled="scanning"
            class="w-full btn btn-primary"
          >
            <Icon v-if="scanning" name="ri:loader-4-line" class="mr-2 animate-spin" />
            <Icon v-else name="ri:scan-line" class="mr-2" />
            {{ scanning ? 'Сканирование...' : 'Начать сканирование' }}
          </button>
        </div>
      </div>

      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-2">
            <Icon name="ri:bar-chart-line" class="text-gray-600" />
            <h2 class="text-lg font-semibold text-gray-900">Прогресс</h2>
          </div>
        </div>
        <div class="p-6 space-y-6">
          <div v-if="scanning" class="space-y-4">
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-600">Сканирование...</span>
              <span class="text-sm font-medium text-primary-600">{{ scanProgress }}%</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-3 overflow-hidden">
              <div
                class="bg-gradient-to-r from-primary-500 to-primary-600 h-3 rounded-full transition-all duration-300"
                :style="{ width: `${scanProgress}%` }"
              />
            </div>
            <p class="text-xs text-gray-500 text-center">{{ currentScanIp }}</p>
          </div>

          <div v-else-if="foundDevices.length > 0" class="text-center py-4">
            <div class="h-16 w-16 rounded-full bg-green-100 flex items-center justify-center mx-auto mb-3">
              <Icon name="ri:checkbox-circle-line" class="text-3xl text-green-600" />
            </div>
            <p class="text-sm font-medium text-gray-900">Сканирование завершено</p>
            <p class="text-xs text-gray-500 mt-1">Найдено {{ foundDevices.length }} устройств</p>
          </div>

          <div v-else class="text-center py-4">
            <div class="h-16 w-16 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-3">
              <Icon name="ri:radar-line" class="text-3xl text-gray-400" />
            </div>
            <p class="text-sm text-gray-600">Готов к сканированию</p>
          </div>

          <div class="pt-4 border-t border-gray-200 space-y-2">
            <div class="flex items-center justify-between text-sm">
              <span class="text-gray-600">Диапазон</span>
              <span class="font-medium text-gray-900">{{ ipRange }}</span>
            </div>
            <div class="flex items-center justify-between text-sm">
              <span class="text-gray-600">Всего адресов</span>
              <span class="font-medium text-gray-900">{{ totalAddresses }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="foundDevices.length > 0" class="card">
      <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <Icon name="ri:device-line" class="text-gray-600" />
            <h2 class="text-lg font-semibold text-gray-900">Найденные устройства</h2>
            <span class="badge bg-primary-100 text-primary-800">{{ foundDevices.length }}</span>
          </div>
          <button @click="exportResults" class="btn btn-secondary btn-sm">
            <Icon name="ri:download-line" class="mr-2" />
            Экспорт
          </button>
        </div>
      </div>
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">IP-адрес</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Имя хоста</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">MAC-адрес</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Открытые порты</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Статус</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Действия</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="device in foundDevices" :key="device.ip" class="hover:bg-gray-50 transition-colors">
              <td class="px-4 py-4">
                <div class="flex items-center gap-2">
                  <div class="h-2 w-2 rounded-full bg-green-500" />
                  <span class="text-sm font-medium text-gray-900">{{ device.ip }}</span>
                </div>
              </td>
              <td class="px-4 py-4">
                <div class="flex items-center gap-2">
                  <Icon name="ri:computer-line" class="text-gray-400" />
                  <span class="text-sm text-gray-900">{{ device.hostname || '-' }}</span>
                </div>
              </td>
              <td class="px-4 py-4">
                <span class="text-sm text-gray-500 font-mono">{{ device.mac || '-' }}</span>
              </td>
              <td class="px-4 py-4">
                <div v-if="device.ports && device.ports.length" class="flex flex-wrap gap-1">
                  <span
                    v-for="port in device.ports"
                    :key="port"
                    class="badge bg-blue-100 text-blue-800 text-xs"
                  >
                    {{ port }}
                  </span>
                </div>
                <span v-else class="text-sm text-gray-400">-</span>
              </td>
              <td class="px-4 py-4">
                <span class="badge bg-green-100 text-green-800">
                  <Icon name="ri:checkbox-circle-line" class="mr-1" />
                  Онлайн
                </span>
              </td>
              <td class="px-4 py-4 text-right">
                <div class="flex items-center justify-end gap-2">
                  <button @click="viewDeviceDetails(device)" class="p-2 rounded-lg hover:bg-gray-100 transition-colors" title="Подробнее">
                    <Icon name="ri:eye-line" class="text-gray-600" />
                  </button>
                  <button @click="addToInventory(device)" class="btn btn-primary btn-sm">
                    <Icon name="ri:add-line" class="mr-1" />
                    Добавить
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in'
  }
})

useHead({
  title: 'Сканирование сети'
})

const toast = useToast()

const scanConfig = ref({
  startIp: '192.168.1.1',
  endIp: '192.168.1.254',
  ports: '22,80,443,3389',
  pingOnly: false
})

const scanning = ref(false)
const scanProgress = ref(0)
const currentScanIp = ref('')
const foundDevices = ref<any[]>([])

const onlineDevices = computed(() => foundDevices.value.length)
const totalOpenPorts = computed(() => {
  return foundDevices.value.reduce((sum, device) => {
    return sum + (device.ports?.length || 0)
  }, 0)
})

const ipRange = computed(() => {
  return `${scanConfig.value.startIp} - ${scanConfig.value.endIp}`
})

const totalAddresses = computed(() => {
  const start = scanConfig.value.startIp.split('.').pop()
  const end = scanConfig.value.endIp.split('.').pop()
  return parseInt(end || '0') - parseInt(start || '0') + 1
})

const startScan = async () => {
  scanning.value = true
  scanProgress.value = 0
  foundDevices.value = []
  
  toast.info('Начато сканирование сети...')

  const mockDevices = [
    { ip: '192.168.1.1', hostname: 'router.local', mac: '00:11:22:33:44:55', ports: [80, 443] },
    { ip: '192.168.1.10', hostname: 'pc-admin', mac: '00:11:22:33:44:66', ports: [22, 3389] },
    { ip: '192.168.1.20', hostname: 'printer-hp', mac: '00:11:22:33:44:77', ports: [80, 9100] },
    { ip: '192.168.1.25', hostname: 'nas-storage', mac: '00:11:22:33:44:88', ports: [22, 80, 445] },
    { ip: '192.168.1.50', hostname: 'pc-user01', mac: '00:11:22:33:44:99', ports: [3389] }
  ]

  for (let i = 0; i <= 100; i += 5) {
    await new Promise(resolve => setTimeout(resolve, 200))
    scanProgress.value = i
    currentScanIp.value = `192.168.1.${Math.floor(i * 2.54)}`
    
    if (i === 20 || i === 40 || i === 60 || i === 75 || i === 95) {
      const deviceIndex = Math.floor(i / 20)
      if (mockDevices[deviceIndex]) {
        foundDevices.value.push(mockDevices[deviceIndex])
      }
    }
  }

  scanning.value = false
  currentScanIp.value = ''
  toast.success(`Сканирование завершено! Найдено ${foundDevices.value.length} устройств`)
}

const exportResults = () => {
  const data = JSON.stringify(foundDevices.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `network-scan-${Date.now()}.json`
  a.click()
  URL.revokeObjectURL(url)
  toast.success('Результаты экспортированы')
}

const viewDeviceDetails = (device: any) => {
  toast.info(`Просмотр деталей устройства ${device.ip}`)
}

const addToInventory = (device: any) => {
  toast.success(`Устройство ${device.ip} добавлено в инвентарь`)
}
</script>
