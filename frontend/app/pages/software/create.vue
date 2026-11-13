<template>
  <div class="max-w-3xl mx-auto space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink to="/software" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-indigo-500 to-indigo-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:add-circle-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Добавление ПО</h1>
            <p class="mt-1 text-gray-600">Добавить ПО в каталог</p>
          </div>
        </div>
      </div>
    </div>

    <form class="space-y-6" @submit.prevent="handleSubmit">
      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <h2 class="text-lg font-semibold text-gray-900">Основная информация</h2>
        </div>
        <div class="p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Название ПО *</label>
            <input
              v-model="form.name"
              type="text"
              required
              class="input-field"
              placeholder="Microsoft Office 365"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Производитель *</label>
            <input
              v-model="form.manufacturer"
              type="text"
              required
              class="input-field"
              placeholder="Microsoft Corporation"
            />
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Категория</label>
              <select v-model="form.category" class="input-field">
                <option value="">Выберите категорию</option>
                <option value="Операционные системы">Операционные системы</option>
                <option value="Офисные пакеты">Офисные пакеты</option>
                <option value="Браузеры">Браузеры</option>
                <option value="Антивирусы">Антивирусы</option>
                <option value="Среды разработки">Среды разработки</option>
                <option value="Медиаплееры">Медиаплееры</option>
                <option value="Коммуникации">Коммуникации</option>
                <option value="Удаленный доступ">Удаленный доступ</option>
                <option value="Текстовые редакторы">Текстовые редакторы</option>
                <option value="Архиваторы">Архиваторы</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Сайт</label>
              <input
                v-model="form.website"
                type="url"
                class="input-field"
                placeholder="https://..."
              />
            </div>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Описание</label>
            <textarea
              v-model="form.description"
              rows="3"
              class="input-field resize-none"
              placeholder="Описание программного обеспечения..."
            ></textarea>
          </div>
          <div class="flex items-center gap-4">
            <label class="flex items-center gap-2">
              <input
                v-model="form.requires_license"
                type="checkbox"
                class="h-4 w-4 text-primary-600 rounded"
              />
              <span class="text-sm text-gray-700">Требует лицензию</span>
            </label>
            <label class="flex items-center gap-2">
              <input
                v-model="form.is_system"
                type="checkbox"
                class="h-4 w-4 text-primary-600 rounded"
              />
              <span class="text-sm text-gray-700">Системное ПО</span>
            </label>
          </div>
        </div>
      </div>

      <div class="flex items-center justify-between gap-4">
        <NuxtLink to="/software" class="btn btn-secondary">
          <Icon name="ri:close-line" class="mr-2" />
          Отмена
        </NuxtLink>
        <button type="submit" class="btn btn-primary disabled:opacity-50" :disabled="saving">
          <Icon v-if="saving" name="ri:loader-4-line" class="mr-2 animate-spin" />
          <Icon v-else name="ri:save-line" class="mr-2" />
          {{ saving ? 'Добавление...' : 'Добавить ПО' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ middleware: 'auth' })
useHead({ title: 'Добавление ПО' })

const router = useRouter()
const toast = useToast()
const { createSoftware } = useSoftware()

const saving = ref(false)

const form = ref({
  name: '',
  manufacturer: '',
  category: '',
  website: '',
  description: '',
  requires_license: false,
  is_system: false,
})

const handleSubmit = async () => {
  if (!form.value.name.trim()) {
    toast.warning('Укажите название ПО')
    return
  }

  saving.value = true
  try {
    const created = await createSoftware({
      name: form.value.name.trim(),
      manufacturer: form.value.manufacturer.trim() || undefined,
      category: form.value.category || undefined,
      website: form.value.website.trim() || undefined,
      description: form.value.description.trim() || undefined,
      requires_license: form.value.requires_license,
      is_system: form.value.is_system,
    })

    toast.success('ПО добавлено в каталог')
    router.push(`/software/${created.id}`)
  } catch (error) {
    toast.error('Не удалось добавить ПО: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}
</script>
