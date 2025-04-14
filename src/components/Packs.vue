<template>
    <div class="hero bg-base-100 min-h-screen md:pt-16 not-md:pb-16">
        <div class="hero-content flex-col w-full max-w-4xl">
            <div class="w-full">
                <div class="flex gap-2">
                    <input v-model="newPackName" placeholder="New pack name" class="input input-bordered w-full" />
                    <button @click="addPack" class="btn btn-neutral">Add Pack</button>
                </div>
                <input v-model="newPackDescription" placeholder="Description (optional)"
                    class="input input-bordered w-full mt-2" />
            </div>
            <div class="join join-vertical w-full" @change="loadPackCards">
                <div v-for="pack in allPacks" :key="pack.id"
                    class="join-item collapse collapse-arrow bg-base-100 border border-base-300">
                    <input type="radio" name="packs" :value="pack.id" v-model="selectedPack" />
                    <div class="collapse-title font-semibold flex justify-between items-center">
                        <span>{{ pack.name }}</span>
                    </div>
                    <div class="collapse-content">
                        <p class="text-sm text-gray-500 mb-2">{{ pack.description }}</p>
                        <div class="space-y-2">
                            <div v-for="card in currentPackCards" :key="card.id"
                                class="flex items-center justify-between p-2 bg-base-100 rounded border border-neutral">
                                <div>
                                    <p class="font-medium">{{ card.front }}</p>
                                    <p class="text-sm">{{ card.back }}</p>
                                </div>
                                <button @click="deleteCard(card.id)" class="btn btn-sm btn-circle btn-ghost">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none"
                                        viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                            d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>
                            </div>
                        </div>
                        <button class="btn btn-neutral mt-2 mr-2" @click="showPackModal(pack.id)">Add Card</button>
                        <button @click="showDeletePackModal(pack.id)" class="btn btn-error mt-2">Delete
                            Pack</button>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <dialog ref="packModal" class="modal">
        <div class="modal-box">
            <form method="dialog">
                <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
            </form>
            <fieldset class="fieldset">
                <legend class="fieldset-legend">Add Card</legend>
                <input v-model="newCardFront" placeholder="Front" class="input input-bordered w-full" />
                <input v-model="newCardBack" placeholder="Back" class="input input-bordered w-full" />
                <button @click.prevent="handleAddCard" class="btn btn-sm btn-primary">Add</button>
            </fieldset>
        </div>
    </dialog>
    <dialog ref="deletePackModal" class="modal">
        <div class="modal-box">
            <form method="dialog">
                <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
            </form>
            <fieldset class="fieldset">
                <legend class="fieldset-legend">Are you sure?</legend>
                <button @click.prevent="handleDeletePack"
                    class="btn btn-error">Sure</button>
            </fieldset>
        </div>
    </dialog>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import Database from '@tauri-apps/plugin-sql';

const currentPackId = ref(null);
const packModal = ref(null);
const deletePackModal = ref(null);

const showPackModal = (packID) => {
    currentPackId.value = packID;
    packModal.value.showModal();
};

const showDeletePackModal = (packID) => {
    currentPackId.value = packID;
    deletePackModal.value.showModal();
};

const hidePackModal = () => {
    packModal.value.close();
};

const hideDeletePackModal = () => {
    deletePackModal.value.close();
};

const db = ref(null);
const allPacks = ref([]);
const selectedPack = ref(null);
const currentPackCards = ref([]);
const newPackName = ref('');
const newPackDescription = ref('');
const newCardFront = ref('');
const newCardBack = ref('');

const handleAddCard = async () => {
    if (!currentPackId.value) {
        console.error('No pack selected');
        return;
    }
    try {
        await addCard(currentPackId.value);
        hidePackModal();
    } catch (error) {
        console.error('Error handling card addition:', error);
    }
};

const handleDeletePack = async () => {
    if (!currentPackId.value) {
        console.error('No pack selected');
        return;
    }
    try {
        await deletePack(currentPackId.value);
        hideDeletePackModal();
    } catch (error) {
        console.error('Error handling pack deletion:', error);
    }
};

const loadAllPacks = async () => {
    try {
        allPacks.value = await db.value.select('SELECT * FROM packs ORDER BY created_at DESC');
        if (allPacks.value.length > 0 && !selectedPack.value) {
            selectedPack.value = allPacks.value[0].id;
            loadPackCards();
        }
    } catch (error) {
        console.error('Error loading packs:', error);
    }
};
const loadPackCards = async () => {
    if (!selectedPack.value) return;
    try {
        currentPackCards.value = await db.value.select(
            'SELECT * FROM cards WHERE pack_id = ? ORDER BY created_at DESC',
            [selectedPack.value]
        );
    } catch (error) {
        console.error('Error loading cards:', error);
    }
};
const addPack = async () => {
    if (!newPackName.value.trim()) return;
    try {
        await db.value.execute(
            'INSERT INTO packs (name, description) VALUES (?, ?)',
            [newPackName.value.trim(), newPackDescription.value.trim()]
        );
        newPackName.value = '';
        newPackDescription.value = '';
        await loadAllPacks();
    } catch (error) {
        console.error('Error adding pack:', error);
    }
};
const deletePack = async (packId) => {
    try {
        await db.value.execute('DELETE FROM cards WHERE pack_id = ?', [packId]);
        await db.value.execute('DELETE FROM packs WHERE id = ?', [packId]);
        await loadAllPacks();
        if (selectedPack.value === packId) {
            selectedPack.value = allPacks.value.length > 0 ? allPacks.value[0].id : null;
            loadPackCards();
        }
    } catch (error) {
        console.error('Error deleting pack:', error);
    }
};

const addCard = async (packId) => {
    if (!newCardFront.value.trim() || !newCardBack.value.trim()) return;
    try {
        await db.value.execute(
            'INSERT INTO cards (front, back, pack_id) VALUES (?, ?, ?)',
            [newCardFront.value.trim(), newCardBack.value.trim(), packId]
        );
        newCardFront.value = '';
        newCardBack.value = '';
        await loadPackCards();
    } catch (error) {
        console.error('Error adding card:', error);
    }
};

const deleteCard = async (cardId) => {
    try {
        await db.value.execute('DELETE FROM cards WHERE id = ?', [cardId]);
        await loadPackCards();
    } catch (error) {
        console.error('Error deleting card:', error);
    }
};

onMounted(async () => {
    try {
        db.value = await Database.load('sqlite:cards.db');
        await loadAllPacks();
    } catch (error) {
        console.error('Database error:', error);
    }
});
</script>