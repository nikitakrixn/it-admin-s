<template>
    <div class="max-w-3xl mx-auto space-y-6">
        <div class="mb-8">
            <div class="flex items-center gap-4 mb-6">
                <NuxtLink
                    :to="`/computers/${route.params.id}/components/${route.params.componentId}`"
                    class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                >
                    <Icon name="ri:arrow-left-line" class="text-xl" />
                </NuxtLink>
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-amber-500 to-amber-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon name="ri:edit-line" class="text-3xl text-white" />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Редактирование компонента
                        </h1>
                        <p class="mt-1 text-gray-600">{{ form.name }}</p>
                    </div>
                </div>
            </div>
        </div>

        <form @submit.prevent="handleSubmit" class="space-y-6">
            <div class="card">
                <div
                    class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <h2 class="text-lg font-semibold text-gray-900">
                        Основная информация
                    </h2>
                </div>
                <div class="p-6 space-y-4">
                    <div>
                        <label
                            class="block text-sm font-medium text-gray-700 mb-2"
                            >Тип компонента</label
                        >
                        <input
                            :value="getComponentTypeLabel(form.type)"
                            type="text"
                            class="input-field"
                            disabled
                        />
                    </div>

                    <div>
                        <label
                            class="block text-sm font-medium text-gray-700 mb-2"
                            >Название *</label
                        >
                        <input
                            v-model="form.name"
                            type="text"
                            required
                            class="input-field"
                        />
                    </div>
                </div>
            </div>

            <div class="card">
                <div
                    class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <h2 class="text-lg font-semibold text-gray-900">
                        Характеристики
                    </h2>
                </div>
                <div class="p-6 space-y-4">
                    <div class="grid grid-cols-2 gap-4">
                        <div v-for="(value, key) in form.specs" :key="key">
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                                >{{ key }}</label
                            >
                            <input
                                v-model="form.specs[key]"
                                type="text"
                                class="input-field"
                            />
                        </div>
                    </div>

                    <div>
                        <label
                            class="block text-sm font-medium text-gray-700 mb-2"
                            >Примечания</label
                        >
                        <textarea
                            v-model="form.notes"
                            rows="3"
                            class="input-field resize-none"
                        />
                    </div>
                </div>
            </div>

            <div class="flex items-center justify-between gap-4">
                <NuxtLink
                    :to="`/computers/${route.params.id}/components/${route.params.componentId}`"
                    class="btn btn-secondary"
                >
                    <Icon name="ri:close-line" class="mr-2" />
                    Отмена
                </NuxtLink>
                <button type="submit" class="btn btn-primary">
                    <Icon name="ri:save-line" class="mr-2" />
                    Сохранить изменения
                </button>
            </div>
        </form>
    </div>
</template>

<script setup lang="ts">
definePageMeta({
    middleware: "auth",
});

const route = useRoute();
const router = useRouter();
const toast = useToast();

const form = ref({
    type: "processor",
    name: "Intel Core i7-10700",
    specs: {
        Ядра: "8",
        Потоки: "16",
        Частота: "2.90 GHz",
        Кэш: "16 МБ",
    },
    notes: "",
});

useHead({
    title: `Редактирование ${form.value.name}`,
});

const handleSubmit = async () => {
    try {
        console.log("Обновление компонента:", form.value);
        toast.success("Компонент успешно обновлен!");
        router.push(
            `/computers/${route.params.id}/components/${route.params.componentId}`,
        );
    } catch (error) {
        toast.error("Ошибка при обновлении компонента");
    }
};

const getComponentTypeLabel = (type: string) => {
    const labels: Record<string, string> = {
        processor: "Процессор",
        ram: "Оперативная память",
        storage: "Накопитель",
        gpu: "Видеокарта",
        motherboard: "Материнская плата",
        network: "Сетевой адаптер",
        monitor: "Монитор",
        peripheral: "Периферия",
    };
    return labels[type] || type;
};
</script>
