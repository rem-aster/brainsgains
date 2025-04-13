<template>
    <div class="hero bg-base-100 min-h-screen md:pt-16 not-md:pb-16">
        <div class="hero-content text-center">
            <div class="max-w-md">
                <div v-if="currentCard">
                    <Card :card="currentCard" />
                </div>
                <div v-else class="size-64 content-center">
                    <div role="alert" class="alert alert-error">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none"
                            viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <span>No cards in this pack</span>
                    </div>
                </div>
                <div class="mt-1">
                    <progress class="progress w-56" :value="currentCardNumber" :max="totalCards"></progress>
                </div>

                <div class="grid grid-flow-col gap-2">
                    <button class="btn" @click="previousCard" :disabled="currentCardIndex === 0">
                        Back
                    </button>
                    <button class="btn btn-primary" @click="nextCard"
                        :disabled="currentCardIndex >= currentPackCards.length - 1">
                        Next
                    </button>
                </div>
                <label class="select mt-2">
                    <span class="label">Pack</span>
                    <select class="select select-sm" v-model="selectedPack" @change="loadPackCards">
                        <option v-for="pack in allPacks" :key="pack.id" :value="pack.id">
                            {{ pack.name }}
                        </option>
                    </select>
                </label>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import Database from '@tauri-apps/plugin-sql';
import Card from './Card.vue';

const db = ref(null);
const dbError = ref(null);
const allPacks = ref([]);
const selectedPack = ref(null);
const currentPackCards = ref([]);
const currentCardIndex = ref(0);
const currentCard = computed(() =>
    currentPackCards.value[currentCardIndex.value]
);

const currentCardNumber = computed(() =>
    currentCardIndex.value + 1
);

const totalCards = computed(() =>
    currentPackCards.value.length
);

const loadAllPacks = async () => {
    try {
        allPacks.value = await db.value.select('SELECT * FROM packs');
        if (allPacks.value.length > 0) {
            selectedPack.value = allPacks.value[0].id;
            loadPackCards();
        }
    } catch (error) {
        dbError.value = error.message;
    }
};

const loadPackCards = async () => {
    try {
        currentPackCards.value = await db.value.select(
            'SELECT * FROM cards WHERE pack_id = ? ORDER BY id',
            [selectedPack.value]
        );
        currentCardIndex.value = 0;
    } catch (error) {
        dbError.value = error.message;
    }
};

const nextCard = () => {
    if (currentCardIndex.value < currentPackCards.value.length - 1) {
        currentCardIndex.value++;
        currentPackCards.value[currentCardIndex.value].checked = false;
    }
};

const previousCard = () => {
    if (currentCardIndex.value > 0) {
        currentCardIndex.value--;
        currentPackCards.value[currentCardIndex.value].checked = false;
    }
};

onMounted(async () => {
    try {
        db.value = await Database.load('sqlite:cards.db');
        await loadAllPacks();
    } catch (error) {
        dbError.value = error.message;
    }
});
</script>