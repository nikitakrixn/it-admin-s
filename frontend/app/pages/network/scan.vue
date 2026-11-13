<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4">
        <div
          class="h-16 w-16 rounded-2xl bg-gradient-to-br from-purple-500 to-purple-700 flex items-center justify-center shadow-lg"
        >
          <Icon name="ri:radar-line" class="text-3xl text-white" />
        </div>
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Сканирование сети</h1>
          <p class="mt-1 text-gray-600">Поиск и анализ устройств в локальной сети</p>
        </div>
      </div>
    </div>

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
              <Icon name="ri:global-line" class="inline mr-1" />
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
              :disabled="scanConfig.pingOnly"
            />
            <p class="mt-1 text-xs text-gray-500">Укажите порты через запятую</p>
          </div>

          <div class="space-y-3">
            <label class="block text-sm font-medium text-gray-700">Режим сканирования</label>
            <div
              class="flex items-center gap-2 p-3 rounded-lg border border-gray-200 hover:border-primary-300 transition-colors cursor-pointer"
            >
              <input
                id="ping-only"
                v-model="scanConfig.pingOnly"
                type="checkbox"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="ping-only" class="flex-1 text-sm text-gray-700 cursor-pointer">
                <span class="font-medium">Только проверка доступности</span>
                <span class="text-gray-500 block text-xs"
                  >Проверяет типовые порты (22, 80, 443, 3389) и не сканирует остальные</span
                >
              </label>
            </div>
          </div>

          <button :disabled="scanning" class="w-full btn btn-primary" @click="startScan">
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
              ></div>
            </div>
            <p class="text-xs text-gray-500 text-center">{{ currentScanIp }}</p>
          </div>

          <div v-else-if="foundDevices.length > 0" class="text-center py-4">
            <div
              class="h-16 w-16 rounded-full bg-green-100 flex items-center justify-center mx-auto mb-3"
            >
              <Icon name="ri:checkbox-circle-line" class="text-3xl text-green-600" />
            </div>
            <p class="text-sm font-medium text-gray-900">Сканирование завершено</p>
            <p class="text-xs text-gray-500 mt-1">Найдено {{ foundDevices.length }} устройств</p>
          </div>

          <div v-else class="text-center py-4">
            <div
              class="h-16 w-16 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-3"
            >
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
          <button class="btn btn-secondary btn-sm" @click="exportResults">
            <Icon name="ri:download-line" class="mr-2" />
            Экспорт
          </button>
        </div>
      </div>
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                IP-адрес
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Имя хоста
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                MAC-адрес
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Открытые порты
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Статус
              </th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                Действия
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr
              v-for="device in foundDevices"
              :key="device.ip"
              class="hover:bg-gray-50 transition-colors"
            >
              <td class="px-4 py-4">
                <div class="flex items-center gap-2">
                  <div class="h-2 w-2 rounded-full bg-green-500"></div>
                  <span class="text-sm font-medium text-gray-900">{{ device.ip }}</span>
                </div>
              </td>
              <td class="px-4 py-4">
                <div class="flex items-center gap-2">
                  <Icon name="ri:computer-line" class="text-gray-400" />
                  <span class="text-sm text-gray-900">{{ device.hostname || '—' }}</span>
                </div>
              </td>
              <td class="px-4 py-4">
                <span class="text-sm text-gray-500 font-mono">{{ device.mac || '—' }}</span>
              </td>
              <td class="px-4 py-4">
                <div v-if="device.ports.length" class="flex flex-wrap gap-1">
                  <span
                    v-for="port in device.ports"
                    :key="port"
                    class="badge bg-blue-100 text-blue-800 text-xs"
                  >
                    {{ port }}
                  </span>
                </div>
                <span v-else class="text-sm text-gray-400">—</span>
              </td>
              <td class="px-4 py-4">
                <span class="badge bg-green-100 text-green-800">
                  <Icon name="ri:checkbox-circle-line" class="mr-1" />
                  Онлайн
                </span>
              </td>
              <td class="px-4 py-4 text-right">
                <div class="flex items-center justify-end gap-2">
                  <button
                    class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                    title="Подробнее"
                    @click="viewDeviceDetails(device)"
                  >
                    <Icon name="ri:eye-line" class="text-gray-600" />
                  </button>
                  <button class="btn btn-primary btn-sm" @click="addToInventory(device)">
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

    <div class="card">
      <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
        <div class="flex items-center gap-2">
          <Icon name="ri:history-line" class="text-gray-600" />
          <h2 class="text-lg font-semibold text-gray-900">История сканирований</h2>
        </div>
      </div>
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Дата</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Диапазон
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Тип</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Устройств
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Длительность
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Статус
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-if="loadingHistory">
              <td colspan="6" class="px-4 py-10 text-center">
                <Icon name="ri:loader-4-line" class="text-2xl text-purple-500 animate-spin mb-2" />
                <p class="text-sm text-gray-500">Загрузка истории...</p>
              </td>
            </tr>
            <tr v-else-if="scans.length === 0">
              <td colspan="6" class="px-4 py-10 text-center">
                <p class="text-sm text-gray-500">Сканирований пока не было</p>
              </td>
            </tr>
            <template v-else>
              <tr v-for="scan in scans" :key="scan.id" class="hover:bg-gray-50">
                <td class="px-4 py-4 text-sm text-gray-900">
                  {{ formatDateTime(scan.started_at) }}
                </td>
                <td class="px-4 py-4 text-sm font-mono text-gray-700">
                  {{ scan.ip_range || '—' }}
                </td>
                <td class="px-4 py-4 text-sm text-gray-700">
                  {{ getScanTypeLabel(scan.scan_type) }}
                </td>
                <td class="px-4 py-4 text-sm text-gray-900">{{ scan.devices_found ?? 0 }}</td>
                <td class="px-4 py-4 text-sm text-gray-700">
                  {{ scan.scan_duration != null ? `${scan.scan_duration} с` : '—' }}
                </td>
                <td class="px-4 py-4">
                  <span :class="['badge', getScanStatus(scan.status).color]">
                    {{ getScanStatus(scan.status).label }}
                  </span>
                </td>
              </tr>
            </template>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { IpAddress, NetworkScan, ScannedHost } from '~/composables/useNetwork'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'Сканирование сети',
})

const toast = useToast()
const { runNetworkScan, fetchNetworkScans, fetchIpAddresses, createIpAddress } = useNetwork()

const scanConfig = ref({
  startIp: '192.168.1.1',
  endIp: '192.168.1.254',
  ports: '22,80,443,3389',
  pingOnly: false,
})

interface FoundDevice {
  ip: string
  hostname: string
  mac: string
  ports: number[]
}

const scanning = ref(false)
const scanProgress = ref(0)
const currentScanIp = ref('')
const foundDevices = ref<FoundDevice[]>([])
const scans = ref<NetworkScan[]>([])
const ipAddresses = ref<IpAddress[]>([])
const loadingHistory = ref(false)

let progressTimer: ReturnType<typeof setInterval> | null = null

const onlineDevices = computed(() => foundDevices.value.length)
const totalOpenPorts = computed(() =>
  foundDevices.value.reduce((sum, device) => sum + device.ports.length, 0),
)

const ipRange = computed(() => `${scanConfig.value.startIp} - ${scanConfig.value.endIp}`)

const ipToNumber = (value: string) => {
  const parts = value.trim().split('.')
  if (parts.length !== 4) return null

  let result = 0
  for (const part of parts) {
    const octet = Number(part)
    if (!Number.isInteger(octet) || octet < 0 || octet > 255) return null
    result = result * 256 + octet
  }
  return result
}

const totalAddresses = computed(() => {
  const start = ipToNumber(scanConfig.value.startIp)
  const end = ipToNumber(scanConfig.value.endIp)
  if (start === null || end === null || end < start) return 0
  return end - start + 1
})

const parsePorts = () =>
  scanConfig.value.ports
    .split(',')
    .map((value) => Number(value.trim()))
    .filter((port) => Number.isInteger(port) && port > 0 && port <= 65535)

const addressFor = (ip: string) => ipAddresses.value.find((address) => address.ip_address === ip)

const startProgress = () => {
  scanProgress.value = 0
  progressTimer = setInterval(() => {
    if (scanProgress.value < 90) {
      scanProgress.value += Math.max(1, Math.round((90 - scanProgress.value) / 12))
    }
  }, 300)
}

const stopProgress = () => {
  if (progressTimer) {
    clearInterval(progressTimer)
    progressTimer = null
  }
}

const fetchHistory = async () => {
  loadingHistory.value = true
  try {
    const [scanList, addressList] = await Promise.all([
      fetchNetworkScans({ per_page: 10 }),
      fetchIpAddresses({ per_page: 100 }),
    ])
    scans.value = scanList.scans
    ipAddresses.value = addressList.addresses
  } catch (error) {
    toast.error('Не удалось загрузить историю сканирований: ' + getErrorMessage(error))
  } finally {
    loadingHistory.value = false
  }
}

const startScan = async () => {
  const start = ipToNumber(scanConfig.value.startIp)
  const end = ipToNumber(scanConfig.value.endIp)

  if (start === null || end === null) {
    toast.warning('Укажите корректные IP-адреса начала и конца диапазона')
    return
  }
  if (end < start) {
    toast.warning('Начальный адрес должен быть меньше конечного')
    return
  }
  if (end - start + 1 > 256) {
    toast.warning('За один раз можно проверить не больше 256 адресов')
    return
  }

  const ports = scanConfig.value.pingOnly ? undefined : parsePorts()
  if (!scanConfig.value.pingOnly && ports && ports.length === 0) {
    toast.warning('Укажите хотя бы один порт или включите проверку доступности')
    return
  }

  scanning.value = true
  foundDevices.value = []
  currentScanIp.value = `Проверка ${ipRange.value}`
  startProgress()
  toast.info('Сканирование запущено')

  try {
    const scan = await runNetworkScan({
      start_ip: scanConfig.value.startIp.trim(),
      end_ip: scanConfig.value.endIp.trim(),
      ports,
      scan_type: scanConfig.value.pingOnly ? 'ping_sweep' : 'port_scan',
    })

    const results = scan.scan_results as { hosts?: ScannedHost[] } | null
    foundDevices.value = (results?.hosts ?? []).map((host) => {
      const address = addressFor(host.ip)
      return {
        ip: host.ip,
        hostname: address?.hostname ?? '',
        mac: address?.mac_address ?? '',
        ports: host.open_ports,
      }
    })

    scanProgress.value = 100
    toast.success(`Сканирование завершено: найдено ${foundDevices.value.length} устройств`)
    await fetchHistory()
  } catch (error) {
    scanProgress.value = 0
    toast.error('Не удалось выполнить сканирование: ' + getErrorMessage(error))
  } finally {
    stopProgress()
    scanning.value = false
    currentScanIp.value = ''
  }
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

const viewDeviceDetails = (device: FoundDevice) => {
  const ports = device.ports.length ? device.ports.join(', ') : 'нет открытых портов'
  toast.info(`${device.ip}: ${device.hostname || 'имя не определено'}, порты: ${ports}`)
}

const addToInventory = async (device: FoundDevice) => {
  if (addressFor(device.ip)) {
    toast.info(`Адрес ${device.ip} уже есть в инвентаре`)
    return
  }

  try {
    await createIpAddress({
      ip_address: device.ip,
      status: 'available',
      hostname: device.hostname || undefined,
      mac_address: device.mac || undefined,
      notes: 'Найдено при сканировании сети',
    })
    toast.success(`Адрес ${device.ip} добавлен в инвентарь`)
    await fetchHistory()
  } catch (error) {
    toast.error('Не удалось добавить адрес: ' + getErrorMessage(error))
  }
}

const formatDateTime = (value: string | null) => {
  if (!value) return '—'
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString('ru-RU')
}

const getScanTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    ping_sweep: 'Проверка доступности',
    port_scan: 'Сканирование портов',
    full_scan: 'Полное сканирование',
    arp_scan: 'ARP-сканирование',
  }
  return labels[type] || type
}

const getScanStatus = (status: string | null) => {
  const statuses: Record<string, { label: string; color: string }> = {
    running: { label: 'Выполняется', color: 'bg-blue-100 text-blue-800' },
    completed: { label: 'Завершено', color: 'bg-green-100 text-green-800' },
    failed: { label: 'Ошибка', color: 'bg-red-100 text-red-800' },
    cancelled: { label: 'Отменено', color: 'bg-gray-100 text-gray-800' },
  }
  return statuses[status ?? ''] ?? { label: status || '—', color: 'bg-gray-100 text-gray-800' }
}

onMounted(fetchHistory)
onBeforeUnmount(stopProgress)
</script>
